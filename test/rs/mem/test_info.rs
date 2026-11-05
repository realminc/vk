//! Live snapshot identity, byte extents and per-region mapping transitions.
use super::*;
use crate::mem::{
	AllocationCreateInfo, AllocationInfo, AllocationPolicy, AllocatorStats, BudgetSnapshot,
	DebugMemorySettings, DefragmentationInfo, DefragmentationStats, DetailedStatistics,
	DeviceMemoryCallbacks, HeapBudget, MemoryPriority, MemoryStatistics, MemoryUsage, PoolCreateInfo,
	PoolStats, TrimStats, allocation_info,
};

#[test]
fn snapshots_are_checked_without_native_effects_and_track_explicit_mapping() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic native storage; pointers are compared, never dereferenced.
	unsafe {
		let mut allocation = allocator.allocate(&request()).unwrap();
		clear();
		let snapshot: crate::AllocationInfo = allocation_info(&allocator, &allocation).unwrap();
		assert_eq!(
			snapshot,
			AllocationInfo {
				device_memory: vk::DeviceMemory::from_raw(100),
				offset: 0,
				size: 257,
				reserved_size: 257,
				memory_type_index: 0,
				heap_index: 0,
				memory_properties: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
				mapped_pointer: None,
				map_count: 0,
				native_size: 257,
				is_separate: true,
				user_data: 0,
			}
		);
		assert_eq!(allocator.allocation_info(&allocation).unwrap(), snapshot);
		assert!(events().is_empty());
		let pointer = allocator.map(&allocation).unwrap();
		clear();
		assert_eq!(
			allocator
				.allocation_info(&allocation)
				.unwrap()
				.mapped_pointer,
			Some(pointer)
		);
		assert!(events().is_empty());
		allocator.unmap(&allocation).unwrap();
		assert_eq!(allocator.allocation_info(&allocation).unwrap(), snapshot);
		allocator.free(&mut allocation).unwrap();
		clear();
		assert_eq!(
			allocator.allocation_info(&allocation),
			Err(AllocError::UnknownAllocation)
		);
		assert!(events().is_empty());
		allocator.close().unwrap();
		assert_eq!(
			allocator.allocation_info(&allocation),
			Err(AllocError::Closed)
		);
	}
}

#[test]
fn foreign_tokens_and_recycled_native_handles_cannot_revive_snapshots() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	let mut other = fixture.allocator();
	// SAFETY: independently tracked mock allocations; explicit cleanup, no accesses.
	unsafe {
		let mut old = allocator.allocate(&request()).unwrap();
		let original = allocator.allocation_info(&old).unwrap();
		clear();
		assert_eq!(other.allocation_info(&old), Err(AllocError::WrongAllocator));
		assert!(events().is_empty());
		allocator.free(&mut old).unwrap();
		STATE.with_borrow_mut(|s| s.next_memory = original.device_memory.as_raw());
		let mut replacement = allocator.allocate(&request()).unwrap();
		assert_eq!(replacement.handle(), original.device_memory);
		assert_eq!(
			allocator.allocation_info(&old),
			Err(AllocError::UnknownAllocation)
		);
		assert_eq!(allocator.allocation_info(&replacement).unwrap(), original);
		allocator.free(&mut replacement).unwrap();
		allocator.close().unwrap();
		other.close().unwrap();
	}
}

#[test]
fn pooled_snapshots_expose_region_offsets_and_only_the_regions_own_mapping() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 4096 },
		persistent_map: true,
		..Default::default()
	};
	// SAFETY: mock regions isolate non-coherent atoms; no synthetic pointer access.
	unsafe {
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					persistent_map: false,
					..options
				},
			)
			.unwrap();
		clear();
		let first = allocator.allocation_info(&x).unwrap();
		let second = allocator.allocation_info(&y).unwrap();
		assert_eq!(first.device_memory, second.device_memory);
		assert_eq!(
			(
				first.offset,
				second.offset,
				second.size,
				second.reserved_size
			),
			(0, 320, 257, 320)
		);
		assert_eq!(first.mapped_pointer.unwrap().addr(), 0x4000);
		assert_eq!(second.mapped_pointer, None);
		assert!(events().is_empty());
		let pointer = allocator.map(&y).unwrap();
		assert_eq!(pointer.addr(), 0x4000 + 320);
		assert_eq!(
			allocator.allocation_info(&y).unwrap().mapped_pointer,
			Some(pointer)
		);
		allocator.unmap(&y).unwrap();
		assert_eq!(allocator.allocation_info(&y).unwrap(), second);
		assert_eq!(allocator.allocation_info(&x).unwrap(), first);
		allocator.destroy_buffer(a, &mut x).unwrap();
		assert_eq!(allocator.allocation_info(&y).unwrap(), second);
		allocator.destroy_buffer(b, &mut y).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn record_constructors_delegate_to_default() {
	assert_eq!(DebugMemorySettings::new(), DebugMemorySettings::default());
	assert_eq!(AllocatorStats::new(), AllocatorStats::default());
	assert_eq!(HeapBudget::new(), HeapBudget::default());
	assert_eq!(BudgetSnapshot::new(), BudgetSnapshot::default());
	assert_eq!(TrimStats::new(), TrimStats::default());
	assert_eq!(PoolCreateInfo::new(), PoolCreateInfo::default());
	assert_eq!(PoolStats::new(), PoolStats::default());
	assert_eq!(DefragmentationStats::new(), DefragmentationStats::default());
	assert_eq!(DetailedStatistics::new(), DetailedStatistics::default());
	assert_eq!(MemoryStatistics::new(), MemoryStatistics::default());
	let callbacks = DeviceMemoryCallbacks::new();
	assert!(callbacks.allocated.is_none());
	assert!(callbacks.freed.is_none());
	assert!(callbacks.p_user_data.is_null());
	let options = AllocationCreateInfo::new();
	assert!(options.pool.is_none());
	assert_eq!(options.usage, MemoryUsage::Explicit);
	assert_eq!(options.priority, MemoryPriority::default());
	assert_eq!(options.memory_type_bits, u32::MAX);
	let defragmentation = DefragmentationInfo::new();
	assert!(defragmentation.pool.is_none());
	assert_eq!(defragmentation.max_bytes, u64::MAX);
	assert_eq!(defragmentation.max_allocations, usize::MAX);
}
