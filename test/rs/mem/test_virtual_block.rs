//! Independent interval-model contracts for the public host-only allocator.
use vk::{
	AllocError, VirtualBlock,
	mem::{AllocationAlgorithm, PlacementStrategy, SynchronizedVirtualBlock, virtual_block},
};

#[test]
fn synchronized_virtual_block_public_operations_reject_foreign_and_stale_tokens() {
	let block = SynchronizedVirtualBlock::new(256).unwrap();
	let other = SynchronizedVirtualBlock::new(256).unwrap();
	let mut region = block.allocate(64, 16, PlacementStrategy::Default).unwrap();
	assert_eq!(other.free(&mut region), Err(AllocError::WrongAllocator));
	assert_eq!(block.allocation_info(&region).unwrap().offset, 0);
	block.free(&mut region).unwrap();
	assert_eq!(block.free(&mut region), Err(AllocError::UnknownAllocation));
	assert!(block.is_empty().unwrap());
	let mut next = block.allocate(32, 8, PlacementStrategy::Default).unwrap();
	block.clear().unwrap();
	assert_eq!(
		block.allocation_info(&next),
		Err(AllocError::UnknownAllocation)
	);
	assert_eq!(block.free(&mut next), Err(AllocError::UnknownAllocation));
}

#[test]
fn synchronized_linear_virtual_block_keeps_upper_placement_explicit() {
	let block =
		SynchronizedVirtualBlock::new_with_algorithm(256, AllocationAlgorithm::Linear).unwrap();
	let mut lower = block.allocate(64, 16, PlacementStrategy::Default).unwrap();
	let mut upper = block.allocate_upper(64, 16).unwrap();
	assert_eq!(block.allocation_info(&upper).unwrap().offset, 192);
	assert_eq!(block.statistics().unwrap().allocation_count, 2);
	block.free(&mut upper).unwrap();
	block.free(&mut lower).unwrap();
	assert!(block.is_empty().unwrap());
}

#[test]
fn donor_virtual_block_vector_and_checked_ownership() {
	let mut block = VirtualBlock::new(16 << 20).unwrap();
	let mut first = block
		.allocate(8 << 20, 256, PlacementStrategy::Default)
		.unwrap();
	let mut second =
		virtual_block::allocate(&mut block, 4 << 20, 256, PlacementStrategy::Default).unwrap();
	assert!(matches!(
		block.allocate(8 << 20, 256, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	virtual_block::free(&mut block, &mut second).unwrap();
	let mut third = block
		.allocate(8 << 20, 256, PlacementStrategy::Default)
		.unwrap();
	assert_eq!(block.statistics().allocated_bytes, 16 << 20);
	assert_eq!(
		virtual_block::allocation_info(&block, &third)
			.unwrap()
			.offset,
		8 << 20
	);
	let mut foreign = VirtualBlock::new(16 << 20).unwrap();
	assert_eq!(foreign.free(&mut first), Err(AllocError::WrongAllocator));
	block.free(&mut first).unwrap();
	assert_eq!(block.free(&mut first), Err(AllocError::UnknownAllocation));
	block.clear();
	assert!(matches!(
		block.allocation_info(&third),
		Err(AllocError::UnknownAllocation)
	));
	let mut current = block
		.allocate(16 << 20, 1, PlacementStrategy::Default)
		.unwrap();
	assert_eq!(block.free(&mut third), Err(AllocError::UnknownAllocation));
	block.free(&mut current).unwrap();
	assert_eq!(block.statistics().largest_free_region, 16 << 20);
}

#[test]
fn fixed_seed_traces_match_an_independent_sorted_interval_model() {
	for strategy in [
		PlacementStrategy::Default,
		PlacementStrategy::MinMemory,
		PlacementStrategy::MinTime,
		PlacementStrategy::MinOffset,
	] {
		let capacity = 4099;
		let mut block = VirtualBlock::new(capacity).unwrap();
		let mut live = Vec::new();
		let mut seed = 0xe23a_7c51_1269_aa43u64;
		for step in 0..12000 {
			seed = seed
				.wrapping_mul(6364136223846793005)
				.wrapping_add(1442695040888963407);
			if !live.is_empty() && seed.is_multiple_of(3) {
				let (mut handle, _, _) = live.swap_remove((seed >> 16) as usize % live.len());
				block.free(&mut handle).unwrap();
			} else {
				let (size, alignment) = (1 + (seed >> 20) % 257, 1 << ((seed >> 40) % 10));
				let mut intervals: Vec<_> = live
					.iter()
					.map(|(_, offset, size)| (*offset, *offset + *size))
					.collect();
				intervals.sort_unstable();
				intervals.push((capacity, capacity));
				let (mut cursor, mut expected) = (0u64, None);
				for (start, end) in intervals {
					let aligned = cursor.div_ceil(alignment) * alignment;
					if aligned <= start && size <= start - aligned {
						expected = Some(aligned);
						break;
					}
					cursor = end;
				}
				match block.allocate(size, alignment, strategy) {
					Ok(handle) => {
						let info = block.allocation_info(&handle).unwrap();
						assert!(expected.is_some());
						assert_eq!(info.offset % alignment, 0);
						assert_eq!(info.size, size);
						assert!(info.offset + size <= capacity);
						if strategy == PlacementStrategy::MinOffset {
							assert_eq!(Some(info.offset), expected);
						}
						for (_, offset, bytes) in &live {
							assert!(info.offset + size <= *offset || *offset + *bytes <= info.offset);
						}
						live.push((handle, info.offset, size));
					}
					Err(AllocError::OutOfSpace) => assert!(expected.is_none()),
					Err(error) => panic!("unexpected placement error: {error}"),
				}
			}
			let stats = block.statistics();
			assert_eq!(stats.allocation_count, live.len());
			assert_eq!(
				stats.allocated_bytes,
				live.iter().map(|(_, _, size)| size).sum::<u64>()
			);
			assert_eq!(stats.allocated_bytes + stats.free_bytes, capacity);
			// Independently reconstruct all positive free intervals and largest span.
			let mut spans: Vec<_> = live
				.iter()
				.map(|(_, offset, size)| (*offset, *offset + *size))
				.collect();
			spans.sort_unstable();
			spans.push((capacity, capacity));
			let (mut end, mut count, mut largest) = (0, 0, 0);
			for (start, next) in spans {
				if start > end {
					count += 1;
					largest = largest.max(start - end);
				}
				end = next;
			}
			assert_eq!(stats.free_region_count, count);
			assert_eq!(stats.largest_free_region, largest);
			if step % 997 == 996 {
				block.clear();
				live.clear();
			}
		}
	}
}

#[test]
fn invalid_requests_and_fragmentation_fail_without_changing_statistics() {
	assert!(matches!(
		VirtualBlock::new(0),
		Err(AllocError::InvalidRequest(_))
	));
	let mut block = VirtualBlock::new(128).unwrap();
	for (size, alignment) in [(0, 1), (1, 0), (1, 3)] {
		let before = block.statistics();
		assert!(matches!(
			block.allocate(size, alignment, PlacementStrategy::Default),
			Err(AllocError::InvalidRequest(_))
		));
		assert_eq!(block.statistics(), before);
	}
	let mut slots: Vec<_> = (0..8)
		.map(|_| block.allocate(16, 1, PlacementStrategy::Default).unwrap())
		.collect();
	for i in [0, 2, 4, 6] {
		block.free(&mut slots[i]).unwrap();
	}
	assert_eq!(block.statistics().free_bytes, 64);
	assert_eq!(block.statistics().largest_free_region, 16);
	assert!(matches!(
		block.allocate(17, 1, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	block.clear();
	assert_eq!(block.statistics().largest_free_region, 128);
}

#[test]
fn dropping_a_token_does_not_release_or_reuse_its_live_region() {
	let mut block = VirtualBlock::new(64).unwrap();
	{
		let _allocation = block.allocate(64, 1, PlacementStrategy::Default).unwrap();
	}
	assert_eq!(block.statistics().allocation_count, 1);
	assert!(matches!(
		block.allocate(1, 1, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	block.clear();
	assert_eq!(block.statistics().free_bytes, 64);
}
