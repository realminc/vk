//! Block-order properties independent of Vulkan effects and live-token ownership.
use super::*;
use crate::mem::{AllocationAlgorithm, MemoryPriority, PlacementStrategy};

fn block(id: u64, used: u64) -> Block<'static> {
	block_sized(id, 1024, used)
}

fn block_sized(id: u64, size: u64, used: u64) -> Block<'static> {
	let mut metadata = Metadata::new(size, AllocationAlgorithm::Tlsf, false).unwrap();
	if used != 0 {
		metadata
			.allocate(used, 1, PlacementStrategy::Default, false)
			.unwrap();
	}
	Block {
		defragmentation_hold: false,
		automatic: false,
		export_types: vk::ExternalMemoryHandleTypeFlags::empty(),
		priority: MemoryPriority::default(),
		pool: None,
		handle: vk::DeviceMemory::from_raw(id),
		size,
		memory_type_index: 0,
		metadata,
		mapping: Mapping::default(),
	}
}

fn ids(blocks: &[Block]) -> Vec<u64> {
	blocks.iter().map(|block| block.handle.as_raw()).collect()
}

#[test]
fn incremental_order_swaps_only_the_first_inversion_and_preserves_equal_order() {
	let mut blocks = [block(1, 128), block(2, 512), block(3, 768), block(4, 512)];
	let domain = BlockDomain::new_from_block(&blocks[0]);
	domain.incrementally_sort(&mut blocks);
	assert_eq!(ids(&blocks), [2, 1, 3, 4]);
	domain.incrementally_sort(&mut blocks);
	assert_eq!(ids(&blocks), [2, 3, 1, 4]);
	domain.incrementally_sort(&mut blocks);
	assert_eq!(ids(&blocks), [3, 2, 1, 4]);
	domain.incrementally_sort(&mut blocks);
	assert_eq!(ids(&blocks), [3, 2, 4, 1]);
	domain.incrementally_sort(&mut blocks);
	assert_eq!(ids(&blocks), [3, 2, 4, 1]);
}

#[test]
fn interleaved_domains_keep_their_slots_and_never_compare_incompatible_blocks() {
	let mut blocks = vec![block(1, 128), block(2, 900), block(3, 768)];
	let domain = BlockDomain::new_from_block(&blocks[0]);
	for boundary in 0..5 {
		blocks[1] = block(2, 900);
		match boundary {
			0 => blocks[1].memory_type_index = 1,
			1 => blocks[1].pool = Some(7),
			2 => blocks[1].automatic = true,
			3 => blocks[1] = block_sized(2, 2048, 900),
			4 => blocks[1].priority = MemoryPriority::new(0.75).unwrap(),
			_ => unreachable!(),
		}
		assert!(!domain.contains(&blocks[1]));
		if blocks[0].handle.as_raw() == 3 {
			blocks.swap(0, 2);
		}
		domain.incrementally_sort(&mut blocks);
		assert_eq!(ids(&blocks), [3, 2, 1], "boundary={boundary}");
	}
	// Automatic vectors intentionally admit different native block sizes.
	let mut automatic = [block(1, 128), block(2, 768)];
	for block in &mut automatic {
		block.automatic = true;
	}
	automatic[1] = block_sized(2, 2048, 1900);
	automatic[1].automatic = true;
	let domain = BlockDomain::new_from_block(&automatic[0]);
	assert!(domain.contains(&automatic[1]));
	domain.incrementally_sort(&mut automatic);
	assert_eq!(ids(&automatic), [2, 1]);
}

#[test]
fn linear_and_extensive_held_vectors_keep_their_order() {
	for linear in [false, true] {
		let mut blocks = [block(1, 128), block(2, 768)];
		if linear {
			for block in &mut blocks {
				block.metadata = Metadata::new(1024, AllocationAlgorithm::Linear, false).unwrap();
			}
			blocks[0]
				.metadata
				.allocate(128, 1, PlacementStrategy::Default, false)
				.unwrap();
			blocks[1]
				.metadata
				.allocate(768, 1, PlacementStrategy::Default, false)
				.unwrap();
		} else {
			blocks[1].defragmentation_hold = true;
		}
		BlockDomain::new_from_block(&blocks[0]).incrementally_sort(&mut blocks);
		assert_eq!(ids(&blocks), [1, 2]);
	}
}
