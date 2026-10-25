//! Explicit memory-budget snapshots and local per-heap policy limits.
//! Donor: pinned VMA GetHeapBudgets/SetCurrentFrameIndex budget separation.
//! Adaptation: queries are explicit; no hot-path driver queries or stale-budget promises.
use super::Allocator;
use crate::{
	self as vk,
	memory::{AllocError, BudgetSnapshot, BudgetSource, HeapBudget},
};

impl Allocator<'_> {
	/// Set a local native-byte ceiling. Existing allocations are never evicted.
	pub fn set_heap_limit(&mut self, heap: u32, limit: u64) -> Result<(), AllocError> {
		set_heap_limit(self, heap, limit)
	}

	/// Heap-size/local-accounting estimate, without querying the driver.
	pub fn budget_snapshot(&self) -> BudgetSnapshot {
		budget_snapshot(self)
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

/// Change a local policy ceiling; this never reserves native heap capacity.
pub fn set_heap_limit(
	allocator: &mut Allocator<'_>,
	heap: u32,
	limit: u64,
) -> Result<(), AllocError> {
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
	let mut snapshot = budget_snapshot(allocator);
	if !crate::memory::resource::extension(allocator.device, c"VK_EXT_memory_budget") {
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
