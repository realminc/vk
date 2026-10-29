//! Explicit memory-budget snapshots and local per-heap policy limits.
//! Donor: pinned VMA GetHeapBudgets/SetCurrentFrameIndex budget separation.
//! Adaptation: queries are explicit; no hot-path driver queries or stale-budget promises.
//! Empty-block eviction follows `VmaBlockVector::Free`; explicit trim is an
//! additional Rust maintenance boundary. See LICENSE-VMA-MIT.
use super::Allocator;
use crate::{
	self as vk,
	mem::{AllocError, BudgetSnapshot, BudgetSource, HeapBudget},
};

/// Native cache released by one explicit maintenance call. Live regions never move.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrimStats {
	pub block_count: usize,
	pub block_bytes: u64,
}

impl Allocator<'_> {
	/// Set a local native-byte ceiling. Existing allocations are never evicted.
	pub fn set_heap_limit(&mut self, heap: u32, limit: u64) -> Result<(), AllocError> {
		set_heap_limit(self, heap, limit)
	}

	/// Heap-size/local-accounting estimate, without querying the driver.
	pub fn budget_snapshot(&self) -> BudgetSnapshot {
		budget_snapshot(self)
	}

	/// Release empty, unmapped cached blocks above their pool minimums.
	/// No driver budget query, allocation, synchronization or live-object movement.
	/// # Errors
	/// Returns `Closed` after allocator close.
	pub fn trim(&mut self) -> Result<TrimStats, AllocError> {
		trim(self)
	}

	/// Query budget pressure and release eligible empty blocks on pressured heaps.
	/// Minimum blocks and all live/mapped blocks remain owned by their pools.
	/// # Errors
	/// Returns closed/context/capability/native query errors before freeing memory.
	/// # Safety
	/// Supply the live instance and physical device used to create this allocator's device.
	pub unsafe fn trim_with_budget(
		&mut self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<TrimStats, AllocError> {
		// SAFETY: identical native query identity/lifetime contract.
		unsafe { trim_with_budget(self, instance, physical) }
	}

	/// Query enabled EXT_memory_budget; otherwise return an explicitly local estimate.
	/// # Safety
	/// Instance/physical device must be the live parent used to create this allocator's device.
	pub unsafe fn query_budget(
		&self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<BudgetSnapshot, AllocError> {
		// SAFETY: identical native identity/lifetime contract.
		unsafe { query_budget(self, instance, physical) }
	}
}

/// Release surplus empty, unmapped cached blocks, preserving each pool minimum.
///
/// Free/method forms share this operation. Pools remain live and can grow again.
/// No native budget query, host allocation, submit/wait or relocation is performed.
/// Native resource uses must already have ended through the existing unsafe free/
/// destroy contract before a block can become empty. Dropping anything never trims.
/// Includes explicit-size default blocks retained by an abandoned Extensive context.
/// # Errors
/// Returns `Closed` after allocator close.
pub fn trim(allocator: &mut Allocator<'_>) -> Result<TrimStats, AllocError> {
	allocator.ensure_allocation()?;
	allocator.ensure_open()?;
	let mut snapshot = budget_snapshot(allocator);
	Ok(super::pool::trim_blocks(allocator, &mut snapshot, true))
}

/// Query an explicit budget snapshot, then trim surplus empty blocks on pressured heaps.
///
/// Pressure means usage >= budget or native bytes >= the local heap ceiling.
/// Without enabled EXT_memory_budget the snapshot is explicitly local. The
/// selection subtracts released native bytes from that one snapshot, stopping
/// once projected pressure ends or no eligible block remains. External usage
/// may change immediately; this does not promise allocation success or budget
/// compliance. No snapshot is cached and no live resource is evicted or moved.
/// # Errors
/// Returns closed/context/capability/native query errors before freeing memory.
/// # Safety
/// Instance/physical device must be the live parent used to create this allocator's device.
pub unsafe fn trim_with_budget(
	allocator: &mut Allocator<'_>,
	instance: &vk::Instance,
	physical: vk::PhysicalDevice,
) -> Result<TrimStats, AllocError> {
	allocator.ensure_allocation()?;
	// SAFETY: caller supplies the exact live physical identity; query validates metadata.
	let mut snapshot = unsafe { query_budget(allocator, instance, physical) }?;
	Ok(super::pool::trim_blocks(allocator, &mut snapshot, false))
}

/// Change a local policy ceiling; this never reserves native heap capacity.
pub fn set_heap_limit(
	allocator: &mut Allocator<'_>,
	heap: u32,
	limit: u64,
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if heap >= allocator.selector.properties.memory_heap_count {
		return Err(AllocError::InvalidRequest("absent memory heap"));
	}
	let index = heap as usize;
	if limit > allocator.selector.properties.memory_heaps[index].size
		|| limit < allocator.heap_bytes[index]
	{
		return Err(AllocError::InvalidRequest(
			"heap limit exceeds size or live native bytes",
		));
	}
	allocator.heap_limits[index] = Some(limit);
	Ok(())
}

/// Local accounting is always distinguished from implementation-reported usage.
pub fn budget_snapshot(allocator: &Allocator<'_>) -> BudgetSnapshot {
	if let Some(shared) = allocator.parallel_state() {
		return shared.budget_snapshot();
	}
	let mut snapshot = BudgetSnapshot {
		heap_count: allocator.selector.properties.memory_heap_count,
		..Default::default()
	};
	for (index, heap) in snapshot.heaps[..snapshot.heap_count as usize]
		.iter_mut()
		.enumerate()
	{
		*heap = HeapBudget {
			size: allocator.selector.properties.memory_heaps[index].size,
			limit: allocator.heap_limits[index]
				.unwrap_or(allocator.selector.properties.memory_heaps[index].size),
			budget: allocator.heap_limits[index]
				.unwrap_or(allocator.selector.properties.memory_heaps[index].size),
			usage: allocator.heap_bytes[index],
			native_bytes: allocator.heap_bytes[index],
		};
	}
	snapshot
}

/// Return an explicit native budget snapshot without caching admission decisions.
/// # Safety
/// Supply the matching live instance and physical device. Metadata checks do not prove identity.
pub unsafe fn query_budget(
	allocator: &Allocator<'_>,
	instance: &vk::Instance,
	physical: vk::PhysicalDevice,
) -> Result<BudgetSnapshot, AllocError> {
	allocator.ensure_open()?;
	let snapshot = budget_snapshot(allocator);
	if !crate::mem::resource::extension(allocator.device, c"VK_EXT_memory_budget") {
		return Ok(snapshot);
	}
	let parent = instance
		.capabilities()
		.ok_or(AllocError::UnsupportedCapability(
			"instance metadata is required",
		))?;
	let capabilities = allocator
		.device
		.capabilities()
		.expect("extension requires metadata");
	if physical.is_null() || !capabilities.matches_instance(parent) {
		return Err(AllocError::InvalidProperties(
			"budget context metadata disagree",
		));
	}
	let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
	let mut output = vk::PhysicalDeviceMemoryProperties2 {
		p_next: std::ptr::from_mut(&mut budget).cast(),
		..Default::default()
	};
	// SAFETY: matching live physical device and initialized output chain.
	unsafe {
		if parent.instance_api_version() >= vk::API_VERSION_1_1 {
			instance.get_physical_device_memory_properties2(physical, &mut output)?;
		} else if parent
			.instance_extensions()
			.any(|name| name == c"VK_KHR_get_physical_device_properties2")
		{
			instance.get_physical_device_memory_properties2_khr(physical, &mut output)?;
		} else {
			return Err(AllocError::UnsupportedCapability(
				"budget requires properties2",
			));
		}
	}
	validate_budget(allocator, &output, &budget)
}

fn validate_budget(
	allocator: &Allocator<'_>,
	output: &vk::PhysicalDeviceMemoryProperties2,
	budget: &vk::PhysicalDeviceMemoryBudgetPropertiesEXT,
) -> Result<BudgetSnapshot, AllocError> {
	let mut snapshot = budget_snapshot(allocator);

	if output.memory_properties.memory_heap_count != snapshot.heap_count {
		return Err(AllocError::InvalidProperties(
			"budget heap inventory changed",
		));
	}
	for (index, heap) in snapshot.heaps[..snapshot.heap_count as usize]
		.iter_mut()
		.enumerate()
	{
		if output.memory_properties.memory_heaps[index].size != heap.size
			|| budget.heap_budget[index] == 0
			|| budget.heap_budget[index] > heap.size
		{
			return Err(AllocError::InvalidProperties("invalid native heap budget"));
		}
		heap.budget = budget.heap_budget[index];
		heap.usage = budget.heap_usage[index];
	}
	if budget.heap_budget[snapshot.heap_count as usize..]
		.iter()
		.any(|&value| value != 0)
		|| budget.heap_usage[snapshot.heap_count as usize..]
			.iter()
			.any(|&value| value != 0)
	{
		return Err(AllocError::InvalidProperties(
			"native budget references absent heaps",
		));
	}
	snapshot.source = BudgetSource::Extension;
	Ok(snapshot)
}

#[derive(Clone, Copy)]
pub(super) struct BudgetQuery {
	physical: vk::PhysicalDevice,
	function: vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
	name: &'static str,
}

pub(super) fn capture_query(
	instance: &vk::Instance,
	physical: vk::PhysicalDevice,
) -> Option<BudgetQuery> {
	let parent = instance.capabilities()?;
	if parent.instance_api_version() >= vk::API_VERSION_1_1 {
		Some(BudgetQuery {
			physical,
			function: instance.fp().get_physical_device_memory_properties2,
			name: "vkGetPhysicalDeviceMemoryProperties2",
		})
	} else if parent
		.instance_extensions()
		.any(|name| name == c"VK_KHR_get_physical_device_properties2")
	{
		Some(BudgetQuery {
			physical,
			function: instance.fp().get_physical_device_memory_properties2_khr,
			name: "vkGetPhysicalDeviceMemoryProperties2KHR",
		})
	} else {
		None
	}
}

/// Refresh an allocator-owned budget snapshot from the retained native context.
///
/// Without EXT_memory_budget this records local accounting. A manually constructed
/// EXT allocator needs `refresh_budget_with_context` before automatic refresh.
/// # Errors
/// Missing context/PFN or invalid native outputs leave the previous cache intact.
pub fn refresh_budget(allocator: &mut Allocator<'_>) -> Result<BudgetSnapshot, AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	let snapshot = if crate::mem::resource::extension(allocator.device, c"VK_EXT_memory_budget") {
		let query = allocator
			.budget_query
			.ok_or(AllocError::UnsupportedCapability(
				"retained physical budget context is required",
			))?;
		let function = query
			.function
			.ok_or(crate::load::CallError::MissingCommand(query.name))?;
		let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
		let mut output = vk::PhysicalDeviceMemoryProperties2 {
			p_next: std::ptr::from_mut(&mut budget).cast(),
			..Default::default()
		};
		// SAFETY: context was captured under the allocator's native parent-lifetime/identity contract;
		// the borrowed Device retains the loader library and both outputs are initialized stack records.
		unsafe { function(query.physical, &mut output) };
		validate_budget(allocator, &output, &budget)?
	} else {
		budget_snapshot(allocator)
	};
	allocator.budget_cache = Some(snapshot);
	allocator.budget_operations = 0;
	Ok(snapshot)
}

/// Capture a checked budget context for future refreshes, including manual constructors.
/// # Errors
/// Preserves context/query errors before changing retained budget state.
/// # Safety
/// Instance/physical device must be the exact live parents of this allocator's
/// device and remain live for its complete remaining lifetime. Returned usage is
/// a snapshot, never a reservation. Device retention keeps PFN code loaded only.
pub unsafe fn refresh_budget_with_context(
	allocator: &mut Allocator<'_>,
	instance: &vk::Instance,
	physical: vk::PhysicalDevice,
) -> Result<BudgetSnapshot, AllocError> {
	allocator.ensure_global()?;
	// SAFETY: caller supplies the exact current and future native parent lifetime/identity contract.
	let snapshot = unsafe { query_budget(allocator, instance, physical) }?;
	allocator.budget_query = capture_query(instance, physical);
	allocator.budget_cache = Some(snapshot);
	allocator.budget_operations = 0;
	Ok(snapshot)
}

/// Return an owned cached budget, refreshed on frame changes or after 30 region operations.
///
/// Local native-byte deltas are added to the last reported external usage. External
/// pressure can change immediately; this does not guarantee allocation success.
/// No arbitrary caller-supplied snapshot is admitted to the cache.
pub fn heap_budgets(allocator: &mut Allocator<'_>) -> Result<BudgetSnapshot, AllocError> {
	allocator.ensure_open()?;
	if let Some(shared) = allocator.parallel_state() {
		return Ok(shared.budget_snapshot());
	}
	if allocator.budget_cache.is_none() || allocator.budget_operations >= 30 {
		refresh_budget(allocator)?;
	}
	let mut snapshot = allocator.budget_cache.expect("refreshed owned budget");
	for (index, heap) in snapshot.heaps[..snapshot.heap_count as usize]
		.iter_mut()
		.enumerate()
	{
		let external = heap.usage.saturating_sub(heap.native_bytes);
		heap.native_bytes = allocator.heap_bytes[index];
		heap.usage = external
			.checked_add(heap.native_bytes)
			.ok_or(AllocError::InvalidProperties("budget usage overflow"))?;
		heap.limit = allocator.heap_limits[index].unwrap_or(heap.size);
		if snapshot.source == BudgetSource::Local {
			heap.budget = heap.limit;
		}
	}
	Ok(snapshot)
}

/// Mark a frame boundary for budget refresh; this never retires resources or waits.
pub fn set_current_frame_index(
	allocator: &mut Allocator<'_>,
	index: u32,
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if allocator.current_frame_index != index {
		allocator.current_frame_index = index;
		allocator.budget_cache = None;
	}
	Ok(())
}

pub(super) fn admit_native(
	allocator: &mut Allocator<'_>,
	index: u32,
	size: u64,
) -> Result<(), AllocError> {
	let snapshot = heap_budgets(allocator)?;
	let heap =
		snapshot.heaps[allocator.selector.properties.memory_types[index as usize].heap_index as usize];
	if heap
		.usage
		.checked_add(size)
		.is_none_or(|usage| usage > heap.budget)
	{
		return Err(AllocError::OutOfSpace);
	}
	Ok(())
}

impl Allocator<'_> {
	/// Explicitly refresh allocator-owned native/local budgets.
	pub fn refresh_budget(&mut self) -> Result<BudgetSnapshot, AllocError> {
		refresh_budget(self)
	}

	/// Capture a native query context under a retained parent-lifetime contract.
	/// # Safety
	/// Same identity/lifetime contract as `mem::refresh_budget_with_context`.
	pub unsafe fn refresh_budget_with_context(
		&mut self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<BudgetSnapshot, AllocError> {
		// SAFETY: caller supplies the canonical retained native context contract.
		unsafe { refresh_budget_with_context(self, instance, physical) }
	}

	/// Get owned cached/periodically refreshed heap budgets with local native-byte deltas.
	pub fn heap_budgets(&mut self) -> Result<BudgetSnapshot, AllocError> {
		heap_budgets(self)
	}

	/// Mark an explicit budget frame boundary; no completion or retirement is implied.
	pub fn set_current_frame_index(&mut self, index: u32) -> Result<(), AllocError> {
		set_current_frame_index(self, index)
	}

	pub fn current_frame_index(&self) -> u32 {
		self.current_frame_index
	}
}

pub(super) fn validate_profile(allocator: &Allocator<'_>) -> Result<(), AllocError> {
	if crate::mem::resource::extension(allocator.device, c"VK_EXT_memory_budget") {
		let query = allocator
			.budget_query
			.ok_or(AllocError::UnsupportedCapability(
				"retained physical budget context is required",
			))?;
		if query.function.is_none() {
			return Err(crate::load::CallError::MissingCommand(query.name).into());
		}
	}
	Ok(())
}

// Native growth refreshes the owned estimate; release never performs a fallible
// driver query after a caller has already destroyed its resource. Correct local
// deltas against that estimate, treating overflow conservatively as pressure.
pub(super) fn cached_pressure(allocator: &Allocator<'_>, heap: usize) -> bool {
	if let Some(shared) = allocator.parallel_state() {
		let snapshot = shared.budget_snapshot();
		return snapshot.heaps[heap].usage >= snapshot.heaps[heap].budget;
	}
	let Some(snapshot) = allocator.budget_cache else {
		return false;
	};
	let old = snapshot.heaps[heap];
	old
		.usage
		.saturating_sub(old.native_bytes)
		.saturating_add(allocator.heap_bytes[heap])
		>= old.budget
}
