use super::*;
use crate::mem::test_host_allocator::FAIL_NEXT_HOST_ALLOCATION;
use std::collections::BTreeSet;

fn validate(t: &Tlsf) {
	let mut physical = BTreeSet::new();
	let mut free = BTreeSet::new();
	let (mut index, mut previous, mut offset, mut taken, mut bytes) = (t.head, NONE, 0, 0, 0);
	loop {
		assert!(physical.insert(index));
		let node = &t.nodes[index];
		assert!(node.state != State::Unused);
		assert_eq!(node.previous, previous);
		assert_eq!(node.offset, offset);
		offset = offset.checked_add(node.size).unwrap();
		if node.state == State::Taken {
			taken += 1;
			bytes += node.size;
		} else if index != t.tail {
			assert!(node.size != 0);
			assert!(t.nodes[node.next].state != State::Free);
			free.insert(index);
		}
		if index == t.tail {
			assert_eq!(node.next, NONE);
			assert!(node.state == State::Free);
			break;
		}
		assert!(node.size != 0);
		previous = index;
		index = node.next;
	}
	assert_eq!(offset, t.size);
	assert_eq!(taken, t.allocations);
	let mut listed = BTreeSet::new();
	let mut first = 0u64;
	let mut second = [0u32; CLASSES];
	for (bucket, head) in t.heads.iter().enumerate() {
		let (mut index, mut previous) = (*head, NONE);
		while index != NONE {
			assert!(listed.insert(index));
			assert_eq!(list(t.nodes[index].size), bucket);
			assert_eq!(t.nodes[index].previous_free, previous);
			assert!(t.nodes[index].state == State::Free);
			first |= 1 << (bucket / 32);
			second[bucket / 32] |= 1 << (bucket % 32);
			previous = index;
			index = t.nodes[index].next_free;
		}
	}
	assert_eq!(free, listed);
	assert_eq!(first, t.first_map);
	assert_eq!(second, t.second_maps);
	assert_eq!(t.free_count, free.len());
	assert_eq!(
		t.free_bytes,
		free.iter().map(|&i| t.nodes[i].size).sum::<u64>()
	);
	let stats = t.statistics();
	assert_eq!(stats.allocated_bytes, bytes);
	assert_eq!(stats.free_bytes, t.size - bytes);
	let mut recycled = BTreeSet::new();
	let mut index = t.recycled;
	while index != NONE {
		assert!(recycled.insert(index));
		assert!(t.nodes[index].state == State::Unused);
		index = t.nodes[index].next_free;
	}
	assert_eq!(recycled.len(), t.recycled_count);
	assert_eq!(physical.len() + recycled.len(), t.nodes.len());
}

#[test]
fn seeded_split_coalesce_reuse_traces_preserve_every_internal_invariant() {
	for strategy in [
		PlacementStrategy::Default,
		PlacementStrategy::MinMemory,
		PlacementStrategy::MinTime,
		PlacementStrategy::MinOffset,
	] {
		let mut random = 0x2ce7_189a_01bd_9391u64;
		let mut t = Tlsf::new(8193).unwrap();
		let mut live = Vec::new();
		for step in 0..10000 {
			random ^= random << 13;
			random ^= random >> 7;
			random ^= random << 17;
			if !live.is_empty() && random.is_multiple_of(3) {
				let (index, serial) = live.swap_remove(random as usize % live.len());
				t.free(index, serial).unwrap();
			} else {
				match t.allocate(
					1 + (random >> 8) % 900,
					1 << ((random >> 20) % 10),
					strategy,
				) {
					Ok(handle) => live.push(handle),
					Err(AllocError::OutOfSpace) => (),
					Err(error) => panic!("{error}"),
				}
			}
			validate(&t);
			if step % 991 == 990 {
				t.clear();
				live.clear();
				validate(&t);
			}
		}
		for (index, serial) in live {
			t.free(index, serial).unwrap();
			validate(&t);
		}
		assert_eq!(t.statistics().largest_free_region, 8193);
	}
}

#[test]
fn full_width_classes_and_alignment_overflow_do_not_wrap_or_hide_large_holes() {
	let mut t = Tlsf::new(u64::MAX).unwrap();
	let (first, serial) = t.allocate(1 << 55, 1, PlacementStrategy::Default).unwrap();
	let (guard, guard_serial) = t
		.allocate(u64::MAX - (1 << 55) - 1, 1, PlacementStrategy::Default)
		.unwrap();
	t.free(first, serial).unwrap();
	// The donor's u32 first-level bitmap truncates these classes. The Rust
	// port deliberately widens it; the tail is too small for this aligned request.
	let (replacement, replacement_serial) = t
		.allocate(1 << 54, 1 << 63, PlacementStrategy::MinMemory)
		.unwrap();
	assert_eq!(t.info(replacement, replacement_serial).unwrap().offset, 0);
	t.free(replacement, replacement_serial).unwrap();
	t.free(guard, guard_serial).unwrap();
	validate(&t);
	let (whole, serial) = t.allocate(u64::MAX, 1, PlacementStrategy::Default).unwrap();
	validate(&t);
	assert_eq!(t.statistics().free_bytes, 0);
	assert!(matches!(
		t.allocate(1, 1, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	t.free(whole, serial).unwrap();
	validate(&t);
	let (prefix, serial) = t
		.allocate(u64::MAX - 1, 1, PlacementStrategy::Default)
		.unwrap();
	assert!(matches!(
		t.allocate(1, 8, PlacementStrategy::Default),
		Err(AllocError::OutOfSpace)
	));
	t.free(prefix, serial).unwrap();
	validate(&t);
}

#[test]
fn arena_generation_reuse_clear_and_exhaustion_fail_before_mutation() {
	let mut t = Tlsf::new(256).unwrap();
	let (old, serial) = t.allocate(256, 1, PlacementStrategy::Default).unwrap();
	t.free(old, serial).unwrap();
	let current = t.allocate(256, 1, PlacementStrategy::Default).unwrap();
	assert!(matches!(
		t.info(old, serial),
		Err(AllocError::UnknownAllocation)
	));
	t.clear();
	assert!(matches!(
		t.info(current.0, current.1),
		Err(AllocError::UnknownAllocation)
	));
	t.next_serial = u64::MAX;
	let before = t.statistics();
	assert!(matches!(
		t.allocate(1, 1, PlacementStrategy::Default),
		Err(AllocError::SerialExhausted)
	));
	assert_eq!(before, t.statistics());
	validate(&t);
}

#[test]
fn actual_arena_growth_oom_retains_live_placements_and_donor_tie_order() {
	let mut t = Tlsf::new(4096).unwrap();
	let live = t.allocate(17, 1, PlacementStrategy::Default).unwrap();
	// Fill arena capacity exactly, then require growth on the next placement.
	while t.nodes.len() + 2 <= t.nodes.capacity() {
		t.allocate(1, 1, PlacementStrategy::Default).unwrap();
	}
	let before = t.statistics();
	let info = t.info(live.0, live.1).unwrap();
	let serial = t.next_serial;
	FAIL_NEXT_HOST_ALLOCATION.set(true);
	let result = t.allocate(19, 64, PlacementStrategy::Default);
	let injected = !FAIL_NEXT_HOST_ALLOCATION.replace(false);
	assert!(injected);
	assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
	assert_eq!(before, t.statistics());
	assert_eq!(info, t.info(live.0, live.1).unwrap());
	assert_eq!(t.next_serial, serial);
	validate(&t);
	t.allocate(19, 64, PlacementStrategy::Default).unwrap();
	validate(&t);
}

#[test]
fn every_u64_power_of_two_boundary_can_reuse_and_coalesce_its_size_class() {
	for bit in 0..64 {
		let size = 1u64 << bit;
		let mut t = Tlsf::new(u64::MAX).unwrap();
		let first = t.allocate(size, 1, PlacementStrategy::Default).unwrap();
		let guard = t.allocate(1, 1, PlacementStrategy::Default).unwrap();
		t.free(first.0, first.1).unwrap();
		validate(&t);
		let replacement = t
			.allocate((size / 2).max(1), 1, PlacementStrategy::MinMemory)
			.unwrap();
		assert_eq!(t.info(replacement.0, replacement.1).unwrap().offset, 0);
		t.free(replacement.0, replacement.1).unwrap();
		t.free(guard.0, guard.1).unwrap();
		validate(&t);
		assert_eq!(t.statistics().largest_free_region, u64::MAX);
	}
}
