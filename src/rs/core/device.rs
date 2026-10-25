//! Device dispatch: loaded via `vkGetDeviceProcAddr` for optimal routing.
use super::Instance;
use crate::core as vk;
use crate::load::{
	CallError, Capabilities, CommandSupport, CreatedHandle, InitializationFailure, LoadError,
};
use libloading::Library;
use std::sync::Arc;

/// Device handle and dispatch. Call `destroy` explicitly; Drop never destroys Vulkan objects.
pub struct Device {
	handle: vk::DeviceHandle,
	fp: vk::FnDevice,
	gdpa:
		unsafe extern "system" fn(vk::DeviceHandle, *const std::ffi::c_char) -> vk::PFN_vkVoidFunction,
	_library: Option<Arc<Library>>,
	capabilities: Option<Capabilities>,
}

impl Device {
	/// Associate a live raw device with the instance that created it.
	///
	/// # Safety
	/// `handle` must be live and have been created from `instance`. The caller
	/// transfers ownership and must not create duplicate wrappers.
	/// On error the caller retains ownership. This raw import has no creation
	/// provenance; enabled-command and feature validation remain caller obligations.
	pub unsafe fn new_from_handle(
		instance: &Instance,
		handle: vk::DeviceHandle,
	) -> Result<Self, LoadError> {
		if handle.is_null() {
			return Err(LoadError::NullHandle("device"));
		}
		let gdpa = instance
			.fp()
			.get_device_proc_addr
			.ok_or(LoadError::MissingSymbol("vkGetDeviceProcAddr"))?;
		// SAFETY: caller guarantees a live device from this instance; gdpa is valid.
		let fp = unsafe { vk::FnDevice::new_from_resolver(gdpa, handle) };
		if fp.destroy_device.is_none() {
			return Err(LoadError::MissingSymbol("vkDestroyDevice"));
		}
		Ok(Self {
			handle,
			fp,
			gdpa,
			_library: instance.library.clone(),
			capabilities: None,
		})
	}

	/// Import a native device with owned creation metadata and filtered dispatch.
	///
	/// # Errors
	/// Reports wrong snapshot scope/parent metadata, a null handle or unavailable
	/// resolver/destruction. Every error leaves native ownership with the caller.
	///
	/// # Safety
	/// Handle must be live, belong to instance and have no other wrapper owner.
	/// Capabilities must truthfully describe this exact device, its actual parent
	/// instance, selected physical-device version and enabled feature chain.
	/// Parent/callback lifetimes and ordinary Vulkan valid usage remain required.
	pub unsafe fn new_from_handle_with_capabilities(
		instance: &Instance,
		handle: vk::DeviceHandle,
		capabilities: Capabilities,
	) -> Result<Self, LoadError> {
		if !capabilities.is_device() {
			return Err(LoadError::InvalidMetadata(
				"device import needs device capabilities",
			));
		}
		if let Some(parent) = instance.capabilities()
			&& !capabilities.matches_instance(parent)
		{
			return Err(LoadError::InvalidMetadata(
				"device capabilities disagree with parent instance",
			));
		}
		// SAFETY: caller guarantees this device's lifetime, parent and exclusive import.
		let mut device = unsafe { Self::new_from_handle(instance, handle) }?;
		device
			.fp
			.retain_enabled(|command| capabilities.support(command) == CommandSupport::Enabled);
		device.capabilities = Some(capabilities);
		Ok(device)
	}

	/// Return the raw handle. Its use must respect this device's lifetime.
	pub fn handle(&self) -> vk::DeviceHandle {
		self.handle
	}

	/// Borrow the full device dispatch table.
	pub fn fp(&self) -> &vk::FnDevice {
		&self.fp
	}

	/// Creation version/extension provenance, or None for an imported raw handle.
	pub fn capabilities(&self) -> Option<&Capabilities> {
		self.capabilities.as_ref()
	}

	/// Destroy once, using callbacks compatible with device creation.
	///
	/// # Safety
	/// All children and outstanding device operations must be complete. The
	/// allocator must match creation, with its user data and functions still live.
	pub unsafe fn destroy(self, allocator: Option<&vk::AllocationCallbacks>) {
		// SAFETY: caller supplies the canonical destruction guarantees.
		unsafe { destroy_device(self, allocator) };
	}
}

use super::command::commands;
commands! { Device, device;
	/// Retrieve a queue handle from this device.
	///
	/// # Safety
	/// `queue_family_index` and `queue_index` must be within the bounds of the
	/// queue families requested at device creation.
	pub unsafe fn get_device_queue(
		queue_family_index: u32,
		queue_index: u32,
	) -> Result<vk::Queue, CallError> {
		let f = device
			.fp
			.get_device_queue
			.ok_or(CallError::MissingCommand("vkGetDeviceQueue"))?;
		let mut queue = vk::Queue::NULL;
		// SAFETY: caller guarantees valid family and queue indices.
		unsafe { f(device.handle, queue_family_index, queue_index, &mut queue) };
		Ok(queue)
	}

	/// Resolve a device-specific function by name using this device's `gdpa`.
	///
	/// Returns `None` if the function is not available for this device.
	///
	/// # Safety
	/// `name` must be a valid NUL-terminated command name. The caller must cast
	/// and use the returned pointer only while this device is live.
	pub unsafe fn get_device_proc_addr(name: &std::ffi::CStr) -> vk::PFN_vkVoidFunction {
		// SAFETY: the caller keeps this device alive while using returned functions.
		unsafe { (device.gdpa)(device.handle, name.as_ptr()) }
	}

	/// Record a compute dispatch into a command buffer.
	///
	/// # Errors
	/// Returns `CallError::MissingCommand` if `vkCmdDispatch` was not loaded
	/// (extension not enabled or unsupported device).
	///
	/// # Safety
	/// `command_buffer` must be in the recording state. `group_count_*` must not
	/// exceed `VkPhysicalDeviceLimits::max_compute_work_group_count`.
	pub unsafe fn cmd_dispatch(
		command_buffer: vk::CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.cmd_dispatch
			.ok_or(CallError::MissingCommand("vkCmdDispatch"))?;
		// SAFETY: caller guarantees the command buffer is recording and group counts are valid.
		unsafe { f(command_buffer, group_count_x, group_count_y, group_count_z) };
		Ok(())
	}

	/// Allocate device memory.
	///
	/// # Errors
	/// `CallError::MissingCommand` if unavailable; `CallError::Vulkan` on failure.
	///
	/// # Safety
	/// The `allocate_info` must be valid and reference a legal memory type index.
	/// The allocation must be freed exactly once via [`Device::free_memory`].
	pub unsafe fn allocate_memory(
		allocate_info: &vk::MemoryAllocateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<vk::DeviceMemory, CallError> {
		let f = device
			.fp
			.allocate_memory
			.ok_or(CallError::MissingCommand("vkAllocateMemory"))?;
		let mut handle = vk::DeviceMemory::NULL;
		let result = unsafe {
			f(
				device.handle,
				allocate_info,
				allocator.map_or(std::ptr::null(), |a| a),
				&mut handle,
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(handle)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Free a previously allocated device memory object.
	///
	/// # Safety
	/// All resources bound to this memory must have been destroyed. The memory
	/// must not be freed more than once.
	pub unsafe fn free_memory(
		memory: vk::DeviceMemory,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.free_memory
			.ok_or(CallError::MissingCommand("vkFreeMemory"))?;
		unsafe {
			f(
				device.handle,
				memory,
				allocator.map_or(std::ptr::null(), |a| a),
			)
		};
		Ok(())
	}

	/// Map a range of a device memory object into host address space.
	///
	/// Returns a raw pointer to the start of the mapped range.
	///
	/// # Safety
	/// `memory` must be host-visible, not already mapped, and remain live until
	/// [`Device::unmap_memory`] is called.
	pub unsafe fn map_memory(
		memory: vk::DeviceMemory,
		offset: vk::DeviceSize,
		size: vk::DeviceSize,
		flags: vk::MemoryMapFlags,
	) -> Result<*mut std::ffi::c_void, CallError> {
		let f = device
			.fp
			.map_memory
			.ok_or(CallError::MissingCommand("vkMapMemory"))?;
		let mut ptr = std::ptr::null_mut();
		let result = unsafe { f(device.handle, memory, offset, size, flags, &mut ptr) };
		if result == vk::Result::SUCCESS {
			Ok(ptr)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Unmap a previously mapped memory object.
	///
	/// # Safety
	/// `memory` must currently be mapped and must not be used from host
	/// after this call.
	pub unsafe fn unmap_memory(memory: vk::DeviceMemory) -> Result<(), CallError> {
		let f = device
			.fp
			.unmap_memory
			.ok_or(CallError::MissingCommand("vkUnmapMemory"))?;
		unsafe { f(device.handle, memory) };
		Ok(())
	}

	/// Flush mapped memory ranges to make host writes visible to the device.
	///
	/// Required for non-host-coherent memory types before GPU reads.
	///
	/// # Safety
	/// All ranges must be within currently mapped memory objects.
	pub unsafe fn flush_mapped_memory_ranges(
		ranges: &[vk::MappedMemoryRange],
	) -> Result<(), CallError> {
		let f = device
			.fp
			.flush_mapped_memory_ranges
			.ok_or(CallError::MissingCommand("vkFlushMappedMemoryRanges"))?;
		let result = unsafe {
			f(
				device.handle,
				u32::try_from(ranges.len()).map_err(|_| CallError::CountOverflow(ranges.len()))?,
				ranges.as_ptr(),
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Invalidate mapped memory ranges to make device writes visible to the host.
	///
	/// Required for non-host-coherent memory types after GPU writes.
	///
	/// # Safety
	/// All ranges must be within currently mapped memory objects.
	pub unsafe fn invalidate_mapped_memory_ranges(
		ranges: &[vk::MappedMemoryRange],
	) -> Result<(), CallError> {
		let f = device
			.fp
			.invalidate_mapped_memory_ranges
			.ok_or(CallError::MissingCommand("vkInvalidateMappedMemoryRanges"))?;
		let result = unsafe {
			f(
				device.handle,
				u32::try_from(ranges.len()).map_err(|_| CallError::CountOverflow(ranges.len()))?,
				ranges.as_ptr(),
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Create a buffer object.
	///
	/// # Safety
	/// `create_info` must be valid. Memory must be bound before use.
	pub unsafe fn create_buffer(
		create_info: &vk::BufferCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<vk::Buffer, CallError> {
		let f = device
			.fp
			.create_buffer
			.ok_or(CallError::MissingCommand("vkCreateBuffer"))?;
		let mut handle = vk::Buffer::NULL;
		let result = unsafe {
			f(
				device.handle,
				create_info,
				allocator.map_or(std::ptr::null(), |a| a),
				&mut handle,
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(handle)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Destroy a buffer object.
	///
	/// # Safety
	/// The buffer must not be in use by the GPU and must not be destroyed twice.
	pub unsafe fn destroy_buffer(
		buffer: vk::Buffer,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.destroy_buffer
			.ok_or(CallError::MissingCommand("vkDestroyBuffer"))?;
		unsafe {
			f(
				device.handle,
				buffer,
				allocator.map_or(std::ptr::null(), |a| a),
			)
		};
		Ok(())
	}

	/// Query the memory requirements of a buffer (size, alignment, type bits).
	///
	/// # Safety
	/// `buffer` must be a live handle created from this device.
	pub unsafe fn get_buffer_memory_requirements(
		buffer: vk::Buffer,
	) -> Result<vk::MemoryRequirements, CallError> {
		let f = device
			.fp
			.get_buffer_memory_requirements
			.ok_or(CallError::MissingCommand("vkGetBufferMemoryRequirements"))?;
		let mut reqs = unsafe { std::mem::zeroed::<vk::MemoryRequirements>() };
		unsafe { f(device.handle, buffer, &mut reqs) };
		Ok(reqs)
	}

	/// Bind device memory to a buffer at the given offset.
	///
	/// # Safety
	/// `memory` must have been allocated with a compatible type and sufficient size.
	/// The buffer must not already have memory bound.
	pub unsafe fn bind_buffer_memory(
		buffer: vk::Buffer,
		memory: vk::DeviceMemory,
		memory_offset: vk::DeviceSize,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.bind_buffer_memory
			.ok_or(CallError::MissingCommand("vkBindBufferMemory"))?;
		let result = unsafe { f(device.handle, buffer, memory, memory_offset) };
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Create a command pool.
	///
	/// # Safety
	/// `create_info.queue_family_index` must be valid for this device.
	pub unsafe fn create_command_pool(
		create_info: &vk::CommandPoolCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<vk::CommandPool, CallError> {
		let f = device
			.fp
			.create_command_pool
			.ok_or(CallError::MissingCommand("vkCreateCommandPool"))?;
		let mut handle = vk::CommandPool::NULL;
		let result = unsafe {
			f(
				device.handle,
				create_info,
				allocator.map_or(std::ptr::null(), |a| a),
				&mut handle,
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(handle)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Destroy a command pool and all command buffers allocated from it.
	///
	/// # Safety
	/// All command buffers from this pool must have finished execution on the GPU.
	pub unsafe fn destroy_command_pool(
		pool: vk::CommandPool,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.destroy_command_pool
			.ok_or(CallError::MissingCommand("vkDestroyCommandPool"))?;
		unsafe {
			f(
				device.handle,
				pool,
				allocator.map_or(std::ptr::null(), |a| a),
			)
		};
		Ok(())
	}

	/// Allocate command buffers from a pool.
	///
	/// # Safety
	/// `allocate_info.command_pool` must be a live pool from this device.
	pub unsafe fn allocate_command_buffers(
		allocate_info: &vk::CommandBufferAllocateInfo,
	) -> Result<Vec<vk::CommandBuffer>, CallError> {
		let f = device
			.fp
			.allocate_command_buffers
			.ok_or(CallError::MissingCommand("vkAllocateCommandBuffers"))?;
		let count = allocate_info.command_buffer_count as usize;
		let mut bufs = vec![vk::CommandBuffer::NULL; count];
		let result = unsafe { f(device.handle, allocate_info, bufs.as_mut_ptr()) };
		if result == vk::Result::SUCCESS {
			Ok(bufs)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Free command buffers back to their pool.
	///
	/// # Safety
	/// All buffers must have finished execution on the GPU.
	pub unsafe fn free_command_buffers(
		pool: vk::CommandPool,
		buffers: &[vk::CommandBuffer],
	) -> Result<(), CallError> {
		let f = device
			.fp
			.free_command_buffers
			.ok_or(CallError::MissingCommand("vkFreeCommandBuffers"))?;
		unsafe {
			f(
				device.handle,
				pool,
				u32::try_from(buffers.len()).map_err(|_| CallError::CountOverflow(buffers.len()))?,
				buffers.as_ptr(),
			)
		};
		Ok(())
	}

	/// Begin recording a command buffer.
	///
	/// # Safety
	/// The command buffer must be in the initial or reset state.
	pub unsafe fn begin_command_buffer(
		command_buffer: vk::CommandBuffer,
		begin_info: &vk::CommandBufferBeginInfo,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.begin_command_buffer
			.ok_or(CallError::MissingCommand("vkBeginCommandBuffer"))?;
		let result = unsafe { f(command_buffer, begin_info) };
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// End recording a command buffer.
	///
	/// # Safety
	/// The command buffer must be in the recording state.
	pub unsafe fn end_command_buffer(command_buffer: vk::CommandBuffer) -> Result<(), CallError> {
		let f = device
			.fp
			.end_command_buffer
			.ok_or(CallError::MissingCommand("vkEndCommandBuffer"))?;
		let result = unsafe { f(command_buffer) };
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Fill a buffer region with a fixed 32-bit value.
	///
	/// # Safety
	/// `buffer` must be bound to device memory, in the executable/pending state,
	/// and `offset`/`size` must be 4-byte aligned.
	pub unsafe fn cmd_fill_buffer(
		command_buffer: vk::CommandBuffer,
		buffer: vk::Buffer,
		offset: vk::DeviceSize,
		size: vk::DeviceSize,
		data: u32,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.cmd_fill_buffer
			.ok_or(CallError::MissingCommand("vkCmdFillBuffer"))?;
		unsafe { f(command_buffer, buffer, offset, size, data) };
		Ok(())
	}

	/// Submit command buffers to a queue.
	///
	/// # Safety
	/// All resources referenced by the command buffers must be live and not
	/// mutated by the host until the fence signals.
	pub unsafe fn queue_submit(
		queue: vk::Queue,
		submits: &[vk::SubmitInfo],
		fence: vk::Fence,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.queue_submit
			.ok_or(CallError::MissingCommand("vkQueueSubmit"))?;
		let result = unsafe {
			f(
				queue,
				u32::try_from(submits.len()).map_err(|_| CallError::CountOverflow(submits.len()))?,
				submits.as_ptr(),
				fence,
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Wait for a queue to become idle (all submitted work complete).
	///
	/// # Safety
	/// `queue` must be a live handle from this device.
	pub unsafe fn queue_wait_idle(queue: vk::Queue) -> Result<(), CallError> {
		let f = device
			.fp
			.queue_wait_idle
			.ok_or(CallError::MissingCommand("vkQueueWaitIdle"))?;
		let result = unsafe { f(queue) };
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Create a fence.
	///
	/// # Safety
	/// `create_info` must be valid. The fence must be destroyed exactly once.
	pub unsafe fn create_fence(
		create_info: &vk::FenceCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<vk::Fence, CallError> {
		let f = device
			.fp
			.create_fence
			.ok_or(CallError::MissingCommand("vkCreateFence"))?;
		let mut handle = vk::Fence::NULL;
		let result = unsafe {
			f(
				device.handle,
				create_info,
				allocator.map_or(std::ptr::null(), |a| a),
				&mut handle,
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(handle)
		} else {
			Err(CallError::Vulkan(result))
		}
	}

	/// Destroy a fence.
	///
	/// # Safety
	/// The fence must not be pending or in use by the GPU.
	pub unsafe fn destroy_fence(
		fence: vk::Fence,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<(), CallError> {
		let f = device
			.fp
			.destroy_fence
			.ok_or(CallError::MissingCommand("vkDestroyFence"))?;
		unsafe {
			f(
				device.handle,
				fence,
				allocator.map_or(std::ptr::null(), |a| a),
			)
		};
		Ok(())
	}

	/// Block until one or all of the given fences signal, or the timeout expires.
	///
	/// Returns `Ok(true)` if all fences signalled within the timeout,
	/// `Ok(false)` if the timeout expired first.
	///
	/// # Safety
	/// All fences must be live handles from this device.
	pub unsafe fn wait_for_fences(
		fences: &[vk::Fence],
		wait_all: bool,
		timeout_ns: u64,
	) -> Result<bool, CallError> {
		let f = device
			.fp
			.wait_for_fences
			.ok_or(CallError::MissingCommand("vkWaitForFences"))?;
		let result = unsafe {
			f(
				device.handle,
				u32::try_from(fences.len()).map_err(|_| CallError::CountOverflow(fences.len()))?,
				fences.as_ptr(),
				wait_all as vk::Bool32,
				timeout_ns,
			)
		};
		match result {
			vk::Result::SUCCESS => Ok(true),
			vk::Result::TIMEOUT => Ok(false),
			other => Err(CallError::Vulkan(other)),
		}
	}

	/// Reset fences to the unsignalled state.
	///
	/// # Safety
	/// No fences in the slice may be pending (submitted but not yet signalled).
	pub unsafe fn reset_fences(fences: &[vk::Fence]) -> Result<(), CallError> {
		let f = device
			.fp
			.reset_fences
			.ok_or(CallError::MissingCommand("vkResetFences"))?;
		let result = unsafe {
			f(
				device.handle,
				u32::try_from(fences.len()).map_err(|_| CallError::CountOverflow(fences.len()))?,
				fences.as_ptr(),
			)
		};
		if result == vk::Result::SUCCESS {
			Ok(())
		} else {
			Err(CallError::Vulkan(result))
		}
	}

}

/// Create a logical device from a physical device.
///
/// # Errors
/// Reports preflight/metadata failures, driver errors, invalid native outputs
/// and unavailable destruction. If rollback is impossible,
/// `CallError::InitializationFailed` retains the device for explicit recovery.
///
/// # Safety
/// The create info, its reachable pointers and allocator callbacks must meet
/// Vulkan valid usage. Allocator state must remain alive until destruction.
pub unsafe fn create_device(
	instance: &Instance,
	physical_device: vk::PhysicalDevice,
	create_info: &vk::DeviceCreateInfo,
	allocator: Option<&vk::AllocationCallbacks>,
) -> Result<Device, CallError> {
	// Resolve indispensable context before making a native allocation.
	let gdpa = instance
		.fp()
		.get_device_proc_addr
		.ok_or(CallError::MissingCommand("vkGetDeviceProcAddr"))?;
	let capabilities = if let Some(parent) = instance.capabilities() {
		// A physical device supports at least 1.0. Higher requested versions must
		// be capped using this selected device, not the loader's version.
		let physical_version = if parent.api_version() == vk::API_VERSION_1_0 {
			vk::API_VERSION_1_0
		} else {
			// SAFETY: caller guarantees this live physical device belongs to instance.
			unsafe { instance.get_physical_device_properties(physical_device) }?.api_version
		};
		// SAFETY: caller guarantees validity of the device create-info graph.
		Some(unsafe { parent.new_from_device_create_info(create_info, physical_version) }?)
	} else {
		None
	};
	let f = instance
		.fp()
		.create_device
		.ok_or(CallError::MissingCommand("vkCreateDevice"))?;
	let allocator_ptr = allocator.map_or(std::ptr::null(), |a| a);
	let mut handle = vk::DeviceHandle::NULL;
	// SAFETY: caller guarantees create-info validity; PFN is from a live instance.
	let result = unsafe { f(physical_device, create_info, allocator_ptr, &mut handle) };
	if result != vk::Result::SUCCESS {
		return Err(CallError::Vulkan(result));
	}
	if handle.is_null() {
		return Err(CallError::InvalidOutput(
			"vkCreateDevice returned a null handle",
		));
	}
	// SAFETY: successful creation returned a live non-null handle for gdpa.
	let mut fp = unsafe { vk::FnDevice::new_from_resolver(gdpa, handle) };
	if fp.destroy_device.is_none() {
		let failure = InitializationFailure {
			handle: CreatedHandle::Device(handle),
			missing_command: "vkDestroyDevice",
			library: instance.library.clone(),
			instance: instance.handle(),
			gipa: instance.gipa,
			gdpa: Some(gdpa),
		};
		// SAFETY: the new device has no children/work; use its original callbacks.
		return match unsafe { failure.destroy(allocator) } {
			Ok(()) => Err(CallError::MissingCommand("vkDestroyDevice")),
			Err(failure) => Err(CallError::InitializationFailed(failure)),
		};
	}
	if let Some(capabilities) = &capabilities {
		fp.retain_enabled(|command| capabilities.support(command) == CommandSupport::Enabled);
	}
	Ok(Device {
		handle,
		fp,
		gdpa,
		_library: instance.library.clone(),
		capabilities,
	})
}

impl Device {
	/// Create a logical device through its parent instance's cached dispatch.
	/// # Safety
	/// Device create info and callbacks must satisfy Vulkan valid usage and remain live as required.
	pub unsafe fn new(
		instance: &Instance,
		physical_device: vk::PhysicalDevice,
		create_info: &vk::DeviceCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<Self, CallError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { create_device(instance, physical_device, create_info, allocator) }
	}
}

impl Instance {
	/// Create a logical device through this instance's cached dispatch.
	/// # Safety
	/// Device create info and callbacks must satisfy Vulkan valid usage and remain live as required.
	pub unsafe fn create_device(
		&self,
		physical_device: vk::PhysicalDevice,
		create_info: &vk::DeviceCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<Device, CallError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { create_device(self, physical_device, create_info, allocator) }
	}
}

/// Destroy once, using callbacks compatible with device creation.
///
/// # Safety
/// All children and outstanding device operations must be complete. The
/// allocator must match creation, with its user data and functions still live.
pub unsafe fn destroy_device(device: Device, allocator: Option<&vk::AllocationCallbacks>) {
	// SAFETY: caller guarantees all destruction and allocator requirements.
	unsafe {
		device.fp.destroy_device.unwrap()(device.handle, allocator.map_or(std::ptr::null(), |a| a))
	};
}
