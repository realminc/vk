// @generated — do not edit by hand.
#![allow(non_camel_case_types)]
use super::{enums::*, types::*};

pub type PFN_vkInternalAllocationNotification = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		allocation_type: InternalAllocationType,
		allocation_scope: SystemAllocationScope,
	) -> (),
>;

pub type PFN_vkInternalFreeNotification = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		allocation_type: InternalAllocationType,
		allocation_scope: SystemAllocationScope,
	) -> (),
>;

pub type PFN_vkReallocationFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		p_original: *mut std::ffi::c_void,
		size: usize,
		alignment: usize,
		allocation_scope: SystemAllocationScope,
	) -> *mut std::ffi::c_void,
>;

pub type PFN_vkAllocationFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		alignment: usize,
		allocation_scope: SystemAllocationScope,
	) -> *mut std::ffi::c_void,
>;

pub type PFN_vkFreeFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		p_memory: *mut std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn() -> ()>;

pub type PFN_vkDebugReportCallbackEXT = Option<
	unsafe extern "system" fn(
		flags: DebugReportFlagsEXT,
		object_type: DebugReportObjectTypeEXT,
		object: u64,
		location: usize,
		message_code: i32,
		p_layer_prefix: *const std::ffi::c_char,
		p_message: *const std::ffi::c_char,
		p_user_data: *mut std::ffi::c_void,
	) -> Bool32,
>;

pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<
	unsafe extern "system" fn(
		message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
		message_types: DebugUtilsMessageTypeFlagsEXT,
		p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
		p_user_data: *mut std::ffi::c_void,
	) -> Bool32,
>;

pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<
	unsafe extern "system" fn(
		p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
		p_user_data: *mut std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkGetInstanceProcAddrLUNARG = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

pub type PFN_vkCreateInstance = Option<
	unsafe extern "system" fn(
		p_create_info: *const InstanceCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_instance: *mut InstanceHandle,
	) -> Result,
>;

pub type PFN_vkDestroyInstance = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkEnumeratePhysicalDevices = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_physical_device_count: *mut u32,
		p_physical_devices: *mut PhysicalDevice,
	) -> Result,
>;

pub type PFN_vkGetDeviceProcAddr = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

pub type PFN_vkGetInstanceProcAddr = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

pub type PFN_vkGetPhysicalDeviceProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_properties: *mut PhysicalDeviceProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_queue_family_property_count: *mut u32,
		p_queue_family_properties: *mut QueueFamilyProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_memory_properties: *mut PhysicalDeviceMemoryProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFeatures = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_features: *mut PhysicalDeviceFeatures,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		p_format_properties: *mut FormatProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		r#type: ImageType,
		tiling: ImageTiling,
		usage: ImageUsageFlags,
		flags: ImageCreateFlags,
		p_image_format_properties: *mut ImageFormatProperties,
	) -> Result,
>;

pub type PFN_vkCreateDevice = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_create_info: *const DeviceCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_device: *mut DeviceHandle,
	) -> Result,
>;

pub type PFN_vkDestroyDevice = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_allocator: *const AllocationCallbacks) -> (),
>;

pub type PFN_vkEnumerateInstanceVersion =
	Option<unsafe extern "system" fn(p_api_version: *mut u32) -> Result>;

pub type PFN_vkEnumerateInstanceLayerProperties = Option<
	unsafe extern "system" fn(
		p_property_count: *mut u32,
		p_properties: *mut LayerProperties,
	) -> Result,
>;

pub type PFN_vkEnumerateInstanceExtensionProperties = Option<
	unsafe extern "system" fn(
		p_layer_name: *const std::ffi::c_char,
		p_property_count: *mut u32,
		p_properties: *mut ExtensionProperties,
	) -> Result,
>;

pub type PFN_vkEnumerateDeviceLayerProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut LayerProperties,
	) -> Result,
>;

pub type PFN_vkEnumerateDeviceExtensionProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_layer_name: *const std::ffi::c_char,
		p_property_count: *mut u32,
		p_properties: *mut ExtensionProperties,
	) -> Result,
>;

pub type PFN_vkGetDeviceQueue = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		queue_family_index: u32,
		queue_index: u32,
		p_queue: *mut Queue,
	) -> (),
>;

pub type PFN_vkQueueSubmit = Option<
	unsafe extern "system" fn(
		queue: Queue,
		submit_count: u32,
		p_submits: *const SubmitInfo,
		fence: Fence,
	) -> Result,
>;

pub type PFN_vkQueueWaitIdle = Option<unsafe extern "system" fn(queue: Queue) -> Result>;

pub type PFN_vkDeviceWaitIdle = Option<unsafe extern "system" fn(device: DeviceHandle) -> Result>;

pub type PFN_vkAllocateMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const MemoryAllocateInfo,
		p_allocator: *const AllocationCallbacks,
		p_memory: *mut DeviceMemory,
	) -> Result,
>;

pub type PFN_vkFreeMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkMapMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		offset: DeviceSize,
		size: DeviceSize,
		flags: MemoryMapFlags,
		pp_data: *mut *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkUnmapMemory =
	Option<unsafe extern "system" fn(device: DeviceHandle, memory: DeviceMemory) -> ()>;

pub type PFN_vkFlushMappedMemoryRanges = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory_range_count: u32,
		p_memory_ranges: *const MappedMemoryRange,
	) -> Result,
>;

pub type PFN_vkInvalidateMappedMemoryRanges = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory_range_count: u32,
		p_memory_ranges: *const MappedMemoryRange,
	) -> Result,
>;

pub type PFN_vkGetDeviceMemoryCommitment = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		p_committed_memory_in_bytes: *mut DeviceSize,
	) -> (),
>;

pub type PFN_vkGetBufferMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		p_memory_requirements: *mut MemoryRequirements,
	) -> (),
>;

pub type PFN_vkBindBufferMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		memory: DeviceMemory,
		memory_offset: DeviceSize,
	) -> Result,
>;

pub type PFN_vkGetImageMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_memory_requirements: *mut MemoryRequirements,
	) -> (),
>;

pub type PFN_vkBindImageMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		memory: DeviceMemory,
		memory_offset: DeviceSize,
	) -> Result,
>;

pub type PFN_vkGetImageSparseMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		r#type: ImageType,
		samples: SampleCountFlagBits,
		usage: ImageUsageFlags,
		tiling: ImageTiling,
		p_property_count: *mut u32,
		p_properties: *mut SparseImageFormatProperties,
	) -> (),
>;

pub type PFN_vkQueueBindSparse = Option<
	unsafe extern "system" fn(
		queue: Queue,
		bind_info_count: u32,
		p_bind_info: *const BindSparseInfo,
		fence: Fence,
	) -> Result,
>;

pub type PFN_vkCreateFence = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const FenceCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

pub type PFN_vkDestroyFence = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence: Fence,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkResetFences = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence_count: u32,
		p_fences: *const Fence,
	) -> Result,
>;

pub type PFN_vkGetFenceStatus =
	Option<unsafe extern "system" fn(device: DeviceHandle, fence: Fence) -> Result>;

pub type PFN_vkWaitForFences = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence_count: u32,
		p_fences: *const Fence,
		wait_all: Bool32,
		timeout: u64,
	) -> Result,
>;

pub type PFN_vkCreateSemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SemaphoreCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_semaphore: *mut Semaphore,
	) -> Result,
>;

pub type PFN_vkDestroySemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		semaphore: Semaphore,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateEvent = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const EventCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_event: *mut Event,
	) -> Result,
>;

pub type PFN_vkDestroyEvent = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		event: Event,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetEventStatus =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

pub type PFN_vkSetEvent =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

pub type PFN_vkResetEvent =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

pub type PFN_vkCreateQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const QueryPoolCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_query_pool: *mut QueryPool,
	) -> Result,
>;

pub type PFN_vkDestroyQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		query_pool: QueryPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetQueryPoolResults = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
		stride: DeviceSize,
		flags: QueryResultFlags,
	) -> Result,
>;

pub type PFN_vkResetQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
	) -> (),
>;

pub type PFN_vkResetQueryPoolEXT = PFN_vkResetQueryPool;

pub type PFN_vkCreateBuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_buffer: *mut Buffer,
	) -> Result,
>;

pub type PFN_vkDestroyBuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateBufferView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferViewCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut BufferView,
	) -> Result,
>;

pub type PFN_vkDestroyBufferView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer_view: BufferView,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ImageCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_image: *mut Image,
	) -> Result,
>;

pub type PFN_vkDestroyImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_subresource: *const ImageSubresource,
		p_layout: *mut SubresourceLayout,
	) -> (),
>;

pub type PFN_vkCreateImageView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ImageViewCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut ImageView,
	) -> Result,
>;

pub type PFN_vkDestroyImageView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_view: ImageView,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateShaderModule = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderModuleCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_shader_module: *mut ShaderModule,
	) -> Result,
>;

pub type PFN_vkDestroyShaderModule = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader_module: ShaderModule,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreatePipelineCache = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineCacheCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_pipeline_cache: *mut PipelineCache,
	) -> Result,
>;

pub type PFN_vkDestroyPipelineCache = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetPipelineCacheData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkMergePipelineCaches = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		dst_cache: PipelineCache,
		src_cache_count: u32,
		p_src_caches: *const PipelineCache,
	) -> Result,
>;

pub type PFN_vkCreatePipelineBinariesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineBinaryCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_binaries: *mut PipelineBinaryHandlesInfoKHR,
	) -> Result,
>;

pub type PFN_vkDestroyPipelineBinaryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_binary: PipelineBinaryKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetPipelineKeyKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_create_info: *const PipelineCreateInfoKHR,
		p_pipeline_key: *mut PipelineBinaryKeyKHR,
	) -> Result,
>;

pub type PFN_vkGetPipelineBinaryDataKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PipelineBinaryDataInfoKHR,
		p_pipeline_binary_key: *mut PipelineBinaryKeyKHR,
		p_pipeline_binary_data_size: *mut usize,
		p_pipeline_binary_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkReleaseCapturedPipelineDataKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ReleaseCapturedPipelineDataInfoKHR,
		p_allocator: *const AllocationCallbacks,
	) -> Result,
>;

pub type PFN_vkCreateGraphicsPipelines = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const GraphicsPipelineCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

pub type PFN_vkCreateComputePipelines = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const ComputePipelineCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		renderpass: RenderPass,
		p_max_workgroup_size: *mut Extent2D,
	) -> Result,
>;

pub type PFN_vkDestroyPipeline = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreatePipelineLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineLayoutCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_pipeline_layout: *mut PipelineLayout,
	) -> Result,
>;

pub type PFN_vkDestroyPipelineLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_layout: PipelineLayout,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateSampler = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SamplerCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_sampler: *mut Sampler,
	) -> Result,
>;

pub type PFN_vkDestroySampler = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		sampler: Sampler,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateDescriptorSetLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorSetLayoutCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_set_layout: *mut DescriptorSetLayout,
	) -> Result,
>;

pub type PFN_vkDestroyDescriptorSetLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set_layout: DescriptorSetLayout,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorPoolCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_descriptor_pool: *mut DescriptorPool,
	) -> Result,
>;

pub type PFN_vkDestroyDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkResetDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		flags: DescriptorPoolResetFlags,
	) -> Result,
>;

pub type PFN_vkAllocateDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const DescriptorSetAllocateInfo,
		p_descriptor_sets: *mut DescriptorSet,
	) -> Result,
>;

pub type PFN_vkFreeDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		descriptor_set_count: u32,
		p_descriptor_sets: *const DescriptorSet,
	) -> Result,
>;

pub type PFN_vkUpdateDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_write_count: u32,
		p_descriptor_writes: *const WriteDescriptorSet,
		descriptor_copy_count: u32,
		p_descriptor_copies: *const CopyDescriptorSet,
	) -> (),
>;

pub type PFN_vkCreateFramebuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const FramebufferCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_framebuffer: *mut Framebuffer,
	) -> Result,
>;

pub type PFN_vkDestroyFramebuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		framebuffer: Framebuffer,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateRenderPass = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const RenderPassCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_render_pass: *mut RenderPass,
	) -> Result,
>;

pub type PFN_vkDestroyRenderPass = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		render_pass: RenderPass,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetRenderAreaGranularity = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		render_pass: RenderPass,
		p_granularity: *mut Extent2D,
	) -> (),
>;

pub type PFN_vkGetRenderingAreaGranularity = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_rendering_area_info: *const RenderingAreaInfo,
		p_granularity: *mut Extent2D,
	) -> (),
>;

pub type PFN_vkGetRenderingAreaGranularityKHR = PFN_vkGetRenderingAreaGranularity;

pub type PFN_vkCreateCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CommandPoolCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_command_pool: *mut CommandPool,
	) -> Result,
>;

pub type PFN_vkDestroyCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkResetCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		flags: CommandPoolResetFlags,
	) -> Result,
>;

pub type PFN_vkAllocateCommandBuffers = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const CommandBufferAllocateInfo,
		p_command_buffers: *mut CommandBuffer,
	) -> Result,
>;

pub type PFN_vkFreeCommandBuffers = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		command_buffer_count: u32,
		p_command_buffers: *const CommandBuffer,
	) -> (),
>;

pub type PFN_vkBeginCommandBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_info: *const CommandBufferBeginInfo,
	) -> Result,
>;

pub type PFN_vkEndCommandBuffer =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result>;

pub type PFN_vkResetCommandBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		flags: CommandBufferResetFlags,
	) -> Result,
>;

pub type PFN_vkCmdBindPipeline = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
	) -> (),
>;

pub type PFN_vkCmdSetPrimitiveRestartIndexEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_index: u32) -> (),
>;

pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags) -> (),
>;

pub type PFN_vkCmdSetViewport = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewports: *const Viewport,
	) -> (),
>;

pub type PFN_vkCmdSetScissor = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_scissor: u32,
		scissor_count: u32,
		p_scissors: *const Rect2D,
	) -> (),
>;

pub type PFN_vkCmdSetLineWidth =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32) -> ()>;

pub type PFN_vkCmdSetDepthBias = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		depth_bias_constant_factor: f32,
		depth_bias_clamp: f32,
		depth_bias_slope_factor: f32,
	) -> (),
>;

pub type PFN_vkCmdSetBlendConstants = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const f32) -> (),
>;

pub type PFN_vkCmdSetDepthBounds = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		min_depth_bounds: f32,
		max_depth_bounds: f32,
	) -> (),
>;

pub type PFN_vkCmdSetStencilCompareMask = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		compare_mask: u32,
	) -> (),
>;

pub type PFN_vkCmdSetStencilWriteMask = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		write_mask: u32,
	) -> (),
>;

pub type PFN_vkCmdSetStencilReference = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		reference: u32,
	) -> (),
>;

pub type PFN_vkCmdBindDescriptorSets = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		first_set: u32,
		descriptor_set_count: u32,
		p_descriptor_sets: *const DescriptorSet,
		dynamic_offset_count: u32,
		p_dynamic_offsets: *const u32,
	) -> (),
>;

pub type PFN_vkCmdBindIndexBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		index_type: IndexType,
	) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_buffers: *const Buffer,
		p_offsets: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdDraw = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		vertex_count: u32,
		instance_count: u32,
		first_vertex: u32,
		first_instance: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndexed = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		index_count: u32,
		instance_count: u32,
		first_index: u32,
		vertex_offset: i32,
		first_instance: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMultiEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		draw_count: u32,
		p_vertex_info: *const MultiDrawInfoEXT,
		instance_count: u32,
		first_instance: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMultiIndexedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		draw_count: u32,
		p_index_info: *const MultiDrawIndexedInfoEXT,
		instance_count: u32,
		first_instance: u32,
		stride: u32,
		p_vertex_offset: *const i32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndexedIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDispatch = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

pub type PFN_vkCmdDispatchIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdSubpassShadingHUAWEI =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdDrawClusterHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawClusterIndirectHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
	) -> (),
>;

pub type PFN_vkCmdCopyBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_buffer: Buffer,
		dst_buffer: Buffer,
		region_count: u32,
		p_regions: *const BufferCopy,
	) -> (),
>;

pub type PFN_vkCmdCopyImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_image: Image,
		src_image_layout: ImageLayout,
		dst_image: Image,
		dst_image_layout: ImageLayout,
		region_count: u32,
		p_regions: *const ImageCopy,
	) -> (),
>;

pub type PFN_vkCmdBlitImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_image: Image,
		src_image_layout: ImageLayout,
		dst_image: Image,
		dst_image_layout: ImageLayout,
		region_count: u32,
		p_regions: *const ImageBlit,
		filter: Filter,
	) -> (),
>;

pub type PFN_vkCmdCopyBufferToImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_buffer: Buffer,
		dst_image: Image,
		dst_image_layout: ImageLayout,
		region_count: u32,
		p_regions: *const BufferImageCopy,
	) -> (),
>;

pub type PFN_vkCmdCopyImageToBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_image: Image,
		src_image_layout: ImageLayout,
		dst_buffer: Buffer,
		region_count: u32,
		p_regions: *const BufferImageCopy,
	) -> (),
>;

pub type PFN_vkCmdCopyMemoryIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		copy_buffer_address: DeviceAddress,
		copy_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdCopyMemoryIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		copy_buffer_address: DeviceAddress,
		copy_count: u32,
		stride: u32,
		dst_image: Image,
		dst_image_layout: ImageLayout,
		p_image_subresources: *const ImageSubresourceLayers,
	) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdUpdateBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		data_size: DeviceSize,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCmdFillBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		size: DeviceSize,
		data: u32,
	) -> (),
>;

pub type PFN_vkCmdClearColorImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image: Image,
		image_layout: ImageLayout,
		p_color: *const ClearColorValue,
		range_count: u32,
		p_ranges: *const ImageSubresourceRange,
	) -> (),
>;

pub type PFN_vkCmdClearDepthStencilImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image: Image,
		image_layout: ImageLayout,
		p_depth_stencil: *const ClearDepthStencilValue,
		range_count: u32,
		p_ranges: *const ImageSubresourceRange,
	) -> (),
>;

pub type PFN_vkCmdClearAttachments = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		attachment_count: u32,
		p_attachments: *const ClearAttachment,
		rect_count: u32,
		p_rects: *const ClearRect,
	) -> (),
>;

pub type PFN_vkCmdResolveImage = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_image: Image,
		src_image_layout: ImageLayout,
		dst_image: Image,
		dst_image_layout: ImageLayout,
		region_count: u32,
		p_regions: *const ImageResolve,
	) -> (),
>;

pub type PFN_vkCmdSetEvent = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags,
	) -> (),
>;

pub type PFN_vkCmdResetEvent = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags,
	) -> (),
>;

pub type PFN_vkCmdWaitEvents = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event_count: u32,
		p_events: *const Event,
		src_stage_mask: PipelineStageFlags,
		dst_stage_mask: PipelineStageFlags,
		memory_barrier_count: u32,
		p_memory_barriers: *const MemoryBarrier,
		buffer_memory_barrier_count: u32,
		p_buffer_memory_barriers: *const BufferMemoryBarrier,
		image_memory_barrier_count: u32,
		p_image_memory_barriers: *const ImageMemoryBarrier,
	) -> (),
>;

pub type PFN_vkCmdPipelineBarrier = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_stage_mask: PipelineStageFlags,
		dst_stage_mask: PipelineStageFlags,
		dependency_flags: DependencyFlags,
		memory_barrier_count: u32,
		p_memory_barriers: *const MemoryBarrier,
		buffer_memory_barrier_count: u32,
		p_buffer_memory_barriers: *const BufferMemoryBarrier,
		image_memory_barrier_count: u32,
		p_image_memory_barriers: *const ImageMemoryBarrier,
	) -> (),
>;

pub type PFN_vkCmdBeginQuery = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		flags: QueryControlFlags,
	) -> (),
>;

pub type PFN_vkCmdEndQuery = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32) -> (),
>;

pub type PFN_vkCmdBeginConditionalRenderingEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdEndConditionalRenderingEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdBeginCustomResolveEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdResetQueryPool = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
	) -> (),
>;

pub type PFN_vkCmdWriteTimestamp = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_stage: PipelineStageFlagBits,
		query_pool: QueryPool,
		query: u32,
	) -> (),
>;

pub type PFN_vkCmdCopyQueryPoolResults = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		stride: DeviceSize,
		flags: QueryResultFlags,
	) -> (),
>;

pub type PFN_vkCmdPushConstants = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		layout: PipelineLayout,
		stage_flags: ShaderStageFlags,
		offset: u32,
		size: u32,
		p_values: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCmdBeginRenderPass = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_render_pass_begin: *const RenderPassBeginInfo,
		contents: SubpassContents,
	) -> (),
>;

pub type PFN_vkCmdNextSubpass =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents) -> ()>;

pub type PFN_vkCmdEndRenderPass =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdExecuteCommands = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		command_buffer_count: u32,
		p_command_buffers: *const CommandBuffer,
	) -> (),
>;

#[cfg(feature = "platform-android")]
pub type PFN_vkCreateAndroidSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const AndroidSurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ohos")]
pub type PFN_vkCreateSurfaceOHOS = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const SurfaceCreateInfoOHOS,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPlanePropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		plane_index: u32,
		p_display_count: *mut u32,
		p_displays: *mut DisplayKHR,
	) -> Result,
>;

pub type PFN_vkGetDisplayModePropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_property_count: *mut u32,
		p_properties: *mut DisplayModePropertiesKHR,
	) -> Result,
>;

pub type PFN_vkCreateDisplayModeKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_create_info: *const DisplayModeCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_mode: *mut DisplayModeKHR,
	) -> Result,
>;

pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		mode: DisplayModeKHR,
		plane_index: u32,
		p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
	) -> Result,
>;

pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DisplaySurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

pub type PFN_vkCreateSharedSwapchainsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain_count: u32,
		p_create_infos: *const SwapchainCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_swapchains: *mut SwapchainKHR,
	) -> Result,
>;

pub type PFN_vkDestroySurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		surface: SurfaceKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		surface: SurfaceKHR,
		p_supported: *mut Bool32,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_format_count: *mut u32,
		p_surface_formats: *mut SurfaceFormatKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_present_mode_count: *mut u32,
		p_present_modes: *mut PresentModeKHR,
	) -> Result,
>;

pub type PFN_vkCreateSwapchainKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SwapchainCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_swapchain: *mut SwapchainKHR,
	) -> Result,
>;

pub type PFN_vkDestroySwapchainKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetSwapchainImagesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_image_count: *mut u32,
		p_swapchain_images: *mut Image,
	) -> Result,
>;

pub type PFN_vkAcquireNextImageKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		timeout: u64,
		semaphore: Semaphore,
		fence: Fence,
		p_image_index: *mut u32,
	) -> Result,
>;

pub type PFN_vkQueuePresentKHR =
	Option<unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR) -> Result>;

#[cfg(feature = "platform-vi")]
pub type PFN_vkCreateViSurfaceNN = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ViSurfaceCreateInfoNN,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-wayland")]
pub type PFN_vkCreateWaylandSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const WaylandSurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-wayland")]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		display: *mut wl_display,
	) -> Bool32,
>;

#[cfg(feature = "platform-ubm")]
pub type PFN_vkCreateUbmSurfaceSEC = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const UbmSurfaceCreateInfoSEC,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ubm")]
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		device: *mut ubm_device,
	) -> Bool32,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkCreateWin32SurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const Win32SurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<
	unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32,
>;

#[cfg(feature = "platform-xlib")]
pub type PFN_vkCreateXlibSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const XlibSurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xlib")]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		dpy: *mut Display,
		visual_id: VisualID,
	) -> Bool32,
>;

#[cfg(feature = "platform-xcb")]
pub type PFN_vkCreateXcbSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const XcbSurfaceCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xcb")]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		connection: *mut xcb_connection_t,
		visual_id: xcb_visualid_t,
	) -> Bool32,
>;

#[cfg(feature = "platform-directfb")]
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DirectFBSurfaceCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-directfb")]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		dfb: *mut IDirectFB,
	) -> Bool32,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ggp")]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
pub type PFN_vkCreateScreenSurfaceQNX = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ScreenSurfaceCreateInfoQNX,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		window: *mut _screen_window,
	) -> Bool32,
>;

pub type PFN_vkCreateDebugReportCallbackEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DebugReportCallbackCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_callback: *mut DebugReportCallbackEXT,
	) -> Result,
>;

pub type PFN_vkDestroyDebugReportCallbackEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		callback: DebugReportCallbackEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDebugReportMessageEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		flags: DebugReportFlagsEXT,
		object_type: DebugReportObjectTypeEXT,
		object: u64,
		location: usize,
		message_code: i32,
		p_layer_prefix: *const std::ffi::c_char,
		p_message: *const std::ffi::c_char,
	) -> (),
>;

pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name_info: *const DebugMarkerObjectNameInfoEXT,
	) -> Result,
>;

pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_tag_info: *const DebugMarkerObjectTagInfoEXT,
	) -> Result,
>;

pub type PFN_vkCmdDebugMarkerBeginEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const DebugMarkerMarkerInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdDebugMarkerEndEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdDebugMarkerInsertEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const DebugMarkerMarkerInfoEXT,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		r#type: ImageType,
		tiling: ImageTiling,
		usage: ImageUsageFlags,
		flags: ImageCreateFlags,
		external_handle_type: ExternalMemoryHandleTypeFlagsNV,
		p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetMemoryWin32HandleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		handle_type: ExternalMemoryHandleTypeFlagsNV,
		p_handle: *mut HANDLE,
	) -> Result,
>;

pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		is_preprocessed: Bool32,
		p_generated_commands_info: *const GeneratedCommandsInfoNV,
	) -> (),
>;

pub type PFN_vkCmdPreprocessGeneratedCommandsNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_generated_commands_info: *const GeneratedCommandsInfoNV,
	) -> (),
>;

pub type PFN_vkCmdBindPipelineShaderGroupNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
		group_index: u32,
	) -> (),
>;

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkCreateIndirectCommandsLayoutNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
	) -> Result,
>;

pub type PFN_vkDestroyIndirectCommandsLayoutNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_commands_layout: IndirectCommandsLayoutNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCmdExecuteGeneratedCommandsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		is_preprocessed: Bool32,
		p_generated_commands_info: *const GeneratedCommandsInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_generated_commands_info: *const GeneratedCommandsInfoEXT,
		state_command_buffer: CommandBuffer,
	) -> (),
>;

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkCreateIndirectCommandsLayoutEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectCommandsLayoutCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
	) -> Result,
>;

pub type PFN_vkDestroyIndirectCommandsLayoutEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_commands_layout: IndirectCommandsLayoutEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateIndirectExecutionSetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectExecutionSetCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_indirect_execution_set: *mut IndirectExecutionSetEXT,
	) -> Result,
>;

pub type PFN_vkDestroyIndirectExecutionSetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		execution_set_write_count: u32,
		p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
	) -> (),
>;

pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		execution_set_write_count: u32,
		p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_features: *mut PhysicalDeviceFeatures2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;

pub type PFN_vkGetPhysicalDeviceProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_properties: *mut PhysicalDeviceProperties2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;

pub type PFN_vkGetPhysicalDeviceFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		p_format_properties: *mut FormatProperties2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
		p_image_format_properties: *mut ImageFormatProperties2,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
	PFN_vkGetPhysicalDeviceImageFormatProperties2;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_queue_family_property_count: *mut u32,
		p_queue_family_properties: *mut QueueFamilyProperties2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
	PFN_vkGetPhysicalDeviceQueueFamilyProperties2;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
		p_property_count: *mut u32,
		p_properties: *mut SparseImageFormatProperties2,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
	PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;

pub type PFN_vkCmdPushDescriptorSet = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		set: u32,
		descriptor_write_count: u32,
		p_descriptor_writes: *const WriteDescriptorSet,
	) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;

pub type PFN_vkTrimCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		flags: CommandPoolTrimFlags,
	) -> (),
>;

pub type PFN_vkTrimCommandPoolKHR = PFN_vkTrimCommandPool;

pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
		p_external_buffer_properties: *mut ExternalBufferProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalBufferProperties;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetMemoryWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		handle: HANDLE,
		p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetMemoryFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const MemoryGetFdInfoKHR,
		p_fd: *mut i32,
	) -> Result,
>;

pub type PFN_vkGetMemoryFdPropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		fd: i32,
		p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
		p_zircon_handle: *mut zx_handle_t,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		zircon_handle: zx_handle_t,
		p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
	) -> Result,
>;

pub type PFN_vkGetMemoryRemoteAddressNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
		p_address: *mut RemoteAddressNV,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
		p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
	) -> Result,
>;

pub type PFN_vkGetSemaphoreFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const SemaphoreGetFdInfoKHR,
		p_fd: *mut i32,
	) -> Result,
>;

pub type PFN_vkImportSemaphoreFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
		p_zircon_handle: *mut zx_handle_t,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
		p_external_fence_properties: *mut ExternalFenceProperties,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalFenceProperties;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetFenceWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkImportFenceWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
	) -> Result,
>;

pub type PFN_vkGetFenceFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const FenceGetFdInfoKHR,
		p_fd: *mut i32,
	) -> Result,
>;

pub type PFN_vkImportFenceFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
	) -> Result,
>;

pub type PFN_vkReleaseDisplayEXT =
	Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result>;

#[cfg(feature = "platform-xlib-xrandr")]
pub type PFN_vkAcquireXlibDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		dpy: *mut Display,
		display: DisplayKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xlib-xrandr")]
pub type PFN_vkGetRandROutputDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		dpy: *mut Display,
		rr_output: RROutput,
		p_display: *mut DisplayKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkAcquireWinrtDisplayNV =
	Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetWinrtDisplayNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		device_relative_id: u32,
		p_display: *mut DisplayKHR,
	) -> Result,
>;

pub type PFN_vkDisplayPowerControlEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		display: DisplayKHR,
		p_display_power_info: *const DisplayPowerInfoEXT,
	) -> Result,
>;

pub type PFN_vkRegisterDeviceEventEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_device_event_info: *const DeviceEventInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

pub type PFN_vkRegisterDisplayEventEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		display: DisplayKHR,
		p_display_event_info: *const DisplayEventInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

pub type PFN_vkGetSwapchainCounterEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		counter: SurfaceCounterFlagBitsEXT,
		p_counter_value: *mut u64,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_capabilities: *mut SurfaceCapabilities2EXT,
	) -> Result,
>;

pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_physical_device_group_count: *mut u32,
		p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
	) -> Result,
>;

pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;

pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		heap_index: u32,
		local_device_index: u32,
		remote_device_index: u32,
		p_peer_memory_features: *mut PeerMemoryFeatureFlags,
	) -> (),
>;

pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;

pub type PFN_vkBindBufferMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindBufferMemoryInfo,
	) -> Result,
>;

pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;

pub type PFN_vkBindImageMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindImageMemoryInfo,
	) -> Result,
>;

pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;

pub type PFN_vkCmdSetDeviceMask =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32) -> ()>;

pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;

pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
	) -> Result,
>;

pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		surface: SurfaceKHR,
		p_modes: *mut DeviceGroupPresentModeFlagsKHR,
	) -> Result,
>;

pub type PFN_vkAcquireNextImage2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_acquire_info: *const AcquireNextImageInfoKHR,
		p_image_index: *mut u32,
	) -> Result,
>;

pub type PFN_vkCmdDispatchBase = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		base_group_x: u32,
		base_group_y: u32,
		base_group_z: u32,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;

pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_rect_count: *mut u32,
		p_rects: *mut Rect2D,
	) -> Result,
>;

pub type PFN_vkCreateDescriptorUpdateTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorUpdateTemplateCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_descriptor_update_template: *mut DescriptorUpdateTemplate,
	) -> Result,
>;

pub type PFN_vkCreateDescriptorUpdateTemplateKHR = PFN_vkCreateDescriptorUpdateTemplate;

pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_update_template: DescriptorUpdateTemplate,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = PFN_vkDestroyDescriptorUpdateTemplate;

pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set: DescriptorSet,
		descriptor_update_template: DescriptorUpdateTemplate,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = PFN_vkUpdateDescriptorSetWithTemplate;

pub type PFN_vkCmdPushDescriptorSetWithTemplate = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		descriptor_update_template: DescriptorUpdateTemplate,
		layout: PipelineLayout,
		set: u32,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;

pub type PFN_vkSetHdrMetadataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain_count: u32,
		p_swapchains: *const SwapchainKHR,
		p_metadata: *const HdrMetadataEXT,
	) -> (),
>;

pub type PFN_vkGetSwapchainStatusKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
	) -> Result,
>;

pub type PFN_vkGetPastPresentationTimingGOOGLE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_presentation_timing_count: *mut u32,
		p_presentation_timings: *mut PastPresentationTimingGOOGLE,
	) -> Result,
>;

#[cfg(feature = "platform-ios")]
pub type PFN_vkCreateIOSSurfaceMVK = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const IOSSurfaceCreateInfoMVK,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-macos")]
pub type PFN_vkCreateMacOSSurfaceMVK = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const MacOSSurfaceCreateInfoMVK,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
pub type PFN_vkCreateMetalSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const MetalSurfaceCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

pub type PFN_vkCmdSetViewportWScalingNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewport_w_scalings: *const ViewportWScalingNV,
	) -> (),
>;

pub type PFN_vkCmdSetDiscardRectangleEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_discard_rectangle: u32,
		discard_rectangle_count: u32,
		p_discard_rectangles: *const Rect2D,
	) -> (),
>;

pub type PFN_vkCmdSetDiscardRectangleEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetDiscardRectangleModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		discard_rectangle_mode: DiscardRectangleModeEXT,
	) -> (),
>;

pub type PFN_vkCmdSetSampleLocationsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_sample_locations_info: *const SampleLocationsInfoEXT,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		samples: SampleCountFlagBits,
		p_multisample_properties: *mut MultisamplePropertiesEXT,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
		p_surface_capabilities: *mut SurfaceCapabilities2KHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
		p_surface_format_count: *mut u32,
		p_surface_formats: *mut SurfaceFormat2KHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayProperties2KHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPlaneProperties2KHR,
	) -> Result,
>;

pub type PFN_vkGetDisplayModeProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_property_count: *mut u32,
		p_properties: *mut DisplayModeProperties2KHR,
	) -> Result,
>;

pub type PFN_vkGetDisplayPlaneCapabilities2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_display_plane_info: *const DisplayPlaneInfo2KHR,
		p_capabilities: *mut DisplayPlaneCapabilities2KHR,
	) -> Result,
>;

pub type PFN_vkGetBufferMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferMemoryRequirementsInfo2,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;

pub type PFN_vkGetImageMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageMemoryRequirementsInfo2,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;

pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageSparseMemoryRequirementsInfo2,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;

pub type PFN_vkGetDeviceBufferMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceBufferMemoryRequirements,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;

pub type PFN_vkGetDeviceImageMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageMemoryRequirements,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;

pub type PFN_vkGetDeviceImageSparseMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageMemoryRequirements,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
	PFN_vkGetDeviceImageSparseMemoryRequirements;

pub type PFN_vkCreateSamplerYcbcrConversion = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SamplerYcbcrConversionCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_ycbcr_conversion: *mut SamplerYcbcrConversion,
	) -> Result,
>;

pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;

pub type PFN_vkDestroySamplerYcbcrConversion = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		ycbcr_conversion: SamplerYcbcrConversion,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;

pub type PFN_vkGetDeviceQueue2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_queue_info: *const DeviceQueueInfo2,
		p_queue: *mut Queue,
	) -> (),
>;

pub type PFN_vkCreateValidationCacheEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ValidationCacheCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_validation_cache: *mut ValidationCacheEXT,
	) -> Result,
>;

pub type PFN_vkDestroyValidationCacheEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		validation_cache: ValidationCacheEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetValidationCacheDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		validation_cache: ValidationCacheEXT,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkMergeValidationCachesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		dst_cache: ValidationCacheEXT,
		src_cache_count: u32,
		p_src_caches: *const ValidationCacheEXT,
	) -> Result,
>;

pub type PFN_vkGetDescriptorSetLayoutSupport = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorSetLayoutCreateInfo,
		p_support: *mut DescriptorSetLayoutSupport,
	) -> (),
>;

pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;

pub type PFN_vkGetShaderInfoAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		shader_stage: ShaderStageFlagBits,
		info_type: ShaderInfoTypeAMD,
		p_info_size: *mut usize,
		p_info: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkSetLocalDimmingAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swap_chain: SwapchainKHR,
		local_dimming_enable: Bool32,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_time_domain_count: *mut u32,
		p_time_domains: *mut TimeDomainKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
	PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;

pub type PFN_vkGetCalibratedTimestampsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		timestamp_count: u32,
		p_timestamp_infos: *const CalibratedTimestampInfoKHR,
		p_timestamps: *mut u64,
		p_max_deviation: *mut u64,
	) -> Result,
>;

pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;

pub type PFN_vkSetDebugUtilsObjectNameEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name_info: *const DebugUtilsObjectNameInfoEXT,
	) -> Result,
>;

pub type PFN_vkSetDebugUtilsObjectTagEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_tag_info: *const DebugUtilsObjectTagInfoEXT,
	) -> Result,
>;

pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
	Option<unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT) -> ()>;

pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<unsafe extern "system" fn(queue: Queue) -> ()>;

pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
	Option<unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT) -> ()>;

pub type PFN_vkCmdBeginDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_label_info: *const DebugUtilsLabelEXT,
	) -> (),
>;

pub type PFN_vkCmdEndDebugUtilsLabelEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdInsertDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_label_info: *const DebugUtilsLabelEXT,
	) -> (),
>;

pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_messenger: *mut DebugUtilsMessengerEXT,
	) -> Result,
>;

pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		messenger: DebugUtilsMessengerEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
		message_types: DebugUtilsMessageTypeFlagsEXT,
		p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
	) -> (),
>;

pub type PFN_vkGetMemoryHostPointerPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		p_host_pointer: *const std::ffi::c_void,
		p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
	) -> Result,
>;

pub type PFN_vkCmdWriteBufferMarkerAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_stage: PipelineStageFlagBits,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		marker: u32,
	) -> (),
>;

pub type PFN_vkCreateRenderPass2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const RenderPassCreateInfo2,
		p_allocator: *const AllocationCallbacks,
		p_render_pass: *mut RenderPass,
	) -> Result,
>;

pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;

pub type PFN_vkCmdBeginRenderPass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_render_pass_begin: *const RenderPassBeginInfo,
		p_subpass_begin_info: *const SubpassBeginInfo,
	) -> (),
>;

pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;

pub type PFN_vkCmdNextSubpass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_subpass_begin_info: *const SubpassBeginInfo,
		p_subpass_end_info: *const SubpassEndInfo,
	) -> (),
>;

pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;

pub type PFN_vkCmdEndRenderPass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_subpass_end_info: *const SubpassEndInfo,
	) -> (),
>;

pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;

pub type PFN_vkGetSemaphoreCounterValue = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		semaphore: Semaphore,
		p_value: *mut u64,
	) -> Result,
>;

pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;

pub type PFN_vkWaitSemaphores = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_wait_info: *const SemaphoreWaitInfo,
		timeout: u64,
	) -> Result,
>;

pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;

pub type PFN_vkSignalSemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_signal_info: *const SemaphoreSignalInfo,
	) -> Result,
>;

pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;

#[cfg(feature = "platform-android")]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const AHardwareBuffer,
		p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
	) -> Result,
>;

#[cfg(feature = "platform-android")]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
		p_buffer: *mut *mut AHardwareBuffer,
	) -> Result,
>;

pub type PFN_vkCmdDrawIndirectCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		count_buffer: Buffer,
		count_buffer_offset: DeviceSize,
		max_draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirectCountKHR = PFN_vkCmdDrawIndirectCount;

pub type PFN_vkCmdDrawIndirectCountAMD = PFN_vkCmdDrawIndirectCount;

pub type PFN_vkCmdDrawIndexedIndirectCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		count_buffer: Buffer,
		count_buffer_offset: DeviceSize,
		max_draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndexedIndirectCountKHR = PFN_vkCmdDrawIndexedIndirectCount;

pub type PFN_vkCmdDrawIndexedIndirectCountAMD = PFN_vkCmdDrawIndexedIndirectCount;

pub type PFN_vkCmdSetCheckpointNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_checkpoint_marker: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkGetQueueCheckpointDataNV = Option<
	unsafe extern "system" fn(
		queue: Queue,
		p_checkpoint_data_count: *mut u32,
		p_checkpoint_data: *mut CheckpointDataNV,
	) -> (),
>;

pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_buffers: *const Buffer,
		p_offsets: *const DeviceSize,
		p_sizes: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdBeginTransformFeedbackEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_buffer: u32,
		counter_buffer_count: u32,
		p_counter_buffers: *const Buffer,
		p_counter_buffer_offsets: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdEndTransformFeedbackEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_buffer: u32,
		counter_buffer_count: u32,
		p_counter_buffers: *const Buffer,
		p_counter_buffer_offsets: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdBeginQueryIndexedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		flags: QueryControlFlags,
		index: u32,
	) -> (),
>;

pub type PFN_vkCmdEndQueryIndexedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		index: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirectByteCountEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		instance_count: u32,
		first_instance: u32,
		counter_buffer: Buffer,
		counter_buffer_offset: DeviceSize,
		counter_offset: u32,
		vertex_stride: u32,
	) -> (),
>;

pub type PFN_vkCmdSetExclusiveScissorNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_exclusive_scissor: u32,
		exclusive_scissor_count: u32,
		p_exclusive_scissors: *const Rect2D,
	) -> (),
>;

pub type PFN_vkCmdSetExclusiveScissorEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_exclusive_scissor: u32,
		exclusive_scissor_count: u32,
		p_exclusive_scissor_enables: *const Bool32,
	) -> (),
>;

pub type PFN_vkCmdBindShadingRateImageNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image_view: ImageView,
		image_layout: ImageLayout,
	) -> (),
>;

pub type PFN_vkCmdSetViewportShadingRatePaletteNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_shading_rate_palettes: *const ShadingRatePaletteNV,
	) -> (),
>;

pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		sample_order_type: CoarseSampleOrderTypeNV,
		custom_sample_order_count: u32,
		p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		count_buffer: Buffer,
		count_buffer_offset: DeviceSize,
		max_draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirectEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		count_buffer: Buffer,
		count_buffer_offset: DeviceSize,
		max_draw_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCompileDeferredNV = Option<
	unsafe extern "system" fn(device: DeviceHandle, pipeline: Pipeline, shader: u32) -> Result,
>;

pub type PFN_vkCreateAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureNV,
	) -> Result,
>;

pub type PFN_vkCmdBindInvocationMaskHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image_view: ImageView,
		image_layout: ImageLayout,
	) -> (),
>;

pub type PFN_vkDestroyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDestroyAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkBindAccelerationStructureMemoryNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
	) -> Result,
>;

pub type PFN_vkCmdCopyAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst: AccelerationStructureNV,
		src: AccelerationStructureNV,
		mode: CopyAccelerationStructureModeKHR,
	) -> (),
>;

pub type PFN_vkCmdCopyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyAccelerationStructureInfoKHR,
	) -> (),
>;

pub type PFN_vkCopyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyAccelerationStructureInfoKHR,
	) -> Result,
>;

pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
	) -> (),
>;

pub type PFN_vkCopyAccelerationStructureToMemoryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
	) -> Result,
>;

pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
	) -> (),
>;

pub type PFN_vkCopyMemoryToAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
	) -> Result,
>;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		acceleration_structure_count: u32,
		p_acceleration_structures: *const AccelerationStructureKHR,
		query_type: QueryType,
		query_pool: QueryPool,
		first_query: u32,
	) -> (),
>;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		acceleration_structure_count: u32,
		p_acceleration_structures: *const AccelerationStructureNV,
		query_type: QueryType,
		query_pool: QueryPool,
		first_query: u32,
	) -> (),
>;

pub type PFN_vkCmdBuildAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const AccelerationStructureInfoNV,
		instance_data: Buffer,
		instance_offset: DeviceSize,
		update: Bool32,
		dst: AccelerationStructureNV,
		src: AccelerationStructureNV,
		scratch: Buffer,
		scratch_offset: DeviceSize,
	) -> (),
>;

pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure_count: u32,
		p_acceleration_structures: *const AccelerationStructureKHR,
		query_type: QueryType,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
		stride: usize,
	) -> Result,
>;

pub type PFN_vkCmdTraceRaysKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		width: u32,
		height: u32,
		depth: u32,
	) -> (),
>;

pub type PFN_vkCmdTraceRaysNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		raygen_shader_binding_table_buffer: Buffer,
		raygen_shader_binding_offset: DeviceSize,
		miss_shader_binding_table_buffer: Buffer,
		miss_shader_binding_offset: DeviceSize,
		miss_shader_binding_stride: DeviceSize,
		hit_shader_binding_table_buffer: Buffer,
		hit_shader_binding_offset: DeviceSize,
		hit_shader_binding_stride: DeviceSize,
		callable_shader_binding_table_buffer: Buffer,
		callable_shader_binding_offset: DeviceSize,
		callable_shader_binding_stride: DeviceSize,
		width: u32,
		height: u32,
		depth: u32,
	) -> (),
>;

pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		first_group: u32,
		group_count: u32,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;

pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		first_group: u32,
		group_count: u32,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetAccelerationStructureHandleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureNV,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkCreateRayTracingPipelinesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const RayTracingPipelineCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

pub type PFN_vkCreateRayTracingPipelinesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const RayTracingPipelineCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixPropertiesNV,
	) -> Result,
>;

pub type PFN_vkCmdTraceRaysIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
		indirect_device_address: DeviceAddress,
	) -> (),
>;

pub type PFN_vkCmdTraceRaysIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		indirect_device_address: DeviceAddress,
	) -> (),
>;

pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ClusterAccelerationStructureInputInfoNV,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
	) -> (),
>;

pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_version_info: *const AccelerationStructureVersionInfoKHR,
		p_compatibility: *mut AccelerationStructureCompatibilityKHR,
	) -> (),
>;

pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		group: u32,
		group_shader: ShaderGroupShaderKHR,
	) -> DeviceSize,
>;

pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32) -> ()>;

pub type PFN_vkGetImageViewHandleNVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *const ImageViewHandleInfoNVX) -> u32,
>;

pub type PFN_vkGetImageViewHandle64NVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *const ImageViewHandleInfoNVX) -> u64,
>;

pub type PFN_vkGetImageViewAddressNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_view: ImageView,
		p_properties: *mut ImageViewAddressPropertiesNVX,
	) -> Result,
>;

pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, image_view_index: u64, sampler_index: u64) -> u64,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
		p_present_mode_count: *mut u32,
		p_present_modes: *mut PresentModeKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
		p_modes: *mut DeviceGroupPresentModeFlagsKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

#[cfg(feature = "platform-win32")]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_counter_count: *mut u32,
		p_counters: *mut PerformanceCounterKHR,
		p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
		p_num_passes: *mut u32,
	) -> (),
>;

pub type PFN_vkAcquireProfilingLockKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AcquireProfilingLockInfoKHR,
	) -> Result,
>;

pub type PFN_vkReleaseProfilingLockKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
	) -> Result,
>;

pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *const BufferDeviceAddressInfo) -> u64,
>;

pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;

pub type PFN_vkGetBufferDeviceAddress = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferDeviceAddressInfo,
	) -> DeviceAddress,
>;

pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;

pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;

pub type PFN_vkCreateHeadlessSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const HeadlessSurfaceCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_combination_count: *mut u32,
		p_combinations: *mut FramebufferMixedSamplesCombinationNV,
	) -> Result,
>;

pub type PFN_vkInitializePerformanceApiINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_initialize_info: *const InitializePerformanceApiInfoINTEL,
	) -> Result,
>;

pub type PFN_vkUninitializePerformanceApiINTEL =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const PerformanceMarkerInfoINTEL,
	) -> Result,
>;

pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
	) -> Result,
>;

pub type PFN_vkCmdSetPerformanceOverrideINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_override_info: *const PerformanceOverrideInfoINTEL,
	) -> Result,
>;

pub type PFN_vkAcquirePerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
		p_configuration: *mut PerformanceConfigurationINTEL,
	) -> Result,
>;

pub type PFN_vkReleasePerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		configuration: PerformanceConfigurationINTEL,
	) -> Result,
>;

pub type PFN_vkQueueSetPerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result,
>;

pub type PFN_vkGetPerformanceParameterINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		parameter: PerformanceParameterTypeINTEL,
		p_value: *mut PerformanceValueINTEL,
	) -> Result,
>;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
	) -> u64,
>;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = PFN_vkGetDeviceMemoryOpaqueCaptureAddress;

pub type PFN_vkGetPipelineExecutablePropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const PipelineInfoKHR,
		p_executable_count: *mut u32,
		p_properties: *mut PipelineExecutablePropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPipelineExecutableStatisticsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_executable_info: *const PipelineExecutableInfoKHR,
		p_statistic_count: *mut u32,
		p_statistics: *mut PipelineExecutableStatisticKHR,
	) -> Result,
>;

pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_executable_info: *const PipelineExecutableInfoKHR,
		p_internal_representation_count: *mut u32,
		p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
	) -> Result,
>;

pub type PFN_vkCmdSetLineStipple = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		line_stipple_factor: u32,
		line_stipple_pattern: u16,
	) -> (),
>;

pub type PFN_vkCmdSetLineStippleKHR = PFN_vkCmdSetLineStipple;

pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;

pub type PFN_vkGetPhysicalDeviceToolProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_tool_count: *mut u32,
		p_tool_properties: *mut PhysicalDeviceToolProperties,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = PFN_vkGetPhysicalDeviceToolProperties;

pub type PFN_vkCreateAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureKHR,
	) -> Result,
>;

pub type PFN_vkCmdBuildAccelerationStructuresKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
		pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
		p_indirect_device_addresses: *const DeviceAddress,
		p_indirect_strides: *const u32,
		pp_max_primitive_counts: *const *const u32,
	) -> (),
>;

pub type PFN_vkBuildAccelerationStructuresKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
		pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
	) -> Result,
>;

pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureDeviceAddressInfoKHR,
	) -> DeviceAddress,
>;

pub type PFN_vkCreateDeferredOperationKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocator: *const AllocationCallbacks,
		p_deferred_operation: *mut DeferredOperationKHR,
	) -> Result,
>;

pub type PFN_vkDestroyDeferredOperationKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		operation: DeferredOperationKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> u32>;

pub type PFN_vkGetDeferredOperationResultKHR = Option<
	unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> Result,
>;

pub type PFN_vkDeferredOperationJoinKHR = Option<
	unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> Result,
>;

pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ComputePipelineCreateInfo,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkGetPipelineIndirectDeviceAddressNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PipelineIndirectDeviceAddressInfoNV,
	) -> DeviceAddress,
>;

pub type PFN_vkAntiLagUpdateAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_data: *const AntiLagDataAMD) -> ()>;

pub type PFN_vkCmdSetCullMode =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags) -> ()>;

pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;

pub type PFN_vkCmdSetFrontFace =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace) -> ()>;

pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;

pub type PFN_vkCmdSetPrimitiveTopology = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		primitive_topology: PrimitiveTopology,
	) -> (),
>;

pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;

pub type PFN_vkCmdSetViewportWithCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		viewport_count: u32,
		p_viewports: *const Viewport,
	) -> (),
>;

pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;

pub type PFN_vkCmdSetScissorWithCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scissor_count: u32,
		p_scissors: *const Rect2D,
	) -> (),
>;

pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;

pub type PFN_vkCmdBindIndexBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		size: DeviceSize,
		index_type: IndexType,
	) -> (),
>;

pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;

pub type PFN_vkCmdBindVertexBuffers2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_buffers: *const Buffer,
		p_offsets: *const DeviceSize,
		p_sizes: *const DeviceSize,
		p_strides: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;

pub type PFN_vkCmdSetDepthTestEnable =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32) -> ()>;

pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;

pub type PFN_vkCmdSetDepthWriteEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;

pub type PFN_vkCmdSetDepthCompareOp = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp) -> (),
>;

pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;

pub type PFN_vkCmdSetDepthBoundsTestEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;

pub type PFN_vkCmdSetStencilTestEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;

pub type PFN_vkCmdSetStencilOp = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		fail_op: StencilOp,
		pass_op: StencilOp,
		depth_fail_op: StencilOp,
		compare_op: CompareOp,
	) -> (),
>;

pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;

pub type PFN_vkCmdSetPatchControlPointsEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32) -> ()>;

pub type PFN_vkCmdSetRasterizerDiscardEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;

pub type PFN_vkCmdSetDepthBiasEnable =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32) -> ()>;

pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;

pub type PFN_vkCmdSetLogicOpEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp) -> ()>;

pub type PFN_vkCmdSetPrimitiveRestartEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;

pub type PFN_vkCmdSetTessellationDomainOriginEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		domain_origin: TessellationDomainOrigin,
	) -> (),
>;

pub type PFN_vkCmdSetDepthClampEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetPolygonModeEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode) -> ()>;

pub type PFN_vkCmdSetRasterizationSamplesEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		rasterization_samples: SampleCountFlagBits,
	) -> (),
>;

pub type PFN_vkCmdSetSampleMaskEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		samples: SampleCountFlagBits,
		p_sample_mask: *const SampleMask,
	) -> (),
>;

pub type PFN_vkCmdSetAlphaToCoverageEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetAlphaToOneEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetLogicOpEnableEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32) -> ()>;

pub type PFN_vkCmdSetColorBlendEnableEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_enables: *const Bool32,
	) -> (),
>;

pub type PFN_vkCmdSetColorBlendEquationEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_equations: *const ColorBlendEquationEXT,
	) -> (),
>;

pub type PFN_vkCmdSetColorWriteMaskEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_write_masks: *const ColorComponentFlags,
	) -> (),
>;

pub type PFN_vkCmdSetRasterizationStreamEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32) -> ()>;

pub type PFN_vkCmdSetConservativeRasterizationModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		conservative_rasterization_mode: ConservativeRasterizationModeEXT,
	) -> (),
>;

pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		extra_primitive_overestimation_size: f32,
	) -> (),
>;

pub type PFN_vkCmdSetDepthClipEnableEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32) -> ()>;

pub type PFN_vkCmdSetSampleLocationsEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetColorBlendAdvancedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_advanced: *const ColorBlendAdvancedEXT,
	) -> (),
>;

pub type PFN_vkCmdSetProvokingVertexModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		provoking_vertex_mode: ProvokingVertexModeEXT,
	) -> (),
>;

pub type PFN_vkCmdSetLineRasterizationModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		line_rasterization_mode: LineRasterizationModeEXT,
	) -> (),
>;

pub type PFN_vkCmdSetLineStippleEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32) -> (),
>;

pub type PFN_vkCmdSetViewportWScalingEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetViewportSwizzleNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewport_swizzles: *const ViewportSwizzleNV,
	) -> (),
>;

pub type PFN_vkCmdSetCoverageToColorEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetCoverageToColorLocationNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32) -> (),
>;

pub type PFN_vkCmdSetCoverageModulationModeNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_mode: CoverageModulationModeNV,
	) -> (),
>;

pub type PFN_vkCmdSetCoverageModulationTableEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_table_enable: Bool32,
	) -> (),
>;

pub type PFN_vkCmdSetCoverageModulationTableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_table_count: u32,
		p_coverage_modulation_table: *const f32,
	) -> (),
>;

pub type PFN_vkCmdSetShadingRateImageEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32) -> (),
>;

pub type PFN_vkCmdSetCoverageReductionModeNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_reduction_mode: CoverageReductionModeNV,
	) -> (),
>;

pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		representative_fragment_test_enable: Bool32,
	) -> (),
>;

pub type PFN_vkCreatePrivateDataSlot = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PrivateDataSlotCreateInfo,
		p_allocator: *const AllocationCallbacks,
		p_private_data_slot: *mut PrivateDataSlot,
	) -> Result,
>;

pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;

pub type PFN_vkDestroyPrivateDataSlot = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		private_data_slot: PrivateDataSlot,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;

pub type PFN_vkSetPrivateData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		object_type: ObjectType,
		object_handle: u64,
		private_data_slot: PrivateDataSlot,
		data: u64,
	) -> Result,
>;

pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;

pub type PFN_vkGetPrivateData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		object_type: ObjectType,
		object_handle: u64,
		private_data_slot: PrivateDataSlot,
		p_data: *mut u64,
	) -> (),
>;

pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;

pub type PFN_vkCmdCopyBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_buffer_info: *const CopyBufferInfo2,
	) -> (),
>;

pub type PFN_vkCmdCopyBuffer2KHR = PFN_vkCmdCopyBuffer2;

pub type PFN_vkCmdCopyImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_image_info: *const CopyImageInfo2,
	) -> (),
>;

pub type PFN_vkCmdCopyImage2KHR = PFN_vkCmdCopyImage2;

pub type PFN_vkCmdBlitImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_blit_image_info: *const BlitImageInfo2,
	) -> (),
>;

pub type PFN_vkCmdBlitImage2KHR = PFN_vkCmdBlitImage2;

pub type PFN_vkCmdCopyBufferToImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
	) -> (),
>;

pub type PFN_vkCmdCopyBufferToImage2KHR = PFN_vkCmdCopyBufferToImage2;

pub type PFN_vkCmdCopyImageToBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
	) -> (),
>;

pub type PFN_vkCmdCopyImageToBuffer2KHR = PFN_vkCmdCopyImageToBuffer2;

pub type PFN_vkCmdResolveImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_resolve_image_info: *const ResolveImageInfo2,
	) -> (),
>;

pub type PFN_vkCmdResolveImage2KHR = PFN_vkCmdResolveImage2;

pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_fragment_size: *const Extent2D,
		combiner_ops: *const FragmentShadingRateCombinerOpKHR,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_fragment_shading_rate_count: *mut u32,
		p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
	) -> Result,
>;

pub type PFN_vkCmdSetFragmentShadingRateEnumNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		shading_rate: FragmentShadingRateNV,
		combiner_ops: *const FragmentShadingRateCombinerOpKHR,
	) -> (),
>;

pub type PFN_vkGetAccelerationStructureBuildSizesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		build_type: AccelerationStructureBuildTypeKHR,
		p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
		p_max_primitive_counts: *const u32,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdSetVertexInputEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		vertex_binding_description_count: u32,
		p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
		vertex_attribute_description_count: u32,
		p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
	) -> (),
>;

pub type PFN_vkCmdSetColorWriteEnableEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		attachment_count: u32,
		p_color_write_enables: *const Bool32,
	) -> (),
>;

pub type PFN_vkCmdSetEvent2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		p_dependency_info: *const DependencyInfo,
	) -> (),
>;

pub type PFN_vkCmdSetEvent2KHR = PFN_vkCmdSetEvent2;

pub type PFN_vkCmdResetEvent2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags2,
	) -> (),
>;

pub type PFN_vkCmdResetEvent2KHR = PFN_vkCmdResetEvent2;

pub type PFN_vkCmdWaitEvents2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event_count: u32,
		p_events: *const Event,
		p_dependency_infos: *const DependencyInfo,
	) -> (),
>;

pub type PFN_vkCmdWaitEvents2KHR = PFN_vkCmdWaitEvents2;

pub type PFN_vkCmdPipelineBarrier2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dependency_info: *const DependencyInfo,
	) -> (),
>;

pub type PFN_vkCmdPipelineBarrier2KHR = PFN_vkCmdPipelineBarrier2;

pub type PFN_vkQueueSubmit2 = Option<
	unsafe extern "system" fn(
		queue: Queue,
		submit_count: u32,
		p_submits: *const SubmitInfo2,
		fence: Fence,
	) -> Result,
>;

pub type PFN_vkQueueSubmit2KHR = PFN_vkQueueSubmit2;

pub type PFN_vkCmdWriteTimestamp2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage: PipelineStageFlags2,
		query_pool: QueryPool,
		query: u32,
	) -> (),
>;

pub type PFN_vkCmdWriteTimestamp2KHR = PFN_vkCmdWriteTimestamp2;

pub type PFN_vkCmdWriteBufferMarker2AMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage: PipelineStageFlags2,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		marker: u32,
	) -> (),
>;

pub type PFN_vkGetQueueCheckpointData2NV = Option<
	unsafe extern "system" fn(
		queue: Queue,
		p_checkpoint_data_count: *mut u32,
		p_checkpoint_data: *mut CheckpointData2NV,
	) -> (),
>;

pub type PFN_vkCopyMemoryToImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_memory_to_image_info: *const CopyMemoryToImageInfo,
	) -> Result,
>;

pub type PFN_vkCopyMemoryToImageEXT = PFN_vkCopyMemoryToImage;

pub type PFN_vkCopyImageToMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_image_to_memory_info: *const CopyImageToMemoryInfo,
	) -> Result,
>;

pub type PFN_vkCopyImageToMemoryEXT = PFN_vkCopyImageToMemory;

pub type PFN_vkCopyImageToImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_image_to_image_info: *const CopyImageToImageInfo,
	) -> Result,
>;

pub type PFN_vkCopyImageToImageEXT = PFN_vkCopyImageToImage;

pub type PFN_vkTransitionImageLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		transition_count: u32,
		p_transitions: *const HostImageLayoutTransitionInfo,
	) -> Result,
>;

pub type PFN_vkTransitionImageLayoutEXT = PFN_vkTransitionImageLayout;

pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_video_profile: *const VideoProfileInfoKHR,
		p_capabilities: *mut VideoCapabilitiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
		p_video_format_property_count: *mut u32,
		p_video_format_properties: *mut VideoFormatPropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
		p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
	) -> Result,
>;

pub type PFN_vkCreateVideoSessionKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const VideoSessionCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_video_session: *mut VideoSessionKHR,
	) -> Result,
>;

pub type PFN_vkDestroyVideoSessionKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const VideoSessionParametersCreateInfoKHR,
		p_allocator: *const AllocationCallbacks,
		p_video_session_parameters: *mut VideoSessionParametersKHR,
	) -> Result,
>;

pub type PFN_vkUpdateVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session_parameters: VideoSessionParametersKHR,
		p_update_info: *const VideoSessionParametersUpdateInfoKHR,
	) -> Result,
>;

pub type PFN_vkGetEncodedVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
		p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkDestroyVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session_parameters: VideoSessionParametersKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		p_memory_requirements_count: *mut u32,
		p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
	) -> Result,
>;

pub type PFN_vkBindVideoSessionMemoryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		bind_session_memory_info_count: u32,
		p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
	) -> Result,
>;

pub type PFN_vkCmdDecodeVideoKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_decode_info: *const VideoDecodeInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdBeginVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_info: *const VideoBeginCodingInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdControlVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_coding_control_info: *const VideoCodingControlInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdEndVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_end_coding_info: *const VideoEndCodingInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdEncodeVideoKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_encode_info: *const VideoEncodeInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDecompressMemoryNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		decompress_region_count: u32,
		p_decompress_memory_regions: *const DecompressMemoryRegionNV,
	) -> (),
>;

pub type PFN_vkCmdDecompressMemoryIndirectCountNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		indirect_commands_address: DeviceAddress,
		indirect_commands_count_address: DeviceAddress,
		stride: u32,
	) -> (),
>;

pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PartitionedAccelerationStructureInstancesInputNV,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_build_info: *const BuildPartitionedAccelerationStructureInfoNV,
	) -> (),
>;

pub type PFN_vkCmdDecompressMemoryEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		decompression_method: MemoryDecompressionMethodFlagsEXT,
		indirect_commands_address: DeviceAddress,
		indirect_commands_count_address: DeviceAddress,
		max_decompression_count: u32,
		stride: u32,
	) -> (),
>;

pub type PFN_vkCreateCuModuleNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CuModuleCreateInfoNVX,
		p_allocator: *const AllocationCallbacks,
		p_module: *mut CuModuleNVX,
	) -> Result,
>;

pub type PFN_vkCreateCuFunctionNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CuFunctionCreateInfoNVX,
		p_allocator: *const AllocationCallbacks,
		p_function: *mut CuFunctionNVX,
	) -> Result,
>;

pub type PFN_vkDestroyCuModuleNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CuModuleNVX,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkDestroyCuFunctionNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		function: CuFunctionNVX,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCmdCuLaunchKernelNVX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_launch_info: *const CuLaunchInfoNVX,
	) -> (),
>;

pub type PFN_vkGetDescriptorSetLayoutSizeEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		layout: DescriptorSetLayout,
		p_layout_size_in_bytes: *mut DeviceSize,
	) -> (),
>;

pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		layout: DescriptorSetLayout,
		binding: u32,
		p_offset: *mut DeviceSize,
	) -> (),
>;

pub type PFN_vkGetDescriptorEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_descriptor_info: *const DescriptorGetInfoEXT,
		data_size: usize,
		p_descriptor: *mut std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCmdBindDescriptorBuffersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer_count: u32,
		p_binding_infos: *const DescriptorBufferBindingInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		first_set: u32,
		set_count: u32,
		p_buffer_indices: *const u32,
		p_offsets: *const DeviceSize,
	) -> (),
>;

pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		set: u32,
	) -> (),
>;

pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferCaptureDescriptorDataInfoEXT,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageCaptureDescriptorDataInfoEXT,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageViewCaptureDescriptorDataInfoEXT,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const SamplerCaptureDescriptorDataInfoEXT,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkSetDeviceMemoryPriorityEXT = Option<
	unsafe extern "system" fn(device: DeviceHandle, memory: DeviceMemory, priority: f32) -> (),
>;

pub type PFN_vkAcquireDrmDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		drm_fd: i32,
		display: DisplayKHR,
	) -> Result,
>;

pub type PFN_vkGetDrmDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		drm_fd: i32,
		connector_id: u32,
		display: *mut DisplayKHR,
	) -> Result,
>;

pub type PFN_vkWaitForPresent2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_present_wait2_info: *const PresentWait2InfoKHR,
	) -> Result,
>;

pub type PFN_vkWaitForPresentKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		present_id: u64,
		timeout: u64,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
		p_allocator: *const AllocationCallbacks,
		p_collection: *mut BufferCollectionFUCHSIA,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "platform-fuchsia")]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_properties: *mut BufferCollectionPropertiesFUCHSIA,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCreateCudaModuleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CudaModuleCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_module: *mut CudaModuleNV,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkGetCudaModuleCacheNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CudaModuleNV,
		p_cache_size: *mut usize,
		p_cache_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCreateCudaFunctionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CudaFunctionCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_function: *mut CudaFunctionNV,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkDestroyCudaModuleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CudaModuleNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "provisional")]
pub type PFN_vkDestroyCudaFunctionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		function: CudaFunctionNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCmdCudaLaunchKernelNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_launch_info: *const CudaLaunchInfoNV,
	) -> (),
>;

pub type PFN_vkCmdBeginRendering = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_rendering_info: *const RenderingInfo,
	) -> (),
>;

pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;

pub type PFN_vkCmdEndRendering =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkCmdEndRendering2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_rendering_end_info: *const RenderingEndInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;

pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;

pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
		p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
	) -> (),
>;

pub type PFN_vkGetDescriptorSetHostMappingVALVE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set: DescriptorSet,
		pp_data: *mut *mut std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCreateMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const MicromapCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_micromap: *mut MicromapEXT,
	) -> Result,
>;

pub type PFN_vkCmdBuildMicromapsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const MicromapBuildInfoEXT,
	) -> (),
>;

pub type PFN_vkBuildMicromapsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		info_count: u32,
		p_infos: *const MicromapBuildInfoEXT,
	) -> Result,
>;

pub type PFN_vkDestroyMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		micromap: MicromapEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCmdCopyMicromapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMicromapInfoEXT,
	) -> (),
>;

pub type PFN_vkCopyMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMicromapInfoEXT,
	) -> Result,
>;

pub type PFN_vkCmdCopyMicromapToMemoryEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMicromapToMemoryInfoEXT,
	) -> (),
>;

pub type PFN_vkCopyMicromapToMemoryEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMicromapToMemoryInfoEXT,
	) -> Result,
>;

pub type PFN_vkCmdCopyMemoryToMicromapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMemoryToMicromapInfoEXT,
	) -> (),
>;

pub type PFN_vkCopyMemoryToMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMemoryToMicromapInfoEXT,
	) -> Result,
>;

pub type PFN_vkCmdWriteMicromapsPropertiesEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		micromap_count: u32,
		p_micromaps: *const MicromapEXT,
		query_type: QueryType,
		query_pool: QueryPool,
		first_query: u32,
	) -> (),
>;

pub type PFN_vkWriteMicromapsPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		micromap_count: u32,
		p_micromaps: *const MicromapEXT,
		query_type: QueryType,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
		stride: usize,
	) -> Result,
>;

pub type PFN_vkGetDeviceMicromapCompatibilityEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_version_info: *const MicromapVersionInfoEXT,
		p_compatibility: *mut AccelerationStructureCompatibilityKHR,
	) -> (),
>;

pub type PFN_vkGetMicromapBuildSizesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		build_type: AccelerationStructureBuildTypeKHR,
		p_build_info: *const MicromapBuildInfoEXT,
		p_size_info: *mut MicromapBuildSizesInfoEXT,
	) -> (),
>;

pub type PFN_vkGetShaderModuleIdentifierEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader_module: ShaderModule,
		p_identifier: *mut ShaderModuleIdentifierEXT,
	) -> (),
>;

pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderModuleCreateInfo,
		p_identifier: *mut ShaderModuleIdentifierEXT,
	) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_subresource: *const ImageSubresource2,
		p_layout: *mut SubresourceLayout2,
	) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout2KHR = PFN_vkGetImageSubresourceLayout2;

pub type PFN_vkGetImageSubresourceLayout2EXT = PFN_vkGetImageSubresourceLayout2;

pub type PFN_vkGetPipelinePropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const PipelineInfoKHR,
		p_pipeline_properties: *mut BaseOutStructure,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
pub type PFN_vkExportMetalObjectsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_metal_objects_info: *mut ExportMetalObjectsInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdBindTileMemoryQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
	) -> (),
>;

pub type PFN_vkGetFramebufferTilePropertiesQCOM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		framebuffer: Framebuffer,
		p_properties_count: *mut u32,
		p_properties: *mut TilePropertiesQCOM,
	) -> Result,
>;

pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_rendering_info: *const RenderingInfo,
		p_properties: *mut TilePropertiesQCOM,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
		p_format_count: *mut u32,
		p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
	) -> Result,
>;

pub type PFN_vkCreateOpticalFlowSessionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const OpticalFlowSessionCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_session: *mut OpticalFlowSessionNV,
	) -> Result,
>;

pub type PFN_vkDestroyOpticalFlowSessionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: OpticalFlowSessionNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkBindOpticalFlowSessionImageNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: OpticalFlowSessionNV,
		binding_point: OpticalFlowSessionBindingPointNV,
		view: ImageView,
		layout: ImageLayout,
	) -> Result,
>;

pub type PFN_vkCmdOpticalFlowExecuteNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		session: OpticalFlowSessionNV,
		p_execute_info: *const OpticalFlowExecuteInfoNV,
	) -> (),
>;

pub type PFN_vkGetDeviceFaultInfoEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_fault_counts: *mut DeviceFaultCountsEXT,
		p_fault_info: *mut DeviceFaultInfoEXT,
	) -> Result,
>;

pub type PFN_vkGetDeviceFaultReportsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		timeout: u64,
		p_fault_counts: *mut u32,
		p_fault_info: *mut DeviceFaultInfoKHR,
	) -> Result,
>;

pub type PFN_vkGetDeviceFaultDebugInfoKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_debug_info: *mut DeviceFaultDebugInfoKHR,
	) -> Result,
>;

pub type PFN_vkCmdSetDepthBias2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_depth_bias_info: *const DepthBiasInfoEXT,
	) -> (),
>;

pub type PFN_vkReleaseSwapchainImagesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_release_info: *const ReleaseSwapchainImagesInfoKHR,
	) -> Result,
>;

pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;

pub type PFN_vkGetDeviceImageSubresourceLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageSubresourceInfo,
		p_layout: *mut SubresourceLayout2,
	) -> (),
>;

pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = PFN_vkGetDeviceImageSubresourceLayout;

pub type PFN_vkMapMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_map_info: *const MemoryMapInfo,
		pp_data: *mut *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;

pub type PFN_vkUnmapMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_unmap_info: *const MemoryUnmapInfo,
	) -> Result,
>;

pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;

pub type PFN_vkCreateShadersEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		create_info_count: u32,
		p_create_infos: *const ShaderCreateInfoEXT,
		p_allocator: *const AllocationCallbacks,
		p_shaders: *mut ShaderEXT,
	) -> Result,
>;

pub type PFN_vkDestroyShaderEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader: ShaderEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetShaderBinaryDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader: ShaderEXT,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkCmdBindShadersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage_count: u32,
		p_stages: *const ShaderStageFlagBits,
		p_shaders: *const ShaderEXT,
	) -> (),
>;

pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT = Option<
	unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR, size: u32) -> Result,
>;

pub type PFN_vkGetSwapchainTimingPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
		p_swapchain_timing_properties_counter: *mut u64,
	) -> Result,
>;

pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
		p_time_domains_counter: *mut u64,
	) -> Result,
>;

pub type PFN_vkGetPastPresentationTimingEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
		p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
pub type PFN_vkGetScreenBufferPropertiesQNX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const _screen_buffer,
		p_properties: *mut ScreenBufferPropertiesQNX,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixPropertiesKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixProperties2EXT,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		execution_graph: Pipeline,
		p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		execution_graph: Pipeline,
		p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
		p_node_index: *mut u32,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		execution_graph: Pipeline,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
	) -> (),
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCmdDispatchGraphAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		p_count_info: *const DispatchGraphCountInfoAMDX,
	) -> (),
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		p_count_info: *const DispatchGraphCountInfoAMDX,
	) -> (),
>;

#[cfg(feature = "provisional")]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		count_info: DeviceAddress,
	) -> (),
>;

pub type PFN_vkCreateGpaSessionAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const GpaSessionCreateInfoAMD,
		p_allocator: *const AllocationCallbacks,
		p_gpa_session: *mut GpaSessionAMD,
	) -> Result,
>;

pub type PFN_vkDestroyGpaSessionAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		gpa_session: GpaSessionAMD,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkSetGpaDeviceClockModeAMD = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *mut GpaDeviceClockModeInfoAMD) -> Result,
>;

pub type PFN_vkGetGpaDeviceClockInfoAMD = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *mut GpaDeviceGetClockInfoAMD) -> Result,
>;

pub type PFN_vkCmdBeginGpaSessionAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result,
>;

pub type PFN_vkCmdEndGpaSessionAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result,
>;

pub type PFN_vkCmdBeginGpaSampleAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		gpa_session: GpaSessionAMD,
		p_gpa_sample_begin_info: *const GpaSampleBeginInfoAMD,
		p_sample_id: *mut u32,
	) -> Result,
>;

pub type PFN_vkCmdEndGpaSampleAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		gpa_session: GpaSessionAMD,
		sample_id: u32,
	) -> (),
>;

pub type PFN_vkGetGpaSessionStatusAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, gpa_session: GpaSessionAMD) -> Result>;

pub type PFN_vkGetGpaSessionResultsAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		gpa_session: GpaSessionAMD,
		sample_id: u32,
		p_size_in_bytes: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkResetGpaSessionAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, gpa_session: GpaSessionAMD) -> Result>;

pub type PFN_vkCmdCopyGpaSessionResultsAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> (),
>;

pub type PFN_vkCmdBindDescriptorSets2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
	) -> (),
>;

pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;

pub type PFN_vkCmdPushConstants2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_constants_info: *const PushConstantsInfo,
	) -> (),
>;

pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;

pub type PFN_vkCmdPushDescriptorSet2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_descriptor_set_info: *const PushDescriptorSetInfo,
	) -> (),
>;

pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;

pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
	) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;

pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = Option<unsafe extern "system" fn(command_buffer: CommandBuffer, p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT) -> ()>;

pub type PFN_vkSetLatencySleepModeNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_sleep_mode_info: *const LatencySleepModeInfoNV,
	) -> Result,
>;

pub type PFN_vkLatencySleepNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_sleep_info: *const LatencySleepInfoNV,
	) -> Result,
>;

pub type PFN_vkSetLatencyMarkerNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_latency_marker_info: *const SetLatencyMarkerInfoNV,
	) -> (),
>;

pub type PFN_vkGetLatencyTimingsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_latency_marker_info: *mut GetLatencyMarkerInfoNV,
	) -> (),
>;

pub type PFN_vkQueueNotifyOutOfBandNV = Option<
	unsafe extern "system" fn(queue: Queue, p_queue_type_info: *const OutOfBandQueueTypeInfoNV) -> (),
>;

pub type PFN_vkSetLatencySleepModeLegacyNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		low_latency_mode: Bool32,
		low_latency_boost: Bool32,
		minimum_interval_us: u32,
	) -> (),
>;

pub type PFN_vkLatencySleepLegacyNV = Option<
	unsafe extern "system" fn(device: DeviceHandle, signal_semaphore: Semaphore, value: u64) -> (),
>;

pub type PFN_vkSetLatencyMarkerLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, frame_id: u64, marker: u32) -> ()>;

pub type PFN_vkGetLatencyTimingsLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_timings: *mut std::ffi::c_void) -> ()>;

pub type PFN_vkQueueNotifyOutOfBandLegacyNV =
	Option<unsafe extern "system" fn(queue: Queue, queue_type: u32) -> ()>;

pub type PFN_vkGetSleepStatusLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_low_latency_mode: *mut Bool32) -> ()>;

pub type PFN_vkShutdownLatencyDeviceLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

pub type PFN_vkCmdSetRenderingAttachmentLocations = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_location_info: *const RenderingAttachmentLocationInfo,
	) -> (),
>;

pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;

pub type PFN_vkCmdSetRenderingInputAttachmentIndices = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
	) -> (),
>;

pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
	PFN_vkCmdSetRenderingInputAttachmentIndices;

pub type PFN_vkCmdSetDepthClampRangeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		depth_clamp_mode: DepthClampModeEXT,
		p_depth_clamp_range: *const DepthClampRangeEXT,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
pub type PFN_vkGetMemoryMetalHandleEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
		p_handle: *mut *mut std::ffi::c_void,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		p_handle: *const std::ffi::c_void,
		p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeVectorPropertiesNV,
	) -> Result,
>;

pub type PFN_vkConvertCooperativeVectorMatrixNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ConvertCooperativeVectorMatrixInfoNV,
	) -> Result,
>;

pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const ConvertCooperativeVectorMatrixInfoNV,
	) -> (),
>;

pub type PFN_vkCmdDispatchTileQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dispatch_tile_info: *const DispatchTileInfoQCOM,
	) -> (),
>;

pub type PFN_vkCmdBeginPerTileExecutionQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_per_tile_begin_info: *const PerTileBeginInfoQCOM,
	) -> (),
>;

pub type PFN_vkCmdEndPerTileExecutionQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_per_tile_end_info: *const PerTileEndInfoQCOM,
	) -> (),
>;

pub type PFN_vkCreateExternalComputeQueueNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ExternalComputeQueueCreateInfoNV,
		p_allocator: *const AllocationCallbacks,
		p_external_queue: *mut ExternalComputeQueueNV,
	) -> Result,
>;

pub type PFN_vkDestroyExternalComputeQueueNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		external_queue: ExternalComputeQueueNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetExternalComputeQueueDataNV = Option<
	unsafe extern "system" fn(
		external_queue: ExternalComputeQueueNV,
		params: *mut ExternalComputeQueueDataParamsNV,
		p_data: *mut std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_description_count: *mut u32,
		p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM,
	) -> Result,
>;

pub type PFN_vkCreateShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderInstrumentationCreateInfoARM,
		p_allocator: *const AllocationCallbacks,
		p_instrumentation: *mut ShaderInstrumentationARM,
	) -> Result,
>;

pub type PFN_vkDestroyShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		instrumentation: ShaderInstrumentationARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCmdBeginShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		instrumentation: ShaderInstrumentationARM,
	) -> (),
>;

pub type PFN_vkCmdEndShaderInstrumentationARM =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

pub type PFN_vkGetShaderInstrumentationValuesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		instrumentation: ShaderInstrumentationARM,
		p_metric_block_count: *mut u32,
		p_metric_values: *mut std::ffi::c_void,
		flags: ShaderInstrumentationValuesFlagsARM,
	) -> Result,
>;

pub type PFN_vkClearShaderInstrumentationMetricsARM = Option<
	unsafe extern "system" fn(device: DeviceHandle, instrumentation: ShaderInstrumentationARM) -> (),
>;

pub type PFN_vkCreateTensorARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const TensorCreateInfoARM,
		p_allocator: *const AllocationCallbacks,
		p_tensor: *mut TensorARM,
	) -> Result,
>;

pub type PFN_vkDestroyTensorARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor: TensorARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCreateTensorViewARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const TensorViewCreateInfoARM,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut TensorViewARM,
	) -> Result,
>;

pub type PFN_vkDestroyTensorViewARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor_view: TensorViewARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkGetTensorMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorMemoryRequirementsInfoARM,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkBindTensorMemoryARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindTensorMemoryInfoARM,
	) -> Result,
>;

pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceTensorMemoryRequirementsARM,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkCmdCopyTensorARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_tensor_info: *const CopyTensorInfoARM,
	) -> (),
>;

pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorCaptureDescriptorDataInfoARM,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorViewCaptureDescriptorDataInfoARM,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
		p_external_tensor_properties: *mut ExternalTensorPropertiesARM,
	) -> (),
>;

pub type PFN_vkCreateDataGraphPipelinesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const DataGraphPipelineCreateInfoARM,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

pub type PFN_vkCreateDataGraphPipelineSessionARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DataGraphPipelineSessionCreateInfoARM,
		p_allocator: *const AllocationCallbacks,
		p_session: *mut DataGraphPipelineSessionARM,
	) -> Result,
>;

pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
		p_bind_point_requirement_count: *mut u32,
		p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
	) -> Result,
>;

pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
		p_memory_requirements: *mut MemoryRequirements2,
	) -> (),
>;

pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
	) -> Result,
>;

pub type PFN_vkDestroyDataGraphPipelineSessionARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: DataGraphPipelineSessionARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

pub type PFN_vkCmdDispatchDataGraphARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		session: DataGraphPipelineSessionARM,
		p_info: *const DataGraphPipelineDispatchInfoARM,
	) -> (),
>;

pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const DataGraphPipelineInfoARM,
		p_properties_count: *mut u32,
		p_properties: *mut DataGraphPipelinePropertyARM,
	) -> Result,
>;

pub type PFN_vkGetDataGraphPipelinePropertiesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const DataGraphPipelineInfoARM,
		properties_count: u32,
		p_properties: *mut DataGraphPipelinePropertyQueryResultARM,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_property_count: *mut u32,
		p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = Option<unsafe extern "system" fn(physical_device: PhysicalDevice, p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM, p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM) -> ()>;

#[cfg(feature = "platform-ohos")]
pub type PFN_vkGetNativeBufferPropertiesOHOS = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const OH_NativeBuffer,
		p_properties: *mut NativeBufferPropertiesOHOS,
	) -> Result,
>;

#[cfg(feature = "platform-ohos")]
pub type PFN_vkGetMemoryNativeBufferOHOS = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const MemoryGetNativeBufferInfoOHOS,
		p_buffer: *mut *mut OH_NativeBuffer,
	) -> Result,
>;

pub type PFN_vkQueueSetPerfHintQCOM = Option<
	unsafe extern "system" fn(queue: Queue, p_perf_hint_info: *const PerfHintInfoQCOM) -> Result,
>;

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_counter_count: *mut u32,
		p_counters: *mut PerformanceCounterARM,
		p_counter_descriptions: *mut PerformanceCounterDescriptionARM,
	) -> Result,
>;

pub type PFN_vkCmdSetComputeOccupancyPriorityNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_parameters: *const ComputeOccupancyPriorityParametersNV,
	) -> (),
>;

pub type PFN_vkWriteSamplerDescriptorsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		sampler_count: u32,
		p_samplers: *const SamplerCreateInfo,
		p_descriptors: *const HostAddressRangeEXT,
	) -> Result,
>;

pub type PFN_vkWriteResourceDescriptorsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		resource_count: u32,
		p_resources: *const ResourceDescriptorInfoEXT,
		p_descriptors: *const HostAddressRangeEXT,
	) -> Result,
>;

pub type PFN_vkCmdBindSamplerHeapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_info: *const BindHeapInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdBindResourceHeapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_info: *const BindHeapInfoEXT,
	) -> (),
>;

pub type PFN_vkCmdPushDataEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_data_info: *const PushDataInfoEXT,
	) -> (),
>;

pub type PFN_vkRegisterCustomBorderColorEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_border_color: *const SamplerCustomBorderColorCreateInfoEXT,
		request_index: Bool32,
		p_index: *mut u32,
	) -> Result,
>;

pub type PFN_vkUnregisterCustomBorderColorEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, index: u32) -> ()>;

pub type PFN_vkGetImageOpaqueCaptureDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_count: u32,
		p_images: *const Image,
		p_datas: *mut HostAddressRangeEXT,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		descriptor_type: DescriptorType,
	) -> DeviceSize,
>;

pub type PFN_vkGetTensorOpaqueCaptureDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor_count: u32,
		p_tensors: *const TensorARM,
		p_datas: *mut HostAddressRangeEXT,
	) -> Result,
>;

pub type PFN_vkCmdCopyMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdCopyImageToMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
	) -> (),
>;

pub type PFN_vkCmdUpdateMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dst_range: *const DeviceAddressRangeKHR,
		dst_flags: AddressCommandFlagsKHR,
		data_size: DeviceSize,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

pub type PFN_vkCmdFillMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dst_range: *const DeviceAddressRangeKHR,
		dst_flags: AddressCommandFlagsKHR,
		data: u32,
	) -> (),
>;

pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
		p_dst_range: *const StridedDeviceAddressRangeKHR,
		dst_flags: AddressCommandFlagsKHR,
		query_result_flags: QueryResultFlags,
	) -> (),
>;

pub type PFN_vkCmdBeginConditionalRendering2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT,
	) -> (),
>;

pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT,
	) -> (),
>;

pub type PFN_vkCmdBeginTransformFeedback2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_range: u32,
		counter_range_count: u32,
		p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
	) -> (),
>;

pub type PFN_vkCmdEndTransformFeedback2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_range: u32,
		counter_range_count: u32,
		p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirectByteCount2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		instance_count: u32,
		first_instance: u32,
		p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT,
		counter_offset: u32,
		vertex_stride: u32,
	) -> (),
>;

pub type PFN_vkCmdWriteMarkerToMemoryAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const MemoryMarkerInfoAMD,
	) -> (),
>;

pub type PFN_vkCmdBindIndexBuffer3KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const BindIndexBuffer3InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers3KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_binding_infos: *const BindVertexBuffer3InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawIndexedIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawIndirectCount2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirect2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR,
	) -> (),
>;

pub type PFN_vkCmdDispatchIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DispatchIndirect2InfoKHR,
	) -> (),
>;

pub type PFN_vkCreateAccelerationStructure2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfo2KHR,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureKHR,
	) -> Result,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
		p_properties: *mut BaseOutStructure,
	) -> Result,
>;

pub type PFN_vkCmdSetDispatchParametersARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dispatch_parameters: *const DispatchParametersARM,
	) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
		p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM,
		p_format_count: *mut u32,
		p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM,
	) -> Result,
>;
