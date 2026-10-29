//! Independent interval/page oracle for mixed TLSF and linear metadata.
use super::*;
use crate::mem::granularity::{Granularity, ResourceClass};
use crate::mem::test_host_allocator::FAIL_NEXT_HOST_ALLOCATION;

fn create(size: u64, algorithm: AllocationAlgorithm, page: Option<u64>) -> Metadata<'static> {
	Metadata::new_native(size, algorithm, true, Granularity(page), Default::default()).unwrap()
}

fn allocate(
	block: &mut Metadata,
	size: u64,
	class: ResourceClass,
	upper: bool,
) -> Result<Placement, AllocError> {
	block.allocate_resource(
		size,
		1,
		super::super::PlacementStrategy::MinOffset,
		upper,
		class,
	)
}

#[test]
fn compatible_classes_pack_tightly_and_conflicting_pages_are_reclaimed() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let mut block = create(2048, algorithm, Some(256));
		let a = allocate(&mut block, 17, ResourceClass::Buffer, false).unwrap();
		let b = allocate(&mut block, 19, ResourceClass::LinearImage, false).unwrap();
		let c = allocate(&mut block, 23, ResourceClass::OptimalImage, false).unwrap();
		let d = allocate(&mut block, 29, ResourceClass::OptimalImage, false).unwrap();
		assert_eq!(block.info(a).unwrap().offset, 0);
		assert_eq!(block.info(b).unwrap().offset, 17);
		assert_eq!(block.info(c).unwrap().offset, 256);
		assert_eq!(block.info(d).unwrap().offset, 279);
		assert_eq!(block.statistics().allocated_bytes, 88);
		for token in [d, c, b, a] {
			block.free(token).unwrap();
		}
		assert_eq!(block.statistics().free_bytes, 2048);
		let e = allocate(&mut block, 2048, ResourceClass::OptimalImage, false).unwrap();
		assert_eq!(block.info(e).unwrap().offset, 0);
	}
}

#[test]
fn tlsf_checks_the_far_end_of_a_hole_and_does_not_consume_padding() {
	let mut block = create(1024, AllocationAlgorithm::Tlsf, Some(256));
	let a = allocate(&mut block, 128, ResourceClass::Buffer, false).unwrap();
	let b = allocate(&mut block, 128, ResourceClass::Buffer, false).unwrap();
	let c = allocate(&mut block, 128, ResourceClass::OptimalImage, false).unwrap();
	block.free(b).unwrap();
	// [128,256) is free, but an optimal image would share page zero with a.
	let d = allocate(&mut block, 64, ResourceClass::OptimalImage, false).unwrap();
	assert_eq!(block.info(d).unwrap().offset, 384);
	block.free(a).unwrap();
	// [0,256) now shares no live conflicting boundary and is immediately reusable.
	let e = allocate(&mut block, 256, ResourceClass::Buffer, false).unwrap();
	assert_eq!(block.info(e).unwrap().offset, 0);
	block.free(e).unwrap();
	// A larger buffer ends in c's page even though its start is unconstrained.
	let f = allocate(&mut block, 257, ResourceClass::Buffer, false).unwrap();
	assert_eq!(block.info(f).unwrap().offset, 512);
	for token in [f, d, c] {
		block.free(token).unwrap();
	}
}

#[test]
fn upper_placement_separates_the_entire_extent_including_multiple_pages() {
	let mut block = create(2048, AllocationAlgorithm::Linear, Some(256));
	let a = allocate(&mut block, 128, ResourceClass::Buffer, true).unwrap();
	let b = allocate(&mut block, 320, ResourceClass::OptimalImage, true).unwrap();
	assert_eq!(block.info(a).unwrap().offset, 1920);
	assert_eq!(block.info(b).unwrap().offset, 1472);
	assert!(block.info(b).unwrap().offset + 320 <= 1792);
	// The same group can use the remaining partial page next to b.
	let c = allocate(&mut block, 31, ResourceClass::OptimalImage, true).unwrap();
	assert_eq!(block.info(c).unwrap().offset, 1441);
	let lower = allocate(&mut block, 1280, ResourceClass::LinearImage, false).unwrap();
	assert_eq!(block.info(lower).unwrap().offset, 0);
	let before = block.statistics();
	assert!(matches!(
		allocate(&mut block, 1, ResourceClass::Buffer, false),
		Err(AllocError::OutOfSpace)
	));
	assert_eq!(block.statistics(), before);
}

#[test]
fn ring_wrap_checks_next_live_page_and_releases_dead_class_information() {
	let mut block = create(1024, AllocationAlgorithm::Linear, Some(256));
	let a = allocate(&mut block, 128, ResourceClass::Buffer, false).unwrap();
	let b = allocate(&mut block, 128, ResourceClass::LinearImage, false).unwrap();
	let c = allocate(&mut block, 768, ResourceClass::OptimalImage, false).unwrap();
	block.free(a).unwrap();
	assert!(matches!(
		allocate(&mut block, 128, ResourceClass::OptimalImage, false),
		Err(AllocError::OutOfSpace)
	));
	let d = allocate(&mut block, 128, ResourceClass::Buffer, false).unwrap();
	assert_eq!(block.info(d).unwrap().offset, 0);
	block.free(b).unwrap();
	block.free(d).unwrap();
	let e = allocate(&mut block, 256, ResourceClass::OptimalImage, false).unwrap();
	assert_eq!(block.info(e).unwrap().offset, 0);
	block.free(c).unwrap();
	block.free(e).unwrap();
	assert_eq!(block.statistics().free_bytes, 1024);
}

#[test]
fn unknown_limit_excludes_conflicts_without_separating_buffer_and_linear_images() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let mut block = create(1024, algorithm, None);
		let a = allocate(&mut block, 1, ResourceClass::Buffer, false).unwrap();
		let b = allocate(&mut block, 1, ResourceClass::LinearImage, false).unwrap();
		assert_eq!(block.info(b).unwrap().offset, 1);
		assert!(matches!(
			allocate(&mut block, 1, ResourceClass::OptimalImage, false),
			Err(AllocError::OutOfSpace)
		));
		block.free(b).unwrap();
		block.free(a).unwrap();
		allocate(&mut block, 1, ResourceClass::OptimalImage, false).unwrap();
	}
}

#[test]
fn full_width_page_alignment_cannot_wrap_or_publish_failed_placements() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let mut block = create(u64::MAX, algorithm, Some(1 << 63));
		let a = allocate(&mut block, (1 << 63) + 1, ResourceClass::Buffer, false).unwrap();
		let before = block.statistics();
		assert!(matches!(
			allocate(&mut block, 1, ResourceClass::OptimalImage, false),
			Err(AllocError::OutOfSpace)
		));
		assert_eq!(block.statistics(), before);
		block.free(a).unwrap();
		if algorithm == AllocationAlgorithm::Linear {
			let a = allocate(&mut block, 1, ResourceClass::Buffer, true).unwrap();
			let b = allocate(&mut block, 1, ResourceClass::OptimalImage, true).unwrap();
			assert_eq!(block.info(a).unwrap().offset, u64::MAX - 1);
			assert_eq!(block.info(b).unwrap().offset, (1 << 63) - 1);
		}
	}
}

#[test]
fn failed_host_reservation_leaves_page_admission_and_statistics_unchanged() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let mut block = create(1024, algorithm, Some(256));
		let before = block.statistics();
		FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
		let failed = allocate(&mut block, 1, ResourceClass::OptimalImage, false);
		assert!(!FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.replace(false)));
		assert!(matches!(failed, Err(AllocError::HostAllocationFailed)));
		assert_eq!(block.statistics(), before);
		let token = allocate(&mut block, 1024, ResourceClass::Buffer, false).unwrap();
		assert_eq!(block.info(token).unwrap().offset, 0);
	}
}

#[test]
fn seeded_mixed_tlsf_placements_match_an_independent_exhaustive_page_oracle() {
	// The oracle enumerates byte offsets and checks every live interval/page pair;
	// it shares no physical links, buckets, neighbor helper or class predicate.
	for page in [1, 2, 64, 256, 512, 4096] {
		let capacity = 1025;
		let mut block = create(capacity, AllocationAlgorithm::Tlsf, Some(page));
		let mut live: Vec<(Placement, u64, u64, u8)> = Vec::new();
		let mut random = 0xacbb93913aa92122u64;
		for _ in 0..2000 {
			random ^= random << 13;
			random ^= random >> 7;
			random ^= random << 17;
			if !live.is_empty() && random.is_multiple_of(3) {
				let (token, ..) = live.swap_remove(random as usize % live.len());
				block.free(token).unwrap();
				assert!(matches!(
					block.info(token),
					Err(AllocError::UnknownAllocation)
				));
			} else {
				let size = 1 + random % 129;
				let alignment = 1 << ((random >> 8) % 8);
				let kind = ((random >> 16) % 5) as u8;
				let class = [
					ResourceClass::Buffer,
					ResourceClass::LinearImage,
					ResourceClass::OptimalImage,
					ResourceClass::UnknownImage,
					ResourceClass::Unknown,
				][usize::from(kind)];
				let expected = (0..=capacity - size).find(|offset| {
					offset.is_multiple_of(alignment)
						&& live.iter().all(|(_, start, length, previous)| {
							let end = offset + size;
							let previous_end = start + length;
							(end <= *start || previous_end <= *offset)
								&& ((kind < 3 && *previous < 3 && (kind == 2) == (*previous == 2))
									|| (end - 1) / page < start / page
									|| (previous_end - 1) / page < offset / page)
						})
				});
				match block.allocate_resource(
					size,
					alignment,
					super::super::PlacementStrategy::MinOffset,
					false,
					class,
				) {
					Ok(token) => {
						let actual = block.info(token).unwrap();
						assert_eq!(Some(actual.offset), expected, "page {page}");
						live.push((token, actual.offset, size, kind));
					}
					Err(error) => {
						assert_eq!(error, AllocError::OutOfSpace);
						assert!(expected.is_none());
					}
				}
			}
			let bytes: u64 = live.iter().map(|(_, _, size, _)| size).sum();
			assert_eq!(block.statistics().allocated_bytes, bytes);
			assert_eq!(block.statistics().free_bytes, capacity - bytes);
		}
	}
}

#[test]
fn seeded_mixed_double_stacks_match_independent_chronological_envelopes() {
	for page in [1, 64, 256, 512, 4096] {
		let capacity = 1025;
		let mut block = create(capacity, AllocationAlgorithm::Linear, Some(page));
		let anchor = allocate(&mut block, 1, ResourceClass::Buffer, true).unwrap();
		let mut live = vec![(anchor, 1024u64, 1u64, 0u8, true)];
		let mut random = 0x2ce7189a01bd9391u64;
		for _ in 0..2000 {
			random ^= random << 13;
			random ^= random >> 7;
			random ^= random << 17;
			if live.len() > 1 && random.is_multiple_of(3) {
				let index = 1 + random as usize % (live.len() - 1);
				let (token, ..) = live.swap_remove(index);
				block.free(token).unwrap();
			} else {
				let size = 1 + random % 65;
				let alignment = 1 << ((random >> 8) % 5);
				let kind = ((random >> 16) % 5) as u8;
				let upper = random & 0x10000000 != 0;
				let low = live
					.iter()
					.filter(|(_, _, _, _, upper)| !upper)
					.map(|(_, offset, size, _, _)| offset + size)
					.max()
					.unwrap_or(0);
				let high = live
					.iter()
					.filter(|(_, _, _, _, upper)| *upper)
					.map(|(_, offset, _, _, _)| *offset)
					.min()
					.unwrap();
				let valid = |offset: &u64| {
					*offset >= low
						&& offset + size <= high
						&& offset.is_multiple_of(alignment)
						&& live.iter().all(|(_, start, length, previous, _)| {
							(kind < 3 && *previous < 3 && (kind == 2) == (*previous == 2))
								|| (offset + size - 1) / page < start / page
								|| (start + length - 1) / page < offset / page
						})
				};
				let expected = if upper {
					(0..=capacity - size).rev().find(valid)
				} else {
					(0..=capacity - size).find(valid)
				};
				let class = [
					ResourceClass::Buffer,
					ResourceClass::LinearImage,
					ResourceClass::OptimalImage,
					ResourceClass::UnknownImage,
					ResourceClass::Unknown,
				][usize::from(kind)];
				match block.allocate_resource(
					size,
					alignment,
					super::super::PlacementStrategy::Default,
					upper,
					class,
				) {
					Ok(token) => {
						let offset = block.info(token).unwrap().offset;
						assert_eq!(Some(offset), expected, "page {page}, upper {upper}");
						live.push((token, offset, size, kind, upper));
					}
					Err(error) => {
						assert_eq!(error, AllocError::OutOfSpace);
						assert!(expected.is_none());
					}
				}
			}
		}
	}
}

#[test]
fn unknown_image_tiling_conflicts_with_every_native_class_including_itself() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let mut block = create(2048, algorithm, Some(256));
		let mut offsets = Vec::new();
		for class in [
			ResourceClass::Buffer,
			ResourceClass::UnknownImage,
			ResourceClass::UnknownImage,
			ResourceClass::LinearImage,
		] {
			let token = allocate(&mut block, 17, class, false).unwrap();
			offsets.push(block.info(token).unwrap().offset);
		}
		assert_eq!(
			offsets,
			if algorithm == AllocationAlgorithm::Tlsf {
				[0, 256, 512, 17]
			} else {
				[0, 256, 512, 768]
			}
		);
	}
}
