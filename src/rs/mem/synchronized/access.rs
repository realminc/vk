//! Direct `&self` operation access, delegating to the canonical allocator.
//!
//! Placement retains the owner mutex. Mapping/binding uses dynamic per-backing
//! leases and releases the placement mutex through the native call. Exclusive
//! services and borrowed access wait for those calls before observing/mutating
//! their state. Native byte/GPU synchronization remains the caller's obligation.
//!
//! Results borrowing allocator metadata and relocation contexts remain scoped
//! to `with_allocator`; no mutable allocator guard or borrowed result escapes.
use super::SynchronizedAllocator;
use crate::{
	self as vk,
	mem::{
		AllocError, AllocRequest, Allocation, AllocationChain, AllocationCreateInfo, AllocationInfo,
		Allocator, AllocatorInfo, BudgetSnapshot, CacheRange, DebugMemorySettings, DetailedStatistics,
		DeviceMemoryCallbacks, MemoryStatistics, Pool, PoolCreateInfo, PoolStats, TrimStats,
	},
};

impl<'device> SynchronizedAllocator<'device> {
	fn locked<R>(
		&self,
		operation: impl FnOnce(&mut Allocator<'device>) -> Result<R, AllocError>,
	) -> Result<R, AllocError> {
		let mut guard = self.lock_idle()?;
		operation(&mut guard.allocator)
	}

	fn placement<R>(
		&self,
		operation: impl FnOnce(&mut Allocator<'device>) -> Result<R, AllocError>,
	) -> Result<R, AllocError> {
		let mut guard = self.lock_owner()?;
		operation(&mut guard.allocator)
	}

	/// Number of live allocation registrations under this allocator's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn live_count(&self) -> Result<usize, AllocError> {
		self.locked(|allocator| Ok(allocator.live_count()))
	}

	/// Observe explicit close state under this allocator's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn is_closed(&self) -> Result<bool, AllocError> {
		self.locked(|allocator| Ok(allocator.is_closed()))
	}

	/// Optional caller-configured size limit under this allocator's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn max_allocation_size(&self) -> Result<Option<u64>, AllocError> {
		self.locked(|allocator| Ok(allocator.max_allocation_size()))
	}

	/// Close the allocator under its lock; later access reports `Closed`.
	/// Concurrent operations complete or fail before this runs.
	/// # Errors
	/// Same live-resource errors as `Allocator::close`, plus SynchronizationPoisoned.
	pub fn close(&self) -> Result<(), AllocError> {
		self.locked(Allocator::close)
	}

	/// Captured native identities under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::allocator_info`, plus SynchronizationPoisoned.
	pub fn allocator_info(&self) -> Result<AllocatorInfo, AllocError> {
		self.locked(|allocator| crate::mem::allocator_info(allocator))
	}

	/// Queried properties of one native memory type under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::memory_type_properties`, plus SynchronizationPoisoned.
	pub fn memory_type_properties(&self, index: u32) -> Result<vk::MemoryPropertyFlags, AllocError> {
		self.locked(|allocator| allocator.memory_type_properties(index))
	}

	/// Score memory types for one allocation request under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::find_memory_type`, plus SynchronizationPoisoned.
	pub fn find_memory_type(
		&self,
		bits: u32,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		self.locked(|allocator| allocator.find_memory_type(bits, options))
	}

	/// Score memory types for one buffer's requirements under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::find_memory_type_for_buffer_info`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same create-chain and requirement query contract as
	/// `Allocator::find_memory_type_for_buffer_info`.
	pub unsafe fn find_memory_type_for_buffer_info(
		&self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		// SAFETY: identical canonical query contract.
		unsafe { self.locked(|allocator| allocator.find_memory_type_for_buffer_info(info, options)) }
	}

	/// Score memory types for one image's requirements under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::find_memory_type_for_image_info`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same create-chain and requirement query contract as
	/// `Allocator::find_memory_type_for_image_info`.
	pub unsafe fn find_memory_type_for_image_info(
		&self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		// SAFETY: identical canonical query contract.
		unsafe { self.locked(|allocator| allocator.find_memory_type_for_image_info(info, options)) }
	}

	/// Run `Allocator::allocate` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same request/chain/import and completion contract as `Allocator::allocate`.
	pub unsafe fn allocate(&self, request: &AllocRequest) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.allocate(request)) }
	}

	/// Run `Allocator::allocate_with_options` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate_with_options`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same requirements/options/chain contract as `Allocator::allocate_with_options`.
	pub unsafe fn allocate_with_options(
		&self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.allocate_with_options(requirements, options)) }
	}

	/// Run `Allocator::allocate_dedicated` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate_dedicated`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same requirements/chain/import contract as `Allocator::allocate_dedicated`.
	pub unsafe fn allocate_dedicated(
		&self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo,
		chain: Option<AllocationChain>,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			self.placement(|allocator| allocator.allocate_dedicated(requirements, options, chain))
		}
	}

	/// Run `Allocator::allocate_many` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate_many`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same requirements/options contract as `Allocator::allocate_many`.
	pub unsafe fn allocate_many(
		&self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo,
		count: usize,
	) -> Result<Vec<Allocation>, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.allocate_many(requirements, options, count)) }
	}

	/// Run `Allocator::free` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::free`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same live-token, mapped-byte and completion contract as `Allocator::free`.
	pub unsafe fn free(&self, allocation: &mut Allocation) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::with_allocation(self, allocation.serial, |allocator| {
				allocator.free(allocation)
			})
		}
	}

	/// Run `Allocator::free_many` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::free_many`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same live-token and completion contract as `Allocator::free_many`.
	pub unsafe fn free_many(&self, allocations: &mut [Allocation]) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.free_many(allocations)) }
	}

	/// Checked allocation snapshot under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::allocation_info`, plus SynchronizationPoisoned.
	pub fn allocation_info(&self, allocation: &Allocation) -> Result<AllocationInfo, AllocError> {
		self.locked(|allocator| allocator.allocation_info(allocation))
	}

	/// Run `Allocator::set_allocation_pinned` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_allocation_pinned`, plus SynchronizationPoisoned.
	pub fn set_allocation_pinned(
		&self,
		allocation: &Allocation,
		pinned: bool,
	) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_allocation_pinned(allocation, pinned))
	}

	/// Run `Allocator::set_allocation_name` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_allocation_name`, plus SynchronizationPoisoned.
	pub fn set_allocation_name(
		&self,
		allocation: &Allocation,
		name: Option<&str>,
	) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_allocation_name(allocation, name))
	}

	/// Run `Allocator::set_allocation_user_data` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_allocation_user_data`, plus SynchronizationPoisoned.
	pub fn set_allocation_user_data(
		&self,
		allocation: &Allocation,
		data: usize,
	) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_allocation_user_data(allocation, data))
	}

	/// Run `Allocator::create_buffer` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_buffer`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/allocation contract as `Allocator::create_buffer`.
	pub unsafe fn create_buffer(
		&self,
		info: &vk::BufferCreateInfo,
		required: vk::MemoryPropertyFlags,
		preferred: vk::MemoryPropertyFlags,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_buffer(info, required, preferred)) }
	}

	/// Run `Allocator::create_buffer_with_options` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_buffer_with_options`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/allocation contract as `Allocator::create_buffer_with_options`.
	pub unsafe fn create_buffer_with_options(
		&self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_buffer_with_options(info, options)) }
	}

	/// Run `Allocator::create_dedicated_buffer` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_dedicated_buffer`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/chain/import contract as `Allocator::create_dedicated_buffer`.
	pub unsafe fn create_dedicated_buffer(
		&self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo,
		chain: Option<AllocationChain>,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_dedicated_buffer(info, options, chain)) }
	}

	/// Run `Allocator::destroy_buffer` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::destroy_buffer`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource ownership and completion contract as `Allocator::destroy_buffer`.
	pub unsafe fn destroy_buffer(
		&self,
		buffer: vk::Buffer,
		allocation: &mut Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::with_allocation(self, allocation.serial, |allocator| {
				allocator.destroy_buffer(buffer, allocation)
			})
		}
	}

	/// Run `Allocator::create_image` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_image`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/allocation contract as `Allocator::create_image`.
	pub unsafe fn create_image(
		&self,
		info: &vk::ImageCreateInfo,
		required: vk::MemoryPropertyFlags,
		preferred: vk::MemoryPropertyFlags,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_image(info, required, preferred)) }
	}

	/// Run `Allocator::create_image_with_options` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_image_with_options`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/allocation contract as `Allocator::create_image_with_options`.
	pub unsafe fn create_image_with_options(
		&self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_image_with_options(info, options)) }
	}

	/// Run `Allocator::create_dedicated_image` under this allocator's lock.
	/// # Errors
	/// Same transaction errors as `Allocator::create_dedicated_image`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource/chain/import contract as `Allocator::create_dedicated_image`.
	pub unsafe fn create_dedicated_image(
		&self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo,
		chain: Option<AllocationChain>,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.create_dedicated_image(info, options, chain)) }
	}

	/// Run `Allocator::destroy_image` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::destroy_image`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same resource ownership and completion contract as `Allocator::destroy_image`.
	pub unsafe fn destroy_image(
		&self,
		image: vk::Image,
		allocation: &mut Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::with_allocation(self, allocation.serial, |allocator| {
				allocator.destroy_image(image, allocation)
			})
		}
	}

	/// Run `Allocator::allocate_for_buffer` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate_for_buffer`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same external-resource and allocation contract as `Allocator::allocate_for_buffer`.
	pub unsafe fn allocate_for_buffer(
		&self,
		buffer: vk::Buffer,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.allocate_for_buffer(buffer, options)) }
	}

	/// Run `Allocator::allocate_for_image` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::allocate_for_image`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same external-resource and allocation contract as `Allocator::allocate_for_image`.
	pub unsafe fn allocate_for_image(
		&self,
		image: vk::Image,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.placement(|allocator| allocator.allocate_for_image(image, options)) }
	}

	/// Run `Allocator::create_aliasing_buffer` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::create_aliasing_buffer`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same alias/storage contract as `Allocator::create_aliasing_buffer`.
	pub unsafe fn create_aliasing_buffer(
		&self,
		allocation: &Allocation,
		local_offset: u64,
		info: &vk::BufferCreateInfo,
	) -> Result<vk::Buffer, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			self.locked(|allocator| allocator.create_aliasing_buffer(allocation, local_offset, info))
		}
	}

	/// Run `Allocator::create_aliasing_image` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::create_aliasing_image`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same alias/storage contract as `Allocator::create_aliasing_image`.
	pub unsafe fn create_aliasing_image(
		&self,
		allocation: &Allocation,
		local_offset: u64,
		info: &vk::ImageCreateInfo,
	) -> Result<vk::Image, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			self.locked(|allocator| allocator.create_aliasing_image(allocation, local_offset, info))
		}
	}

	/// Run `Allocator::copy_to_allocation` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::copy_to_allocation`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same mapped-range and byte-validity contract as `Allocator::copy_to_allocation`.
	pub unsafe fn copy_to_allocation(
		&self,
		allocation: &Allocation,
		offset: u64,
		source: &[u8],
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.copy_to_allocation(allocation, offset, source)) }
	}

	/// Run `Allocator::copy_from_allocation` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::copy_from_allocation`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same mapped-range and byte-validity contract as `Allocator::copy_from_allocation`.
	pub unsafe fn copy_from_allocation(
		&self,
		allocation: &Allocation,
		offset: u64,
		destination: &mut [u8],
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			self.locked(|allocator| allocator.copy_from_allocation(allocation, offset, destination))
		}
	}

	/// Run `Allocator::bind_buffer_memory` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked errors as `Allocator::bind_buffer_memory`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same native binding contract as `Allocator::bind_buffer_memory`.
	pub unsafe fn bind_buffer_memory(
		&self,
		buffer: vk::Buffer,
		allocation: &Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::bind(
				self,
				buffer.into(),
				allocation,
				0,
				std::ptr::null(),
				false,
			)
		}
	}

	/// Run `Allocator::bind_image_memory` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked errors as `Allocator::bind_image_memory`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same native binding contract as `Allocator::bind_image_memory`.
	pub unsafe fn bind_image_memory(
		&self,
		image: vk::Image,
		allocation: &Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::bind(
				self,
				image.into(),
				allocation,
				0,
				std::ptr::null(),
				false,
			)
		}
	}

	/// Run `Allocator::bind_buffer_memory2` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked errors as `Allocator::bind_buffer_memory2`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same native chain/binding contract as `Allocator::bind_buffer_memory2`.
	pub unsafe fn bind_buffer_memory2(
		&self,
		buffer: vk::Buffer,
		allocation: &Allocation,
		local_offset: u64,
		next: *const std::ffi::c_void,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::bind(self, buffer.into(), allocation, local_offset, next, true)
		}
	}

	/// Run `Allocator::bind_image_memory2` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked errors as `Allocator::bind_image_memory2`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same native chain/binding contract as `Allocator::bind_image_memory2`.
	pub unsafe fn bind_image_memory2(
		&self,
		image: vk::Image,
		allocation: &Allocation,
		local_offset: u64,
		next: *const std::ffi::c_void,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			crate::mem::allocator::native::bind(self, image.into(), allocation, local_offset, next, true)
		}
	}

	/// Run `Allocator::map` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked/native errors as `Allocator::map`, plus SynchronizationPoisoned.
	/// # Safety
	/// May fail to reserve the stable backing metadata before native effects.
	/// Same mapping, mapped-byte synchronization and completion contract as
	/// `Allocator::map`; escaping pointers retain `SynchronizedAllocator` rules.
	pub unsafe fn map(&self, allocation: &Allocation) -> Result<*mut std::ffi::c_void, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { crate::mem::allocator::native::map(self, allocation, true) }
	}

	/// Run `Allocator::unmap` under a live lease and this backing's mutex.
	/// # Errors
	/// Same checked errors as `Allocator::unmap`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same mapped-byte and completion contract as `Allocator::unmap`.
	pub unsafe fn unmap(&self, allocation: &Allocation) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { crate::mem::allocator::native::map(self, allocation, false) }.map(|_| ())
	}

	/// Current mapping pointer for one allocation under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::mapped_pointer`, plus SynchronizationPoisoned.
	pub fn mapped_pointer(
		&self,
		allocation: &Allocation,
	) -> Result<Option<*mut std::ffi::c_void>, AllocError> {
		self.locked(|allocator| allocator.mapped_pointer(allocation))
	}

	/// Run `Allocator::flush` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::flush`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same cache-range and completion contract as `Allocator::flush`.
	pub unsafe fn flush(
		&self,
		allocation: &Allocation,
		offset: u64,
		size: u64,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.flush(allocation, offset, size)) }
	}

	/// Run `Allocator::invalidate` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::invalidate`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same cache-range and completion contract as `Allocator::invalidate`.
	pub unsafe fn invalidate(
		&self,
		allocation: &Allocation,
		offset: u64,
		size: u64,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.invalidate(allocation, offset, size)) }
	}

	/// Run `Allocator::flush_many` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::flush_many`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same cache-range contract as `Allocator::flush_many`.
	pub unsafe fn flush_many(&self, ranges: &[CacheRange<'_>]) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.flush_many(ranges)) }
	}

	/// Run `Allocator::invalidate_many` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::invalidate_many`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same cache-range contract as `Allocator::invalidate_many`.
	pub unsafe fn invalidate_many(&self, ranges: &[CacheRange<'_>]) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.invalidate_many(ranges)) }
	}

	/// Run `Allocator::set_heap_limit` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_heap_limit`, plus SynchronizationPoisoned.
	pub fn set_heap_limit(&self, heap: u32, limit: u64) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_heap_limit(heap, limit))
	}

	/// Cached budget snapshot under this allocator's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn budget_snapshot(&self) -> Result<BudgetSnapshot, AllocError> {
		self.locked(|allocator| Ok(allocator.budget_snapshot()))
	}

	/// Run `Allocator::query_budget` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::query_budget`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same instance/physical-device and PFN contract as `Allocator::query_budget`.
	pub unsafe fn query_budget(
		&self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<BudgetSnapshot, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.query_budget(instance, physical)) }
	}

	/// Run `Allocator::refresh_budget` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::refresh_budget`, plus SynchronizationPoisoned.
	pub fn refresh_budget(&self) -> Result<BudgetSnapshot, AllocError> {
		self.locked(Allocator::refresh_budget)
	}

	/// Run `Allocator::refresh_budget_with_context` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::refresh_budget_with_context`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same context/PFN contract as `Allocator::refresh_budget_with_context`.
	pub unsafe fn refresh_budget_with_context(
		&self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<BudgetSnapshot, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.refresh_budget_with_context(instance, physical)) }
	}

	/// Run `Allocator::heap_budgets` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::heap_budgets`, plus SynchronizationPoisoned.
	pub fn heap_budgets(&self) -> Result<BudgetSnapshot, AllocError> {
		self.locked(Allocator::heap_budgets)
	}

	/// Run `Allocator::set_current_frame_index` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_current_frame_index`, plus SynchronizationPoisoned.
	pub fn set_current_frame_index(&self, index: u32) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_current_frame_index(index))
	}

	/// Current frame index under this allocator's lock.
	/// # Errors
	/// Returns SynchronizationPoisoned after a closure panic.
	pub fn current_frame_index(&self) -> Result<u32, AllocError> {
		self.locked(|allocator| Ok(allocator.current_frame_index()))
	}

	/// Run `Allocator::trim` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::trim`, plus SynchronizationPoisoned.
	pub fn trim(&self) -> Result<TrimStats, AllocError> {
		self.locked(Allocator::trim)
	}

	/// Run `Allocator::trim_with_budget` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::trim_with_budget`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same instance/physical-device and PFN contract as `Allocator::trim_with_budget`.
	pub unsafe fn trim_with_budget(
		&self,
		instance: &vk::Instance,
		physical: vk::PhysicalDevice,
	) -> Result<TrimStats, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.trim_with_budget(instance, physical)) }
	}

	/// Run `Allocator::create_pool` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::create_pool`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same policy/storage contract as `Allocator::create_pool`.
	pub unsafe fn create_pool(&self, info: &PoolCreateInfo) -> Result<Pool, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.create_pool(info)) }
	}

	/// Run `Allocator::create_pool_with_chain` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::create_pool_with_chain`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same retained-chain contract as `Allocator::create_pool_with_chain`;
	/// the chain must also satisfy `SynchronizedAllocator::new`'s any-thread rules.
	pub unsafe fn create_pool_with_chain(
		&self,
		info: &PoolCreateInfo,
		chain: AllocationChain<'device>,
	) -> Result<Pool, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.create_pool_with_chain(info, chain)) }
	}

	/// Run `Allocator::destroy_pool` under this allocator's lock.
	/// # Errors
	/// Same live-resource errors as `Allocator::destroy_pool`, plus SynchronizationPoisoned.
	pub fn destroy_pool(&self, pool: &mut Pool) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.destroy_pool(pool))
	}

	/// One pool's statistics under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::pool_statistics`, plus SynchronizationPoisoned.
	pub fn pool_statistics(&self, pool: &Pool) -> Result<PoolStats, AllocError> {
		self.locked(|allocator| allocator.pool_statistics(pool))
	}

	/// Run `Allocator::set_pool_name` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_pool_name`, plus SynchronizationPoisoned.
	pub fn set_pool_name(&self, pool: &Pool, name: Option<&str>) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_pool_name(pool, name))
	}

	/// Run `Allocator::set_preferred_block_size` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_preferred_block_size`, plus SynchronizationPoisoned.
	pub fn set_preferred_block_size(&self, size: u64) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_preferred_block_size(size))
	}

	/// Run `Allocator::set_mapping_hysteresis` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_mapping_hysteresis`, plus SynchronizationPoisoned.
	pub fn set_mapping_hysteresis(&self, enabled: bool) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_mapping_hysteresis(enabled))
	}

	/// Aggregate memory statistics under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::detailed_statistics`, plus SynchronizationPoisoned.
	pub fn detailed_statistics(&self) -> Result<MemoryStatistics, AllocError> {
		self.locked(|allocator| allocator.detailed_statistics())
	}

	/// One pool's detailed statistics under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::detailed_pool_statistics`, plus SynchronizationPoisoned.
	pub fn detailed_pool_statistics(&self, pool: &Pool) -> Result<DetailedStatistics, AllocError> {
		self.locked(|allocator| allocator.detailed_pool_statistics(pool))
	}

	/// Run `Allocator::write_statistics_json` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::write_statistics_json`, plus SynchronizationPoisoned.
	pub fn write_statistics_json(
		&self,
		output: &mut impl std::io::Write,
		detailed: bool,
	) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.write_statistics_json(output, detailed))
	}

	/// Run `Allocator::set_debug_memory_settings` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_debug_memory_settings`, plus SynchronizationPoisoned.
	pub fn set_debug_memory_settings(&self, settings: DebugMemorySettings) -> Result<(), AllocError> {
		self.locked(|allocator| allocator.set_debug_memory_settings(settings))
	}

	/// Run `Allocator::check_corruption` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::check_corruption`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same guard-pattern byte-validity contract as `Allocator::check_corruption`.
	pub unsafe fn check_corruption(&self, bits: u32) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.check_corruption(bits)) }
	}

	/// Run `Allocator::check_pool_corruption` under this allocator's lock.
	/// # Errors
	/// Same checked errors as `Allocator::check_pool_corruption`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same guard-pattern byte-validity contract as `Allocator::check_pool_corruption`.
	pub unsafe fn check_pool_corruption(&self, pool: &Pool) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.check_pool_corruption(pool)) }
	}

	/// Run `Allocator::set_external_memory_types` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_external_memory_types`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same capability/creation contract as `Allocator::set_external_memory_types`.
	pub unsafe fn set_external_memory_types(
		&self,
		types: &[vk::ExternalMemoryHandleTypeFlags],
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.set_external_memory_types(types)) }
	}

	/// One allocation's external handle types under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::external_memory_types`, plus SynchronizationPoisoned.
	pub fn external_memory_types(
		&self,
		allocation: &Allocation,
	) -> Result<vk::ExternalMemoryHandleTypeFlags, AllocError> {
		self.locked(|allocator| allocator.external_memory_types(allocation))
	}

	/// Run `Allocator::get_memory_fd` under this allocator's lock.
	/// # Errors
	/// Same checked/native errors as `Allocator::get_memory_fd`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same external-memory and descriptor-ownership contract as `Allocator::get_memory_fd`.
	#[cfg(unix)]
	pub unsafe fn get_memory_fd(
		&self,
		allocation: &Allocation,
		handle_type: vk::ExternalMemoryHandleTypeFlagBits,
	) -> Result<std::os::fd::RawFd, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.get_memory_fd(allocation, handle_type)) }
	}

	/// Run `Allocator::get_memory_win32_handle` under this allocator's lock.
	/// # Errors
	/// Same checked/native/OS errors as `Allocator::get_memory_win32_handle`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same sharing, target-process and handle-closing contract as
	/// `Allocator::get_memory_win32_handle`.
	#[cfg(all(windows, feature = "platform-win32"))]
	pub unsafe fn get_memory_win32_handle(
		&self,
		allocation: &Allocation,
		handle_type: vk::ExternalMemoryHandleTypeFlagBits,
		target_process: *mut std::ffi::c_void,
	) -> Result<vk::HANDLE, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe {
			self.locked(|allocator| {
				allocator.get_memory_win32_handle(allocation, handle_type, target_process)
			})
		}
	}

	/// Run `Allocator::set_device_memory_callbacks` under this allocator's lock.
	/// # Errors
	/// Same errors as `Allocator::set_device_memory_callbacks`, plus SynchronizationPoisoned.
	/// # Safety
	/// Same retained-callback contract as `Allocator::set_device_memory_callbacks`;
	/// callbacks must also satisfy `SynchronizedAllocator::new`'s any-thread rules.
	pub unsafe fn set_device_memory_callbacks(
		&self,
		callbacks: Option<&'device DeviceMemoryCallbacks>,
	) -> Result<(), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { self.locked(|allocator| allocator.set_device_memory_callbacks(callbacks)) }
	}
}
