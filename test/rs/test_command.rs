//! Independent exact-ABI mocks for native operation semantics.
use ::vk::{self, CallError};
use std::{
	cell::{Cell, RefCell},
	ffi::{CStr, c_char, c_void},
};

pub(super) struct Fixture {
	pub instance: vk::Instance,
	pub device: vk::Device,
}

thread_local! {
	static OVERRIDE: Cell<Option<(&'static std::ffi::CStr, vk::PFN_vkVoidFunction)>> = const { Cell::new(None) };
	static WITH_COMMANDS: Cell<bool> = const { Cell::new(false) };
	static QUERIES: Cell<usize> = const { Cell::new(0) };
	static CALLS: Cell<usize> = const { Cell::new(0) };
	static STATUS: Cell<vk::Result> = const { Cell::new(vk::Result::SUCCESS) };
	static TRACE: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
	static CACHE_PASS: Cell<u32> = const { Cell::new(0) };
	static BLOCKED: Cell<Option<&'static [u8]>> = const { Cell::new(None) };
	static SCALAR: Cell<u64> = const { Cell::new(0) };
	static BAD_COUNT: Cell<bool> = const { Cell::new(false) };
}

impl Fixture {
	pub fn new() -> Self {
		Self::new_with_commands(false)
	}

	pub(super) fn new_with_command(name: &'static CStr, function: vk::PFN_vkVoidFunction) -> Self {
		OVERRIDE.set(Some((name, function)));
		let fixture = Self::new();
		OVERRIDE.set(None);
		fixture
	}

	pub(super) fn query_count() -> usize {
		QUERIES.get()
	}

	fn new_with_commands(enabled: bool) -> Self {
		WITH_COMMANDS.set(enabled);
		STATUS.set(vk::Result::SUCCESS);
		TRACE.with_borrow_mut(Vec::clear);
		CACHE_PASS.set(0);
		BAD_COUNT.set(false);
		SCALAR.set(0);
		// SAFETY: all resolvers are static exact-ABI mocks; imported tokens have
		// no native objects, and the wrappers' Drop performs no native destruction.
		let fixture = unsafe {
			let entry = vk::Entry::new_from_resolver(gipa).unwrap();
			let instance = vk::Instance::new_from_handle(
				&entry,
				vk::InstanceHandle(std::ptr::without_provenance_mut::<c_void>(0x1000)),
			)
			.unwrap();
			let device = vk::Device::new_from_handle(
				&instance,
				vk::DeviceHandle(std::ptr::without_provenance_mut::<c_void>(0x2000)),
			)
			.unwrap();
			Self { instance, device }
		};
		WITH_COMMANDS.set(false);
		fixture
	}
}

unsafe extern "system" fn create_instance(
	_: *const vk::InstanceCreateInfo,
	_: *const vk::AllocationCallbacks,
	_: *mut vk::InstanceHandle,
) -> vk::Result {
	vk::Result::ERROR_INITIALIZATION_FAILED
}

unsafe extern "system" fn extensions(
	_: *const c_char,
	_: *mut u32,
	_: *mut vk::ExtensionProperties,
) -> vk::Result {
	vk::Result::SUCCESS
}

unsafe extern "system" fn layers(_: *mut u32, _: *mut vk::LayerProperties) -> vk::Result {
	vk::Result::SUCCESS
}

unsafe extern "system" fn destroy_instance(
	_: vk::InstanceHandle,
	_: *const vk::AllocationCallbacks,
) {
}
unsafe extern "system" fn destroy_device(_: vk::DeviceHandle, _: *const vk::AllocationCallbacks) {}

unsafe extern "system" fn gipa(
	_: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	QUERIES.set(QUERIES.get() + 1);
	// SAFETY: table construction supplies a static NUL-terminated command name;
	// each erased pointer is restored only to that command's exact native signature.
	unsafe {
		if let Some((expected, function)) = OVERRIDE.get()
			&& CStr::from_ptr(name) == expected
		{
			return function;
		}
		match CStr::from_ptr(name).to_bytes() {
			b"vkCreateInstance" => Some(std::mem::transmute::<
				vk::PFN_vkCreateInstance,
				unsafe extern "system" fn(),
			>(Some(create_instance))),
			b"vkEnumerateInstanceExtensionProperties" => Some(std::mem::transmute::<
				vk::PFN_vkEnumerateInstanceExtensionProperties,
				unsafe extern "system" fn(),
			>(Some(extensions))),
			b"vkEnumerateInstanceLayerProperties" => Some(std::mem::transmute::<
				vk::PFN_vkEnumerateInstanceLayerProperties,
				unsafe extern "system" fn(),
			>(Some(layers))),
			b"vkDestroyInstance" => Some(std::mem::transmute::<
				vk::PFN_vkDestroyInstance,
				unsafe extern "system" fn(),
			>(Some(destroy_instance))),
			b"vkGetDeviceProcAddr" => Some(std::mem::transmute::<
				vk::PFN_vkGetDeviceProcAddr,
				unsafe extern "system" fn(),
			>(Some(gdpa))),
			b"vkGetPhysicalDeviceFeatures2" if WITH_COMMANDS.get() => Some(std::mem::transmute::<
				vk::PFN_vkGetPhysicalDeviceFeatures2,
				unsafe extern "system" fn(),
			>(Some(features))),
			b"vkEnumeratePhysicalDeviceGroups" if WITH_COMMANDS.get() => Some(std::mem::transmute::<
				vk::PFN_vkEnumeratePhysicalDeviceGroups,
				unsafe extern "system" fn(),
			>(Some(groups))),
			b"vkEnumeratePhysicalDeviceGroupsKHR" if WITH_COMMANDS.get() => Some(std::mem::transmute::<
				vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR,
				unsafe extern "system" fn(),
			>(Some(groups))),
			b"vkGetPhysicalDeviceQueueFamilyProperties2" if WITH_COMMANDS.get() => {
				Some(std::mem::transmute::<
					vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
					unsafe extern "system" fn(),
				>(Some(families2)))
			}
			b"vkGetPhysicalDeviceQueueFamilyProperties2KHR" if WITH_COMMANDS.get() => {
				Some(std::mem::transmute::<
					vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,
					unsafe extern "system" fn(),
				>(Some(families2)))
			}
			b"vkGetPhysicalDeviceSparseImageFormatProperties2" if WITH_COMMANDS.get() => {
				Some(std::mem::transmute::<
					vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
					unsafe extern "system" fn(),
				>(Some(sparse_formats2)))
			}
			b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR" if WITH_COMMANDS.get() => {
				Some(std::mem::transmute::<
					vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
					unsafe extern "system" fn(),
				>(Some(sparse_formats2)))
			}
			b"vkGetPhysicalDeviceImageFormatProperties2" if WITH_COMMANDS.get() => {
				Some(std::mem::transmute::<
					vk::PFN_vkGetPhysicalDeviceImageFormatProperties2,
					unsafe extern "system" fn(),
				>(Some(image_format2)))
			}
			_ => None,
		}
	}
}

unsafe extern "system" fn gdpa(_: vk::DeviceHandle, name: *const c_char) -> vk::PFN_vkVoidFunction {
	QUERIES.set(QUERIES.get() + 1);
	// SAFETY: table construction supplies a static command name and restores
	// each erased function only to the corresponding exact native prototype.
	unsafe {
		if let Some((expected, function)) = OVERRIDE.get()
			&& CStr::from_ptr(name) == expected
		{
			return function;
		}
		match CStr::from_ptr(name).to_bytes() {
			b"vkDestroyDevice" => Some(std::mem::transmute::<
				vk::PFN_vkDestroyDevice,
				unsafe extern "system" fn(),
			>(Some(destroy_device))),
			name if BLOCKED.get() == Some(name) => None,
			_ if !WITH_COMMANDS.get() => None,
			b"vkWaitSemaphores" => Some(std::mem::transmute::<
				vk::PFN_vkWaitSemaphores,
				unsafe extern "system" fn(),
			>(Some(wait_semaphores))),
			b"vkCmdWaitEvents2" => Some(std::mem::transmute::<
				vk::PFN_vkCmdWaitEvents2,
				unsafe extern "system" fn(),
			>(Some(wait_events2))),
			b"vkQueueSubmit2" => Some(std::mem::transmute::<
				vk::PFN_vkQueueSubmit2,
				unsafe extern "system" fn(),
			>(Some(submit2))),
			b"vkCreateShaderModule" => Some(std::mem::transmute::<
				vk::PFN_vkCreateShaderModule,
				unsafe extern "system" fn(),
			>(Some(create_shader))),
			b"vkCreateComputePipelines" => Some(std::mem::transmute::<
				vk::PFN_vkCreateComputePipelines,
				unsafe extern "system" fn(),
			>(Some(create_pipelines))),
			b"vkCreateGraphicsPipelines" => Some(std::mem::transmute::<
				vk::PFN_vkCreateGraphicsPipelines,
				unsafe extern "system" fn(),
			>(Some(create_graphics))),
			b"vkCmdBindVertexBuffers" => Some(std::mem::transmute::<
				vk::PFN_vkCmdBindVertexBuffers,
				unsafe extern "system" fn(),
			>(Some(bind_vertices))),
			b"vkCmdSetBlendConstants" => Some(std::mem::transmute::<
				vk::PFN_vkCmdSetBlendConstants,
				unsafe extern "system" fn(),
			>(Some(blend_constants))),
			b"vkCmdPushConstants" => Some(std::mem::transmute::<
				vk::PFN_vkCmdPushConstants,
				unsafe extern "system" fn(),
			>(Some(push_bytes))),
			b"vkCmdUpdateBuffer" => Some(std::mem::transmute::<
				vk::PFN_vkCmdUpdateBuffer,
				unsafe extern "system" fn(),
			>(Some(update_bytes))),
			b"vkGetQueryPoolResults" => Some(std::mem::transmute::<
				vk::PFN_vkGetQueryPoolResults,
				unsafe extern "system" fn(),
			>(Some(query_bytes))),
			b"vkGetPipelineCacheData" => Some(std::mem::transmute::<
				vk::PFN_vkGetPipelineCacheData,
				unsafe extern "system" fn(),
			>(Some(cache_bytes))),
			b"vkGetImageSparseMemoryRequirements" => Some(std::mem::transmute::<
				vk::PFN_vkGetImageSparseMemoryRequirements,
				unsafe extern "system" fn(),
			>(Some(sparse_requirements))),
			b"vkCmdDrawIndexed" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndexed,
				unsafe extern "system" fn(),
			>(Some(draw_indexed))),
			b"vkGetEventStatus" => Some(std::mem::transmute::<
				vk::PFN_vkGetEventStatus,
				unsafe extern "system" fn(),
			>(Some(event_status))),
			b"vkGetFenceStatus" => Some(std::mem::transmute::<
				vk::PFN_vkGetFenceStatus,
				unsafe extern "system" fn(),
			>(Some(fence_status))),
			b"vkAllocateDescriptorSets" => Some(std::mem::transmute::<
				vk::PFN_vkAllocateDescriptorSets,
				unsafe extern "system" fn(),
			>(Some(allocate_sets))),
			b"vkCmdBindDescriptorSets" => Some(std::mem::transmute::<
				vk::PFN_vkCmdBindDescriptorSets,
				unsafe extern "system" fn(),
			>(Some(bind_sets))),
			b"vkCmdPipelineBarrier" => Some(std::mem::transmute::<
				vk::PFN_vkCmdPipelineBarrier,
				unsafe extern "system" fn(),
			>(Some(barrier))),
			b"vkGetBufferMemoryRequirements2" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferMemoryRequirements2,
				unsafe extern "system" fn(),
			>(Some(requirements))),
			b"vkGetImageSparseMemoryRequirements2" => Some(std::mem::transmute::<
				vk::PFN_vkGetImageSparseMemoryRequirements2,
				unsafe extern "system" fn(),
			>(Some(sparse_requirements2))),
			b"vkGetImageSparseMemoryRequirements2KHR" => Some(std::mem::transmute::<
				vk::PFN_vkGetImageSparseMemoryRequirements2KHR,
				unsafe extern "system" fn(),
			>(Some(sparse_requirements2))),
			b"vkCreateDescriptorUpdateTemplate" => Some(std::mem::transmute::<
				vk::PFN_vkCreateDescriptorUpdateTemplate,
				unsafe extern "system" fn(),
			>(Some(create_template))),
			b"vkCreateDescriptorUpdateTemplateKHR" => Some(std::mem::transmute::<
				vk::PFN_vkCreateDescriptorUpdateTemplateKHR,
				unsafe extern "system" fn(),
			>(Some(create_template))),
			b"vkUpdateDescriptorSetWithTemplate" => Some(std::mem::transmute::<
				vk::PFN_vkUpdateDescriptorSetWithTemplate,
				unsafe extern "system" fn(),
			>(Some(template_payload))),
			b"vkUpdateDescriptorSetWithTemplateKHR" => Some(std::mem::transmute::<
				vk::PFN_vkUpdateDescriptorSetWithTemplateKHR,
				unsafe extern "system" fn(),
			>(Some(template_payload))),
			b"vkDestroyDescriptorUpdateTemplate" => Some(std::mem::transmute::<
				vk::PFN_vkDestroyDescriptorUpdateTemplate,
				unsafe extern "system" fn(),
			>(Some(destroy_template))),
			b"vkCreateSamplerYcbcrConversion" => Some(std::mem::transmute::<
				vk::PFN_vkCreateSamplerYcbcrConversion,
				unsafe extern "system" fn(),
			>(Some(create_conversion))),
			b"vkDestroySamplerYcbcrConversion" => Some(std::mem::transmute::<
				vk::PFN_vkDestroySamplerYcbcrConversion,
				unsafe extern "system" fn(),
			>(Some(destroy_conversion))),
			b"vkGetDeviceQueue2" => Some(std::mem::transmute::<
				vk::PFN_vkGetDeviceQueue2,
				unsafe extern "system" fn(),
			>(Some(queue2))),
			b"vkCmdDispatchBase" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDispatchBase,
				unsafe extern "system" fn(),
			>(Some(dispatch_base))),
			b"vkCmdSetDeviceMask" => Some(std::mem::transmute::<
				vk::PFN_vkCmdSetDeviceMask,
				unsafe extern "system" fn(),
			>(Some(device_mask))),
			b"vkGetDeviceGroupPeerMemoryFeatures" => Some(std::mem::transmute::<
				vk::PFN_vkGetDeviceGroupPeerMemoryFeatures,
				unsafe extern "system" fn(),
			>(Some(peer_features))),
			b"vkGetBufferDeviceAddress" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferDeviceAddress,
				unsafe extern "system" fn(),
			>(Some(buffer_address))),
			b"vkGetBufferDeviceAddressKHR" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferDeviceAddressKHR,
				unsafe extern "system" fn(),
			>(Some(buffer_address))),
			b"vkGetBufferDeviceAddressEXT" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferDeviceAddressEXT,
				unsafe extern "system" fn(),
			>(Some(buffer_address))),
			b"vkGetBufferOpaqueCaptureAddress" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferOpaqueCaptureAddress,
				unsafe extern "system" fn(),
			>(Some(buffer_address))),
			b"vkGetBufferOpaqueCaptureAddressKHR" => Some(std::mem::transmute::<
				vk::PFN_vkGetBufferOpaqueCaptureAddressKHR,
				unsafe extern "system" fn(),
			>(Some(buffer_address))),
			b"vkGetDeviceMemoryOpaqueCaptureAddress" => Some(std::mem::transmute::<
				vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
				unsafe extern "system" fn(),
			>(Some(memory_address))),
			b"vkGetDeviceMemoryOpaqueCaptureAddressKHR" => Some(std::mem::transmute::<
				vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR,
				unsafe extern "system" fn(),
			>(Some(memory_address))),
			b"vkResetQueryPool" => Some(std::mem::transmute::<
				vk::PFN_vkResetQueryPool,
				unsafe extern "system" fn(),
			>(Some(reset_queries))),
			b"vkResetQueryPoolEXT" => Some(std::mem::transmute::<
				vk::PFN_vkResetQueryPoolEXT,
				unsafe extern "system" fn(),
			>(Some(reset_queries))),
			b"vkCmdDrawIndirectCount" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndirectCount,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCmdDrawIndirectCountKHR" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndirectCountKHR,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCmdDrawIndirectCountAMD" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndirectCountAMD,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCmdDrawIndexedIndirectCount" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndexedIndirectCount,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCmdDrawIndexedIndirectCountKHR" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndexedIndirectCountKHR,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCmdDrawIndexedIndirectCountAMD" => Some(std::mem::transmute::<
				vk::PFN_vkCmdDrawIndexedIndirectCountAMD,
				unsafe extern "system" fn(),
			>(Some(indirect_count))),
			b"vkCreateRenderPass2" => Some(std::mem::transmute::<
				vk::PFN_vkCreateRenderPass2,
				unsafe extern "system" fn(),
			>(Some(create_render_pass2))),
			b"vkCreateRenderPass2KHR" => Some(std::mem::transmute::<
				vk::PFN_vkCreateRenderPass2KHR,
				unsafe extern "system" fn(),
			>(Some(create_render_pass2))),
			b"vkCmdBeginRenderPass2" => Some(std::mem::transmute::<
				vk::PFN_vkCmdBeginRenderPass2,
				unsafe extern "system" fn(),
			>(Some(begin_pass2))),
			b"vkCmdBeginRenderPass2KHR" => Some(std::mem::transmute::<
				vk::PFN_vkCmdBeginRenderPass2KHR,
				unsafe extern "system" fn(),
			>(Some(begin_pass2))),
			b"vkCmdNextSubpass2" => Some(std::mem::transmute::<
				vk::PFN_vkCmdNextSubpass2,
				unsafe extern "system" fn(),
			>(Some(next_subpass2))),
			b"vkCmdNextSubpass2KHR" => Some(std::mem::transmute::<
				vk::PFN_vkCmdNextSubpass2KHR,
				unsafe extern "system" fn(),
			>(Some(next_subpass2))),
			b"vkCmdEndRenderPass2" => Some(std::mem::transmute::<
				vk::PFN_vkCmdEndRenderPass2,
				unsafe extern "system" fn(),
			>(Some(end_pass2))),
			b"vkCmdEndRenderPass2KHR" => Some(std::mem::transmute::<
				vk::PFN_vkCmdEndRenderPass2KHR,
				unsafe extern "system" fn(),
			>(Some(end_pass2))),
			_ => None,
		}
	}
}

unsafe extern "system" fn create_shader(
	device: vk::DeviceHandle,
	info: *const vk::ShaderModuleCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::ShaderModule,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the public operation supplies initialized input and writable output.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				(*info).code_size as u64,
				callbacks.is_null() as u64,
			])
		});
		*output = vk::ShaderModule(0x3000);
	}
	STATUS.get()
}

unsafe extern "system" fn create_pipelines(
	_: vk::DeviceHandle,
	_: vk::PipelineCache,
	count: u32,
	_: *const vk::ComputePipelineCreateInfo,
	_: *const vk::AllocationCallbacks,
	outputs: *mut vk::Pipeline,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the operation checked that outputs has exactly count writable entries.
	unsafe {
		for i in 0..count as usize {
			*outputs.add(i) = if i == 0 {
				vk::Pipeline(0x4000)
			} else {
				vk::Pipeline::NULL
			};
		}
	}
	STATUS.get()
}

unsafe extern "system" fn allocate_sets(
	_: vk::DeviceHandle,
	info: *const vk::DescriptorSetAllocateInfo,
	outputs: *mut vk::DescriptorSet,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the operation verifies output length against the info's native count.
	unsafe {
		for i in 0..(*info).descriptor_set_count as usize {
			*outputs.add(i) = vk::DescriptorSet(0x5000 + i as u64);
		}
	}
	STATUS.get()
}

unsafe extern "system" fn create_graphics(
	device: vk::DeviceHandle,
	cache: vk::PipelineCache,
	count: u32,
	infos: *const vk::GraphicsPipelineCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	outputs: *mut vk::Pipeline,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the wrapper checks the writable output extent against the infos slice.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				cache.0,
				count as u64,
				(*infos).stage_count as u64,
				callbacks.is_null() as u64,
			])
		});
		for index in 0..count as usize {
			*outputs.add(index) = if index == 0 {
				vk::Pipeline(0x4400)
			} else {
				vk::Pipeline::NULL
			};
		}
	}
	STATUS.get()
}

unsafe extern "system" fn bind_vertices(
	command: vk::CommandBuffer,
	first: u32,
	count: u32,
	buffers: *const vk::Buffer,
	offsets: *const vk::DeviceSize,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the wrapper passes equally sized borrowed slices and their count.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([command.0 as usize as u64, first as u64, count as u64]);
			trace.extend(
				std::slice::from_raw_parts(buffers, count as usize)
					.iter()
					.map(|value| value.0),
			);
			trace.extend(std::slice::from_raw_parts(offsets, count as usize));
		});
	}
}

unsafe extern "system" fn blend_constants(command: vk::CommandBuffer, constants: *const f32) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the public operation requires a complete four-element array.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.push(command.0 as usize as u64);
			trace.extend(
				std::slice::from_raw_parts(constants, 4)
					.iter()
					.map(|value| value.to_bits() as u64),
			);
		});
	}
}

unsafe extern "system" fn push_bytes(
	_: vk::CommandBuffer,
	layout: vk::PipelineLayout,
	stages: vk::ShaderStageFlags,
	offset: u32,
	size: u32,
	data: *const c_void,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the wrapper derives the byte count from the initialized input slice.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([layout.0, stages.as_raw() as u64, offset as u64, size as u64]);
			trace.extend(
				std::slice::from_raw_parts(data.cast::<u8>(), size as usize)
					.iter()
					.map(|&byte| u64::from(byte)),
			);
		});
	}
}

unsafe extern "system" fn update_bytes(
	_: vk::CommandBuffer,
	buffer: vk::Buffer,
	offset: vk::DeviceSize,
	size: vk::DeviceSize,
	data: *const c_void,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the test's payload fits usize and the wrapper supplies its full byte extent.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([buffer.0, offset, size]);
			trace.extend(
				std::slice::from_raw_parts(data.cast::<u8>(), size as usize)
					.iter()
					.map(|&byte| u64::from(byte)),
			);
		});
	}
}

unsafe extern "system" fn query_bytes(
	device: vk::DeviceHandle,
	pool: vk::QueryPool,
	first: u32,
	count: u32,
	size: usize,
	data: *mut c_void,
	stride: vk::DeviceSize,
	flags: vk::QueryResultFlags,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			device.0 as usize as u64,
			pool.0,
			first as u64,
			count as u64,
			size as u64,
			stride,
			flags.as_raw() as u64,
		])
	});
	// SAFETY: the wrapper supplies size writable bytes; poison outside that range is untouched.
	unsafe { std::ptr::write_bytes(data.cast::<u8>(), 0xa5, size) };
	STATUS.get()
}

unsafe extern "system" fn cache_bytes(
	device: vk::DeviceHandle,
	cache: vk::PipelineCache,
	size: *mut usize,
	data: *mut c_void,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| trace.extend([device.0 as usize as u64, cache.0]));
	// SAFETY: each successful data query fills at most the supplied capacity.
	unsafe {
		if data.is_null() {
			*size = if CACHE_PASS.get() == 0 { 1 } else { 3 };
			return vk::Result::SUCCESS;
		}
		if CACHE_PASS.replace(1) == 0 {
			*data.cast::<u8>() = 9;
			return vk::Result::INCOMPLETE;
		}
		std::ptr::copy_nonoverlapping([0x80_u8, 0, 0xff].as_ptr(), data.cast(), 3);
		*size = 3;
	}
	STATUS.get()
}

unsafe extern "system" fn sparse_requirements(
	device: vk::DeviceHandle,
	image: vk::Image,
	count: *mut u32,
	data: *mut vk::SparseImageMemoryRequirements,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| trace.extend([device.0 as usize as u64, image.0]));
	// SAFETY: count-only queries report two records; data queries initialize both.
	unsafe {
		*count = 2;
		if !data.is_null() {
			for index in 0..2 {
				*data.add(index) = vk::SparseImageMemoryRequirements {
					format_properties: vk::SparseImageFormatProperties {
						aspect_mask: vk::ImageAspectFlags::COLOR_BIT,
						image_granularity: vk::Extent3D {
							width: 3,
							height: 5,
							depth: 7,
						},
						flags: vk::SparseImageFormatFlags::default(),
					},
					image_mip_tail_first_lod: index as u32,
					image_mip_tail_size: 0x100000001 + index as u64,
					image_mip_tail_offset: 64,
					image_mip_tail_stride: 128,
				};
			}
		}
	}
}

unsafe extern "system" fn draw_indexed(
	command: vk::CommandBuffer,
	indices: u32,
	instances: u32,
	first: u32,
	vertex_offset: i32,
	first_instance: u32,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			command.0 as usize as u64,
			indices as u64,
			instances as u64,
			first as u64,
			vertex_offset as i64 as u64,
			first_instance as u64,
		])
	});
}

unsafe extern "system" fn event_status(_: vk::DeviceHandle, _: vk::Event) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	STATUS.get()
}

unsafe extern "system" fn fence_status(_: vk::DeviceHandle, _: vk::Fence) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	STATUS.get()
}

#[test]
fn graphics_partial_outputs_and_statuses_remain_caller_owned() {
	let fixture = Fixture::new_with_commands(true);
	let infos = std::array::from_fn::<_, 2, _>(|_| vk::GraphicsPipelineCreateInfo {
		stage_count: 3,
		..Default::default()
	});
	let mut outputs = [vk::Pipeline::NULL; 2];
	let queries = QUERIES.get();
	let calls = CALLS.get();
	// SAFETY: the exact-ABI mock inspects one info and writes checked output slots only.
	unsafe {
		assert!(matches!(
			fixture.device.create_graphics_pipelines(
				vk::PipelineCache(19),
				&infos,
				None,
				&mut outputs[..1]
			),
			Err(CallError::SliceLengthMismatch { .. })
		));
		assert_eq!(CALLS.get(), calls);
		STATUS.set(vk::Result::PIPELINE_COMPILE_REQUIRED_EXT);
		assert_eq!(
			vk::create_graphics_pipelines(
				&fixture.device,
				vk::PipelineCache(19),
				&infos,
				None,
				&mut outputs
			),
			Ok(vk::Result::PIPELINE_COMPILE_REQUIRED_EXT)
		);
		assert_eq!(outputs, [vk::Pipeline(0x4400), vk::Pipeline::NULL]);
		STATUS.set(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
		assert_eq!(
			fixture
				.device
				.create_graphics_pipelines(vk::PipelineCache(19), &infos, None, &mut outputs),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
		);
	}
	assert_eq!(outputs, [vk::Pipeline(0x4400), vk::Pipeline::NULL]);
	assert_eq!(QUERIES.get(), queries);
	TRACE.with_borrow(|trace| assert_eq!(trace, &[0x2000, 19, 2, 3, 1, 0x2000, 19, 2, 3, 1]));
}

#[test]
fn graphics_parallel_arrays_fixed_arrays_and_signed_scalars_are_forwarded() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let calls = CALLS.get();
	let command = vk::CommandBuffer(std::ptr::without_provenance_mut(0x6000));
	let buffers = [vk::Buffer(17), vk::Buffer(29)];
	let offsets = [0x100000001, u64::MAX];
	let blend = [0.0_f32, -0.0, 0.25, 1.0];
	// SAFETY: mocks consume only initialized slices/arrays and record scalar bits.
	unsafe {
		assert!(matches!(
			fixture
				.device
				.cmd_bind_vertex_buffers(command, 3, &buffers, &offsets[..1]),
			Err(CallError::SliceLengthMismatch { .. })
		));
		assert_eq!(CALLS.get(), calls);
		vk::cmd_bind_vertex_buffers(&fixture.device, command, 3, &buffers, &offsets).unwrap();
		fixture
			.device
			.cmd_bind_vertex_buffers(command, 7, &[], &[])
			.unwrap();
		fixture
			.device
			.cmd_set_blend_constants(command, &blend)
			.unwrap();
		fixture
			.device
			.cmd_draw_indexed(command, u32::MAX, 2, 19, i32::MIN, 23)
			.unwrap();
	}
	let mut expected = vec![
		0x6000, 3, 2, 17, 29, offsets[0], offsets[1], 0x6000, 7, 0, 0x6000,
	];
	expected.extend(blend.map(|value| value.to_bits() as u64));
	expected.extend([0x6000, u32::MAX as u64, 2, 19, i32::MIN as i64 as u64, 23]);
	TRACE.with_borrow(|trace| assert_eq!(*trace, expected));
	assert_eq!(QUERIES.get(), queries);
}

#[test]
fn byte_operations_derive_u32_u64_and_size_t_counts_and_preserve_status() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let command = vk::CommandBuffer(std::ptr::without_provenance_mut(0x6000));
	let payload = [0_u8, 0xff, 0x80, 7];
	let mut output = [0x77; 16];
	// SAFETY: byte-only mocks inspect/write exactly the passed slices; no native resources exist.
	unsafe {
		fixture
			.device
			.cmd_push_constants(
				command,
				vk::PipelineLayout(17),
				vk::ShaderStageFlags::VERTEX_BIT,
				12,
				&payload,
			)
			.unwrap();
		fixture
			.device
			.cmd_update_buffer(command, vk::Buffer(29), 0x100000001, &payload)
			.unwrap();
		STATUS.set(vk::Result::NOT_READY);
		assert_eq!(
			fixture.device.get_query_pool_results(
				vk::QueryPool(31),
				3,
				2,
				&mut output[4..12],
				8,
				vk::QueryResultFlags::_64_BIT
			),
			Ok(vk::Result::NOT_READY)
		);
		STATUS.set(vk::Result::ERROR_DEVICE_LOST);
		assert_eq!(
			fixture.device.get_query_pool_results(
				vk::QueryPool(31),
				3,
				2,
				&mut output[4..12],
				8,
				vk::QueryResultFlags::_64_BIT
			),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
	}
	assert_eq!(&output[..4], &[0x77; 4]);
	assert_eq!(&output[4..12], &[0xa5; 8]);
	assert_eq!(&output[12..], &[0x77; 4]);
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace,
			&[
				17,
				1,
				12,
				4,
				0,
				255,
				128,
				7,
				29,
				0x100000001,
				4,
				0,
				255,
				128,
				7,
				0x2000,
				31,
				3,
				2,
				8,
				8,
				1,
				0x2000,
				31,
				3,
				2,
				8,
				8,
				1
			]
		)
	});
	assert_eq!(QUERIES.get(), queries);
}

#[test]
fn generated_enumerations_retry_and_preserve_record_fields() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	// SAFETY: the exact-ABI mocks implement the count/data protocol without native resources.
	unsafe {
		assert_eq!(
			fixture
				.device
				.get_pipeline_cache_data(vk::PipelineCache(17)),
			Ok(vec![0x80, 0, 0xff])
		);
		STATUS.set(vk::Result::ERROR_DEVICE_LOST);
		assert_eq!(
			vk::get_pipeline_cache_data(&fixture.device, vk::PipelineCache(17)),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
		let records = fixture
			.device
			.get_image_sparse_memory_requirements(vk::Image(29))
			.unwrap();
		assert_eq!(records.len(), 2);
		for (index, record) in records.iter().enumerate() {
			assert_eq!(record.format_properties.image_granularity.height, 5);
			assert_eq!(record.image_mip_tail_size, 0x100000001 + index as u64);
			assert_eq!(record.image_mip_tail_first_lod, index as u32);
		}
	}
	assert_eq!(QUERIES.get(), queries);
}

#[test]
fn event_and_fence_queries_preserve_non_error_status_codes() {
	let fixture = Fixture::new_with_commands(true);
	// SAFETY: mocks return the selected native status without touching native objects.
	unsafe {
		for status in [vk::Result::EVENT_SET, vk::Result::EVENT_RESET] {
			STATUS.set(status);
			assert_eq!(fixture.device.get_event_status(vk::Event(17)), Ok(status));
		}
		for status in [vk::Result::SUCCESS, vk::Result::NOT_READY] {
			STATUS.set(status);
			assert_eq!(fixture.device.get_fence_status(vk::Fence(29)), Ok(status));
		}
		STATUS.set(vk::Result::ERROR_DEVICE_LOST);
		assert_eq!(
			fixture.device.get_fence_status(vk::Fence(29)),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
	}
}

unsafe extern "system" fn bind_sets(
	command: vk::CommandBuffer,
	point: vk::PipelineBindPoint,
	layout: vk::PipelineLayout,
	first: u32,
	count: u32,
	sets: *const vk::DescriptorSet,
	offset_count: u32,
	offsets: *const u32,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the operation derives both native counts from borrowed slices.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				command.0 as usize as u64,
				point.as_raw() as u64,
				layout.0,
				first as u64,
				count as u64,
				offset_count as u64,
			]);
			trace.extend(
				std::slice::from_raw_parts(sets, count as usize)
					.iter()
					.map(|set| set.0),
			);
			trace.extend(
				std::slice::from_raw_parts(offsets, offset_count as usize)
					.iter()
					.map(|&v| v as u64),
			);
		});
	}
}

unsafe extern "system" fn requirements(
	device: vk::DeviceHandle,
	info: *const vk::BufferMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the test supplies initialized records and a live dedicated-output chain.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				(*info).buffer.0,
				(*output).s_type.as_raw() as u64,
			])
		});
		(*output).memory_requirements = vk::MemoryRequirements {
			size: 4096,
			alignment: 256,
			memory_type_bits: 5,
		};
		if !(*output).p_next.is_null() {
			let dedicated = (*output).p_next.cast::<vk::MemoryDedicatedRequirements>();
			(*dedicated).requires_dedicated_allocation = vk::TRUE;
		}
	}
}

unsafe extern "system" fn features(
	physical: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceFeatures2,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the caller provides a valid output record; the mock writes only its feature payload.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([physical.0 as usize as u64, (*output).s_type.as_raw() as u64])
		});
		(*output).features.shader_int64 = vk::TRUE;
	}
}

unsafe extern "system" fn barrier(
	command: vk::CommandBuffer,
	source: vk::PipelineStageFlags,
	destination: vk::PipelineStageFlags,
	flags: vk::DependencyFlags,
	memory_count: u32,
	memory: *const vk::MemoryBarrier,
	buffer_count: u32,
	buffers: *const vk::BufferMemoryBarrier,
	image_count: u32,
	images: *const vk::ImageMemoryBarrier,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: each native count is derived from the corresponding borrowed slice.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				command.0 as usize as u64,
				source.as_raw() as u64,
				destination.as_raw() as u64,
				flags.as_raw() as u64,
				memory_count as u64,
				buffer_count as u64,
				image_count as u64,
			]);
			trace.extend(
				std::slice::from_raw_parts(memory, memory_count as usize)
					.iter()
					.map(|v| v.src_access_mask.as_raw() as u64),
			);
			trace.extend(
				std::slice::from_raw_parts(buffers, buffer_count as usize)
					.iter()
					.map(|v| v.offset),
			);
			trace.extend(
				std::slice::from_raw_parts(images, image_count as usize)
					.iter()
					.map(|v| v.image.0),
			);
		});
	}
}

#[test]
fn shader_creation_forwards_callbacks_and_preserves_errors_without_resolving_again() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let info = vk::ShaderModuleCreateInfo {
		code_size: 20,
		..Default::default()
	};
	// SAFETY: mocks only inspect initialized records and produce fake handles.
	unsafe {
		assert_eq!(
			vk::create_shader_module(&fixture.device, &info, None),
			Ok(vk::ShaderModule(0x3000))
		);
		let callbacks = vk::AllocationCallbacks {
			p_user_data: std::ptr::null_mut(),
			pfn_allocation: None,
			pfn_reallocation: None,
			pfn_free: None,
			pfn_internal_allocation: None,
			pfn_internal_free: None,
		};
		assert_eq!(
			fixture.device.create_shader_module(&info, Some(&callbacks)),
			Ok(vk::ShaderModule(0x3000))
		);
		STATUS.set(vk::Result::ERROR_OUT_OF_HOST_MEMORY);
		assert_eq!(
			fixture.device.create_shader_module(&info, None),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		);
	}
	assert_eq!(QUERIES.get(), queries);
	TRACE.with_borrow(|trace| assert_eq!(trace, &[0x2000, 20, 1, 0x2000, 20, 0, 0x2000, 20, 1]));
}

#[test]
fn pipeline_status_and_partial_failure_leave_handles_visible() {
	let fixture = Fixture::new_with_commands(true);
	let infos = std::array::from_fn::<_, 2, _>(|_| vk::ComputePipelineCreateInfo::default());
	let mut pipelines = [vk::Pipeline::NULL; 2];
	// SAFETY: the mock only writes within the verified output slice.
	unsafe {
		STATUS.set(vk::Result::PIPELINE_COMPILE_REQUIRED_EXT);
		assert_eq!(
			fixture.device.create_compute_pipelines(
				vk::PipelineCache::NULL,
				&infos,
				None,
				&mut pipelines
			),
			Ok(vk::Result::PIPELINE_COMPILE_REQUIRED_EXT)
		);
		assert_eq!(pipelines, [vk::Pipeline(0x4000), vk::Pipeline::NULL]);
		STATUS.set(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
		assert_eq!(
			vk::create_compute_pipelines(
				&fixture.device,
				vk::PipelineCache::NULL,
				&infos,
				None,
				&mut pipelines
			),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
		);
		assert_eq!(pipelines, [vk::Pipeline(0x4000), vk::Pipeline::NULL]);
	}
}

#[test]
fn invalid_output_lengths_never_call_the_driver_or_touch_outputs() {
	let fixture = Fixture::new_with_commands(true);
	let calls = CALLS.get();
	let mut pipelines = [vk::Pipeline(0x7777)];
	let mut sets = [vk::DescriptorSet(0x8888)];
	// SAFETY: slice mismatch must fail before the exact-ABI mock can run.
	unsafe {
		assert_eq!(
			fixture.device.create_compute_pipelines(
				vk::PipelineCache::NULL,
				&std::array::from_fn::<_, 2, _>(|_| vk::ComputePipelineCreateInfo::default()),
				None,
				&mut pipelines
			),
			Err(CallError::SliceLengthMismatch {
				parameter: "pipelines",
				expected: 2,
				actual: 1
			})
		);
		assert_eq!(
			fixture.device.allocate_descriptor_sets(
				&vk::DescriptorSetAllocateInfo {
					descriptor_set_count: 2,
					..Default::default()
				},
				&mut sets
			),
			Err(CallError::SliceLengthMismatch {
				parameter: "descriptor_sets",
				expected: 2,
				actual: 1
			})
		);
	}
	assert_eq!(pipelines, [vk::Pipeline(0x7777)]);
	assert_eq!(sets, [vk::DescriptorSet(0x8888)]);
	assert_eq!(CALLS.get(), calls);
}

#[test]
fn descriptor_allocation_and_binding_forward_exact_counts_and_elements() {
	let fixture = Fixture::new_with_commands(true);
	let mut sets = [vk::DescriptorSet::NULL; 2];
	let info = vk::DescriptorSetAllocateInfo {
		descriptor_set_count: 2,
		..Default::default()
	};
	// SAFETY: mocks inspect only the borrowed info/slices; no native objects exist.
	unsafe {
		fixture
			.device
			.allocate_descriptor_sets(&info, &mut sets)
			.unwrap();
		assert_eq!(sets, [vk::DescriptorSet(0x5000), vk::DescriptorSet(0x5001)]);
		fixture
			.device
			.cmd_bind_descriptor_sets(
				vk::CommandBuffer(std::ptr::without_provenance_mut::<c_void>(0x6000)),
				vk::PipelineBindPoint::COMPUTE,
				vk::PipelineLayout(0x7000),
				3,
				&sets,
				&[17, 29, 41],
			)
			.unwrap();
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace,
			&[0x6000, 1, 0x7000, 3, 2, 3, 0x5000, 0x5001, 17, 29, 41]
		)
	});
}

#[test]
fn requirements2_preserves_output_chain_and_does_not_admit_an_unloaded_alias() {
	let fixture = Fixture::new_with_commands(true);
	let mut dedicated = vk::MemoryDedicatedRequirements::default();
	let mut output = vk::MemoryRequirements2 {
		p_next: (&mut dedicated as *mut vk::MemoryDedicatedRequirements).cast(),
		..Default::default()
	};
	let chain = output.p_next;
	let info = vk::BufferMemoryRequirementsInfo2 {
		buffer: vk::Buffer(0x9000),
		..Default::default()
	};
	// SAFETY: the dedicated-output record remains live for this synchronous mock query.
	unsafe {
		vk::get_buffer_memory_requirements2(&fixture.device, &info, &mut output).unwrap();
		assert_eq!(
			fixture
				.device
				.get_buffer_memory_requirements2_khr(&info, &mut output),
			Err(CallError::MissingCommand(
				"vkGetBufferMemoryRequirements2KHR"
			))
		);
	}
	assert_eq!(output.p_next, chain);
	assert_eq!(output.s_type, vk::StructureType::MEMORY_REQUIREMENTS_2);
	assert_eq!(output.memory_requirements.size, 4096);
	assert_eq!(output.memory_requirements.alignment, 256);
	assert_eq!(
		dedicated.s_type,
		vk::StructureType::MEMORY_DEDICATED_REQUIREMENTS
	);
	assert_eq!(dedicated.requires_dedicated_allocation, vk::TRUE);
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace,
			&[
				0x2000,
				0x9000,
				vk::StructureType::MEMORY_REQUIREMENTS_2.as_raw() as u64
			]
		)
	});
}

#[test]
fn physical_device_query_uses_instance_dispatch_and_preserves_chain() {
	let fixture = Fixture::new_with_commands(true);
	let mut extension = vk::PhysicalDeviceVulkan11Features::default();
	let mut output = vk::PhysicalDeviceFeatures2 {
		p_next: (&mut extension as *mut vk::PhysicalDeviceVulkan11Features).cast(),
		..Default::default()
	};
	let chain = output.p_next;
	let physical = vk::PhysicalDevice(std::ptr::without_provenance_mut::<c_void>(0x8000));
	// SAFETY: the exact-ABI mock writes the supplied output payload without accessing native resources.
	unsafe {
		fixture
			.instance
			.get_physical_device_features2(physical, &mut output)
			.unwrap();
	}
	assert_eq!(output.p_next, chain);
	assert_eq!(output.features.shader_int64, vk::TRUE);
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace,
			&[
				0x8000,
				vk::StructureType::PHYSICAL_DEVICE_FEATURES_2.as_raw() as u64
			]
		)
	});
}

#[test]
fn barrier_forwards_three_independent_slice_counts_including_empty_slices() {
	let fixture = Fixture::new_with_commands(true);
	let memory = [vk::MemoryBarrier {
		src_access_mask: vk::AccessFlagBits::SHADER_WRITE_BIT,
		..Default::default()
	}];
	let buffers = [
		vk::BufferMemoryBarrier {
			offset: 13,
			..Default::default()
		},
		vk::BufferMemoryBarrier {
			offset: 29,
			..Default::default()
		},
	];
	let images = [vk::ImageMemoryBarrier {
		image: vk::Image(0x1234),
		..Default::default()
	}];
	// SAFETY: mocks inspect only initialized slices and never execute Vulkan work.
	unsafe {
		fixture
			.device
			.cmd_pipeline_barrier(
				vk::CommandBuffer::NULL,
				vk::PipelineStageFlagBits::COMPUTE_SHADER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlagBits::BY_REGION_BIT,
				&memory,
				&buffers,
				&images,
			)
			.unwrap();
		fixture
			.device
			.cmd_pipeline_barrier(
				vk::CommandBuffer::NULL,
				vk::PipelineStageFlags::empty(),
				vk::PipelineStageFlags::empty(),
				vk::DependencyFlags::empty(),
				&[],
				&[],
				&[],
			)
			.unwrap();
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace,
			&[
				0,
				vk::PipelineStageFlagBits::COMPUTE_SHADER_BIT.as_raw() as u64,
				vk::PipelineStageFlagBits::HOST_BIT.as_raw() as u64,
				vk::DependencyFlagBits::BY_REGION_BIT.as_raw() as u64,
				1,
				2,
				1,
				vk::AccessFlagBits::SHADER_WRITE_BIT.as_raw() as u64,
				13,
				29,
				0x1234,
				0,
				0,
				0,
				0,
				0,
				0,
				0,
			]
		)
	});
}

unsafe extern "system" fn wait_semaphores(
	device: vk::DeviceHandle,
	info: *const vk::SemaphoreWaitInfo,
	timeout: u64,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: tests provide an initialized complete wait record.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([device.as_raw(), (*info).semaphore_count as u64, timeout])
		});
	}
	STATUS.get()
}

unsafe extern "system" fn wait_events2(
	command: vk::CommandBuffer,
	count: u32,
	events: *const vk::Event,
	dependencies: *const vk::DependencyInfo,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: operation verifies equal counts for both initialized input slices.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([command.as_raw(), count as u64]);
			for index in 0..count as usize {
				trace.extend([
					(*events.add(index)).as_raw(),
					(*dependencies.add(index)).memory_barrier_count as u64,
				]);
			}
		});
	}
}

unsafe extern "system" fn submit2(
	queue: vk::Queue,
	count: u32,
	submits: *const vk::SubmitInfo2,
	fence: vk::Fence,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: operation supplies a borrowed initialized slice with the derived count.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([queue.as_raw(), count as u64, fence.as_raw()]);
			for index in 0..count as usize {
				trace.push((*submits.add(index)).command_buffer_info_count as u64);
			}
		});
	}
	STATUS.get()
}

#[test]
fn timeline_wait_preserves_timeout_errors_and_full_width_timeout() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let info = vk::SemaphoreWaitInfo::default();
	// SAFETY: independent mocks inspect only initialized records and scalar arguments.
	unsafe {
		STATUS.set(vk::Result::TIMEOUT);
		assert_eq!(
			fixture.device.wait_semaphores(&info, u64::MAX),
			Ok(vk::Result::TIMEOUT)
		);
		STATUS.set(vk::Result::ERROR_DEVICE_LOST);
		assert_eq!(
			vk::wait_semaphores(&fixture.device, &info, 0),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
		assert!(matches!(
			fixture.device.wait_semaphores_khr(&info, 0),
			Err(CallError::MissingCommand("vkWaitSemaphoresKHR"))
		));
	}
	assert_eq!(QUERIES.get(), queries);
	TRACE.with_borrow(|trace| assert_eq!(trace, &[0x2000, 0, u64::MAX, 0x2000, 0, 0]));
}

#[test]
fn sync2_checks_parallel_arrays_and_preserves_counts_queue_and_native_status() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let calls = CALLS.get();
	let dependencies = [
		vk::DependencyInfo {
			memory_barrier_count: 3,
			..Default::default()
		},
		vk::DependencyInfo {
			memory_barrier_count: 5,
			..Default::default()
		},
	];
	// SAFETY: mocks accept opaque tokens and only read within checked input slices.
	unsafe {
		assert!(matches!(
			fixture
				.device
				.cmd_wait_events2(vk::CommandBuffer::NULL, &[vk::Event(17)], &dependencies),
			Err(CallError::SliceLengthMismatch { .. })
		));
		assert_eq!(CALLS.get(), calls);
		fixture
			.device
			.cmd_wait_events2(
				vk::CommandBuffer::NULL,
				&[vk::Event(17), vk::Event(29)],
				&dependencies,
			)
			.unwrap();
		fixture
			.device
			.cmd_wait_events2(vk::CommandBuffer::NULL, &[], &[])
			.unwrap();
		STATUS.set(vk::Result::ERROR_DEVICE_LOST);
		assert_eq!(
			fixture.device.queue_submit2(
				vk::Queue::NULL,
				&[vk::SubmitInfo2 {
					command_buffer_info_count: 7,
					..Default::default()
				}],
				vk::Fence(41)
			),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
	}
	assert_eq!(QUERIES.get(), queries);
	TRACE.with_borrow(|trace| assert_eq!(trace, &[0, 2, 17, 3, 29, 5, 0, 0, 0, 1, 41, 7]));
}

unsafe extern "system" fn groups(
	instance: vk::InstanceHandle,
	count: *mut u32,
	values: *mut vk::PhysicalDeviceGroupProperties,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: tests pass initialized records with writable count/storage. The
	// deliberately invalid reported count never causes an out-of-bounds write.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				instance.0 as usize as u64,
				u64::from(*count),
				values.is_null() as u64,
			])
		});
		if values.is_null() {
			*count = 2;
		} else {
			let capacity = *count;
			for index in 0..capacity.min(2) as usize {
				let value = &mut *values.add(index);
				TRACE.with_borrow_mut(|trace| {
					trace.extend([value.s_type.as_raw() as u64, value.p_next as usize as u64])
				});
				value.physical_device_count = 1;
				value.physical_devices[0] =
					vk::PhysicalDevice(std::ptr::without_provenance_mut(0x7000 + index));
			}
			*count = if BAD_COUNT.get() {
				capacity + 1
			} else {
				capacity.min(2)
			};
		}
	}
	STATUS.get()
}

unsafe extern "system" fn families2(
	_: vk::PhysicalDevice,
	count: *mut u32,
	values: *mut vk::QueueFamilyProperties2,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: count is writable; nonempty output has a complete initialized
	// header and an optional exact QueueFamilyCheckpointPropertiesNV extension.
	unsafe {
		if values.is_null() {
			*count = 3;
		} else {
			let capacity = *count;
			if capacity != 0 {
				TRACE.with_borrow_mut(|trace| {
					trace.extend([
						(*values).s_type.as_raw() as u64,
						(*values).p_next as usize as u64,
					])
				});
				(*values).queue_family_properties.queue_count = u32::MAX;
				if !(*values).p_next.is_null() {
					(*(*values)
						.p_next
						.cast::<vk::QueueFamilyCheckpointPropertiesNV>())
					.checkpoint_execution_stage_mask = vk::PipelineStageFlagBits::TRANSFER_BIT;
				}
			}
			*count = if BAD_COUNT.get() {
				capacity + 1
			} else {
				capacity.min(1)
			};
		}
	}
}

unsafe extern "system" fn sparse_formats2(
	physical: vk::PhysicalDevice,
	info: *const vk::PhysicalDeviceSparseImageFormatInfo2,
	count: *mut u32,
	values: *mut vk::SparseImageFormatProperties2,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: complete input and count remain live; output is written only when
	// the caller provided nonzero initialized capacity.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([physical.0 as usize as u64, (*info).format.as_raw() as u64])
		});
		if values.is_null() {
			*count = 1;
		} else if *count != 0 {
			TRACE.with_borrow_mut(|trace| {
				trace.extend([
					(*values).s_type.as_raw() as u64,
					(*values).p_next as usize as u64,
				])
			});
			(*values).properties.image_granularity.width = 127;
			*count = 1;
		}
	}
}

unsafe extern "system" fn sparse_requirements2(
	device: vk::DeviceHandle,
	info: *const vk::ImageSparseMemoryRequirementsInfo2,
	count: *mut u32,
	values: *mut vk::SparseImageMemoryRequirements2,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: typed inputs/count are live and output is initialized through capacity.
	unsafe {
		TRACE.with_borrow_mut(|trace| trace.extend([device.0 as usize as u64, (*info).image.0]));
		if values.is_null() {
			*count = 1;
		} else if *count != 0 {
			TRACE.with_borrow_mut(|trace| {
				trace.extend([
					(*values).s_type.as_raw() as u64,
					(*values).p_next as usize as u64,
				])
			});
			(*values).memory_requirements.image_mip_tail_size = u64::MAX;
			*count = 1;
		}
	}
}

unsafe extern "system" fn image_format2(
	_: vk::PhysicalDevice,
	info: *const vk::PhysicalDeviceImageFormatInfo2,
	output: *mut vk::ImageFormatProperties2,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: operation forwards complete live input and output records.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				(*info).format.as_raw() as u64,
				(*output).s_type.as_raw() as u64,
				(*output).p_next as usize as u64,
			])
		});
		(*output).image_format_properties.max_resource_size = u64::MAX;
	}
	STATUS.get()
}

unsafe extern "system" fn create_template(
	device: vk::DeviceHandle,
	info: *const vk::DescriptorUpdateTemplateCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::DescriptorUpdateTemplate,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: typed complete input and writable handle output are live.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				(*info).descriptor_update_entry_count.into(),
				callbacks as usize as u64,
			])
		});
		*output = vk::DescriptorUpdateTemplate(0x8000);
	}
	STATUS.get()
}

unsafe extern "system" fn template_payload(
	device: vk::DeviceHandle,
	set: vk::DescriptorSet,
	template: vk::DescriptorUpdateTemplate,
	data: *const c_void,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: tests provide one live, aligned descriptor-buffer record. The
	// pointer identity is recorded, proving no replacement or byte copying.
	unsafe {
		let value = &*data.cast::<vk::DescriptorBufferInfo>();
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				set.0,
				template.0,
				data as usize as u64,
				value.buffer.0,
				value.offset,
				value.range,
			])
		});
	}
}

unsafe extern "system" fn destroy_template(
	device: vk::DeviceHandle,
	template: vk::DescriptorUpdateTemplate,
	callbacks: *const vk::AllocationCallbacks,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			device.0 as usize as u64,
			template.0,
			callbacks as usize as u64,
		])
	});
}

unsafe extern "system" fn create_conversion(
	_: vk::DeviceHandle,
	info: *const vk::SamplerYcbcrConversionCreateInfo,
	_: *const vk::AllocationCallbacks,
	output: *mut vk::SamplerYcbcrConversion,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: complete create info and output storage come from the operation.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				(*info).s_type.as_raw() as u64,
				(*info).format.as_raw() as u64,
			])
		});
		*output = vk::SamplerYcbcrConversion(0x8100);
	}
	STATUS.get()
}

unsafe extern "system" fn destroy_conversion(
	_: vk::DeviceHandle,
	conversion: vk::SamplerYcbcrConversion,
	_: *const vk::AllocationCallbacks,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| trace.push(conversion.0));
}

unsafe extern "system" fn queue2(
	device: vk::DeviceHandle,
	info: *const vk::DeviceQueueInfo2,
	output: *mut vk::Queue,
) {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: input and writable output are complete and live during this call.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				(*info).flags.as_raw() as u64,
				(*info).queue_family_index.into(),
				(*info).queue_index.into(),
			])
		});
		*output = vk::Queue(std::ptr::without_provenance_mut(0x9000));
	}
}

unsafe extern "system" fn dispatch_base(
	command: vk::CommandBuffer,
	bx: u32,
	by: u32,
	bz: u32,
	x: u32,
	y: u32,
	z: u32,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			command.0 as usize as u64,
			bx.into(),
			by.into(),
			bz.into(),
			x.into(),
			y.into(),
			z.into(),
		])
	});
}

unsafe extern "system" fn device_mask(command: vk::CommandBuffer, mask: u32) {
	CALLS.set(CALLS.get() + 1);
	TRACE.with_borrow_mut(|trace| trace.extend([command.0 as usize as u64, mask.into()]));
}

unsafe extern "system" fn peer_features(
	_: vk::DeviceHandle,
	heap: u32,
	local: u32,
	remote: u32,
	output: *mut vk::PeerMemoryFeatureFlags,
) {
	CALLS.set(CALLS.get() + 1);
	TRACE
		.with_borrow_mut(|trace| trace.extend([u64::from(heap), u64::from(local), u64::from(remote)]));
	// SAFETY: operation passes a live writable flags scalar.
	unsafe {
		*output = vk::PeerMemoryFeatureFlags::from_raw(u32::MAX);
	}
}

#[test]
fn counted_groups_preserve_status_capacity_headers_and_partial_outputs() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	CALLS.set(0);
	// SAFETY: exact-ABI mocks, initialized output storage and no native objects.
	unsafe {
		let mut count = u32::MAX;
		assert_eq!(
			fixture
				.instance
				.enumerate_physical_device_groups(&mut count, None)
				.unwrap(),
			vk::Result::SUCCESS
		);
		assert_eq!(count, 2);
		assert_eq!(
			TRACE.with_borrow(|t| t.clone()),
			[0x1000, u64::from(u32::MAX), 1]
		);
		TRACE.with_borrow_mut(Vec::clear);
		let mut records =
			std::array::from_fn::<_, 3, _>(|_| vk::PhysicalDeviceGroupProperties::default());
		let untouched = vk::PhysicalDevice(std::ptr::without_provenance_mut(0xdead));
		records[2].physical_devices[0] = untouched;
		STATUS.set(vk::Result::INCOMPLETE);
		assert_eq!(
			vk::enumerate_physical_device_groups(&fixture.instance, &mut count, Some(&mut records[..1]))
				.unwrap(),
			vk::Result::INCOMPLETE
		);
		assert_eq!(count, 1);
		assert_eq!(records[0].physical_devices[0].0 as usize, 0x7000);
		assert_eq!(
			TRACE.with_borrow(|t| t.clone()),
			[
				0x1000,
				1,
				0,
				vk::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES.as_raw() as u64,
				0
			]
		);
		assert_eq!(CALLS.get(), 2, "no implicit retry");
		STATUS.set(vk::Result::SUCCESS);
		fixture
			.instance
			.enumerate_physical_device_groups_khr(&mut count, Some(&mut records))
			.unwrap();
		assert_eq!(count, 2, "shrink retains the actual initialized prefix");
		assert_eq!(records[2].physical_devices[0], untouched);
		fixture
			.instance
			.enumerate_physical_device_groups(&mut count, Some(&mut []))
			.unwrap();
		assert_eq!(count, 0, "non-null zero capacity is not a null count query");
		BAD_COUNT.set(true);
		assert!(matches!(
			fixture
				.instance
				.enumerate_physical_device_groups(&mut count, Some(&mut records[..1])),
			Err(CallError::InvalidOutput("vkEnumeratePhysicalDeviceGroups"))
		));
		STATUS.set(vk::Result::ERROR_OUT_OF_HOST_MEMORY);
		assert!(matches!(
			fixture
				.instance
				.enumerate_physical_device_groups(&mut count, Some(&mut records[..1])),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		));
		assert_eq!(QUERIES.get(), queries);
	}
}

#[test]
fn void_counted_queries_keep_output_chains_and_exact_native_counts() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let physical = vk::PhysicalDevice(std::ptr::without_provenance_mut(0x5000));
	// SAFETY: exact mock prototypes and complete initialized local records;
	// QueueFamilyCheckpointPropertiesNV is the mock's declared chain payload.
	unsafe {
		let mut count = 99;
		fixture
			.instance
			.get_physical_device_queue_family_properties2(physical, &mut count, None)
			.unwrap();
		assert_eq!(count, 3);
		let mut extension = vk::QueueFamilyCheckpointPropertiesNV::default();
		let next = (&mut extension as *mut vk::QueueFamilyCheckpointPropertiesNV).cast();
		let mut families = std::array::from_fn::<_, 2, _>(|_| vk::QueueFamilyProperties2 {
			p_next: next,
			..Default::default()
		});
		// Only the first record has this extension; native writes just that record.
		families[1].p_next = std::ptr::null_mut();
		TRACE.with_borrow_mut(Vec::clear);
		fixture
			.instance
			.get_physical_device_queue_family_properties2_khr(physical, &mut count, Some(&mut families))
			.unwrap();
		assert_eq!(count, 1);
		assert_eq!(families[0].p_next, next);
		assert_eq!(
			families[0].s_type,
			vk::StructureType::QUEUE_FAMILY_PROPERTIES_2
		);
		assert_eq!(
			extension.s_type,
			vk::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV
		);
		assert_eq!(
			extension.checkpoint_execution_stage_mask,
			vk::PipelineStageFlagBits::TRANSFER_BIT
		);
		assert_eq!(families[0].queue_family_properties.queue_count, u32::MAX);
		assert_eq!(
			TRACE.with_borrow(|t| t.clone()),
			[
				vk::StructureType::QUEUE_FAMILY_PROPERTIES_2.as_raw() as u64,
				next as usize as u64
			]
		);
		BAD_COUNT.set(true);
		assert!(matches!(
			fixture
				.instance
				.get_physical_device_queue_family_properties2(physical, &mut count, Some(&mut families)),
			Err(CallError::InvalidOutput(_))
		));
		BAD_COUNT.set(false);
		let info = vk::PhysicalDeviceSparseImageFormatInfo2 {
			format: vk::Format::R8_UNORM,
			..Default::default()
		};
		let mut formats = [vk::SparseImageFormatProperties2::default()];
		for operation in [
			vk::get_physical_device_sparse_image_format_properties2,
			vk::get_physical_device_sparse_image_format_properties2_khr,
		] {
			operation(&fixture.instance, physical, &info, &mut count, None).unwrap();
			assert_eq!(count, 1);
			operation(
				&fixture.instance,
				physical,
				&info,
				&mut count,
				Some(&mut formats),
			)
			.unwrap();
			assert_eq!(count, 1);
			assert_eq!(formats[0].properties.image_granularity.width, 127);
			assert_eq!(
				formats[0].s_type,
				vk::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2
			);
		}
		let info = vk::ImageSparseMemoryRequirementsInfo2 {
			image: vk::Image(0x6000),
			..Default::default()
		};
		let mut requirements = [vk::SparseImageMemoryRequirements2::default()];
		for operation in [
			vk::get_image_sparse_memory_requirements2,
			vk::get_image_sparse_memory_requirements2_khr,
		] {
			operation(&fixture.device, &info, &mut count, None).unwrap();
			assert_eq!(count, 1);
			operation(&fixture.device, &info, &mut count, Some(&mut requirements)).unwrap();
			assert_eq!(
				requirements[0].memory_requirements.image_mip_tail_size,
				u64::MAX
			);
			assert_eq!(
				requirements[0].s_type,
				vk::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2
			);
		}
		let mut output = vk::ImageFormatProperties2::default();
		STATUS.set(vk::Result::ERROR_FORMAT_NOT_SUPPORTED);
		assert!(matches!(
			fixture
				.instance
				.get_physical_device_image_format_properties2(
					physical,
					&vk::PhysicalDeviceImageFormatInfo2::default(),
					&mut output
				),
			Err(CallError::Vulkan(vk::Result::ERROR_FORMAT_NOT_SUPPORTED))
		));
		assert_eq!(output.image_format_properties.max_resource_size, u64::MAX);
		assert_eq!(output.s_type, vk::StructureType::IMAGE_FORMAT_PROPERTIES_2);
		assert_eq!(QUERIES.get(), queries);
	}
}

#[test]
fn descriptor_template_and_conversion_preserve_payload_identity_and_native_errors() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	// SAFETY: mock contexts read only complete local ABI records; explicit
	// destroy operations record tokens without destroying any native resource.
	unsafe {
		let info = vk::DescriptorUpdateTemplateCreateInfo::default();
		let template = fixture
			.device
			.create_descriptor_update_template(&info, None)
			.unwrap();
		assert_eq!(template, vk::DescriptorUpdateTemplate(0x8000));
		let aliased = fixture
			.device
			.create_descriptor_update_template_khr(&info, None)
			.unwrap();
		assert_eq!(aliased, template);
		STATUS.set(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
		assert!(matches!(
			fixture
				.device
				.create_descriptor_update_template(&info, None),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
		));
		let data = vk::DescriptorBufferInfo {
			buffer: vk::Buffer(0x1234),
			offset: 1 << 40,
			range: u64::MAX,
		};
		let ptr = (&data as *const vk::DescriptorBufferInfo).cast();
		for operation in [
			vk::update_descriptor_set_with_template,
			vk::update_descriptor_set_with_template_khr,
		] {
			TRACE.with_borrow_mut(Vec::clear);
			operation(&fixture.device, vk::DescriptorSet(0xabcd), template, ptr).unwrap();
			assert_eq!(
				TRACE.with_borrow(|t| t.clone()),
				[
					0x2000,
					0xabcd,
					0x8000,
					ptr as usize as u64,
					0x1234,
					1 << 40,
					u64::MAX
				]
			);
		}
		fixture
			.device
			.destroy_descriptor_update_template(template, None)
			.unwrap();
		STATUS.set(vk::Result::SUCCESS);
		let conversion = fixture
			.device
			.create_sampler_ycbcr_conversion(
				&vk::SamplerYcbcrConversionCreateInfo {
					format: vk::Format::G8_B8_R8_3PLANE_420_UNORM,
					..Default::default()
				},
				None,
			)
			.unwrap();
		assert_eq!(conversion, vk::SamplerYcbcrConversion(0x8100));
		fixture
			.device
			.destroy_sampler_ycbcr_conversion(conversion, None)
			.unwrap();
		assert_eq!(QUERIES.get(), queries);
	}
}

#[test]
fn device_group_and_queue_operations_preserve_native_scalar_arguments() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	// SAFETY: exact mocks, initialized queue output, artificial non-native handles.
	unsafe {
		let mut queue = vk::Queue::NULL;
		fixture
			.device
			.get_device_queue2(
				&vk::DeviceQueueInfo2 {
					flags: vk::DeviceQueueCreateFlags::from_raw(1),
					queue_family_index: 17,
					queue_index: 3,
					..Default::default()
				},
				&mut queue,
			)
			.unwrap();
		assert_eq!(queue.0 as usize, 0x9000);
		assert_eq!(TRACE.with_borrow(|t| t.clone()), [0x2000, 1, 17, 3]);
		TRACE.with_borrow_mut(Vec::clear);
		let command = vk::CommandBuffer(std::ptr::without_provenance_mut(0x6000));
		fixture
			.device
			.cmd_dispatch_base(command, 1, 2, u32::MAX, 5, 6, 7)
			.unwrap();
		fixture
			.device
			.cmd_set_device_mask(command, 0x80000000)
			.unwrap();
		let mut features = vk::PeerMemoryFeatureFlags::default();
		fixture
			.device
			.get_device_group_peer_memory_features(11, 12, 13, &mut features)
			.unwrap();
		assert_eq!(features.as_raw(), u32::MAX);
		assert_eq!(
			TRACE.with_borrow(|t| t.clone()),
			[
				0x6000,
				1,
				2,
				u64::from(u32::MAX),
				5,
				6,
				7,
				0x6000,
				0x80000000,
				11,
				12,
				13
			]
		);
		assert_eq!(QUERIES.get(), queries);
	}
}

unsafe extern "system" fn buffer_address(
	device: vk::DeviceHandle,
	info: *const vk::BufferDeviceAddressInfo,
) -> u64 {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the operation forwards a live initialized record without changing its chain.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				info as usize as u64,
				(*info).s_type.as_raw() as u64,
				(*info).p_next as usize as u64,
				(*info).buffer.0,
			])
		});
	}
	SCALAR.get()
}

unsafe extern "system" fn memory_address(
	device: vk::DeviceHandle,
	info: *const vk::DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the operation forwards the caller's initialized record and live chain.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				info as usize as u64,
				(*info).s_type.as_raw() as u64,
				(*info).p_next as usize as u64,
				(*info).memory.0,
			])
		});
	}
	SCALAR.get()
}

#[test]
fn address_queries_preserve_zero_high_bits_input_identity_and_alias_independence() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let info = vk::BufferDeviceAddressInfo {
		buffer: vk::Buffer(0xfedcba9876543210),
		..Default::default()
	};
	let memory = vk::DeviceMemoryOpaqueCaptureAddressInfo {
		memory: vk::DeviceMemory(u64::MAX),
		..Default::default()
	};
	// SAFETY: exact mocks only inspect initialized input records; native Vulkan is not invoked.
	unsafe {
		for value in [0, 1, 1 << 40, 1 << 63, u64::MAX] {
			SCALAR.set(value);
			let calls = CALLS.get();
			for function in [
				vk::get_buffer_device_address,
				vk::get_buffer_device_address_khr,
				vk::Device::get_buffer_device_address_ext,
				vk::get_buffer_opaque_capture_address,
				vk::Device::get_buffer_opaque_capture_address_khr,
			] {
				assert_eq!(function(&fixture.device, &info).unwrap(), value);
			}
			assert_eq!(
				fixture
					.device
					.get_device_memory_opaque_capture_address(&memory)
					.unwrap(),
				value
			);
			assert_eq!(
				vk::get_device_memory_opaque_capture_address_khr(&fixture.device, &memory).unwrap(),
				value
			);
			assert_eq!(CALLS.get(), calls + 7);
		}
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(trace.len(), 175);
		for group in trace.as_chunks::<35>().0 {
			for actual in group[..25].as_chunks::<5>().0 {
				assert_eq!(
					*actual,
					[
						0x2000,
						&info as *const _ as usize as u64,
						info.s_type.as_raw() as u64,
						0,
						info.buffer.0
					]
				);
			}
			for actual in group[25..].as_chunks::<5>().0 {
				assert_eq!(
					*actual,
					[
						0x2000,
						&memory as *const _ as usize as u64,
						memory.s_type.as_raw() as u64,
						0,
						memory.memory.0
					]
				);
			}
		}
	});
	assert_eq!(QUERIES.get(), queries);
	BLOCKED.set(Some(b"vkGetBufferDeviceAddressKHR"));
	let fixture = Fixture::new_with_commands(true);
	BLOCKED.set(None);
	// SAFETY: the core mock is present but cannot substitute for the absent KHR PFN.
	unsafe {
		SCALAR.set(1 << 63);
		assert_eq!(
			fixture.device.get_buffer_device_address(&info).unwrap(),
			1 << 63
		);
		assert_eq!(
			fixture.device.get_buffer_device_address_khr(&info),
			Err(CallError::MissingCommand("vkGetBufferDeviceAddressKHR"))
		);
	}
}

unsafe extern "system" fn reset_queries(
	device: vk::DeviceHandle,
	pool: vk::QueryPool,
	first: u32,
	count: u32,
) {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			device.0 as usize as u64,
			pool.0,
			u64::from(first),
			u64::from(count),
		])
	});
}

unsafe extern "system" fn indirect_count(
	command: vk::CommandBuffer,
	buffer: vk::Buffer,
	offset: u64,
	count_buffer: vk::Buffer,
	count_offset: u64,
	max: u32,
	stride: u32,
) {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			command.0 as usize as u64,
			buffer.0,
			offset,
			count_buffer.0,
			count_offset,
			u64::from(max),
			u64::from(stride),
		])
	});
}

#[test]
fn indirect_count_and_host_query_reset_keep_exact_native_scalar_arguments() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	// SAFETY: scalar-only exact mocks use artificial handles and do not execute native commands.
	unsafe {
		fixture
			.device
			.reset_query_pool(vk::QueryPool(17), u32::MAX, 0)
			.unwrap();
		vk::reset_query_pool_ext(&fixture.device, vk::QueryPool(29), 0, u32::MAX).unwrap();
		assert_eq!(
			TRACE.with_borrow(|t| t.clone()),
			[
				0x2000,
				17,
				u64::from(u32::MAX),
				0,
				0x2000,
				29,
				0,
				u64::from(u32::MAX)
			]
		);
		TRACE.with_borrow_mut(Vec::clear);
		let command = vk::CommandBuffer(std::ptr::without_provenance_mut(0x6000));
		for function in [
			vk::cmd_draw_indirect_count,
			vk::Device::cmd_draw_indirect_count_khr,
			vk::cmd_draw_indirect_count_amd,
			vk::Device::cmd_draw_indexed_indirect_count,
			vk::cmd_draw_indexed_indirect_count_khr,
			vk::cmd_draw_indexed_indirect_count_amd,
		] {
			function(
				&fixture.device,
				command,
				vk::Buffer(11),
				u64::MAX,
				vk::Buffer(23),
				1 << 40,
				u32::MAX,
				0,
			)
			.unwrap();
		}
		TRACE.with_borrow(|trace| {
			assert_eq!(trace.len(), 42);
			for actual in trace.as_chunks::<7>().0 {
				assert_eq!(
					*actual,
					[0x6000, 11, u64::MAX, 23, 1 << 40, u64::from(u32::MAX), 0]
				);
			}
		});
	}
	assert_eq!(QUERIES.get(), queries);
}

unsafe extern "system" fn create_render_pass2(
	device: vk::DeviceHandle,
	info: *const vk::RenderPassCreateInfo2,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::RenderPass,
) -> vk::Result {
	CALLS.set(CALLS.get() + 1);
	// SAFETY: the wrapper supplies live initialized input/callbacks and a writable output slot.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				device.0 as usize as u64,
				info as usize as u64,
				(*info).p_next as usize as u64,
				u64::from((*info).subpass_count),
				(*info).p_subpasses as usize as u64,
				callbacks as usize as u64,
			])
		});
		if !STATUS.get().is_err() {
			*output = vk::RenderPass(u64::MAX);
		}
	}
	STATUS.get()
}

unsafe extern "system" fn begin_pass2(
	command: vk::CommandBuffer,
	info: *const vk::RenderPassBeginInfo,
	begin: *const vk::SubpassBeginInfo,
) {
	// SAFETY: tests supply live initialized records; chains and nested pointers are only recorded.
	unsafe {
		TRACE.with_borrow_mut(|trace| {
			trace.extend([
				command.0 as usize as u64,
				info as usize as u64,
				(*info).render_pass.0,
				begin as usize as u64,
				(*begin).contents.as_raw() as u64,
			])
		});
	}
}

unsafe extern "system" fn next_subpass2(
	command: vk::CommandBuffer,
	begin: *const vk::SubpassBeginInfo,
	end: *const vk::SubpassEndInfo,
) {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			command.0 as usize as u64,
			begin as usize as u64,
			end as usize as u64,
		])
	});
}

unsafe extern "system" fn end_pass2(command: vk::CommandBuffer, end: *const vk::SubpassEndInfo) {
	TRACE.with_borrow_mut(|trace| trace.extend([command.0 as usize as u64, end as usize as u64]));
}

#[test]
fn render_pass2_preserves_callbacks_errors_and_borrowed_record_identity() {
	let fixture = Fixture::new_with_commands(true);
	let queries = QUERIES.get();
	let subpasses = std::array::from_fn::<_, 2, _>(|_| vk::SubpassDescription2::default());
	let input = vk::RenderPassCreateInfo2 {
		subpass_count: 2,
		p_subpasses: subpasses.as_ptr(),
		..Default::default()
	};
	let callbacks = vk::AllocationCallbacks {
		p_user_data: std::ptr::null_mut(),
		pfn_allocation: None,
		pfn_reallocation: None,
		pfn_free: None,
		pfn_internal_allocation: None,
		pfn_internal_free: None,
	};
	// SAFETY: exact mocks only inspect initialized records; callbacks are recorded, never called.
	unsafe {
		for function in [vk::create_render_pass2, vk::Device::create_render_pass2_khr] {
			assert_eq!(
				function(&fixture.device, &input, Some(&callbacks)).unwrap(),
				vk::RenderPass(u64::MAX)
			);
			STATUS.set(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
			assert_eq!(
				function(&fixture.device, &input, None),
				Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
			);
			STATUS.set(vk::Result::SUCCESS);
		}
		TRACE.with_borrow(|trace| {
			assert_eq!(trace.len(), 24);
			for (index, actual) in trace.as_chunks::<6>().0.iter().enumerate() {
				assert_eq!(
					*actual,
					[
						0x2000,
						&input as *const _ as usize as u64,
						0,
						2,
						subpasses.as_ptr() as usize as u64,
						if index % 2 == 0 {
							&callbacks as *const _ as usize as u64
						} else {
							0
						}
					]
				);
			}
		});
		TRACE.with_borrow_mut(Vec::clear);
		let command = vk::CommandBuffer(std::ptr::without_provenance_mut(0x6000));
		let render = vk::RenderPassBeginInfo {
			render_pass: vk::RenderPass(91),
			..Default::default()
		};
		let begin = vk::SubpassBeginInfo {
			contents: vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
			..Default::default()
		};
		let end = vk::SubpassEndInfo::default();
		for function in [
			vk::cmd_begin_render_pass2,
			vk::Device::cmd_begin_render_pass2_khr,
		] {
			function(&fixture.device, command, &render, &begin).unwrap();
		}
		for function in [vk::Device::cmd_next_subpass2, vk::cmd_next_subpass2_khr] {
			function(&fixture.device, command, &begin, &end).unwrap();
		}
		for function in [vk::cmd_end_render_pass2, vk::cmd_end_render_pass2_khr] {
			function(&fixture.device, command, &end).unwrap();
		}
		let begin_ptr = &begin as *const _ as usize as u64;
		let end_ptr = &end as *const _ as usize as u64;
		let mut expected = Vec::new();
		for _ in 0..2 {
			expected.extend([
				0x6000,
				&render as *const _ as usize as u64,
				91,
				begin_ptr,
				begin.contents.as_raw() as u64,
			]);
		}
		for _ in 0..2 {
			expected.extend([0x6000, begin_ptr, end_ptr]);
		}
		for _ in 0..2 {
			expected.extend([0x6000, end_ptr]);
		}
		TRACE.with_borrow(|trace| assert_eq!(*trace, expected));
	}
	assert_eq!(QUERIES.get(), queries);
}
