//! Fixed-type TLSF/linear pools following pinned VMA 3.4.0 `CreatePool`,
//! `VmaBlockVector::{CreateMinBlocks,AllocatePage,Free}` and `GetPoolStatistics`.
//! See LICENSE-VMA-MIT.
//! Adaptation: checked tokens, fallible metadata, explicit cleanup, and the
//! shared resource-aware placement, atom isolation and native mapping/accounting.

use super::*;
use crate::mem::BudgetSnapshot;

/// Identity of one live custom pool. Dropping it never destroys native storage.
#[derive(Debug)]
pub struct Pool {
	pub(super) owner: Identity,
	pub(super) serial: u64,
}

impl Pool {
	pub fn is_destroyed(&self) -> bool {
		self.serial == 0
	}
}

/// Fixed-type ordinary-memory pool; not a Vulkan ABI record.
///
/// Uses the selected placement algorithm. Priority requires captured enabled
/// EXT feature metadata. Use `create_pool_with_chain` for a retained borrowed native chain.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolCreateInfo {
	pub priority: MemoryPriority,
	pub memory_type_index: u32,
	/// TLSF by default; linear supports explicit free-at-once and stack lifetimes.
	/// Ring and upper-address placement require `max_block_count = 1`.
	pub algorithm: AllocationAlgorithm,
	/// Exact native block size in bytes, or zero for VMA's automatic sizing policy.
	pub block_size: u64,
	/// Preallocate this many blocks transactionally and retain this floor.
	pub min_block_count: usize,
	/// Pooled-block ceiling; zero means unlimited. Automatic pools can additionally
	/// own dedicated allocations, still subject to native allocation/heap limits.
	pub max_block_count: usize,
	/// Zero means no additional constraint; otherwise a power of two.
	pub min_allocation_alignment: u64,
}

/// Local pool statistics. No native budget query or host allocation is performed.
/// Free spans describe physical holes; alignment/granularity may prevent their use.
/// Linear holes additionally wait for their stack/ring lifetime to retire.
/// Counts/bytes include pool-owned dedicated allocations and retained empty blocks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolStats {
	pub allocation_count: usize,
	pub allocation_bytes: u64,
	pub reserved_bytes: u64,
	pub block_count: usize,
	pub block_bytes: u64,
	pub free_bytes: u64,
	pub largest_free_region: u64,
}

pub(super) struct PoolState<'callbacks> {
	pub allocation_chain: super::super::chain::ChainData,
	pub name: Option<HostString<'callbacks>>,
	pub info: PoolCreateInfo,
	pub preferred_block_size: u64,
}

// Donor: CalcAllocationParams, AllocateMemoryOfType and the pool's own dedicated
// list. Fixed pools never dedicate; automatic pools keep that storage in the
// same identity/type/accounting domain rather than escaping to a default pool.
pub(super) unsafe fn allocate(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	storage: StorageOptions<'_>,
	dedication: Dedication,
	options: &AllocationCreateInfo,
	class: ResourceClass,
	avoided: vk::MemoryPropertyFlags,
) -> Result<Allocation, AllocError> {
	let pool = options.pool.expect("selected custom pool");
	let state = validate(allocator, pool)?;
	let info = state.info;
	let preferred_block_size = state.preferred_block_size;
	if !request.alignment.is_power_of_two() {
		return Err(AllocError::InvalidRequest("invalid native alignment"));
	}
	let request = AllocRequest {
		memory_type_bits: request.memory_type_bits & (1 << info.memory_type_index),
		alignment: request.alignment.max(info.min_allocation_alignment),
		..*request
	};
	// Never relax mandatory properties/masks or select outside this fixed type.
	allocator
		.selector
		.find_memory_type_with_preferences(&request, avoided)?;
	let blocks = BlockOptions {
		automatic: info.block_size == 0,
		block_size: preferred_block_size,
		strategy: options.strategy,
		class,
		grow: !options.never_allocate,
		pool: Some(pool.serial),
		upper_address: options.upper_address,
		priority: info.priority,
		within_budget: options.within_budget,
		prefer_mapped: super::super::policy::prefers_mapped(options),
	};
	let allocation = if info.block_size != 0 {
		if dedication == Dedication::Required {
			return Err(AllocError::UnsupportedCapability(
				"fixed-size pools cannot dedicate resources",
			));
		}
		// SAFETY: exact pool/type, queried requirements and checked resource page separation.
		unsafe { allocate_pooled(allocator, &request, avoided, blocks) }?
	} else {
		// SAFETY: identical native obligations; every success is attached below.
		unsafe { allocate_automatic(allocator, &request, storage, dedication, avoided, blocks) }?
	};
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("new pool allocation")
		.pool = Some(pool.serial);
	Ok(allocation)
}

// Pinned VMA AllocateMemoryOfType and AllocatePage: dedicated preference,
// existing blocks first, growth from 1/8 towards the preferred size and at most
// three halving steps. Rust retries only recoverable native/storage failures;
// device loss, missing commands and metadata failures keep their original error.
pub(super) unsafe fn allocate_automatic(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	storage: StorageOptions<'_>,
	dedication: Dedication,
	avoided: vk::MemoryPropertyFlags,
	blocks: BlockOptions,
) -> Result<Allocation, AllocError> {
	if blocks.upper_address && dedication == Dedication::Required {
		return Err(AllocError::UnsupportedCapability(
			"upper address cannot use required dedication",
		));
	}
	if dedication == Dedication::Required {
		if !blocks.grow {
			return Err(AllocError::OutOfSpace);
		}
		// SAFETY: required dedication keeps the exact resource and pool type.
		return unsafe { allocate_storage(allocator, request, storage, avoided) };
	}
	let high_count = allocator.allocation_limit < u32::MAX / 4
		&& allocator.total_native_count() > allocator.allocation_limit as usize * 3 / 4;
	let prefer_dedicated = blocks.grow
		&& !blocks.upper_address
		&& !high_count
		&& (dedication == Dedication::Preferred || request.size > blocks.block_size / 2);
	if prefer_dedicated {
		// SAFETY: matching resource-specific chain and fixed compatible type.
		match unsafe { allocate_storage(allocator, request, storage, avoided) } {
			Ok(allocation) => return Ok(allocation),
			Err(error) if retryable(&error) => {}
			Err(error) => return Err(error),
		}
	}
	// SAFETY: reuse can publish a placement but performs no native block acquisition.
	let mut last = match unsafe {
		allocate_pooled(
			allocator,
			request,
			avoided,
			BlockOptions {
				grow: false,
				..blocks
			},
		)
	} {
		Ok(allocation) => return Ok(allocation),
		Err(error) if retryable(&error) => error,
		Err(error) => return Err(error),
	};
	if !blocks.grow {
		return Err(last);
	}
	let index = request.memory_type_bits.trailing_zeros() as usize;
	let flags = allocator.selector.properties.memory_types[index].property_flags;
	let atom = if flags.as_raw()
		& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT)
			.as_raw()
		== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
	{
		allocator.selector.atom
	} else {
		1
	};
	let reserved = request
		.size
		.checked_add(atom - 1)
		.map(|value| value & !(atom - 1))
		.ok_or(AllocError::InvalidRange)?;
	let max_existing = allocator
		.blocks
		.iter()
		.filter(|block| {
			block.pool == blocks.pool
				&& block.memory_type_index as usize == index
				&& block.priority == blocks.priority
				&& block.automatic == blocks.automatic
		})
		.map(|block| block.size)
		.max()
		.unwrap_or(0);
	let mut size = blocks.block_size;
	let mut shift = 0;
	while shift < 3 && size / 2 > max_existing && size / 4 >= reserved {
		size /= 2;
		shift += 1;
	}
	// VMA's growth admission uses free budget to choose pooled block sizes.
	// It does not turn an ordinary dedicated fallback into WITHIN_BUDGET.
	let snapshot = super::budget::heap_budgets(allocator)?;
	let heap = allocator.selector.properties.memory_types[index].heap_index as usize;
	let available = snapshot.heaps[heap]
		.budget
		.saturating_sub(snapshot.heaps[heap].usage)
		.min(
			snapshot.heaps[heap]
				.limit
				.saturating_sub(snapshot.heaps[heap].native_bytes),
		);
	while size >= reserved {
		if !blocks.upper_address && size > available {
			last = AllocError::OutOfSpace;
		} else {
			// SAFETY: compatible fixed pool/type, checked padding and bounded native sizes.
			match unsafe {
				allocate_pooled(
					allocator,
					request,
					avoided,
					BlockOptions {
						block_size: size,
						..blocks
					},
				)
			} {
				Ok(allocation) => return Ok(allocation),
				Err(error) if retryable(&error) => last = error,
				Err(error) => return Err(error),
			}
		}
		if shift == 3 {
			break;
		}
		size /= 2;
		shift += 1;
	}
	if !prefer_dedicated && !blocks.upper_address {
		// SAFETY: fallback remains owned by this pool and uses its fixed type.
		return unsafe { allocate_storage(allocator, request, storage, avoided) };
	}
	Err(last)
}

fn retryable(error: &AllocError) -> bool {
	matches!(
		error,
		AllocError::OutOfSpace | AllocError::AllocationLimit | AllocError::AllocationTooLarge { .. }
	) || matches!(error, AllocError::Call(error) if recoverable(error))
}

impl Allocator<'_> {
	/// Create a fixed-type pool with the selected placement algorithm and minimum blocks.
	/// # Errors
	/// Rejects invalid policy/types/limits and preserves metadata/native failures.
	/// # Safety
	/// The device/physical metadata and native allocation limits must remain valid.
	pub unsafe fn create_pool(&mut self, info: &PoolCreateInfo) -> Result<Pool, AllocError> {
		// SAFETY: identical native metadata/lifetime obligations.
		unsafe { create_pool(self, info) }
	}

	/// Release an empty pool and invalidate its identity. Live pooled/dedicated regions are rejected.
	pub fn destroy_pool(&mut self, pool: &mut Pool) -> Result<(), AllocError> {
		destroy_pool(self, pool)
	}

	/// Observe checked local pool accounting without native queries.
	pub fn pool_statistics(&self, pool: &Pool) -> Result<PoolStats, AllocError> {
		pool_statistics(self, pool)
	}
}

pub(super) fn validate_options<'a, 'device>(
	allocator: &'a Allocator<'device>,
	pool: &Pool,
	options: &AllocationCreateInfo,
) -> Result<&'a PoolState<'device>, AllocError> {
	let state = validate(allocator, pool)?;
	if options.upper_address
		&& (state.info.algorithm != AllocationAlgorithm::Linear || state.info.max_block_count != 1)
	{
		return Err(AllocError::UnsupportedCapability(
			"upper address requires a single-block linear pool",
		));
	}
	Ok(state)
}

pub(super) fn validate<'a, 'device>(
	allocator: &'a Allocator<'device>,
	pool: &Pool,
) -> Result<&'a PoolState<'device>, AllocError> {
	allocator.ensure_open()?;
	if allocator.owner != pool.owner {
		return Err(AllocError::WrongAllocator);
	}
	allocator
		.pools
		.get(&pool.serial)
		.ok_or(AllocError::UnknownPool)
}

/// Create a fixed-memory-type TLSF pool transactionally.
///
/// A zero maximum follows VMA's unlimited convention. Minimum blocks stay
/// allocated until explicit pool destruction. Above the minimum, one empty
/// block can be retained to avoid repeated native allocation/free churn.
/// Zero block size selects VMA's heap-based preferred size, bounded growth and
/// pool-owned dedicated fallback. Minimum blocks use the full preferred size.
/// # Errors
/// Returns checked invalid-policy/type/state/limit errors or the original native
/// failure. A failed minimum-block acquisition rolls back only this new pool.
/// # Safety
/// Context/physical metadata must be valid; enforce any unqueried native limits.
pub unsafe fn create_pool(
	allocator: &mut Allocator<'_>,
	info: &PoolCreateInfo,
) -> Result<Pool, AllocError> {
	// SAFETY: ordinary pool has no retained caller chain.
	unsafe { create_pool_inner(allocator, info, Default::default()) }
}

/// Create a pool retaining a borrowed native allocation chain for every block.
/// Import chains are rejected: a single imported payload cannot back repeated growth.
/// # Errors
/// Same policy/native errors as `create_pool`, plus conflicting or import chains.
/// # Safety
/// The complete chain and nested storage remain valid and immutable until this
/// pool is destroyed. The borrow lasts for the allocator's device lifetime.
/// Every future block allocation must meet the chain's native valid usage.
pub unsafe fn create_pool_with_chain<'device>(
	allocator: &mut Allocator<'device>,
	info: &PoolCreateInfo,
	chain: super::super::AllocationChain<'device>,
) -> Result<Pool, AllocError> {
	if chain.data.imports {
		return Err(AllocError::InvalidRequest(
			"pool chains cannot import memory",
		));
	}
	// SAFETY: caller retains the complete chain for every future pool acquisition.
	unsafe { create_pool_inner(allocator, info, chain.data) }
}

unsafe fn create_pool_inner(
	allocator: &mut Allocator<'_>,
	info: &PoolCreateInfo,
	chain: super::super::chain::ChainData,
) -> Result<Pool, AllocError> {
	allocator.ensure_open()?;
	allocator.ensure_global()?;
	if info.priority != MemoryPriority::default() && !allocator.memory_priority {
		return Err(AllocError::UnsupportedCapability(
			"pool priority requires enabled EXT feature metadata",
		));
	}
	if (info.max_block_count != 0 && info.min_block_count > info.max_block_count)
		|| (info.min_allocation_alignment != 0 && !info.min_allocation_alignment.is_power_of_two())
	{
		return Err(AllocError::InvalidRequest("invalid pool policy"));
	}
	if info.memory_type_index >= allocator.selector.properties.memory_type_count {
		return Err(AllocError::InvalidRequest("absent pool memory type"));
	}
	if info.block_size != 0 {
		allocator.validate_size(info.block_size)?;
	}
	let flags =
		allocator.selector.properties.memory_types[info.memory_type_index as usize].property_flags;
	let protected = flags.as_raw() & vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw() != 0;
	if protected && !allocator.protected_memory {
		return Err(AllocError::UnsupportedCapability(
			"protected pool requires enabled feature metadata",
		));
	}
	super::external::allocation_exports(allocator, info.memory_type_index, chain)?;
	// Native type/capability exclusions remain shared with all ordinary selection.
	allocator.selector.find_memory_type(&AllocRequest {
		size: info.block_size.max(1),
		alignment: 1,
		memory_type_bits: 1 << info.memory_type_index,
		required_flags: if protected {
			vk::MemoryPropertyFlagBits::PROTECTED_BIT
		} else {
			vk::MemoryPropertyFlags::empty()
		},
		preferred_flags: vk::MemoryPropertyFlags::empty(),
	})?;
	let preferred_block_size = if info.block_size != 0 {
		info.block_size
	} else {
		preferred_block_size(allocator, info.memory_type_index)?
	};
	allocator
		.next_serial
		.checked_add(1)
		.ok_or(AllocError::SerialExhausted)?;
	allocator
		.pools
		.try_reserve(1)
		.map_err(|_| AllocError::HostAllocationFailed)?;
	let serial = allocator.next_serial;
	for _ in 0..info.min_block_count {
		// SAFETY: validated ordinary type, inverse PFN preflighted at allocator creation.
		if let Err(error) = unsafe {
			create_block(
				allocator,
				serial,
				info.memory_type_index,
				preferred_block_size,
				info,
				chain,
			)
		} {
			remove_blocks(allocator, serial);
			return Err(error);
		}
	}
	allocator.next_serial += 1;
	allocator.pools.insert_reserved(
		serial,
		PoolState {
			allocation_chain: chain,
			name: None,
			info: *info,
			preferred_block_size,
		},
	);
	Ok(Pool {
		owner: allocator.owner,
		serial,
	})
}

unsafe fn create_block(
	allocator: &mut Allocator<'_>,
	serial: u64,
	memory_type_index: u32,
	block_size: u64,
	info: &PoolCreateInfo,
	chain: super::super::chain::ChainData,
) -> Result<(), AllocError> {
	allocator.check_native(block_size, memory_type_index)?;
	allocator
		.blocks
		.try_reserve(1)
		.map_err(|_| AllocError::HostAllocationFailed)?;
	let metadata = Metadata::new_native(
		block_size,
		info.algorithm,
		info.max_block_count == 1,
		allocator.granularity,
		allocator.host,
	)?;
	let export_types = super::external::allocation_exports(allocator, memory_type_index, chain)?;
	// SAFETY: exact pool type/size, admitted priority/address metadata and preflighted bookkeeping.
	let handle = unsafe {
		allocate_native(
			allocator,
			block_size,
			memory_type_index,
			StorageOptions {
				chain,
				dedicated: None,
				device_address: allocator.buffer_device_address,
				priority: info.priority,
				within_budget: false,
			},
		)
	}?;
	if handle.is_null() || allocator.contains_memory(handle) {
		return Err(
			CallError::InvalidOutput("allocator returned a null or already-live memory handle").into(),
		);
	}
	allocator.blocks.push_reserved(Block {
		defragmentation_hold: false,
		automatic: info.block_size == 0,
		export_types,
		priority: info.priority,
		pool: Some(serial),
		handle,
		size: block_size,
		memory_type_index,
		metadata,
		mapping: Mapping::new_with_hysteresis(allocator.mapping_hysteresis),
	});
	allocator.add_native(block_size, memory_type_index, handle);
	Ok(())
}

fn remove_block(allocator: &mut Allocator<'_>, index: usize) {
	let block = &mut allocator.blocks[index];
	debug_assert!(block.metadata.is_empty());
	debug_assert_eq!(block.mapping.references, 0);
	// SAFETY: empty unmapped block; any cached mapping has no user pointer holds.
	unsafe { block.mapping.clear_cache(allocator.device, block.handle) }
		.expect("preflighted cached mapping cleanup");
	// SAFETY: no regions/maps remain; native inverse PFN is immutable and preflighted.
	unsafe {
		allocator
			.device
			.free_memory(block.handle, allocator.callbacks)
	}
	.expect("preflighted pool cleanup");
	let block = allocator.blocks.remove(index);
	allocator.remove_native(block.size, block.memory_type_index, block.handle);
}

fn remove_blocks(allocator: &mut Allocator<'_>, serial: u64) {
	for index in (0..allocator.blocks.len()).rev() {
		if allocator.blocks[index].pool == Some(serial) {
			remove_block(allocator, index);
		}
	}
}

/// Destroy an empty pool, releasing its retained native blocks explicitly.
/// # Errors
/// Rejects closed, foreign, stale or live-allocation pools before native effects.
pub fn destroy_pool(allocator: &mut Allocator<'_>, pool: &mut Pool) -> Result<(), AllocError> {
	allocator.ensure_allocation()?;
	validate(allocator, pool)?;
	if allocator
		.live
		.values()
		.any(|live| live.pool == Some(pool.serial))
	{
		return Err(AllocError::LiveAllocations);
	}
	remove_blocks(allocator, pool.serial);
	allocator.pools.remove(&pool.serial);
	pool.serial = 0;
	Ok(())
}

/// Query a pool's local native bytes, placements and free ranges.
/// # Errors
/// Rejects closed, foreign and destroyed pool identities.
pub fn pool_statistics(allocator: &Allocator<'_>, pool: &Pool) -> Result<PoolStats, AllocError> {
	validate(allocator, pool)?;
	let mut result = PoolStats::default();
	for block in allocator
		.blocks
		.iter()
		.filter(|block| block.pool == Some(pool.serial))
	{
		let stats = block.metadata.statistics();
		result.block_count += 1;
		result.block_bytes += block.size;
		result.reserved_bytes += stats.allocated_bytes;
		result.free_bytes += stats.free_bytes;
		result.largest_free_region = result.largest_free_region.max(stats.largest_free_region);
	}
	for live in allocator
		.live
		.values()
		.filter(|live| live.pool == Some(pool.serial))
	{
		result.allocation_count += 1;
		result.allocation_bytes += live.size;
		if live.placement.is_none() {
			result.block_count += 1;
			result.block_bytes += live.size;
			result.reserved_bytes += live.reserved_size;
		}
	}
	Ok(result)
}

// Pinned VMA Free: keep one empty spare unless under budget pressure. Partial
// release can discard the vector's trailing empty spare. Canonical region
// retirement performs one incremental free-space swap afterward. No driver query here.
pub(super) fn release_block(allocator: &mut Allocator<'_>, index: usize) {
	let selected = &allocator.blocks[index];
	if selected.defragmentation_hold {
		return;
	}
	let domain = BlockDomain::new_from_block(selected);
	let min = domain
		.pool
		.map_or(0, |serial| allocator.pools[&serial].info.min_block_count);
	let same_domain = |block: &Block| domain.contains(block);
	let count = allocator
		.blocks
		.iter()
		.filter(|block| same_domain(block))
		.count();
	if count <= min {
		return;
	}
	let empty = allocator.blocks[index].metadata.is_empty();
	let other_empty = allocator
		.blocks
		.iter()
		.enumerate()
		.rfind(|(other, block)| {
			*other != index
				&& !block.defragmentation_hold
				&& same_domain(block)
				&& block.metadata.is_empty()
				&& block.mapping.references == 0
		})
		.map(|(index, _)| index);
	if empty {
		let heap = allocator.selector.properties.memory_types
			[allocator.blocks[index].memory_type_index as usize]
			.heap_index as usize;
		let ceiling =
			allocator.heap_limits[heap].unwrap_or(allocator.selector.properties.memory_heaps[heap].size);
		if other_empty.is_some()
			|| allocator.heap_bytes[heap] >= ceiling
			|| super::budget::cached_pressure(allocator, heap)
		{
			remove_block(allocator, index);
		}
	} else if other_empty.is_some() {
		// Donor Free inspects only the vector's last block after partial free.
		// Incremental sorting does not promise an entirely sorted vector.
		let last = allocator
			.blocks
			.iter()
			.rposition(same_domain)
			.expect("live block vector");
		let block = &allocator.blocks[last];
		if !block.defragmentation_hold && block.metadata.is_empty() && block.mapping.references == 0 {
			remove_block(allocator, last);
		}
	}
}

// Explicit maintenance shares the same native block removal/accounting owner.
// Reverse vector order preserves pool floors and never caches external budgets.
pub(super) fn trim_blocks(
	allocator: &mut Allocator<'_>,
	snapshot: &mut BudgetSnapshot,
	all: bool,
) -> TrimStats {
	let mut result = TrimStats::default();
	for index in (0..allocator.blocks.len()).rev() {
		let block = &allocator.blocks[index];
		let serial = block.pool;
		if block.defragmentation_hold || block.mapping.references != 0 || !block.metadata.is_empty() {
			continue;
		}
		let min = serial.map_or(0, |serial| allocator.pools[&serial].info.min_block_count);
		if allocator
			.blocks
			.iter()
			.filter(|other| {
				other.pool == serial
					&& other.memory_type_index == block.memory_type_index
					&& other.priority == block.priority
					&& other.automatic == block.automatic
			})
			.count()
			<= min
		{
			continue;
		}
		let heap = allocator.selector.properties.memory_types[block.memory_type_index as usize]
			.heap_index as usize;
		let budget = &mut snapshot.heaps[heap];
		if !all && budget.usage < budget.budget && budget.native_bytes < budget.limit {
			continue;
		}
		// Totals are bounded by the allocator's already checked native count/bytes.
		result.block_count += 1;
		result.block_bytes += block.size;
		budget.usage = budget.usage.saturating_sub(block.size);
		budget.native_bytes -= block.size;
		remove_block(allocator, index);
	}
	result
}

// Extensive contexts retain evacuated blocks for subsequent class packing.
// Explicit context completion releases their surplus empty storage; Drop only
// clears the host holds, leaving trim/reuse/close as native cleanup boundaries.
pub(super) fn finish_defragmentation_blocks(allocator: &mut Allocator<'_>) -> TrimStats {
	let mut result = TrimStats::default();
	for index in (0..allocator.blocks.len()).rev() {
		let block = &mut allocator.blocks[index];
		if !block.defragmentation_hold {
			continue;
		}
		block.defragmentation_hold = false;
		if !block.metadata.is_empty() || block.mapping.references != 0 {
			continue;
		}
		let pool = block.pool;
		let minimum = pool.map_or(0, |serial| allocator.pools[&serial].info.min_block_count);
		if pool.is_some()
			&& allocator
				.blocks
				.iter()
				.filter(|block| block.pool == pool)
				.count()
				<= minimum
		{
			continue;
		}
		result.block_count += 1;
		result.block_bytes += allocator.blocks[index].size;
		remove_block(allocator, index);
	}
	result
}

// Pinned VMA CalcPreferredBlockSize. One owner for custom and default vectors.
fn preferred_block_size(allocator: &Allocator<'_>, index: u32) -> Result<u64, AllocError> {
	let heap = allocator.selector.properties.memory_types[index as usize].heap_index as usize;
	let heap_size = allocator.selector.properties.memory_heaps[heap].size;
	let size = if heap_size <= 1024 * 1024 * 1024 {
		heap_size / 8
	} else {
		allocator.preferred_large_heap_block_size
	};
	size
		.checked_add(31)
		.map(|size| size & !31)
		.filter(|&size| size != 0)
		.ok_or(AllocError::InvalidProperties("zero automatic block size"))
}

pub(super) unsafe fn allocate_default(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	storage: StorageOptions<'_>,
	dedication: Dedication,
	options: &AllocationCreateInfo,
	class: ResourceClass,
	avoided: vk::MemoryPropertyFlags,
) -> Result<Allocation, AllocError> {
	let mut bits = request.memory_type_bits;
	let mut last = None;
	loop {
		let candidate = AllocRequest {
			memory_type_bits: bits,
			..*request
		};
		let index = match allocator
			.selector
			.find_memory_type_with_preferences(&candidate, avoided)
		{
			Ok(index) => index,
			Err(error) => return Err(last.unwrap_or(error)),
		};
		bits &= !(1 << index);
		let request = AllocRequest {
			memory_type_bits: 1 << index,
			..*request
		};
		let blocks = BlockOptions {
			automatic: true,
			block_size: preferred_block_size(allocator, index)?,
			strategy: options.strategy,
			class,
			grow: !options.never_allocate,
			pool: None,
			upper_address: false,
			priority: options.priority,
			within_budget: options.within_budget,
			prefer_mapped: super::super::policy::prefers_mapped(options),
		};
		// SAFETY: one compatible type at a time; reuse/growth/fallback have the same native requirements.
		match unsafe { allocate_automatic(allocator, &request, storage, dedication, avoided, blocks) } {
			Ok(allocation) => return Ok(allocation),
			Err(error) if retryable(&error) => last = Some(error),
			Err(error) => return Err(error),
		}
	}
}

/// Set preferred block bytes for heaps larger than 1 GiB before pool/storage creation.
/// Small heaps use one eighth of their heap size; both paths round up to 32 bytes.
/// Does not alter explicit `Pooled`/`Adaptive` policies or query/reserve a budget.
/// # Errors
/// Rejects zero/overflowing sizes, closed state and existing native/pool domains.
pub fn set_preferred_block_size(
	allocator: &mut Allocator<'_>,
	size: u64,
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if allocator.native_count != 0 || !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	if !allocator.pools.is_empty() {
		return Err(AllocError::LivePools);
	}
	let size = size
		.checked_add(31)
		.map(|size| size & !31)
		.filter(|&size| size != 0)
		.ok_or(AllocError::InvalidRequest(
			"preferred block size must be nonzero and roundable",
		))?;
	allocator.preferred_large_heap_block_size = size;
	Ok(())
}

impl Allocator<'_> {
	/// Configure large-heap automatic block sizing before any pool/storage creation.
	pub fn set_preferred_block_size(&mut self, size: u64) -> Result<(), AllocError> {
		set_preferred_block_size(self, size)
	}
}

/// Configure native pooled mapping retention before creating storage or pools.
/// Enabled by default. Separate allocations never cache mappings. This affects
/// native map/unmap call counts, never allocation-level pointer access rights.
/// # Errors
/// Rejects a closed allocator or any existing native/pool domain.
pub fn set_mapping_hysteresis(
	allocator: &mut Allocator<'_>,
	enabled: bool,
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if allocator.native_count != 0 || !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	if !allocator.pools.is_empty() {
		return Err(AllocError::LivePools);
	}
	allocator.mapping_hysteresis = enabled;
	Ok(())
}

impl Allocator<'_> {
	/// Configure pooled mapping retention before native/pool acquisition.
	pub fn set_mapping_hysteresis(&mut self, enabled: bool) -> Result<(), AllocError> {
		set_mapping_hysteresis(self, enabled)
	}
}
