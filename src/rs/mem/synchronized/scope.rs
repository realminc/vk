//! Scoped vector and native-block concurrency over the canonical allocation implementation.
//!
//! Donor: VMA 3.4.0 VmaBlockVector's mutex and allocator-wide allocation-count /
//! heap accounting (LICENSE-VMA-MIT). Rust adaptation: borrowed worker scope,
//! one pool mutex, short shared reservation lock and allocation-free restoration.
#[path = "resource.rs"]
pub(in crate::mem::allocator) mod resource;
pub(super) use resource::claim;

use super::*;
use crate::mem::{BudgetSnapshot, SynchronizedAllocator};
use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum Domain {
	#[default]
	Global,
	Pool(u64),
	MemoryType(u32),
	Block(vk::DeviceMemory),
}

struct Routes<'device> {
	kind: Domain,
	indices: HostMap<'device, usize>,
	#[cfg(all(windows, feature = "platform-win32"))]
	memory: HostMap<'device, usize>,
}

impl<'device> Routes<'device> {
	fn new(root: &Allocator<'device>, domains: &[Domain]) -> Result<Self, AllocError> {
		let mut result = Self {
			kind: domains.first().copied().unwrap_or_default(),
			indices: HostMap::new(root.host),
			#[cfg(all(windows, feature = "platform-win32"))]
			memory: HostMap::new(root.host),
		};
		result.indices.try_reserve(domains.len())?;
		for (index, domain) in domains.iter().enumerate() {
			let key = match *domain {
				Domain::Pool(serial) => serial,
				Domain::MemoryType(index) => u64::from(index),
				Domain::Block(handle) => handle.as_raw(),
				Domain::Global => unreachable!("normalized domain"),
			};
			result.indices.insert_reserved(key, index);
		}
		#[cfg(all(windows, feature = "platform-win32"))]
		{
			result.memory.try_reserve(root.native_count)?;
			for block in &root.blocks {
				if let Some(index) = result.block(block) {
					result.memory.insert_reserved(block.handle.as_raw(), index);
				}
			}
			for live in root.live.values().filter(|live| live.placement.is_none()) {
				if let Some(index) = result.live(live) {
					result.memory.insert_reserved(live.handle.as_raw(), index);
				}
			}
		}
		Ok(result)
	}

	fn find(&self, pool: Option<u64>, memory_type: u32, handle: vk::DeviceMemory) -> Option<usize> {
		let key = match self.kind {
			Domain::Pool(_) => pool?,
			Domain::MemoryType(_) if pool.is_none() => u64::from(memory_type),
			Domain::Block(_) => handle.as_raw(),
			_ => return None,
		};
		self.indices.get(&key).copied()
	}

	fn live(&self, live: &Live<'_>) -> Option<usize> {
		self.find(live.pool, live.memory_type_index, live.handle)
	}

	fn block(&self, block: &Block<'_>) -> Option<usize> {
		self.find(block.pool, block.memory_type_index, block.handle)
	}
}

pub(in crate::mem) enum Selection<'scope> {
	Pools(&'scope [&'scope Pool]),
	MemoryTypes(&'scope [u32]),
	Blocks(&'scope [&'scope Allocation]),
}

impl Selection<'_> {
	fn normalize<'device>(
		&self,
		root: &Allocator<'device>,
	) -> Result<HostVec<'device, Domain>, AllocError> {
		let mut result = HostVec::new(root.host);
		let count = match self {
			Self::Pools(values) => values.len(),
			Self::MemoryTypes(values) => values.len(),
			Self::Blocks(values) => values.len(),
		};
		result.try_reserve_exact(count)?;
		for index in 0..count {
			let domain = match self {
				Self::Pools(values) => {
					pool::validate(root, values[index])?;
					Domain::Pool(values[index].serial)
				}
				Self::MemoryTypes(values) => {
					if values[index] >= root.selector.properties.memory_type_count {
						return Err(AllocError::InvalidRequest("absent scoped memory type"));
					}
					Domain::MemoryType(values[index])
				}
				Self::Blocks(values) => {
					root.validate(values[index])?;
					Domain::Block(values[index].handle)
				}
			};
			if result.contains(&domain) {
				return Err(AllocError::InvalidRequest("duplicate parallel domain"));
			}
			result.push_reserved(domain);
		}
		Ok(result)
	}
}

struct State<'device> {
	live: HostMap<'device, Live<'device>>,
	blocks: HostVec<'device, Block<'device>>,
	next_serial: u64,
	live_count: usize,
	native_count: usize,
	native_bytes: u64,
	heap_bytes: [u64; 16],
	allocation_limit: u32,
	budget: BudgetSnapshot,
	preserve_budget: bool,
	domains: usize,
	resources: resource::Resources<'device>,
	restore_order: HostMap<'device, usize>,
	#[cfg(all(windows, feature = "platform-win32"))]
	external_handles: HostVec<'device, external::CachedHandle>,
	#[cfg(all(windows, feature = "platform-win32"))]
	external_count: usize,
}

// SAFETY: this private state exists only during a borrowed scope, under the same
// any-thread callback/chain contract as its parent SynchronizedAllocator.
// The mutex exclusively owns all registry/vector buffers. Other workers only
// retain raw native identities, never references into these buffers. No user
// closure or native PFN runs under this mutex. CPU allocation callbacks must
// support concurrent independent owners and cannot unwind/reenter the allocator.
unsafe impl Send for State<'_> {}

pub(super) struct Shared<'device>(Mutex<State<'device>>);

impl<'device> Shared<'device> {
	fn lock(&self) -> MutexGuard<'_, State<'device>> {
		// No user closure executes under this private lock. Recovering an internal
		// poison preserves native reservations during scope unwind; the outer
		// public owner still refuses access after a user panic.
		self.0.lock().unwrap_or_else(PoisonError::into_inner)
	}

	pub fn native_count(&self) -> usize {
		self.lock().native_count
	}

	#[cfg(all(windows, feature = "platform-win32"))]
	pub fn reserve_external(&self) -> Result<(), AllocError> {
		let mut state = self.lock();
		let needed = state
			.external_count
			.checked_add(state.domains)
			.ok_or(AllocError::HostAllocationFailed)?;
		let additional = needed.saturating_sub(state.external_handles.len());
		state.external_handles.try_reserve(additional)
	}

	#[cfg(all(windows, feature = "platform-win32"))]
	pub fn external_added(&self) {
		self.lock().external_count += 1;
	}

	#[cfg(all(windows, feature = "platform-win32"))]
	pub fn external_removed(&self, count: usize) {
		self.lock().external_count -= count;
	}

	pub fn reserve_publication(&self) -> Result<u64, AllocError> {
		let mut state = self.lock();
		let next = state
			.next_serial
			.checked_add(1)
			.ok_or(AllocError::SerialExhausted)?;
		// Each locked domain can have at most one unpublished allocation and
		// backing block. Reserve their future merge before any native effect.
		// This makes restoration infallible even if later host allocation fails.
		let live = state
			.live_count
			.checked_add(state.domains)
			.ok_or(AllocError::HostAllocationFailed)?;
		let blocks = state
			.native_count
			.checked_add(state.domains)
			.ok_or(AllocError::HostAllocationFailed)?;
		let extra_live = live.saturating_sub(state.live.len());
		let extra_blocks = blocks.saturating_sub(state.blocks.len());
		state.live.try_reserve(extra_live)?;
		state.blocks.try_reserve(extra_blocks)?;
		let serial = state.next_serial;
		state.next_serial = next;
		Ok(serial)
	}

	pub fn publish(&self) {
		self.lock().live_count += 1;
	}

	pub fn retire(
		&self,
		target: Option<Resource>,
		plane: Option<vk::ImageAspectFlagBits>,
		serial: u64,
	) {
		let mut state = self.lock();
		state.live_count -= 1;
		resource::retire(&mut state, target, plane, serial);
	}

	pub fn release_resource(
		&self,
		target: Option<Resource>,
		plane: Option<vk::ImageAspectFlagBits>,
		serial: u64,
	) {
		resource::retire(&mut self.lock(), target, plane, serial);
	}

	pub fn associated(&self, target: Resource, domain: u64) -> bool {
		self.lock().resources.associated(target, domain)
	}

	pub fn reserve_native(
		&self,
		size: u64,
		heap: usize,
		within_budget: bool,
	) -> Result<Reservation<'_, 'device>, AllocError> {
		let mut state = self.lock();
		if state.native_count >= state.allocation_limit as usize {
			return Err(AllocError::AllocationLimit);
		}
		let budget = state.budget.heaps[heap];
		if size > budget.limit.saturating_sub(state.heap_bytes[heap]) {
			return Err(AllocError::OutOfSpace);
		}
		let external = budget.usage.saturating_sub(budget.native_bytes);
		if within_budget
			&& size
				> budget
					.budget
					.saturating_sub(external.saturating_add(state.heap_bytes[heap]))
		{
			return Err(AllocError::OutOfSpace);
		}
		let bytes = state
			.native_bytes
			.checked_add(size)
			.ok_or(AllocError::InvalidRange)?;
		state.native_count += 1;
		state.native_bytes = bytes;
		state.heap_bytes[heap] += size;
		Ok(Reservation {
			shared: self,
			size,
			heap,
			committed: false,
		})
	}

	pub fn release_native(&self, size: u64, heap: usize) {
		let mut state = self.lock();
		state.native_count -= 1;
		state.native_bytes -= size;
		state.heap_bytes[heap] -= size;
	}

	pub fn budget_snapshot(&self) -> BudgetSnapshot {
		let state = self.lock();
		let mut snapshot = state.budget;
		for (index, heap) in snapshot.heaps[..snapshot.heap_count as usize]
			.iter_mut()
			.enumerate()
		{
			let external = heap.usage.saturating_sub(heap.native_bytes);
			heap.native_bytes = state.heap_bytes[index];
			heap.usage = external.saturating_add(heap.native_bytes);
		}
		snapshot
	}
}

pub(super) struct Reservation<'borrow, 'device> {
	shared: &'borrow Shared<'device>,
	size: u64,
	heap: usize,
	committed: bool,
}

impl Reservation<'_, '_> {
	pub fn commit(mut self) {
		self.committed = true;
	}
}

impl Drop for Reservation<'_, '_> {
	fn drop(&mut self) {
		if !self.committed {
			self.shared.release_native(self.size, self.heap);
		}
	}
}

struct Scope<'borrow, 'device> {
	root: &'borrow mut Allocator<'device>,
	shared: HostVec<'device, Shared<'device>>,
	children: HostVec<'device, SynchronizedAllocator<'device>>,
}

impl Drop for Scope<'_, '_> {
	fn drop(&mut self) {
		// Every worker's borrow ends before this guard can be dropped. Poison
		// does not discard a child owner; move it back without native cleanup.
		let state = self.shared[0]
			.0
			.get_mut()
			.unwrap_or_else(PoisonError::into_inner);
		while let Some(child) = self.children.pop() {
			let mut child = child.into_inner().unwrap_or_else(PoisonError::into_inner);
			child.parallel = None;
			child.partition = Domain::Global;
			child.partition_id = 0;
			state.live.append_reserved(&mut child.live);
			state.blocks.append_reserved(&mut child.blocks);
			#[cfg(all(windows, feature = "platform-win32"))]
			state
				.external_handles
				.append_reserved(&mut child.external_handles);
			// Pool count cannot grow inside this scope; root retains its original
			// registry capacity, so returning the selected pool cannot allocate.
			self.root.pools.append_reserved(&mut child.pools);
		}
		if !state.restore_order.is_empty() {
			state
				.blocks
				.sort_unstable_by_key(|block| state.restore_order[&block.handle.as_raw()]);
		}
		#[cfg(all(windows, feature = "platform-win32"))]
		std::mem::swap(&mut self.root.external_handles, &mut state.external_handles);
		std::mem::swap(&mut self.root.live, &mut state.live);
		std::mem::swap(&mut self.root.blocks, &mut state.blocks);
		self.root.next_serial = state.next_serial;
		self.root.native_count = state.native_count;
		self.root.native_bytes = state.native_bytes;
		self.root.heap_bytes = state.heap_bytes;
		// The scope's external-pressure sample is bounded to this batch. Force
		// the ordinary cadence to refresh at the next native budget admission.
		if !state.preserve_budget {
			self.root.budget_cache = Some(state.budget);
			self.root.budget_operations = u32::MAX;
		}
	}
}

fn new_child<'device>(root: &Allocator<'device>, domain: Domain) -> Allocator<'device> {
	Allocator {
		host: root.host,
		device: root.device,
		callbacks: root.callbacks,
		device_memory_callbacks: root.device_memory_callbacks,
		physical_device: root.physical_device,
		instance_handle: root.instance_handle,
		physical_properties: root.physical_properties,
		selector: MemoryTypeSelector {
			properties: root.selector.properties,
			atom: root.selector.atom,
			device_coherent: root.selector.device_coherent,
			allowed_bits: match domain {
				Domain::MemoryType(index) => 1 << index,
				_ => root.selector.allowed_bits,
			},
		},
		owner: root.owner,
		next_serial: root.next_serial,
		live: HostMap::new(root.host),
		pools: HostMap::new(root.host),
		blocks: HostVec::new(root.host),
		native_count: 0,
		native_bytes: 0,
		heap_bytes: [0; 16],
		heap_limits: root.heap_limits,
		allocation_limit: root.allocation_limit,
		allocation_size_limit: root.allocation_size_limit,
		integrated: root.integrated,
		buffer_device_address: root.buffer_device_address,
		memory_priority: root.memory_priority,
		protected_memory: root.protected_memory,
		budget_query: root.budget_query,
		budget_cache: root.budget_cache,
		budget_operations: root.budget_operations,
		current_frame_index: root.current_frame_index,
		debug_settings: root.debug_settings,
		external_types: root.external_types,
		preferred_large_heap_block_size: root.preferred_large_heap_block_size,
		mapping_hysteresis: root.mapping_hysteresis,
		#[cfg(all(windows, feature = "platform-win32"))]
		external_handles: HostVec::new(root.host),
		granularity: root.granularity,
		closed: false,
		parallel: None,
		partition: domain,
		partition_id: 0,
	}
}

// Called only with the original public mutex held. The caller propagates a
// poisoned child's status into the parent after restoration, even if a worker
// panic was caught by the user closure.

pub(in crate::mem) unsafe fn with_domains<'device, R>(
	root: &mut Allocator<'device>,
	selection: Selection<'_>,
	operation: impl FnOnce(&[SynchronizedAllocator<'device>]) -> R,
) -> Result<(R, bool), AllocError> {
	root.ensure_open()?;
	root.ensure_global()?;
	let domains = selection.normalize(root)?;
	let mut children = HostVec::new(root.host);
	children.try_reserve_exact(domains.len())?;
	let mut shared = HostVec::new(root.host);
	shared.try_reserve_exact(1)?;
	let mut restore_order = HostMap::new(root.host);
	if matches!(selection, Selection::Blocks(_)) {
		restore_order.try_reserve(root.blocks.len())?;
		for (index, block) in root.blocks.iter().enumerate() {
			restore_order.insert_reserved(block.handle.as_raw(), index);
		}
	}
	let routes = Routes::new(root, &domains)?;
	let mut counts = HostVec::new(root.host);
	counts.try_reserve_exact(domains.len())?;
	counts.resize_reserved(domains.len(), (0_usize, 0_usize, 0_usize));
	for live in root.live.values() {
		if let Some(index) = routes.live(live) {
			counts[index].0 += 1;
		}
	}
	for block in &root.blocks {
		if let Some(index) = routes.block(block) {
			counts[index].1 += 1;
		}
	}
	#[cfg(all(windows, feature = "platform-win32"))]
	for entry in &root.external_handles {
		if let Some(&index) = routes.memory.get(&entry.memory.as_raw()) {
			counts[index].2 += 1;
		}
	}
	// All fallible metadata preparation precedes moving any live owner.
	for (index, &domain) in domains.iter().enumerate() {
		let mut child = new_child(root, domain);
		if matches!(domain, Domain::Pool(_)) {
			child.pools.try_reserve(1)?;
		}
		child.live.try_reserve(counts[index].0)?;
		child.blocks.try_reserve_exact(counts[index].1)?;
		#[cfg(all(windows, feature = "platform-win32"))]
		child.external_handles.try_reserve_exact(counts[index].2)?;
		// SAFETY: the scope inherits the parent's established any-thread contract.
		children.push_reserved(unsafe { SynchronizedAllocator::new(child) });
	}

	let preserve_budget = matches!(selection, Selection::Blocks(_));
	// Block phases cannot acquire/release native memory, so preserve their
	// parent's query cadence and avoid requiring a budget-query context for maps.
	let budget = if preserve_budget {
		root
			.budget_cache
			.unwrap_or_else(|| budget::budget_snapshot(root))
	} else {
		budget::heap_budgets(root)?
	};
	root.live.try_reserve(domains.len())?;
	root.blocks.try_reserve(domains.len())?;
	let resources = resource::Resources::new(root)?;
	let state = State {
		live_count: root.live.len(),
		live: std::mem::replace(&mut root.live, HostMap::new(root.host)),
		blocks: std::mem::replace(&mut root.blocks, HostVec::new(root.host)),
		next_serial: root.next_serial,
		native_count: root.native_count,
		native_bytes: root.native_bytes,
		heap_bytes: root.heap_bytes,
		allocation_limit: root.allocation_limit,
		budget,
		preserve_budget,
		domains: domains.len(),
		resources,
		restore_order,
		#[cfg(all(windows, feature = "platform-win32"))]
		external_count: root.external_handles.len(),
		#[cfg(all(windows, feature = "platform-win32"))]
		external_handles: std::mem::replace(&mut root.external_handles, HostVec::new(root.host)),
	};
	shared.push_reserved(Shared(Mutex::new(state)));
	let pointer = std::ptr::NonNull::from(&mut shared[0]);
	let mut scope = Scope {
		root,
		shared,
		children,
	};
	for (index, &domain) in domains.iter().enumerate() {
		let child = scope.children[index].owner_mut();
		child.parallel = Some(pointer.cast());
		child.partition_id = u64::try_from(index)
			.expect("reserved domain index")
			.checked_add(1)
			.expect("reserved domain count");
		if let Domain::Pool(serial) = domain {
			child.pools.insert_reserved(
				serial,
				scope.root.pools.remove(&serial).expect("validated pool"),
			);
		}
	}
	let state = scope.shared[0]
		.0
		.get_mut()
		.unwrap_or_else(PoisonError::into_inner);
	state.live.extract_to_reserved(
		|live| routes.live(live),
		|index, serial, live| {
			scope.children[index]
				.owner_mut()
				.live
				.insert_reserved(serial, live);
		},
	);
	state.blocks.extract_to_reserved(
		|block| routes.block(block),
		|index, block| {
			scope.children[index]
				.owner_mut()
				.blocks
				.push_reserved(block)
		},
	);
	#[cfg(all(windows, feature = "platform-win32"))]
	state.external_handles.extract_to_reserved(
		|entry| routes.memory.get(&entry.memory.as_raw()).copied(),
		|index, entry| {
			scope.children[index]
				.owner_mut()
				.external_handles
				.push_reserved(entry)
		},
	);
	for child in &mut scope.children {
		let child = child.owner_mut();

		for block in &child.blocks {
			child.native_count += 1;
			child.native_bytes += block.size;
			let heap = child.selector.properties.memory_types[block.memory_type_index as usize].heap_index
				as usize;
			child.heap_bytes[heap] += block.size;
		}
		for live in child.live.values().filter(|live| live.placement.is_none()) {
			child.native_count += 1;
			child.native_bytes += live.size;
			let heap =
				child.selector.properties.memory_types[live.memory_type_index as usize].heap_index as usize;
			child.heap_bytes[heap] += live.size;
		}
	}
	let result = operation(&scope.children);
	let poisoned = scope
		.children
		.iter()
		.any(SynchronizedAllocator::is_poisoned);
	Ok((result, poisoned))
}
