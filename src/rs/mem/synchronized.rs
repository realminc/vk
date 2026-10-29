//! Opt-in internally synchronized native access, following VMA 3.4.0's vector
//! and map/bind locks. Rust adaptation: one placement owner, stable per-backing
//! mutexes, checked native leases, explicit scope partitioning and poison recovery.
use super::{AllocError, Allocator, AllocatorStats};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError, TryLockError};

#[path = "synchronized/access.rs"]
mod access;
#[path = "synchronized/virtual_block.rs"]
mod virtual_block;
pub use virtual_block::SynchronizedVirtualBlock;

/// One allocator with synchronized direct operations and concurrent pool scopes.
///
/// Placement uses one mutex per owner. Direct map/unmap/bind operations acquire
/// a live allocation lease, then execute under that backing's stable mutex
/// without the placement mutex. Other backings can map/bind, allocate and free
/// concurrently, including inside the same pool. Regions sharing one backing
/// serialize native map/bind work and retain exact explicit/persistent counts.
/// The first such operation fallibly reserves callback-owned backing metadata.
///
/// Borrowed results, relocation and global services use exclusive access and
/// wait for outstanding native leases. `try_with_allocator` reports busy during
/// those leases. Borrowed pool/type/block scopes add independent placement
/// owners for their documented domains. There is no process-global lock or
/// new Send/Sync assertion on ordinary [`Allocator`]. Performance is unmeasured.
/// Callbacks must never reenter this owner or create cross-owner lock cycles.
/// Vulkan completion and mapped-byte synchronization remain caller obligations.
/// Drop frees CPU metadata only; explicitly close native storage before Drop.
/// A scoped operation cannot return its allocator borrow:
/// ```compile_fail
/// # fn escape(shared: &vk::SynchronizedAllocator<'_>) {
/// let escaped = unsafe { shared.with_allocator(|allocator| allocator) };
/// # }
/// ```
/// An existing ordinary allocator's device borrow may still be shortened:
/// ```
/// fn shorten<'device: 'scope, 'scope>(owner: vk::Allocator<'device>) -> vk::Allocator<'scope> {
///     owner
/// }
/// ```
/// The ordinary allocator remains non-Send:
/// ```compile_fail
/// fn require_send<T: Send>() {}
/// require_send::<vk::Allocator<'static>>();
/// ```
///
/// ```no_run
/// # fn example(allocator: vk::Allocator<'_>) -> Result<(), vk::AllocError> {
/// // SAFETY: retained chains/callbacks and mapped storage satisfy new's
/// // any-thread contract through final host cleanup.
/// let shared = unsafe { vk::SynchronizedAllocator::new(allocator) };
/// std::thread::scope(|scope| {
///     scope.spawn(|| {
///         // SAFETY: no replacement/thread-affine state is installed; no
///         // native work is live and ordinary close preconditions hold.
///         unsafe { shared.with_allocator(|allocator| allocator.close()) }??;
///         Ok::<_, vk::AllocError>(())
///     }).join().unwrap()
/// })?;
/// # Ok(())
/// # }
/// ```
pub struct SynchronizedAllocator<'device> {
	pub(in crate::mem) inner: Mutex<TransferableAllocator<'device>>,
	idle: Condvar,
}

pub(in crate::mem) struct TransferableAllocator<'device> {
	pub(in crate::mem) allocator: Allocator<'device>,
	pub(in crate::mem) poisoned: bool,
	pub(in crate::mem) active: usize,
}

// SAFETY: this wrapper is created only by unsafe new, which establishes
// any-thread validity of every retained native/callback/chain/map reference.
// CPU containers own their initialized storage; raw mapping pointers are opaque
// addresses, not Rust references or permission to dereference. Device is Sync.
// Moving changes no native identity. The wrapper is private and never exposed;
// the placement mutex covers registries, budgets, native counts and leases;
// stable backing mutexes cover native mapping/binding transitions. with_allocator's unsafe contract preserves
// these invariants for replacement/registration and all escaping raw pointers.
// No Sync is asserted for the underlying allocator. Drop only releases CPU/OS
// metadata, whose any-thread callback contract is retained by the device borrow.
unsafe impl Send for TransferableAllocator<'_> {}

impl<'device> SynchronizedAllocator<'device> {
	/// Adopt one allocator for synchronized access from any host thread.
	/// No native operation, allocation, mapping or thread is started.
	/// # Safety
	/// Every retained callback record/code/user datum, pool allocation chain and
	/// nested external reference must remain valid for the device borrow and
	/// support access on any participating thread, including final CPU cleanup.
	/// Retained records must be immutable across threads; callback-owned mutable
	/// state needs its own synchronization when shared with independent owners.
	/// Existing mapped-byte uses must be synchronized against subsequent operations.
	/// Native contexts must meet Vulkan's host-thread lifetime/valid-usage contract.
	/// No callback may unwind or reenter this same allocator.
	pub unsafe fn new(allocator: Allocator<'device>) -> Self {
		Self {
			inner: Mutex::new(TransferableAllocator {
				allocator,
				poisoned: false,
				active: 0,
			}),
			idle: Condvar::new(),
		}
	}

	pub(in crate::mem) fn lock_owner(
		&self,
	) -> Result<MutexGuard<'_, TransferableAllocator<'device>>, AllocError> {
		let guard = self
			.inner
			.lock()
			.map_err(|_| AllocError::SynchronizationPoisoned)?;
		if guard.poisoned {
			return Err(AllocError::SynchronizationPoisoned);
		}
		Ok(guard)
	}

	pub(in crate::mem) fn wait_owner<'owner>(
		&'owner self,
		guard: MutexGuard<'owner, TransferableAllocator<'device>>,
	) -> Result<MutexGuard<'owner, TransferableAllocator<'device>>, AllocError> {
		let guard = self
			.idle
			.wait(guard)
			.map_err(|_| AllocError::SynchronizationPoisoned)?;
		if guard.poisoned {
			return Err(AllocError::SynchronizationPoisoned);
		}
		Ok(guard)
	}

	pub(in crate::mem) fn lock_idle(
		&self,
	) -> Result<MutexGuard<'_, TransferableAllocator<'device>>, AllocError> {
		let mut guard = self.lock_owner()?;
		while guard.active != 0 {
			guard = self.wait_owner(guard)?;
		}
		Ok(guard)
	}

	pub(in crate::mem) fn notify_idle(&self) {
		self.idle.notify_all();
	}

	/// Run one operation/batch with exclusive access to the canonical allocator.
	/// The borrow cannot escape the closure. Returned tokens/handles remain raw
	/// native identities; no lock covers their later GPU or mapped-byte uses.
	/// # Errors
	/// Returns SynchronizationPoisoned without running the closure after a panic.
	/// Operation errors remain the closure's result and do not poison the mutex.
	/// # Panics
	/// A closure panic propagates and poisons subsequent access. Reentrant use of
	/// this same allocator can deadlock and violates the unsafe access contract.
	/// # Safety
	/// Preserve new's any-thread contract for every retained registration/state
	/// change. Do not replace the allocator with thread-affine state. All native
	/// operation/callback/completion rules apply. Inside any partitioned scope, never move
	/// out or replace the borrowed child allocator. Synchronize any escaping mapped
	/// pointer before unmap/free/relocation; the mutex protects metadata only.
	pub unsafe fn with_allocator<R>(
		&self,
		operation: impl FnOnce(&mut Allocator<'device>) -> R,
	) -> Result<R, AllocError> {
		let mut guard = self.lock_idle()?;
		Ok(operation(&mut guard.allocator))
	}

	/// Attempt one exclusive operation without waiting for another host user.
	/// `None` means busy; poison is an error and never bypassed.
	/// # Errors
	/// Same poison refusal as with_allocator; operation failures stay in R.
	/// # Safety
	/// The complete with_allocator contract applies, including any-thread retained
	/// state, callback non-reentrancy and synchronization of escaping mappings.
	pub unsafe fn try_with_allocator<R>(
		&self,
		operation: impl FnOnce(&mut Allocator<'device>) -> R,
	) -> Result<Option<R>, AllocError> {
		let mut guard = match self.inner.try_lock() {
			Ok(guard) => guard,
			Err(TryLockError::WouldBlock) => return Ok(None),
			Err(TryLockError::Poisoned(_)) => return Err(AllocError::SynchronizationPoisoned),
		};
		if guard.poisoned {
			return Err(AllocError::SynchronizationPoisoned);
		}
		if guard.active != 0 {
			return Ok(None);
		}
		Ok(Some(operation(&mut guard.allocator)))
	}

	pub(in crate::mem) fn owner_mut(&mut self) -> &mut Allocator<'device> {
		&mut self
			.inner
			.get_mut()
			.unwrap_or_else(PoisonError::into_inner)
			.allocator
	}

	pub(in crate::mem) fn is_poisoned(&self) -> bool {
		self.inner.is_poisoned() || self.inner.lock().is_ok_and(|owner| owner.poisoned)
	}

	/// Run independent custom pools concurrently through borrowed child owners.
	///
	/// Children correspond to `pools` in input order, preserve allocation/pool
	/// identities and use the same canonical methods through `with_allocator`.
	/// Each child serializes its own pool; different pools can execute native
	/// operations concurrently, even on the same memory type. A shared reservation
	/// lock enforces aggregate native-count, heap and within-budget limits, but
	/// never covers native PFNs. The parent remains locked for the whole scope.
	/// Its ordinary operations, statistics and relocation resume after restoration.
	///
	/// All scope metadata uses the allocator's CPU callbacks. Merge capacity is
	/// reserved before native effects; scope exit and panic restoration allocate
	/// no host memory and perform no native cleanup. Existing mappings remain live.
	/// Budget pressure is sampled at entry and corrected for all local reservations
	/// through this batch; split long-lived workloads into bounded scopes to refresh.
	///
	/// ```no_run
	/// # fn example(shared: &vk::SynchronizedAllocator<'_>, first: &vk::Pool, second: &vk::Pool) -> Result<(), vk::AllocError> {
	/// // SAFETY: all retained references/callbacks support concurrent workers;
	/// // this batch installs no thread-affine state and leaves native cleanup explicit.
	/// unsafe { shared.with_pools(&[first, second], |children| {
	///     std::thread::scope(|scope| {
	///         for (child, pool) in children.iter().zip([first, second]) {
	///             scope.spawn(move || child.with_allocator(|allocator| {
	///                 allocator.pool_statistics(pool)
	///             }).unwrap().unwrap());
	///         }
	///     });
	/// })?; }
	/// # Ok(())
	/// # }
	/// ```
	/// # Errors
	/// Rejects foreign/stale/duplicate pools, nested pool scopes, preparation OOM,
	/// and poisoned access. A child panic poisons the parent even if caught by
	/// the operation. Cached export-root ownership follows its backing memory across the scope.
	/// # Safety
	/// The complete `with_allocator` contract applies to every child. Use only
	/// its selected pool for allocations; allocator-wide changes are rejected.
	/// Never replace, move out or retain a child's allocator/context. All borrowed
	/// workers must finish before returning; do not reenter the locked parent.
	/// Synchronize shared callbacks/user data across pools. Mapped-byte and GPU
	/// ownership/completion remain explicit, including after a worker panic.
	pub unsafe fn with_pools<R>(
		&self,
		pools: &[&super::Pool],
		operation: impl FnOnce(&[Self]) -> R,
	) -> Result<R, AllocError> {
		// SAFETY: same any-thread, non-reentrancy and borrowed-worker contract.
		unsafe { self.with_domains(super::allocator::Selection::Pools(pools), operation) }
	}

	/// Run independent default memory-type vectors concurrently.
	///
	/// Children correspond to distinct native memory-type indices. They own only
	/// default storage of that type, including separate allocations and cached
	/// blocks; custom pools remain with the parent. Native type selection cannot
	/// escape its selected type. Tokens/maps and accounting retain parent identity.
	///
	/// ```no_run
	/// # fn example(shared: &vk::SynchronizedAllocator<'_>) -> Result<(), vk::AllocError> {
	/// // SAFETY: retained device/callback state supports these joined workers;
	/// // no replacement, registration or mapped byte access occurs in this batch.
	/// unsafe { shared.with_memory_types(&[0], |children| {
	///     std::thread::scope(|scope| {
	///         scope.spawn(|| children[0].with_allocator(|allocator| {
	///             allocator.memory_type_properties(0)
	///         }).unwrap().unwrap()).join().unwrap();
	///     });
	/// })?; }
	/// # Ok(())
	/// # }
	/// ```
	/// # Errors
	/// Rejects absent/duplicate types, nested scopes, poison and metadata OOM.
	/// # Safety
	/// The complete `with_pools` worker/callback/native-lifetime contract applies.
	/// Use default allocation policies, without a custom pool. Never move out or
	/// replace child owners, reenter the parent or form cross-child lock cycles.
	pub unsafe fn with_memory_types<R>(
		&self,
		memory_types: &[u32],
		operation: impl FnOnce(&[Self]) -> R,
	) -> Result<R, AllocError> {
		// SAFETY: same borrowed scope and any-thread retained-state contract.
		unsafe {
			self.with_domains(
				super::allocator::Selection::MemoryTypes(memory_types),
				operation,
			)
		}
	}

	/// Run mapping/cache/binding work concurrently on distinct native blocks.
	///
	/// Each seed is a live allocation; seeds must select different backing memory
	/// objects. A child contains every live region on that backing object, including
	/// regions not used as seeds. Regions sharing a block share its one mutex/map
	/// count. Different blocks of one custom/default pool can progress independently.
	/// Allocation/free, pool changes, trim and relocation are excluded for this
	/// borrowed phase. Map holds, metadata, cached exports and original block order
	/// return to the parent, including after panic. No native cleanup occurs on exit.
	/// A child borrow cannot escape this phase:
	/// ```compile_fail
	/// # fn escape(shared: &vk::SynchronizedAllocator<'_>, seed: &vk::Allocation) {
	/// let child = unsafe { shared.with_blocks(&[seed], |children| &children[0]) };
	/// # }
	/// ```
	/// # Errors
	/// Rejects stale/foreign seeds, duplicate backings, nested scopes, poison and OOM.
	/// # Safety
	/// The complete `with_pools` borrowing/any-thread/native contract applies.
	/// Use each token only in its matching child. No worker/reference may escape;
	/// synchronize mapped bytes and external object uses, and avoid lock cycles.
	pub unsafe fn with_blocks<R>(
		&self,
		allocations: &[&super::Allocation],
		operation: impl FnOnce(&[Self]) -> R,
	) -> Result<R, AllocError> {
		// SAFETY: matching live seeds and the same bounded borrowed-worker contract.
		unsafe { self.with_domains(super::allocator::Selection::Blocks(allocations), operation) }
	}

	unsafe fn with_domains<R>(
		&self,
		selection: super::allocator::Selection<'_>,
		operation: impl FnOnce(&[Self]) -> R,
	) -> Result<R, AllocError> {
		let mut guard = self.lock_idle()?;
		// SAFETY: the parent lock excludes all ordinary access; caller retains
		// every worker inside the borrowed scope and preserves any-thread state.
		let (result, poisoned) =
			unsafe { super::allocator::with_domains(&mut guard.allocator, selection, operation) }?;
		if poisoned {
			guard.poisoned = true;
			return Err(AllocError::SynchronizationPoisoned);
		}
		Ok(result)
	}

	/// Observe a serialized allocator-owned statistics snapshot.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn statistics(&self) -> Result<AllocatorStats, AllocError> {
		let guard = self.lock_idle()?;
		Ok(guard.allocator.statistics())
	}

	/// Recover the exclusive allocator without performing native cleanup.
	/// # Errors
	/// PoisonError retains the allocator for deliberate inspection/recovery using
	/// into_inner; do not assume panic-interrupted native transactions completed.
	// Keep the original native owner in the poison result without a new fallible
	// allocation during recovery. Boxing here could lose explicit cleanup on OOM.
	#[allow(clippy::result_large_err)]
	pub fn into_inner(self) -> Result<Allocator<'device>, PoisonError<Allocator<'device>>> {
		match self.inner.into_inner() {
			Ok(owner) if !owner.poisoned => Ok(owner.allocator),
			Ok(owner) => Err(PoisonError::new(owner.allocator)),
			Err(error) => Err(PoisonError::new(error.into_inner().allocator)),
		}
	}
}
