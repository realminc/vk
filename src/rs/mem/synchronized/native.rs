//! Dynamic backing leases for map/unmap/bind, following VMA 3.4.0's independent
//! VmaDeviceMemoryBlock lock. Adaptation: reserve live ownership under the vector
//! mutex, run the canonical native transition under the stable backing mutex,
//! release that mutex before publication. No vector/registry reference escapes.
use super::*;
use crate::mem::{SynchronizedAllocator, block::SharedMapping};
use std::sync::{MutexGuard, PoisonError};

fn lock_allocation<'owner, 'device>(
	shared: &'owner SynchronizedAllocator<'device>,
	allocation: &Allocation,
) -> Result<MutexGuard<'owner, crate::mem::synchronized::TransferableAllocator<'device>>, AllocError>
{
	let mut owner = shared.lock_owner()?;
	loop {
		if owner.allocator.validate(allocation)?.native_users == 0 {
			return Ok(owner);
		}
		owner = shared.wait_owner(owner)?;
	}
}

pub(in crate::mem) fn with_allocation<R>(
	shared: &SynchronizedAllocator<'_>,
	serial: u64,
	operation: impl FnOnce(&mut Allocator<'_>) -> Result<R, AllocError>,
) -> Result<R, AllocError> {
	let mut owner = shared.lock_owner()?;
	while owner
		.allocator
		.live
		.get(&serial)
		.is_some_and(|live| live.native_users != 0)
	{
		owner = shared.wait_owner(owner)?;
	}
	operation(&mut owner.allocator)
}

struct Lease<'owner, 'device> {
	shared: &'owner SynchronizedAllocator<'device>,
	serial: u64,
	mapping: SharedMapping<'device>,
	finished: bool,
}

impl<'owner, 'device> Lease<'owner, 'device> {
	fn new(
		shared: &'owner SynchronizedAllocator<'device>,
		owner: &mut crate::mem::synchronized::TransferableAllocator<'device>,
		allocation: &Allocation,
	) -> Result<Self, AllocError> {
		let count = owner
			.active
			.checked_add(1)
			.ok_or(AllocError::InvalidRange)?;
		let host = owner.allocator.host;
		let mapping = owner.allocator.mapping_owner_mut(allocation).share(host)?;
		owner
			.allocator
			.live
			.get_mut(&allocation.serial)
			.expect("validated live region")
			.native_users = 1;
		owner.active = count;
		Ok(Self {
			shared,
			serial: allocation.serial,
			mapping,
			finished: false,
		})
	}

	fn finish<R>(mut self, operation: impl FnOnce(&mut Allocator<'device>) -> R) -> R {
		// Always publish the native result, even if an unrelated writer poisoned
		// the owner. Refusing later access must not lose a successful map/bind.
		let mut owner = self
			.shared
			.inner
			.lock()
			.unwrap_or_else(PoisonError::into_inner);
		let result = operation(&mut owner.allocator);
		let live = owner
			.allocator
			.live
			.get_mut(&self.serial)
			.expect("leased live region");
		live.native_users = 0;
		live.pending_binding = None;
		owner.active -= 1;
		self.finished = true;
		self.shared.notify_idle();
		result
	}
}

impl Drop for Lease<'_, '_> {
	fn drop(&mut self) {
		if !self.finished {
			// A Rust unwind cannot certify the interrupted native transition.
			// Retain all native ownership and refuse access until consuming recovery.
			let mut owner = self
				.shared
				.inner
				.lock()
				.unwrap_or_else(PoisonError::into_inner);
			owner.poisoned = true;
			let live = owner
				.allocator
				.live
				.get_mut(&self.serial)
				.expect("leased live region");
			live.native_users = 0;
			live.pending_binding = None;
			owner.active -= 1;
			self.shared.notify_idle();
		}
	}
}

pub(super) struct MapOperation<'device> {
	device: DeviceRetention<'device>,
	serial: u64,
	handle: vk::DeviceMemory,
	block_size: u64,
	offset: u64,
	count: u32,
	acquire: bool,
}

impl<'device> MapOperation<'device> {
	pub(super) fn prepare(
		allocator: &Allocator<'device>,
		allocation: &Allocation,
		acquire: bool,
	) -> Result<Self, AllocError> {
		let live = allocator.validate(allocation)?;
		if live.native_users != 0 {
			return Err(AllocError::InvalidRequest("native operation in progress"));
		}
		let count = if acquire {
			let count = live
				.map_count
				.checked_add(1)
				.ok_or(AllocError::InvalidRange)?;
			if !allocation.is_host_visible() {
				return Err(AllocError::NotHostVisible);
			}
			count
		} else {
			live.map_count.checked_sub(1).ok_or(AllocError::NotMapped)?
		};
		Ok(Self {
			device: allocator.device.clone(),
			serial: allocation.serial,
			handle: allocation.handle,
			block_size: allocation.block_size,
			offset: allocation.offset,
			count,
			acquire,
		})
	}

	pub(super) unsafe fn execute(
		&self,
		mapping: &mut super::super::block::Mapping,
	) -> Result<*mut std::ffi::c_void, AllocError> {
		if self.acquire {
			// SAFETY: the exclusive owner or live native lease prevents retirement;
			// the backing guard serializes map/bind; caller synchronizes byte use.
			unsafe { mapping.acquire(self.device.as_device(), self.handle, self.block_size) }
				.map(|base| base.cast::<u8>().wrapping_add(self.offset as usize).cast())
		} else {
			// SAFETY: the checked explicit hold exists and caller ends its uses.
			unsafe { mapping.release(self.device.as_device(), self.handle) }
				.map(|()| std::ptr::null_mut())
		}
	}

	pub(super) fn commit(
		self,
		allocator: &mut Allocator<'_>,
		result: Result<*mut std::ffi::c_void, AllocError>,
	) -> Result<*mut std::ffi::c_void, AllocError> {
		if result.is_ok() {
			allocator
				.live
				.get_mut(&self.serial)
				.expect("live map operation")
				.map_count = self.count;
		}
		result
	}
}

pub(in crate::mem) unsafe fn map(
	shared: &SynchronizedAllocator<'_>,
	allocation: &Allocation,
	acquire: bool,
) -> Result<*mut std::ffi::c_void, AllocError> {
	let mut owner = lock_allocation(shared, allocation)?;
	let operation = MapOperation::prepare(&owner.allocator, allocation, acquire)?;
	let lease = Lease::new(shared, &mut owner, allocation)?;
	drop(owner);
	let result = {
		let mut mapping = lease.mapping.lock();
		// SAFETY: lease owns the stable backing through native completion;
		// caller supplies the same map/unmap byte/hold contract as Allocator.
		unsafe { operation.execute(&mut mapping) }
	};
	// Lock order is vector -> backing. Never reacquire the vector while a
	// backing guard is live: placement/free may be waiting for this mutex.
	lease.finish(|allocator| operation.commit(allocator, result))
}

pub(in crate::mem) unsafe fn bind(
	shared: &SynchronizedAllocator<'_>,
	resource: Resource,
	allocation: &Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
	extended: bool,
) -> Result<(), AllocError> {
	let mut owner = lock_allocation(shared, allocation)?;
	// SAFETY: caller keeps this resource and bind chain live for the complete call.
	let operation = unsafe {
		binding::prepare_bind(
			&owner.allocator,
			resource,
			allocation,
			local_offset,
			next,
			extended,
			false,
		)
	}?;
	let lease = Lease::new(shared, &mut owner, allocation)?;
	owner
		.allocator
		.live
		.get_mut(&allocation.serial)
		.expect("leased live region")
		.pending_binding = Some((resource, operation.plane));
	drop(owner);
	let result = {
		let _backing = lease.mapping.lock();
		// SAFETY: resource claim and native backing remain live, and this
		// backing's map/bind mutex excludes competing native host operations.
		unsafe { operation.execute() }
	};
	lease.finish(|allocator| match result {
		Ok(()) => operation.commit(allocator),
		Err(error) => Err(error),
	})
}
