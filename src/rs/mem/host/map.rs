//! Callback-owned registry for checked allocator/pool serials.
//!
//! Rust adaptation: VMA uses intrusive allocation identities; vk additionally
//! validates non-owning generation tokens. Keys are internal monotonic u64
//! serials and non-null native handles, never consumer-selected strings. Open
//! addressing keeps lookup independent of the number of previously freed
//! serials; no tombstones remain. Keys and values sit in parallel arrays so
//! each probe step compares one sixteen-byte slot instead of an entry bucket.

use super::{AllocError, HostMemory, HostVec};
use std::{
	ops::Index,
	sync::atomic::{AtomicUsize, Ordering},
};

pub(in crate::mem) struct HostMap<'callbacks, V> {
	// `None` marks an empty slot, so every possible u64 key remains valid.
	keys: HostVec<'callbacks, Option<u64>>,
	values: HostVec<'callbacks, Option<V>>,
	length: usize,
	// A hint only: every read checks both the slot bounds and complete key.
	// Relaxed atomics preserve shared-read Sync; competing readers may replace
	// the hint, but cannot grant access to a different entry or stale value.
	last_slot: AtomicUsize,
}

// Fibonacci hashing: one odd-constant multiply, keeping the top log2(capacity)
// product bits. Those bits depend on every key bit, which spreads sequential
// serials and aligned native handles across the table. This is private
// registry indexing, not a hash for adversarial public keys or crypto.
#[inline]
fn index(key: u64, mask: usize) -> usize {
	let product = key.wrapping_mul(0x9e37_79b9_7f4a_7c15);
	// Masks are 2^n - 1, so the width comes from the leading-zero count: baseline
	// x86-64 has BSR but no POPCNT, where `count_ones` becomes a bit-count loop.
	let bits = usize::BITS - mask.leading_zeros();
	(product.checked_shr(64 - bits).unwrap_or(0) as usize) & mask
}

impl<'callbacks, V> HostMap<'callbacks, V> {
	pub fn new(host: HostMemory<'callbacks>) -> Self {
		Self {
			keys: HostVec::new(host),
			values: HostVec::new(host),
			length: 0,
			last_slot: AtomicUsize::new(usize::MAX),
		}
	}

	#[cfg(test)]
	pub fn capacity(&self) -> usize {
		self.keys.len() - self.keys.len() / 4
	}

	pub fn len(&self) -> usize {
		self.length
	}

	pub fn is_empty(&self) -> bool {
		self.length == 0
	}

	#[inline]
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		let required = self
			.length
			.checked_add(additional)
			.ok_or(AllocError::HostAllocationFailed)?;
		let capacity = self.keys.len();
		if required <= capacity - capacity / 4 {
			return Ok(());
		}
		self.grow(required)
	}

	#[cold]
	#[inline(never)]
	fn grow(&mut self, required: usize) -> Result<(), AllocError> {
		let mut capacity = self.keys.len().max(8);
		while required > capacity - capacity / 4 {
			capacity = capacity
				.checked_mul(2)
				.ok_or(AllocError::HostAllocationFailed)?;
		}
		let mut keys = HostVec::new(self.keys.host_memory());
		let mut values = HostVec::new(self.values.host_memory());
		keys.try_reserve_exact(capacity)?;
		values.try_reserve_exact(capacity)?;
		for _ in 0..capacity {
			keys.push_reserved(None);
			values.push_reserved(None);
		}
		// All fallible work precedes movement. No hashing/user callback or
		// allocation occurs while entries move into the new registry.
		let mask = capacity - 1;
		for (key, value) in self.keys.iter_mut().zip(self.values.iter_mut()) {
			let (Some(key), Some(value)) = (*key, value.take()) else {
				continue;
			};
			let mut slot = index(key, mask);
			while keys[slot].is_some() {
				slot = (slot + 1) & mask;
			}
			keys[slot] = Some(key);
			values[slot] = Some(value);
		}
		self.keys = keys;
		self.values = values;
		Ok(())
	}

	// Merge only after the owner reserved the complete destination capacity.
	// No allocation, user code or fallible work occurs while values move.
	pub fn append_reserved(&mut self, other: &mut Self) {
		for (key, value) in other.keys.iter_mut().zip(other.values.iter_mut()) {
			if let (Some(key), Some(value)) = (*key, value.take()) {
				self.insert_reserved(key, value);
			}
		}
		other.length = 0;
	}

	#[cfg(test)]
	pub fn extract_if_reserved(&mut self, target: &mut Self, mut selected: impl FnMut(&V) -> bool) {
		self.extract_to_reserved(
			|value| selected(value).then_some(0),
			|_, key, value| {
				target.insert_reserved(key, value);
			},
		);
	}

	pub fn extract_to_reserved(
		&mut self,
		mut selected: impl FnMut(&V) -> Option<usize>,
		mut accept: impl FnMut(usize, u64, V),
	) {
		let mut slot = 0;
		while slot < self.keys.len() {
			if self.keys[slot].is_some()
				&& let Some(target) = selected(self.values[slot].as_ref().expect("occupied registry slot"))
			{
				let key = self.keys[slot].expect("occupied registry slot");
				let value = self.remove(&key).expect("selected registry entry");
				accept(target, key, value);
			} else {
				slot += 1;
			}
		}
	}

	fn find(&self, key: u64) -> Option<usize> {
		let cached = self.last_slot.load(Ordering::Relaxed);
		if self.keys.get(cached) == Some(&Some(key)) {
			return Some(cached);
		}
		if self.keys.is_empty() {
			return None;
		}
		let mask = self.keys.len() - 1;
		let mut slot = index(key, mask);
		loop {
			let entry = self.keys[slot]?;
			if entry == key {
				self.last_slot.store(slot, Ordering::Relaxed);
				return Some(slot);
			}
			slot = (slot + 1) & mask;
		}
	}

	pub fn get(&self, key: &u64) -> Option<&V> {
		self
			.find(*key)
			.map(|slot| self.values[slot].as_ref().expect("occupied registry slot"))
	}

	#[inline]
	pub fn get_mut(&mut self, key: &u64) -> Option<&mut V> {
		self
			.find(*key)
			.map(|slot| self.values[slot].as_mut().expect("occupied registry slot"))
	}

	#[inline]
	pub fn insert_new_reserved_with(&mut self, key: u64, value: impl FnOnce() -> V) {
		let capacity = self.keys.len();
		assert!(
			self.length < capacity - capacity / 4,
			"registry insertion requires reserved capacity"
		);
		let mask = capacity - 1;
		let mut slot = index(key, mask);
		while let Some(entry) = self.keys[slot] {
			assert!(entry != key, "registry insertion requires a fresh key");
			slot = (slot + 1) & mask;
		}
		// A panicking builder leaves keys, values and length unchanged.
		self.values[slot] = Some(value());
		self.keys[slot] = Some(key);
		*self.last_slot.get_mut() = slot;
		self.length += 1;
	}

	/// Return the value for `key`, inserting `value()` with one probe when it is
	/// absent. Capacity for a new key must already be reserved.
	pub fn get_or_insert_reserved_with(&mut self, key: u64, value: impl FnOnce() -> V) -> &mut V {
		let capacity = self.keys.len();
		let mask = capacity.wrapping_sub(1);
		let mut slot = index(key, mask);
		loop {
			match *self
				.keys
				.get(slot)
				.expect("registry insertion requires reserved capacity")
			{
				Some(entry) if entry == key => break,
				None => {
					assert!(
						self.length < capacity - capacity / 4,
						"registry insertion requires reserved capacity"
					);
					self.values[slot] = Some(value());
					self.keys[slot] = Some(key);
					self.length += 1;
					break;
				}
				Some(_) => slot = (slot + 1) & mask,
			}
		}
		*self.last_slot.get_mut() = slot;
		self.values[slot].as_mut().expect("occupied registry slot")
	}

	/// Insert `value` only when `key` is absent, with one probe. Returns whether
	/// it was inserted; an existing entry is left unchanged. Capacity for a new
	/// key must already be reserved.
	pub fn insert_absent_reserved(&mut self, key: u64, value: V) -> bool {
		let capacity = self.keys.len();
		let mask = capacity.wrapping_sub(1);
		let mut slot = index(key, mask);
		loop {
			match *self
				.keys
				.get(slot)
				.expect("registry insertion requires reserved capacity")
			{
				Some(entry) if entry == key => return false,
				Some(_) => slot = (slot + 1) & mask,
				None => {
					assert!(
						self.length < capacity - capacity / 4,
						"registry insertion requires reserved capacity"
					);
					self.values[slot] = Some(value);
					self.keys[slot] = Some(key);
					*self.last_slot.get_mut() = slot;
					self.length += 1;
					return true;
				}
			}
		}
	}

	pub fn insert_reserved(&mut self, key: u64, value: V) -> Option<V> {
		let capacity = self.keys.len();
		let mask = capacity.wrapping_sub(1);
		let mut slot = index(key, mask);
		loop {
			let slot_ref = self
				.keys
				.get_mut(slot)
				.expect("registry insertion requires reserved capacity");
			match *slot_ref {
				Some(entry) if entry == key => {
					*self.last_slot.get_mut() = slot;
					return Some(std::mem::replace(
						self.values[slot].as_mut().expect("occupied registry slot"),
						value,
					));
				}
				None => {
					assert!(
						self.length < capacity - capacity / 4,
						"registry insertion requires reserved capacity"
					);
					self.values[slot] = Some(value);
					*slot_ref = Some(key);
					*self.last_slot.get_mut() = slot;
					self.length += 1;
					return None;
				}
				Some(_) => slot = (slot + 1) & mask,
			}
		}
	}

	pub fn remove(&mut self, key: &u64) -> Option<V> {
		let hole = self.find(*key)?;
		Some(self.remove_at(hole))
	}

	// Association retirement changes one plane and conditionally drops the key.
	// Retain its already-found slot rather than hashing/probing it a second time.
	pub fn remove_if(&mut self, key: &u64, remove: impl FnOnce(&mut V) -> bool) -> Option<V> {
		let hole = self.find(*key)?;
		if !remove(self.values[hole].as_mut().expect("occupied registry slot")) {
			return None;
		}
		Some(self.remove_at(hole))
	}

	fn remove_at(&mut self, hole: usize) -> V {
		self.keys[hole] = None;
		let value = self.values[hole].take().expect("occupied registry slot");
		self.close_hole(hole);
		value
	}

	/// Remove and drop an entry in its bucket, without moving the value out.
	/// Returns whether `key` was present.
	pub fn discard(&mut self, key: &u64) -> bool {
		let Some(hole) = self.find(*key) else {
			return false;
		};
		self.keys[hole] = None;
		self.values[hole] = None;
		self.close_hole(hole);
		true
	}

	// Backward-shift deletion after the caller vacated `hole`: no tombstones.
	fn close_hole(&mut self, mut hole: usize) {
		self.length -= 1;
		let mask = self.keys.len() - 1;
		let mut current = (hole + 1) & mask;
		while let Some(entry) = self.keys[current] {
			let home = index(entry, mask);
			if (current.wrapping_sub(home) & mask) > (hole.wrapping_sub(home) & mask) {
				self.keys[hole] = self.keys[current].take();
				self.values[hole] = self.values[current].take();
				hole = current;
			}
			current = (current + 1) & mask;
		}
	}

	pub fn iter(&self) -> Iter<'_, V> {
		Iter {
			keys: self.keys.iter(),
			values: self.values.iter(),
		}
	}

	pub fn values(&self) -> impl Iterator<Item = &V> {
		self.iter().map(|(_, value)| value)
	}
}

pub(in crate::mem) struct Iter<'borrow, V> {
	keys: std::slice::Iter<'borrow, Option<u64>>,
	values: std::slice::Iter<'borrow, Option<V>>,
}

impl<'borrow, V> Iterator for Iter<'borrow, V> {
	type Item = (&'borrow u64, &'borrow V);

	fn next(&mut self) -> Option<Self::Item> {
		for key in self.keys.by_ref() {
			let value = self.values.next().expect("registry key/value parity");
			if let Some(key) = key {
				return Some((key, value.as_ref().expect("occupied registry slot")));
			}
		}
		None
	}
}

impl<'borrow, V> IntoIterator for &'borrow HostMap<'_, V> {
	type Item = (&'borrow u64, &'borrow V);
	type IntoIter = Iter<'borrow, V>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<V> Index<&u64> for HostMap<'_, V> {
	type Output = V;

	fn index(&self, key: &u64) -> &V {
		self.get(key).expect("validated allocator registry key")
	}
}

#[cfg(test)]
#[path = "../../../../test/rs/mem/host/test_map.rs"]
mod tests;
