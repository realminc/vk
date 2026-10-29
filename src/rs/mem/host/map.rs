//! Callback-owned registry for checked allocator/pool serials.
//!
//! Rust adaptation: VMA uses intrusive allocation identities; vk additionally
//! validates non-owning generation tokens. Keys are internal monotonic u64
//! serials, never consumer-selected strings. Open addressing keeps lookup
//! independent of the number of previously freed serials; no tombstones remain.

use super::{AllocError, HostMemory, HostVec};
use std::ops::Index;

struct Entry<V> {
	key: u64,
	value: V,
}

pub(in crate::mem) struct HostMap<'callbacks, V> {
	buckets: HostVec<'callbacks, Option<Entry<V>>>,
	length: usize,
}

// SplitMix64's finalizer distributes the allocator's sequential serials. This
// is private registry indexing, not a hash for adversarial public keys or crypto.
fn index(key: u64, mask: usize) -> usize {
	let mut mixed = key;
	mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
	mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
	((mixed ^ (mixed >> 31)) as usize) & mask
}

impl<'callbacks, V> HostMap<'callbacks, V> {
	pub fn new(host: HostMemory<'callbacks>) -> Self {
		Self {
			buckets: HostVec::new(host),
			length: 0,
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

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		let required = self
			.length
			.checked_add(additional)
			.ok_or(AllocError::HostAllocationFailed)?;
		let mut capacity = self.buckets.len();
		if required <= capacity - capacity / 4 {
			return Ok(());
		}
		capacity = capacity.max(8);
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
		if self.buckets.is_empty() {
			return None;
		}
		let mask = self.buckets.len() - 1;
		let mut slot = index(key, mask);
		loop {
			let entry = self.buckets[slot].as_ref()?;
			if entry.key == key {
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

	pub fn get_mut(&mut self, key: &u64) -> Option<&mut V> {
		self.find(*key).map(|slot| {
			&mut self.buckets[slot]
				.as_mut()
				.expect("occupied registry slot")
				.value
		})
	}

	pub fn insert_reserved(&mut self, key: u64, value: V) -> Option<V> {
		if let Some(slot) = self.find(key) {
			return Some(std::mem::replace(
				&mut self.buckets[slot]
					.as_mut()
					.expect("occupied registry slot")
					.value,
				value,
			));
		}
		assert!(
			self.length < self.buckets.len() - self.buckets.len() / 4,
			"registry insertion requires reserved capacity"
		);
		Self::place(&mut self.buckets, Entry { key, value });
		self.length += 1;
		None
	}

	pub fn remove(&mut self, key: &u64) -> Option<V> {
		let mut hole = self.find(*key)?;
		let entry = self.buckets[hole].take().expect("occupied registry slot");
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
		Some(entry.value)
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
