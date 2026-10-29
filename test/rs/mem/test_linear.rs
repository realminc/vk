//! Private failure/generation tests, separate from the public donor replay.
use super::*;
use crate::mem::test_host_allocator::FAIL_NEXT_HOST_ALLOCATION;

#[test]
fn failed_metadata_reservation_preserves_mode_cursor_and_generation() {
	for upper in [false, true] {
		let mut block = Linear::new(4099, true).unwrap();
		FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
		let result = block.allocate(257, 64, upper);
		let consumed = FAIL_NEXT_HOST_ALLOCATION.with(|flag| !flag.replace(false));
		assert!(consumed);
		assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
		assert_eq!(block.statistics().allocation_count, 0);
		assert_eq!(block.next_serial, 1);
		assert!(block.mode == Mode::Empty);
		let token = block.allocate(257, 64, upper).unwrap();
		let Placement::Linear { offset, serial } = token else {
			unreachable!()
		};
		assert_eq!(offset, if upper { 3840 } else { 0 });
		assert_eq!(serial, 1);
	}
}

#[test]
fn maximum_extent_alignment_and_generation_exhaustion_are_checked() {
	let mut block = Linear::new(u64::MAX, true).unwrap();
	let first = block.allocate(1, 1, true).unwrap();
	let Placement::Linear { offset, serial } = first else {
		unreachable!()
	};
	assert_eq!(offset, u64::MAX - 1);
	assert_eq!(block.info(offset, serial).unwrap().size, 1);
	assert!(matches!(
		block.allocate(u64::MAX, 1, false),
		Err(AllocError::OutOfSpace)
	));
	block.free(offset, serial).unwrap();
	block.next_serial = u64::MAX;
	assert!(matches!(
		block.allocate(1, 1, false),
		Err(AllocError::SerialExhausted)
	));
	assert_eq!(block.statistics().free_bytes, u64::MAX);
	block.clear();
	assert!(matches!(
		block.allocate(1, 1, false),
		Err(AllocError::SerialExhausted)
	));
}

#[test]
fn multi_block_metadata_does_not_wrap_into_a_ring() {
	let mut block = Linear::new(1024, false).unwrap();
	let a = block.allocate(512, 1, false).unwrap();
	block.allocate(512, 1, false).unwrap();
	let Placement::Linear { offset, serial } = a else {
		unreachable!()
	};
	block.free(offset, serial).unwrap();
	assert!(matches!(
		block.allocate(512, 1, false),
		Err(AllocError::OutOfSpace)
	));
	assert_eq!(block.statistics().free_bytes, 512);
}

#[test]
fn ring_transition_metadata_oom_preserves_live_tail_and_lower_mode() {
	let mut block = Linear::new(1024, true).unwrap();
	let first = block.allocate(512, 1, false).unwrap();
	let last = block.allocate(512, 1, false).unwrap();
	let Placement::Linear { offset, serial } = first else {
		unreachable!()
	};
	block.free(offset, serial).unwrap();
	let before = block.statistics();
	let generation = block.next_serial;
	FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
	let failed = block.allocate(512, 1, false);
	assert!(!FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.replace(false)));
	assert!(matches!(failed, Err(AllocError::HostAllocationFailed)));
	assert_eq!(block.statistics(), before);
	assert_eq!(block.next_serial, generation);
	assert!(block.mode == Mode::Empty);
	let Placement::Linear { offset, serial } = last else {
		unreachable!()
	};
	assert_eq!(block.info(offset, serial).unwrap().offset, 512);
	let wrap = block.allocate(512, 1, false).unwrap();
	let Placement::Linear { offset, .. } = wrap else {
		unreachable!()
	};
	assert_eq!(offset, 0);
}
