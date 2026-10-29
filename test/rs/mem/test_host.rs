//! Host-buffer ownership/failure/alignment tests independent of placement code.
use super::*;
use crate::mem::test_host_allocator::FAIL_NEXT_HOST_ALLOCATION;
use std::sync::atomic::AtomicUsize;

use super::test_callbacks::Tracker;

#[test]
fn callbacks_own_aligned_growth_and_null_failure_preserves_initialized_values() {
	#[repr(align(256))]
	#[derive(Clone, Copy)]
	struct Aligned(u64);
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: synchronized fixture supplies exact layout/family ownership and outlives the buffer.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut values = HostVec::<Aligned>::new(host);
	values.try_reserve_exact(2).unwrap();
	assert_eq!(values.as_ptr().addr() % 256, 0);
	values.push_reserved(Aligned(11));
	values.push_reserved(Aligned(29));
	let before = tracker.snapshot();
	tracker.fail_at(before.attempts + 1);
	assert_eq!(values.try_reserve(1), Err(AllocError::HostAllocationFailed));
	assert_eq!((values.len(), values[0].0, values[1].0), (2, 11, 29));
	assert_eq!(tracker.snapshot().live, before.live);
	values.try_reserve(1).unwrap();
	assert_eq!(values.as_ptr().addr() % 256, 0);
	assert_eq!((values[0].0, values[1].0), (11, 29));
	drop(values);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn callback_storage_never_falls_back_to_the_rust_global_allocator() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: the fixture retains synchronized user data and exact allocation/free pairing.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut values = HostVec::new(host);
	FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
	let result = values.try_reserve_exact(12);
	let untouched = FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.replace(false));
	assert!(result.is_ok());
	assert!(untouched);
	values.push_reserved(4u64);
	drop(values);
	assert_eq!(tracker.snapshot().live, 0);
}

#[test]
fn overflow_and_zero_sized_elements_do_not_invoke_callbacks() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: exact, synchronized callback fixture retained until both buffers are gone.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut wide = HostVec::<u64>::new(host);
	assert_eq!(
		wide.try_reserve_exact(usize::MAX),
		Err(AllocError::HostAllocationFailed)
	);
	let mut zero = HostVec::new(host);
	zero.try_reserve_exact(10).unwrap();
	for _ in 0..10 {
		zero.push_reserved(());
	}
	zero.remove_prefix(3);
	zero.retain(|_| true);
	assert_eq!(zero.len(), 7);
	zero.clear();
	drop(zero);
	drop(wide);
	assert_eq!(
		(tracker.snapshot().attempts, tracker.snapshot().frees),
		(0, 0)
	);
}

#[test]
fn moved_elements_drop_once_and_buffer_is_released_during_element_unwind() {
	struct Counted<'a>(&'a AtomicUsize, bool);
	impl Drop for Counted<'_> {
		fn drop(&mut self) {
			self.0.fetch_add(1, Ordering::Relaxed);
			assert!(!self.1, "deliberate element destructor unwind");
		}
	}
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	let drops = AtomicUsize::new(0);
	let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
		// SAFETY: live synchronized callback fixture, including through unwind cleanup.
		let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
		let mut values = HostVec::new(host);
		values.try_reserve_exact(1).unwrap();
		values.push_reserved(Counted(&drops, true));
		values.try_reserve(2).unwrap();
		values.push_reserved(Counted(&drops, false));
		drop(values);
	}));
	assert!(result.is_err());
	assert_eq!(drops.load(Ordering::Relaxed), 2);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.unexpected), (0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn identity_counter_fails_closed_instead_of_reusing_an_owner() {
	let counter = AtomicUsize::new(usize::MAX - 1);
	assert_eq!(
		Identity::new_from_counter(&counter).unwrap().0,
		usize::MAX - 1
	);
	assert!(matches!(
		Identity::new_from_counter(&counter),
		Err(AllocError::SerialExhausted)
	));
	assert_eq!(counter.load(Ordering::Relaxed), usize::MAX);
}

#[test]
fn complete_custom_virtual_path_uses_no_rust_global_allocation() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	for algorithm in [
		super::super::AllocationAlgorithm::Tlsf,
		super::super::AllocationAlgorithm::Linear,
	] {
		FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
		let result = (|| -> Result<(), AllocError> {
			// SAFETY: synchronized exact-layout callbacks retained through this complete block lifetime.
			let mut block =
				unsafe { super::super::VirtualBlock::new_with_callbacks(4096, algorithm, &callbacks) }?;
			let mut region = block.allocate(17, 16, super::super::PlacementStrategy::Default)?;
			block.set_user_data(&region, 9)?;
			block.write_statistics_json(&mut std::io::sink(), true)?;
			block.free(&mut region)?;
			block.clear();
			Ok(())
		})();
		let untouched = FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.replace(false));
		assert!(result.is_ok());
		assert!(untouched, "{algorithm:?} used the Rust global allocator");
		assert_eq!(tracker.snapshot().live, 0);
	}
}

#[test]
fn zero_sized_destructors_drop_once_without_callback_storage() {
	static DROPS: AtomicUsize = AtomicUsize::new(0);
	struct Zero;
	impl Drop for Zero {
		fn drop(&mut self) {
			DROPS.fetch_add(1, Ordering::Relaxed);
		}
	}
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: valid synchronized callbacks remain borrowed until buffer Drop.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	{
		let mut values = HostVec::new(host);
		values.try_reserve_exact(5).unwrap();
		for _ in 0..5 {
			values.push_reserved(Zero);
		}
		values.clear();
		assert_eq!(DROPS.load(Ordering::Relaxed), 5);
		values.push_reserved(Zero);
	}
	assert_eq!(DROPS.load(Ordering::Relaxed), 6);
	assert_eq!(tracker.snapshot().attempts, 0);
}

#[test]
fn noncopy_retain_recovers_ownership_after_predicate_or_destructor_unwind() {
	for destructor_panics in [false, true] {
		struct Value<'a> {
			id: usize,
			drops: &'a [AtomicUsize; 6],
			panic: bool,
		}
		impl Drop for Value<'_> {
			fn drop(&mut self) {
				self.drops[self.id].fetch_add(1, Ordering::Relaxed);
				assert!(!self.panic, "deliberate retain destructor unwind");
			}
		}
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		let drops: [AtomicUsize; 6] = std::array::from_fn(|_| AtomicUsize::new(0));
		// SAFETY: exact synchronized fixture storage remains live through unwind recovery.
		let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
		let mut values = HostVec::new(host);
		values.try_reserve_exact(6).unwrap();
		for id in 0..6 {
			values.push_reserved(Value {
				id,
				drops: &drops,
				panic: destructor_panics && id == 2,
			});
		}
		let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
			values.retain(|value| {
				assert!(
					destructor_panics || value.id != 3,
					"deliberate retain predicate unwind"
				);
				!value.id.is_multiple_of(2)
			})
		}));
		assert!(result.is_err());
		assert_eq!(
			values.iter().map(|value| value.id).collect::<Vec<_>>(),
			[1, 3, 4, 5]
		);
		drop(values);
		for count in &drops {
			assert_eq!(count.load(Ordering::Relaxed), 1);
		}
		assert_eq!(tracker.snapshot().live, 0);
	}
}

#[test]
fn owned_utf8_names_and_noncopy_removal_balance_storage_and_values() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: exact synchronized callback family retained through all names/buffers.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut names = HostVec::new(host);
	names.try_reserve_exact(3).unwrap();
	for name in ["alpha", "λ雪", ""] {
		names.push_reserved(HostString::new_from_str(host, name).unwrap());
	}
	let removed = names.remove(1);
	assert_eq!(&*removed, "λ雪");
	assert_eq!((&*names[0], &*names[1]), ("alpha", ""));
	drop(removed);
	drop(names);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.unexpected), (0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn extraction_preserves_order_and_recovers_noncopy_unread_values_on_panic() {
	struct Value<'a> {
		value: usize,
		drops: &'a AtomicUsize,
	}
	impl Drop for Value<'_> {
		fn drop(&mut self) {
			self.drops.fetch_add(1, Ordering::SeqCst);
		}
	}
	for panic_at in [None, Some(4)] {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		// SAFETY: exact callback family remains live through all moves and unwind.
		let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
		let drops = AtomicUsize::new(0);
		let mut source = HostVec::new(host);
		let mut target = HostVec::new(host);
		source.try_reserve_exact(6).unwrap();
		target.try_reserve_exact(3).unwrap();
		for value in 0..6 {
			source.push_reserved(Value {
				value,
				drops: &drops,
			});
		}
		let attempts = tracker.snapshot().attempts;
		let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
			source.extract_if_reserved(&mut target, |value| {
				assert_ne!(
					Some(value.value),
					panic_at,
					"injected extraction predicate panic"
				);
				value.value % 2 == 0
			})
		}));
		assert_eq!(result.is_err(), panic_at.is_some());
		let remaining = source.iter().map(|value| value.value).collect::<Vec<_>>();
		let moved = target.iter().map(|value| value.value).collect::<Vec<_>>();
		assert_eq!(
			remaining,
			if panic_at.is_some() {
				vec![1, 3, 4, 5]
			} else {
				vec![1, 3, 5]
			}
		);
		assert_eq!(
			moved,
			if panic_at.is_some() {
				vec![0, 2]
			} else {
				vec![0, 2, 4]
			}
		);
		assert_eq!(tracker.snapshot().attempts, attempts);
		assert_eq!(drops.load(Ordering::SeqCst), 0);
		drop(source);
		drop(target);
		assert_eq!(drops.load(Ordering::SeqCst), 6);
		assert_eq!(tracker.snapshot().live, 0);
	}
}
