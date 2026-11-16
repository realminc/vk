//! Registry traces against BTreeMap and exact callback/failure ownership.
use super::*;
use std::{
	collections::BTreeMap,
	sync::atomic::{AtomicUsize, Ordering},
};

use crate::mem::host::test_callbacks::Tracker;

#[test]
fn seeded_insert_replace_lookup_and_shift_delete_match_an_independent_tree() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: exact synchronized fixture callbacks remain live through registry Drop.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut registry = HostMap::new(host);
	let mut oracle = BTreeMap::new();
	let mut random = 0xc312_8ca4_4d2e_0175u64;
	for step in 0..12000 {
		random ^= random << 13;
		random ^= random >> 7;
		random ^= random << 17;
		let key = random % 257;
		if random & 7 == 0 {
			let remove = oracle.get_mut(&key).is_some_and(|value| {
				*value += 1;
				*value % 3 == 0
			});
			let expected = if remove { oracle.remove(&key) } else { None };
			assert_eq!(
				registry.remove_if(&key, |value| {
					*value += 1;
					*value % 3 == 0
				}),
				expected
			);
		} else if random & 3 == 0 {
			if random & 32 != 0 {
				assert_eq!(registry.discard(&key), oracle.remove(&key).is_some());
			} else {
				assert_eq!(registry.remove(&key), oracle.remove(&key));
			}
		} else {
			registry.try_reserve(1).unwrap();
			if !oracle.contains_key(&key) && random & 8 != 0 {
				registry.insert_new_reserved_with(key, || step);
				assert_eq!(oracle.insert(key, step), None);
			} else if random & 16 != 0 {
				let value = registry.get_or_insert_reserved_with(key, || step);
				let expected = oracle.entry(key).or_insert(step);
				assert_eq!(value, expected);
				*value += 1;
				*expected += 1;
			} else {
				assert_eq!(
					registry.insert_reserved(key, step),
					oracle.insert(key, step)
				);
			}
		}
		assert_eq!(registry.len(), oracle.len());
		for (key, value) in &oracle {
			assert_eq!(registry.get(key), Some(value));
		}
		assert_eq!(registry.iter().count(), oracle.len());
	}
	drop(registry);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.unexpected), (0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn fresh_publication_preserves_noncopy_owners_on_collision_and_builder_unwind() {
	use std::panic::{AssertUnwindSafe, catch_unwind};
	struct Value<'a>(&'a AtomicUsize);
	impl Drop for Value<'_> {
		fn drop(&mut self) {
			self.0.fetch_add(1, Ordering::Relaxed);
		}
	}
	let drops = AtomicUsize::new(0);
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: exact fixture callbacks outlive the registry and all its entries.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut registry = HostMap::new(host);
	registry.try_reserve(6).unwrap();
	let keys: Vec<_> = (0..1000)
		.filter(|&key| index(key, 7) == 7)
		.take(6)
		.collect();
	let attempts = tracker.snapshot().attempts;
	registry.insert_new_reserved_with(keys[0], || Value(&drops));
	let mut duplicate_called = false;
	assert!(
		catch_unwind(AssertUnwindSafe(|| {
			registry.insert_new_reserved_with(keys[0], || {
				duplicate_called = true;
				Value(&drops)
			});
		}))
		.is_err()
	);
	assert!(!duplicate_called);
	assert!(
		catch_unwind(AssertUnwindSafe(|| {
			registry.insert_new_reserved_with(keys[1], || {
				let _temporary = Value(&drops);
				panic!("construction failed");
			});
		}))
		.is_err()
	);
	assert_eq!(registry.len(), 1);
	assert!(registry.get(&keys[0]).is_some());
	assert!(registry.get(&keys[1]).is_none());
	assert_eq!(drops.load(Ordering::Relaxed), 1);
	for &key in &keys[1..] {
		registry.insert_new_reserved_with(key, || Value(&drops));
	}
	assert_eq!(tracker.snapshot().attempts, attempts);
	drop(registry.remove(&keys[0]));
	for key in &keys[1..] {
		assert!(registry.get(key).is_some());
	}
	registry.try_reserve(1).unwrap();
	registry.insert_new_reserved_with(keys[0], || Value(&drops));
	drop(registry);
	assert_eq!(drops.load(Ordering::Relaxed), 8);
	assert_eq!(tracker.snapshot().live, 0);
}

#[test]
fn discard_drops_noncopy_entries_in_place_and_keeps_collisions_reachable() {
	struct Value<'a>(u64, &'a AtomicUsize);
	impl Drop for Value<'_> {
		fn drop(&mut self) {
			self.1.fetch_add(1, Ordering::Relaxed);
		}
	}
	let drops = AtomicUsize::new(0);
	let mut registry = HostMap::new(HostMemory::default());
	registry.try_reserve(6).unwrap();
	let keys: Vec<_> = (0..1000)
		.filter(|&key| index(key, 7) == 7)
		.take(6)
		.collect();
	assert_eq!(
		registry
			.get_or_insert_reserved_with(keys[0], || Value(1, &drops))
			.0,
		1
	);
	assert_eq!(
		registry
			.get_or_insert_reserved_with(keys[0], || Value(2, &drops))
			.0,
		1
	);
	for (value, &key) in (2..).zip(&keys[1..]) {
		registry.get_or_insert_reserved_with(key, || Value(value, &drops));
	}
	assert_eq!((registry.len(), drops.load(Ordering::Relaxed)), (6, 0));
	assert!(!registry.discard(&1001));
	for (count, &removed) in [keys[0], keys[3], keys[5]].iter().enumerate() {
		assert!(registry.discard(&removed));
		assert!(!registry.discard(&removed));
		assert_eq!(drops.load(Ordering::Relaxed), count + 1);
		for (value, key) in (1..).zip(&keys) {
			if ![keys[0], keys[3], keys[5]][..=count].contains(key) {
				assert_eq!(registry.get(key).map(|entry| entry.0), Some(value));
			}
		}
	}
	assert_eq!(registry.len(), 3);
	drop(registry);
	assert_eq!(drops.load(Ordering::Relaxed), 6);
}

#[test]
fn colliding_wraparound_clusters_keep_every_probe_reachable_after_deletion() {
	let mut registry = HostMap::new(HostMemory::default());
	registry.try_reserve(6).unwrap();
	let keys: Vec<_> = (0..1000)
		.filter(|&key| index(key, 7) == 7)
		.take(6)
		.collect();
	for &key in &keys {
		registry.insert_reserved(key, key + 1);
	}
	for &removed in &[keys[0], keys[3], keys[5]] {
		assert_eq!(registry.remove(&removed), Some(removed + 1));
		for &key in &keys {
			if ![keys[0], keys[3], keys[5]].contains(&key) {
				assert_eq!(registry.get(&key), Some(&(key + 1)));
			}
		}
	}
	for &key in &[keys[1], keys[2], keys[4]] {
		*registry.get_mut(&key).unwrap() += 7;
	}
	for &key in &[keys[1], keys[2], keys[4]] {
		assert_eq!(registry.remove(&key), Some(key + 8));
	}
	assert!(registry.is_empty());
}

#[test]
fn failed_rehash_preserves_noncopy_entries_and_successful_growth_moves_them_once() {
	struct Value<'a>(&'a AtomicUsize);
	impl Drop for Value<'_> {
		fn drop(&mut self) {
			self.0.fetch_add(1, Ordering::Relaxed);
		}
	}
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	let drops = AtomicUsize::new(0);
	// SAFETY: exact callback pairing retained through every failed/successful reservation.
	let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
	let mut registry = HostMap::new(host);
	registry.try_reserve(6).unwrap();
	for key in 1..=6 {
		registry.insert_reserved(key, Value(&drops));
	}
	let before = tracker.snapshot();
	tracker.fail_at(before.attempts + 1);
	assert_eq!(
		registry.try_reserve(1),
		Err(AllocError::HostAllocationFailed)
	);
	assert_eq!(registry.len(), 6);
	for key in 1..=6 {
		assert!(registry.get(&key).is_some());
	}
	assert_eq!(drops.load(Ordering::Relaxed), 0);
	assert_eq!(tracker.snapshot().live, before.live);
	registry.try_reserve(1).unwrap();
	assert_eq!(drops.load(Ordering::Relaxed), 0);
	assert_eq!(
		registry.try_reserve(usize::MAX),
		Err(AllocError::HostAllocationFailed)
	);
	drop(registry);
	assert_eq!(drops.load(Ordering::Relaxed), 6);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.unexpected), (0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn extracting_every_subset_of_a_wraparound_cluster_preserves_lookup_and_noncopy_owners() {
	struct Value<'a> {
		rank: usize,
		drops: &'a AtomicUsize,
	}
	impl Drop for Value<'_> {
		fn drop(&mut self) {
			self.drops.fetch_add(1, Ordering::Relaxed);
		}
	}
	for mask in 0..64_u32 {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		// SAFETY: synchronized callback state outlives both owner registries.
		let host = unsafe { HostMemory::new_from_callbacks(&callbacks) }.unwrap();
		let drops = AtomicUsize::new(0);
		let mut source = HostMap::new(host);
		let mut target = HostMap::new(host);
		source.try_reserve(6).unwrap();
		target.try_reserve(6).unwrap();
		let keys = (0..1000)
			.filter(|&key| index(key, 7) == 7)
			.take(6)
			.collect::<Vec<_>>();
		for (rank, &key) in keys.iter().enumerate() {
			source.insert_reserved(
				key,
				Value {
					rank,
					drops: &drops,
				},
			);
		}
		let attempts = tracker.snapshot().attempts;
		source.extract_if_reserved(&mut target, |value| mask & (1 << value.rank) != 0);
		assert_eq!(source.len() + target.len(), 6);
		assert_eq!(drops.load(Ordering::Relaxed), 0);
		assert_eq!(tracker.snapshot().attempts, attempts);
		for (rank, key) in keys.iter().enumerate() {
			let selected = mask & (1 << rank) != 0;
			assert_eq!(source.get(key).map(|v| v.rank), (!selected).then_some(rank));
			assert_eq!(target.get(key).map(|v| v.rank), selected.then_some(rank));
		}
		drop(source);
		drop(target);
		assert_eq!(drops.load(Ordering::Relaxed), 6);
		assert_eq!(tracker.snapshot().live, 0);
	}
}

#[test]
fn cached_slots_never_substitute_for_full_key_validation() {
	let mut registry = HostMap::new(HostMemory::default());
	registry.try_reserve(6).unwrap();
	for key in [0, 1, 7, 255, u64::MAX - 1, u64::MAX] {
		registry.insert_reserved(key, key ^ 0xfeed);
	}
	// Any cached slot may become stale after shift deletion, rehash or transfer.
	// Deliberately supply every slot, including out-of-range hints, and compare
	// the queried key independently of the hinted value.
	for hint in 0..=registry.keys.len() {
		for key in [0, 1, 7, 255, u64::MAX - 1, u64::MAX, 42] {
			registry.last_slot.store(hint, Ordering::Relaxed);
			let expected = (key != 42).then_some(key ^ 0xfeed);
			assert_eq!(registry.get(&key).copied(), expected);
		}
	}
	assert_eq!(registry.remove(&7), Some(7 ^ 0xfeed));
	registry.try_reserve(100).unwrap();
	assert_eq!(registry.get(&7), None);
	registry.insert_reserved(7, 1234);
	assert_eq!(registry.get(&7), Some(&1234));
}

#[test]
fn competing_shared_lookup_hints_preserve_exact_results() {
	let mut registry = HostMap::new(HostMemory::default());
	registry.try_reserve(128).unwrap();
	for key in 0..128 {
		registry.insert_reserved(key, key ^ 0xfeed);
	}
	std::thread::scope(|scope| {
		for worker in 0..4 {
			let registry = &registry;
			scope.spawn(move || {
				for step in 0..10000 {
					let key = (step * 17 + worker * 31) % 160;
					let expected = (key < 128).then_some(key ^ 0xfeed);
					assert_eq!(registry.get(&key).copied(), expected);
					assert_eq!(registry.get(&key).copied(), expected);
				}
			});
		}
	});
}
