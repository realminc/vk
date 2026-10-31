//! Explicit relocation passes derived from pinned VMA 3.4.0
//! `ComputeDefragmentation_{Fast,Balanced,Full,Extensive}`, `UpdateVectorStatistics`,
//! `AllocInOtherBlock` and `EndPass` (LICENSE-VMA-MIT).
//! Adaptation: exclusive allocator borrow, bounded existing-block reservations,
//! checked consumer replacement/copy/completion, cooperative stop predicates,
//! raw/alias-group and counted mapping transfer, and no implicit work in Drop.
//! Extensive keeps context-owned phase state and empty blocks for class packing.
use super::*;

#[path = "defragmentation/extensive.rs"]
mod extensive;
use extensive::ExtensiveState;

/// Existing-block relocation policy, derived from the pinned VMA algorithms.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DefragmentationAlgorithm {
	/// Consolidate sparse blocks into fuller compatible blocks, without same-block moves.
	Fast,
	/// Also compact within a block when average allocation/free sizes justify the copy.
	#[default]
	Balanced,
	/// Also try every eligible source at a strictly lower offset within its own block.
	Full,
	/// Evacuate blocks, pack compatible resource classes, then compact remaining gaps.
	/// Requires a persistent `DefragmentationContext`; no native allocation/copy is implicit.
	Extensive,
}

/// Bounded relocation planning. Zero limits produce an empty pass.
#[derive(Clone, Copy, Debug)]
pub struct DefragmentationInfo<'pool> {
	pub pool: Option<&'pool Pool>,
	pub max_bytes: u64,
	pub max_allocations: usize,
	pub algorithm: DefragmentationAlgorithm,
}

impl Default for DefragmentationInfo<'_> {
	fn default() -> Self {
		Self {
			pool: None,
			max_bytes: u64::MAX,
			max_allocations: usize::MAX,
			algorithm: DefragmentationAlgorithm::default(),
		}
	}
}

/// Allocator-recorded primary resource, or raw storage, associated with a move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefragmentationResource {
	Buffer(vk::Buffer),
	Image(vk::Image),
	/// Raw storage without an allocator-recorded resource. External bindings and
	/// all resources using this storage remain the caller's responsibility.
	Unassociated,
}

/// Borrow-free binding snapshots. They do not retain resources or mappings.
/// Mapped pointers describe counted holds owned by the pass's live regions.
/// Source pointers expire on commit/destroy, destination pointers on ignore/destroy;
/// copied snapshots extend no lifetime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefragmentationMove {
	pub source: AllocationInfo,
	pub destination: AllocationInfo,
	pub resource: DefragmentationResource,
	/// The whole alias group must be recreated, copied and retired together.
	/// `resource` identifies only the recorded primary resource, if any.
	pub can_alias: bool,
}

/// Actual completed work; proposed or ignored moves are not counted as moved bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DefragmentationStats {
	pub moved_allocations: usize,
	pub moved_bytes: u64,
	pub freed_blocks: usize,
	pub freed_bytes: u64,
	pub ignored_allocations: usize,
	pub destroyed_allocations: usize,
}

struct Move {
	source_serial: u64,
	proposal: DefragmentationMove,
	destination: Option<Allocation>,
}

/// Exclusively borrows the allocator while replacement resources are prepared.
///
/// Create/bind replacements, copy and complete GPU work, update consumer native
/// references, destroy each old resource, then commit its move. Ignore/abort needs
/// replacement resources destroyed first. `finish` requires every move resolved.
/// Alias groups additionally require all members recreated and retired together.
/// Source and destination mapping holds stay distinct until resolution. Old
/// source pointers expire on commit; destination pointers expire on ignore/destroy.
/// Drop does no native work: omitting explicit resolution can leak reservations.
pub struct DefragmentationPass<'allocator, 'device> {
	allocator: &'allocator mut Allocator<'device>,
	moves: HostVec<'device, Move>,
	stats: DefragmentationStats,
	finished: bool,
	interrupted: bool,
	context: Option<&'allocator mut ContextState<'device>>,
}

/// Reserve earlier free storage without creating native resources, growing blocks or copying data.
///
/// Pinned, imported/exported, separate, disjoint-plane and linear-pool storage
/// is excluded. Raw, aliasable and mapped storage retains its placement class
/// and counted mapping holds. Every affected alias and pointer is caller-owned.
/// Optional custom-pool identity, native type, priority, alignment and granularity
/// remain fixed. Source metadata and handles remain unchanged during planning.
/// # Errors
/// Returns identity or host metadata errors; failed planning
/// frees its unbound destinations, preserving source resources and live tokens.
/// # Safety
/// Source resources must remain live and ordinary on this device. Caller owns
/// GPU completion and must explicitly commit or ignore every proposed move.
/// End old mapped-pointer uses and recreate every alias before committing a move.
/// All resources bound outside this allocator are part of that same obligation.
/// Raw/alias placement retains the recorded alignment, not undisclosed stronger
/// resource alignment; supply the complete constraint at allocation or pin it.
pub unsafe fn begin_defragmentation<'allocator, 'device>(
	allocator: &'allocator mut Allocator<'device>,
	info: &DefragmentationInfo,
) -> Result<DefragmentationPass<'allocator, 'device>, AllocError> {
	// SAFETY: caller provides the canonical source/pointer/alias and completion contract.
	unsafe { begin_defragmentation_with_break(allocator, info, &mut || false) }
}

/// Plan a pass with a borrowed cooperative stop predicate, following VMA's
/// `pfnBreakCallback` / `CheckCounters` contract. Returning true stops scheduling;
/// already proposed moves still need explicit resolution. Use `was_interrupted`
/// to distinguish a stop from an exhausted search. Polls occur between source
/// candidates, after host snapshot/preflight, and never during consumer copies.
/// A panic rolls back unpublished destinations before unwinding (with panic=unwind).
/// The callback is used only during this call and cannot borrow this allocator.
/// # Safety
/// Same resource, alias, mapped-pointer and completion contract as `begin_defragmentation`.
pub unsafe fn begin_defragmentation_with_break<'allocator, 'device>(
	allocator: &'allocator mut Allocator<'device>,
	info: &DefragmentationInfo,
	should_break: &mut dyn FnMut() -> bool,
) -> Result<DefragmentationPass<'allocator, 'device>, AllocError> {
	allocator.ensure_global()?;
	if info.algorithm == DefragmentationAlgorithm::Extensive {
		return Err(AllocError::UnsupportedCapability(
			"Extensive relocation requires DefragmentationContext",
		));
	}
	// Capture fallible scheduling metadata before any destination reservation.
	let mut state = PlanningState::new(allocator, info)?;
	// SAFETY: identical caller-owned source/move completion contract; no persistent exclusions.
	unsafe { plan(allocator, info, &[], &mut state, should_break) }
}

// A vector in VMA is fixed to one memory type/pool. Explicit Rust block-size
// policies additionally have distinct reuse domains; never merge them here.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Domain {
	memory_type_index: u32,
	pool: Option<u64>,
	automatic: bool,
	priority: u32,
	block_size: u64,
}

fn domain(block: &Block) -> Domain {
	Domain {
		memory_type_index: block.memory_type_index,
		pool: block.pool,
		automatic: block.automatic,
		priority: block.priority.get().to_bits(),
		block_size: if block.automatic { 0 } else { block.size },
	}
}

#[derive(Clone, Copy)]
struct OrderedBlock {
	handle: vk::DeviceMemory,
	domain: Domain,
	free_bytes: u64,
	rank: usize,
	immovable: bool,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct BalancedState {
	average_allocation: u64,
	average_free: u64,
}

#[derive(Clone, Copy)]
struct DomainState {
	key: Domain,
	balanced: BalancedState,
	extensive: ExtensiveState,
}

struct PlanningState<'device> {
	blocks: HostVec<'device, OrderedBlock>,
	domains: HostVec<'device, DomainState>,
}

impl<'device> PlanningState<'device> {
	fn new(allocator: &Allocator<'device>, info: &DefragmentationInfo) -> Result<Self, AllocError> {
		allocator.ensure_open()?;
		if let Some(pool) = info.pool
			&& pool::validate(allocator, pool)?.info.algorithm != AllocationAlgorithm::Tlsf
		{
			return Err(AllocError::UnsupportedCapability(
				"linear pools cannot defragment",
			));
		}
		let mut blocks = HostVec::new(allocator.host);
		blocks
			.try_reserve(allocator.blocks.len())
			.map_err(|_| AllocError::HostAllocationFailed)?;
		for (rank, block) in allocator.blocks.iter().enumerate() {
			if info
				.pool
				.is_some_and(|pool| block.pool != Some(pool.serial))
				|| block.export_types.as_raw() != 0
				|| !matches!(block.metadata, Metadata::Tlsf(_))
			{
				continue;
			}
			blocks.push_reserved(OrderedBlock {
				handle: block.handle,
				domain: domain(block),
				free_bytes: block.metadata.statistics().free_bytes,
				rank,
				immovable: false,
			});
		}
		// Donor SortByFreeSize: fuller compatible blocks are destinations first.
		// The original rank breaks ties without depending on native handle values.
		blocks.sort_unstable_by_key(|block| (block.domain, block.free_bytes, block.rank));
		for (rank, block) in blocks.iter_mut().enumerate() {
			block.rank = rank;
		}
		let mut domains: HostVec<'device, DomainState> = HostVec::new(allocator.host);
		domains
			.try_reserve(blocks.len())
			.map_err(|_| AllocError::HostAllocationFailed)?;
		for block in &blocks {
			if domains.last().is_none_or(|state| state.key != block.domain) {
				domains.push_reserved(DomainState {
					key: block.domain,
					balanced: vector_statistics(allocator, block.domain),
					extensive: ExtensiveState::default(),
				});
			}
		}
		Ok(Self { blocks, domains })
	}
}

fn vector_statistics(allocator: &Allocator<'_>, key: Domain) -> BalancedState {
	// Aggregate in u128: valid per-block extents must not wrap a vector's totals.
	let (mut allocated, mut free, mut allocations, mut regions) = (0_u128, 0_u128, 0_u128, 0_u128);
	for block in &allocator.blocks {
		if domain(block) == key {
			let stats = block.metadata.statistics();
			allocated += u128::from(stats.allocated_bytes);
			free += u128::from(stats.free_bytes);
			allocations += stats.allocation_count as u128;
			regions += stats.free_region_count as u128;
		}
	}
	BalancedState {
		average_allocation: allocated.checked_div(allocations).unwrap_or(0) as u64,
		average_free: free.checked_div(regions).unwrap_or(0) as u64,
	}
}

fn worthwhile_same_block(
	allocator: &Allocator<'_>,
	source: &Allocation,
	averages: BalancedState,
) -> bool {
	let block = allocator
		.blocks
		.iter()
		.find(|block| block.handle == source.handle)
		.expect("live source block");
	let mut previous = 0;
	let mut next = 0;
	block.metadata.for_each_free(|offset, size| {
		if offset + size == source.offset {
			previous = size;
		}
		if offset == source.offset + source.reserved_size {
			next = size;
		}
	});
	previous >= averages.average_free / 2
		|| next >= averages.average_free / 2
		|| source.size <= averages.average_free
		|| source.size <= averages.average_allocation
}

// All bounds/atom/guard/alignment/class extents were checked on source creation.
// Reserve exactly that extent in one explicit compatible block. A same-block
// miss is undone directly before publication, mapping events or debug effects.
fn reserve_destination(
	allocator: &mut Allocator<'_>,
	source: &Allocation,
	target: vk::DeviceMemory,
) -> Result<Option<Allocation>, AllocError> {
	let live = allocator.validate(source)?;
	let alignment = live.alignment;
	let class = live.class;
	let pool = live.pool;
	let can_alias = live.can_alias;
	let guard_size = live.guard_size;
	let guard_offset = live.guard_offset;
	allocator.reserve()?;
	let block = allocator
		.blocks
		.iter_mut()
		.find(|block| block.handle == target)
		.expect("snapshot target block");
	let placement = match block.metadata.allocate_resource(
		source.reserved_size,
		alignment,
		super::super::PlacementStrategy::MinOffset,
		false,
		class,
	) {
		Ok(placement) => placement,
		Err(AllocError::OutOfSpace) => return Ok(None),
		Err(error) => return Err(error),
	};
	let offset = block
		.metadata
		.info(placement)
		.expect("new destination placement")
		.offset;
	if target == source.handle && offset >= source.offset {
		block
			.metadata
			.free(placement)
			.expect("unpublished same-block placement");
		return Ok(None);
	}
	let size = block.size;
	let allocation = publish(
		allocator,
		Region {
			handle: target,
			index: source.memory_type_index,
			size: source.size,
			reserved_size: source.reserved_size,
			offset,
			alignment,
			block_size: size,
			placement: Some(placement),
		},
	);
	let live = allocator
		.live
		.get_mut(&allocation.serial)
		.expect("new destination");
	live.pool = pool;
	live.class = class;
	live.can_alias = can_alias;
	live.guard_size = guard_size;
	live.guard_offset = guard_offset;
	Ok(Some(allocation))
}

#[derive(Clone, Copy)]
struct Source {
	rank: usize,
	offset: u64,
	serial: u64,
}

struct PassCounters<'callback> {
	remaining: u64,
	oversized: u8,
	should_break: &'callback mut dyn FnMut() -> bool,
	interrupted: bool,
	panic: Option<Box<dyn std::any::Any + Send>>,
}

// Donor CheckCounters bounds consecutive oversized-source scans per pass.
const MAX_OVERSIZED_ALLOCATIONS: u8 = 16;

struct Scan<'a> {
	blocks: &'a [OrderedBlock],
	sources: &'a [Source],
	max_allocations: usize,
}

#[derive(Clone, Copy)]
enum Within {
	Never,
	Full,
	Balanced(BalancedState),
}

#[derive(Clone, Copy)]
enum ResourceGroup {
	Textures,
	Buffers,
	All,
}

impl ResourceGroup {
	fn admits(self, class: ResourceClass) -> bool {
		match self {
			Self::Textures => class == ResourceClass::OptimalImage,
			Self::Buffers => matches!(class, ResourceClass::Buffer | ResourceClass::LinearImage),
			Self::All => true,
		}
	}
}

struct Selection {
	sources: std::ops::Range<usize>,
	targets: std::ops::Range<usize>,
	earlier_only: bool,
	within: Within,
	group: ResourceGroup,
}

impl PassCounters<'_> {
	fn exhausted(&self, max_allocations: usize, moves: usize) -> bool {
		self.interrupted
			|| moves == max_allocations
			|| self.remaining == 0
			|| self.oversized == MAX_OVERSIZED_ALLOCATIONS
	}

	fn poll(&mut self) -> bool {
		match std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut self.should_break)) {
			Ok(false) => false,
			Ok(true) => {
				self.interrupted = true;
				true
			}
			Err(payload) => {
				self.interrupted = true;
				self.panic = Some(payload);
				true
			}
		}
	}
}

// Destination holds exist before publication, so a failed map or later planner
// failure never invalidates a source pointer. Same-block moves share one native
// mapping; different-block moves may fail their single pre-commit map attempt.
unsafe fn prepare_destination(
	allocator: &mut Allocator<'_>,
	source: &Allocation,
	destination: &Allocation,
) -> Result<(), AllocError> {
	// SAFETY: new unbound reservation, without consumer data or live pointer uses.
	unsafe { guard::initialize(allocator, destination) }?;
	let live = allocator.validate(source)?;
	let map_count = live.map_count;
	let persistent = live.persistent;
	let holds = map_count
		.checked_add(u32::from(persistent))
		.ok_or(AllocError::InvalidRange)?;
	if holds != 0 {
		let device = allocator.device.clone();
		// SAFETY: host-visible source implies the identical native type is visible;
		// exclusive allocator borrow serializes mapping and no destination uses exist.
		unsafe {
			allocator.mapping_mut(destination).acquire_many(
				device.as_device(),
				destination.handle,
				destination.block_size,
				holds,
			)
		}?;
		let live = allocator
			.live
			.get_mut(&destination.serial)
			.expect("new destination");
		live.map_count = map_count;
		live.persistent = persistent;
	}
	Ok(())
}

unsafe fn record_move(
	allocator: &mut Allocator<'_>,
	source: &Allocation,
	mut destination: Allocation,
	moves: &mut HostVec<'_, Move>,
	counters: &mut PassCounters,
) -> Result<(), AllocError> {
	// SAFETY: new unbound destination; debug setup and duplicated mapping holds only.
	if let Err(error) = unsafe { prepare_destination(allocator, source, &destination) } {
		// SAFETY: unpublished reservation, no replacement/copy or user mappings exist.
		unsafe { rollback_allocation(allocator, &mut destination) }
			.expect("debug destination rollback");
		return Err(error);
	}
	let live = &allocator.live[&source.serial];
	let resource = live.resource;
	let can_alias = live.can_alias;
	let destination_info =
		allocation_info(allocator, &destination).expect("new destination identity");
	let source_info = allocation_info(allocator, source).expect("source unchanged during planning");
	counters.remaining -= source.size;
	moves.push_reserved(Move {
		source_serial: source.serial,
		destination: Some(destination),
		proposal: DefragmentationMove {
			source: source_info,
			destination: destination_info,
			resource: match resource {
				Some(Resource::Buffer(buffer)) => DefragmentationResource::Buffer(buffer),
				Some(Resource::Image(image)) => DefragmentationResource::Image(image),
				None => DefragmentationResource::Unassociated,
			},
			can_alias,
		},
	});
	Ok(())
}

unsafe fn plan_selected(
	allocator: &mut Allocator<'_>,
	scan: &Scan<'_>,
	selection: &Selection,
	moves: &mut HostVec<'_, Move>,
	counters: &mut PassCounters,
) -> Result<(), AllocError> {
	for source in scan
		.sources
		.iter()
		.filter(|source| selection.sources.contains(&source.rank))
	{
		if counters.exhausted(scan.max_allocations, moves.len()) {
			break;
		}
		if moves
			.iter()
			.any(|movement| movement.source_serial == source.serial)
		{
			continue;
		}
		if counters.poll() {
			break;
		}
		let token = allocator.token(source.serial)?;
		if token.size > counters.remaining {
			counters.oversized += 1;
			continue;
		}
		counters.oversized = 0;
		if !selection.group.admits(allocator.live[&source.serial].class) {
			continue;
		}
		let mut destination = None;
		for rank in selection.targets.clone() {
			let block = &scan.blocks[rank];
			if block.handle == token.handle
				|| block.domain != scan.blocks[source.rank].domain
				|| selection.earlier_only && rank >= source.rank
			{
				continue;
			}
			if let Some(allocation) = reserve_destination(allocator, &token, block.handle)? {
				destination = Some(allocation);
				break;
			}
		}
		if destination.is_none()
			&& token.offset != 0
			&& match selection.within {
				Within::Never => false,
				Within::Full => true,
				Within::Balanced(averages) => worthwhile_same_block(allocator, &token, averages),
			} {
			destination = reserve_destination(allocator, &token, token.handle)?;
		}
		if let Some(destination) = destination {
			// SAFETY: private new reservation; identical source/move completion contract.
			unsafe { record_move(allocator, &token, destination, moves, counters) }?;
		}
	}
	Ok(())
}

unsafe fn plan<'allocator, 'device>(
	allocator: &'allocator mut Allocator<'device>,
	info: &DefragmentationInfo,
	ignored: &[u64],
	state: &mut PlanningState<'device>,
	should_break: &mut dyn FnMut() -> bool,
) -> Result<DefragmentationPass<'allocator, 'device>, AllocError> {
	allocator.ensure_open()?;
	let mut blocks = HostVec::new(allocator.host);
	blocks
		.try_reserve(state.blocks.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for block in &state.blocks {
		if allocator
			.blocks
			.iter()
			.any(|live| live.handle == block.handle)
		{
			blocks.push_reserved(OrderedBlock {
				immovable: ignored.iter().any(|serial| {
					allocator
						.live
						.get(serial)
						.is_some_and(|live| live.handle == block.handle)
				}),
				..*block
			});
		}
	}
	let mut domains = HostVec::new(allocator.host);
	domains
		.try_reserve(state.domains.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	domains.extend_from_slice_reserved(&state.domains);
	// Donor EndPass moves blocks containing an ignored source to the immovable
	// prefix. They remain destination storage but no longer supply sources.
	if info.algorithm == DefragmentationAlgorithm::Extensive && allocator.granularity.0 != Some(1) {
		extensive::exclude_ignored(&mut blocks, &mut domains);
	} else {
		blocks.sort_unstable_by_key(|block| (block.domain, !block.immovable, block.rank));
	}
	let mut sources = HostVec::new(allocator.host);
	sources
		.try_reserve(allocator.live.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for (&serial, live) in &allocator.live {
		if live.plane.is_some() || live.imported || live.pinned || live.placement.is_none() {
			continue;
		}
		if let Some(rank) = blocks
			.iter()
			.position(|block| block.handle == live.handle && !block.immovable)
		{
			sources.push_reserved(Source {
				rank,
				offset: live.offset,
				serial,
			});
		}
	}
	sources.sort_unstable_by(|left, right| {
		right
			.rank
			.cmp(&left.rank)
			.then(left.offset.cmp(&right.offset))
			.then(left.serial.cmp(&right.serial))
	});
	let mut moves = HostVec::new(allocator.host);
	moves
		.try_reserve(sources.len().min(info.max_allocations))
		.map_err(|_| AllocError::HostAllocationFailed)?;
	let mut counters = PassCounters {
		remaining: info.max_bytes,
		oversized: 0,
		should_break,
		interrupted: false,
		panic: None,
	};
	let planned = (|| -> Result<(), AllocError> {
		for domain in &mut domains {
			if counters.exhausted(info.max_allocations, moves.len()) {
				break;
			}
			let Some(start) = blocks.iter().position(|block| block.domain == domain.key) else {
				continue;
			};
			let end = blocks[start..]
				.iter()
				.position(|block| block.domain != domain.key)
				.map_or(blocks.len(), |length| start + length);
			let scan = Scan {
				blocks: &blocks,
				sources: &sources,
				max_allocations: info.max_allocations,
			};
			if info.algorithm == DefragmentationAlgorithm::Extensive && allocator.granularity.0 != Some(1)
			{
				// SAFETY: same selected sources and explicit alias/pointer/move-resolution contract.
				unsafe {
					extensive::plan(
						allocator,
						&scan,
						start..end,
						&mut domain.extensive,
						&mut moves,
						&mut counters,
					)
				}?;
				continue;
			}
			let within = match info.algorithm {
				DefragmentationAlgorithm::Fast => Within::Never,
				DefragmentationAlgorithm::Balanced => Within::Balanced(domain.balanced),
				DefragmentationAlgorithm::Full | DefragmentationAlgorithm::Extensive => Within::Full,
			};
			let mut selection = Selection {
				sources: start..end,
				targets: start..end,
				earlier_only: true,
				within,
				group: ResourceGroup::All,
			};
			let before = moves.len();
			// SAFETY: selected live sources; reservations remain unpublished to the consumer.
			unsafe { plan_selected(allocator, &scan, &selection, &mut moves, &mut counters) }?;
			if before == moves.len()
				&& info.algorithm == DefragmentationAlgorithm::Balanced
				&& !counters.exhausted(info.max_allocations, moves.len())
			{
				let refreshed = vector_statistics(allocator, domain.key);
				if refreshed != domain.balanced {
					domain.balanced = refreshed;
					selection.within = Within::Balanced(refreshed);
					// SAFETY: no reservations were made by this domain's first scan.
					unsafe { plan_selected(allocator, &scan, &selection, &mut moves, &mut counters) }?;
				}
			}
		}
		Ok(())
	})();
	if planned.is_err() || counters.panic.is_some() {
		for movement in &mut moves {
			// SAFETY: still unbound unpublished destinations; source objects/tokens are unchanged.
			unsafe {
				rollback_allocation(
					allocator,
					movement.destination.as_mut().expect("unpublished move"),
				)
			}
			.expect("planning rollback");
		}
		if let Some(payload) = counters.panic.take() {
			std::panic::resume_unwind(payload);
		}
		planned?;
	}
	state.blocks = blocks;
	state.domains = domains;
	Ok(DefragmentationPass {
		allocator,
		moves,
		stats: DefragmentationStats::default(),
		finished: false,
		interrupted: counters.interrupted,
		context: None,
	})
}

impl DefragmentationPass<'_, '_> {
	/// Proposed move count, including moves already explicitly resolved.
	pub fn len(&self) -> usize {
		self.moves.len()
	}

	pub fn is_empty(&self) -> bool {
		self.moves.is_empty()
	}

	/// The stop predicate returned true. An interrupted empty pass does not mean
	/// compaction is complete; a later context pass can resume scheduling.
	pub fn was_interrupted(&self) -> bool {
		self.interrupted
	}

	/// Borrow a proposal's stable original snapshots; no native ownership is acquired.
	pub fn movement(&self, index: usize) -> Result<&DefragmentationMove, AllocError> {
		Ok(
			&self
				.moves
				.get(index)
				.ok_or(AllocError::InvalidRange)?
				.proposal,
		)
	}

	/// Access the same borrowed device for explicit replacement/copy/completion operations.
	pub fn device(&self) -> &vk::Device {
		self.allocator.device.as_device()
	}

	/// Retained native callbacks for explicit old/replacement resource destruction.
	pub fn allocation_callbacks(&self) -> Option<&vk::AllocationCallbacks> {
		self.allocator.callbacks
	}

	/// Bind a caller-created replacement buffer to this move's reserved destination.
	/// # Safety
	/// Replacement must satisfy the source requirements and native valid usage;
	/// no copy/completion/destruction is performed here.
	pub unsafe fn bind_buffer(&mut self, index: usize, buffer: vk::Buffer) -> Result<(), AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		let movement = self.moves.get(index).ok_or(AllocError::InvalidRange)?;
		if !matches!(
			movement.proposal.resource,
			DefragmentationResource::Buffer(_) | DefragmentationResource::Unassociated
		) {
			return Err(AllocError::WrongResource);
		}
		let destination = movement
			.destination
			.as_ref()
			.ok_or(AllocError::UnknownAllocation)?;
		// SAFETY: caller supplies an ordinary matching unbound replacement and retains its lifetime.
		unsafe { bind_buffer_memory(self.allocator, buffer, destination) }
	}

	/// Bind a caller-created replacement image to the reserved destination.
	/// # Safety
	/// Caller supplies matching ordinary image requirements/layout/tiling and valid enabled features.
	pub unsafe fn bind_image(&mut self, index: usize, image: vk::Image) -> Result<(), AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		let movement = self.moves.get(index).ok_or(AllocError::InvalidRange)?;
		if !matches!(
			movement.proposal.resource,
			DefragmentationResource::Image(_) | DefragmentationResource::Unassociated
		) {
			return Err(AllocError::WrongResource);
		}
		let destination = movement
			.destination
			.as_ref()
			.ok_or(AllocError::UnknownAllocation)?;
		// SAFETY: caller supplies a matching ordinary unbound replacement and owns native completion.
		unsafe { bind_image_memory(self.allocator, image, destination) }
	}

	/// Recreate one buffer alias on this move's destination, without allocating memory.
	/// Binding/requirements checks and native failure cleanup use the ordinary alias owner.
	/// The returned resource is caller-owned and does not replace a recorded primary.
	/// # Safety
	/// Same native creation/alias/barrier contract as `mem::create_aliasing_buffer`.
	/// Recreate every group member at its retained local offset; complete copies,
	/// update references and destroy all old aliases before committing the move.
	pub unsafe fn create_aliasing_buffer(
		&mut self,
		index: usize,
		local_offset: u64,
		info: &vk::BufferCreateInfo,
	) -> Result<vk::Buffer, AllocError> {
		let destination = self.destination(index)?;
		let token = self.allocator.token(destination.serial)?;
		// SAFETY: caller supplies the canonical alias creation/lifetime contract.
		unsafe { create_aliasing_buffer(self.allocator, &token, local_offset, info) }
	}

	/// Recreate one image alias on the reserved destination, preserving ALIAS_BIT
	/// and the ordinary alias owner's checked requirements/offset/type admission.
	/// # Safety
	/// Same group completion/reference/destruction contract as `create_aliasing_buffer`,
	/// plus `mem::create_aliasing_image` native image/layout requirements.
	pub unsafe fn create_aliasing_image(
		&mut self,
		index: usize,
		local_offset: u64,
		info: &vk::ImageCreateInfo,
	) -> Result<vk::Image, AllocError> {
		let destination = self.destination(index)?;
		let token = self.allocator.token(destination.serial)?;
		// SAFETY: caller supplies the canonical image alias/lifetime contract.
		unsafe { create_aliasing_image(self.allocator, &token, local_offset, info) }
	}

	fn destination(&self, index: usize) -> Result<&Allocation, AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		self
			.moves
			.get(index)
			.ok_or(AllocError::InvalidRange)?
			.destination
			.as_ref()
			.ok_or(AllocError::UnknownAllocation)
	}

	/// Replace the caller's source token after completed replacement and old-resource destruction.
	///
	/// Keeps copied diagnostic name, opaque user data and pinning metadata. Publishes
	/// the already bound destination and frees the old placement; no resource is
	/// created/destroyed and no GPU work is submitted or waited on.
	/// # Errors
	/// Rejects wrong/stale source, unresolved primary destination binding, or a
	/// mapping/guard/native failure. Raw storage may commit without a recorded binding.
	/// # Safety
	/// Copy/initialization must be complete. Destroy the old source resource first;
	/// update all dependent descriptors, addresses, views and consumer references
	/// before subsequent use. Caller owns the replacement's identity/callback lifetime.
	/// For aliasable storage retire the entire old alias group. End all uses of old
	/// mapped pointers before this call; counted explicit/persistent holds transfer
	/// to the destination. Obtain its new pointer from the destination snapshot or
	/// `mapped_pointer` after the context ends. Old pointers never update themselves.
	pub unsafe fn commit_move(
		&mut self,
		index: usize,
		source: &mut Allocation,
	) -> Result<(), AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		let movement = self.moves.get_mut(index).ok_or(AllocError::InvalidRange)?;
		if source.serial != movement.source_serial {
			return Err(AllocError::WrongResource);
		}
		let old = self.allocator.validate(source)?;
		let destination = movement
			.destination
			.as_ref()
			.ok_or(AllocError::UnknownAllocation)?;
		if (old.bound || old.resource.is_some()) && !self.allocator.validate(destination)?.bound {
			return Err(AllocError::InvalidRequest("replacement is not bound"));
		}
		let user_data = old.user_data;
		let pinned = old.pinned;
		let name = self
			.allocator
			.live
			.get_mut(&source.serial)
			.expect("validated source")
			.name
			.take();
		let old_serial = source.serial;
		let before = statistics(self.allocator);
		// SAFETY: caller retired old resources, aliases and pointer uses after exact completion.
		if let Err(error) = unsafe { free_region(self.allocator, source, true) } {
			self
				.allocator
				.live
				.get_mut(&old_serial)
				.expect("retained failed source")
				.name = name;
			return Err(error);
		}
		let destination = movement.destination.take().expect("validated destination");
		let live = self
			.allocator
			.live
			.get_mut(&destination.serial)
			.expect("live destination");
		live.name = name;
		live.user_data = user_data;
		live.pinned = pinned;
		*source = destination;
		let after = statistics(self.allocator);
		self.stats.moved_allocations += 1;
		self.stats.moved_bytes += source.size;
		self.stats.freed_blocks += before.native_allocation_count - after.native_allocation_count;
		self.stats.freed_bytes += before.native_bytes - after.native_bytes;
		Ok(())
	}

	/// Release one unused destination, retaining its original source unchanged.
	/// # Safety
	/// Destroy every bound replacement/alias and complete all its uses first.
	/// End every destination mapped-pointer use; source mapping holds stay intact.
	pub unsafe fn ignore_move(&mut self, index: usize) -> Result<(), AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		let movement = self.moves.get_mut(index).ok_or(AllocError::InvalidRange)?;
		let destination = movement
			.destination
			.as_mut()
			.ok_or(AllocError::UnknownAllocation)?;
		let before = statistics(self.allocator);
		// SAFETY: caller ends all replacement uses and destroys any replacement object.
		unsafe { free_region(self.allocator, destination, true) }?;
		let after = statistics(self.allocator);
		self.stats.freed_blocks += before.native_allocation_count - after.native_allocation_count;
		self.stats.freed_bytes += before.native_bytes - after.native_bytes;
		movement.destination = None;
		if let Some(context) = self.context.as_mut() {
			// Capacity for every proposed move was reserved before native planning.
			if !context.ignored.contains(&movement.source_serial) {
				context.ignored.push_reserved(movement.source_serial);
			}
		}
		self.stats.ignored_allocations += 1;
		Ok(())
	}

	/// Discard a source allocation instead of moving it, releasing both reservations.
	/// # Errors
	/// Rejects stale/wrong source, resolved moves or guard/native release failures.
	/// # Safety
	/// Destroy the old source and any bound replacement resource first and complete
	/// all their uses. Caller must discard every dependent descriptor/address/view.
	/// Destroy every alias and end all source/destination mapped-pointer uses;
	/// counted mapping holds are explicitly retired with their storage.
	pub unsafe fn destroy_move(
		&mut self,
		index: usize,
		source: &mut Allocation,
	) -> Result<(), AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		let movement = self.moves.get_mut(index).ok_or(AllocError::InvalidRange)?;
		if source.serial != movement.source_serial {
			return Err(AllocError::WrongResource);
		}
		self.allocator.validate(source)?;
		let destination = movement
			.destination
			.as_mut()
			.ok_or(AllocError::UnknownAllocation)?;
		self.allocator.validate(destination)?;
		// Diagnose/poison both completed regions before either token is retired.
		// A guard or map failure must leave this move resolvable, including all
		// counted mapping holds. Successful preflight need not run twice in free.
		// SAFETY: caller ended every old/replacement alias, host and GPU use.
		unsafe {
			guard::before_free(self.allocator, destination)?;
			guard::before_free(self.allocator, source)?;
		}
		for serial in [destination.serial, source.serial] {
			self
				.allocator
				.live
				.get_mut(&serial)
				.expect("validated move region")
				.debug_ready = false;
		}
		let before = statistics(self.allocator);
		// SAFETY: caller destroyed all old/replacement objects after completion; holds are prevalidated.
		unsafe { free_region(self.allocator, destination, true) }?;
		movement.destination = None;
		// SAFETY: completed source and ended pointer uses; cached native inverses are immutable.
		unsafe { free_region(self.allocator, source, true) }?;
		let after = statistics(self.allocator);
		self.stats.freed_blocks += before.native_allocation_count - after.native_allocation_count;
		self.stats.freed_bytes += before.native_bytes - after.native_bytes;
		self.stats.destroyed_allocations += 1;
		Ok(())
	}

	/// Release every unresolved destination; completed moves remain committed.
	/// # Safety
	/// Destroy all unresolved replacement resources/aliases and complete their uses
	/// first. End every unresolved destination mapped-pointer use before this call.
	pub unsafe fn abort(&mut self) -> Result<DefragmentationStats, AllocError> {
		for index in 0..self.moves.len() {
			if self.moves[index].destination.is_some() {
				// SAFETY: caller provides the same per-replacement completion/destruction contract.
				unsafe { self.ignore_move(index) }?;
			}
		}
		self.finish()
	}

	/// Close the host pass after every proposed move has been committed or ignored.
	pub fn finish(&mut self) -> Result<DefragmentationStats, AllocError> {
		if self.finished {
			return Ok(self.stats);
		}
		if self
			.moves
			.iter()
			.any(|movement| movement.destination.is_some())
		{
			return Err(AllocError::LiveAllocations);
		}
		if let Some(context) = self.context.as_mut() {
			context.stats = add_statistics(context.stats, self.stats)?;
			context.active = false;
		}
		self.finished = true;
		Ok(self.stats)
	}
}

/// Explicitly pin/unpin a region against future allocator relocation planning.
pub fn set_allocation_pinned(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	pinned: bool,
) -> Result<(), AllocError> {
	allocator.validate(allocation)?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("validated allocation")
		.pinned = pinned;
	Ok(())
}

impl<'device> Allocator<'device> {
	/// Begin an explicitly resolved relocation pass over eligible ordinary TLSF allocations.
	/// # Safety
	/// Same native resource and explicit completion/commit contract as `mem::begin_defragmentation`.
	pub unsafe fn begin_defragmentation<'allocator>(
		&'allocator mut self,
		info: &DefragmentationInfo,
	) -> Result<DefragmentationPass<'allocator, 'device>, AllocError> {
		// SAFETY: caller provides canonical live-resource and move-resolution guarantees.
		unsafe { begin_defragmentation(self, info) }
	}

	/// Plan with a borrowed stop predicate; interrupted proposals still need resolution.
	/// # Safety
	/// Same source/alias/pointer/completion contract as `mem::begin_defragmentation_with_break`.
	pub unsafe fn begin_defragmentation_with_break<'allocator>(
		&'allocator mut self,
		info: &DefragmentationInfo,
		should_break: &mut dyn FnMut() -> bool,
	) -> Result<DefragmentationPass<'allocator, 'device>, AllocError> {
		// SAFETY: caller provides canonical live-source and explicit resolution guarantees.
		unsafe { begin_defragmentation_with_break(self, info, should_break) }
	}

	/// Prevent or admit an allocation in future explicit relocation passes.
	pub fn set_allocation_pinned(
		&mut self,
		allocation: &Allocation,
		pinned: bool,
	) -> Result<(), AllocError> {
		set_allocation_pinned(self, allocation, pinned)
	}
}

struct ContextState<'device> {
	ignored: HostVec<'device, u64>,
	stats: DefragmentationStats,
	active: bool,
}

/// Persistent multi-pass relocation state with one exclusive allocator borrow.
/// Ignored allocations stay immovable for subsequent passes in this context.
/// Each pass must be explicitly resolved and finished; neither context nor pass
/// Drop submits, waits or destroys native resources. An abandoned unresolved pass
/// leaks its reservations, as documented by `DefragmentationPass`.
///
/// Initial per-domain order, Balanced averages and Extensive evacuation/packing
/// phases persist across passes. Extensive holds its blocks for reuse until
/// explicit `finish`; Drop clears host holds but never frees native memory.
pub struct DefragmentationContext<'allocator, 'device> {
	allocator: &'allocator mut Allocator<'device>,
	pool: Option<Pool>,
	max_bytes: u64,
	max_allocations: usize,
	algorithm: DefragmentationAlgorithm,
	planning: PlanningState<'device>,
	state: ContextState<'device>,
	finished: bool,
}

impl<'allocator, 'device> DefragmentationContext<'allocator, 'device> {
	/// Retain one allocator and optional TLSF pool for successive bounded passes.
	/// No placement reservation or native effect occurs until `begin_pass`.
	/// # Errors
	/// Rejects closed/foreign/stale pools, linear-pool relocation and host metadata failure.
	pub fn new(
		allocator: &'allocator mut Allocator<'device>,
		info: &DefragmentationInfo,
	) -> Result<Self, AllocError> {
		allocator.ensure_global()?;
		allocator.ensure_open()?;
		let pool = if let Some(pool) = info.pool {
			if pool::validate(allocator, pool)?.info.algorithm != AllocationAlgorithm::Tlsf {
				return Err(AllocError::UnsupportedCapability(
					"linear pools cannot defragment",
				));
			}
			Some(Pool {
				owner: allocator.owner,
				serial: pool.serial,
			})
		} else {
			None
		};
		let planning = PlanningState::new(allocator, info)?;
		if info.algorithm == DefragmentationAlgorithm::Extensive {
			for block in &mut allocator.blocks {
				if planning
					.blocks
					.iter()
					.any(|selected| selected.handle == block.handle)
				{
					block.defragmentation_hold = true;
				}
			}
		}
		let state = ContextState {
			ignored: HostVec::new(allocator.host),
			stats: DefragmentationStats::default(),
			active: false,
		};
		Ok(Self {
			allocator,
			pool,
			max_bytes: info.max_bytes,
			max_allocations: info.max_allocations,
			algorithm: info.algorithm,
			planning,
			state,
			finished: false,
		})
	}

	/// Reserve another bounded pass, excluding previously ignored source identities.
	/// Extensive may move into a later evacuated block to pack compatible classes.
	/// `is_empty` means this pass proposed no move under its policy and copy limits.
	/// # Errors
	/// Preserves policy/host metadata/planning errors without changing source storage.
	/// # Safety
	/// Same live-source, completed-copy and explicit move-resolution obligations as
	/// `mem::begin_defragmentation`. Finish/abort every pass before beginning another.
	pub unsafe fn begin_pass(&mut self) -> Result<DefragmentationPass<'_, 'device>, AllocError> {
		// SAFETY: identical caller-owned source/alias/pointer and explicit completion contract.
		unsafe { self.begin_pass_with_break(&mut || false) }
	}

	/// Plan the next pass with a stop predicate borrowed only for this call.
	/// True preserves already proposed moves for resolution; `was_interrupted`
	/// distinguishes a stopped empty pass from completed scheduling. A later pass
	/// can use a different predicate; this context retains policy and ignored state.
	/// # Safety
	/// Same source/alias/mapped-pointer and explicit resolution contract as `begin_pass`.
	pub unsafe fn begin_pass_with_break(
		&mut self,
		should_break: &mut dyn FnMut() -> bool,
	) -> Result<DefragmentationPass<'_, 'device>, AllocError> {
		if self.finished {
			return Err(AllocError::Closed);
		}
		if self.state.active {
			return Err(AllocError::LiveAllocations);
		}
		self
			.state
			.ignored
			.try_reserve(self.allocator.live.len().min(self.max_allocations))
			.map_err(|_| AllocError::HostAllocationFailed)?;
		let info = DefragmentationInfo {
			pool: self.pool.as_ref(),
			max_bytes: self.max_bytes,
			max_allocations: self.max_allocations,
			algorithm: self.algorithm,
		};
		// SAFETY: caller supplies the canonical native source/move contract; context excludes persistent ignored identities.
		let mut pass = unsafe {
			plan(
				self.allocator,
				&info,
				&self.state.ignored,
				&mut self.planning,
				should_break,
			)
		}?;
		self.state.active = true;
		pass.context = Some(&mut self.state);
		Ok(pass)
	}

	/// Totals from explicitly finished passes, never uncommitted proposals.
	pub fn statistics(&self) -> DefragmentationStats {
		self.state.stats
	}

	pub fn ignored_allocations(&self) -> usize {
		self.state.ignored.len()
	}

	/// Complete the context after all passes are resolved and finished.
	/// Extensive explicitly releases held empty blocks above pool minimums here;
	/// freed native block/byte counts are included in the returned total statistics.
	pub fn finish(&mut self) -> Result<DefragmentationStats, AllocError> {
		if self.state.active {
			return Err(AllocError::LiveAllocations);
		}
		if !self.finished && self.algorithm == DefragmentationAlgorithm::Extensive {
			let freed = pool::finish_defragmentation_blocks(self.allocator);
			// No native growth is possible through this exclusive context. These
			// cumulative frees are bounded by its initial native count/bytes.
			self.state.stats.freed_blocks += freed.block_count;
			self.state.stats.freed_bytes += freed.block_bytes;
		}
		self.finished = true;
		Ok(self.state.stats)
	}
}

impl Drop for DefragmentationContext<'_, '_> {
	fn drop(&mut self) {
		if self.algorithm == DefragmentationAlgorithm::Extensive {
			for block in &mut self.allocator.blocks {
				if self
					.planning
					.blocks
					.iter()
					.any(|selected| selected.handle == block.handle)
				{
					block.defragmentation_hold = false;
				}
			}
		}
	}
}

fn add_statistics(
	a: DefragmentationStats,
	b: DefragmentationStats,
) -> Result<DefragmentationStats, AllocError> {
	Ok(DefragmentationStats {
		moved_allocations: a
			.moved_allocations
			.checked_add(b.moved_allocations)
			.ok_or(AllocError::InvalidRange)?,
		moved_bytes: a
			.moved_bytes
			.checked_add(b.moved_bytes)
			.ok_or(AllocError::InvalidRange)?,
		freed_blocks: a
			.freed_blocks
			.checked_add(b.freed_blocks)
			.ok_or(AllocError::InvalidRange)?,
		freed_bytes: a
			.freed_bytes
			.checked_add(b.freed_bytes)
			.ok_or(AllocError::InvalidRange)?,
		ignored_allocations: a
			.ignored_allocations
			.checked_add(b.ignored_allocations)
			.ok_or(AllocError::InvalidRange)?,
		destroyed_allocations: a
			.destroyed_allocations
			.checked_add(b.destroyed_allocations)
			.ok_or(AllocError::InvalidRange)?,
	})
}
