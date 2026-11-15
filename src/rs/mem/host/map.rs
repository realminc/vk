//! Callback-owned registry for checked allocator/pool serials.
//!
//! Rust adaptation: VMA uses intrusive allocation identities; vk additionally
//! validates non-owning generation tokens. Keys are internal monotonic u64
//! serials, never consumer-selected strings. Open addressing keeps lookup
//! independent of the number of previously freed serials; no tombstones remain.

use super::{AllocError, HostMemory, HostVec};
use std::{
	ops::Index,
	sync::atomic::{AtomicUsize, Ordering},
};

struct Entry<V> {
	key: u64,
	value: V,
}

pub(in crate::mem) struct HostMap<'callbacks, V> {
	buckets: HostVec<'callbacks, Option<Entry<V>>>,
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
	(product.checked_shr(64 - mask.count_ones()).unwrap_or(0) as usize) & mask
}

impl<'callbacks, V> HostMap<'callbacks, V> {
	pub fn new(host: HostMemory<'callbacks>) -> Self {
		Self {
			buckets: HostVec::new(host),
			length: 0,
			last_slot: AtomicUsize::new(usize::MAX),
		}
	}

	#[cfg(test)]
	pub fn capacity(&self) -> usize {
		self.buckets.len() - self.buckets.len() / 4
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
		let capacity = self.buckets.len();
		if required <= capacity - capacity / 4 {
			return Ok(());
		}
		self.grow(required)
	}

	#[cold]
	#[inline(never)]
	fn grow(&mut self, required: usize) -> Result<(), AllocError> {
		let mut capacity = self.buckets.len().max(8);
		while required > capacity - capacity / 4 {
			capacity = capacity
				.checked_mul(2)
				.ok_or(AllocError::HostAllocationFailed)?;
		}
		let mut buckets = HostVec::new(self.buckets.host_memory());
		buckets.try_reserve_exact(capacity)?;
		for _ in 0..capacity {
			buckets.push_reserved(None);
		}
		// All fallible work precedes movement. No hashing/user callback or
		// allocation occurs while entries move into the new registry.
		for bucket in &mut self.buckets {
			if let Some(entry) = bucket.take() {
				Self::place(&mut buckets, entry);
			}
		}
		self.buckets = buckets;
		Ok(())
	}

	// Merge only after the owner reserved the complete destination capacity.
	// No allocation, user code or fallible work occurs while values move.
	pub fn append_reserved(&mut self, other: &mut Self) {
		for bucket in &mut other.buckets {
			if let Some(entry) = bucket.take() {
				self.insert_reserved(entry.key, entry.value);
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
		while slot < self.buckets.len() {
			if let Some((key, target)) = self.buckets[slot]
				.as_ref()
				.and_then(|entry| selected(&entry.value).map(|target| (entry.key, target)))
			{
				let value = self.remove(&key).expect("selected registry entry");
				accept(target, key, value);
			} else {
				slot += 1;
			}
		}
	}

	fn place(buckets: &mut [Option<Entry<V>>], entry: Entry<V>) {
		let mask = buckets.len() - 1;
		let mut slot = index(entry.key, mask);
		while buckets[slot].is_some() {
			slot = (slot + 1) & mask;
		}
		buckets[slot] = Some(entry);
	}

	fn find(&self, key: u64) -> Option<usize> {
		let cached = self.last_slot.load(Ordering::Relaxed);
		if self
			.buckets
			.get(cached)
			.and_then(Option::as_ref)
			.is_some_and(|entry| entry.key == key)
		{
			return Some(cached);
		}
		if self.buckets.is_empty() {
			return None;
		}
		let mask = self.buckets.len() - 1;
		let mut slot = index(key, mask);
		loop {
			let entry = self.buckets[slot].as_ref()?;
			if entry.key == key {
				self.last_slot.store(slot, Ordering::Relaxed);
				return Some(slot);
			}
			slot = (slot + 1) & mask;
		}
	}

	pub fn get(&self, key: &u64) -> Option<&V> {
		self.find(*key).map(|slot| {
			&self.buckets[slot]
				.as_ref()
				.expect("occupied registry slot")
				.value
		})
	}

	#[inline]
	pub fn get_mut(&mut self, key: &u64) -> Option<&mut V> {
		self.find(*key).map(|slot| {
			&mut self.buckets[slot]
				.as_mut()
				.expect("occupied registry slot")
				.value
		})
	}

	// Fresh monotonic identities cannot replace an entry. Delay construction
	// until its vacant bucket is known, avoiding a large temporary value and
	// replacement/drop path during allocation publication. A panicking builder
	// leaves the registry unchanged; capacity was reserved by the owner.
	#[inline]
	pub fn insert_new_reserved_with(&mut self, key: u64, value: impl FnOnce() -> V) {
		let capacity = self.buckets.len();
		assert!(
			self.length < capacity - capacity / 4,
			"registry insertion requires reserved capacity"
		);
		let mask = capacity - 1;
		let mut slot = index(key, mask);
		while let Some(entry) = &self.buckets[slot] {
			assert!(entry.key != key, "registry insertion requires a fresh key");
			slot = (slot + 1) & mask;
		}
		self.buckets[slot] = Some(Entry {
			key,
			value: value(),
		});
		*self.last_slot.get_mut() = slot;
		self.length += 1;
	}

	/// Return the value for `key`, inserting `value()` with one probe when it is
	/// absent. Capacity for a new key must already be reserved.
	pub fn get_or_insert_reserved_with(&mut self, key: u64, value: impl FnOnce() -> V) -> &mut V {
		let capacity = self.buckets.len();
		let mask = capacity.wrapping_sub(1);
		let mut slot = index(key, mask);
		loop {
			match self
				.buckets
				.get(slot)
				.expect("registry insertion requires reserved capacity")
			{
				Some(entry) if entry.key == key => break,
				Some(_) => slot = (slot + 1) & mask,
				None => {
					assert!(
						self.length < capacity - capacity / 4,
						"registry insertion requires reserved capacity"
					);
					self.buckets[slot] = Some(Entry {
						key,
						value: value(),
					});
					self.length += 1;
					break;
				}
			}
		}
		*self.last_slot.get_mut() = slot;
		&mut self.buckets[slot]
			.as_mut()
			.expect("occupied registry slot")
			.value
	}

	pub fn insert_reserved(&mut self, key: u64, value: V) -> Option<V> {
		let capacity = self.buckets.len();
		let mask = capacity.wrapping_sub(1);
		let mut slot = index(key, mask);
		loop {
			let bucket = self
				.buckets
				.get_mut(slot)
				.expect("registry insertion requires reserved capacity");
			match bucket {
				Some(entry) if entry.key == key => {
					*self.last_slot.get_mut() = slot;
					return Some(std::mem::replace(&mut entry.value, value));
				}
				Some(_) => slot = (slot + 1) & mask,
				None => {
					assert!(
						self.length < capacity - capacity / 4,
						"registry insertion requires reserved capacity"
					);
					*bucket = Some(Entry { key, value });
					*self.last_slot.get_mut() = slot;
					self.length += 1;
					return None;
				}
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
		if !remove(
			&mut self.buckets[hole]
				.as_mut()
				.expect("occupied registry slot")
				.value,
		) {
			return None;
		}
		Some(self.remove_at(hole))
	}

	fn remove_at(&mut self, hole: usize) -> V {
		let entry = self.buckets[hole].take().expect("occupied registry slot");
		self.close_hole(hole);
		entry.value
	}

	/// Remove and drop an entry in its bucket, without moving the value out.
	/// Returns whether `key` was present.
	pub fn discard(&mut self, key: &u64) -> bool {
		let Some(hole) = self.find(*key) else {
			return false;
		};
		self.buckets[hole] = None;
		self.close_hole(hole);
		true
	}

	// Backward-shift deletion after the caller vacated `hole`: no tombstones.
	fn close_hole(&mut self, mut hole: usize) {
		self.length -= 1;
		let mask = self.buckets.len() - 1;
		let mut current = (hole + 1) & mask;
		while let Some(next) = self.buckets[current].as_ref() {
			let home = index(next.key, mask);
			if (current.wrapping_sub(home) & mask) > (hole.wrapping_sub(home) & mask) {
				self.buckets[hole] = self.buckets[current].take();
				hole = current;
			}
			current = (current + 1) & mask;
		}
	}

	pub fn iter(&self) -> Iter<'_, V> {
		Iter(self.buckets.iter())
	}

	pub fn values(&self) -> impl Iterator<Item = &V> {
		self.iter().map(|(_, value)| value)
	}
}

pub(in crate::mem) struct Iter<'borrow, V>(std::slice::Iter<'borrow, Option<Entry<V>>>);

impl<'borrow, V> Iterator for Iter<'borrow, V> {
	type Item = (&'borrow u64, &'borrow V);

	fn next(&mut self) -> Option<Self::Item> {
		self
			.0
			.find_map(|entry| entry.as_ref().map(|entry| (&entry.key, &entry.value)))
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
