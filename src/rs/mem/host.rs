//! Fallible host storage for VMA-owned metadata, not Vulkan device memory.
//!
//! Donor: pinned VMA 3.4.0 `VmaMalloc`, `VmaFree` and `VmaVector`.
//! See LICENSE-VMA-MIT. Adaptation: borrowed callbacks, checked Rust layouts,
//! initialized-element ownership and allocation failure before buffer mutation.

mod map;
pub(super) use map::HostMap;

use super::AllocError;
use crate as vk;
use std::{
	alloc::Layout,
	ops::{Deref, DerefMut},
	ptr::NonNull,
	sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Clone, Copy, Default)]
pub(super) struct HostMemory<'callbacks> {
	callbacks: Option<&'callbacks vk::AllocationCallbacks>,
}

impl<'callbacks> HostMemory<'callbacks> {
	/// # Safety
	/// Callbacks and user data must obey allocation/free alignment, ownership,
	/// non-unwinding and thread-safety contracts for every borrow of this owner.
	pub unsafe fn new_from_callbacks(
		callbacks: &'callbacks vk::AllocationCallbacks,
	) -> Result<Self, AllocError> {
		if callbacks.pfn_allocation.is_none() || callbacks.pfn_free.is_none() {
			return Err(AllocError::InvalidRequest(
				"host callbacks require allocation and free functions",
			));
		}
		Ok(Self {
			callbacks: Some(callbacks),
		})
	}

	unsafe fn allocate(self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		debug_assert_ne!(layout.size(), 0);
		let pointer = if let Some(callbacks) = self.callbacks {
			// SAFETY: constructor contract covers this borrowed callback and user
			// data; Layout supplies nonzero size and exact power-of-two alignment.
			unsafe {
				(callbacks
					.pfn_allocation
					.expect("validated host allocation callback"))(
					callbacks.p_user_data,
					layout.size(),
					layout.align(),
					vk::SystemAllocationScope::OBJECT,
				)
				.cast()
			}
		} else {
			// SAFETY: nonzero, checked layout; this buffer pairs it with dealloc.
			unsafe { std::alloc::alloc(layout) }
		};
		NonNull::new(pointer).ok_or(AllocError::HostAllocationFailed)
	}

	unsafe fn deallocate(self, pointer: NonNull<u8>, layout: Layout) {
		if let Some(callbacks) = self.callbacks {
			// SAFETY: this owner acquired the live buffer using this callback
			// family; all elements have been moved/dropped and the borrow remains live.
			unsafe {
				(callbacks.pfn_free.expect("validated host free callback"))(
					callbacks.p_user_data,
					pointer.as_ptr().cast(),
				)
			}
		} else {
			// SAFETY: exact original nonzero layout and global allocation family.
			unsafe { std::alloc::dealloc(pointer.as_ptr(), layout) }
		}
	}
}

// SAFETY: global allocation is thread-safe. Callback construction is unsafe and
// requires the borrowed callbacks/user data to support allocation/free on any
// thread and concurrent independent owners. Moving the owner preserves the borrow.
unsafe impl Send for HostMemory<'_> {}

// SAFETY: shared access only reads callback records; callback user-data mutations
// must satisfy the same thread-safety contract established at construction.
unsafe impl Sync for HostMemory<'_> {}

/// Private vector with explicit fallible reservation and reserved-only writes.
/// No method silently falls back from callbacks to the global Rust allocator.
/// This owner deliberately exposes only operations needed by placement metadata.
pub(super) struct HostVec<'callbacks, T> {
	pointer: NonNull<T>,
	length: usize,
	capacity: usize,
	host: HostMemory<'callbacks>,
}

impl<'callbacks, T> HostVec<'callbacks, T> {
	pub fn new(host: HostMemory<'callbacks>) -> Self {
		Self {
			pointer: NonNull::dangling(),
			length: 0,
			capacity: if size_of::<T>() == 0 { usize::MAX } else { 0 },
			host,
		}
	}

	#[cfg(test)]
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn host_memory(&self) -> HostMemory<'callbacks> {
		self.host
	}

	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), AllocError> {
		self.reserve(additional, false)
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		self.reserve(additional, true)
	}

	fn reserve(&mut self, additional: usize, amortized: bool) -> Result<(), AllocError> {
		let required = self
			.length
			.checked_add(additional)
			.ok_or(AllocError::HostAllocationFailed)?;
		if required <= self.capacity {
			return Ok(());
		}
		let capacity = if amortized {
			required.max(self.capacity.saturating_mul(2)).max(4)
		} else {
			required
		};
		let layout = Layout::array::<T>(capacity).map_err(|_| AllocError::HostAllocationFailed)?;
		// SAFETY: Layout validated size/alignment and is nonzero for growth of a
		// non-ZST buffer. Failure retains the complete old buffer and all elements.
		let pointer = unsafe { self.host.allocate(layout) }?.cast::<T>();
		// SAFETY: distinct aligned buffers, initialized prefix of length elements,
		// and sufficient destination capacity. Copy transfers ownership once.
		unsafe { std::ptr::copy_nonoverlapping(self.pointer.as_ptr(), pointer.as_ptr(), self.length) };
		let old = self.storage();
		self.pointer = pointer;
		self.capacity = capacity;
		drop(old);
		Ok(())
	}

	pub fn push_reserved(&mut self, value: T) {
		assert!(
			self.length < self.capacity,
			"host metadata write requires reserved capacity"
		);
		// SAFETY: the next aligned element is uninitialized and inside capacity;
		// ZST dangling pointers are aligned and need no backing allocation.
		unsafe { self.pointer.as_ptr().add(self.length).write(value) };
		self.length += 1;
	}

	pub fn resize_reserved(&mut self, length: usize, value: T)
	where
		T: Clone,
	{
		assert!(
			length <= self.capacity,
			"host metadata resize requires reserved capacity"
		);
		while self.length > length {
			self.pop();
		}
		while self.length < length {
			self.push_reserved(value.clone());
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.length == 0 {
			return None;
		}
		self.length -= 1;
		// SAFETY: the former final initialized element is moved out exactly once.
		Some(unsafe { self.pointer.as_ptr().add(self.length).read() })
	}

	pub fn clear(&mut self) {
		let length = std::mem::replace(&mut self.length, 0);
		// SAFETY: the original initialized prefix is valid and exclusively owned.
		// Length was cleared before invoking element destructors, including unwind.
		unsafe {
			std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
				self.pointer.as_ptr(),
				length,
			))
		};
	}

	pub fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
		let original = std::mem::replace(&mut self.length, 0);
		let mut guard = Retain {
			vector: self,
			read: 0,
			written: 0,
			original,
		};
		while guard.read < original {
			// SAFETY: read is inside the original initialized prefix; this
			// exclusive owner keeps every not-yet-processed value alive.
			let current = unsafe { guard.vector.pointer.as_ptr().add(guard.read) };
			// SAFETY: current names the initialized current element, borrowed
			// only for this predicate call before any move or destruction.
			if keep(unsafe { &*current }) {
				if guard.written != guard.read {
					// SAFETY: these distinct slots transfer one initialized value
					// into an earlier uninitialized hole; no element is duplicated.
					unsafe {
						std::ptr::copy_nonoverlapping(
							current,
							guard.vector.pointer.as_ptr().add(guard.written),
							1,
						)
					};
				}
				guard.written += 1;
				guard.read += 1;
			} else {
				guard.read += 1;
				// SAFETY: exclude this element from unwind recovery before
				// invoking its destructor; the unread suffix remains owned.
				unsafe { std::ptr::drop_in_place(current) };
			}
		}
	}

	#[cfg(test)]
	pub fn extract_if_reserved(&mut self, target: &mut Self, mut selected: impl FnMut(&T) -> bool) {
		self.extract_to_reserved(
			|value| selected(value).then_some(0),
			|_, value| target.push_reserved(value),
		);
	}

	/// Move selected initialized values in order; unread values recover on unwind.
	pub fn extract_to_reserved(
		&mut self,
		mut selected: impl FnMut(&T) -> Option<usize>,
		mut accept: impl FnMut(usize, T),
	) {
		let original = std::mem::replace(&mut self.length, 0);
		let mut guard = Retain {
			vector: self,
			read: 0,
			written: 0,
			original,
		};
		while guard.read < original {
			// SAFETY: current is in the initialized unread suffix. The retain guard
			// restores that suffix if the predicate unwinds before movement.
			let current = unsafe { guard.vector.pointer.as_ptr().add(guard.read) };
			// SAFETY: temporary read-only borrow ends before this exclusive move.
			if let Some(target) = selected(unsafe { &*current }) {
				// SAFETY: move this initialized value once into the reserved target.
				let value = unsafe { current.read() };
				guard.read += 1;
				accept(target, value);
			} else {
				if guard.written != guard.read {
					// SAFETY: distinct slots transfer ownership to an earlier hole.
					unsafe {
						std::ptr::copy_nonoverlapping(
							current,
							guard.vector.pointer.as_ptr().add(guard.written),
							1,
						)
					};
				}
				guard.written += 1;
				guard.read += 1;
			}
		}
	}

	pub fn remove(&mut self, index: usize) -> T {
		assert!(
			index < self.length,
			"host metadata removal exceeds initialized length"
		);
		self.length -= 1;
		// SAFETY: the removed value is moved out once, then its initialized
		// suffix shifts left. The former final slot is outside the new length.
		unsafe {
			let pointer = self.pointer.as_ptr().add(index);
			let value = pointer.read();
			std::ptr::copy(pointer.add(1), pointer, self.length - index);
			value
		}
	}

	/// Transfer all initialized elements without allocating, preserving order.
	pub fn append_reserved(&mut self, other: &mut Self) {
		let length = self
			.length
			.checked_add(other.length)
			.expect("reserved metadata length");
		assert!(
			length <= self.capacity,
			"metadata append requires reserved capacity"
		);
		// SAFETY: exclusively borrowed distinct owners, valid initialized source,
		// and sufficient aligned destination capacity. Transfer each value once.
		unsafe {
			std::ptr::copy_nonoverlapping(
				other.pointer.as_ptr(),
				self.pointer.as_ptr().add(self.length),
				other.length,
			)
		};
		self.length = length;
		other.length = 0;
	}

	pub fn extend_from_slice_reserved(&mut self, values: &[T])
	where
		T: Copy,
	{
		let length = self
			.length
			.checked_add(values.len())
			.expect("reserved metadata length");
		assert!(
			length <= self.capacity,
			"host metadata extension requires reserved capacity"
		);
		// SAFETY: the source shared slice cannot alias this exclusively borrowed
		// destination; Copy elements initialize only the reserved suffix.
		unsafe {
			std::ptr::copy_nonoverlapping(
				values.as_ptr(),
				self.pointer.as_ptr().add(self.length),
				values.len(),
			)
		};
		self.length = length;
	}

	pub fn remove_prefix(&mut self, count: usize)
	where
		T: Copy,
	{
		assert!(
			count <= self.length,
			"host metadata prefix exceeds initialized length"
		);
		// SAFETY: the retained initialized suffix is within this allocation;
		// regions may overlap and Copy elements have no destruction obligations.
		unsafe {
			std::ptr::copy(
				self.pointer.as_ptr().add(count),
				self.pointer.as_ptr(),
				self.length - count,
			)
		};
		self.length -= count;
	}

	fn storage(&self) -> Option<Storage<'callbacks>> {
		if self.capacity == 0 || size_of::<T>() == 0 {
			return None;
		}
		Some(Storage {
			pointer: self.pointer.cast(),
			layout: Layout::array::<T>(self.capacity).expect("validated host capacity layout"),
			host: self.host,
		})
	}
}

// Restore the unread suffix if either predicate or element destruction unwinds.
struct Retain<'vector, 'callbacks, T> {
	vector: &'vector mut HostVec<'callbacks, T>,
	read: usize,
	written: usize,
	original: usize,
}

impl<T> Drop for Retain<'_, '_, T> {
	fn drop(&mut self) {
		let remaining = self.original - self.read;
		// SAFETY: move the entire initialized unread suffix into its earlier
		// hole, permitting overlap, then publish exactly the initialized prefix.
		unsafe {
			std::ptr::copy(
				self.vector.pointer.as_ptr().add(self.read),
				self.vector.pointer.as_ptr().add(self.written),
				remaining,
			)
		};
		self.vector.length = self.written + remaining;
	}
}

pub(super) struct HostString<'callbacks>(HostVec<'callbacks, u8>);

impl<'callbacks> HostString<'callbacks> {
	pub fn new_from_str(host: HostMemory<'callbacks>, text: &str) -> Result<Self, AllocError> {
		let mut bytes = HostVec::new(host);
		bytes.try_reserve_exact(text.len())?;
		bytes.extend_from_slice_reserved(text.as_bytes());
		Ok(Self(bytes))
	}
}

impl Deref for HostString<'_> {
	type Target = str;

	fn deref(&self) -> &str {
		// SAFETY: the only constructor copies valid UTF-8 and no mutable byte
		// access escapes this owner. Moving/growing preserves every byte.
		unsafe { std::str::from_utf8_unchecked(&self.0) }
	}
}

// Keep deallocation paired even if an element's destructor panics during Drop.
struct Storage<'callbacks> {
	pointer: NonNull<u8>,
	layout: Layout,
	host: HostMemory<'callbacks>,
}

impl Drop for Storage<'_> {
	fn drop(&mut self) {
		// SAFETY: the vector detached this exact buffer after moving/dropping its
		// initialized elements; its callback borrow is still retained by this guard.
		unsafe { self.host.deallocate(self.pointer, self.layout) };
	}
}

impl<T> Drop for HostVec<'_, T> {
	fn drop(&mut self) {
		let _storage = self.storage();
		self.clear();
	}
}

impl<T> Deref for HostVec<'_, T> {
	type Target = [T];

	fn deref(&self) -> &[T] {
		// SAFETY: aligned non-null pointer and initialized prefix; capacity was
		// checked by Layout and this shared borrow excludes mutation/destruction.
		unsafe { std::slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
	}
}

impl<T> DerefMut for HostVec<'_, T> {
	fn deref_mut(&mut self) -> &mut [T] {
		// SAFETY: the same initialized-prefix invariant with an exclusive borrow.
		unsafe { std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length) }
	}
}

impl<'borrow, T> IntoIterator for &'borrow HostVec<'_, T> {
	type Item = &'borrow T;
	type IntoIter = std::slice::Iter<'borrow, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'borrow, T> IntoIterator for &'borrow mut HostVec<'_, T> {
	type Item = &'borrow mut T;
	type IntoIter = std::slice::IterMut<'borrow, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

// SAFETY: this vector exclusively owns its initialized T values and raw buffer;
// moving them requires T: Send. HostMemory's constructor establishes callback
// thread safety, and the lifetime retains its callback record through Drop.
unsafe impl<T: Send> Send for HostVec<'_, T> {}

// SAFETY: shared access exposes only shared T references and immutable metadata;
// T: Sync and HostMemory's callback contract cover the reachable shared state.
unsafe impl<T: Sync> Sync for HostVec<'_, T> {}

/// Non-owning checked identity; tokens retain no heap reference after block Drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Identity(usize);

static NEXT_IDENTITY: AtomicUsize = AtomicUsize::new(1);

impl Identity {
	pub fn new() -> Result<Self, AllocError> {
		Self::new_from_counter(&NEXT_IDENTITY)
	}

	fn new_from_counter(counter: &AtomicUsize) -> Result<Self, AllocError> {
		counter
			.try_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
				value.checked_add(1)
			})
			.map(Self)
			.map_err(|_| AllocError::SerialExhausted)
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/host/test_callbacks.rs"]
pub(super) mod test_callbacks;

#[cfg(test)]
#[path = "../../../test/rs/mem/test_host.rs"]
mod tests;
