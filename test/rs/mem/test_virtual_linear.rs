//! Public linear stack/ring identity and independent spatial/accounting contracts.
use vk::{
	AllocError, VirtualBlock,
	mem::{AllocationAlgorithm, PlacementStrategy, virtual_block},
};

fn block(size: u64) -> VirtualBlock<'static> {
	VirtualBlock::new_with_algorithm(size, AllocationAlgorithm::Linear).unwrap()
}

#[test]
fn stack_rewinds_tombstones_without_reusing_live_middle_holes() {
	let mut block = block(1024);
	let mut a = block.allocate(257, 1, PlacementStrategy::Default).unwrap();
	let mut b = block
		.allocate(257, 1, PlacementStrategy::MinOffset)
		.unwrap();
	let mut c = block.allocate(257, 1, PlacementStrategy::MinTime).unwrap();
	block.free(&mut b).unwrap();
	assert_eq!(block.statistics().largest_free_region, 257);
	assert!(matches!(
		block.allocate(257, 1, PlacementStrategy::MinMemory),
		Err(AllocError::OutOfSpace)
	));
	block.free(&mut c).unwrap();
	let mut d = block.allocate(512, 1, PlacementStrategy::Default).unwrap();
	assert_eq!(block.allocation_info(&d).unwrap().offset, 257);
	assert_eq!(block.free(&mut c), Err(AllocError::UnknownAllocation));
	block.free(&mut a).unwrap();
	block.free(&mut d).unwrap();
	assert_eq!(block.statistics().largest_free_region, 1024);
}

#[test]
fn ring_wraps_rotates_and_reuses_only_completed_prefixes() {
	let mut block = block(1024);
	let mut live = Vec::new();
	for _ in 0..4 {
		live.push(block.allocate(256, 1, PlacementStrategy::Default).unwrap());
	}
	for expected in [0, 256, 512, 768, 0, 256, 512, 768] {
		let mut first = live.remove(0);
		assert_eq!(block.allocation_info(&first).unwrap().offset, expected);
		block.free(&mut first).unwrap();
		let replacement =
			virtual_block::allocate(&mut block, 256, 1, PlacementStrategy::MinOffset).unwrap();
		assert_eq!(
			block.allocation_info(&replacement).unwrap().offset,
			expected
		);
		assert_eq!(block.statistics().free_bytes, 0);
		live.push(replacement);
	}
	for mut token in live {
		block.free(&mut token).unwrap();
	}
	assert_eq!(block.statistics().free_region_count, 1);
}

#[test]
fn double_stack_keeps_alignment_rewinds_and_rejects_overlap() {
	let mut block = block(4099);
	let mut low = block.allocate(257, 64, PlacementStrategy::Default).unwrap();
	let mut high = virtual_block::allocate_upper(&mut block, 257, 64).unwrap();
	assert_eq!(block.allocation_info(&high).unwrap().offset, 3840);
	let mut inner = block.allocate_upper(257, 64).unwrap();
	assert_eq!(block.allocation_info(&inner).unwrap().offset, 3520);
	assert!(matches!(
		block.allocate(3520, 1, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	block.free(&mut high).unwrap();
	assert_eq!(block.allocation_info(&inner).unwrap().offset, 3520);
	block.free(&mut inner).unwrap();
	let mut reuse = block.allocate_upper(257, 64).unwrap();
	assert_eq!(block.allocation_info(&reuse).unwrap().offset, 3840);
	block.free(&mut low).unwrap();
	block.free(&mut reuse).unwrap();
	assert_eq!(block.statistics().free_bytes, 4099);
}

#[test]
fn foreign_stale_clear_and_invalid_requests_leave_placements_intact() {
	let mut first = block(1024);
	let mut second = block(1024);
	let mut old = first.allocate_upper(64, 64).unwrap();
	assert_eq!(second.free(&mut old), Err(AllocError::WrongAllocator));
	first.clear();
	let current = first.allocate_upper(64, 64).unwrap();
	assert_eq!(first.free(&mut old), Err(AllocError::UnknownAllocation));
	let before = first.statistics();
	for (size, align) in [(0, 1), (1, 0), (1, 3)] {
		assert!(matches!(
			first.allocate_upper(size, align),
			Err(AllocError::InvalidRequest(_))
		));
	}
	assert_eq!(first.statistics(), before);
	assert_eq!(first.allocation_info(&current).unwrap().offset, 960);
	let mut tlsf = VirtualBlock::new(1024).unwrap();
	assert!(matches!(
		tlsf.allocate_upper(1, 1),
		Err(AllocError::UnsupportedCapability(_))
	));
}

#[test]
fn ring_rejects_upper_without_mutation_and_allows_it_after_ring_drains() {
	let mut block = block(1024);
	let mut a = block.allocate(512, 1, PlacementStrategy::Default).unwrap();
	let mut b = block.allocate(512, 1, PlacementStrategy::Default).unwrap();
	block.free(&mut a).unwrap();
	let mut wrap = block.allocate(512, 1, PlacementStrategy::Default).unwrap();
	let before = block.statistics();
	assert!(matches!(
		block.allocate_upper(1, 1),
		Err(AllocError::InvalidRequest(_))
	));
	assert_eq!(block.statistics(), before);
	block.free(&mut b).unwrap();
	block.free(&mut wrap).unwrap();
	let upper = block.allocate_upper(1, 1).unwrap();
	assert_eq!(block.allocation_info(&upper).unwrap().offset, 1023);
}

#[test]
fn seeded_tombstone_compaction_and_clear_preserve_spatial_and_generation_invariants() {
	for upper in [false, true] {
		let capacity = 8193;
		let mut block = block(capacity);
		let mut live = Vec::new();
		let mut seed = 0x3aa9_2122_1234_5678u64;
		for step in 0..10000 {
			seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
			if !live.is_empty() && seed.is_multiple_of(3) {
				let (mut token, _, _) = live.remove((seed >> 20) as usize % live.len());
				block.free(&mut token).unwrap();
				assert!(matches!(
					block.allocation_info(&token),
					Err(AllocError::UnknownAllocation)
				));
			} else {
				let (size, align) = (1 + (seed >> 16) % 97, 1 << ((seed >> 40) % 8));
				let result = if upper {
					block.allocate_upper(size, align)
				} else {
					block.allocate(size, align, PlacementStrategy::MinOffset)
				};
				match result {
					Ok(token) => {
						let info = block.allocation_info(&token).unwrap();
						assert_eq!(info.offset % align, 0);
						assert!(info.offset + size <= capacity);
						assert!(
							live
								.iter()
								.all(|(_, offset, length)| info.offset + size <= *offset
									|| *offset + *length <= info.offset)
						);
						live.push((token, info.offset, size));
					}
					Err(AllocError::OutOfSpace) => {}
					Err(error) => panic!("unexpected failure: {error}"),
				}
			}
			let stats = block.statistics();
			assert_eq!(stats.allocation_count, live.len());
			assert_eq!(
				stats.allocated_bytes,
				live.iter().map(|(_, _, size)| size).sum::<u64>()
			);
			let mut spans: Vec<_> = live
				.iter()
				.map(|(_, offset, size)| (*offset, *offset + *size))
				.collect();
			spans.sort_unstable();
			spans.push((capacity, capacity));
			let (mut cursor, mut count, mut largest) = (0, 0, 0);
			for (start, end) in spans {
				if start > cursor {
					count += 1;
					largest = largest.max(start - cursor);
				}
				cursor = end;
			}
			assert_eq!(
				(stats.free_region_count, stats.largest_free_region),
				(count, largest)
			);
			if step % 503 == 502 {
				block.clear();
				for (token, _, _) in live.drain(..) {
					assert!(matches!(
						block.allocation_info(&token),
						Err(AllocError::UnknownAllocation)
					));
				}
			}
		}
	}
}
