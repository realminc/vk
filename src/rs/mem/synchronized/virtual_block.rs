//! Internally synchronized host placement over an integer byte range.
//!
//! Donor: VMA 3.4.0's externally synchronized `VmaVirtualBlock` placement.
//! Rust adaptation: an optional, explicitly owned mutex around the canonical
//! `VirtualBlock`, safe `&self` operation access, poison refusal and consuming
//! recovery. Tokens remain explicit and freely movable between threads;
//! dropping one never releases its region. No Vulkan memory or GPU work exists.
use super::super::virtual_block::{
	VirtualAllocation, VirtualAllocationInfo, VirtualBlock, VirtualBlockDetails, VirtualBlockStats,
};
use super::super::{AllocError, AllocationAlgorithm, PlacementStrategy};
use crate as vk;
use std::sync::{Mutex, PoisonError};

/// One virtual block with serialized operation access from any host thread.
///
/// Default-constructed blocks allocate metadata with the system allocator and
/// are safe to share immediately. Callback-backed blocks are adopted through
/// unsafe constructors under the same any-thread contract as `VirtualBlock`'s
/// unsafe callback construction. Access serializes per block; independent
/// blocks run concurrently. There is no GPU or native completion tracking.
/// Caller-provided diagnostic writers must not reenter this block.
pub struct SynchronizedVirtualBlock<'callbacks> {
	inner: Mutex<Transferable<'callbacks>>,
}

struct Transferable<'callbacks>(VirtualBlock<'callbacks>);

// SAFETY: this wrapper is created only by the synchronized constructors. The
// default constructor builds system-allocator metadata with no borrowed
// record; unsafe constructors transfer VirtualBlock's established any-thread
// callback/user-data contract, which also covers final CPU cleanup on the
// releasing thread. The metadata buffers own their initialized storage and
// retain no Rust references between operations. Moving changes no identity:
// tokens validate against the block's generation identity, not its address.
unsafe impl Send for Transferable<'_> {}

impl<'callbacks> SynchronizedVirtualBlock<'callbacks> {
	fn locked<R>(
		&self,
		operation: impl FnOnce(&mut VirtualBlock<'callbacks>) -> Result<R, AllocError>,
	) -> Result<R, AllocError> {
		let mut guard = self
			.inner
			.lock()
			.map_err(|_| AllocError::SynchronizationPoisoned)?;
		operation(&mut guard.0)
	}

	/// Create TLSF host-only placement behind one mutex. No callbacks are borrowed.
	/// # Errors
	/// Rejects a zero size or failed host metadata reservation.
	pub fn new(size: u64) -> Result<Self, AllocError> {
		Self::new_with_algorithm(size, AllocationAlgorithm::Tlsf)
	}

	/// Create TLSF or linear host-only placement behind one mutex.
	/// # Errors
	/// Rejects a zero size or failed host metadata reservation.
	pub fn new_with_algorithm(size: u64, algorithm: AllocationAlgorithm) -> Result<Self, AllocError> {
		Ok(Self {
			inner: Mutex::new(Transferable(VirtualBlock::new_with_algorithm(
				size, algorithm,
			)?)),
		})
	}

	/// Create callback-backed placement behind one mutex.
	/// # Errors
	/// Same errors as `VirtualBlock::new_with_callbacks`.
	/// # Safety
	/// The complete `VirtualBlock::new_with_callbacks` contract applies. Callback
	/// code and user data additionally must support calls on any participating
	/// worker thread, not only the constructing thread, through final cleanup.
	pub unsafe fn new_with_callbacks(
		size: u64,
		algorithm: AllocationAlgorithm,
		callbacks: &'callbacks vk::AllocationCallbacks,
	) -> Result<Self, AllocError> {
		// SAFETY: caller establishes the callback lifetime/thread contract.
		let block = unsafe { VirtualBlock::new_with_callbacks(size, algorithm, callbacks)? };
		Ok(Self {
			inner: Mutex::new(Transferable(block)),
		})
	}

	/// Adopt an existing block for serialized access from any host thread.
	/// # Safety
	/// If the block borrows allocation callbacks, their records/code/user data
	/// must satisfy `VirtualBlock::new_with_callbacks`'s contract and support
	/// calls on any participating worker thread through final cleanup.
	pub unsafe fn new_from_block(block: VirtualBlock<'callbacks>) -> Self {
		Self {
			inner: Mutex::new(Transferable(block)),
		}
	}

	/// Run `VirtualBlock::allocate` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::allocate`, plus SynchronizationPoisoned.
	pub fn allocate(
		&self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
	) -> Result<VirtualAllocation, AllocError> {
		self.locked(|block| block.allocate(size, alignment, strategy))
	}

	/// Run `VirtualBlock::allocate_upper` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::allocate_upper`, plus SynchronizationPoisoned.
	pub fn allocate_upper(&self, size: u64, alignment: u64) -> Result<VirtualAllocation, AllocError> {
		self.locked(|block| block.allocate_upper(size, alignment))
	}

	/// Run `VirtualBlock::free` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::free`, plus SynchronizationPoisoned.
	pub fn free(&self, allocation: &mut VirtualAllocation) -> Result<(), AllocError> {
		self.locked(|block| block.free(allocation))
	}

	/// Run `VirtualBlock::allocation_info` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::allocation_info`, plus SynchronizationPoisoned.
	pub fn allocation_info(
		&self,
		allocation: &VirtualAllocation,
	) -> Result<VirtualAllocationInfo, AllocError> {
		self.locked(|block| block.allocation_info(allocation))
	}

	/// Run `VirtualBlock::set_user_data` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::set_user_data`, plus SynchronizationPoisoned.
	pub fn set_user_data(
		&self,
		allocation: &VirtualAllocation,
		data: usize,
	) -> Result<(), AllocError> {
		self.locked(|block| block.set_user_data(allocation, data))
	}

	/// Run `VirtualBlock::is_empty` under this block's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn is_empty(&self) -> Result<bool, AllocError> {
		self.locked(|block| Ok(block.is_empty()))
	}

	/// Run `VirtualBlock::clear` under this block's lock.
	/// Users of previously returned offsets must stop using them before reuse.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn clear(&self) -> Result<(), AllocError> {
		self.locked(|block| {
			block.clear();
			Ok(())
		})
	}

	/// Run `VirtualBlock::statistics` under this block's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn statistics(&self) -> Result<VirtualBlockStats, AllocError> {
		self.locked(|block| Ok(block.statistics()))
	}

	/// Run `VirtualBlock::detailed_statistics` under this block's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn detailed_statistics(&self) -> Result<VirtualBlockDetails, AllocError> {
		self.locked(|block| Ok(block.detailed_statistics()))
	}

	/// Run `VirtualBlock::write_statistics_json` under this block's lock.
	/// # Errors
	/// Same errors as `VirtualBlock::write_statistics_json`, plus SynchronizationPoisoned.
	/// # Panics
	/// A writer panic propagates and poisons subsequent operations. Writers
	/// must not reenter the block while this method holds its lock.
	pub fn write_statistics_json(
		&self,
		output: &mut impl std::io::Write,
		detailed: bool,
	) -> Result<(), AllocError> {
		self.locked(|block| block.write_statistics_json(output, detailed))
	}

	/// Recover the exclusive block without native cleanup.
	/// # Errors
	/// PoisonError retains the block for explicit inspection/cleanup after a panic.
	#[allow(clippy::result_large_err)]
	pub fn into_inner(
		self,
	) -> Result<VirtualBlock<'callbacks>, PoisonError<VirtualBlock<'callbacks>>> {
		match self.inner.into_inner() {
			Ok(owner) => Ok(owner.0),
			Err(error) => Err(PoisonError::new(error.into_inner().0)),
		}
	}
}
