use super::*;

fn allocation(size: u64) -> Allocation {
	Allocation {
		handle: vk::DeviceMemory::from_raw(1),
		size,
		offset: 0,
		block_size: size,
		reserved_size: size,
		memory_type_index: 0,
		heap_index: 0,
		flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
		atom: 64,
		owner: Arc::new(()),
		serial: 1,
	}
}

#[test]
fn cache_range_rounds_both_ends_and_observes_allocation_end_exception() {
	let a = allocation(257);
	let middle = a.mapped_range(65, 2).unwrap().unwrap();
	assert_eq!((middle.offset, middle.size), (64, 64));
	let tail = a.mapped_range(255, 2).unwrap().unwrap();
	assert_eq!((tail.offset, tail.size), (192, 65));
	let tail = a.mapped_range(255, vk::WHOLE_SIZE).unwrap().unwrap();
	assert_eq!((tail.offset, tail.size), (192, 65));
}

#[test]
fn cache_range_rejects_overflow_and_bounds_before_coherent_noop() {
	let mut a = allocation(257);
	a.flags = a.flags | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	for (offset, size) in [(258, 0), (256, 2), (u64::MAX - 1, 3)] {
		assert!(matches!(
			a.mapped_range(offset, size),
			Err(AllocError::InvalidRange)
		));
	}
	assert!(a.mapped_range(0, vk::WHOLE_SIZE).unwrap().is_none());
	assert!(a.mapped_range(257, 0).unwrap().is_none());
}

#[test]
fn cache_range_handles_rounding_overflow_and_invalid_allocation_states() {
	let mut a = allocation(u64::MAX - 1);
	let range = a.mapped_range(u64::MAX - 2, 1).unwrap().unwrap();
	assert_eq!(range.offset + range.size, a.size());
	a.handle = vk::DeviceMemory::NULL;
	assert!(matches!(
		a.mapped_range(0, 1),
		Err(AllocError::UnknownAllocation)
	));
	a.handle = vk::DeviceMemory::from_raw(1);
	a.flags = vk::MemoryPropertyFlags::empty();
	assert!(matches!(
		a.mapped_range(0, 1),
		Err(AllocError::NotHostVisible)
	));
}
