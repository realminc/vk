// @generated — do not edit by hand.
#![allow(non_camel_case_types)]
use super::{enums::*, types::*};

#[doc = "Nullable native callback `PFN_vkInternalAllocationNotification`."]
#[doc = "\n\n[Khronos reference: `PFN_vkInternalAllocationNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html)"]
pub type PFN_vkInternalAllocationNotification = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		allocation_type: InternalAllocationType,
		allocation_scope: SystemAllocationScope,
	) -> (),
>;

#[doc = "Nullable native callback `PFN_vkInternalFreeNotification`."]
#[doc = "\n\n[Khronos reference: `PFN_vkInternalFreeNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html)"]
pub type PFN_vkInternalFreeNotification = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		allocation_type: InternalAllocationType,
		allocation_scope: SystemAllocationScope,
	) -> (),
>;

#[doc = "Nullable native callback `PFN_vkReallocationFunction`."]
#[doc = "\n\n[Khronos reference: `PFN_vkReallocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html)"]
pub type PFN_vkReallocationFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		p_original: *mut std::ffi::c_void,
		size: usize,
		alignment: usize,
		allocation_scope: SystemAllocationScope,
	) -> *mut std::ffi::c_void,
>;

#[doc = "Nullable native callback `PFN_vkAllocationFunction`."]
#[doc = "\n\n[Khronos reference: `PFN_vkAllocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html)"]
pub type PFN_vkAllocationFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		size: usize,
		alignment: usize,
		allocation_scope: SystemAllocationScope,
	) -> *mut std::ffi::c_void,
>;

#[doc = "Nullable native callback `PFN_vkFreeFunction`."]
#[doc = "\n\n[Khronos reference: `PFN_vkFreeFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html)"]
pub type PFN_vkFreeFunction = Option<
	unsafe extern "system" fn(
		p_user_data: *mut std::ffi::c_void,
		p_memory: *mut std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable native callback `PFN_vkVoidFunction`."]
#[doc = "\n\n[Khronos reference: `PFN_vkVoidFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html)"]
pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn() -> ()>;

#[doc = "Nullable native callback `PFN_vkDebugReportCallbackEXT`."]
#[doc = "\n\n[Khronos reference: `PFN_vkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugReportCallbackEXT.html)"]
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

#[doc = "Nullable native callback `PFN_vkDebugUtilsMessengerCallbackEXT`."]
#[doc = "\n\n[Khronos reference: `PFN_vkDebugUtilsMessengerCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugUtilsMessengerCallbackEXT.html)"]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<
	unsafe extern "system" fn(
		message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
		message_types: DebugUtilsMessageTypeFlagsEXT,
		p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
		p_user_data: *mut std::ffi::c_void,
	) -> Bool32,
>;

#[doc = "Nullable native callback `PFN_vkDeviceMemoryReportCallbackEXT`."]
#[doc = "\n\n[Khronos reference: `PFN_vkDeviceMemoryReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDeviceMemoryReportCallbackEXT.html)"]
pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<
	unsafe extern "system" fn(
		p_callback_data: *const DeviceMemoryReportCallbackDataEXT<'_>,
		p_user_data: *mut std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable native callback `PFN_vkGetInstanceProcAddrLUNARG`."]
#[doc = "\n\n[Khronos reference: `PFN_vkGetInstanceProcAddrLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html)"]
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

#[doc = "Nullable function pointer for `vkCreateInstance`."]
#[doc(alias = "vkCreateInstance")]
#[doc = "\n\n[Khronos reference: `vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html)"]
pub type PFN_vkCreateInstance = Option<
	unsafe extern "system" fn(
		p_create_info: *const InstanceCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_instance: *mut InstanceHandle,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyInstance`."]
#[doc(alias = "vkDestroyInstance")]
#[doc = "\n\n[Khronos reference: `vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html)"]
pub type PFN_vkDestroyInstance = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDevices`."]
#[doc(alias = "vkEnumeratePhysicalDevices")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html)"]
pub type PFN_vkEnumeratePhysicalDevices = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_physical_device_count: *mut u32,
		p_physical_devices: *mut PhysicalDevice,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceProcAddr`."]
#[doc(alias = "vkGetDeviceProcAddr")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html)"]
pub type PFN_vkGetDeviceProcAddr = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

#[doc = "Nullable function pointer for `vkGetInstanceProcAddr`."]
#[doc(alias = "vkGetInstanceProcAddr")]
#[doc = "\n\n[Khronos reference: `vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)"]
pub type PFN_vkGetInstanceProcAddr = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_name: *const std::ffi::c_char,
	) -> PFN_vkVoidFunction,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceProperties`."]
#[doc(alias = "vkGetPhysicalDeviceProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html)"]
pub type PFN_vkGetPhysicalDeviceProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_properties: *mut PhysicalDeviceProperties,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyProperties`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_queue_family_property_count: *mut u32,
		p_queue_family_properties: *mut QueueFamilyProperties,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceMemoryProperties`."]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html)"]
pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_memory_properties: *mut PhysicalDeviceMemoryProperties,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFeatures`."]
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html)"]
pub type PFN_vkGetPhysicalDeviceFeatures = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_features: *mut PhysicalDeviceFeatures,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFormatProperties`."]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html)"]
pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		p_format_properties: *mut FormatProperties,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceImageFormatProperties`."]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html)"]
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

#[doc = "Nullable function pointer for `vkCreateDevice`."]
#[doc(alias = "vkCreateDevice")]
#[doc = "\n\n[Khronos reference: `vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html)"]
pub type PFN_vkCreateDevice = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_create_info: *const DeviceCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_device: *mut DeviceHandle,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDevice`."]
#[doc(alias = "vkDestroyDevice")]
#[doc = "\n\n[Khronos reference: `vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html)"]
pub type PFN_vkDestroyDevice = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_allocator: *const AllocationCallbacks) -> (),
>;

#[doc = "Nullable function pointer for `vkEnumerateInstanceVersion`."]
#[doc(alias = "vkEnumerateInstanceVersion")]
#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)"]
pub type PFN_vkEnumerateInstanceVersion =
	Option<unsafe extern "system" fn(p_api_version: *mut u32) -> Result>;

#[doc = "Nullable function pointer for `vkEnumerateInstanceLayerProperties`."]
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)"]
pub type PFN_vkEnumerateInstanceLayerProperties = Option<
	unsafe extern "system" fn(
		p_property_count: *mut u32,
		p_properties: *mut LayerProperties,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumerateInstanceExtensionProperties`."]
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)"]
pub type PFN_vkEnumerateInstanceExtensionProperties = Option<
	unsafe extern "system" fn(
		p_layer_name: *const std::ffi::c_char,
		p_property_count: *mut u32,
		p_properties: *mut ExtensionProperties,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumerateDeviceLayerProperties`."]
#[doc(alias = "vkEnumerateDeviceLayerProperties")]
#[doc = "\n\n[Khronos reference: `vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html)"]
pub type PFN_vkEnumerateDeviceLayerProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut LayerProperties,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumerateDeviceExtensionProperties`."]
#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
#[doc = "\n\n[Khronos reference: `vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html)"]
pub type PFN_vkEnumerateDeviceExtensionProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_layer_name: *const std::ffi::c_char,
		p_property_count: *mut u32,
		p_properties: *mut ExtensionProperties,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceQueue`."]
#[doc(alias = "vkGetDeviceQueue")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html)"]
pub type PFN_vkGetDeviceQueue = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		queue_family_index: u32,
		queue_index: u32,
		p_queue: *mut Queue,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkQueueSubmit`."]
#[doc(alias = "vkQueueSubmit")]
#[doc = "\n\n[Khronos reference: `vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html)"]
pub type PFN_vkQueueSubmit = Option<
	unsafe extern "system" fn(
		queue: Queue,
		submit_count: u32,
		p_submits: *const SubmitInfo<'_>,
		fence: Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkQueueWaitIdle`."]
#[doc(alias = "vkQueueWaitIdle")]
#[doc = "\n\n[Khronos reference: `vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html)"]
pub type PFN_vkQueueWaitIdle = Option<unsafe extern "system" fn(queue: Queue) -> Result>;

#[doc = "Nullable function pointer for `vkDeviceWaitIdle`."]
#[doc(alias = "vkDeviceWaitIdle")]
#[doc = "\n\n[Khronos reference: `vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html)"]
pub type PFN_vkDeviceWaitIdle = Option<unsafe extern "system" fn(device: DeviceHandle) -> Result>;

#[doc = "Nullable function pointer for `vkAllocateMemory`."]
#[doc(alias = "vkAllocateMemory")]
#[doc = "\n\n[Khronos reference: `vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html)"]
pub type PFN_vkAllocateMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const MemoryAllocateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_memory: *mut DeviceMemory,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkFreeMemory`."]
#[doc(alias = "vkFreeMemory")]
#[doc = "\n\n[Khronos reference: `vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html)"]
pub type PFN_vkFreeMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkMapMemory`."]
#[doc(alias = "vkMapMemory")]
#[doc = "\n\n[Khronos reference: `vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html)"]
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

#[doc = "Nullable function pointer for `vkUnmapMemory`."]
#[doc(alias = "vkUnmapMemory")]
#[doc = "\n\n[Khronos reference: `vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html)"]
pub type PFN_vkUnmapMemory =
	Option<unsafe extern "system" fn(device: DeviceHandle, memory: DeviceMemory) -> ()>;

#[doc = "Nullable function pointer for `vkFlushMappedMemoryRanges`."]
#[doc(alias = "vkFlushMappedMemoryRanges")]
#[doc = "\n\n[Khronos reference: `vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html)"]
pub type PFN_vkFlushMappedMemoryRanges = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory_range_count: u32,
		p_memory_ranges: *const MappedMemoryRange<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkInvalidateMappedMemoryRanges`."]
#[doc(alias = "vkInvalidateMappedMemoryRanges")]
#[doc = "\n\n[Khronos reference: `vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html)"]
pub type PFN_vkInvalidateMappedMemoryRanges = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory_range_count: u32,
		p_memory_ranges: *const MappedMemoryRange<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceMemoryCommitment`."]
#[doc(alias = "vkGetDeviceMemoryCommitment")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html)"]
pub type PFN_vkGetDeviceMemoryCommitment = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		p_committed_memory_in_bytes: *mut DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetBufferMemoryRequirements`."]
#[doc(alias = "vkGetBufferMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html)"]
pub type PFN_vkGetBufferMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		p_memory_requirements: *mut MemoryRequirements,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindBufferMemory`."]
#[doc(alias = "vkBindBufferMemory")]
#[doc = "\n\n[Khronos reference: `vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html)"]
pub type PFN_vkBindBufferMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		memory: DeviceMemory,
		memory_offset: DeviceSize,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetImageMemoryRequirements`."]
#[doc(alias = "vkGetImageMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html)"]
pub type PFN_vkGetImageMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_memory_requirements: *mut MemoryRequirements,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindImageMemory`."]
#[doc(alias = "vkBindImageMemory")]
#[doc = "\n\n[Khronos reference: `vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html)"]
pub type PFN_vkBindImageMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		memory: DeviceMemory,
		memory_offset: DeviceSize,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetImageSparseMemoryRequirements`."]
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html)"]
pub type PFN_vkGetImageSparseMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties`."]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html)"]
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

#[doc = "Nullable function pointer for `vkQueueBindSparse`."]
#[doc(alias = "vkQueueBindSparse")]
#[doc = "\n\n[Khronos reference: `vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html)"]
pub type PFN_vkQueueBindSparse = Option<
	unsafe extern "system" fn(
		queue: Queue,
		bind_info_count: u32,
		p_bind_info: *const BindSparseInfo<'_>,
		fence: Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateFence`."]
#[doc(alias = "vkCreateFence")]
#[doc = "\n\n[Khronos reference: `vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html)"]
pub type PFN_vkCreateFence = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const FenceCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyFence`."]
#[doc(alias = "vkDestroyFence")]
#[doc = "\n\n[Khronos reference: `vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html)"]
pub type PFN_vkDestroyFence = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence: Fence,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkResetFences`."]
#[doc(alias = "vkResetFences")]
#[doc = "\n\n[Khronos reference: `vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html)"]
pub type PFN_vkResetFences = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence_count: u32,
		p_fences: *const Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetFenceStatus`."]
#[doc(alias = "vkGetFenceStatus")]
#[doc = "\n\n[Khronos reference: `vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html)"]
pub type PFN_vkGetFenceStatus =
	Option<unsafe extern "system" fn(device: DeviceHandle, fence: Fence) -> Result>;

#[doc = "Nullable function pointer for `vkWaitForFences`."]
#[doc(alias = "vkWaitForFences")]
#[doc = "\n\n[Khronos reference: `vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html)"]
pub type PFN_vkWaitForFences = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		fence_count: u32,
		p_fences: *const Fence,
		wait_all: Bool32,
		timeout: u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateSemaphore`."]
#[doc(alias = "vkCreateSemaphore")]
#[doc = "\n\n[Khronos reference: `vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html)"]
pub type PFN_vkCreateSemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SemaphoreCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_semaphore: *mut Semaphore,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroySemaphore`."]
#[doc(alias = "vkDestroySemaphore")]
#[doc = "\n\n[Khronos reference: `vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html)"]
pub type PFN_vkDestroySemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		semaphore: Semaphore,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateEvent`."]
#[doc(alias = "vkCreateEvent")]
#[doc = "\n\n[Khronos reference: `vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html)"]
pub type PFN_vkCreateEvent = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const EventCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_event: *mut Event,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyEvent`."]
#[doc(alias = "vkDestroyEvent")]
#[doc = "\n\n[Khronos reference: `vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html)"]
pub type PFN_vkDestroyEvent = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		event: Event,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetEventStatus`."]
#[doc(alias = "vkGetEventStatus")]
#[doc = "\n\n[Khronos reference: `vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html)"]
pub type PFN_vkGetEventStatus =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

#[doc = "Nullable function pointer for `vkSetEvent`."]
#[doc(alias = "vkSetEvent")]
#[doc = "\n\n[Khronos reference: `vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html)"]
pub type PFN_vkSetEvent =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

#[doc = "Nullable function pointer for `vkResetEvent`."]
#[doc(alias = "vkResetEvent")]
#[doc = "\n\n[Khronos reference: `vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html)"]
pub type PFN_vkResetEvent =
	Option<unsafe extern "system" fn(device: DeviceHandle, event: Event) -> Result>;

#[doc = "Nullable function pointer for `vkCreateQueryPool`."]
#[doc(alias = "vkCreateQueryPool")]
#[doc = "\n\n[Khronos reference: `vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html)"]
pub type PFN_vkCreateQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const QueryPoolCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_query_pool: *mut QueryPool,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyQueryPool`."]
#[doc(alias = "vkDestroyQueryPool")]
#[doc = "\n\n[Khronos reference: `vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html)"]
pub type PFN_vkDestroyQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		query_pool: QueryPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetQueryPoolResults`."]
#[doc(alias = "vkGetQueryPoolResults")]
#[doc = "\n\n[Khronos reference: `vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html)"]
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

#[doc = "Nullable function pointer for `vkResetQueryPool`."]
#[doc(alias = "vkResetQueryPool")]
#[doc = "\n\n[Khronos reference: `vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)"]
pub type PFN_vkResetQueryPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkResetQueryPoolEXT`."]
#[doc(alias = "vkResetQueryPoolEXT")]
#[doc = "\n\n[Khronos reference: `vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html)"]
pub type PFN_vkResetQueryPoolEXT = PFN_vkResetQueryPool;

#[doc = "Nullable function pointer for `vkCreateBuffer`."]
#[doc(alias = "vkCreateBuffer")]
#[doc = "\n\n[Khronos reference: `vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html)"]
pub type PFN_vkCreateBuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_buffer: *mut Buffer,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyBuffer`."]
#[doc(alias = "vkDestroyBuffer")]
#[doc = "\n\n[Khronos reference: `vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html)"]
pub type PFN_vkDestroyBuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: Buffer,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateBufferView`."]
#[doc(alias = "vkCreateBufferView")]
#[doc = "\n\n[Khronos reference: `vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html)"]
pub type PFN_vkCreateBufferView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferViewCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut BufferView,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyBufferView`."]
#[doc(alias = "vkDestroyBufferView")]
#[doc = "\n\n[Khronos reference: `vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html)"]
pub type PFN_vkDestroyBufferView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer_view: BufferView,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateImage`."]
#[doc(alias = "vkCreateImage")]
#[doc = "\n\n[Khronos reference: `vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html)"]
pub type PFN_vkCreateImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ImageCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_image: *mut Image,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyImage`."]
#[doc(alias = "vkDestroyImage")]
#[doc = "\n\n[Khronos reference: `vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html)"]
pub type PFN_vkDestroyImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetImageSubresourceLayout`."]
#[doc(alias = "vkGetImageSubresourceLayout")]
#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html)"]
pub type PFN_vkGetImageSubresourceLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_subresource: *const ImageSubresource,
		p_layout: *mut SubresourceLayout,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateImageView`."]
#[doc(alias = "vkCreateImageView")]
#[doc = "\n\n[Khronos reference: `vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html)"]
pub type PFN_vkCreateImageView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ImageViewCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut ImageView,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyImageView`."]
#[doc(alias = "vkDestroyImageView")]
#[doc = "\n\n[Khronos reference: `vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html)"]
pub type PFN_vkDestroyImageView = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_view: ImageView,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateShaderModule`."]
#[doc(alias = "vkCreateShaderModule")]
#[doc = "\n\n[Khronos reference: `vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html)"]
pub type PFN_vkCreateShaderModule = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderModuleCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_shader_module: *mut ShaderModule,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyShaderModule`."]
#[doc(alias = "vkDestroyShaderModule")]
#[doc = "\n\n[Khronos reference: `vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)"]
pub type PFN_vkDestroyShaderModule = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader_module: ShaderModule,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreatePipelineCache`."]
#[doc(alias = "vkCreatePipelineCache")]
#[doc = "\n\n[Khronos reference: `vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html)"]
pub type PFN_vkCreatePipelineCache = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineCacheCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipeline_cache: *mut PipelineCache,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyPipelineCache`."]
#[doc(alias = "vkDestroyPipelineCache")]
#[doc = "\n\n[Khronos reference: `vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html)"]
pub type PFN_vkDestroyPipelineCache = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPipelineCacheData`."]
#[doc(alias = "vkGetPipelineCacheData")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html)"]
pub type PFN_vkGetPipelineCacheData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkMergePipelineCaches`."]
#[doc(alias = "vkMergePipelineCaches")]
#[doc = "\n\n[Khronos reference: `vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html)"]
pub type PFN_vkMergePipelineCaches = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		dst_cache: PipelineCache,
		src_cache_count: u32,
		p_src_caches: *const PipelineCache,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreatePipelineBinariesKHR`."]
#[doc(alias = "vkCreatePipelineBinariesKHR")]
#[doc = "\n\n[Khronos reference: `vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html)"]
pub type PFN_vkCreatePipelineBinariesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyPipelineBinaryKHR`."]
#[doc(alias = "vkDestroyPipelineBinaryKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html)"]
pub type PFN_vkDestroyPipelineBinaryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_binary: PipelineBinaryKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPipelineKeyKHR`."]
#[doc(alias = "vkGetPipelineKeyKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html)"]
pub type PFN_vkGetPipelineKeyKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
		p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPipelineBinaryDataKHR`."]
#[doc(alias = "vkGetPipelineBinaryDataKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html)"]
pub type PFN_vkGetPipelineBinaryDataKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PipelineBinaryDataInfoKHR<'_>,
		p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
		p_pipeline_binary_data_size: *mut usize,
		p_pipeline_binary_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkReleaseCapturedPipelineDataKHR`."]
#[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
#[doc = "\n\n[Khronos reference: `vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html)"]
pub type PFN_vkReleaseCapturedPipelineDataKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateGraphicsPipelines`."]
#[doc(alias = "vkCreateGraphicsPipelines")]
#[doc = "\n\n[Khronos reference: `vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html)"]
pub type PFN_vkCreateGraphicsPipelines = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateComputePipelines`."]
#[doc(alias = "vkCreateComputePipelines")]
#[doc = "\n\n[Khronos reference: `vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html)"]
pub type PFN_vkCreateComputePipelines = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const ComputePipelineCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`."]
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)"]
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		renderpass: RenderPass,
		p_max_workgroup_size: *mut Extent2D,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyPipeline`."]
#[doc(alias = "vkDestroyPipeline")]
#[doc = "\n\n[Khronos reference: `vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html)"]
pub type PFN_vkDestroyPipeline = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreatePipelineLayout`."]
#[doc(alias = "vkCreatePipelineLayout")]
#[doc = "\n\n[Khronos reference: `vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html)"]
pub type PFN_vkCreatePipelineLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PipelineLayoutCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipeline_layout: *mut PipelineLayout,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyPipelineLayout`."]
#[doc(alias = "vkDestroyPipelineLayout")]
#[doc = "\n\n[Khronos reference: `vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html)"]
pub type PFN_vkDestroyPipelineLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_layout: PipelineLayout,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateSampler`."]
#[doc(alias = "vkCreateSampler")]
#[doc = "\n\n[Khronos reference: `vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html)"]
pub type PFN_vkCreateSampler = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SamplerCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_sampler: *mut Sampler,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroySampler`."]
#[doc(alias = "vkDestroySampler")]
#[doc = "\n\n[Khronos reference: `vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html)"]
pub type PFN_vkDestroySampler = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		sampler: Sampler,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateDescriptorSetLayout`."]
#[doc(alias = "vkCreateDescriptorSetLayout")]
#[doc = "\n\n[Khronos reference: `vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html)"]
pub type PFN_vkCreateDescriptorSetLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_set_layout: *mut DescriptorSetLayout,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDescriptorSetLayout`."]
#[doc(alias = "vkDestroyDescriptorSetLayout")]
#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html)"]
pub type PFN_vkDestroyDescriptorSetLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set_layout: DescriptorSetLayout,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateDescriptorPool`."]
#[doc(alias = "vkCreateDescriptorPool")]
#[doc = "\n\n[Khronos reference: `vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html)"]
pub type PFN_vkCreateDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorPoolCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_descriptor_pool: *mut DescriptorPool,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDescriptorPool`."]
#[doc(alias = "vkDestroyDescriptorPool")]
#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html)"]
pub type PFN_vkDestroyDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkResetDescriptorPool`."]
#[doc(alias = "vkResetDescriptorPool")]
#[doc = "\n\n[Khronos reference: `vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html)"]
pub type PFN_vkResetDescriptorPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		flags: DescriptorPoolResetFlags,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkAllocateDescriptorSets`."]
#[doc(alias = "vkAllocateDescriptorSets")]
#[doc = "\n\n[Khronos reference: `vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html)"]
pub type PFN_vkAllocateDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
		p_descriptor_sets: *mut DescriptorSet,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkFreeDescriptorSets`."]
#[doc(alias = "vkFreeDescriptorSets")]
#[doc = "\n\n[Khronos reference: `vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html)"]
pub type PFN_vkFreeDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_pool: DescriptorPool,
		descriptor_set_count: u32,
		p_descriptor_sets: *const DescriptorSet,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkUpdateDescriptorSets`."]
#[doc(alias = "vkUpdateDescriptorSets")]
#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html)"]
pub type PFN_vkUpdateDescriptorSets = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_write_count: u32,
		p_descriptor_writes: *const WriteDescriptorSet<'_>,
		descriptor_copy_count: u32,
		p_descriptor_copies: *const CopyDescriptorSet<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateFramebuffer`."]
#[doc(alias = "vkCreateFramebuffer")]
#[doc = "\n\n[Khronos reference: `vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html)"]
pub type PFN_vkCreateFramebuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const FramebufferCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_framebuffer: *mut Framebuffer,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyFramebuffer`."]
#[doc(alias = "vkDestroyFramebuffer")]
#[doc = "\n\n[Khronos reference: `vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)"]
pub type PFN_vkDestroyFramebuffer = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		framebuffer: Framebuffer,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateRenderPass`."]
#[doc(alias = "vkCreateRenderPass")]
#[doc = "\n\n[Khronos reference: `vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html)"]
pub type PFN_vkCreateRenderPass = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const RenderPassCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_render_pass: *mut RenderPass,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyRenderPass`."]
#[doc(alias = "vkDestroyRenderPass")]
#[doc = "\n\n[Khronos reference: `vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html)"]
pub type PFN_vkDestroyRenderPass = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		render_pass: RenderPass,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetRenderAreaGranularity`."]
#[doc(alias = "vkGetRenderAreaGranularity")]
#[doc = "\n\n[Khronos reference: `vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html)"]
pub type PFN_vkGetRenderAreaGranularity = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		render_pass: RenderPass,
		p_granularity: *mut Extent2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetRenderingAreaGranularity`."]
#[doc(alias = "vkGetRenderingAreaGranularity")]
#[doc = "\n\n[Khronos reference: `vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html)"]
pub type PFN_vkGetRenderingAreaGranularity = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_rendering_area_info: *const RenderingAreaInfo<'_>,
		p_granularity: *mut Extent2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetRenderingAreaGranularityKHR`."]
#[doc(alias = "vkGetRenderingAreaGranularityKHR")]
#[doc = "\n\n[Khronos reference: `vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html)"]
pub type PFN_vkGetRenderingAreaGranularityKHR = PFN_vkGetRenderingAreaGranularity;

#[doc = "Nullable function pointer for `vkCreateCommandPool`."]
#[doc(alias = "vkCreateCommandPool")]
#[doc = "\n\n[Khronos reference: `vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html)"]
pub type PFN_vkCreateCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CommandPoolCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_command_pool: *mut CommandPool,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyCommandPool`."]
#[doc(alias = "vkDestroyCommandPool")]
#[doc = "\n\n[Khronos reference: `vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html)"]
pub type PFN_vkDestroyCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkResetCommandPool`."]
#[doc(alias = "vkResetCommandPool")]
#[doc = "\n\n[Khronos reference: `vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html)"]
pub type PFN_vkResetCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		flags: CommandPoolResetFlags,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkAllocateCommandBuffers`."]
#[doc(alias = "vkAllocateCommandBuffers")]
#[doc = "\n\n[Khronos reference: `vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html)"]
pub type PFN_vkAllocateCommandBuffers = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocate_info: *const CommandBufferAllocateInfo<'_>,
		p_command_buffers: *mut CommandBuffer,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkFreeCommandBuffers`."]
#[doc(alias = "vkFreeCommandBuffers")]
#[doc = "\n\n[Khronos reference: `vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html)"]
pub type PFN_vkFreeCommandBuffers = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		command_buffer_count: u32,
		p_command_buffers: *const CommandBuffer,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBeginCommandBuffer`."]
#[doc(alias = "vkBeginCommandBuffer")]
#[doc = "\n\n[Khronos reference: `vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html)"]
pub type PFN_vkBeginCommandBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_info: *const CommandBufferBeginInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEndCommandBuffer`."]
#[doc(alias = "vkEndCommandBuffer")]
#[doc = "\n\n[Khronos reference: `vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html)"]
pub type PFN_vkEndCommandBuffer =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result>;

#[doc = "Nullable function pointer for `vkResetCommandBuffer`."]
#[doc(alias = "vkResetCommandBuffer")]
#[doc = "\n\n[Khronos reference: `vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html)"]
pub type PFN_vkResetCommandBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		flags: CommandBufferResetFlags,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBindPipeline`."]
#[doc(alias = "vkCmdBindPipeline")]
#[doc = "\n\n[Khronos reference: `vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html)"]
pub type PFN_vkCmdBindPipeline = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetPrimitiveRestartIndexEXT`."]
#[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html)"]
pub type PFN_vkCmdSetPrimitiveRestartIndexEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_index: u32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetAttachmentFeedbackLoopEnableEXT`."]
#[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html)"]
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetViewport`."]
#[doc(alias = "vkCmdSetViewport")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html)"]
pub type PFN_vkCmdSetViewport = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewports: *const Viewport,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetScissor`."]
#[doc(alias = "vkCmdSetScissor")]
#[doc = "\n\n[Khronos reference: `vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html)"]
pub type PFN_vkCmdSetScissor = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_scissor: u32,
		scissor_count: u32,
		p_scissors: *const Rect2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetLineWidth`."]
#[doc(alias = "vkCmdSetLineWidth")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html)"]
pub type PFN_vkCmdSetLineWidth =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetDepthBias`."]
#[doc(alias = "vkCmdSetDepthBias")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html)"]
pub type PFN_vkCmdSetDepthBias = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		depth_bias_constant_factor: f32,
		depth_bias_clamp: f32,
		depth_bias_slope_factor: f32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetBlendConstants`."]
#[doc(alias = "vkCmdSetBlendConstants")]
#[doc = "\n\n[Khronos reference: `vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html)"]
pub type PFN_vkCmdSetBlendConstants = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const f32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthBounds`."]
#[doc(alias = "vkCmdSetDepthBounds")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html)"]
pub type PFN_vkCmdSetDepthBounds = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		min_depth_bounds: f32,
		max_depth_bounds: f32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetStencilCompareMask`."]
#[doc(alias = "vkCmdSetStencilCompareMask")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html)"]
pub type PFN_vkCmdSetStencilCompareMask = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		compare_mask: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetStencilWriteMask`."]
#[doc(alias = "vkCmdSetStencilWriteMask")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html)"]
pub type PFN_vkCmdSetStencilWriteMask = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		write_mask: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetStencilReference`."]
#[doc(alias = "vkCmdSetStencilReference")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html)"]
pub type PFN_vkCmdSetStencilReference = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		face_mask: StencilFaceFlags,
		reference: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindDescriptorSets`."]
#[doc(alias = "vkCmdBindDescriptorSets")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBindIndexBuffer`."]
#[doc(alias = "vkCmdBindIndexBuffer")]
#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html)"]
pub type PFN_vkCmdBindIndexBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		index_type: IndexType,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindVertexBuffers`."]
#[doc(alias = "vkCmdBindVertexBuffers")]
#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html)"]
pub type PFN_vkCmdBindVertexBuffers = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_buffers: *const Buffer,
		p_offsets: *const DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDraw`."]
#[doc(alias = "vkCmdDraw")]
#[doc = "\n\n[Khronos reference: `vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html)"]
pub type PFN_vkCmdDraw = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		vertex_count: u32,
		instance_count: u32,
		first_vertex: u32,
		first_instance: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndexed`."]
#[doc(alias = "vkCmdDrawIndexed")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawMultiEXT`."]
#[doc(alias = "vkCmdDrawMultiEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawMultiIndexedEXT`."]
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawIndirect`."]
#[doc(alias = "vkCmdDrawIndirect")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html)"]
pub type PFN_vkCmdDrawIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirect`."]
#[doc(alias = "vkCmdDrawIndexedIndirect")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html)"]
pub type PFN_vkCmdDrawIndexedIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDispatch`."]
#[doc(alias = "vkCmdDispatch")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html)"]
pub type PFN_vkCmdDispatch = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDispatchIndirect`."]
#[doc(alias = "vkCmdDispatchIndirect")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html)"]
pub type PFN_vkCmdDispatchIndirect = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSubpassShadingHUAWEI`."]
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
#[doc = "\n\n[Khronos reference: `vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html)"]
pub type PFN_vkCmdSubpassShadingHUAWEI =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdDrawClusterHUAWEI`."]
#[doc(alias = "vkCmdDrawClusterHUAWEI")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html)"]
pub type PFN_vkCmdDrawClusterHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawClusterIndirectHUAWEI`."]
#[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html)"]
pub type PFN_vkCmdDrawClusterIndirectHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdUpdatePipelineIndirectBufferNV`."]
#[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
#[doc = "\n\n[Khronos reference: `vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html)"]
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyBuffer`."]
#[doc(alias = "vkCmdCopyBuffer")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html)"]
pub type PFN_vkCmdCopyBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_buffer: Buffer,
		dst_buffer: Buffer,
		region_count: u32,
		p_regions: *const BufferCopy,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyImage`."]
#[doc(alias = "vkCmdCopyImage")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBlitImage`."]
#[doc(alias = "vkCmdBlitImage")]
#[doc = "\n\n[Khronos reference: `vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdCopyBufferToImage`."]
#[doc(alias = "vkCmdCopyBufferToImage")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdCopyImageToBuffer`."]
#[doc(alias = "vkCmdCopyImageToBuffer")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html)"]
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

#[doc = "Nullable function pointer for `vkCmdCopyMemoryIndirectNV`."]
#[doc(alias = "vkCmdCopyMemoryIndirectNV")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html)"]
pub type PFN_vkCmdCopyMemoryIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		copy_buffer_address: DeviceAddress,
		copy_count: u32,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryIndirectKHR`."]
#[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html)"]
pub type PFN_vkCmdCopyMemoryIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryToImageIndirectNV`."]
#[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html)"]
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

#[doc = "Nullable function pointer for `vkCmdCopyMemoryToImageIndirectKHR`."]
#[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html)"]
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdUpdateBuffer`."]
#[doc(alias = "vkCmdUpdateBuffer")]
#[doc = "\n\n[Khronos reference: `vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html)"]
pub type PFN_vkCmdUpdateBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		data_size: DeviceSize,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdFillBuffer`."]
#[doc(alias = "vkCmdFillBuffer")]
#[doc = "\n\n[Khronos reference: `vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html)"]
pub type PFN_vkCmdFillBuffer = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		size: DeviceSize,
		data: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdClearColorImage`."]
#[doc(alias = "vkCmdClearColorImage")]
#[doc = "\n\n[Khronos reference: `vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdClearDepthStencilImage`."]
#[doc(alias = "vkCmdClearDepthStencilImage")]
#[doc = "\n\n[Khronos reference: `vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdClearAttachments`."]
#[doc(alias = "vkCmdClearAttachments")]
#[doc = "\n\n[Khronos reference: `vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html)"]
pub type PFN_vkCmdClearAttachments = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		attachment_count: u32,
		p_attachments: *const ClearAttachment,
		rect_count: u32,
		p_rects: *const ClearRect,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdResolveImage`."]
#[doc(alias = "vkCmdResolveImage")]
#[doc = "\n\n[Khronos reference: `vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html)"]
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

#[doc = "Nullable function pointer for `vkCmdSetEvent`."]
#[doc(alias = "vkCmdSetEvent")]
#[doc = "\n\n[Khronos reference: `vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html)"]
pub type PFN_vkCmdSetEvent = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdResetEvent`."]
#[doc(alias = "vkCmdResetEvent")]
#[doc = "\n\n[Khronos reference: `vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html)"]
pub type PFN_vkCmdResetEvent = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdWaitEvents`."]
#[doc(alias = "vkCmdWaitEvents")]
#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html)"]
pub type PFN_vkCmdWaitEvents = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event_count: u32,
		p_events: *const Event,
		src_stage_mask: PipelineStageFlags,
		dst_stage_mask: PipelineStageFlags,
		memory_barrier_count: u32,
		p_memory_barriers: *const MemoryBarrier<'_>,
		buffer_memory_barrier_count: u32,
		p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
		image_memory_barrier_count: u32,
		p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPipelineBarrier`."]
#[doc(alias = "vkCmdPipelineBarrier")]
#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html)"]
pub type PFN_vkCmdPipelineBarrier = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		src_stage_mask: PipelineStageFlags,
		dst_stage_mask: PipelineStageFlags,
		dependency_flags: DependencyFlags,
		memory_barrier_count: u32,
		p_memory_barriers: *const MemoryBarrier<'_>,
		buffer_memory_barrier_count: u32,
		p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
		image_memory_barrier_count: u32,
		p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginQuery`."]
#[doc(alias = "vkCmdBeginQuery")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html)"]
pub type PFN_vkCmdBeginQuery = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		flags: QueryControlFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndQuery`."]
#[doc(alias = "vkCmdEndQuery")]
#[doc = "\n\n[Khronos reference: `vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html)"]
pub type PFN_vkCmdEndQuery = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginConditionalRenderingEXT`."]
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html)"]
pub type PFN_vkCmdBeginConditionalRenderingEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndConditionalRenderingEXT`."]
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html)"]
pub type PFN_vkCmdEndConditionalRenderingEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdBeginCustomResolveEXT`."]
#[doc(alias = "vkCmdBeginCustomResolveEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html)"]
pub type PFN_vkCmdBeginCustomResolveEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdResetQueryPool`."]
#[doc(alias = "vkCmdResetQueryPool")]
#[doc = "\n\n[Khronos reference: `vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html)"]
pub type PFN_vkCmdResetQueryPool = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		first_query: u32,
		query_count: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdWriteTimestamp`."]
#[doc(alias = "vkCmdWriteTimestamp")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html)"]
pub type PFN_vkCmdWriteTimestamp = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_stage: PipelineStageFlagBits,
		query_pool: QueryPool,
		query: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyQueryPoolResults`."]
#[doc(alias = "vkCmdCopyQueryPoolResults")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html)"]
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

#[doc = "Nullable function pointer for `vkCmdPushConstants`."]
#[doc(alias = "vkCmdPushConstants")]
#[doc = "\n\n[Khronos reference: `vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBeginRenderPass`."]
#[doc(alias = "vkCmdBeginRenderPass")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html)"]
pub type PFN_vkCmdBeginRenderPass = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_render_pass_begin: *const RenderPassBeginInfo<'_>,
		contents: SubpassContents,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdNextSubpass`."]
#[doc(alias = "vkCmdNextSubpass")]
#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html)"]
pub type PFN_vkCmdNextSubpass =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents) -> ()>;

#[doc = "Nullable function pointer for `vkCmdEndRenderPass`."]
#[doc(alias = "vkCmdEndRenderPass")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html)"]
pub type PFN_vkCmdEndRenderPass =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdExecuteCommands`."]
#[doc(alias = "vkCmdExecuteCommands")]
#[doc = "\n\n[Khronos reference: `vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html)"]
pub type PFN_vkCmdExecuteCommands = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		command_buffer_count: u32,
		p_command_buffers: *const CommandBuffer,
	) -> (),
>;

#[cfg(feature = "platform-android")]
#[doc = "Nullable function pointer for `vkCreateAndroidSurfaceKHR`."]
#[doc(alias = "vkCreateAndroidSurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html)"]
pub type PFN_vkCreateAndroidSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ohos")]
#[doc = "Nullable function pointer for `vkCreateSurfaceOHOS`."]
#[doc(alias = "vkCreateSurfaceOHOS")]
#[doc = "\n\n[Khronos reference: `vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html)"]
pub type PFN_vkCreateSurfaceOHOS = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const SurfaceCreateInfoOHOS<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDisplayPropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPlanePropertiesKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDisplayPlaneSupportedDisplaysKHR`."]
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html)"]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		plane_index: u32,
		p_display_count: *mut u32,
		p_displays: *mut DisplayKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDisplayModePropertiesKHR`."]
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html)"]
pub type PFN_vkGetDisplayModePropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_property_count: *mut u32,
		p_properties: *mut DisplayModePropertiesKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateDisplayModeKHR`."]
#[doc(alias = "vkCreateDisplayModeKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html)"]
pub type PFN_vkCreateDisplayModeKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_create_info: *const DisplayModeCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_mode: *mut DisplayModeKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDisplayPlaneCapabilitiesKHR`."]
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html)"]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		mode: DisplayModeKHR,
		plane_index: u32,
		p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateDisplayPlaneSurfaceKHR`."]
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html)"]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateSharedSwapchainsKHR`."]
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html)"]
pub type PFN_vkCreateSharedSwapchainsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain_count: u32,
		p_create_infos: *const SwapchainCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_swapchains: *mut SwapchainKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroySurfaceKHR`."]
#[doc(alias = "vkDestroySurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)"]
pub type PFN_vkDestroySurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		surface: SurfaceKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceSupportKHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		surface: SurfaceKHR,
		p_supported: *mut Bool32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceFormatsKHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_format_count: *mut u32,
		p_surface_formats: *mut SurfaceFormatKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfacePresentModesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_present_mode_count: *mut u32,
		p_present_modes: *mut PresentModeKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateSwapchainKHR`."]
#[doc(alias = "vkCreateSwapchainKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html)"]
pub type PFN_vkCreateSwapchainKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SwapchainCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_swapchain: *mut SwapchainKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroySwapchainKHR`."]
#[doc(alias = "vkDestroySwapchainKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html)"]
pub type PFN_vkDestroySwapchainKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetSwapchainImagesKHR`."]
#[doc(alias = "vkGetSwapchainImagesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html)"]
pub type PFN_vkGetSwapchainImagesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_image_count: *mut u32,
		p_swapchain_images: *mut Image,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkAcquireNextImageKHR`."]
#[doc(alias = "vkAcquireNextImageKHR")]
#[doc = "\n\n[Khronos reference: `vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html)"]
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

#[doc = "Nullable function pointer for `vkQueuePresentKHR`."]
#[doc(alias = "vkQueuePresentKHR")]
#[doc = "\n\n[Khronos reference: `vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)"]
pub type PFN_vkQueuePresentKHR = Option<
	unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR<'_>) -> Result,
>;

#[cfg(feature = "platform-vi")]
#[doc = "Nullable function pointer for `vkCreateViSurfaceNN`."]
#[doc(alias = "vkCreateViSurfaceNN")]
#[doc = "\n\n[Khronos reference: `vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html)"]
pub type PFN_vkCreateViSurfaceNN = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ViSurfaceCreateInfoNN<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-wayland")]
#[doc = "Nullable function pointer for `vkCreateWaylandSurfaceKHR`."]
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html)"]
pub type PFN_vkCreateWaylandSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-wayland")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceWaylandPresentationSupportKHR`."]
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)"]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		display: *mut wl_display,
	) -> Bool32,
>;

#[cfg(feature = "platform-ubm")]
#[doc = "Nullable function pointer for `vkCreateUbmSurfaceSEC`."]
#[doc(alias = "vkCreateUbmSurfaceSEC")]
#[doc = "\n\n[Khronos reference: `vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html)"]
pub type PFN_vkCreateUbmSurfaceSEC = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const UbmSurfaceCreateInfoSEC<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ubm")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceUbmPresentationSupportSEC`."]
#[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html)"]
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		device: *mut ubm_device,
	) -> Bool32,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkCreateWin32SurfaceKHR`."]
#[doc(alias = "vkCreateWin32SurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html)"]
pub type PFN_vkCreateWin32SurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceWin32PresentationSupportKHR`."]
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)"]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<
	unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32,
>;

#[cfg(feature = "platform-xlib")]
#[doc = "Nullable function pointer for `vkCreateXlibSurfaceKHR`."]
#[doc(alias = "vkCreateXlibSurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html)"]
pub type PFN_vkCreateXlibSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xlib")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceXlibPresentationSupportKHR`."]
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)"]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		dpy: *mut Display,
		visual_id: VisualID,
	) -> Bool32,
>;

#[cfg(feature = "platform-xcb")]
#[doc = "Nullable function pointer for `vkCreateXcbSurfaceKHR`."]
#[doc(alias = "vkCreateXcbSurfaceKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html)"]
pub type PFN_vkCreateXcbSurfaceKHR = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xcb")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceXcbPresentationSupportKHR`."]
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)"]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		connection: *mut xcb_connection_t,
		visual_id: xcb_visualid_t,
	) -> Bool32,
>;

#[cfg(feature = "platform-directfb")]
#[doc = "Nullable function pointer for `vkCreateDirectFBSurfaceEXT`."]
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html)"]
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-directfb")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDirectFBPresentationSupportEXT`."]
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)"]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		dfb: *mut IDirectFB,
	) -> Bool32,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkCreateImagePipeSurfaceFUCHSIA`."]
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html)"]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-ggp")]
#[doc = "Nullable function pointer for `vkCreateStreamDescriptorSurfaceGGP`."]
#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
#[doc = "\n\n[Khronos reference: `vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html)"]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
#[doc = "Nullable function pointer for `vkCreateScreenSurfaceQNX`."]
#[doc(alias = "vkCreateScreenSurfaceQNX")]
#[doc = "\n\n[Khronos reference: `vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html)"]
pub type PFN_vkCreateScreenSurfaceQNX = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceScreenPresentationSupportQNX`."]
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)"]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		window: *mut _screen_window,
	) -> Bool32,
>;

#[doc = "Nullable function pointer for `vkCreateDebugReportCallbackEXT`."]
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html)"]
pub type PFN_vkCreateDebugReportCallbackEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_callback: *mut DebugReportCallbackEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDebugReportCallbackEXT`."]
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)"]
pub type PFN_vkDestroyDebugReportCallbackEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		callback: DebugReportCallbackEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDebugReportMessageEXT`."]
#[doc(alias = "vkDebugReportMessageEXT")]
#[doc = "\n\n[Khronos reference: `vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)"]
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

#[doc = "Nullable function pointer for `vkDebugMarkerSetObjectNameEXT`."]
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
#[doc = "\n\n[Khronos reference: `vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html)"]
pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDebugMarkerSetObjectTagEXT`."]
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
#[doc = "\n\n[Khronos reference: `vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html)"]
pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdDebugMarkerBeginEXT`."]
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html)"]
pub type PFN_vkCmdDebugMarkerBeginEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDebugMarkerEndEXT`."]
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html)"]
pub type PFN_vkCmdDebugMarkerEndEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdDebugMarkerInsertEXT`."]
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html)"]
pub type PFN_vkCmdDebugMarkerInsertEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`."]
#[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)"]
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
#[doc = "Nullable function pointer for `vkGetMemoryWin32HandleNV`."]
#[doc(alias = "vkGetMemoryWin32HandleNV")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html)"]
pub type PFN_vkGetMemoryWin32HandleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		memory: DeviceMemory,
		handle_type: ExternalMemoryHandleTypeFlagsNV,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdExecuteGeneratedCommandsNV`."]
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
#[doc = "\n\n[Khronos reference: `vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html)"]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		is_preprocessed: Bool32,
		p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPreprocessGeneratedCommandsNV`."]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
#[doc = "\n\n[Khronos reference: `vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html)"]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindPipelineShaderGroupNV`."]
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
#[doc = "\n\n[Khronos reference: `vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html)"]
pub type PFN_vkCmdBindPipelineShaderGroupNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		pipeline: Pipeline,
		group_index: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetGeneratedCommandsMemoryRequirementsNV`."]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
#[doc = "\n\n[Khronos reference: `vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html)"]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateIndirectCommandsLayoutNV`."]
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
#[doc = "\n\n[Khronos reference: `vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html)"]
pub type PFN_vkCreateIndirectCommandsLayoutNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyIndirectCommandsLayoutNV`."]
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html)"]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_commands_layout: IndirectCommandsLayoutNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdExecuteGeneratedCommandsEXT`."]
#[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html)"]
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		is_preprocessed: Bool32,
		p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPreprocessGeneratedCommandsEXT`."]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html)"]
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
		state_command_buffer: CommandBuffer,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetGeneratedCommandsMemoryRequirementsEXT`."]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
#[doc = "\n\n[Khronos reference: `vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html)"]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateIndirectCommandsLayoutEXT`."]
#[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html)"]
pub type PFN_vkCreateIndirectCommandsLayoutEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectCommandsLayoutCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyIndirectCommandsLayoutEXT`."]
#[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html)"]
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_commands_layout: IndirectCommandsLayoutEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateIndirectExecutionSetEXT`."]
#[doc(alias = "vkCreateIndirectExecutionSetEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html)"]
pub type PFN_vkCreateIndirectExecutionSetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const IndirectExecutionSetCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_indirect_execution_set: *mut IndirectExecutionSetEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyIndirectExecutionSetEXT`."]
#[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html)"]
pub type PFN_vkDestroyIndirectExecutionSetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkUpdateIndirectExecutionSetPipelineEXT`."]
#[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
#[doc = "\n\n[Khronos reference: `vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html)"]
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		execution_set_write_count: u32,
		p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkUpdateIndirectExecutionSetShaderEXT`."]
#[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
#[doc = "\n\n[Khronos reference: `vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html)"]
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		indirect_execution_set: IndirectExecutionSetEXT,
		execution_set_write_count: u32,
		p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFeatures2`."]
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html)"]
pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_features: *mut PhysicalDeviceFeatures2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFeatures2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_properties: *mut PhysicalDeviceProperties2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFormatProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		format: Format,
		p_format_properties: *mut FormatProperties2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFormatProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceImageFormatProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
		p_image_format_properties: *mut ImageFormatProperties2<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceImageFormatProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
	PFN_vkGetPhysicalDeviceImageFormatProperties2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_queue_family_property_count: *mut u32,
		p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
	PFN_vkGetPhysicalDeviceQueueFamilyProperties2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceMemoryProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceMemoryProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties2`."]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html)"]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
		p_property_count: *mut u32,
		p_properties: *mut SparseImageFormatProperties2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
	PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSet`."]
#[doc(alias = "vkCmdPushDescriptorSet")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html)"]
pub type PFN_vkCmdPushDescriptorSet = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		set: u32,
		descriptor_write_count: u32,
		p_descriptor_writes: *const WriteDescriptorSet<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSetKHR`."]
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html)"]
pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;

#[doc = "Nullable function pointer for `vkTrimCommandPool`."]
#[doc(alias = "vkTrimCommandPool")]
#[doc = "\n\n[Khronos reference: `vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)"]
pub type PFN_vkTrimCommandPool = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		command_pool: CommandPool,
		flags: CommandPoolTrimFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkTrimCommandPoolKHR`."]
#[doc(alias = "vkTrimCommandPoolKHR")]
#[doc = "\n\n[Khronos reference: `vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html)"]
pub type PFN_vkTrimCommandPoolKHR = PFN_vkTrimCommandPool;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalBufferProperties`."]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html)"]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
		p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalBufferPropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalBufferProperties;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetMemoryWin32HandleKHR`."]
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html)"]
pub type PFN_vkGetMemoryWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetMemoryWin32HandlePropertiesKHR`."]
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html)"]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		handle: HANDLE,
		p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetMemoryFdKHR`."]
#[doc(alias = "vkGetMemoryFdKHR")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html)"]
pub type PFN_vkGetMemoryFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
		p_fd: *mut i32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetMemoryFdPropertiesKHR`."]
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html)"]
pub type PFN_vkGetMemoryFdPropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		fd: i32,
		p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkGetMemoryZirconHandleFUCHSIA`."]
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html)"]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
		p_zircon_handle: *mut zx_handle_t,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkGetMemoryZirconHandlePropertiesFUCHSIA`."]
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)"]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		zircon_handle: zx_handle_t,
		p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetMemoryRemoteAddressNV`."]
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html)"]
pub type PFN_vkGetMemoryRemoteAddressNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
		p_address: *mut RemoteAddressNV,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalSemaphoreProperties`."]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html)"]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
		p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetSemaphoreWin32HandleKHR`."]
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
#[doc = "\n\n[Khronos reference: `vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html)"]
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkImportSemaphoreWin32HandleKHR`."]
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
#[doc = "\n\n[Khronos reference: `vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html)"]
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSemaphoreFdKHR`."]
#[doc(alias = "vkGetSemaphoreFdKHR")]
#[doc = "\n\n[Khronos reference: `vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html)"]
pub type PFN_vkGetSemaphoreFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
		p_fd: *mut i32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkImportSemaphoreFdKHR`."]
#[doc(alias = "vkImportSemaphoreFdKHR")]
#[doc = "\n\n[Khronos reference: `vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html)"]
pub type PFN_vkImportSemaphoreFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkGetSemaphoreZirconHandleFUCHSIA`."]
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html)"]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
		p_zircon_handle: *mut zx_handle_t,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkImportSemaphoreZirconHandleFUCHSIA`."]
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html)"]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalFenceProperties`."]
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html)"]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
		p_external_fence_properties: *mut ExternalFenceProperties<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalFencePropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
	PFN_vkGetPhysicalDeviceExternalFenceProperties;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetFenceWin32HandleKHR`."]
#[doc(alias = "vkGetFenceWin32HandleKHR")]
#[doc = "\n\n[Khronos reference: `vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html)"]
pub type PFN_vkGetFenceWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
		p_handle: *mut HANDLE,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkImportFenceWin32HandleKHR`."]
#[doc(alias = "vkImportFenceWin32HandleKHR")]
#[doc = "\n\n[Khronos reference: `vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html)"]
pub type PFN_vkImportFenceWin32HandleKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetFenceFdKHR`."]
#[doc(alias = "vkGetFenceFdKHR")]
#[doc = "\n\n[Khronos reference: `vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html)"]
pub type PFN_vkGetFenceFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
		p_fd: *mut i32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkImportFenceFdKHR`."]
#[doc(alias = "vkImportFenceFdKHR")]
#[doc = "\n\n[Khronos reference: `vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html)"]
pub type PFN_vkImportFenceFdKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkReleaseDisplayEXT`."]
#[doc(alias = "vkReleaseDisplayEXT")]
#[doc = "\n\n[Khronos reference: `vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html)"]
pub type PFN_vkReleaseDisplayEXT =
	Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result>;

#[cfg(feature = "platform-xlib-xrandr")]
#[doc = "Nullable function pointer for `vkAcquireXlibDisplayEXT`."]
#[doc(alias = "vkAcquireXlibDisplayEXT")]
#[doc = "\n\n[Khronos reference: `vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html)"]
pub type PFN_vkAcquireXlibDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		dpy: *mut Display,
		display: DisplayKHR,
	) -> Result,
>;

#[cfg(feature = "platform-xlib-xrandr")]
#[doc = "Nullable function pointer for `vkGetRandROutputDisplayEXT`."]
#[doc(alias = "vkGetRandROutputDisplayEXT")]
#[doc = "\n\n[Khronos reference: `vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html)"]
pub type PFN_vkGetRandROutputDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		dpy: *mut Display,
		rr_output: RROutput,
		p_display: *mut DisplayKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkAcquireWinrtDisplayNV`."]
#[doc(alias = "vkAcquireWinrtDisplayNV")]
#[doc = "\n\n[Khronos reference: `vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html)"]
pub type PFN_vkAcquireWinrtDisplayNV =
	Option<unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetWinrtDisplayNV`."]
#[doc(alias = "vkGetWinrtDisplayNV")]
#[doc = "\n\n[Khronos reference: `vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html)"]
pub type PFN_vkGetWinrtDisplayNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		device_relative_id: u32,
		p_display: *mut DisplayKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDisplayPowerControlEXT`."]
#[doc(alias = "vkDisplayPowerControlEXT")]
#[doc = "\n\n[Khronos reference: `vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html)"]
pub type PFN_vkDisplayPowerControlEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		display: DisplayKHR,
		p_display_power_info: *const DisplayPowerInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkRegisterDeviceEventEXT`."]
#[doc(alias = "vkRegisterDeviceEventEXT")]
#[doc = "\n\n[Khronos reference: `vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html)"]
pub type PFN_vkRegisterDeviceEventEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_device_event_info: *const DeviceEventInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkRegisterDisplayEventEXT`."]
#[doc(alias = "vkRegisterDisplayEventEXT")]
#[doc = "\n\n[Khronos reference: `vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html)"]
pub type PFN_vkRegisterDisplayEventEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		display: DisplayKHR,
		p_display_event_info: *const DisplayEventInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_fence: *mut Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSwapchainCounterEXT`."]
#[doc(alias = "vkGetSwapchainCounterEXT")]
#[doc = "\n\n[Khronos reference: `vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html)"]
pub type PFN_vkGetSwapchainCounterEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		counter: SurfaceCounterFlagBitsEXT,
		p_counter_value: *mut u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceCapabilities2EXT`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDeviceGroups`."]
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)"]
pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_physical_device_group_count: *mut u32,
		p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDeviceGroupsKHR`."]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html)"]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;

#[doc = "Nullable function pointer for `vkGetDeviceGroupPeerMemoryFeatures`."]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html)"]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		heap_index: u32,
		local_device_index: u32,
		remote_device_index: u32,
		p_peer_memory_features: *mut PeerMemoryFeatureFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceGroupPeerMemoryFeaturesKHR`."]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)"]
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;

#[doc = "Nullable function pointer for `vkBindBufferMemory2`."]
#[doc(alias = "vkBindBufferMemory2")]
#[doc = "\n\n[Khronos reference: `vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html)"]
pub type PFN_vkBindBufferMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindBufferMemoryInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkBindBufferMemory2KHR`."]
#[doc(alias = "vkBindBufferMemory2KHR")]
#[doc = "\n\n[Khronos reference: `vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html)"]
pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;

#[doc = "Nullable function pointer for `vkBindImageMemory2`."]
#[doc(alias = "vkBindImageMemory2")]
#[doc = "\n\n[Khronos reference: `vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html)"]
pub type PFN_vkBindImageMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindImageMemoryInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkBindImageMemory2KHR`."]
#[doc(alias = "vkBindImageMemory2KHR")]
#[doc = "\n\n[Khronos reference: `vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html)"]
pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;

#[doc = "Nullable function pointer for `vkCmdSetDeviceMask`."]
#[doc(alias = "vkCmdSetDeviceMask")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html)"]
pub type PFN_vkCmdSetDeviceMask =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetDeviceMaskKHR`."]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html)"]
pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;

#[doc = "Nullable function pointer for `vkGetDeviceGroupPresentCapabilitiesKHR`."]
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html)"]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceGroupSurfacePresentModesKHR`."]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html)"]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		surface: SurfaceKHR,
		p_modes: *mut DeviceGroupPresentModeFlagsKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkAcquireNextImage2KHR`."]
#[doc(alias = "vkAcquireNextImage2KHR")]
#[doc = "\n\n[Khronos reference: `vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html)"]
pub type PFN_vkAcquireNextImage2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
		p_image_index: *mut u32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdDispatchBase`."]
#[doc(alias = "vkCmdDispatchBase")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDispatchBaseKHR`."]
#[doc(alias = "vkCmdDispatchBaseKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html)"]
pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;

#[doc = "Nullable function pointer for `vkGetPhysicalDevicePresentRectanglesKHR`."]
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html)"]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
		p_rect_count: *mut u32,
		p_rects: *mut Rect2D,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateDescriptorUpdateTemplate`."]
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
#[doc = "\n\n[Khronos reference: `vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html)"]
pub type PFN_vkCreateDescriptorUpdateTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_descriptor_update_template: *mut DescriptorUpdateTemplate,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateDescriptorUpdateTemplateKHR`."]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html)"]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = PFN_vkCreateDescriptorUpdateTemplate;

#[doc = "Nullable function pointer for `vkDestroyDescriptorUpdateTemplate`."]
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)"]
pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_update_template: DescriptorUpdateTemplate,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroyDescriptorUpdateTemplateKHR`."]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html)"]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = PFN_vkDestroyDescriptorUpdateTemplate;

#[doc = "Nullable function pointer for `vkUpdateDescriptorSetWithTemplate`."]
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)"]
pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set: DescriptorSet,
		descriptor_update_template: DescriptorUpdateTemplate,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkUpdateDescriptorSetWithTemplateKHR`."]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html)"]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = PFN_vkUpdateDescriptorSetWithTemplate;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSetWithTemplate`."]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html)"]
pub type PFN_vkCmdPushDescriptorSetWithTemplate = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		descriptor_update_template: DescriptorUpdateTemplate,
		layout: PipelineLayout,
		set: u32,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSetWithTemplateKHR`."]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html)"]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;

#[doc = "Nullable function pointer for `vkSetHdrMetadataEXT`."]
#[doc(alias = "vkSetHdrMetadataEXT")]
#[doc = "\n\n[Khronos reference: `vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html)"]
pub type PFN_vkSetHdrMetadataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain_count: u32,
		p_swapchains: *const SwapchainKHR,
		p_metadata: *const HdrMetadataEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetSwapchainStatusKHR`."]
#[doc(alias = "vkGetSwapchainStatusKHR")]
#[doc = "\n\n[Khronos reference: `vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html)"]
pub type PFN_vkGetSwapchainStatusKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

#[doc = "Nullable function pointer for `vkGetRefreshCycleDurationGOOGLE`."]
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
#[doc = "\n\n[Khronos reference: `vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html)"]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPastPresentationTimingGOOGLE`."]
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
#[doc = "\n\n[Khronos reference: `vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)"]
pub type PFN_vkGetPastPresentationTimingGOOGLE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_presentation_timing_count: *mut u32,
		p_presentation_timings: *mut PastPresentationTimingGOOGLE,
	) -> Result,
>;

#[cfg(feature = "platform-ios")]
#[doc = "Nullable function pointer for `vkCreateIOSSurfaceMVK`."]
#[doc(alias = "vkCreateIOSSurfaceMVK")]
#[doc = "\n\n[Khronos reference: `vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html)"]
pub type PFN_vkCreateIOSSurfaceMVK = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-macos")]
#[doc = "Nullable function pointer for `vkCreateMacOSSurfaceMVK`."]
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
#[doc = "\n\n[Khronos reference: `vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html)"]
pub type PFN_vkCreateMacOSSurfaceMVK = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
#[doc = "Nullable function pointer for `vkCreateMetalSurfaceEXT`."]
#[doc(alias = "vkCreateMetalSurfaceEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html)"]
pub type PFN_vkCreateMetalSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetViewportWScalingNV`."]
#[doc(alias = "vkCmdSetViewportWScalingNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html)"]
pub type PFN_vkCmdSetViewportWScalingNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewport_w_scalings: *const ViewportWScalingNV,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDiscardRectangleEXT`."]
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html)"]
pub type PFN_vkCmdSetDiscardRectangleEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_discard_rectangle: u32,
		discard_rectangle_count: u32,
		p_discard_rectangles: *const Rect2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDiscardRectangleEnableEXT`."]
#[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html)"]
pub type PFN_vkCmdSetDiscardRectangleEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDiscardRectangleModeEXT`."]
#[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html)"]
pub type PFN_vkCmdSetDiscardRectangleModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		discard_rectangle_mode: DiscardRectangleModeEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetSampleLocationsEXT`."]
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html)"]
pub type PFN_vkCmdSetSampleLocationsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceMultisamplePropertiesEXT`."]
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)"]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		samples: SampleCountFlagBits,
		p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceCapabilities2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
		p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfaceFormats2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
		p_surface_format_count: *mut u32,
		p_surface_formats: *mut SurfaceFormat2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDisplayProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayProperties2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDisplayPlaneProperties2KHR`."]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)"]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut DisplayPlaneProperties2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDisplayModeProperties2KHR`."]
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html)"]
pub type PFN_vkGetDisplayModeProperties2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		display: DisplayKHR,
		p_property_count: *mut u32,
		p_properties: *mut DisplayModeProperties2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDisplayPlaneCapabilities2KHR`."]
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html)"]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
		p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetBufferMemoryRequirements2`."]
#[doc(alias = "vkGetBufferMemoryRequirements2")]
#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html)"]
pub type PFN_vkGetBufferMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferMemoryRequirementsInfo2<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetBufferMemoryRequirements2KHR`."]
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html)"]
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;

#[doc = "Nullable function pointer for `vkGetImageMemoryRequirements2`."]
#[doc(alias = "vkGetImageMemoryRequirements2")]
#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html)"]
pub type PFN_vkGetImageMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageMemoryRequirementsInfo2<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetImageMemoryRequirements2KHR`."]
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html)"]
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;

#[doc = "Nullable function pointer for `vkGetImageSparseMemoryRequirements2`."]
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html)"]
pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetImageSparseMemoryRequirements2KHR`."]
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html)"]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;

#[doc = "Nullable function pointer for `vkGetDeviceBufferMemoryRequirements`."]
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html)"]
pub type PFN_vkGetDeviceBufferMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceBufferMemoryRequirements<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceBufferMemoryRequirementsKHR`."]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html)"]
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;

#[doc = "Nullable function pointer for `vkGetDeviceImageMemoryRequirements`."]
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html)"]
pub type PFN_vkGetDeviceImageMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageMemoryRequirements<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceImageMemoryRequirementsKHR`."]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html)"]
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;

#[doc = "Nullable function pointer for `vkGetDeviceImageSparseMemoryRequirements`."]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html)"]
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageMemoryRequirements<'_>,
		p_sparse_memory_requirement_count: *mut u32,
		p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceImageSparseMemoryRequirementsKHR`."]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html)"]
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
	PFN_vkGetDeviceImageSparseMemoryRequirements;

#[doc = "Nullable function pointer for `vkCreateSamplerYcbcrConversion`."]
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
#[doc = "\n\n[Khronos reference: `vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html)"]
pub type PFN_vkCreateSamplerYcbcrConversion = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_ycbcr_conversion: *mut SamplerYcbcrConversion,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateSamplerYcbcrConversionKHR`."]
#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html)"]
pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;

#[doc = "Nullable function pointer for `vkDestroySamplerYcbcrConversion`."]
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
#[doc = "\n\n[Khronos reference: `vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)"]
pub type PFN_vkDestroySamplerYcbcrConversion = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		ycbcr_conversion: SamplerYcbcrConversion,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroySamplerYcbcrConversionKHR`."]
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html)"]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;

#[doc = "Nullable function pointer for `vkGetDeviceQueue2`."]
#[doc(alias = "vkGetDeviceQueue2")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html)"]
pub type PFN_vkGetDeviceQueue2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_queue_info: *const DeviceQueueInfo2<'_>,
		p_queue: *mut Queue,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateValidationCacheEXT`."]
#[doc(alias = "vkCreateValidationCacheEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html)"]
pub type PFN_vkCreateValidationCacheEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_validation_cache: *mut ValidationCacheEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyValidationCacheEXT`."]
#[doc(alias = "vkDestroyValidationCacheEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html)"]
pub type PFN_vkDestroyValidationCacheEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		validation_cache: ValidationCacheEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetValidationCacheDataEXT`."]
#[doc(alias = "vkGetValidationCacheDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html)"]
pub type PFN_vkGetValidationCacheDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		validation_cache: ValidationCacheEXT,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkMergeValidationCachesEXT`."]
#[doc(alias = "vkMergeValidationCachesEXT")]
#[doc = "\n\n[Khronos reference: `vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html)"]
pub type PFN_vkMergeValidationCachesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		dst_cache: ValidationCacheEXT,
		src_cache_count: u32,
		p_src_caches: *const ValidationCacheEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDescriptorSetLayoutSupport`."]
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html)"]
pub type PFN_vkGetDescriptorSetLayoutSupport = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
		p_support: *mut DescriptorSetLayoutSupport<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDescriptorSetLayoutSupportKHR`."]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html)"]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;

#[doc = "Nullable function pointer for `vkGetShaderInfoAMD`."]
#[doc(alias = "vkGetShaderInfoAMD")]
#[doc = "\n\n[Khronos reference: `vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html)"]
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

#[doc = "Nullable function pointer for `vkSetLocalDimmingAMD`."]
#[doc(alias = "vkSetLocalDimmingAMD")]
#[doc = "\n\n[Khronos reference: `vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html)"]
pub type PFN_vkSetLocalDimmingAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swap_chain: SwapchainKHR,
		local_dimming_enable: Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`."]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html)"]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_time_domain_count: *mut u32,
		p_time_domains: *mut TimeDomainKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`."]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)"]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
	PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;

#[doc = "Nullable function pointer for `vkGetCalibratedTimestampsKHR`."]
#[doc(alias = "vkGetCalibratedTimestampsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html)"]
pub type PFN_vkGetCalibratedTimestampsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		timestamp_count: u32,
		p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
		p_timestamps: *mut u64,
		p_max_deviation: *mut u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetCalibratedTimestampsEXT`."]
#[doc(alias = "vkGetCalibratedTimestampsEXT")]
#[doc = "\n\n[Khronos reference: `vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html)"]
pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;

#[doc = "Nullable function pointer for `vkSetDebugUtilsObjectNameEXT`."]
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
#[doc = "\n\n[Khronos reference: `vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html)"]
pub type PFN_vkSetDebugUtilsObjectNameEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkSetDebugUtilsObjectTagEXT`."]
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
#[doc = "\n\n[Khronos reference: `vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html)"]
pub type PFN_vkSetDebugUtilsObjectTagEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkQueueBeginDebugUtilsLabelEXT`."]
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html)"]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>) -> (),
>;

#[doc = "Nullable function pointer for `vkQueueEndDebugUtilsLabelEXT`."]
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html)"]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<unsafe extern "system" fn(queue: Queue) -> ()>;

#[doc = "Nullable function pointer for `vkQueueInsertDebugUtilsLabelEXT`."]
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html)"]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginDebugUtilsLabelEXT`."]
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html)"]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_label_info: *const DebugUtilsLabelEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndDebugUtilsLabelEXT`."]
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html)"]
pub type PFN_vkCmdEndDebugUtilsLabelEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdInsertDebugUtilsLabelEXT`."]
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html)"]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_label_info: *const DebugUtilsLabelEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateDebugUtilsMessengerEXT`."]
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html)"]
pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_messenger: *mut DebugUtilsMessengerEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDebugUtilsMessengerEXT`."]
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)"]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		messenger: DebugUtilsMessengerEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkSubmitDebugUtilsMessageEXT`."]
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
#[doc = "\n\n[Khronos reference: `vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)"]
pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
		message_types: DebugUtilsMessageTypeFlagsEXT,
		p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetMemoryHostPointerPropertiesEXT`."]
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html)"]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		p_host_pointer: *const std::ffi::c_void,
		p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdWriteBufferMarkerAMD`."]
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html)"]
pub type PFN_vkCmdWriteBufferMarkerAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_stage: PipelineStageFlagBits,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		marker: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateRenderPass2`."]
#[doc(alias = "vkCreateRenderPass2")]
#[doc = "\n\n[Khronos reference: `vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html)"]
pub type PFN_vkCreateRenderPass2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const RenderPassCreateInfo2<'_>,
		p_allocator: *const AllocationCallbacks,
		p_render_pass: *mut RenderPass,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateRenderPass2KHR`."]
#[doc(alias = "vkCreateRenderPass2KHR")]
#[doc = "\n\n[Khronos reference: `vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html)"]
pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;

#[doc = "Nullable function pointer for `vkCmdBeginRenderPass2`."]
#[doc(alias = "vkCmdBeginRenderPass2")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html)"]
pub type PFN_vkCmdBeginRenderPass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_render_pass_begin: *const RenderPassBeginInfo<'_>,
		p_subpass_begin_info: *const SubpassBeginInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginRenderPass2KHR`."]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html)"]
pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;

#[doc = "Nullable function pointer for `vkCmdNextSubpass2`."]
#[doc(alias = "vkCmdNextSubpass2")]
#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html)"]
pub type PFN_vkCmdNextSubpass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_subpass_begin_info: *const SubpassBeginInfo<'_>,
		p_subpass_end_info: *const SubpassEndInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdNextSubpass2KHR`."]
#[doc(alias = "vkCmdNextSubpass2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html)"]
pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;

#[doc = "Nullable function pointer for `vkCmdEndRenderPass2`."]
#[doc(alias = "vkCmdEndRenderPass2")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html)"]
pub type PFN_vkCmdEndRenderPass2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_subpass_end_info: *const SubpassEndInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndRenderPass2KHR`."]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html)"]
pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;

#[doc = "Nullable function pointer for `vkGetSemaphoreCounterValue`."]
#[doc(alias = "vkGetSemaphoreCounterValue")]
#[doc = "\n\n[Khronos reference: `vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)"]
pub type PFN_vkGetSemaphoreCounterValue = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		semaphore: Semaphore,
		p_value: *mut u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSemaphoreCounterValueKHR`."]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
#[doc = "\n\n[Khronos reference: `vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html)"]
pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;

#[doc = "Nullable function pointer for `vkWaitSemaphores`."]
#[doc(alias = "vkWaitSemaphores")]
#[doc = "\n\n[Khronos reference: `vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html)"]
pub type PFN_vkWaitSemaphores = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_wait_info: *const SemaphoreWaitInfo<'_>,
		timeout: u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkWaitSemaphoresKHR`."]
#[doc(alias = "vkWaitSemaphoresKHR")]
#[doc = "\n\n[Khronos reference: `vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html)"]
pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;

#[doc = "Nullable function pointer for `vkSignalSemaphore`."]
#[doc(alias = "vkSignalSemaphore")]
#[doc = "\n\n[Khronos reference: `vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html)"]
pub type PFN_vkSignalSemaphore = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_signal_info: *const SemaphoreSignalInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkSignalSemaphoreKHR`."]
#[doc(alias = "vkSignalSemaphoreKHR")]
#[doc = "\n\n[Khronos reference: `vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html)"]
pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;

#[cfg(feature = "platform-android")]
#[doc = "Nullable function pointer for `vkGetAndroidHardwareBufferPropertiesANDROID`."]
#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
#[doc = "\n\n[Khronos reference: `vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html)"]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const AHardwareBuffer,
		p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-android")]
#[doc = "Nullable function pointer for `vkGetMemoryAndroidHardwareBufferANDROID`."]
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html)"]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
		p_buffer: *mut *mut AHardwareBuffer,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndirectCount`."]
#[doc(alias = "vkCmdDrawIndirectCount")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawIndirectCountKHR`."]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html)"]
pub type PFN_vkCmdDrawIndirectCountKHR = PFN_vkCmdDrawIndirectCount;

#[doc = "Nullable function pointer for `vkCmdDrawIndirectCountAMD`."]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html)"]
pub type PFN_vkCmdDrawIndirectCountAMD = PFN_vkCmdDrawIndirectCount;

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirectCount`."]
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirectCountKHR`."]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html)"]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = PFN_vkCmdDrawIndexedIndirectCount;

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirectCountAMD`."]
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html)"]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = PFN_vkCmdDrawIndexedIndirectCount;

#[doc = "Nullable function pointer for `vkCmdSetCheckpointNV`."]
#[doc(alias = "vkCmdSetCheckpointNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html)"]
pub type PFN_vkCmdSetCheckpointNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_checkpoint_marker: *const std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetQueueCheckpointDataNV`."]
#[doc(alias = "vkGetQueueCheckpointDataNV")]
#[doc = "\n\n[Khronos reference: `vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html)"]
pub type PFN_vkGetQueueCheckpointDataNV = Option<
	unsafe extern "system" fn(
		queue: Queue,
		p_checkpoint_data_count: *mut u32,
		p_checkpoint_data: *mut CheckpointDataNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindTransformFeedbackBuffersEXT`."]
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBeginTransformFeedbackEXT`."]
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html)"]
pub type PFN_vkCmdBeginTransformFeedbackEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_buffer: u32,
		counter_buffer_count: u32,
		p_counter_buffers: *const Buffer,
		p_counter_buffer_offsets: *const DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndTransformFeedbackEXT`."]
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html)"]
pub type PFN_vkCmdEndTransformFeedbackEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_buffer: u32,
		counter_buffer_count: u32,
		p_counter_buffers: *const Buffer,
		p_counter_buffer_offsets: *const DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginQueryIndexedEXT`."]
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html)"]
pub type PFN_vkCmdBeginQueryIndexedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		flags: QueryControlFlags,
		index: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndQueryIndexedEXT`."]
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html)"]
pub type PFN_vkCmdEndQueryIndexedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		query_pool: QueryPool,
		query: u32,
		index: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndirectByteCountEXT`."]
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCmdSetExclusiveScissorNV`."]
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html)"]
pub type PFN_vkCmdSetExclusiveScissorNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_exclusive_scissor: u32,
		exclusive_scissor_count: u32,
		p_exclusive_scissors: *const Rect2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetExclusiveScissorEnableNV`."]
#[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html)"]
pub type PFN_vkCmdSetExclusiveScissorEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_exclusive_scissor: u32,
		exclusive_scissor_count: u32,
		p_exclusive_scissor_enables: *const Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindShadingRateImageNV`."]
#[doc(alias = "vkCmdBindShadingRateImageNV")]
#[doc = "\n\n[Khronos reference: `vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html)"]
pub type PFN_vkCmdBindShadingRateImageNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image_view: ImageView,
		image_layout: ImageLayout,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetViewportShadingRatePaletteNV`."]
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html)"]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoarseSampleOrderNV`."]
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html)"]
pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		sample_order_type: CoarseSampleOrderTypeNV,
		custom_sample_order_count: u32,
		p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksNV`."]
#[doc(alias = "vkCmdDrawMeshTasksNV")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html)"]
pub type PFN_vkCmdDrawMeshTasksNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirectNV`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html)"]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirectCountNV`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html)"]
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

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksEXT`."]
#[doc(alias = "vkCmdDrawMeshTasksEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html)"]
pub type PFN_vkCmdDrawMeshTasksEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		group_count_x: u32,
		group_count_y: u32,
		group_count_z: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirectEXT`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html)"]
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		draw_count: u32,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirectCountEXT`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCompileDeferredNV`."]
#[doc(alias = "vkCompileDeferredNV")]
#[doc = "\n\n[Khronos reference: `vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html)"]
pub type PFN_vkCompileDeferredNV = Option<
	unsafe extern "system" fn(device: DeviceHandle, pipeline: Pipeline, shader: u32) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateAccelerationStructureNV`."]
#[doc(alias = "vkCreateAccelerationStructureNV")]
#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html)"]
pub type PFN_vkCreateAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureNV,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBindInvocationMaskHUAWEI`."]
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
#[doc = "\n\n[Khronos reference: `vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html)"]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		image_view: ImageView,
		image_layout: ImageLayout,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroyAccelerationStructureKHR`."]
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html)"]
pub type PFN_vkDestroyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroyAccelerationStructureNV`."]
#[doc(alias = "vkDestroyAccelerationStructureNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)"]
pub type PFN_vkDestroyAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetAccelerationStructureMemoryRequirementsNV`."]
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html)"]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindAccelerationStructureMemoryNV`."]
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
#[doc = "\n\n[Khronos reference: `vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html)"]
pub type PFN_vkBindAccelerationStructureMemoryNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyAccelerationStructureNV`."]
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html)"]
pub type PFN_vkCmdCopyAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		dst: AccelerationStructureNV,
		src: AccelerationStructureNV,
		mode: CopyAccelerationStructureModeKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyAccelerationStructureKHR`."]
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html)"]
pub type PFN_vkCmdCopyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyAccelerationStructureInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyAccelerationStructureKHR`."]
#[doc(alias = "vkCopyAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html)"]
pub type PFN_vkCopyAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyAccelerationStructureInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyAccelerationStructureToMemoryKHR`."]
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html)"]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyAccelerationStructureToMemoryKHR`."]
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html)"]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryToAccelerationStructureKHR`."]
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html)"]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyMemoryToAccelerationStructureKHR`."]
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html)"]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdWriteAccelerationStructuresPropertiesKHR`."]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html)"]
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

#[doc = "Nullable function pointer for `vkCmdWriteAccelerationStructuresPropertiesNV`."]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBuildAccelerationStructureNV`."]
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html)"]
pub type PFN_vkCmdBuildAccelerationStructureNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const AccelerationStructureInfoNV<'_>,
		instance_data: Buffer,
		instance_offset: DeviceSize,
		update: Bool32,
		dst: AccelerationStructureNV,
		src: AccelerationStructureNV,
		scratch: Buffer,
		scratch_offset: DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkWriteAccelerationStructuresPropertiesKHR`."]
#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html)"]
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

#[doc = "Nullable function pointer for `vkCmdTraceRaysKHR`."]
#[doc(alias = "vkCmdTraceRaysKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html)"]
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

#[doc = "Nullable function pointer for `vkCmdTraceRaysNV`."]
#[doc(alias = "vkCmdTraceRaysNV")]
#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html)"]
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

#[doc = "Nullable function pointer for `vkGetRayTracingShaderGroupHandlesKHR`."]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html)"]
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

#[doc = "Nullable function pointer for `vkGetRayTracingShaderGroupHandlesNV`."]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html)"]
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;

#[doc = "Nullable function pointer for `vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`."]
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)"]
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

#[doc = "Nullable function pointer for `vkGetAccelerationStructureHandleNV`."]
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)"]
pub type PFN_vkGetAccelerationStructureHandleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		acceleration_structure: AccelerationStructureNV,
		data_size: usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateRayTracingPipelinesNV`."]
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
#[doc = "\n\n[Khronos reference: `vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html)"]
pub type PFN_vkCreateRayTracingPipelinesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateRayTracingPipelinesKHR`."]
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html)"]
pub type PFN_vkCreateRayTracingPipelinesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`."]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)"]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdTraceRaysIndirectKHR`."]
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html)"]
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

#[doc = "Nullable function pointer for `vkCmdTraceRaysIndirect2KHR`."]
#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html)"]
pub type PFN_vkCmdTraceRaysIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		indirect_device_address: DeviceAddress,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetClusterAccelerationStructureBuildSizesNV`."]
#[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
#[doc = "\n\n[Khronos reference: `vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html)"]
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ClusterAccelerationStructureInputInfoNV<'_>,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBuildClusterAccelerationStructureIndirectNV`."]
#[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html)"]
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceAccelerationStructureCompatibilityKHR`."]
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html)"]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
		p_compatibility: *mut AccelerationStructureCompatibilityKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetRayTracingShaderGroupStackSizeKHR`."]
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html)"]
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline: Pipeline,
		group: u32,
		group_shader: ShaderGroupShaderKHR,
	) -> DeviceSize,
>;

#[doc = "Nullable function pointer for `vkCmdSetRayTracingPipelineStackSizeKHR`."]
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html)"]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32) -> ()>;

#[doc = "Nullable function pointer for `vkGetImageViewHandleNVX`."]
#[doc(alias = "vkGetImageViewHandleNVX")]
#[doc = "\n\n[Khronos reference: `vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html)"]
pub type PFN_vkGetImageViewHandleNVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32,
>;

#[doc = "Nullable function pointer for `vkGetImageViewHandle64NVX`."]
#[doc(alias = "vkGetImageViewHandle64NVX")]
#[doc = "\n\n[Khronos reference: `vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html)"]
pub type PFN_vkGetImageViewHandle64NVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, p_info: *const ImageViewHandleInfoNVX<'_>) -> u64,
>;

#[doc = "Nullable function pointer for `vkGetImageViewAddressNVX`."]
#[doc(alias = "vkGetImageViewAddressNVX")]
#[doc = "\n\n[Khronos reference: `vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html)"]
pub type PFN_vkGetImageViewAddressNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_view: ImageView,
		p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceCombinedImageSamplerIndexNVX`."]
#[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html)"]
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX = Option<
	unsafe extern "system" fn(device: DeviceHandle, image_view_index: u64, sampler_index: u64) -> u64,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSurfacePresentModes2EXT`."]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)"]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
		p_present_mode_count: *mut u32,
		p_present_modes: *mut PresentModeKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkGetDeviceGroupSurfacePresentModes2EXT`."]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html)"]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
		p_modes: *mut DeviceGroupPresentModeFlagsKHR,
	) -> Result,
>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkAcquireFullScreenExclusiveModeEXT`."]
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
#[doc = "\n\n[Khronos reference: `vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)"]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

#[cfg(feature = "platform-win32")]
#[doc = "Nullable function pointer for `vkReleaseFullScreenExclusiveModeEXT`."]
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
#[doc = "\n\n[Khronos reference: `vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)"]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR) -> Result>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`."]
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)"]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_counter_count: *mut u32,
		p_counters: *mut PerformanceCounterKHR<'_>,
		p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
		p_num_passes: *mut u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkAcquireProfilingLockKHR`."]
#[doc(alias = "vkAcquireProfilingLockKHR")]
#[doc = "\n\n[Khronos reference: `vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html)"]
pub type PFN_vkAcquireProfilingLockKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AcquireProfilingLockInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkReleaseProfilingLockKHR`."]
#[doc(alias = "vkReleaseProfilingLockKHR")]
#[doc = "\n\n[Khronos reference: `vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html)"]
pub type PFN_vkReleaseProfilingLockKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

#[doc = "Nullable function pointer for `vkGetImageDrmFormatModifierPropertiesEXT`."]
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html)"]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetBufferOpaqueCaptureAddress`."]
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html)"]
pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferDeviceAddressInfo<'_>,
	) -> u64,
>;

#[doc = "Nullable function pointer for `vkGetBufferOpaqueCaptureAddressKHR`."]
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html)"]
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;

#[doc = "Nullable function pointer for `vkGetBufferDeviceAddress`."]
#[doc(alias = "vkGetBufferDeviceAddress")]
#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)"]
pub type PFN_vkGetBufferDeviceAddress = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferDeviceAddressInfo<'_>,
	) -> DeviceAddress,
>;

#[doc = "Nullable function pointer for `vkGetBufferDeviceAddressKHR`."]
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html)"]
pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;

#[doc = "Nullable function pointer for `vkGetBufferDeviceAddressEXT`."]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html)"]
pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;

#[doc = "Nullable function pointer for `vkCreateHeadlessSurfaceEXT`."]
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html)"]
pub type PFN_vkCreateHeadlessSurfaceEXT = Option<
	unsafe extern "system" fn(
		instance: InstanceHandle,
		p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_surface: *mut SurfaceKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`."]
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)"]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_combination_count: *mut u32,
		p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkInitializePerformanceApiINTEL`."]
#[doc(alias = "vkInitializePerformanceApiINTEL")]
#[doc = "\n\n[Khronos reference: `vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html)"]
pub type PFN_vkInitializePerformanceApiINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkUninitializePerformanceApiINTEL`."]
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
#[doc = "\n\n[Khronos reference: `vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html)"]
pub type PFN_vkUninitializePerformanceApiINTEL =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetPerformanceMarkerINTEL`."]
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html)"]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetPerformanceStreamMarkerINTEL`."]
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html)"]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetPerformanceOverrideINTEL`."]
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html)"]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkAcquirePerformanceConfigurationINTEL`."]
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
#[doc = "\n\n[Khronos reference: `vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html)"]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
		p_configuration: *mut PerformanceConfigurationINTEL,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkReleasePerformanceConfigurationINTEL`."]
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
#[doc = "\n\n[Khronos reference: `vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html)"]
pub type PFN_vkReleasePerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		configuration: PerformanceConfigurationINTEL,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkQueueSetPerformanceConfigurationINTEL`."]
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
#[doc = "\n\n[Khronos reference: `vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html)"]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = Option<
	unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPerformanceParameterINTEL`."]
#[doc(alias = "vkGetPerformanceParameterINTEL")]
#[doc = "\n\n[Khronos reference: `vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html)"]
pub type PFN_vkGetPerformanceParameterINTEL = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		parameter: PerformanceParameterTypeINTEL,
		p_value: *mut PerformanceValueINTEL,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceMemoryOpaqueCaptureAddress`."]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html)"]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
	) -> u64,
>;

#[doc = "Nullable function pointer for `vkGetDeviceMemoryOpaqueCaptureAddressKHR`."]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)"]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = PFN_vkGetDeviceMemoryOpaqueCaptureAddress;

#[doc = "Nullable function pointer for `vkGetPipelineExecutablePropertiesKHR`."]
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html)"]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const PipelineInfoKHR<'_>,
		p_executable_count: *mut u32,
		p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPipelineExecutableStatisticsKHR`."]
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html)"]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_executable_info: *const PipelineExecutableInfoKHR<'_>,
		p_statistic_count: *mut u32,
		p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPipelineExecutableInternalRepresentationsKHR`."]
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html)"]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_executable_info: *const PipelineExecutableInfoKHR<'_>,
		p_internal_representation_count: *mut u32,
		p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetLineStipple`."]
#[doc(alias = "vkCmdSetLineStipple")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)"]
pub type PFN_vkCmdSetLineStipple = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		line_stipple_factor: u32,
		line_stipple_pattern: u16,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetLineStippleKHR`."]
#[doc(alias = "vkCmdSetLineStippleKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html)"]
pub type PFN_vkCmdSetLineStippleKHR = PFN_vkCmdSetLineStipple;

#[doc = "Nullable function pointer for `vkCmdSetLineStippleEXT`."]
#[doc(alias = "vkCmdSetLineStippleEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html)"]
pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceToolProperties`."]
#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html)"]
pub type PFN_vkGetPhysicalDeviceToolProperties = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_tool_count: *mut u32,
		p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceToolPropertiesEXT`."]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html)"]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = PFN_vkGetPhysicalDeviceToolProperties;

#[doc = "Nullable function pointer for `vkCreateAccelerationStructureKHR`."]
#[doc(alias = "vkCreateAccelerationStructureKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html)"]
pub type PFN_vkCreateAccelerationStructureKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBuildAccelerationStructuresKHR`."]
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html)"]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
		pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBuildAccelerationStructuresIndirectKHR`."]
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html)"]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
		p_indirect_device_addresses: *const DeviceAddress,
		p_indirect_strides: *const u32,
		pp_max_primitive_counts: *const *const u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBuildAccelerationStructuresKHR`."]
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
#[doc = "\n\n[Khronos reference: `vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html)"]
pub type PFN_vkBuildAccelerationStructuresKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		info_count: u32,
		p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
		pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetAccelerationStructureDeviceAddressKHR`."]
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html)"]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
	) -> DeviceAddress,
>;

#[doc = "Nullable function pointer for `vkCreateDeferredOperationKHR`."]
#[doc(alias = "vkCreateDeferredOperationKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html)"]
pub type PFN_vkCreateDeferredOperationKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_allocator: *const AllocationCallbacks,
		p_deferred_operation: *mut DeferredOperationKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDeferredOperationKHR`."]
#[doc(alias = "vkDestroyDeferredOperationKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html)"]
pub type PFN_vkDestroyDeferredOperationKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		operation: DeferredOperationKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeferredOperationMaxConcurrencyKHR`."]
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html)"]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
	Option<unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> u32>;

#[doc = "Nullable function pointer for `vkGetDeferredOperationResultKHR`."]
#[doc(alias = "vkGetDeferredOperationResultKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html)"]
pub type PFN_vkGetDeferredOperationResultKHR = Option<
	unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> Result,
>;

#[doc = "Nullable function pointer for `vkDeferredOperationJoinKHR`."]
#[doc(alias = "vkDeferredOperationJoinKHR")]
#[doc = "\n\n[Khronos reference: `vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html)"]
pub type PFN_vkDeferredOperationJoinKHR = Option<
	unsafe extern "system" fn(device: DeviceHandle, operation: DeferredOperationKHR) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPipelineIndirectMemoryRequirementsNV`."]
#[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html)"]
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ComputePipelineCreateInfo<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPipelineIndirectDeviceAddressNV`."]
#[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
#[doc = "\n\n[Khronos reference: `vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html)"]
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
	) -> DeviceAddress,
>;

#[doc = "Nullable function pointer for `vkAntiLagUpdateAMD`."]
#[doc(alias = "vkAntiLagUpdateAMD")]
#[doc = "\n\n[Khronos reference: `vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html)"]
pub type PFN_vkAntiLagUpdateAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_data: *const AntiLagDataAMD<'_>) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetCullMode`."]
#[doc(alias = "vkCmdSetCullMode")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html)"]
pub type PFN_vkCmdSetCullMode =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetCullModeEXT`."]
#[doc(alias = "vkCmdSetCullModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html)"]
pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;

#[doc = "Nullable function pointer for `vkCmdSetFrontFace`."]
#[doc(alias = "vkCmdSetFrontFace")]
#[doc = "\n\n[Khronos reference: `vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html)"]
pub type PFN_vkCmdSetFrontFace =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetFrontFaceEXT`."]
#[doc(alias = "vkCmdSetFrontFaceEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html)"]
pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;

#[doc = "Nullable function pointer for `vkCmdSetPrimitiveTopology`."]
#[doc(alias = "vkCmdSetPrimitiveTopology")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html)"]
pub type PFN_vkCmdSetPrimitiveTopology = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		primitive_topology: PrimitiveTopology,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetPrimitiveTopologyEXT`."]
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html)"]
pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;

#[doc = "Nullable function pointer for `vkCmdSetViewportWithCount`."]
#[doc(alias = "vkCmdSetViewportWithCount")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html)"]
pub type PFN_vkCmdSetViewportWithCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		viewport_count: u32,
		p_viewports: *const Viewport,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetViewportWithCountEXT`."]
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html)"]
pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;

#[doc = "Nullable function pointer for `vkCmdSetScissorWithCount`."]
#[doc(alias = "vkCmdSetScissorWithCount")]
#[doc = "\n\n[Khronos reference: `vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html)"]
pub type PFN_vkCmdSetScissorWithCount = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scissor_count: u32,
		p_scissors: *const Rect2D,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetScissorWithCountEXT`."]
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html)"]
pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;

#[doc = "Nullable function pointer for `vkCmdBindIndexBuffer2`."]
#[doc(alias = "vkCmdBindIndexBuffer2")]
#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html)"]
pub type PFN_vkCmdBindIndexBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer: Buffer,
		offset: DeviceSize,
		size: DeviceSize,
		index_type: IndexType,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindIndexBuffer2KHR`."]
#[doc(alias = "vkCmdBindIndexBuffer2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html)"]
pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;

#[doc = "Nullable function pointer for `vkCmdBindVertexBuffers2`."]
#[doc(alias = "vkCmdBindVertexBuffers2")]
#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBindVertexBuffers2EXT`."]
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html)"]
pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;

#[doc = "Nullable function pointer for `vkCmdSetDepthTestEnable`."]
#[doc(alias = "vkCmdSetDepthTestEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html)"]
pub type PFN_vkCmdSetDepthTestEnable =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetDepthTestEnableEXT`."]
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html)"]
pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;

#[doc = "Nullable function pointer for `vkCmdSetDepthWriteEnable`."]
#[doc(alias = "vkCmdSetDepthWriteEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html)"]
pub type PFN_vkCmdSetDepthWriteEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthWriteEnableEXT`."]
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html)"]
pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;

#[doc = "Nullable function pointer for `vkCmdSetDepthCompareOp`."]
#[doc(alias = "vkCmdSetDepthCompareOp")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html)"]
pub type PFN_vkCmdSetDepthCompareOp = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthCompareOpEXT`."]
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html)"]
pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;

#[doc = "Nullable function pointer for `vkCmdSetDepthBoundsTestEnable`."]
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html)"]
pub type PFN_vkCmdSetDepthBoundsTestEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthBoundsTestEnableEXT`."]
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html)"]
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;

#[doc = "Nullable function pointer for `vkCmdSetStencilTestEnable`."]
#[doc(alias = "vkCmdSetStencilTestEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html)"]
pub type PFN_vkCmdSetStencilTestEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetStencilTestEnableEXT`."]
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html)"]
pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;

#[doc = "Nullable function pointer for `vkCmdSetStencilOp`."]
#[doc(alias = "vkCmdSetStencilOp")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html)"]
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

#[doc = "Nullable function pointer for `vkCmdSetStencilOpEXT`."]
#[doc(alias = "vkCmdSetStencilOpEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html)"]
pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;

#[doc = "Nullable function pointer for `vkCmdSetPatchControlPointsEXT`."]
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html)"]
pub type PFN_vkCmdSetPatchControlPointsEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetRasterizerDiscardEnable`."]
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html)"]
pub type PFN_vkCmdSetRasterizerDiscardEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetRasterizerDiscardEnableEXT`."]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html)"]
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;

#[doc = "Nullable function pointer for `vkCmdSetDepthBiasEnable`."]
#[doc(alias = "vkCmdSetDepthBiasEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html)"]
pub type PFN_vkCmdSetDepthBiasEnable =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetDepthBiasEnableEXT`."]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html)"]
pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;

#[doc = "Nullable function pointer for `vkCmdSetLogicOpEXT`."]
#[doc(alias = "vkCmdSetLogicOpEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html)"]
pub type PFN_vkCmdSetLogicOpEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetPrimitiveRestartEnable`."]
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html)"]
pub type PFN_vkCmdSetPrimitiveRestartEnable = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetPrimitiveRestartEnableEXT`."]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html)"]
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;

#[doc = "Nullable function pointer for `vkCmdSetTessellationDomainOriginEXT`."]
#[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html)"]
pub type PFN_vkCmdSetTessellationDomainOriginEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		domain_origin: TessellationDomainOrigin,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthClampEnableEXT`."]
#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html)"]
pub type PFN_vkCmdSetDepthClampEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetPolygonModeEXT`."]
#[doc(alias = "vkCmdSetPolygonModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html)"]
pub type PFN_vkCmdSetPolygonModeEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetRasterizationSamplesEXT`."]
#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html)"]
pub type PFN_vkCmdSetRasterizationSamplesEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		rasterization_samples: SampleCountFlagBits,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetSampleMaskEXT`."]
#[doc(alias = "vkCmdSetSampleMaskEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html)"]
pub type PFN_vkCmdSetSampleMaskEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		samples: SampleCountFlagBits,
		p_sample_mask: *const SampleMask,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetAlphaToCoverageEnableEXT`."]
#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html)"]
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetAlphaToOneEnableEXT`."]
#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html)"]
pub type PFN_vkCmdSetAlphaToOneEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetLogicOpEnableEXT`."]
#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html)"]
pub type PFN_vkCmdSetLogicOpEnableEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetColorBlendEnableEXT`."]
#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html)"]
pub type PFN_vkCmdSetColorBlendEnableEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_enables: *const Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetColorBlendEquationEXT`."]
#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html)"]
pub type PFN_vkCmdSetColorBlendEquationEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_equations: *const ColorBlendEquationEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetColorWriteMaskEXT`."]
#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html)"]
pub type PFN_vkCmdSetColorWriteMaskEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_write_masks: *const ColorComponentFlags,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetRasterizationStreamEXT`."]
#[doc(alias = "vkCmdSetRasterizationStreamEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html)"]
pub type PFN_vkCmdSetRasterizationStreamEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetConservativeRasterizationModeEXT`."]
#[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html)"]
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		conservative_rasterization_mode: ConservativeRasterizationModeEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetExtraPrimitiveOverestimationSizeEXT`."]
#[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)"]
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		extra_primitive_overestimation_size: f32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthClipEnableEXT`."]
#[doc(alias = "vkCmdSetDepthClipEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html)"]
pub type PFN_vkCmdSetDepthClipEnableEXT =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetSampleLocationsEnableEXT`."]
#[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html)"]
pub type PFN_vkCmdSetSampleLocationsEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetColorBlendAdvancedEXT`."]
#[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html)"]
pub type PFN_vkCmdSetColorBlendAdvancedEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_attachment: u32,
		attachment_count: u32,
		p_color_blend_advanced: *const ColorBlendAdvancedEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetProvokingVertexModeEXT`."]
#[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html)"]
pub type PFN_vkCmdSetProvokingVertexModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		provoking_vertex_mode: ProvokingVertexModeEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetLineRasterizationModeEXT`."]
#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html)"]
pub type PFN_vkCmdSetLineRasterizationModeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		line_rasterization_mode: LineRasterizationModeEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetLineStippleEnableEXT`."]
#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html)"]
pub type PFN_vkCmdSetLineStippleEnableEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthClipNegativeOneToOneEXT`."]
#[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html)"]
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetViewportWScalingEnableNV`."]
#[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html)"]
pub type PFN_vkCmdSetViewportWScalingEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetViewportSwizzleNV`."]
#[doc(alias = "vkCmdSetViewportSwizzleNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html)"]
pub type PFN_vkCmdSetViewportSwizzleNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_viewport: u32,
		viewport_count: u32,
		p_viewport_swizzles: *const ViewportSwizzleNV,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageToColorEnableNV`."]
#[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html)"]
pub type PFN_vkCmdSetCoverageToColorEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageToColorLocationNV`."]
#[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html)"]
pub type PFN_vkCmdSetCoverageToColorLocationNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageModulationModeNV`."]
#[doc(alias = "vkCmdSetCoverageModulationModeNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html)"]
pub type PFN_vkCmdSetCoverageModulationModeNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_mode: CoverageModulationModeNV,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageModulationTableEnableNV`."]
#[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html)"]
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_table_enable: Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageModulationTableNV`."]
#[doc(alias = "vkCmdSetCoverageModulationTableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html)"]
pub type PFN_vkCmdSetCoverageModulationTableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_modulation_table_count: u32,
		p_coverage_modulation_table: *const f32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetShadingRateImageEnableNV`."]
#[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html)"]
pub type PFN_vkCmdSetShadingRateImageEnableNV = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetCoverageReductionModeNV`."]
#[doc(alias = "vkCmdSetCoverageReductionModeNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html)"]
pub type PFN_vkCmdSetCoverageReductionModeNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		coverage_reduction_mode: CoverageReductionModeNV,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetRepresentativeFragmentTestEnableNV`."]
#[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html)"]
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		representative_fragment_test_enable: Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreatePrivateDataSlot`."]
#[doc(alias = "vkCreatePrivateDataSlot")]
#[doc = "\n\n[Khronos reference: `vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html)"]
pub type PFN_vkCreatePrivateDataSlot = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const PrivateDataSlotCreateInfo<'_>,
		p_allocator: *const AllocationCallbacks,
		p_private_data_slot: *mut PrivateDataSlot,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreatePrivateDataSlotEXT`."]
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
#[doc = "\n\n[Khronos reference: `vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html)"]
pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;

#[doc = "Nullable function pointer for `vkDestroyPrivateDataSlot`."]
#[doc(alias = "vkDestroyPrivateDataSlot")]
#[doc = "\n\n[Khronos reference: `vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)"]
pub type PFN_vkDestroyPrivateDataSlot = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		private_data_slot: PrivateDataSlot,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroyPrivateDataSlotEXT`."]
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html)"]
pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;

#[doc = "Nullable function pointer for `vkSetPrivateData`."]
#[doc(alias = "vkSetPrivateData")]
#[doc = "\n\n[Khronos reference: `vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html)"]
pub type PFN_vkSetPrivateData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		object_type: ObjectType,
		object_handle: u64,
		private_data_slot: PrivateDataSlot,
		data: u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkSetPrivateDataEXT`."]
#[doc(alias = "vkSetPrivateDataEXT")]
#[doc = "\n\n[Khronos reference: `vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html)"]
pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;

#[doc = "Nullable function pointer for `vkGetPrivateData`."]
#[doc(alias = "vkGetPrivateData")]
#[doc = "\n\n[Khronos reference: `vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html)"]
pub type PFN_vkGetPrivateData = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		object_type: ObjectType,
		object_handle: u64,
		private_data_slot: PrivateDataSlot,
		p_data: *mut u64,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPrivateDataEXT`."]
#[doc(alias = "vkGetPrivateDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html)"]
pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;

#[doc = "Nullable function pointer for `vkCmdCopyBuffer2`."]
#[doc(alias = "vkCmdCopyBuffer2")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html)"]
pub type PFN_vkCmdCopyBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_buffer_info: *const CopyBufferInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyBuffer2KHR`."]
#[doc(alias = "vkCmdCopyBuffer2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html)"]
pub type PFN_vkCmdCopyBuffer2KHR = PFN_vkCmdCopyBuffer2;

#[doc = "Nullable function pointer for `vkCmdCopyImage2`."]
#[doc(alias = "vkCmdCopyImage2")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html)"]
pub type PFN_vkCmdCopyImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_image_info: *const CopyImageInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyImage2KHR`."]
#[doc(alias = "vkCmdCopyImage2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html)"]
pub type PFN_vkCmdCopyImage2KHR = PFN_vkCmdCopyImage2;

#[doc = "Nullable function pointer for `vkCmdBlitImage2`."]
#[doc(alias = "vkCmdBlitImage2")]
#[doc = "\n\n[Khronos reference: `vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html)"]
pub type PFN_vkCmdBlitImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_blit_image_info: *const BlitImageInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBlitImage2KHR`."]
#[doc(alias = "vkCmdBlitImage2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html)"]
pub type PFN_vkCmdBlitImage2KHR = PFN_vkCmdBlitImage2;

#[doc = "Nullable function pointer for `vkCmdCopyBufferToImage2`."]
#[doc(alias = "vkCmdCopyBufferToImage2")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html)"]
pub type PFN_vkCmdCopyBufferToImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyBufferToImage2KHR`."]
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html)"]
pub type PFN_vkCmdCopyBufferToImage2KHR = PFN_vkCmdCopyBufferToImage2;

#[doc = "Nullable function pointer for `vkCmdCopyImageToBuffer2`."]
#[doc(alias = "vkCmdCopyImageToBuffer2")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html)"]
pub type PFN_vkCmdCopyImageToBuffer2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyImageToBuffer2KHR`."]
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html)"]
pub type PFN_vkCmdCopyImageToBuffer2KHR = PFN_vkCmdCopyImageToBuffer2;

#[doc = "Nullable function pointer for `vkCmdResolveImage2`."]
#[doc(alias = "vkCmdResolveImage2")]
#[doc = "\n\n[Khronos reference: `vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html)"]
pub type PFN_vkCmdResolveImage2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_resolve_image_info: *const ResolveImageInfo2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdResolveImage2KHR`."]
#[doc(alias = "vkCmdResolveImage2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html)"]
pub type PFN_vkCmdResolveImage2KHR = PFN_vkCmdResolveImage2;

#[doc = "Nullable function pointer for `vkCmdSetFragmentShadingRateKHR`."]
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html)"]
pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_fragment_size: *const Extent2D,
		combiner_ops: *const FragmentShadingRateCombinerOpKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceFragmentShadingRatesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_fragment_shading_rate_count: *mut u32,
		p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetFragmentShadingRateEnumNV`."]
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html)"]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		shading_rate: FragmentShadingRateNV,
		combiner_ops: *const FragmentShadingRateCombinerOpKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetAccelerationStructureBuildSizesKHR`."]
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html)"]
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		build_type: AccelerationStructureBuildTypeKHR,
		p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
		p_max_primitive_counts: *const u32,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetVertexInputEXT`."]
#[doc(alias = "vkCmdSetVertexInputEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html)"]
pub type PFN_vkCmdSetVertexInputEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		vertex_binding_description_count: u32,
		p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
		vertex_attribute_description_count: u32,
		p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetColorWriteEnableEXT`."]
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html)"]
pub type PFN_vkCmdSetColorWriteEnableEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		attachment_count: u32,
		p_color_write_enables: *const Bool32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetEvent2`."]
#[doc(alias = "vkCmdSetEvent2")]
#[doc = "\n\n[Khronos reference: `vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html)"]
pub type PFN_vkCmdSetEvent2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		p_dependency_info: *const DependencyInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetEvent2KHR`."]
#[doc(alias = "vkCmdSetEvent2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html)"]
pub type PFN_vkCmdSetEvent2KHR = PFN_vkCmdSetEvent2;

#[doc = "Nullable function pointer for `vkCmdResetEvent2`."]
#[doc(alias = "vkCmdResetEvent2")]
#[doc = "\n\n[Khronos reference: `vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html)"]
pub type PFN_vkCmdResetEvent2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event: Event,
		stage_mask: PipelineStageFlags2,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdResetEvent2KHR`."]
#[doc(alias = "vkCmdResetEvent2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html)"]
pub type PFN_vkCmdResetEvent2KHR = PFN_vkCmdResetEvent2;

#[doc = "Nullable function pointer for `vkCmdWaitEvents2`."]
#[doc(alias = "vkCmdWaitEvents2")]
#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html)"]
pub type PFN_vkCmdWaitEvents2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		event_count: u32,
		p_events: *const Event,
		p_dependency_infos: *const DependencyInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdWaitEvents2KHR`."]
#[doc(alias = "vkCmdWaitEvents2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html)"]
pub type PFN_vkCmdWaitEvents2KHR = PFN_vkCmdWaitEvents2;

#[doc = "Nullable function pointer for `vkCmdPipelineBarrier2`."]
#[doc(alias = "vkCmdPipelineBarrier2")]
#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html)"]
pub type PFN_vkCmdPipelineBarrier2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dependency_info: *const DependencyInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPipelineBarrier2KHR`."]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html)"]
pub type PFN_vkCmdPipelineBarrier2KHR = PFN_vkCmdPipelineBarrier2;

#[doc = "Nullable function pointer for `vkQueueSubmit2`."]
#[doc(alias = "vkQueueSubmit2")]
#[doc = "\n\n[Khronos reference: `vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)"]
pub type PFN_vkQueueSubmit2 = Option<
	unsafe extern "system" fn(
		queue: Queue,
		submit_count: u32,
		p_submits: *const SubmitInfo2<'_>,
		fence: Fence,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkQueueSubmit2KHR`."]
#[doc(alias = "vkQueueSubmit2KHR")]
#[doc = "\n\n[Khronos reference: `vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html)"]
pub type PFN_vkQueueSubmit2KHR = PFN_vkQueueSubmit2;

#[doc = "Nullable function pointer for `vkCmdWriteTimestamp2`."]
#[doc(alias = "vkCmdWriteTimestamp2")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html)"]
pub type PFN_vkCmdWriteTimestamp2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage: PipelineStageFlags2,
		query_pool: QueryPool,
		query: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdWriteTimestamp2KHR`."]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html)"]
pub type PFN_vkCmdWriteTimestamp2KHR = PFN_vkCmdWriteTimestamp2;

#[doc = "Nullable function pointer for `vkCmdWriteBufferMarker2AMD`."]
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html)"]
pub type PFN_vkCmdWriteBufferMarker2AMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage: PipelineStageFlags2,
		dst_buffer: Buffer,
		dst_offset: DeviceSize,
		marker: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetQueueCheckpointData2NV`."]
#[doc(alias = "vkGetQueueCheckpointData2NV")]
#[doc = "\n\n[Khronos reference: `vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html)"]
pub type PFN_vkGetQueueCheckpointData2NV = Option<
	unsafe extern "system" fn(
		queue: Queue,
		p_checkpoint_data_count: *mut u32,
		p_checkpoint_data: *mut CheckpointData2NV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyMemoryToImage`."]
#[doc(alias = "vkCopyMemoryToImage")]
#[doc = "\n\n[Khronos reference: `vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html)"]
pub type PFN_vkCopyMemoryToImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCopyMemoryToImageEXT`."]
#[doc(alias = "vkCopyMemoryToImageEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html)"]
pub type PFN_vkCopyMemoryToImageEXT = PFN_vkCopyMemoryToImage;

#[doc = "Nullable function pointer for `vkCopyImageToMemory`."]
#[doc(alias = "vkCopyImageToMemory")]
#[doc = "\n\n[Khronos reference: `vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html)"]
pub type PFN_vkCopyImageToMemory = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCopyImageToMemoryEXT`."]
#[doc(alias = "vkCopyImageToMemoryEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html)"]
pub type PFN_vkCopyImageToMemoryEXT = PFN_vkCopyImageToMemory;

#[doc = "Nullable function pointer for `vkCopyImageToImage`."]
#[doc(alias = "vkCopyImageToImage")]
#[doc = "\n\n[Khronos reference: `vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html)"]
pub type PFN_vkCopyImageToImage = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCopyImageToImageEXT`."]
#[doc(alias = "vkCopyImageToImageEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html)"]
pub type PFN_vkCopyImageToImageEXT = PFN_vkCopyImageToImage;

#[doc = "Nullable function pointer for `vkTransitionImageLayout`."]
#[doc(alias = "vkTransitionImageLayout")]
#[doc = "\n\n[Khronos reference: `vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html)"]
pub type PFN_vkTransitionImageLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		transition_count: u32,
		p_transitions: *const HostImageLayoutTransitionInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkTransitionImageLayoutEXT`."]
#[doc(alias = "vkTransitionImageLayoutEXT")]
#[doc = "\n\n[Khronos reference: `vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html)"]
pub type PFN_vkTransitionImageLayoutEXT = PFN_vkTransitionImageLayout;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceVideoCapabilitiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_video_profile: *const VideoProfileInfoKHR<'_>,
		p_capabilities: *mut VideoCapabilitiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceVideoFormatPropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
		p_video_format_property_count: *mut u32,
		p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
		p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateVideoSessionKHR`."]
#[doc(alias = "vkCreateVideoSessionKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html)"]
pub type PFN_vkCreateVideoSessionKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const VideoSessionCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_video_session: *mut VideoSessionKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyVideoSessionKHR`."]
#[doc(alias = "vkDestroyVideoSessionKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html)"]
pub type PFN_vkDestroyVideoSessionKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateVideoSessionParametersKHR`."]
#[doc(alias = "vkCreateVideoSessionParametersKHR")]
#[doc = "\n\n[Khronos reference: `vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html)"]
pub type PFN_vkCreateVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_video_session_parameters: *mut VideoSessionParametersKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkUpdateVideoSessionParametersKHR`."]
#[doc(alias = "vkUpdateVideoSessionParametersKHR")]
#[doc = "\n\n[Khronos reference: `vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)"]
pub type PFN_vkUpdateVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session_parameters: VideoSessionParametersKHR,
		p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetEncodedVideoSessionParametersKHR`."]
#[doc(alias = "vkGetEncodedVideoSessionParametersKHR")]
#[doc = "\n\n[Khronos reference: `vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html)"]
pub type PFN_vkGetEncodedVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
		p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyVideoSessionParametersKHR`."]
#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
#[doc = "\n\n[Khronos reference: `vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)"]
pub type PFN_vkDestroyVideoSessionParametersKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session_parameters: VideoSessionParametersKHR,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetVideoSessionMemoryRequirementsKHR`."]
#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html)"]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		p_memory_requirements_count: *mut u32,
		p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkBindVideoSessionMemoryKHR`."]
#[doc(alias = "vkBindVideoSessionMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html)"]
pub type PFN_vkBindVideoSessionMemoryKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		video_session: VideoSessionKHR,
		bind_session_memory_info_count: u32,
		p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdDecodeVideoKHR`."]
#[doc(alias = "vkCmdDecodeVideoKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html)"]
pub type PFN_vkCmdDecodeVideoKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_decode_info: *const VideoDecodeInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginVideoCodingKHR`."]
#[doc(alias = "vkCmdBeginVideoCodingKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html)"]
pub type PFN_vkCmdBeginVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdControlVideoCodingKHR`."]
#[doc(alias = "vkCmdControlVideoCodingKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html)"]
pub type PFN_vkCmdControlVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndVideoCodingKHR`."]
#[doc(alias = "vkCmdEndVideoCodingKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html)"]
pub type PFN_vkCmdEndVideoCodingKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEncodeVideoKHR`."]
#[doc(alias = "vkCmdEncodeVideoKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html)"]
pub type PFN_vkCmdEncodeVideoKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_encode_info: *const VideoEncodeInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDecompressMemoryNV`."]
#[doc(alias = "vkCmdDecompressMemoryNV")]
#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html)"]
pub type PFN_vkCmdDecompressMemoryNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		decompress_region_count: u32,
		p_decompress_memory_regions: *const DecompressMemoryRegionNV,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDecompressMemoryIndirectCountNV`."]
#[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html)"]
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		indirect_commands_address: DeviceAddress,
		indirect_commands_count_address: DeviceAddress,
		stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPartitionedAccelerationStructuresBuildSizesNV`."]
#[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
#[doc = "\n\n[Khronos reference: `vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html)"]
pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const PartitionedAccelerationStructureInstancesInputNV<'_>,
		p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBuildPartitionedAccelerationStructuresNV`."]
#[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html)"]
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_build_info: *const BuildPartitionedAccelerationStructureInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDecompressMemoryEXT`."]
#[doc(alias = "vkCmdDecompressMemoryEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html)"]
pub type PFN_vkCmdDecompressMemoryEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDecompressMemoryIndirectCountEXT`."]
#[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCreateCuModuleNVX`."]
#[doc(alias = "vkCreateCuModuleNVX")]
#[doc = "\n\n[Khronos reference: `vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html)"]
pub type PFN_vkCreateCuModuleNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CuModuleCreateInfoNVX<'_>,
		p_allocator: *const AllocationCallbacks,
		p_module: *mut CuModuleNVX,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateCuFunctionNVX`."]
#[doc(alias = "vkCreateCuFunctionNVX")]
#[doc = "\n\n[Khronos reference: `vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html)"]
pub type PFN_vkCreateCuFunctionNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CuFunctionCreateInfoNVX<'_>,
		p_allocator: *const AllocationCallbacks,
		p_function: *mut CuFunctionNVX,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyCuModuleNVX`."]
#[doc(alias = "vkDestroyCuModuleNVX")]
#[doc = "\n\n[Khronos reference: `vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html)"]
pub type PFN_vkDestroyCuModuleNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CuModuleNVX,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkDestroyCuFunctionNVX`."]
#[doc(alias = "vkDestroyCuFunctionNVX")]
#[doc = "\n\n[Khronos reference: `vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html)"]
pub type PFN_vkDestroyCuFunctionNVX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		function: CuFunctionNVX,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCuLaunchKernelNVX`."]
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
#[doc = "\n\n[Khronos reference: `vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html)"]
pub type PFN_vkCmdCuLaunchKernelNVX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_launch_info: *const CuLaunchInfoNVX<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDescriptorSetLayoutSizeEXT`."]
#[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html)"]
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		layout: DescriptorSetLayout,
		p_layout_size_in_bytes: *mut DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDescriptorSetLayoutBindingOffsetEXT`."]
#[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html)"]
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		layout: DescriptorSetLayout,
		binding: u32,
		p_offset: *mut DeviceSize,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDescriptorEXT`."]
#[doc(alias = "vkGetDescriptorEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html)"]
pub type PFN_vkGetDescriptorEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
		data_size: usize,
		p_descriptor: *mut std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindDescriptorBuffersEXT`."]
#[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html)"]
pub type PFN_vkCmdBindDescriptorBuffersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		buffer_count: u32,
		p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetDescriptorBufferOffsetsEXT`."]
#[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBindDescriptorBufferEmbeddedSamplersEXT`."]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html)"]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		pipeline_bind_point: PipelineBindPoint,
		layout: PipelineLayout,
		set: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetBufferOpaqueCaptureDescriptorDataEXT`."]
#[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html)"]
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetImageOpaqueCaptureDescriptorDataEXT`."]
#[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html)"]
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetImageViewOpaqueCaptureDescriptorDataEXT`."]
#[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html)"]
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSamplerOpaqueCaptureDescriptorDataEXT`."]
#[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html)"]
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`."]
#[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)"]
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkSetDeviceMemoryPriorityEXT`."]
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
#[doc = "\n\n[Khronos reference: `vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html)"]
pub type PFN_vkSetDeviceMemoryPriorityEXT = Option<
	unsafe extern "system" fn(device: DeviceHandle, memory: DeviceMemory, priority: f32) -> (),
>;

#[doc = "Nullable function pointer for `vkAcquireDrmDisplayEXT`."]
#[doc(alias = "vkAcquireDrmDisplayEXT")]
#[doc = "\n\n[Khronos reference: `vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html)"]
pub type PFN_vkAcquireDrmDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		drm_fd: i32,
		display: DisplayKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDrmDisplayEXT`."]
#[doc(alias = "vkGetDrmDisplayEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html)"]
pub type PFN_vkGetDrmDisplayEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		drm_fd: i32,
		connector_id: u32,
		display: *mut DisplayKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkWaitForPresent2KHR`."]
#[doc(alias = "vkWaitForPresent2KHR")]
#[doc = "\n\n[Khronos reference: `vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html)"]
pub type PFN_vkWaitForPresent2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkWaitForPresentKHR`."]
#[doc(alias = "vkWaitForPresentKHR")]
#[doc = "\n\n[Khronos reference: `vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html)"]
pub type PFN_vkWaitForPresentKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		present_id: u64,
		timeout: u64,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkCreateBufferCollectionFUCHSIA`."]
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html)"]
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
		p_allocator: *const AllocationCallbacks,
		p_collection: *mut BufferCollectionFUCHSIA,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkSetBufferCollectionBufferConstraintsFUCHSIA`."]
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)"]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkSetBufferCollectionImageConstraintsFUCHSIA`."]
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html)"]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkDestroyBufferCollectionFUCHSIA`."]
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html)"]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "platform-fuchsia")]
#[doc = "Nullable function pointer for `vkGetBufferCollectionPropertiesFUCHSIA`."]
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
#[doc = "\n\n[Khronos reference: `vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html)"]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		collection: BufferCollectionFUCHSIA,
		p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCreateCudaModuleNV`."]
#[doc(alias = "vkCreateCudaModuleNV")]
#[doc = "\n\n[Khronos reference: `vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html)"]
pub type PFN_vkCreateCudaModuleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CudaModuleCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_module: *mut CudaModuleNV,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkGetCudaModuleCacheNV`."]
#[doc(alias = "vkGetCudaModuleCacheNV")]
#[doc = "\n\n[Khronos reference: `vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html)"]
pub type PFN_vkGetCudaModuleCacheNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CudaModuleNV,
		p_cache_size: *mut usize,
		p_cache_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCreateCudaFunctionNV`."]
#[doc(alias = "vkCreateCudaFunctionNV")]
#[doc = "\n\n[Khronos reference: `vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html)"]
pub type PFN_vkCreateCudaFunctionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const CudaFunctionCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_function: *mut CudaFunctionNV,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkDestroyCudaModuleNV`."]
#[doc(alias = "vkDestroyCudaModuleNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html)"]
pub type PFN_vkDestroyCudaModuleNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		module: CudaModuleNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkDestroyCudaFunctionNV`."]
#[doc(alias = "vkDestroyCudaFunctionNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html)"]
pub type PFN_vkDestroyCudaFunctionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		function: CudaFunctionNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCmdCudaLaunchKernelNV`."]
#[doc(alias = "vkCmdCudaLaunchKernelNV")]
#[doc = "\n\n[Khronos reference: `vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html)"]
pub type PFN_vkCmdCudaLaunchKernelNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_launch_info: *const CudaLaunchInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginRendering`."]
#[doc(alias = "vkCmdBeginRendering")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html)"]
pub type PFN_vkCmdBeginRendering = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_rendering_info: *const RenderingInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginRenderingKHR`."]
#[doc(alias = "vkCmdBeginRenderingKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html)"]
pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;

#[doc = "Nullable function pointer for `vkCmdEndRendering`."]
#[doc(alias = "vkCmdEndRendering")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html)"]
pub type PFN_vkCmdEndRendering =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkCmdEndRendering2KHR`."]
#[doc(alias = "vkCmdEndRendering2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html)"]
pub type PFN_vkCmdEndRendering2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndRendering2EXT`."]
#[doc(alias = "vkCmdEndRendering2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html)"]
pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;

#[doc = "Nullable function pointer for `vkCmdEndRenderingKHR`."]
#[doc(alias = "vkCmdEndRenderingKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html)"]
pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;

#[doc = "Nullable function pointer for `vkGetDescriptorSetLayoutHostMappingInfoVALVE`."]
#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)"]
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
		p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDescriptorSetHostMappingVALVE`."]
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)"]
pub type PFN_vkGetDescriptorSetHostMappingVALVE = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		descriptor_set: DescriptorSet,
		pp_data: *mut *mut std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateMicromapEXT`."]
#[doc(alias = "vkCreateMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html)"]
pub type PFN_vkCreateMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const MicromapCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_micromap: *mut MicromapEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBuildMicromapsEXT`."]
#[doc(alias = "vkCmdBuildMicromapsEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html)"]
pub type PFN_vkCmdBuildMicromapsEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const MicromapBuildInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBuildMicromapsEXT`."]
#[doc(alias = "vkBuildMicromapsEXT")]
#[doc = "\n\n[Khronos reference: `vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html)"]
pub type PFN_vkBuildMicromapsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		info_count: u32,
		p_infos: *const MicromapBuildInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyMicromapEXT`."]
#[doc(alias = "vkDestroyMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)"]
pub type PFN_vkDestroyMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		micromap: MicromapEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyMicromapEXT`."]
#[doc(alias = "vkCmdCopyMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html)"]
pub type PFN_vkCmdCopyMicromapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMicromapInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyMicromapEXT`."]
#[doc(alias = "vkCopyMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html)"]
pub type PFN_vkCopyMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMicromapInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyMicromapToMemoryEXT`."]
#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html)"]
pub type PFN_vkCmdCopyMicromapToMemoryEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyMicromapToMemoryEXT`."]
#[doc(alias = "vkCopyMicromapToMemoryEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html)"]
pub type PFN_vkCopyMicromapToMemoryEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryToMicromapEXT`."]
#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html)"]
pub type PFN_vkCmdCopyMemoryToMicromapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCopyMemoryToMicromapEXT`."]
#[doc(alias = "vkCopyMemoryToMicromapEXT")]
#[doc = "\n\n[Khronos reference: `vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html)"]
pub type PFN_vkCopyMemoryToMicromapEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdWriteMicromapsPropertiesEXT`."]
#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html)"]
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

#[doc = "Nullable function pointer for `vkWriteMicromapsPropertiesEXT`."]
#[doc(alias = "vkWriteMicromapsPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html)"]
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

#[doc = "Nullable function pointer for `vkGetDeviceMicromapCompatibilityEXT`."]
#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html)"]
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_version_info: *const MicromapVersionInfoEXT<'_>,
		p_compatibility: *mut AccelerationStructureCompatibilityKHR,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetMicromapBuildSizesEXT`."]
#[doc(alias = "vkGetMicromapBuildSizesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html)"]
pub type PFN_vkGetMicromapBuildSizesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		build_type: AccelerationStructureBuildTypeKHR,
		p_build_info: *const MicromapBuildInfoEXT<'_>,
		p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetShaderModuleIdentifierEXT`."]
#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
#[doc = "\n\n[Khronos reference: `vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)"]
pub type PFN_vkGetShaderModuleIdentifierEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader_module: ShaderModule,
		p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetShaderModuleCreateInfoIdentifierEXT`."]
#[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
#[doc = "\n\n[Khronos reference: `vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html)"]
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderModuleCreateInfo<'_>,
		p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetImageSubresourceLayout2`."]
#[doc(alias = "vkGetImageSubresourceLayout2")]
#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)"]
pub type PFN_vkGetImageSubresourceLayout2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image: Image,
		p_subresource: *const ImageSubresource2<'_>,
		p_layout: *mut SubresourceLayout2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetImageSubresourceLayout2KHR`."]
#[doc(alias = "vkGetImageSubresourceLayout2KHR")]
#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html)"]
pub type PFN_vkGetImageSubresourceLayout2KHR = PFN_vkGetImageSubresourceLayout2;

#[doc = "Nullable function pointer for `vkGetImageSubresourceLayout2EXT`."]
#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html)"]
pub type PFN_vkGetImageSubresourceLayout2EXT = PFN_vkGetImageSubresourceLayout2;

#[doc = "Nullable function pointer for `vkGetPipelinePropertiesEXT`."]
#[doc(alias = "vkGetPipelinePropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html)"]
pub type PFN_vkGetPipelinePropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const PipelineInfoKHR<'_>,
		p_pipeline_properties: *mut BaseOutStructure,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
#[doc = "Nullable function pointer for `vkExportMetalObjectsEXT`."]
#[doc(alias = "vkExportMetalObjectsEXT")]
#[doc = "\n\n[Khronos reference: `vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html)"]
pub type PFN_vkExportMetalObjectsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindTileMemoryQCOM`."]
#[doc(alias = "vkCmdBindTileMemoryQCOM")]
#[doc = "\n\n[Khronos reference: `vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html)"]
pub type PFN_vkCmdBindTileMemoryQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetFramebufferTilePropertiesQCOM`."]
#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
#[doc = "\n\n[Khronos reference: `vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)"]
pub type PFN_vkGetFramebufferTilePropertiesQCOM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		framebuffer: Framebuffer,
		p_properties_count: *mut u32,
		p_properties: *mut TilePropertiesQCOM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDynamicRenderingTilePropertiesQCOM`."]
#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
#[doc = "\n\n[Khronos reference: `vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html)"]
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_rendering_info: *const RenderingInfo<'_>,
		p_properties: *mut TilePropertiesQCOM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceOpticalFlowImageFormatsNV`."]
#[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)"]
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
		p_format_count: *mut u32,
		p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateOpticalFlowSessionNV`."]
#[doc(alias = "vkCreateOpticalFlowSessionNV")]
#[doc = "\n\n[Khronos reference: `vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html)"]
pub type PFN_vkCreateOpticalFlowSessionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_session: *mut OpticalFlowSessionNV,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyOpticalFlowSessionNV`."]
#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html)"]
pub type PFN_vkDestroyOpticalFlowSessionNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: OpticalFlowSessionNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindOpticalFlowSessionImageNV`."]
#[doc(alias = "vkBindOpticalFlowSessionImageNV")]
#[doc = "\n\n[Khronos reference: `vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html)"]
pub type PFN_vkBindOpticalFlowSessionImageNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: OpticalFlowSessionNV,
		binding_point: OpticalFlowSessionBindingPointNV,
		view: ImageView,
		layout: ImageLayout,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdOpticalFlowExecuteNV`."]
#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
#[doc = "\n\n[Khronos reference: `vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html)"]
pub type PFN_vkCmdOpticalFlowExecuteNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		session: OpticalFlowSessionNV,
		p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceFaultInfoEXT`."]
#[doc(alias = "vkGetDeviceFaultInfoEXT")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html)"]
pub type PFN_vkGetDeviceFaultInfoEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
		p_fault_info: *mut DeviceFaultInfoEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceFaultReportsKHR`."]
#[doc(alias = "vkGetDeviceFaultReportsKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html)"]
pub type PFN_vkGetDeviceFaultReportsKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		timeout: u64,
		p_fault_counts: *mut u32,
		p_fault_info: *mut DeviceFaultInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceFaultDebugInfoKHR`."]
#[doc(alias = "vkGetDeviceFaultDebugInfoKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html)"]
pub type PFN_vkGetDeviceFaultDebugInfoKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_debug_info: *mut DeviceFaultDebugInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetDepthBias2EXT`."]
#[doc(alias = "vkCmdSetDepthBias2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html)"]
pub type PFN_vkCmdSetDepthBias2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkReleaseSwapchainImagesKHR`."]
#[doc(alias = "vkReleaseSwapchainImagesKHR")]
#[doc = "\n\n[Khronos reference: `vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html)"]
pub type PFN_vkReleaseSwapchainImagesKHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkReleaseSwapchainImagesEXT`."]
#[doc(alias = "vkReleaseSwapchainImagesEXT")]
#[doc = "\n\n[Khronos reference: `vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html)"]
pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;

#[doc = "Nullable function pointer for `vkGetDeviceImageSubresourceLayout`."]
#[doc(alias = "vkGetDeviceImageSubresourceLayout")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html)"]
pub type PFN_vkGetDeviceImageSubresourceLayout = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceImageSubresourceInfo<'_>,
		p_layout: *mut SubresourceLayout2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDeviceImageSubresourceLayoutKHR`."]
#[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html)"]
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = PFN_vkGetDeviceImageSubresourceLayout;

#[doc = "Nullable function pointer for `vkMapMemory2`."]
#[doc(alias = "vkMapMemory2")]
#[doc = "\n\n[Khronos reference: `vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html)"]
pub type PFN_vkMapMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_map_info: *const MemoryMapInfo<'_>,
		pp_data: *mut *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkMapMemory2KHR`."]
#[doc(alias = "vkMapMemory2KHR")]
#[doc = "\n\n[Khronos reference: `vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html)"]
pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;

#[doc = "Nullable function pointer for `vkUnmapMemory2`."]
#[doc(alias = "vkUnmapMemory2")]
#[doc = "\n\n[Khronos reference: `vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html)"]
pub type PFN_vkUnmapMemory2 = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkUnmapMemory2KHR`."]
#[doc(alias = "vkUnmapMemory2KHR")]
#[doc = "\n\n[Khronos reference: `vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html)"]
pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;

#[doc = "Nullable function pointer for `vkCreateShadersEXT`."]
#[doc(alias = "vkCreateShadersEXT")]
#[doc = "\n\n[Khronos reference: `vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html)"]
pub type PFN_vkCreateShadersEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		create_info_count: u32,
		p_create_infos: *const ShaderCreateInfoEXT<'_>,
		p_allocator: *const AllocationCallbacks,
		p_shaders: *mut ShaderEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyShaderEXT`."]
#[doc(alias = "vkDestroyShaderEXT")]
#[doc = "\n\n[Khronos reference: `vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html)"]
pub type PFN_vkDestroyShaderEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader: ShaderEXT,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetShaderBinaryDataEXT`."]
#[doc(alias = "vkGetShaderBinaryDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html)"]
pub type PFN_vkGetShaderBinaryDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		shader: ShaderEXT,
		p_data_size: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBindShadersEXT`."]
#[doc(alias = "vkCmdBindShadersEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html)"]
pub type PFN_vkCmdBindShadersEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		stage_count: u32,
		p_stages: *const ShaderStageFlagBits,
		p_shaders: *const ShaderEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkSetSwapchainPresentTimingQueueSizeEXT`."]
#[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
#[doc = "\n\n[Khronos reference: `vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html)"]
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT = Option<
	unsafe extern "system" fn(device: DeviceHandle, swapchain: SwapchainKHR, size: u32) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSwapchainTimingPropertiesEXT`."]
#[doc(alias = "vkGetSwapchainTimingPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html)"]
pub type PFN_vkGetSwapchainTimingPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
		p_swapchain_timing_properties_counter: *mut u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetSwapchainTimeDomainPropertiesEXT`."]
#[doc(alias = "vkGetSwapchainTimeDomainPropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html)"]
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
		p_time_domains_counter: *mut u64,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPastPresentationTimingEXT`."]
#[doc(alias = "vkGetPastPresentationTimingEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html)"]
pub type PFN_vkGetPastPresentationTimingEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
		p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-screen")]
#[doc = "Nullable function pointer for `vkGetScreenBufferPropertiesQNX`."]
#[doc(alias = "vkGetScreenBufferPropertiesQNX")]
#[doc = "\n\n[Khronos reference: `vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html)"]
pub type PFN_vkGetScreenBufferPropertiesQNX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const _screen_buffer,
		p_properties: *mut ScreenBufferPropertiesQNX<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`."]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html)"]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`."]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixProperties2EXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html)"]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT<'_>,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixProperties2EXT<'_>,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkGetExecutionGraphPipelineScratchSizeAMDX`."]
#[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
#[doc = "\n\n[Khronos reference: `vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html)"]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		execution_graph: Pipeline,
		p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkGetExecutionGraphPipelineNodeIndexAMDX`."]
#[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
#[doc = "\n\n[Khronos reference: `vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html)"]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		execution_graph: Pipeline,
		p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
		p_node_index: *mut u32,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCreateExecutionGraphPipelinesAMDX`."]
#[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
#[doc = "\n\n[Khronos reference: `vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html)"]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCmdInitializeGraphScratchMemoryAMDX`."]
#[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
#[doc = "\n\n[Khronos reference: `vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html)"]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		execution_graph: Pipeline,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
	) -> (),
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCmdDispatchGraphAMDX`."]
#[doc(alias = "vkCmdDispatchGraphAMDX")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html)"]
pub type PFN_vkCmdDispatchGraphAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		p_count_info: *const DispatchGraphCountInfoAMDX,
	) -> (),
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCmdDispatchGraphIndirectAMDX`."]
#[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html)"]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		p_count_info: *const DispatchGraphCountInfoAMDX,
	) -> (),
>;

#[cfg(feature = "provisional")]
#[doc = "Nullable function pointer for `vkCmdDispatchGraphIndirectCountAMDX`."]
#[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html)"]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		scratch: DeviceAddress,
		scratch_size: DeviceSize,
		count_info: DeviceAddress,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateGpaSessionAMD`."]
#[doc(alias = "vkCreateGpaSessionAMD")]
#[doc = "\n\n[Khronos reference: `vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html)"]
pub type PFN_vkCreateGpaSessionAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const GpaSessionCreateInfoAMD<'_>,
		p_allocator: *const AllocationCallbacks,
		p_gpa_session: *mut GpaSessionAMD,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyGpaSessionAMD`."]
#[doc(alias = "vkDestroyGpaSessionAMD")]
#[doc = "\n\n[Khronos reference: `vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html)"]
pub type PFN_vkDestroyGpaSessionAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		gpa_session: GpaSessionAMD,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkSetGpaDeviceClockModeAMD`."]
#[doc(alias = "vkSetGpaDeviceClockModeAMD")]
#[doc = "\n\n[Khronos reference: `vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html)"]
pub type PFN_vkSetGpaDeviceClockModeAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *mut GpaDeviceClockModeInfoAMD<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetGpaDeviceClockInfoAMD`."]
#[doc(alias = "vkGetGpaDeviceClockInfoAMD")]
#[doc = "\n\n[Khronos reference: `vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html)"]
pub type PFN_vkGetGpaDeviceClockInfoAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *mut GpaDeviceGetClockInfoAMD<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBeginGpaSessionAMD`."]
#[doc(alias = "vkCmdBeginGpaSessionAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html)"]
pub type PFN_vkCmdBeginGpaSessionAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdEndGpaSessionAMD`."]
#[doc(alias = "vkCmdEndGpaSessionAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html)"]
pub type PFN_vkCmdEndGpaSessionAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBeginGpaSampleAMD`."]
#[doc(alias = "vkCmdBeginGpaSampleAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html)"]
pub type PFN_vkCmdBeginGpaSampleAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		gpa_session: GpaSessionAMD,
		p_gpa_sample_begin_info: *const GpaSampleBeginInfoAMD<'_>,
		p_sample_id: *mut u32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdEndGpaSampleAMD`."]
#[doc(alias = "vkCmdEndGpaSampleAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html)"]
pub type PFN_vkCmdEndGpaSampleAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		gpa_session: GpaSessionAMD,
		sample_id: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetGpaSessionStatusAMD`."]
#[doc(alias = "vkGetGpaSessionStatusAMD")]
#[doc = "\n\n[Khronos reference: `vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html)"]
pub type PFN_vkGetGpaSessionStatusAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, gpa_session: GpaSessionAMD) -> Result>;

#[doc = "Nullable function pointer for `vkGetGpaSessionResultsAMD`."]
#[doc(alias = "vkGetGpaSessionResultsAMD")]
#[doc = "\n\n[Khronos reference: `vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html)"]
pub type PFN_vkGetGpaSessionResultsAMD = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		gpa_session: GpaSessionAMD,
		sample_id: u32,
		p_size_in_bytes: *mut usize,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkResetGpaSessionAMD`."]
#[doc(alias = "vkResetGpaSessionAMD")]
#[doc = "\n\n[Khronos reference: `vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html)"]
pub type PFN_vkResetGpaSessionAMD =
	Option<unsafe extern "system" fn(device: DeviceHandle, gpa_session: GpaSessionAMD) -> Result>;

#[doc = "Nullable function pointer for `vkCmdCopyGpaSessionResultsAMD`."]
#[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html)"]
pub type PFN_vkCmdCopyGpaSessionResultsAMD = Option<
	unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindDescriptorSets2`."]
#[doc(alias = "vkCmdBindDescriptorSets2")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html)"]
pub type PFN_vkCmdBindDescriptorSets2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindDescriptorSets2KHR`."]
#[doc(alias = "vkCmdBindDescriptorSets2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html)"]
pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;

#[doc = "Nullable function pointer for `vkCmdPushConstants2`."]
#[doc(alias = "vkCmdPushConstants2")]
#[doc = "\n\n[Khronos reference: `vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html)"]
pub type PFN_vkCmdPushConstants2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_constants_info: *const PushConstantsInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushConstants2KHR`."]
#[doc(alias = "vkCmdPushConstants2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html)"]
pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSet2`."]
#[doc(alias = "vkCmdPushDescriptorSet2")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html)"]
pub type PFN_vkCmdPushDescriptorSet2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSet2KHR`."]
#[doc(alias = "vkCmdPushDescriptorSet2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html)"]
pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSetWithTemplate2`."]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html)"]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushDescriptorSetWithTemplate2KHR`."]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html)"]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;

#[doc = "Nullable function pointer for `vkCmdSetDescriptorBufferOffsets2EXT`."]
#[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html)"]
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`."]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html)"]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = Option<unsafe extern "system" fn(command_buffer: CommandBuffer, p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>) -> ()>;

#[doc = "Nullable function pointer for `vkSetLatencySleepModeNV`."]
#[doc(alias = "vkSetLatencySleepModeNV")]
#[doc = "\n\n[Khronos reference: `vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html)"]
pub type PFN_vkSetLatencySleepModeNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkLatencySleepNV`."]
#[doc(alias = "vkLatencySleepNV")]
#[doc = "\n\n[Khronos reference: `vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html)"]
pub type PFN_vkLatencySleepNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_sleep_info: *const LatencySleepInfoNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkSetLatencyMarkerNV`."]
#[doc(alias = "vkSetLatencyMarkerNV")]
#[doc = "\n\n[Khronos reference: `vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html)"]
pub type PFN_vkSetLatencyMarkerNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetLatencyTimingsNV`."]
#[doc(alias = "vkGetLatencyTimingsNV")]
#[doc = "\n\n[Khronos reference: `vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html)"]
pub type PFN_vkGetLatencyTimingsNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		swapchain: SwapchainKHR,
		p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkQueueNotifyOutOfBandNV`."]
#[doc(alias = "vkQueueNotifyOutOfBandNV")]
#[doc = "\n\n[Khronos reference: `vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html)"]
pub type PFN_vkQueueNotifyOutOfBandNV = Option<
	unsafe extern "system" fn(
		queue: Queue,
		p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkSetLatencySleepModeLegacyNV`."]
#[doc(alias = "vkSetLatencySleepModeLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkSetLatencySleepModeLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html)"]
pub type PFN_vkSetLatencySleepModeLegacyNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		low_latency_mode: Bool32,
		low_latency_boost: Bool32,
		minimum_interval_us: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkLatencySleepLegacyNV`."]
#[doc(alias = "vkLatencySleepLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkLatencySleepLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html)"]
pub type PFN_vkLatencySleepLegacyNV = Option<
	unsafe extern "system" fn(device: DeviceHandle, signal_semaphore: Semaphore, value: u64) -> (),
>;

#[doc = "Nullable function pointer for `vkSetLatencyMarkerLegacyNV`."]
#[doc(alias = "vkSetLatencyMarkerLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkSetLatencyMarkerLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html)"]
pub type PFN_vkSetLatencyMarkerLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, frame_id: u64, marker: u32) -> ()>;

#[doc = "Nullable function pointer for `vkGetLatencyTimingsLegacyNV`."]
#[doc(alias = "vkGetLatencyTimingsLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkGetLatencyTimingsLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html)"]
pub type PFN_vkGetLatencyTimingsLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_timings: *mut std::ffi::c_void) -> ()>;

#[doc = "Nullable function pointer for `vkQueueNotifyOutOfBandLegacyNV`."]
#[doc(alias = "vkQueueNotifyOutOfBandLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkQueueNotifyOutOfBandLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html)"]
pub type PFN_vkQueueNotifyOutOfBandLegacyNV =
	Option<unsafe extern "system" fn(queue: Queue, queue_type: u32) -> ()>;

#[doc = "Nullable function pointer for `vkGetSleepStatusLegacyNV`."]
#[doc(alias = "vkGetSleepStatusLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkGetSleepStatusLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html)"]
pub type PFN_vkGetSleepStatusLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle, p_low_latency_mode: *mut Bool32) -> ()>;

#[doc = "Nullable function pointer for `vkShutdownLatencyDeviceLegacyNV`."]
#[doc(alias = "vkShutdownLatencyDeviceLegacyNV")]
#[doc = "\n\n[Khronos reference: `vkShutdownLatencyDeviceLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html)"]
pub type PFN_vkShutdownLatencyDeviceLegacyNV =
	Option<unsafe extern "system" fn(device: DeviceHandle) -> ()>;

#[doc = "Nullable function pointer for `vkCmdSetRenderingAttachmentLocations`."]
#[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html)"]
pub type PFN_vkCmdSetRenderingAttachmentLocations = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_location_info: *const RenderingAttachmentLocationInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetRenderingAttachmentLocationsKHR`."]
#[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html)"]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;

#[doc = "Nullable function pointer for `vkCmdSetRenderingInputAttachmentIndices`."]
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html)"]
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdSetRenderingInputAttachmentIndicesKHR`."]
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html)"]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
	PFN_vkCmdSetRenderingInputAttachmentIndices;

#[doc = "Nullable function pointer for `vkCmdSetDepthClampRangeEXT`."]
#[doc(alias = "vkCmdSetDepthClampRangeEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html)"]
pub type PFN_vkCmdSetDepthClampRangeEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		depth_clamp_mode: DepthClampModeEXT,
		p_depth_clamp_range: *const DepthClampRangeEXT,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`."]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html)"]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
#[doc = "Nullable function pointer for `vkGetMemoryMetalHandleEXT`."]
#[doc(alias = "vkGetMemoryMetalHandleEXT")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html)"]
pub type PFN_vkGetMemoryMetalHandleEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT<'_>,
		p_handle: *mut *mut std::ffi::c_void,
	) -> Result,
>;

#[cfg(feature = "platform-metal")]
#[doc = "Nullable function pointer for `vkGetMemoryMetalHandlePropertiesEXT`."]
#[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html)"]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		handle_type: ExternalMemoryHandleTypeFlagBits,
		p_handle: *const std::ffi::c_void,
		p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceCooperativeVectorPropertiesNV`."]
#[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html)"]
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_property_count: *mut u32,
		p_properties: *mut CooperativeVectorPropertiesNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkConvertCooperativeVectorMatrixNV`."]
#[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
#[doc = "\n\n[Khronos reference: `vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html)"]
pub type PFN_vkConvertCooperativeVectorMatrixNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdConvertCooperativeVectorMatrixNV`."]
#[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
#[doc = "\n\n[Khronos reference: `vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html)"]
pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		info_count: u32,
		p_infos: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDispatchTileQCOM`."]
#[doc(alias = "vkCmdDispatchTileQCOM")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html)"]
pub type PFN_vkCmdDispatchTileQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dispatch_tile_info: *const DispatchTileInfoQCOM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginPerTileExecutionQCOM`."]
#[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html)"]
pub type PFN_vkCmdBeginPerTileExecutionQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_per_tile_begin_info: *const PerTileBeginInfoQCOM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndPerTileExecutionQCOM`."]
#[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
#[doc = "\n\n[Khronos reference: `vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html)"]
pub type PFN_vkCmdEndPerTileExecutionQCOM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_per_tile_end_info: *const PerTileEndInfoQCOM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateExternalComputeQueueNV`."]
#[doc(alias = "vkCreateExternalComputeQueueNV")]
#[doc = "\n\n[Khronos reference: `vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html)"]
pub type PFN_vkCreateExternalComputeQueueNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ExternalComputeQueueCreateInfoNV<'_>,
		p_allocator: *const AllocationCallbacks,
		p_external_queue: *mut ExternalComputeQueueNV,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyExternalComputeQueueNV`."]
#[doc(alias = "vkDestroyExternalComputeQueueNV")]
#[doc = "\n\n[Khronos reference: `vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html)"]
pub type PFN_vkDestroyExternalComputeQueueNV = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		external_queue: ExternalComputeQueueNV,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetExternalComputeQueueDataNV`."]
#[doc(alias = "vkGetExternalComputeQueueDataNV")]
#[doc = "\n\n[Khronos reference: `vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html)"]
pub type PFN_vkGetExternalComputeQueueDataNV = Option<
	unsafe extern "system" fn(
		external_queue: ExternalComputeQueueNV,
		params: *mut ExternalComputeQueueDataParamsNV<'_>,
		p_data: *mut std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`."]
#[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html)"]
pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_description_count: *mut u32,
		p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateShaderInstrumentationARM`."]
#[doc(alias = "vkCreateShaderInstrumentationARM")]
#[doc = "\n\n[Khronos reference: `vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html)"]
pub type PFN_vkCreateShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const ShaderInstrumentationCreateInfoARM<'_>,
		p_allocator: *const AllocationCallbacks,
		p_instrumentation: *mut ShaderInstrumentationARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyShaderInstrumentationARM`."]
#[doc(alias = "vkDestroyShaderInstrumentationARM")]
#[doc = "\n\n[Khronos reference: `vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html)"]
pub type PFN_vkDestroyShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		instrumentation: ShaderInstrumentationARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginShaderInstrumentationARM`."]
#[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html)"]
pub type PFN_vkCmdBeginShaderInstrumentationARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		instrumentation: ShaderInstrumentationARM,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndShaderInstrumentationARM`."]
#[doc(alias = "vkCmdEndShaderInstrumentationARM")]
#[doc = "\n\n[Khronos reference: `vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html)"]
pub type PFN_vkCmdEndShaderInstrumentationARM =
	Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> ()>;

#[doc = "Nullable function pointer for `vkGetShaderInstrumentationValuesARM`."]
#[doc(alias = "vkGetShaderInstrumentationValuesARM")]
#[doc = "\n\n[Khronos reference: `vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html)"]
pub type PFN_vkGetShaderInstrumentationValuesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		instrumentation: ShaderInstrumentationARM,
		p_metric_block_count: *mut u32,
		p_metric_values: *mut std::ffi::c_void,
		flags: ShaderInstrumentationValuesFlagsARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkClearShaderInstrumentationMetricsARM`."]
#[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
#[doc = "\n\n[Khronos reference: `vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html)"]
pub type PFN_vkClearShaderInstrumentationMetricsARM = Option<
	unsafe extern "system" fn(device: DeviceHandle, instrumentation: ShaderInstrumentationARM) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateTensorARM`."]
#[doc(alias = "vkCreateTensorARM")]
#[doc = "\n\n[Khronos reference: `vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html)"]
pub type PFN_vkCreateTensorARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const TensorCreateInfoARM<'_>,
		p_allocator: *const AllocationCallbacks,
		p_tensor: *mut TensorARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyTensorARM`."]
#[doc(alias = "vkDestroyTensorARM")]
#[doc = "\n\n[Khronos reference: `vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html)"]
pub type PFN_vkDestroyTensorARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor: TensorARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateTensorViewARM`."]
#[doc(alias = "vkCreateTensorViewARM")]
#[doc = "\n\n[Khronos reference: `vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html)"]
pub type PFN_vkCreateTensorViewARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const TensorViewCreateInfoARM<'_>,
		p_allocator: *const AllocationCallbacks,
		p_view: *mut TensorViewARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyTensorViewARM`."]
#[doc(alias = "vkDestroyTensorViewARM")]
#[doc = "\n\n[Khronos reference: `vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html)"]
pub type PFN_vkDestroyTensorViewARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor_view: TensorViewARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetTensorMemoryRequirementsARM`."]
#[doc(alias = "vkGetTensorMemoryRequirementsARM")]
#[doc = "\n\n[Khronos reference: `vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html)"]
pub type PFN_vkGetTensorMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorMemoryRequirementsInfoARM<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindTensorMemoryARM`."]
#[doc(alias = "vkBindTensorMemoryARM")]
#[doc = "\n\n[Khronos reference: `vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html)"]
pub type PFN_vkBindTensorMemoryARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDeviceTensorMemoryRequirementsARM`."]
#[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
#[doc = "\n\n[Khronos reference: `vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html)"]
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyTensorARM`."]
#[doc(alias = "vkCmdCopyTensorARM")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html)"]
pub type PFN_vkCmdCopyTensorARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetTensorOpaqueCaptureDescriptorDataARM`."]
#[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
#[doc = "\n\n[Khronos reference: `vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html)"]
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetTensorViewOpaqueCaptureDescriptorDataARM`."]
#[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
#[doc = "\n\n[Khronos reference: `vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html)"]
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
		p_data: *mut std::ffi::c_void,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceExternalTensorPropertiesARM`."]
#[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html)"]
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
		p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateDataGraphPipelinesARM`."]
#[doc(alias = "vkCreateDataGraphPipelinesARM")]
#[doc = "\n\n[Khronos reference: `vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html)"]
pub type PFN_vkCreateDataGraphPipelinesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		deferred_operation: DeferredOperationKHR,
		pipeline_cache: PipelineCache,
		create_info_count: u32,
		p_create_infos: *const DataGraphPipelineCreateInfoARM<'_>,
		p_allocator: *const AllocationCallbacks,
		p_pipelines: *mut Pipeline,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCreateDataGraphPipelineSessionARM`."]
#[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
#[doc = "\n\n[Khronos reference: `vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html)"]
pub type PFN_vkCreateDataGraphPipelineSessionARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const DataGraphPipelineSessionCreateInfoARM<'_>,
		p_allocator: *const AllocationCallbacks,
		p_session: *mut DataGraphPipelineSessionARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDataGraphPipelineSessionBindPointRequirementsARM`."]
#[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html)"]
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
		p_bind_point_requirement_count: *mut u32,
		p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDataGraphPipelineSessionMemoryRequirementsARM`."]
#[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html)"]
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
		p_memory_requirements: *mut MemoryRequirements2<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkBindDataGraphPipelineSessionMemoryARM`."]
#[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
#[doc = "\n\n[Khronos reference: `vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html)"]
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		bind_info_count: u32,
		p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkDestroyDataGraphPipelineSessionARM`."]
#[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
#[doc = "\n\n[Khronos reference: `vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html)"]
pub type PFN_vkDestroyDataGraphPipelineSessionARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		session: DataGraphPipelineSessionARM,
		p_allocator: *const AllocationCallbacks,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDispatchDataGraphARM`."]
#[doc(alias = "vkCmdDispatchDataGraphARM")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html)"]
pub type PFN_vkCmdDispatchDataGraphARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		session: DataGraphPipelineSessionARM,
		p_info: *const DataGraphPipelineDispatchInfoARM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetDataGraphPipelineAvailablePropertiesARM`."]
#[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html)"]
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
		p_properties_count: *mut u32,
		p_properties: *mut DataGraphPipelinePropertyARM,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetDataGraphPipelinePropertiesARM`."]
#[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html)"]
pub type PFN_vkGetDataGraphPipelinePropertiesARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
		properties_count: u32,
		p_properties: *mut DataGraphPipelinePropertyQueryResultARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_property_count: *mut u32,
		p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = Option<unsafe extern "system" fn(physical_device: PhysicalDevice, p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>, p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>) -> ()>;

#[cfg(feature = "platform-ohos")]
#[doc = "Nullable function pointer for `vkGetNativeBufferPropertiesOHOS`."]
#[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
#[doc = "\n\n[Khronos reference: `vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html)"]
pub type PFN_vkGetNativeBufferPropertiesOHOS = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		buffer: *const OH_NativeBuffer,
		p_properties: *mut NativeBufferPropertiesOHOS<'_>,
	) -> Result,
>;

#[cfg(feature = "platform-ohos")]
#[doc = "Nullable function pointer for `vkGetMemoryNativeBufferOHOS`."]
#[doc(alias = "vkGetMemoryNativeBufferOHOS")]
#[doc = "\n\n[Khronos reference: `vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html)"]
pub type PFN_vkGetMemoryNativeBufferOHOS = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
		p_buffer: *mut *mut OH_NativeBuffer,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkQueueSetPerfHintQCOM`."]
#[doc(alias = "vkQueueSetPerfHintQCOM")]
#[doc = "\n\n[Khronos reference: `vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html)"]
pub type PFN_vkQueueSetPerfHintQCOM = Option<
	unsafe extern "system" fn(queue: Queue, p_perf_hint_info: *const PerfHintInfoQCOM<'_>) -> Result,
>;

#[doc = "Nullable function pointer for `vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`."]
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")]
#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html)"]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_counter_count: *mut u32,
		p_counters: *mut PerformanceCounterARM<'_>,
		p_counter_descriptions: *mut PerformanceCounterDescriptionARM<'_>,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetComputeOccupancyPriorityNV`."]
#[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
#[doc = "\n\n[Khronos reference: `vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html)"]
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_parameters: *const ComputeOccupancyPriorityParametersNV<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkWriteSamplerDescriptorsEXT`."]
#[doc(alias = "vkWriteSamplerDescriptorsEXT")]
#[doc = "\n\n[Khronos reference: `vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html)"]
pub type PFN_vkWriteSamplerDescriptorsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		sampler_count: u32,
		p_samplers: *const SamplerCreateInfo<'_>,
		p_descriptors: *const HostAddressRangeEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkWriteResourceDescriptorsEXT`."]
#[doc(alias = "vkWriteResourceDescriptorsEXT")]
#[doc = "\n\n[Khronos reference: `vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html)"]
pub type PFN_vkWriteResourceDescriptorsEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		resource_count: u32,
		p_resources: *const ResourceDescriptorInfoEXT<'_>,
		p_descriptors: *const HostAddressRangeEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdBindSamplerHeapEXT`."]
#[doc(alias = "vkCmdBindSamplerHeapEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html)"]
pub type PFN_vkCmdBindSamplerHeapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_info: *const BindHeapInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindResourceHeapEXT`."]
#[doc(alias = "vkCmdBindResourceHeapEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html)"]
pub type PFN_vkCmdBindResourceHeapEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_bind_info: *const BindHeapInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdPushDataEXT`."]
#[doc(alias = "vkCmdPushDataEXT")]
#[doc = "\n\n[Khronos reference: `vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html)"]
pub type PFN_vkCmdPushDataEXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_push_data_info: *const PushDataInfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkRegisterCustomBorderColorEXT`."]
#[doc(alias = "vkRegisterCustomBorderColorEXT")]
#[doc = "\n\n[Khronos reference: `vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html)"]
pub type PFN_vkRegisterCustomBorderColorEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
		request_index: Bool32,
		p_index: *mut u32,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkUnregisterCustomBorderColorEXT`."]
#[doc(alias = "vkUnregisterCustomBorderColorEXT")]
#[doc = "\n\n[Khronos reference: `vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html)"]
pub type PFN_vkUnregisterCustomBorderColorEXT =
	Option<unsafe extern "system" fn(device: DeviceHandle, index: u32) -> ()>;

#[doc = "Nullable function pointer for `vkGetImageOpaqueCaptureDataEXT`."]
#[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
#[doc = "\n\n[Khronos reference: `vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html)"]
pub type PFN_vkGetImageOpaqueCaptureDataEXT = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		image_count: u32,
		p_images: *const Image,
		p_datas: *mut HostAddressRangeEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceDescriptorSizeEXT`."]
#[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html)"]
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		descriptor_type: DescriptorType,
	) -> DeviceSize,
>;

#[doc = "Nullable function pointer for `vkGetTensorOpaqueCaptureDataARM`."]
#[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
#[doc = "\n\n[Khronos reference: `vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html)"]
pub type PFN_vkGetTensorOpaqueCaptureDataARM = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		tensor_count: u32,
		p_tensors: *const TensorARM,
		p_datas: *mut HostAddressRangeEXT,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryKHR`."]
#[doc(alias = "vkCmdCopyMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html)"]
pub type PFN_vkCmdCopyMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyMemoryToImageKHR`."]
#[doc(alias = "vkCmdCopyMemoryToImageKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html)"]
pub type PFN_vkCmdCopyMemoryToImageKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyImageToMemoryKHR`."]
#[doc(alias = "vkCmdCopyImageToMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html)"]
pub type PFN_vkCmdCopyImageToMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdUpdateMemoryKHR`."]
#[doc(alias = "vkCmdUpdateMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html)"]
pub type PFN_vkCmdUpdateMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dst_range: *const DeviceAddressRangeKHR,
		dst_flags: AddressCommandFlagsKHR,
		data_size: DeviceSize,
		p_data: *const std::ffi::c_void,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdFillMemoryKHR`."]
#[doc(alias = "vkCmdFillMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html)"]
pub type PFN_vkCmdFillMemoryKHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dst_range: *const DeviceAddressRangeKHR,
		dst_flags: AddressCommandFlagsKHR,
		data: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdCopyQueryPoolResultsToMemoryKHR`."]
#[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
#[doc = "\n\n[Khronos reference: `vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html)"]
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

#[doc = "Nullable function pointer for `vkCmdBeginConditionalRendering2EXT`."]
#[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html)"]
pub type PFN_vkCmdBeginConditionalRendering2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindTransformFeedbackBuffers2EXT`."]
#[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html)"]
pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBeginTransformFeedback2EXT`."]
#[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html)"]
pub type PFN_vkCmdBeginTransformFeedback2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_range: u32,
		counter_range_count: u32,
		p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdEndTransformFeedback2EXT`."]
#[doc(alias = "vkCmdEndTransformFeedback2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html)"]
pub type PFN_vkCmdEndTransformFeedback2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_counter_range: u32,
		counter_range_count: u32,
		p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndirectByteCount2EXT`."]
#[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html)"]
pub type PFN_vkCmdDrawIndirectByteCount2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		instance_count: u32,
		first_instance: u32,
		p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
		counter_offset: u32,
		vertex_stride: u32,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdWriteMarkerToMemoryAMD`."]
#[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
#[doc = "\n\n[Khronos reference: `vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html)"]
pub type PFN_vkCmdWriteMarkerToMemoryAMD = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const MemoryMarkerInfoAMD<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindIndexBuffer3KHR`."]
#[doc(alias = "vkCmdBindIndexBuffer3KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html)"]
pub type PFN_vkCmdBindIndexBuffer3KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const BindIndexBuffer3InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdBindVertexBuffers3KHR`."]
#[doc(alias = "vkCmdBindVertexBuffers3KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html)"]
pub type PFN_vkCmdBindVertexBuffers3KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		first_binding: u32,
		binding_count: u32,
		p_binding_infos: *const BindVertexBuffer3InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndirect2KHR`."]
#[doc(alias = "vkCmdDrawIndirect2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html)"]
pub type PFN_vkCmdDrawIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirect2KHR`."]
#[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html)"]
pub type PFN_vkCmdDrawIndexedIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndirectCount2KHR`."]
#[doc(alias = "vkCmdDrawIndirectCount2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html)"]
pub type PFN_vkCmdDrawIndirectCount2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawIndexedIndirectCount2KHR`."]
#[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html)"]
pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirect2EXT`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html)"]
pub type PFN_vkCmdDrawMeshTasksIndirect2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirect2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDrawMeshTasksIndirectCount2EXT`."]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html)"]
pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DrawIndirectCount2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCmdDispatchIndirect2KHR`."]
#[doc(alias = "vkCmdDispatchIndirect2KHR")]
#[doc = "\n\n[Khronos reference: `vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html)"]
pub type PFN_vkCmdDispatchIndirect2KHR = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_info: *const DispatchIndirect2InfoKHR<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkCreateAccelerationStructure2KHR`."]
#[doc(alias = "vkCreateAccelerationStructure2KHR")]
#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html)"]
pub type PFN_vkCreateAccelerationStructure2KHR = Option<
	unsafe extern "system" fn(
		device: DeviceHandle,
		p_create_info: *const AccelerationStructureCreateInfo2KHR<'_>,
		p_allocator: *const AllocationCallbacks,
		p_acceleration_structure: *mut AccelerationStructureKHR,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
		p_properties: *mut BaseOutStructure,
	) -> Result,
>;

#[doc = "Nullable function pointer for `vkCmdSetDispatchParametersARM`."]
#[doc(alias = "vkCmdSetDispatchParametersARM")]
#[doc = "\n\n[Khronos reference: `vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html)"]
pub type PFN_vkCmdSetDispatchParametersARM = Option<
	unsafe extern "system" fn(
		command_buffer: CommandBuffer,
		p_dispatch_parameters: *const DispatchParametersARM<'_>,
	) -> (),
>;

#[doc = "Nullable function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`."]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")]
#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html)"]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM = Option<
	unsafe extern "system" fn(
		physical_device: PhysicalDevice,
		queue_family_index: u32,
		p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
		p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM<'_>,
		p_format_count: *mut u32,
		p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM<'_>,
	) -> Result,
>;
