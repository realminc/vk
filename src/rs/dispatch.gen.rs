// @generated — do not edit by hand.
use super::{pfn::*, types::*};
/// Cached nullable PFNs for Global dispatch.
///
/// A resolved pointer does not establish enabled API, extension or feature state.
/// Keep the native objects and supplying library alive while using these pointers.
#[derive(Clone, Default)]
pub struct FnEntry {
	#[doc = "Cached function pointer for `vkCreateInstance`."]
	#[doc(alias = "vkCreateInstance")]
	#[doc = "\n\n[Khronos reference: `vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html)"]
	pub create_instance: PFN_vkCreateInstance,
	#[doc = "Cached function pointer for `vkGetInstanceProcAddr`."]
	#[doc(alias = "vkGetInstanceProcAddr")]
	#[doc = "\n\n[Khronos reference: `vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)"]
	pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
	#[doc = "Cached function pointer for `vkEnumerateInstanceVersion`."]
	#[doc(alias = "vkEnumerateInstanceVersion")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)"]
	pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
	#[doc = "Cached function pointer for `vkEnumerateInstanceLayerProperties`."]
	#[doc(alias = "vkEnumerateInstanceLayerProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)"]
	pub enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
	#[doc = "Cached function pointer for `vkEnumerateInstanceExtensionProperties`."]
	#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)"]
	pub enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
}

/// Cached nullable PFNs for Instance dispatch.
///
/// A resolved pointer does not establish enabled API, extension or feature state.
/// Keep the native objects and supplying library alive while using these pointers.
#[derive(Clone, Default)]
pub struct FnInstance {
	#[doc = "Cached function pointer for `vkDestroyInstance`."]
	#[doc(alias = "vkDestroyInstance")]
	#[doc = "\n\n[Khronos reference: `vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html)"]
	pub destroy_instance: PFN_vkDestroyInstance,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDevices`."]
	#[doc(alias = "vkEnumeratePhysicalDevices")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html)"]
	pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
	#[doc = "Cached function pointer for `vkGetDeviceProcAddr`."]
	#[doc(alias = "vkGetDeviceProcAddr")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html)"]
	pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html)"]
	pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html)"]
	pub get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceMemoryProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html)"]
	pub get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFeatures`."]
	#[doc(alias = "vkGetPhysicalDeviceFeatures")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html)"]
	pub get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFormatProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html)"]
	pub get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceImageFormatProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html)"]
	pub get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
	#[doc = "Cached function pointer for `vkCreateDevice`."]
	#[doc(alias = "vkCreateDevice")]
	#[doc = "\n\n[Khronos reference: `vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html)"]
	pub create_device: PFN_vkCreateDevice,
	#[doc = "Cached function pointer for `vkEnumerateDeviceLayerProperties`."]
	#[doc(alias = "vkEnumerateDeviceLayerProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html)"]
	pub enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
	#[doc = "Cached function pointer for `vkEnumerateDeviceExtensionProperties`."]
	#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html)"]
	pub enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html)"]
	pub get_physical_device_sparse_image_format_properties:
		PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
	#[cfg(feature = "platform-android")]
	#[doc = "Cached function pointer for `vkCreateAndroidSurfaceKHR`."]
	#[doc(alias = "vkCreateAndroidSurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html)"]
	pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
	#[cfg(feature = "platform-ohos")]
	#[doc = "Cached function pointer for `vkCreateSurfaceOHOS`."]
	#[doc(alias = "vkCreateSurfaceOHOS")]
	#[doc = "\n\n[Khronos reference: `vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html)"]
	pub create_surface_ohos: PFN_vkCreateSurfaceOHOS,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDisplayPropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html)"]
	pub get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)"]
	pub get_physical_device_display_plane_properties_khr:
		PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
	#[doc = "Cached function pointer for `vkGetDisplayPlaneSupportedDisplaysKHR`."]
	#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html)"]
	pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
	#[doc = "Cached function pointer for `vkGetDisplayModePropertiesKHR`."]
	#[doc(alias = "vkGetDisplayModePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html)"]
	pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
	#[doc = "Cached function pointer for `vkCreateDisplayModeKHR`."]
	#[doc(alias = "vkCreateDisplayModeKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html)"]
	pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
	#[doc = "Cached function pointer for `vkGetDisplayPlaneCapabilitiesKHR`."]
	#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html)"]
	pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
	#[doc = "Cached function pointer for `vkCreateDisplayPlaneSurfaceKHR`."]
	#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html)"]
	pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
	#[doc = "Cached function pointer for `vkDestroySurfaceKHR`."]
	#[doc(alias = "vkDestroySurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)"]
	pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceSupportKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html)"]
	pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)"]
	pub get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceFormatsKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html)"]
	pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfacePresentModesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html)"]
	pub get_physical_device_surface_present_modes_khr: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
	#[cfg(feature = "platform-vi")]
	#[doc = "Cached function pointer for `vkCreateViSurfaceNN`."]
	#[doc(alias = "vkCreateViSurfaceNN")]
	#[doc = "\n\n[Khronos reference: `vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html)"]
	pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
	#[cfg(feature = "platform-wayland")]
	#[doc = "Cached function pointer for `vkCreateWaylandSurfaceKHR`."]
	#[doc(alias = "vkCreateWaylandSurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html)"]
	pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
	#[cfg(feature = "platform-wayland")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceWaylandPresentationSupportKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)"]
	pub get_physical_device_wayland_presentation_support_khr:
		PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
	#[cfg(feature = "platform-ubm")]
	#[doc = "Cached function pointer for `vkCreateUbmSurfaceSEC`."]
	#[doc(alias = "vkCreateUbmSurfaceSEC")]
	#[doc = "\n\n[Khronos reference: `vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html)"]
	pub create_ubm_surface_sec: PFN_vkCreateUbmSurfaceSEC,
	#[cfg(feature = "platform-ubm")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceUbmPresentationSupportSEC`."]
	#[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html)"]
	pub get_physical_device_ubm_presentation_support_sec:
		PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkCreateWin32SurfaceKHR`."]
	#[doc(alias = "vkCreateWin32SurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html)"]
	pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceWin32PresentationSupportKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)"]
	pub get_physical_device_win32_presentation_support_khr:
		PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
	#[cfg(feature = "platform-xlib")]
	#[doc = "Cached function pointer for `vkCreateXlibSurfaceKHR`."]
	#[doc(alias = "vkCreateXlibSurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html)"]
	pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
	#[cfg(feature = "platform-xlib")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceXlibPresentationSupportKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)"]
	pub get_physical_device_xlib_presentation_support_khr:
		PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
	#[cfg(feature = "platform-xcb")]
	#[doc = "Cached function pointer for `vkCreateXcbSurfaceKHR`."]
	#[doc(alias = "vkCreateXcbSurfaceKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html)"]
	pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
	#[cfg(feature = "platform-xcb")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceXcbPresentationSupportKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)"]
	pub get_physical_device_xcb_presentation_support_khr:
		PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
	#[cfg(feature = "platform-directfb")]
	#[doc = "Cached function pointer for `vkCreateDirectFBSurfaceEXT`."]
	#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html)"]
	pub create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
	#[cfg(feature = "platform-directfb")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDirectFBPresentationSupportEXT`."]
	#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)"]
	pub get_physical_device_direct_fb_presentation_support_ext:
		PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkCreateImagePipeSurfaceFUCHSIA`."]
	#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html)"]
	pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
	#[cfg(feature = "platform-ggp")]
	#[doc = "Cached function pointer for `vkCreateStreamDescriptorSurfaceGGP`."]
	#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
	#[doc = "\n\n[Khronos reference: `vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html)"]
	pub create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
	#[cfg(feature = "platform-screen")]
	#[doc = "Cached function pointer for `vkCreateScreenSurfaceQNX`."]
	#[doc(alias = "vkCreateScreenSurfaceQNX")]
	#[doc = "\n\n[Khronos reference: `vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html)"]
	pub create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
	#[cfg(feature = "platform-screen")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceScreenPresentationSupportQNX`."]
	#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)"]
	pub get_physical_device_screen_presentation_support_qnx:
		PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
	#[doc = "Cached function pointer for `vkCreateDebugReportCallbackEXT`."]
	#[doc(alias = "vkCreateDebugReportCallbackEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html)"]
	pub create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
	#[doc = "Cached function pointer for `vkDestroyDebugReportCallbackEXT`."]
	#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)"]
	pub destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
	#[doc = "Cached function pointer for `vkDebugReportMessageEXT`."]
	#[doc(alias = "vkDebugReportMessageEXT")]
	#[doc = "\n\n[Khronos reference: `vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)"]
	pub debug_report_message_ext: PFN_vkDebugReportMessageEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)"]
	pub get_physical_device_external_image_format_properties_nv:
		PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFeatures2`."]
	#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html)"]
	pub get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFeatures2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html)"]
	pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html)"]
	pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html)"]
	pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFormatProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html)"]
	pub get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFormatProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html)"]
	pub get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceImageFormatProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html)"]
	pub get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceImageFormatProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html)"]
	pub get_physical_device_image_format_properties2_khr:
		PFN_vkGetPhysicalDeviceImageFormatProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html)"]
	pub get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)"]
	pub get_physical_device_queue_family_properties2_khr:
		PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceMemoryProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html)"]
	pub get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceMemoryProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html)"]
	pub get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties2`."]
	#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html)"]
	pub get_physical_device_sparse_image_format_properties2:
		PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSparseImageFormatProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)"]
	pub get_physical_device_sparse_image_format_properties2_khr:
		PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalBufferProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html)"]
	pub get_physical_device_external_buffer_properties:
		PFN_vkGetPhysicalDeviceExternalBufferProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalBufferPropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)"]
	pub get_physical_device_external_buffer_properties_khr:
		PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalSemaphoreProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html)"]
	pub get_physical_device_external_semaphore_properties:
		PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)"]
	pub get_physical_device_external_semaphore_properties_khr:
		PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalFenceProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html)"]
	pub get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalFencePropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)"]
	pub get_physical_device_external_fence_properties_khr:
		PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR,
	#[doc = "Cached function pointer for `vkReleaseDisplayEXT`."]
	#[doc(alias = "vkReleaseDisplayEXT")]
	#[doc = "\n\n[Khronos reference: `vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html)"]
	pub release_display_ext: PFN_vkReleaseDisplayEXT,
	#[cfg(feature = "platform-xlib-xrandr")]
	#[doc = "Cached function pointer for `vkAcquireXlibDisplayEXT`."]
	#[doc(alias = "vkAcquireXlibDisplayEXT")]
	#[doc = "\n\n[Khronos reference: `vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html)"]
	pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
	#[cfg(feature = "platform-xlib-xrandr")]
	#[doc = "Cached function pointer for `vkGetRandROutputDisplayEXT`."]
	#[doc(alias = "vkGetRandROutputDisplayEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html)"]
	pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkAcquireWinrtDisplayNV`."]
	#[doc(alias = "vkAcquireWinrtDisplayNV")]
	#[doc = "\n\n[Khronos reference: `vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html)"]
	pub acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetWinrtDisplayNV`."]
	#[doc(alias = "vkGetWinrtDisplayNV")]
	#[doc = "\n\n[Khronos reference: `vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html)"]
	pub get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceCapabilities2EXT`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)"]
	pub get_physical_device_surface_capabilities2_ext: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDeviceGroups`."]
	#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)"]
	pub enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDeviceGroupsKHR`."]
	#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html)"]
	pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroupsKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDevicePresentRectanglesKHR`."]
	#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html)"]
	pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
	#[cfg(feature = "platform-ios")]
	#[doc = "Cached function pointer for `vkCreateIOSSurfaceMVK`."]
	#[doc(alias = "vkCreateIOSSurfaceMVK")]
	#[doc = "\n\n[Khronos reference: `vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html)"]
	pub create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
	#[cfg(feature = "platform-macos")]
	#[doc = "Cached function pointer for `vkCreateMacOSSurfaceMVK`."]
	#[doc(alias = "vkCreateMacOSSurfaceMVK")]
	#[doc = "\n\n[Khronos reference: `vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html)"]
	pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
	#[cfg(feature = "platform-metal")]
	#[doc = "Cached function pointer for `vkCreateMetalSurfaceEXT`."]
	#[doc(alias = "vkCreateMetalSurfaceEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html)"]
	pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceMultisamplePropertiesEXT`."]
	#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)"]
	pub get_physical_device_multisample_properties_ext:
		PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceCapabilities2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)"]
	pub get_physical_device_surface_capabilities2_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfaceFormats2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html)"]
	pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDisplayProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html)"]
	pub get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDisplayPlaneProperties2KHR`."]
	#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)"]
	pub get_physical_device_display_plane_properties2_khr:
		PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
	#[doc = "Cached function pointer for `vkGetDisplayModeProperties2KHR`."]
	#[doc(alias = "vkGetDisplayModeProperties2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html)"]
	pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
	#[doc = "Cached function pointer for `vkGetDisplayPlaneCapabilities2KHR`."]
	#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html)"]
	pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html)"]
	pub get_physical_device_calibrateable_time_domains_khr:
		PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`."]
	#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)"]
	pub get_physical_device_calibrateable_time_domains_ext:
		PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
	#[doc = "Cached function pointer for `vkCreateDebugUtilsMessengerEXT`."]
	#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html)"]
	pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
	#[doc = "Cached function pointer for `vkDestroyDebugUtilsMessengerEXT`."]
	#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)"]
	pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
	#[doc = "Cached function pointer for `vkSubmitDebugUtilsMessageEXT`."]
	#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
	#[doc = "\n\n[Khronos reference: `vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)"]
	pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`."]
	#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)"]
	pub get_physical_device_cooperative_matrix_properties_nv:
		PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSurfacePresentModes2EXT`."]
	#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)"]
	pub get_physical_device_surface_present_modes2_ext:
		PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`."]
	#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)"]
	pub enumerate_physical_device_queue_family_performance_query_counters_khr:
		PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)"]
	pub get_physical_device_queue_family_performance_query_passes_khr:
		PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
	#[doc = "Cached function pointer for `vkCreateHeadlessSurfaceEXT`."]
	#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html)"]
	pub create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`."]
	#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)"]
	pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
		PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceToolProperties`."]
	#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html)"]
	pub get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceToolPropertiesEXT`."]
	#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html)"]
	pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolPropertiesEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceFragmentShadingRatesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)"]
	pub get_physical_device_fragment_shading_rates_khr:
		PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceVideoCapabilitiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)"]
	pub get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceVideoFormatPropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)"]
	pub get_physical_device_video_format_properties_khr:
		PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html)"]
	pub get_physical_device_video_encode_quality_level_properties_khr:
		PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
	#[doc = "Cached function pointer for `vkAcquireDrmDisplayEXT`."]
	#[doc(alias = "vkAcquireDrmDisplayEXT")]
	#[doc = "\n\n[Khronos reference: `vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html)"]
	pub acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
	#[doc = "Cached function pointer for `vkGetDrmDisplayEXT`."]
	#[doc(alias = "vkGetDrmDisplayEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html)"]
	pub get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceOpticalFlowImageFormatsNV`."]
	#[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)"]
	pub get_physical_device_optical_flow_image_formats_nv:
		PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`."]
	#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html)"]
	pub get_physical_device_cooperative_matrix_properties_khr:
		PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`."]
	#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixProperties2EXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html)"]
	pub get_physical_device_cooperative_matrix_properties2_ext:
		PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`."]
	#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html)"]
	pub get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
		PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceCooperativeVectorPropertiesNV`."]
	#[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html)"]
	pub get_physical_device_cooperative_vector_properties_nv:
		PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`."]
	#[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html)"]
	pub enumerate_physical_device_shader_instrumentation_metrics_arm:
		PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceExternalTensorPropertiesARM`."]
	#[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html)"]
	pub get_physical_device_external_tensor_properties_arm:
		PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html)"]
	pub get_physical_device_queue_family_data_graph_properties_arm:
		PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html)"]
	pub get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
		PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
	#[doc = "Cached function pointer for `vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`."]
	#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")]
	#[doc = "\n\n[Khronos reference: `vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html)"]
	pub enumerate_physical_device_queue_family_performance_counters_by_region_arm:
		PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceDescriptorSizeEXT`."]
	#[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html)"]
	pub get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html)"]
	pub get_physical_device_queue_family_data_graph_engine_operation_properties_arm:
		PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
	#[doc = "Cached function pointer for `vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`."]
	#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")]
	#[doc = "\n\n[Khronos reference: `vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html)"]
	pub get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm:
		PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM,
}

/// Cached nullable PFNs for Device dispatch.
///
/// A resolved pointer does not establish enabled API, extension or feature state.
/// Keep the native objects and supplying library alive while using these pointers.
#[derive(Clone, Default)]
pub struct FnDevice {
	#[doc = "Cached function pointer for `vkDestroyDevice`."]
	#[doc(alias = "vkDestroyDevice")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html)"]
	pub destroy_device: PFN_vkDestroyDevice,
	#[doc = "Cached function pointer for `vkGetDeviceQueue`."]
	#[doc(alias = "vkGetDeviceQueue")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html)"]
	pub get_device_queue: PFN_vkGetDeviceQueue,
	#[doc = "Cached function pointer for `vkQueueSubmit`."]
	#[doc(alias = "vkQueueSubmit")]
	#[doc = "\n\n[Khronos reference: `vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html)"]
	pub queue_submit: PFN_vkQueueSubmit,
	#[doc = "Cached function pointer for `vkQueueWaitIdle`."]
	#[doc(alias = "vkQueueWaitIdle")]
	#[doc = "\n\n[Khronos reference: `vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html)"]
	pub queue_wait_idle: PFN_vkQueueWaitIdle,
	#[doc = "Cached function pointer for `vkDeviceWaitIdle`."]
	#[doc(alias = "vkDeviceWaitIdle")]
	#[doc = "\n\n[Khronos reference: `vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html)"]
	pub device_wait_idle: PFN_vkDeviceWaitIdle,
	#[doc = "Cached function pointer for `vkAllocateMemory`."]
	#[doc(alias = "vkAllocateMemory")]
	#[doc = "\n\n[Khronos reference: `vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html)"]
	pub allocate_memory: PFN_vkAllocateMemory,
	#[doc = "Cached function pointer for `vkFreeMemory`."]
	#[doc(alias = "vkFreeMemory")]
	#[doc = "\n\n[Khronos reference: `vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html)"]
	pub free_memory: PFN_vkFreeMemory,
	#[doc = "Cached function pointer for `vkMapMemory`."]
	#[doc(alias = "vkMapMemory")]
	#[doc = "\n\n[Khronos reference: `vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html)"]
	pub map_memory: PFN_vkMapMemory,
	#[doc = "Cached function pointer for `vkUnmapMemory`."]
	#[doc(alias = "vkUnmapMemory")]
	#[doc = "\n\n[Khronos reference: `vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html)"]
	pub unmap_memory: PFN_vkUnmapMemory,
	#[doc = "Cached function pointer for `vkFlushMappedMemoryRanges`."]
	#[doc(alias = "vkFlushMappedMemoryRanges")]
	#[doc = "\n\n[Khronos reference: `vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html)"]
	pub flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
	#[doc = "Cached function pointer for `vkInvalidateMappedMemoryRanges`."]
	#[doc(alias = "vkInvalidateMappedMemoryRanges")]
	#[doc = "\n\n[Khronos reference: `vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html)"]
	pub invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
	#[doc = "Cached function pointer for `vkGetDeviceMemoryCommitment`."]
	#[doc(alias = "vkGetDeviceMemoryCommitment")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html)"]
	pub get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
	#[doc = "Cached function pointer for `vkGetBufferMemoryRequirements`."]
	#[doc(alias = "vkGetBufferMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html)"]
	pub get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
	#[doc = "Cached function pointer for `vkBindBufferMemory`."]
	#[doc(alias = "vkBindBufferMemory")]
	#[doc = "\n\n[Khronos reference: `vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html)"]
	pub bind_buffer_memory: PFN_vkBindBufferMemory,
	#[doc = "Cached function pointer for `vkGetImageMemoryRequirements`."]
	#[doc(alias = "vkGetImageMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html)"]
	pub get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
	#[doc = "Cached function pointer for `vkBindImageMemory`."]
	#[doc(alias = "vkBindImageMemory")]
	#[doc = "\n\n[Khronos reference: `vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html)"]
	pub bind_image_memory: PFN_vkBindImageMemory,
	#[doc = "Cached function pointer for `vkGetImageSparseMemoryRequirements`."]
	#[doc(alias = "vkGetImageSparseMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html)"]
	pub get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
	#[doc = "Cached function pointer for `vkQueueBindSparse`."]
	#[doc(alias = "vkQueueBindSparse")]
	#[doc = "\n\n[Khronos reference: `vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html)"]
	pub queue_bind_sparse: PFN_vkQueueBindSparse,
	#[doc = "Cached function pointer for `vkCreateFence`."]
	#[doc(alias = "vkCreateFence")]
	#[doc = "\n\n[Khronos reference: `vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html)"]
	pub create_fence: PFN_vkCreateFence,
	#[doc = "Cached function pointer for `vkDestroyFence`."]
	#[doc(alias = "vkDestroyFence")]
	#[doc = "\n\n[Khronos reference: `vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html)"]
	pub destroy_fence: PFN_vkDestroyFence,
	#[doc = "Cached function pointer for `vkResetFences`."]
	#[doc(alias = "vkResetFences")]
	#[doc = "\n\n[Khronos reference: `vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html)"]
	pub reset_fences: PFN_vkResetFences,
	#[doc = "Cached function pointer for `vkGetFenceStatus`."]
	#[doc(alias = "vkGetFenceStatus")]
	#[doc = "\n\n[Khronos reference: `vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html)"]
	pub get_fence_status: PFN_vkGetFenceStatus,
	#[doc = "Cached function pointer for `vkWaitForFences`."]
	#[doc(alias = "vkWaitForFences")]
	#[doc = "\n\n[Khronos reference: `vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html)"]
	pub wait_for_fences: PFN_vkWaitForFences,
	#[doc = "Cached function pointer for `vkCreateSemaphore`."]
	#[doc(alias = "vkCreateSemaphore")]
	#[doc = "\n\n[Khronos reference: `vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html)"]
	pub create_semaphore: PFN_vkCreateSemaphore,
	#[doc = "Cached function pointer for `vkDestroySemaphore`."]
	#[doc(alias = "vkDestroySemaphore")]
	#[doc = "\n\n[Khronos reference: `vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html)"]
	pub destroy_semaphore: PFN_vkDestroySemaphore,
	#[doc = "Cached function pointer for `vkCreateEvent`."]
	#[doc(alias = "vkCreateEvent")]
	#[doc = "\n\n[Khronos reference: `vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html)"]
	pub create_event: PFN_vkCreateEvent,
	#[doc = "Cached function pointer for `vkDestroyEvent`."]
	#[doc(alias = "vkDestroyEvent")]
	#[doc = "\n\n[Khronos reference: `vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html)"]
	pub destroy_event: PFN_vkDestroyEvent,
	#[doc = "Cached function pointer for `vkGetEventStatus`."]
	#[doc(alias = "vkGetEventStatus")]
	#[doc = "\n\n[Khronos reference: `vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html)"]
	pub get_event_status: PFN_vkGetEventStatus,
	#[doc = "Cached function pointer for `vkSetEvent`."]
	#[doc(alias = "vkSetEvent")]
	#[doc = "\n\n[Khronos reference: `vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html)"]
	pub set_event: PFN_vkSetEvent,
	#[doc = "Cached function pointer for `vkResetEvent`."]
	#[doc(alias = "vkResetEvent")]
	#[doc = "\n\n[Khronos reference: `vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html)"]
	pub reset_event: PFN_vkResetEvent,
	#[doc = "Cached function pointer for `vkCreateQueryPool`."]
	#[doc(alias = "vkCreateQueryPool")]
	#[doc = "\n\n[Khronos reference: `vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html)"]
	pub create_query_pool: PFN_vkCreateQueryPool,
	#[doc = "Cached function pointer for `vkDestroyQueryPool`."]
	#[doc(alias = "vkDestroyQueryPool")]
	#[doc = "\n\n[Khronos reference: `vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html)"]
	pub destroy_query_pool: PFN_vkDestroyQueryPool,
	#[doc = "Cached function pointer for `vkGetQueryPoolResults`."]
	#[doc(alias = "vkGetQueryPoolResults")]
	#[doc = "\n\n[Khronos reference: `vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html)"]
	pub get_query_pool_results: PFN_vkGetQueryPoolResults,
	#[doc = "Cached function pointer for `vkResetQueryPool`."]
	#[doc(alias = "vkResetQueryPool")]
	#[doc = "\n\n[Khronos reference: `vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)"]
	pub reset_query_pool: PFN_vkResetQueryPool,
	#[doc = "Cached function pointer for `vkResetQueryPoolEXT`."]
	#[doc(alias = "vkResetQueryPoolEXT")]
	#[doc = "\n\n[Khronos reference: `vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html)"]
	pub reset_query_pool_ext: PFN_vkResetQueryPoolEXT,
	#[doc = "Cached function pointer for `vkCreateBuffer`."]
	#[doc(alias = "vkCreateBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html)"]
	pub create_buffer: PFN_vkCreateBuffer,
	#[doc = "Cached function pointer for `vkDestroyBuffer`."]
	#[doc(alias = "vkDestroyBuffer")]
	#[doc = "\n\n[Khronos reference: `vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html)"]
	pub destroy_buffer: PFN_vkDestroyBuffer,
	#[doc = "Cached function pointer for `vkCreateBufferView`."]
	#[doc(alias = "vkCreateBufferView")]
	#[doc = "\n\n[Khronos reference: `vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html)"]
	pub create_buffer_view: PFN_vkCreateBufferView,
	#[doc = "Cached function pointer for `vkDestroyBufferView`."]
	#[doc(alias = "vkDestroyBufferView")]
	#[doc = "\n\n[Khronos reference: `vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html)"]
	pub destroy_buffer_view: PFN_vkDestroyBufferView,
	#[doc = "Cached function pointer for `vkCreateImage`."]
	#[doc(alias = "vkCreateImage")]
	#[doc = "\n\n[Khronos reference: `vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html)"]
	pub create_image: PFN_vkCreateImage,
	#[doc = "Cached function pointer for `vkDestroyImage`."]
	#[doc(alias = "vkDestroyImage")]
	#[doc = "\n\n[Khronos reference: `vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html)"]
	pub destroy_image: PFN_vkDestroyImage,
	#[doc = "Cached function pointer for `vkGetImageSubresourceLayout`."]
	#[doc(alias = "vkGetImageSubresourceLayout")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html)"]
	pub get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
	#[doc = "Cached function pointer for `vkCreateImageView`."]
	#[doc(alias = "vkCreateImageView")]
	#[doc = "\n\n[Khronos reference: `vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html)"]
	pub create_image_view: PFN_vkCreateImageView,
	#[doc = "Cached function pointer for `vkDestroyImageView`."]
	#[doc(alias = "vkDestroyImageView")]
	#[doc = "\n\n[Khronos reference: `vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html)"]
	pub destroy_image_view: PFN_vkDestroyImageView,
	#[doc = "Cached function pointer for `vkCreateShaderModule`."]
	#[doc(alias = "vkCreateShaderModule")]
	#[doc = "\n\n[Khronos reference: `vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html)"]
	pub create_shader_module: PFN_vkCreateShaderModule,
	#[doc = "Cached function pointer for `vkDestroyShaderModule`."]
	#[doc(alias = "vkDestroyShaderModule")]
	#[doc = "\n\n[Khronos reference: `vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)"]
	pub destroy_shader_module: PFN_vkDestroyShaderModule,
	#[doc = "Cached function pointer for `vkCreatePipelineCache`."]
	#[doc(alias = "vkCreatePipelineCache")]
	#[doc = "\n\n[Khronos reference: `vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html)"]
	pub create_pipeline_cache: PFN_vkCreatePipelineCache,
	#[doc = "Cached function pointer for `vkDestroyPipelineCache`."]
	#[doc(alias = "vkDestroyPipelineCache")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html)"]
	pub destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
	#[doc = "Cached function pointer for `vkGetPipelineCacheData`."]
	#[doc(alias = "vkGetPipelineCacheData")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html)"]
	pub get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
	#[doc = "Cached function pointer for `vkMergePipelineCaches`."]
	#[doc(alias = "vkMergePipelineCaches")]
	#[doc = "\n\n[Khronos reference: `vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html)"]
	pub merge_pipeline_caches: PFN_vkMergePipelineCaches,
	#[doc = "Cached function pointer for `vkCreatePipelineBinariesKHR`."]
	#[doc(alias = "vkCreatePipelineBinariesKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html)"]
	pub create_pipeline_binaries_khr: PFN_vkCreatePipelineBinariesKHR,
	#[doc = "Cached function pointer for `vkDestroyPipelineBinaryKHR`."]
	#[doc(alias = "vkDestroyPipelineBinaryKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html)"]
	pub destroy_pipeline_binary_khr: PFN_vkDestroyPipelineBinaryKHR,
	#[doc = "Cached function pointer for `vkGetPipelineKeyKHR`."]
	#[doc(alias = "vkGetPipelineKeyKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html)"]
	pub get_pipeline_key_khr: PFN_vkGetPipelineKeyKHR,
	#[doc = "Cached function pointer for `vkGetPipelineBinaryDataKHR`."]
	#[doc(alias = "vkGetPipelineBinaryDataKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html)"]
	pub get_pipeline_binary_data_khr: PFN_vkGetPipelineBinaryDataKHR,
	#[doc = "Cached function pointer for `vkReleaseCapturedPipelineDataKHR`."]
	#[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
	#[doc = "\n\n[Khronos reference: `vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html)"]
	pub release_captured_pipeline_data_khr: PFN_vkReleaseCapturedPipelineDataKHR,
	#[doc = "Cached function pointer for `vkCreateGraphicsPipelines`."]
	#[doc(alias = "vkCreateGraphicsPipelines")]
	#[doc = "\n\n[Khronos reference: `vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html)"]
	pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
	#[doc = "Cached function pointer for `vkCreateComputePipelines`."]
	#[doc(alias = "vkCreateComputePipelines")]
	#[doc = "\n\n[Khronos reference: `vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html)"]
	pub create_compute_pipelines: PFN_vkCreateComputePipelines,
	#[doc = "Cached function pointer for `vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`."]
	#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)"]
	pub get_device_subpass_shading_max_workgroup_size_huawei:
		PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
	#[doc = "Cached function pointer for `vkDestroyPipeline`."]
	#[doc(alias = "vkDestroyPipeline")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html)"]
	pub destroy_pipeline: PFN_vkDestroyPipeline,
	#[doc = "Cached function pointer for `vkCreatePipelineLayout`."]
	#[doc(alias = "vkCreatePipelineLayout")]
	#[doc = "\n\n[Khronos reference: `vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html)"]
	pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
	#[doc = "Cached function pointer for `vkDestroyPipelineLayout`."]
	#[doc(alias = "vkDestroyPipelineLayout")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html)"]
	pub destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
	#[doc = "Cached function pointer for `vkCreateSampler`."]
	#[doc(alias = "vkCreateSampler")]
	#[doc = "\n\n[Khronos reference: `vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html)"]
	pub create_sampler: PFN_vkCreateSampler,
	#[doc = "Cached function pointer for `vkDestroySampler`."]
	#[doc(alias = "vkDestroySampler")]
	#[doc = "\n\n[Khronos reference: `vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html)"]
	pub destroy_sampler: PFN_vkDestroySampler,
	#[doc = "Cached function pointer for `vkCreateDescriptorSetLayout`."]
	#[doc(alias = "vkCreateDescriptorSetLayout")]
	#[doc = "\n\n[Khronos reference: `vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html)"]
	pub create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
	#[doc = "Cached function pointer for `vkDestroyDescriptorSetLayout`."]
	#[doc(alias = "vkDestroyDescriptorSetLayout")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html)"]
	pub destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
	#[doc = "Cached function pointer for `vkCreateDescriptorPool`."]
	#[doc(alias = "vkCreateDescriptorPool")]
	#[doc = "\n\n[Khronos reference: `vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html)"]
	pub create_descriptor_pool: PFN_vkCreateDescriptorPool,
	#[doc = "Cached function pointer for `vkDestroyDescriptorPool`."]
	#[doc(alias = "vkDestroyDescriptorPool")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html)"]
	pub destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
	#[doc = "Cached function pointer for `vkResetDescriptorPool`."]
	#[doc(alias = "vkResetDescriptorPool")]
	#[doc = "\n\n[Khronos reference: `vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html)"]
	pub reset_descriptor_pool: PFN_vkResetDescriptorPool,
	#[doc = "Cached function pointer for `vkAllocateDescriptorSets`."]
	#[doc(alias = "vkAllocateDescriptorSets")]
	#[doc = "\n\n[Khronos reference: `vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html)"]
	pub allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
	#[doc = "Cached function pointer for `vkFreeDescriptorSets`."]
	#[doc(alias = "vkFreeDescriptorSets")]
	#[doc = "\n\n[Khronos reference: `vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html)"]
	pub free_descriptor_sets: PFN_vkFreeDescriptorSets,
	#[doc = "Cached function pointer for `vkUpdateDescriptorSets`."]
	#[doc(alias = "vkUpdateDescriptorSets")]
	#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html)"]
	pub update_descriptor_sets: PFN_vkUpdateDescriptorSets,
	#[doc = "Cached function pointer for `vkCreateFramebuffer`."]
	#[doc(alias = "vkCreateFramebuffer")]
	#[doc = "\n\n[Khronos reference: `vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html)"]
	pub create_framebuffer: PFN_vkCreateFramebuffer,
	#[doc = "Cached function pointer for `vkDestroyFramebuffer`."]
	#[doc(alias = "vkDestroyFramebuffer")]
	#[doc = "\n\n[Khronos reference: `vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)"]
	pub destroy_framebuffer: PFN_vkDestroyFramebuffer,
	#[doc = "Cached function pointer for `vkCreateRenderPass`."]
	#[doc(alias = "vkCreateRenderPass")]
	#[doc = "\n\n[Khronos reference: `vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html)"]
	pub create_render_pass: PFN_vkCreateRenderPass,
	#[doc = "Cached function pointer for `vkDestroyRenderPass`."]
	#[doc(alias = "vkDestroyRenderPass")]
	#[doc = "\n\n[Khronos reference: `vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html)"]
	pub destroy_render_pass: PFN_vkDestroyRenderPass,
	#[doc = "Cached function pointer for `vkGetRenderAreaGranularity`."]
	#[doc(alias = "vkGetRenderAreaGranularity")]
	#[doc = "\n\n[Khronos reference: `vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html)"]
	pub get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
	#[doc = "Cached function pointer for `vkGetRenderingAreaGranularity`."]
	#[doc(alias = "vkGetRenderingAreaGranularity")]
	#[doc = "\n\n[Khronos reference: `vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html)"]
	pub get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
	#[doc = "Cached function pointer for `vkGetRenderingAreaGranularityKHR`."]
	#[doc(alias = "vkGetRenderingAreaGranularityKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html)"]
	pub get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularityKHR,
	#[doc = "Cached function pointer for `vkCreateCommandPool`."]
	#[doc(alias = "vkCreateCommandPool")]
	#[doc = "\n\n[Khronos reference: `vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html)"]
	pub create_command_pool: PFN_vkCreateCommandPool,
	#[doc = "Cached function pointer for `vkDestroyCommandPool`."]
	#[doc(alias = "vkDestroyCommandPool")]
	#[doc = "\n\n[Khronos reference: `vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html)"]
	pub destroy_command_pool: PFN_vkDestroyCommandPool,
	#[doc = "Cached function pointer for `vkResetCommandPool`."]
	#[doc(alias = "vkResetCommandPool")]
	#[doc = "\n\n[Khronos reference: `vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html)"]
	pub reset_command_pool: PFN_vkResetCommandPool,
	#[doc = "Cached function pointer for `vkAllocateCommandBuffers`."]
	#[doc(alias = "vkAllocateCommandBuffers")]
	#[doc = "\n\n[Khronos reference: `vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html)"]
	pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
	#[doc = "Cached function pointer for `vkFreeCommandBuffers`."]
	#[doc(alias = "vkFreeCommandBuffers")]
	#[doc = "\n\n[Khronos reference: `vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html)"]
	pub free_command_buffers: PFN_vkFreeCommandBuffers,
	#[doc = "Cached function pointer for `vkBeginCommandBuffer`."]
	#[doc(alias = "vkBeginCommandBuffer")]
	#[doc = "\n\n[Khronos reference: `vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html)"]
	pub begin_command_buffer: PFN_vkBeginCommandBuffer,
	#[doc = "Cached function pointer for `vkEndCommandBuffer`."]
	#[doc(alias = "vkEndCommandBuffer")]
	#[doc = "\n\n[Khronos reference: `vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html)"]
	pub end_command_buffer: PFN_vkEndCommandBuffer,
	#[doc = "Cached function pointer for `vkResetCommandBuffer`."]
	#[doc(alias = "vkResetCommandBuffer")]
	#[doc = "\n\n[Khronos reference: `vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html)"]
	pub reset_command_buffer: PFN_vkResetCommandBuffer,
	#[doc = "Cached function pointer for `vkCmdBindPipeline`."]
	#[doc(alias = "vkCmdBindPipeline")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html)"]
	pub cmd_bind_pipeline: PFN_vkCmdBindPipeline,
	#[doc = "Cached function pointer for `vkCmdSetPrimitiveRestartIndexEXT`."]
	#[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html)"]
	pub cmd_set_primitive_restart_index_ext: PFN_vkCmdSetPrimitiveRestartIndexEXT,
	#[doc = "Cached function pointer for `vkCmdSetAttachmentFeedbackLoopEnableEXT`."]
	#[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html)"]
	pub cmd_set_attachment_feedback_loop_enable_ext: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetViewport`."]
	#[doc(alias = "vkCmdSetViewport")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html)"]
	pub cmd_set_viewport: PFN_vkCmdSetViewport,
	#[doc = "Cached function pointer for `vkCmdSetScissor`."]
	#[doc(alias = "vkCmdSetScissor")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html)"]
	pub cmd_set_scissor: PFN_vkCmdSetScissor,
	#[doc = "Cached function pointer for `vkCmdSetLineWidth`."]
	#[doc(alias = "vkCmdSetLineWidth")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html)"]
	pub cmd_set_line_width: PFN_vkCmdSetLineWidth,
	#[doc = "Cached function pointer for `vkCmdSetDepthBias`."]
	#[doc(alias = "vkCmdSetDepthBias")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html)"]
	pub cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
	#[doc = "Cached function pointer for `vkCmdSetBlendConstants`."]
	#[doc(alias = "vkCmdSetBlendConstants")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html)"]
	pub cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
	#[doc = "Cached function pointer for `vkCmdSetDepthBounds`."]
	#[doc(alias = "vkCmdSetDepthBounds")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html)"]
	pub cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
	#[doc = "Cached function pointer for `vkCmdSetStencilCompareMask`."]
	#[doc(alias = "vkCmdSetStencilCompareMask")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html)"]
	pub cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
	#[doc = "Cached function pointer for `vkCmdSetStencilWriteMask`."]
	#[doc(alias = "vkCmdSetStencilWriteMask")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html)"]
	pub cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
	#[doc = "Cached function pointer for `vkCmdSetStencilReference`."]
	#[doc(alias = "vkCmdSetStencilReference")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html)"]
	pub cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorSets`."]
	#[doc(alias = "vkCmdBindDescriptorSets")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html)"]
	pub cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
	#[doc = "Cached function pointer for `vkCmdBindIndexBuffer`."]
	#[doc(alias = "vkCmdBindIndexBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html)"]
	pub cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
	#[doc = "Cached function pointer for `vkCmdBindVertexBuffers`."]
	#[doc(alias = "vkCmdBindVertexBuffers")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html)"]
	pub cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
	#[doc = "Cached function pointer for `vkCmdDraw`."]
	#[doc(alias = "vkCmdDraw")]
	#[doc = "\n\n[Khronos reference: `vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html)"]
	pub cmd_draw: PFN_vkCmdDraw,
	#[doc = "Cached function pointer for `vkCmdDrawIndexed`."]
	#[doc(alias = "vkCmdDrawIndexed")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html)"]
	pub cmd_draw_indexed: PFN_vkCmdDrawIndexed,
	#[doc = "Cached function pointer for `vkCmdDrawMultiEXT`."]
	#[doc(alias = "vkCmdDrawMultiEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html)"]
	pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
	#[doc = "Cached function pointer for `vkCmdDrawMultiIndexedEXT`."]
	#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html)"]
	pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
	#[doc = "Cached function pointer for `vkCmdDrawIndirect`."]
	#[doc(alias = "vkCmdDrawIndirect")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html)"]
	pub cmd_draw_indirect: PFN_vkCmdDrawIndirect,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirect`."]
	#[doc(alias = "vkCmdDrawIndexedIndirect")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html)"]
	pub cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
	#[doc = "Cached function pointer for `vkCmdDispatch`."]
	#[doc(alias = "vkCmdDispatch")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html)"]
	pub cmd_dispatch: PFN_vkCmdDispatch,
	#[doc = "Cached function pointer for `vkCmdDispatchIndirect`."]
	#[doc(alias = "vkCmdDispatchIndirect")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html)"]
	pub cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
	#[doc = "Cached function pointer for `vkCmdSubpassShadingHUAWEI`."]
	#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
	#[doc = "\n\n[Khronos reference: `vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html)"]
	pub cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
	#[doc = "Cached function pointer for `vkCmdDrawClusterHUAWEI`."]
	#[doc(alias = "vkCmdDrawClusterHUAWEI")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html)"]
	pub cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
	#[doc = "Cached function pointer for `vkCmdDrawClusterIndirectHUAWEI`."]
	#[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html)"]
	pub cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
	#[doc = "Cached function pointer for `vkCmdUpdatePipelineIndirectBufferNV`."]
	#[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html)"]
	pub cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
	#[doc = "Cached function pointer for `vkCmdCopyBuffer`."]
	#[doc(alias = "vkCmdCopyBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html)"]
	pub cmd_copy_buffer: PFN_vkCmdCopyBuffer,
	#[doc = "Cached function pointer for `vkCmdCopyImage`."]
	#[doc(alias = "vkCmdCopyImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html)"]
	pub cmd_copy_image: PFN_vkCmdCopyImage,
	#[doc = "Cached function pointer for `vkCmdBlitImage`."]
	#[doc(alias = "vkCmdBlitImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html)"]
	pub cmd_blit_image: PFN_vkCmdBlitImage,
	#[doc = "Cached function pointer for `vkCmdCopyBufferToImage`."]
	#[doc(alias = "vkCmdCopyBufferToImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html)"]
	pub cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
	#[doc = "Cached function pointer for `vkCmdCopyImageToBuffer`."]
	#[doc(alias = "vkCmdCopyImageToBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html)"]
	pub cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryIndirectNV`."]
	#[doc(alias = "vkCmdCopyMemoryIndirectNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html)"]
	pub cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryIndirectKHR`."]
	#[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html)"]
	pub cmd_copy_memory_indirect_khr: PFN_vkCmdCopyMemoryIndirectKHR,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryToImageIndirectNV`."]
	#[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html)"]
	pub cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryToImageIndirectKHR`."]
	#[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html)"]
	pub cmd_copy_memory_to_image_indirect_khr: PFN_vkCmdCopyMemoryToImageIndirectKHR,
	#[doc = "Cached function pointer for `vkCmdUpdateBuffer`."]
	#[doc(alias = "vkCmdUpdateBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html)"]
	pub cmd_update_buffer: PFN_vkCmdUpdateBuffer,
	#[doc = "Cached function pointer for `vkCmdFillBuffer`."]
	#[doc(alias = "vkCmdFillBuffer")]
	#[doc = "\n\n[Khronos reference: `vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html)"]
	pub cmd_fill_buffer: PFN_vkCmdFillBuffer,
	#[doc = "Cached function pointer for `vkCmdClearColorImage`."]
	#[doc(alias = "vkCmdClearColorImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html)"]
	pub cmd_clear_color_image: PFN_vkCmdClearColorImage,
	#[doc = "Cached function pointer for `vkCmdClearDepthStencilImage`."]
	#[doc(alias = "vkCmdClearDepthStencilImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html)"]
	pub cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
	#[doc = "Cached function pointer for `vkCmdClearAttachments`."]
	#[doc(alias = "vkCmdClearAttachments")]
	#[doc = "\n\n[Khronos reference: `vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html)"]
	pub cmd_clear_attachments: PFN_vkCmdClearAttachments,
	#[doc = "Cached function pointer for `vkCmdResolveImage`."]
	#[doc(alias = "vkCmdResolveImage")]
	#[doc = "\n\n[Khronos reference: `vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html)"]
	pub cmd_resolve_image: PFN_vkCmdResolveImage,
	#[doc = "Cached function pointer for `vkCmdSetEvent`."]
	#[doc(alias = "vkCmdSetEvent")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html)"]
	pub cmd_set_event: PFN_vkCmdSetEvent,
	#[doc = "Cached function pointer for `vkCmdResetEvent`."]
	#[doc(alias = "vkCmdResetEvent")]
	#[doc = "\n\n[Khronos reference: `vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html)"]
	pub cmd_reset_event: PFN_vkCmdResetEvent,
	#[doc = "Cached function pointer for `vkCmdWaitEvents`."]
	#[doc(alias = "vkCmdWaitEvents")]
	#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html)"]
	pub cmd_wait_events: PFN_vkCmdWaitEvents,
	#[doc = "Cached function pointer for `vkCmdPipelineBarrier`."]
	#[doc(alias = "vkCmdPipelineBarrier")]
	#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html)"]
	pub cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
	#[doc = "Cached function pointer for `vkCmdBeginQuery`."]
	#[doc(alias = "vkCmdBeginQuery")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html)"]
	pub cmd_begin_query: PFN_vkCmdBeginQuery,
	#[doc = "Cached function pointer for `vkCmdEndQuery`."]
	#[doc(alias = "vkCmdEndQuery")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html)"]
	pub cmd_end_query: PFN_vkCmdEndQuery,
	#[doc = "Cached function pointer for `vkCmdBeginConditionalRenderingEXT`."]
	#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html)"]
	pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
	#[doc = "Cached function pointer for `vkCmdEndConditionalRenderingEXT`."]
	#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html)"]
	pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
	#[doc = "Cached function pointer for `vkCmdBeginCustomResolveEXT`."]
	#[doc(alias = "vkCmdBeginCustomResolveEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html)"]
	pub cmd_begin_custom_resolve_ext: PFN_vkCmdBeginCustomResolveEXT,
	#[doc = "Cached function pointer for `vkCmdResetQueryPool`."]
	#[doc(alias = "vkCmdResetQueryPool")]
	#[doc = "\n\n[Khronos reference: `vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html)"]
	pub cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
	#[doc = "Cached function pointer for `vkCmdWriteTimestamp`."]
	#[doc(alias = "vkCmdWriteTimestamp")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html)"]
	pub cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
	#[doc = "Cached function pointer for `vkCmdCopyQueryPoolResults`."]
	#[doc(alias = "vkCmdCopyQueryPoolResults")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html)"]
	pub cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
	#[doc = "Cached function pointer for `vkCmdPushConstants`."]
	#[doc(alias = "vkCmdPushConstants")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html)"]
	pub cmd_push_constants: PFN_vkCmdPushConstants,
	#[doc = "Cached function pointer for `vkCmdBeginRenderPass`."]
	#[doc(alias = "vkCmdBeginRenderPass")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html)"]
	pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
	#[doc = "Cached function pointer for `vkCmdNextSubpass`."]
	#[doc(alias = "vkCmdNextSubpass")]
	#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html)"]
	pub cmd_next_subpass: PFN_vkCmdNextSubpass,
	#[doc = "Cached function pointer for `vkCmdEndRenderPass`."]
	#[doc(alias = "vkCmdEndRenderPass")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html)"]
	pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
	#[doc = "Cached function pointer for `vkCmdExecuteCommands`."]
	#[doc(alias = "vkCmdExecuteCommands")]
	#[doc = "\n\n[Khronos reference: `vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html)"]
	pub cmd_execute_commands: PFN_vkCmdExecuteCommands,
	#[doc = "Cached function pointer for `vkCreateSharedSwapchainsKHR`."]
	#[doc(alias = "vkCreateSharedSwapchainsKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html)"]
	pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
	#[doc = "Cached function pointer for `vkCreateSwapchainKHR`."]
	#[doc(alias = "vkCreateSwapchainKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html)"]
	pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
	#[doc = "Cached function pointer for `vkDestroySwapchainKHR`."]
	#[doc(alias = "vkDestroySwapchainKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html)"]
	pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
	#[doc = "Cached function pointer for `vkGetSwapchainImagesKHR`."]
	#[doc(alias = "vkGetSwapchainImagesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html)"]
	pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
	#[doc = "Cached function pointer for `vkAcquireNextImageKHR`."]
	#[doc(alias = "vkAcquireNextImageKHR")]
	#[doc = "\n\n[Khronos reference: `vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html)"]
	pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
	#[doc = "Cached function pointer for `vkQueuePresentKHR`."]
	#[doc(alias = "vkQueuePresentKHR")]
	#[doc = "\n\n[Khronos reference: `vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)"]
	pub queue_present_khr: PFN_vkQueuePresentKHR,
	#[doc = "Cached function pointer for `vkDebugMarkerSetObjectNameEXT`."]
	#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
	#[doc = "\n\n[Khronos reference: `vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html)"]
	pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
	#[doc = "Cached function pointer for `vkDebugMarkerSetObjectTagEXT`."]
	#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
	#[doc = "\n\n[Khronos reference: `vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html)"]
	pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
	#[doc = "Cached function pointer for `vkCmdDebugMarkerBeginEXT`."]
	#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html)"]
	pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
	#[doc = "Cached function pointer for `vkCmdDebugMarkerEndEXT`."]
	#[doc(alias = "vkCmdDebugMarkerEndEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html)"]
	pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
	#[doc = "Cached function pointer for `vkCmdDebugMarkerInsertEXT`."]
	#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html)"]
	pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetMemoryWin32HandleNV`."]
	#[doc(alias = "vkGetMemoryWin32HandleNV")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html)"]
	pub get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
	#[doc = "Cached function pointer for `vkCmdExecuteGeneratedCommandsNV`."]
	#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html)"]
	pub cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
	#[doc = "Cached function pointer for `vkCmdPreprocessGeneratedCommandsNV`."]
	#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html)"]
	pub cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
	#[doc = "Cached function pointer for `vkCmdBindPipelineShaderGroupNV`."]
	#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html)"]
	pub cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
	#[doc = "Cached function pointer for `vkGetGeneratedCommandsMemoryRequirementsNV`."]
	#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html)"]
	pub get_generated_commands_memory_requirements_nv: PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
	#[doc = "Cached function pointer for `vkCreateIndirectCommandsLayoutNV`."]
	#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html)"]
	pub create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
	#[doc = "Cached function pointer for `vkDestroyIndirectCommandsLayoutNV`."]
	#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html)"]
	pub destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
	#[doc = "Cached function pointer for `vkCmdExecuteGeneratedCommandsEXT`."]
	#[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html)"]
	pub cmd_execute_generated_commands_ext: PFN_vkCmdExecuteGeneratedCommandsEXT,
	#[doc = "Cached function pointer for `vkCmdPreprocessGeneratedCommandsEXT`."]
	#[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html)"]
	pub cmd_preprocess_generated_commands_ext: PFN_vkCmdPreprocessGeneratedCommandsEXT,
	#[doc = "Cached function pointer for `vkGetGeneratedCommandsMemoryRequirementsEXT`."]
	#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html)"]
	pub get_generated_commands_memory_requirements_ext:
		PFN_vkGetGeneratedCommandsMemoryRequirementsEXT,
	#[doc = "Cached function pointer for `vkCreateIndirectCommandsLayoutEXT`."]
	#[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html)"]
	pub create_indirect_commands_layout_ext: PFN_vkCreateIndirectCommandsLayoutEXT,
	#[doc = "Cached function pointer for `vkDestroyIndirectCommandsLayoutEXT`."]
	#[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html)"]
	pub destroy_indirect_commands_layout_ext: PFN_vkDestroyIndirectCommandsLayoutEXT,
	#[doc = "Cached function pointer for `vkCreateIndirectExecutionSetEXT`."]
	#[doc(alias = "vkCreateIndirectExecutionSetEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html)"]
	pub create_indirect_execution_set_ext: PFN_vkCreateIndirectExecutionSetEXT,
	#[doc = "Cached function pointer for `vkDestroyIndirectExecutionSetEXT`."]
	#[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html)"]
	pub destroy_indirect_execution_set_ext: PFN_vkDestroyIndirectExecutionSetEXT,
	#[doc = "Cached function pointer for `vkUpdateIndirectExecutionSetPipelineEXT`."]
	#[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
	#[doc = "\n\n[Khronos reference: `vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html)"]
	pub update_indirect_execution_set_pipeline_ext: PFN_vkUpdateIndirectExecutionSetPipelineEXT,
	#[doc = "Cached function pointer for `vkUpdateIndirectExecutionSetShaderEXT`."]
	#[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
	#[doc = "\n\n[Khronos reference: `vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html)"]
	pub update_indirect_execution_set_shader_ext: PFN_vkUpdateIndirectExecutionSetShaderEXT,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSet`."]
	#[doc(alias = "vkCmdPushDescriptorSet")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html)"]
	pub cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSetKHR`."]
	#[doc(alias = "vkCmdPushDescriptorSetKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html)"]
	pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
	#[doc = "Cached function pointer for `vkTrimCommandPool`."]
	#[doc(alias = "vkTrimCommandPool")]
	#[doc = "\n\n[Khronos reference: `vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)"]
	pub trim_command_pool: PFN_vkTrimCommandPool,
	#[doc = "Cached function pointer for `vkTrimCommandPoolKHR`."]
	#[doc(alias = "vkTrimCommandPoolKHR")]
	#[doc = "\n\n[Khronos reference: `vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html)"]
	pub trim_command_pool_khr: PFN_vkTrimCommandPoolKHR,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetMemoryWin32HandleKHR`."]
	#[doc(alias = "vkGetMemoryWin32HandleKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html)"]
	pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetMemoryWin32HandlePropertiesKHR`."]
	#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html)"]
	pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
	#[doc = "Cached function pointer for `vkGetMemoryFdKHR`."]
	#[doc(alias = "vkGetMemoryFdKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html)"]
	pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
	#[doc = "Cached function pointer for `vkGetMemoryFdPropertiesKHR`."]
	#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html)"]
	pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkGetMemoryZirconHandleFUCHSIA`."]
	#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html)"]
	pub get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkGetMemoryZirconHandlePropertiesFUCHSIA`."]
	#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)"]
	pub get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
	#[doc = "Cached function pointer for `vkGetMemoryRemoteAddressNV`."]
	#[doc(alias = "vkGetMemoryRemoteAddressNV")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html)"]
	pub get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetSemaphoreWin32HandleKHR`."]
	#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html)"]
	pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkImportSemaphoreWin32HandleKHR`."]
	#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
	#[doc = "\n\n[Khronos reference: `vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html)"]
	pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
	#[doc = "Cached function pointer for `vkGetSemaphoreFdKHR`."]
	#[doc(alias = "vkGetSemaphoreFdKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html)"]
	pub get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
	#[doc = "Cached function pointer for `vkImportSemaphoreFdKHR`."]
	#[doc(alias = "vkImportSemaphoreFdKHR")]
	#[doc = "\n\n[Khronos reference: `vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html)"]
	pub import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkGetSemaphoreZirconHandleFUCHSIA`."]
	#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html)"]
	pub get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkImportSemaphoreZirconHandleFUCHSIA`."]
	#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html)"]
	pub import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetFenceWin32HandleKHR`."]
	#[doc(alias = "vkGetFenceWin32HandleKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html)"]
	pub get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkImportFenceWin32HandleKHR`."]
	#[doc(alias = "vkImportFenceWin32HandleKHR")]
	#[doc = "\n\n[Khronos reference: `vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html)"]
	pub import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
	#[doc = "Cached function pointer for `vkGetFenceFdKHR`."]
	#[doc(alias = "vkGetFenceFdKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html)"]
	pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
	#[doc = "Cached function pointer for `vkImportFenceFdKHR`."]
	#[doc(alias = "vkImportFenceFdKHR")]
	#[doc = "\n\n[Khronos reference: `vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html)"]
	pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
	#[doc = "Cached function pointer for `vkDisplayPowerControlEXT`."]
	#[doc(alias = "vkDisplayPowerControlEXT")]
	#[doc = "\n\n[Khronos reference: `vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html)"]
	pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
	#[doc = "Cached function pointer for `vkRegisterDeviceEventEXT`."]
	#[doc(alias = "vkRegisterDeviceEventEXT")]
	#[doc = "\n\n[Khronos reference: `vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html)"]
	pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
	#[doc = "Cached function pointer for `vkRegisterDisplayEventEXT`."]
	#[doc(alias = "vkRegisterDisplayEventEXT")]
	#[doc = "\n\n[Khronos reference: `vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html)"]
	pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
	#[doc = "Cached function pointer for `vkGetSwapchainCounterEXT`."]
	#[doc(alias = "vkGetSwapchainCounterEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html)"]
	pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
	#[doc = "Cached function pointer for `vkGetDeviceGroupPeerMemoryFeatures`."]
	#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html)"]
	pub get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
	#[doc = "Cached function pointer for `vkGetDeviceGroupPeerMemoryFeaturesKHR`."]
	#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)"]
	pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR,
	#[doc = "Cached function pointer for `vkBindBufferMemory2`."]
	#[doc(alias = "vkBindBufferMemory2")]
	#[doc = "\n\n[Khronos reference: `vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html)"]
	pub bind_buffer_memory2: PFN_vkBindBufferMemory2,
	#[doc = "Cached function pointer for `vkBindBufferMemory2KHR`."]
	#[doc(alias = "vkBindBufferMemory2KHR")]
	#[doc = "\n\n[Khronos reference: `vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html)"]
	pub bind_buffer_memory2_khr: PFN_vkBindBufferMemory2KHR,
	#[doc = "Cached function pointer for `vkBindImageMemory2`."]
	#[doc(alias = "vkBindImageMemory2")]
	#[doc = "\n\n[Khronos reference: `vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html)"]
	pub bind_image_memory2: PFN_vkBindImageMemory2,
	#[doc = "Cached function pointer for `vkBindImageMemory2KHR`."]
	#[doc(alias = "vkBindImageMemory2KHR")]
	#[doc = "\n\n[Khronos reference: `vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html)"]
	pub bind_image_memory2_khr: PFN_vkBindImageMemory2KHR,
	#[doc = "Cached function pointer for `vkCmdSetDeviceMask`."]
	#[doc(alias = "vkCmdSetDeviceMask")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html)"]
	pub cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
	#[doc = "Cached function pointer for `vkCmdSetDeviceMaskKHR`."]
	#[doc(alias = "vkCmdSetDeviceMaskKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html)"]
	pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMaskKHR,
	#[doc = "Cached function pointer for `vkGetDeviceGroupPresentCapabilitiesKHR`."]
	#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html)"]
	pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
	#[doc = "Cached function pointer for `vkGetDeviceGroupSurfacePresentModesKHR`."]
	#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html)"]
	pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
	#[doc = "Cached function pointer for `vkAcquireNextImage2KHR`."]
	#[doc(alias = "vkAcquireNextImage2KHR")]
	#[doc = "\n\n[Khronos reference: `vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html)"]
	pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
	#[doc = "Cached function pointer for `vkCmdDispatchBase`."]
	#[doc(alias = "vkCmdDispatchBase")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html)"]
	pub cmd_dispatch_base: PFN_vkCmdDispatchBase,
	#[doc = "Cached function pointer for `vkCmdDispatchBaseKHR`."]
	#[doc(alias = "vkCmdDispatchBaseKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html)"]
	pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBaseKHR,
	#[doc = "Cached function pointer for `vkCreateDescriptorUpdateTemplate`."]
	#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
	#[doc = "\n\n[Khronos reference: `vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html)"]
	pub create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
	#[doc = "Cached function pointer for `vkCreateDescriptorUpdateTemplateKHR`."]
	#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html)"]
	pub create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplateKHR,
	#[doc = "Cached function pointer for `vkDestroyDescriptorUpdateTemplate`."]
	#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)"]
	pub destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
	#[doc = "Cached function pointer for `vkDestroyDescriptorUpdateTemplateKHR`."]
	#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html)"]
	pub destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplateKHR,
	#[doc = "Cached function pointer for `vkUpdateDescriptorSetWithTemplate`."]
	#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
	#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)"]
	pub update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
	#[doc = "Cached function pointer for `vkUpdateDescriptorSetWithTemplateKHR`."]
	#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
	#[doc = "\n\n[Khronos reference: `vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html)"]
	pub update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplateKHR,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSetWithTemplate`."]
	#[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html)"]
	pub cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSetWithTemplateKHR`."]
	#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html)"]
	pub cmd_push_descriptor_set_with_template_khr: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
	#[doc = "Cached function pointer for `vkSetHdrMetadataEXT`."]
	#[doc(alias = "vkSetHdrMetadataEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html)"]
	pub set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
	#[doc = "Cached function pointer for `vkGetSwapchainStatusKHR`."]
	#[doc(alias = "vkGetSwapchainStatusKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html)"]
	pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
	#[doc = "Cached function pointer for `vkGetRefreshCycleDurationGOOGLE`."]
	#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
	#[doc = "\n\n[Khronos reference: `vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html)"]
	pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
	#[doc = "Cached function pointer for `vkGetPastPresentationTimingGOOGLE`."]
	#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
	#[doc = "\n\n[Khronos reference: `vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)"]
	pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
	#[doc = "Cached function pointer for `vkCmdSetViewportWScalingNV`."]
	#[doc(alias = "vkCmdSetViewportWScalingNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html)"]
	pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
	#[doc = "Cached function pointer for `vkCmdSetDiscardRectangleEXT`."]
	#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html)"]
	pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
	#[doc = "Cached function pointer for `vkCmdSetDiscardRectangleEnableEXT`."]
	#[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html)"]
	pub cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetDiscardRectangleModeEXT`."]
	#[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html)"]
	pub cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetSampleLocationsEXT`."]
	#[doc(alias = "vkCmdSetSampleLocationsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html)"]
	pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
	#[doc = "Cached function pointer for `vkGetBufferMemoryRequirements2`."]
	#[doc(alias = "vkGetBufferMemoryRequirements2")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html)"]
	pub get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
	#[doc = "Cached function pointer for `vkGetBufferMemoryRequirements2KHR`."]
	#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html)"]
	pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2KHR,
	#[doc = "Cached function pointer for `vkGetImageMemoryRequirements2`."]
	#[doc(alias = "vkGetImageMemoryRequirements2")]
	#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html)"]
	pub get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
	#[doc = "Cached function pointer for `vkGetImageMemoryRequirements2KHR`."]
	#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html)"]
	pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2KHR,
	#[doc = "Cached function pointer for `vkGetImageSparseMemoryRequirements2`."]
	#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html)"]
	pub get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
	#[doc = "Cached function pointer for `vkGetImageSparseMemoryRequirements2KHR`."]
	#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html)"]
	pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2KHR,
	#[doc = "Cached function pointer for `vkGetDeviceBufferMemoryRequirements`."]
	#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html)"]
	pub get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
	#[doc = "Cached function pointer for `vkGetDeviceBufferMemoryRequirementsKHR`."]
	#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html)"]
	pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirementsKHR,
	#[doc = "Cached function pointer for `vkGetDeviceImageMemoryRequirements`."]
	#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html)"]
	pub get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
	#[doc = "Cached function pointer for `vkGetDeviceImageMemoryRequirementsKHR`."]
	#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html)"]
	pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirementsKHR,
	#[doc = "Cached function pointer for `vkGetDeviceImageSparseMemoryRequirements`."]
	#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html)"]
	pub get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
	#[doc = "Cached function pointer for `vkGetDeviceImageSparseMemoryRequirementsKHR`."]
	#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html)"]
	pub get_device_image_sparse_memory_requirements_khr:
		PFN_vkGetDeviceImageSparseMemoryRequirementsKHR,
	#[doc = "Cached function pointer for `vkCreateSamplerYcbcrConversion`."]
	#[doc(alias = "vkCreateSamplerYcbcrConversion")]
	#[doc = "\n\n[Khronos reference: `vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html)"]
	pub create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
	#[doc = "Cached function pointer for `vkCreateSamplerYcbcrConversionKHR`."]
	#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html)"]
	pub create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversionKHR,
	#[doc = "Cached function pointer for `vkDestroySamplerYcbcrConversion`."]
	#[doc(alias = "vkDestroySamplerYcbcrConversion")]
	#[doc = "\n\n[Khronos reference: `vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)"]
	pub destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
	#[doc = "Cached function pointer for `vkDestroySamplerYcbcrConversionKHR`."]
	#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html)"]
	pub destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversionKHR,
	#[doc = "Cached function pointer for `vkGetDeviceQueue2`."]
	#[doc(alias = "vkGetDeviceQueue2")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html)"]
	pub get_device_queue2: PFN_vkGetDeviceQueue2,
	#[doc = "Cached function pointer for `vkCreateValidationCacheEXT`."]
	#[doc(alias = "vkCreateValidationCacheEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html)"]
	pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
	#[doc = "Cached function pointer for `vkDestroyValidationCacheEXT`."]
	#[doc(alias = "vkDestroyValidationCacheEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html)"]
	pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
	#[doc = "Cached function pointer for `vkGetValidationCacheDataEXT`."]
	#[doc(alias = "vkGetValidationCacheDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html)"]
	pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
	#[doc = "Cached function pointer for `vkMergeValidationCachesEXT`."]
	#[doc(alias = "vkMergeValidationCachesEXT")]
	#[doc = "\n\n[Khronos reference: `vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html)"]
	pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
	#[doc = "Cached function pointer for `vkGetDescriptorSetLayoutSupport`."]
	#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html)"]
	pub get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
	#[doc = "Cached function pointer for `vkGetDescriptorSetLayoutSupportKHR`."]
	#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html)"]
	pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupportKHR,
	#[doc = "Cached function pointer for `vkGetShaderInfoAMD`."]
	#[doc(alias = "vkGetShaderInfoAMD")]
	#[doc = "\n\n[Khronos reference: `vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html)"]
	pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
	#[doc = "Cached function pointer for `vkSetLocalDimmingAMD`."]
	#[doc(alias = "vkSetLocalDimmingAMD")]
	#[doc = "\n\n[Khronos reference: `vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html)"]
	pub set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
	#[doc = "Cached function pointer for `vkGetCalibratedTimestampsKHR`."]
	#[doc(alias = "vkGetCalibratedTimestampsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html)"]
	pub get_calibrated_timestamps_khr: PFN_vkGetCalibratedTimestampsKHR,
	#[doc = "Cached function pointer for `vkGetCalibratedTimestampsEXT`."]
	#[doc(alias = "vkGetCalibratedTimestampsEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html)"]
	pub get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsEXT,
	#[doc = "Cached function pointer for `vkSetDebugUtilsObjectNameEXT`."]
	#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html)"]
	pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
	#[doc = "Cached function pointer for `vkSetDebugUtilsObjectTagEXT`."]
	#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html)"]
	pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
	#[doc = "Cached function pointer for `vkQueueBeginDebugUtilsLabelEXT`."]
	#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html)"]
	pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkQueueEndDebugUtilsLabelEXT`."]
	#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html)"]
	pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkQueueInsertDebugUtilsLabelEXT`."]
	#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html)"]
	pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkCmdBeginDebugUtilsLabelEXT`."]
	#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html)"]
	pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkCmdEndDebugUtilsLabelEXT`."]
	#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html)"]
	pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkCmdInsertDebugUtilsLabelEXT`."]
	#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html)"]
	pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
	#[doc = "Cached function pointer for `vkGetMemoryHostPointerPropertiesEXT`."]
	#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html)"]
	pub get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
	#[doc = "Cached function pointer for `vkCmdWriteBufferMarkerAMD`."]
	#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html)"]
	pub cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
	#[doc = "Cached function pointer for `vkCreateRenderPass2`."]
	#[doc(alias = "vkCreateRenderPass2")]
	#[doc = "\n\n[Khronos reference: `vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html)"]
	pub create_render_pass2: PFN_vkCreateRenderPass2,
	#[doc = "Cached function pointer for `vkCreateRenderPass2KHR`."]
	#[doc(alias = "vkCreateRenderPass2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html)"]
	pub create_render_pass2_khr: PFN_vkCreateRenderPass2KHR,
	#[doc = "Cached function pointer for `vkCmdBeginRenderPass2`."]
	#[doc(alias = "vkCmdBeginRenderPass2")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html)"]
	pub cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
	#[doc = "Cached function pointer for `vkCmdBeginRenderPass2KHR`."]
	#[doc(alias = "vkCmdBeginRenderPass2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html)"]
	pub cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2KHR,
	#[doc = "Cached function pointer for `vkCmdNextSubpass2`."]
	#[doc(alias = "vkCmdNextSubpass2")]
	#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html)"]
	pub cmd_next_subpass2: PFN_vkCmdNextSubpass2,
	#[doc = "Cached function pointer for `vkCmdNextSubpass2KHR`."]
	#[doc(alias = "vkCmdNextSubpass2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html)"]
	pub cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2KHR,
	#[doc = "Cached function pointer for `vkCmdEndRenderPass2`."]
	#[doc(alias = "vkCmdEndRenderPass2")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html)"]
	pub cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
	#[doc = "Cached function pointer for `vkCmdEndRenderPass2KHR`."]
	#[doc(alias = "vkCmdEndRenderPass2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html)"]
	pub cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2KHR,
	#[doc = "Cached function pointer for `vkGetSemaphoreCounterValue`."]
	#[doc(alias = "vkGetSemaphoreCounterValue")]
	#[doc = "\n\n[Khronos reference: `vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)"]
	pub get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
	#[doc = "Cached function pointer for `vkGetSemaphoreCounterValueKHR`."]
	#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html)"]
	pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValueKHR,
	#[doc = "Cached function pointer for `vkWaitSemaphores`."]
	#[doc(alias = "vkWaitSemaphores")]
	#[doc = "\n\n[Khronos reference: `vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html)"]
	pub wait_semaphores: PFN_vkWaitSemaphores,
	#[doc = "Cached function pointer for `vkWaitSemaphoresKHR`."]
	#[doc(alias = "vkWaitSemaphoresKHR")]
	#[doc = "\n\n[Khronos reference: `vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html)"]
	pub wait_semaphores_khr: PFN_vkWaitSemaphoresKHR,
	#[doc = "Cached function pointer for `vkSignalSemaphore`."]
	#[doc(alias = "vkSignalSemaphore")]
	#[doc = "\n\n[Khronos reference: `vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html)"]
	pub signal_semaphore: PFN_vkSignalSemaphore,
	#[doc = "Cached function pointer for `vkSignalSemaphoreKHR`."]
	#[doc(alias = "vkSignalSemaphoreKHR")]
	#[doc = "\n\n[Khronos reference: `vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html)"]
	pub signal_semaphore_khr: PFN_vkSignalSemaphoreKHR,
	#[cfg(feature = "platform-android")]
	#[doc = "Cached function pointer for `vkGetAndroidHardwareBufferPropertiesANDROID`."]
	#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
	#[doc = "\n\n[Khronos reference: `vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html)"]
	pub get_android_hardware_buffer_properties_android:
		PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
	#[cfg(feature = "platform-android")]
	#[doc = "Cached function pointer for `vkGetMemoryAndroidHardwareBufferANDROID`."]
	#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html)"]
	pub get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectCount`."]
	#[doc(alias = "vkCmdDrawIndirectCount")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)"]
	pub cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectCountKHR`."]
	#[doc(alias = "vkCmdDrawIndirectCountKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html)"]
	pub cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCountKHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectCountAMD`."]
	#[doc(alias = "vkCmdDrawIndirectCountAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html)"]
	pub cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCountAMD,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirectCount`."]
	#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)"]
	pub cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirectCountKHR`."]
	#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html)"]
	pub cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCountKHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirectCountAMD`."]
	#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html)"]
	pub cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCountAMD,
	#[doc = "Cached function pointer for `vkCmdSetCheckpointNV`."]
	#[doc(alias = "vkCmdSetCheckpointNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html)"]
	pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
	#[doc = "Cached function pointer for `vkGetQueueCheckpointDataNV`."]
	#[doc(alias = "vkGetQueueCheckpointDataNV")]
	#[doc = "\n\n[Khronos reference: `vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html)"]
	pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
	#[doc = "Cached function pointer for `vkCmdBindTransformFeedbackBuffersEXT`."]
	#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html)"]
	pub cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
	#[doc = "Cached function pointer for `vkCmdBeginTransformFeedbackEXT`."]
	#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html)"]
	pub cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
	#[doc = "Cached function pointer for `vkCmdEndTransformFeedbackEXT`."]
	#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html)"]
	pub cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
	#[doc = "Cached function pointer for `vkCmdBeginQueryIndexedEXT`."]
	#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html)"]
	pub cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
	#[doc = "Cached function pointer for `vkCmdEndQueryIndexedEXT`."]
	#[doc(alias = "vkCmdEndQueryIndexedEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html)"]
	pub cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectByteCountEXT`."]
	#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html)"]
	pub cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
	#[doc = "Cached function pointer for `vkCmdSetExclusiveScissorNV`."]
	#[doc(alias = "vkCmdSetExclusiveScissorNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html)"]
	pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
	#[doc = "Cached function pointer for `vkCmdSetExclusiveScissorEnableNV`."]
	#[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html)"]
	pub cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
	#[doc = "Cached function pointer for `vkCmdBindShadingRateImageNV`."]
	#[doc(alias = "vkCmdBindShadingRateImageNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html)"]
	pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
	#[doc = "Cached function pointer for `vkCmdSetViewportShadingRatePaletteNV`."]
	#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html)"]
	pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
	#[doc = "Cached function pointer for `vkCmdSetCoarseSampleOrderNV`."]
	#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html)"]
	pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksNV`."]
	#[doc(alias = "vkCmdDrawMeshTasksNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html)"]
	pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirectNV`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html)"]
	pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirectCountNV`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html)"]
	pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksEXT`."]
	#[doc(alias = "vkCmdDrawMeshTasksEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html)"]
	pub cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirectEXT`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html)"]
	pub cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirectCountEXT`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html)"]
	pub cmd_draw_mesh_tasks_indirect_count_ext: PFN_vkCmdDrawMeshTasksIndirectCountEXT,
	#[doc = "Cached function pointer for `vkCompileDeferredNV`."]
	#[doc(alias = "vkCompileDeferredNV")]
	#[doc = "\n\n[Khronos reference: `vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html)"]
	pub compile_deferred_nv: PFN_vkCompileDeferredNV,
	#[doc = "Cached function pointer for `vkCreateAccelerationStructureNV`."]
	#[doc(alias = "vkCreateAccelerationStructureNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html)"]
	pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
	#[doc = "Cached function pointer for `vkCmdBindInvocationMaskHUAWEI`."]
	#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html)"]
	pub cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
	#[doc = "Cached function pointer for `vkDestroyAccelerationStructureKHR`."]
	#[doc(alias = "vkDestroyAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html)"]
	pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkDestroyAccelerationStructureNV`."]
	#[doc(alias = "vkDestroyAccelerationStructureNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)"]
	pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
	#[doc = "Cached function pointer for `vkGetAccelerationStructureMemoryRequirementsNV`."]
	#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html)"]
	pub get_acceleration_structure_memory_requirements_nv:
		PFN_vkGetAccelerationStructureMemoryRequirementsNV,
	#[doc = "Cached function pointer for `vkBindAccelerationStructureMemoryNV`."]
	#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
	#[doc = "\n\n[Khronos reference: `vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html)"]
	pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
	#[doc = "Cached function pointer for `vkCmdCopyAccelerationStructureNV`."]
	#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html)"]
	pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
	#[doc = "Cached function pointer for `vkCmdCopyAccelerationStructureKHR`."]
	#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html)"]
	pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkCopyAccelerationStructureKHR`."]
	#[doc(alias = "vkCopyAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html)"]
	pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkCmdCopyAccelerationStructureToMemoryKHR`."]
	#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html)"]
	pub cmd_copy_acceleration_structure_to_memory_khr: PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
	#[doc = "Cached function pointer for `vkCopyAccelerationStructureToMemoryKHR`."]
	#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html)"]
	pub copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryToAccelerationStructureKHR`."]
	#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html)"]
	pub cmd_copy_memory_to_acceleration_structure_khr: PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkCopyMemoryToAccelerationStructureKHR`."]
	#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html)"]
	pub copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkCmdWriteAccelerationStructuresPropertiesKHR`."]
	#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html)"]
	pub cmd_write_acceleration_structures_properties_khr:
		PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
	#[doc = "Cached function pointer for `vkCmdWriteAccelerationStructuresPropertiesNV`."]
	#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html)"]
	pub cmd_write_acceleration_structures_properties_nv:
		PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
	#[doc = "Cached function pointer for `vkCmdBuildAccelerationStructureNV`."]
	#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html)"]
	pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
	#[doc = "Cached function pointer for `vkWriteAccelerationStructuresPropertiesKHR`."]
	#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html)"]
	pub write_acceleration_structures_properties_khr: PFN_vkWriteAccelerationStructuresPropertiesKHR,
	#[doc = "Cached function pointer for `vkCmdTraceRaysKHR`."]
	#[doc(alias = "vkCmdTraceRaysKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html)"]
	pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
	#[doc = "Cached function pointer for `vkCmdTraceRaysNV`."]
	#[doc(alias = "vkCmdTraceRaysNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html)"]
	pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
	#[doc = "Cached function pointer for `vkGetRayTracingShaderGroupHandlesKHR`."]
	#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html)"]
	pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
	#[doc = "Cached function pointer for `vkGetRayTracingShaderGroupHandlesNV`."]
	#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html)"]
	pub get_ray_tracing_shader_group_handles_nv: PFN_vkGetRayTracingShaderGroupHandlesNV,
	#[doc = "Cached function pointer for `vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`."]
	#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)"]
	pub get_ray_tracing_capture_replay_shader_group_handles_khr:
		PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
	#[doc = "Cached function pointer for `vkGetAccelerationStructureHandleNV`."]
	#[doc(alias = "vkGetAccelerationStructureHandleNV")]
	#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)"]
	pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
	#[doc = "Cached function pointer for `vkCreateRayTracingPipelinesNV`."]
	#[doc(alias = "vkCreateRayTracingPipelinesNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html)"]
	pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
	#[doc = "Cached function pointer for `vkCreateRayTracingPipelinesKHR`."]
	#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html)"]
	pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
	#[doc = "Cached function pointer for `vkCmdTraceRaysIndirectKHR`."]
	#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html)"]
	pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
	#[doc = "Cached function pointer for `vkCmdTraceRaysIndirect2KHR`."]
	#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html)"]
	pub cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
	#[doc = "Cached function pointer for `vkGetClusterAccelerationStructureBuildSizesNV`."]
	#[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html)"]
	pub get_cluster_acceleration_structure_build_sizes_nv:
		PFN_vkGetClusterAccelerationStructureBuildSizesNV,
	#[doc = "Cached function pointer for `vkCmdBuildClusterAccelerationStructureIndirectNV`."]
	#[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html)"]
	pub cmd_build_cluster_acceleration_structure_indirect_nv:
		PFN_vkCmdBuildClusterAccelerationStructureIndirectNV,
	#[doc = "Cached function pointer for `vkGetDeviceAccelerationStructureCompatibilityKHR`."]
	#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html)"]
	pub get_device_acceleration_structure_compatibility_khr:
		PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
	#[doc = "Cached function pointer for `vkGetRayTracingShaderGroupStackSizeKHR`."]
	#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html)"]
	pub get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
	#[doc = "Cached function pointer for `vkCmdSetRayTracingPipelineStackSizeKHR`."]
	#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html)"]
	pub cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
	#[doc = "Cached function pointer for `vkGetImageViewHandleNVX`."]
	#[doc(alias = "vkGetImageViewHandleNVX")]
	#[doc = "\n\n[Khronos reference: `vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html)"]
	pub get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
	#[doc = "Cached function pointer for `vkGetImageViewHandle64NVX`."]
	#[doc(alias = "vkGetImageViewHandle64NVX")]
	#[doc = "\n\n[Khronos reference: `vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html)"]
	pub get_image_view_handle64_nvx: PFN_vkGetImageViewHandle64NVX,
	#[doc = "Cached function pointer for `vkGetImageViewAddressNVX`."]
	#[doc(alias = "vkGetImageViewAddressNVX")]
	#[doc = "\n\n[Khronos reference: `vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html)"]
	pub get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
	#[doc = "Cached function pointer for `vkGetDeviceCombinedImageSamplerIndexNVX`."]
	#[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html)"]
	pub get_device_combined_image_sampler_index_nvx: PFN_vkGetDeviceCombinedImageSamplerIndexNVX,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkGetDeviceGroupSurfacePresentModes2EXT`."]
	#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html)"]
	pub get_device_group_surface_present_modes2_ext: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkAcquireFullScreenExclusiveModeEXT`."]
	#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)"]
	pub acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
	#[cfg(feature = "platform-win32")]
	#[doc = "Cached function pointer for `vkReleaseFullScreenExclusiveModeEXT`."]
	#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)"]
	pub release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
	#[doc = "Cached function pointer for `vkAcquireProfilingLockKHR`."]
	#[doc(alias = "vkAcquireProfilingLockKHR")]
	#[doc = "\n\n[Khronos reference: `vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html)"]
	pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
	#[doc = "Cached function pointer for `vkReleaseProfilingLockKHR`."]
	#[doc(alias = "vkReleaseProfilingLockKHR")]
	#[doc = "\n\n[Khronos reference: `vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html)"]
	pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
	#[doc = "Cached function pointer for `vkGetImageDrmFormatModifierPropertiesEXT`."]
	#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html)"]
	pub get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
	#[doc = "Cached function pointer for `vkGetBufferOpaqueCaptureAddress`."]
	#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html)"]
	pub get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
	#[doc = "Cached function pointer for `vkGetBufferOpaqueCaptureAddressKHR`."]
	#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html)"]
	pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddressKHR,
	#[doc = "Cached function pointer for `vkGetBufferDeviceAddress`."]
	#[doc(alias = "vkGetBufferDeviceAddress")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)"]
	pub get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
	#[doc = "Cached function pointer for `vkGetBufferDeviceAddressKHR`."]
	#[doc(alias = "vkGetBufferDeviceAddressKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html)"]
	pub get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddressKHR,
	#[doc = "Cached function pointer for `vkGetBufferDeviceAddressEXT`."]
	#[doc(alias = "vkGetBufferDeviceAddressEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html)"]
	pub get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddressEXT,
	#[doc = "Cached function pointer for `vkInitializePerformanceApiINTEL`."]
	#[doc(alias = "vkInitializePerformanceApiINTEL")]
	#[doc = "\n\n[Khronos reference: `vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html)"]
	pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
	#[doc = "Cached function pointer for `vkUninitializePerformanceApiINTEL`."]
	#[doc(alias = "vkUninitializePerformanceApiINTEL")]
	#[doc = "\n\n[Khronos reference: `vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html)"]
	pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
	#[doc = "Cached function pointer for `vkCmdSetPerformanceMarkerINTEL`."]
	#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html)"]
	pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
	#[doc = "Cached function pointer for `vkCmdSetPerformanceStreamMarkerINTEL`."]
	#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html)"]
	pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
	#[doc = "Cached function pointer for `vkCmdSetPerformanceOverrideINTEL`."]
	#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html)"]
	pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
	#[doc = "Cached function pointer for `vkAcquirePerformanceConfigurationINTEL`."]
	#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
	#[doc = "\n\n[Khronos reference: `vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html)"]
	pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
	#[doc = "Cached function pointer for `vkReleasePerformanceConfigurationINTEL`."]
	#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
	#[doc = "\n\n[Khronos reference: `vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html)"]
	pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
	#[doc = "Cached function pointer for `vkQueueSetPerformanceConfigurationINTEL`."]
	#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
	#[doc = "\n\n[Khronos reference: `vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html)"]
	pub queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
	#[doc = "Cached function pointer for `vkGetPerformanceParameterINTEL`."]
	#[doc(alias = "vkGetPerformanceParameterINTEL")]
	#[doc = "\n\n[Khronos reference: `vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html)"]
	pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
	#[doc = "Cached function pointer for `vkGetDeviceMemoryOpaqueCaptureAddress`."]
	#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html)"]
	pub get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
	#[doc = "Cached function pointer for `vkGetDeviceMemoryOpaqueCaptureAddressKHR`."]
	#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)"]
	pub get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR,
	#[doc = "Cached function pointer for `vkGetPipelineExecutablePropertiesKHR`."]
	#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html)"]
	pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
	#[doc = "Cached function pointer for `vkGetPipelineExecutableStatisticsKHR`."]
	#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html)"]
	pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
	#[doc = "Cached function pointer for `vkGetPipelineExecutableInternalRepresentationsKHR`."]
	#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html)"]
	pub get_pipeline_executable_internal_representations_khr:
		PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
	#[doc = "Cached function pointer for `vkCmdSetLineStipple`."]
	#[doc(alias = "vkCmdSetLineStipple")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)"]
	pub cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
	#[doc = "Cached function pointer for `vkCmdSetLineStippleKHR`."]
	#[doc(alias = "vkCmdSetLineStippleKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html)"]
	pub cmd_set_line_stipple_khr: PFN_vkCmdSetLineStippleKHR,
	#[doc = "Cached function pointer for `vkCmdSetLineStippleEXT`."]
	#[doc(alias = "vkCmdSetLineStippleEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html)"]
	pub cmd_set_line_stipple_ext: PFN_vkCmdSetLineStippleEXT,
	#[doc = "Cached function pointer for `vkCreateAccelerationStructureKHR`."]
	#[doc(alias = "vkCreateAccelerationStructureKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html)"]
	pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
	#[doc = "Cached function pointer for `vkCmdBuildAccelerationStructuresKHR`."]
	#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html)"]
	pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
	#[doc = "Cached function pointer for `vkCmdBuildAccelerationStructuresIndirectKHR`."]
	#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html)"]
	pub cmd_build_acceleration_structures_indirect_khr:
		PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
	#[doc = "Cached function pointer for `vkBuildAccelerationStructuresKHR`."]
	#[doc(alias = "vkBuildAccelerationStructuresKHR")]
	#[doc = "\n\n[Khronos reference: `vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html)"]
	pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
	#[doc = "Cached function pointer for `vkGetAccelerationStructureDeviceAddressKHR`."]
	#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html)"]
	pub get_acceleration_structure_device_address_khr: PFN_vkGetAccelerationStructureDeviceAddressKHR,
	#[doc = "Cached function pointer for `vkCreateDeferredOperationKHR`."]
	#[doc(alias = "vkCreateDeferredOperationKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html)"]
	pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
	#[doc = "Cached function pointer for `vkDestroyDeferredOperationKHR`."]
	#[doc(alias = "vkDestroyDeferredOperationKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html)"]
	pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
	#[doc = "Cached function pointer for `vkGetDeferredOperationMaxConcurrencyKHR`."]
	#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html)"]
	pub get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
	#[doc = "Cached function pointer for `vkGetDeferredOperationResultKHR`."]
	#[doc(alias = "vkGetDeferredOperationResultKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html)"]
	pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
	#[doc = "Cached function pointer for `vkDeferredOperationJoinKHR`."]
	#[doc(alias = "vkDeferredOperationJoinKHR")]
	#[doc = "\n\n[Khronos reference: `vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html)"]
	pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
	#[doc = "Cached function pointer for `vkGetPipelineIndirectMemoryRequirementsNV`."]
	#[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html)"]
	pub get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
	#[doc = "Cached function pointer for `vkGetPipelineIndirectDeviceAddressNV`."]
	#[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html)"]
	pub get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
	#[doc = "Cached function pointer for `vkAntiLagUpdateAMD`."]
	#[doc(alias = "vkAntiLagUpdateAMD")]
	#[doc = "\n\n[Khronos reference: `vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html)"]
	pub anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
	#[doc = "Cached function pointer for `vkCmdSetCullMode`."]
	#[doc(alias = "vkCmdSetCullMode")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html)"]
	pub cmd_set_cull_mode: PFN_vkCmdSetCullMode,
	#[doc = "Cached function pointer for `vkCmdSetCullModeEXT`."]
	#[doc(alias = "vkCmdSetCullModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html)"]
	pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetFrontFace`."]
	#[doc(alias = "vkCmdSetFrontFace")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html)"]
	pub cmd_set_front_face: PFN_vkCmdSetFrontFace,
	#[doc = "Cached function pointer for `vkCmdSetFrontFaceEXT`."]
	#[doc(alias = "vkCmdSetFrontFaceEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html)"]
	pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFaceEXT,
	#[doc = "Cached function pointer for `vkCmdSetPrimitiveTopology`."]
	#[doc(alias = "vkCmdSetPrimitiveTopology")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html)"]
	pub cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
	#[doc = "Cached function pointer for `vkCmdSetPrimitiveTopologyEXT`."]
	#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html)"]
	pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopologyEXT,
	#[doc = "Cached function pointer for `vkCmdSetViewportWithCount`."]
	#[doc(alias = "vkCmdSetViewportWithCount")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html)"]
	pub cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
	#[doc = "Cached function pointer for `vkCmdSetViewportWithCountEXT`."]
	#[doc(alias = "vkCmdSetViewportWithCountEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html)"]
	pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCountEXT,
	#[doc = "Cached function pointer for `vkCmdSetScissorWithCount`."]
	#[doc(alias = "vkCmdSetScissorWithCount")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html)"]
	pub cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
	#[doc = "Cached function pointer for `vkCmdSetScissorWithCountEXT`."]
	#[doc(alias = "vkCmdSetScissorWithCountEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html)"]
	pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCountEXT,
	#[doc = "Cached function pointer for `vkCmdBindIndexBuffer2`."]
	#[doc(alias = "vkCmdBindIndexBuffer2")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html)"]
	pub cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
	#[doc = "Cached function pointer for `vkCmdBindIndexBuffer2KHR`."]
	#[doc(alias = "vkCmdBindIndexBuffer2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html)"]
	pub cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2KHR,
	#[doc = "Cached function pointer for `vkCmdBindVertexBuffers2`."]
	#[doc(alias = "vkCmdBindVertexBuffers2")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html)"]
	pub cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
	#[doc = "Cached function pointer for `vkCmdBindVertexBuffers2EXT`."]
	#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html)"]
	pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2EXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthTestEnable`."]
	#[doc(alias = "vkCmdSetDepthTestEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html)"]
	pub cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
	#[doc = "Cached function pointer for `vkCmdSetDepthTestEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html)"]
	pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthWriteEnable`."]
	#[doc(alias = "vkCmdSetDepthWriteEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html)"]
	pub cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
	#[doc = "Cached function pointer for `vkCmdSetDepthWriteEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html)"]
	pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthCompareOp`."]
	#[doc(alias = "vkCmdSetDepthCompareOp")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html)"]
	pub cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
	#[doc = "Cached function pointer for `vkCmdSetDepthCompareOpEXT`."]
	#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html)"]
	pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOpEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthBoundsTestEnable`."]
	#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html)"]
	pub cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
	#[doc = "Cached function pointer for `vkCmdSetDepthBoundsTestEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html)"]
	pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetStencilTestEnable`."]
	#[doc(alias = "vkCmdSetStencilTestEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html)"]
	pub cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
	#[doc = "Cached function pointer for `vkCmdSetStencilTestEnableEXT`."]
	#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html)"]
	pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetStencilOp`."]
	#[doc(alias = "vkCmdSetStencilOp")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html)"]
	pub cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
	#[doc = "Cached function pointer for `vkCmdSetStencilOpEXT`."]
	#[doc(alias = "vkCmdSetStencilOpEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html)"]
	pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOpEXT,
	#[doc = "Cached function pointer for `vkCmdSetPatchControlPointsEXT`."]
	#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html)"]
	pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
	#[doc = "Cached function pointer for `vkCmdSetRasterizerDiscardEnable`."]
	#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html)"]
	pub cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
	#[doc = "Cached function pointer for `vkCmdSetRasterizerDiscardEnableEXT`."]
	#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html)"]
	pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthBiasEnable`."]
	#[doc(alias = "vkCmdSetDepthBiasEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html)"]
	pub cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
	#[doc = "Cached function pointer for `vkCmdSetDepthBiasEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html)"]
	pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetLogicOpEXT`."]
	#[doc(alias = "vkCmdSetLogicOpEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html)"]
	pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
	#[doc = "Cached function pointer for `vkCmdSetPrimitiveRestartEnable`."]
	#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html)"]
	pub cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
	#[doc = "Cached function pointer for `vkCmdSetPrimitiveRestartEnableEXT`."]
	#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html)"]
	pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetTessellationDomainOriginEXT`."]
	#[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html)"]
	pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthClampEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html)"]
	pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetPolygonModeEXT`."]
	#[doc(alias = "vkCmdSetPolygonModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html)"]
	pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetRasterizationSamplesEXT`."]
	#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html)"]
	pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
	#[doc = "Cached function pointer for `vkCmdSetSampleMaskEXT`."]
	#[doc(alias = "vkCmdSetSampleMaskEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html)"]
	pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
	#[doc = "Cached function pointer for `vkCmdSetAlphaToCoverageEnableEXT`."]
	#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html)"]
	pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetAlphaToOneEnableEXT`."]
	#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html)"]
	pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetLogicOpEnableEXT`."]
	#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html)"]
	pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetColorBlendEnableEXT`."]
	#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html)"]
	pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetColorBlendEquationEXT`."]
	#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html)"]
	pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
	#[doc = "Cached function pointer for `vkCmdSetColorWriteMaskEXT`."]
	#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html)"]
	pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
	#[doc = "Cached function pointer for `vkCmdSetRasterizationStreamEXT`."]
	#[doc(alias = "vkCmdSetRasterizationStreamEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html)"]
	pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
	#[doc = "Cached function pointer for `vkCmdSetConservativeRasterizationModeEXT`."]
	#[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html)"]
	pub cmd_set_conservative_rasterization_mode_ext: PFN_vkCmdSetConservativeRasterizationModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetExtraPrimitiveOverestimationSizeEXT`."]
	#[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)"]
	pub cmd_set_extra_primitive_overestimation_size_ext:
		PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthClipEnableEXT`."]
	#[doc(alias = "vkCmdSetDepthClipEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html)"]
	pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetSampleLocationsEnableEXT`."]
	#[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html)"]
	pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetColorBlendAdvancedEXT`."]
	#[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html)"]
	pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
	#[doc = "Cached function pointer for `vkCmdSetProvokingVertexModeEXT`."]
	#[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html)"]
	pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetLineRasterizationModeEXT`."]
	#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html)"]
	pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
	#[doc = "Cached function pointer for `vkCmdSetLineStippleEnableEXT`."]
	#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html)"]
	pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetDepthClipNegativeOneToOneEXT`."]
	#[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html)"]
	pub cmd_set_depth_clip_negative_one_to_one_ext: PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
	#[doc = "Cached function pointer for `vkCmdSetViewportWScalingEnableNV`."]
	#[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html)"]
	pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
	#[doc = "Cached function pointer for `vkCmdSetViewportSwizzleNV`."]
	#[doc(alias = "vkCmdSetViewportSwizzleNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html)"]
	pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageToColorEnableNV`."]
	#[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html)"]
	pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageToColorLocationNV`."]
	#[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html)"]
	pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageModulationModeNV`."]
	#[doc(alias = "vkCmdSetCoverageModulationModeNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html)"]
	pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageModulationTableEnableNV`."]
	#[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html)"]
	pub cmd_set_coverage_modulation_table_enable_nv: PFN_vkCmdSetCoverageModulationTableEnableNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageModulationTableNV`."]
	#[doc(alias = "vkCmdSetCoverageModulationTableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html)"]
	pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
	#[doc = "Cached function pointer for `vkCmdSetShadingRateImageEnableNV`."]
	#[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html)"]
	pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
	#[doc = "Cached function pointer for `vkCmdSetCoverageReductionModeNV`."]
	#[doc(alias = "vkCmdSetCoverageReductionModeNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html)"]
	pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
	#[doc = "Cached function pointer for `vkCmdSetRepresentativeFragmentTestEnableNV`."]
	#[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html)"]
	pub cmd_set_representative_fragment_test_enable_nv:
		PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
	#[doc = "Cached function pointer for `vkCreatePrivateDataSlot`."]
	#[doc(alias = "vkCreatePrivateDataSlot")]
	#[doc = "\n\n[Khronos reference: `vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html)"]
	pub create_private_data_slot: PFN_vkCreatePrivateDataSlot,
	#[doc = "Cached function pointer for `vkCreatePrivateDataSlotEXT`."]
	#[doc(alias = "vkCreatePrivateDataSlotEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html)"]
	pub create_private_data_slot_ext: PFN_vkCreatePrivateDataSlotEXT,
	#[doc = "Cached function pointer for `vkDestroyPrivateDataSlot`."]
	#[doc(alias = "vkDestroyPrivateDataSlot")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)"]
	pub destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
	#[doc = "Cached function pointer for `vkDestroyPrivateDataSlotEXT`."]
	#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html)"]
	pub destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlotEXT,
	#[doc = "Cached function pointer for `vkSetPrivateData`."]
	#[doc(alias = "vkSetPrivateData")]
	#[doc = "\n\n[Khronos reference: `vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html)"]
	pub set_private_data: PFN_vkSetPrivateData,
	#[doc = "Cached function pointer for `vkSetPrivateDataEXT`."]
	#[doc(alias = "vkSetPrivateDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html)"]
	pub set_private_data_ext: PFN_vkSetPrivateDataEXT,
	#[doc = "Cached function pointer for `vkGetPrivateData`."]
	#[doc(alias = "vkGetPrivateData")]
	#[doc = "\n\n[Khronos reference: `vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html)"]
	pub get_private_data: PFN_vkGetPrivateData,
	#[doc = "Cached function pointer for `vkGetPrivateDataEXT`."]
	#[doc(alias = "vkGetPrivateDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html)"]
	pub get_private_data_ext: PFN_vkGetPrivateDataEXT,
	#[doc = "Cached function pointer for `vkCmdCopyBuffer2`."]
	#[doc(alias = "vkCmdCopyBuffer2")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html)"]
	pub cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
	#[doc = "Cached function pointer for `vkCmdCopyBuffer2KHR`."]
	#[doc(alias = "vkCmdCopyBuffer2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html)"]
	pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2KHR,
	#[doc = "Cached function pointer for `vkCmdCopyImage2`."]
	#[doc(alias = "vkCmdCopyImage2")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html)"]
	pub cmd_copy_image2: PFN_vkCmdCopyImage2,
	#[doc = "Cached function pointer for `vkCmdCopyImage2KHR`."]
	#[doc(alias = "vkCmdCopyImage2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html)"]
	pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2KHR,
	#[doc = "Cached function pointer for `vkCmdBlitImage2`."]
	#[doc(alias = "vkCmdBlitImage2")]
	#[doc = "\n\n[Khronos reference: `vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html)"]
	pub cmd_blit_image2: PFN_vkCmdBlitImage2,
	#[doc = "Cached function pointer for `vkCmdBlitImage2KHR`."]
	#[doc(alias = "vkCmdBlitImage2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html)"]
	pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2KHR,
	#[doc = "Cached function pointer for `vkCmdCopyBufferToImage2`."]
	#[doc(alias = "vkCmdCopyBufferToImage2")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html)"]
	pub cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
	#[doc = "Cached function pointer for `vkCmdCopyBufferToImage2KHR`."]
	#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html)"]
	pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2KHR,
	#[doc = "Cached function pointer for `vkCmdCopyImageToBuffer2`."]
	#[doc(alias = "vkCmdCopyImageToBuffer2")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html)"]
	pub cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
	#[doc = "Cached function pointer for `vkCmdCopyImageToBuffer2KHR`."]
	#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html)"]
	pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2KHR,
	#[doc = "Cached function pointer for `vkCmdResolveImage2`."]
	#[doc(alias = "vkCmdResolveImage2")]
	#[doc = "\n\n[Khronos reference: `vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html)"]
	pub cmd_resolve_image2: PFN_vkCmdResolveImage2,
	#[doc = "Cached function pointer for `vkCmdResolveImage2KHR`."]
	#[doc(alias = "vkCmdResolveImage2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html)"]
	pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2KHR,
	#[doc = "Cached function pointer for `vkCmdSetFragmentShadingRateKHR`."]
	#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html)"]
	pub cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
	#[doc = "Cached function pointer for `vkCmdSetFragmentShadingRateEnumNV`."]
	#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html)"]
	pub cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
	#[doc = "Cached function pointer for `vkGetAccelerationStructureBuildSizesKHR`."]
	#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html)"]
	pub get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
	#[doc = "Cached function pointer for `vkCmdSetVertexInputEXT`."]
	#[doc(alias = "vkCmdSetVertexInputEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html)"]
	pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
	#[doc = "Cached function pointer for `vkCmdSetColorWriteEnableEXT`."]
	#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html)"]
	pub cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
	#[doc = "Cached function pointer for `vkCmdSetEvent2`."]
	#[doc(alias = "vkCmdSetEvent2")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html)"]
	pub cmd_set_event2: PFN_vkCmdSetEvent2,
	#[doc = "Cached function pointer for `vkCmdSetEvent2KHR`."]
	#[doc(alias = "vkCmdSetEvent2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html)"]
	pub cmd_set_event2_khr: PFN_vkCmdSetEvent2KHR,
	#[doc = "Cached function pointer for `vkCmdResetEvent2`."]
	#[doc(alias = "vkCmdResetEvent2")]
	#[doc = "\n\n[Khronos reference: `vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html)"]
	pub cmd_reset_event2: PFN_vkCmdResetEvent2,
	#[doc = "Cached function pointer for `vkCmdResetEvent2KHR`."]
	#[doc(alias = "vkCmdResetEvent2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html)"]
	pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2KHR,
	#[doc = "Cached function pointer for `vkCmdWaitEvents2`."]
	#[doc(alias = "vkCmdWaitEvents2")]
	#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html)"]
	pub cmd_wait_events2: PFN_vkCmdWaitEvents2,
	#[doc = "Cached function pointer for `vkCmdWaitEvents2KHR`."]
	#[doc(alias = "vkCmdWaitEvents2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html)"]
	pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2KHR,
	#[doc = "Cached function pointer for `vkCmdPipelineBarrier2`."]
	#[doc(alias = "vkCmdPipelineBarrier2")]
	#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html)"]
	pub cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
	#[doc = "Cached function pointer for `vkCmdPipelineBarrier2KHR`."]
	#[doc(alias = "vkCmdPipelineBarrier2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html)"]
	pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2KHR,
	#[doc = "Cached function pointer for `vkQueueSubmit2`."]
	#[doc(alias = "vkQueueSubmit2")]
	#[doc = "\n\n[Khronos reference: `vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)"]
	pub queue_submit2: PFN_vkQueueSubmit2,
	#[doc = "Cached function pointer for `vkQueueSubmit2KHR`."]
	#[doc(alias = "vkQueueSubmit2KHR")]
	#[doc = "\n\n[Khronos reference: `vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html)"]
	pub queue_submit2_khr: PFN_vkQueueSubmit2KHR,
	#[doc = "Cached function pointer for `vkCmdWriteTimestamp2`."]
	#[doc(alias = "vkCmdWriteTimestamp2")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html)"]
	pub cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
	#[doc = "Cached function pointer for `vkCmdWriteTimestamp2KHR`."]
	#[doc(alias = "vkCmdWriteTimestamp2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html)"]
	pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2KHR,
	#[doc = "Cached function pointer for `vkCmdWriteBufferMarker2AMD`."]
	#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html)"]
	pub cmd_write_buffer_marker2_amd: PFN_vkCmdWriteBufferMarker2AMD,
	#[doc = "Cached function pointer for `vkGetQueueCheckpointData2NV`."]
	#[doc(alias = "vkGetQueueCheckpointData2NV")]
	#[doc = "\n\n[Khronos reference: `vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html)"]
	pub get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
	#[doc = "Cached function pointer for `vkCopyMemoryToImage`."]
	#[doc(alias = "vkCopyMemoryToImage")]
	#[doc = "\n\n[Khronos reference: `vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html)"]
	pub copy_memory_to_image: PFN_vkCopyMemoryToImage,
	#[doc = "Cached function pointer for `vkCopyMemoryToImageEXT`."]
	#[doc(alias = "vkCopyMemoryToImageEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html)"]
	pub copy_memory_to_image_ext: PFN_vkCopyMemoryToImageEXT,
	#[doc = "Cached function pointer for `vkCopyImageToMemory`."]
	#[doc(alias = "vkCopyImageToMemory")]
	#[doc = "\n\n[Khronos reference: `vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html)"]
	pub copy_image_to_memory: PFN_vkCopyImageToMemory,
	#[doc = "Cached function pointer for `vkCopyImageToMemoryEXT`."]
	#[doc(alias = "vkCopyImageToMemoryEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html)"]
	pub copy_image_to_memory_ext: PFN_vkCopyImageToMemoryEXT,
	#[doc = "Cached function pointer for `vkCopyImageToImage`."]
	#[doc(alias = "vkCopyImageToImage")]
	#[doc = "\n\n[Khronos reference: `vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html)"]
	pub copy_image_to_image: PFN_vkCopyImageToImage,
	#[doc = "Cached function pointer for `vkCopyImageToImageEXT`."]
	#[doc(alias = "vkCopyImageToImageEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html)"]
	pub copy_image_to_image_ext: PFN_vkCopyImageToImageEXT,
	#[doc = "Cached function pointer for `vkTransitionImageLayout`."]
	#[doc(alias = "vkTransitionImageLayout")]
	#[doc = "\n\n[Khronos reference: `vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html)"]
	pub transition_image_layout: PFN_vkTransitionImageLayout,
	#[doc = "Cached function pointer for `vkTransitionImageLayoutEXT`."]
	#[doc(alias = "vkTransitionImageLayoutEXT")]
	#[doc = "\n\n[Khronos reference: `vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html)"]
	pub transition_image_layout_ext: PFN_vkTransitionImageLayoutEXT,
	#[doc = "Cached function pointer for `vkCreateVideoSessionKHR`."]
	#[doc(alias = "vkCreateVideoSessionKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html)"]
	pub create_video_session_khr: PFN_vkCreateVideoSessionKHR,
	#[doc = "Cached function pointer for `vkDestroyVideoSessionKHR`."]
	#[doc(alias = "vkDestroyVideoSessionKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html)"]
	pub destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
	#[doc = "Cached function pointer for `vkCreateVideoSessionParametersKHR`."]
	#[doc(alias = "vkCreateVideoSessionParametersKHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html)"]
	pub create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
	#[doc = "Cached function pointer for `vkUpdateVideoSessionParametersKHR`."]
	#[doc(alias = "vkUpdateVideoSessionParametersKHR")]
	#[doc = "\n\n[Khronos reference: `vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)"]
	pub update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
	#[doc = "Cached function pointer for `vkGetEncodedVideoSessionParametersKHR`."]
	#[doc(alias = "vkGetEncodedVideoSessionParametersKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html)"]
	pub get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
	#[doc = "Cached function pointer for `vkDestroyVideoSessionParametersKHR`."]
	#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
	#[doc = "\n\n[Khronos reference: `vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)"]
	pub destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
	#[doc = "Cached function pointer for `vkGetVideoSessionMemoryRequirementsKHR`."]
	#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html)"]
	pub get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
	#[doc = "Cached function pointer for `vkBindVideoSessionMemoryKHR`."]
	#[doc(alias = "vkBindVideoSessionMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html)"]
	pub bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdDecodeVideoKHR`."]
	#[doc(alias = "vkCmdDecodeVideoKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html)"]
	pub cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
	#[doc = "Cached function pointer for `vkCmdBeginVideoCodingKHR`."]
	#[doc(alias = "vkCmdBeginVideoCodingKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html)"]
	pub cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
	#[doc = "Cached function pointer for `vkCmdControlVideoCodingKHR`."]
	#[doc(alias = "vkCmdControlVideoCodingKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html)"]
	pub cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
	#[doc = "Cached function pointer for `vkCmdEndVideoCodingKHR`."]
	#[doc(alias = "vkCmdEndVideoCodingKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html)"]
	pub cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
	#[doc = "Cached function pointer for `vkCmdEncodeVideoKHR`."]
	#[doc(alias = "vkCmdEncodeVideoKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html)"]
	pub cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
	#[doc = "Cached function pointer for `vkCmdDecompressMemoryNV`."]
	#[doc(alias = "vkCmdDecompressMemoryNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html)"]
	pub cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
	#[doc = "Cached function pointer for `vkCmdDecompressMemoryIndirectCountNV`."]
	#[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html)"]
	pub cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
	#[doc = "Cached function pointer for `vkGetPartitionedAccelerationStructuresBuildSizesNV`."]
	#[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
	#[doc = "\n\n[Khronos reference: `vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html)"]
	pub get_partitioned_acceleration_structures_build_sizes_nv:
		PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV,
	#[doc = "Cached function pointer for `vkCmdBuildPartitionedAccelerationStructuresNV`."]
	#[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html)"]
	pub cmd_build_partitioned_acceleration_structures_nv:
		PFN_vkCmdBuildPartitionedAccelerationStructuresNV,
	#[doc = "Cached function pointer for `vkCmdDecompressMemoryEXT`."]
	#[doc(alias = "vkCmdDecompressMemoryEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html)"]
	pub cmd_decompress_memory_ext: PFN_vkCmdDecompressMemoryEXT,
	#[doc = "Cached function pointer for `vkCmdDecompressMemoryIndirectCountEXT`."]
	#[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html)"]
	pub cmd_decompress_memory_indirect_count_ext: PFN_vkCmdDecompressMemoryIndirectCountEXT,
	#[doc = "Cached function pointer for `vkCreateCuModuleNVX`."]
	#[doc(alias = "vkCreateCuModuleNVX")]
	#[doc = "\n\n[Khronos reference: `vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html)"]
	pub create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
	#[doc = "Cached function pointer for `vkCreateCuFunctionNVX`."]
	#[doc(alias = "vkCreateCuFunctionNVX")]
	#[doc = "\n\n[Khronos reference: `vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html)"]
	pub create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
	#[doc = "Cached function pointer for `vkDestroyCuModuleNVX`."]
	#[doc(alias = "vkDestroyCuModuleNVX")]
	#[doc = "\n\n[Khronos reference: `vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html)"]
	pub destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
	#[doc = "Cached function pointer for `vkDestroyCuFunctionNVX`."]
	#[doc(alias = "vkDestroyCuFunctionNVX")]
	#[doc = "\n\n[Khronos reference: `vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html)"]
	pub destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
	#[doc = "Cached function pointer for `vkCmdCuLaunchKernelNVX`."]
	#[doc(alias = "vkCmdCuLaunchKernelNVX")]
	#[doc = "\n\n[Khronos reference: `vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html)"]
	pub cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
	#[doc = "Cached function pointer for `vkGetDescriptorSetLayoutSizeEXT`."]
	#[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html)"]
	pub get_descriptor_set_layout_size_ext: PFN_vkGetDescriptorSetLayoutSizeEXT,
	#[doc = "Cached function pointer for `vkGetDescriptorSetLayoutBindingOffsetEXT`."]
	#[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html)"]
	pub get_descriptor_set_layout_binding_offset_ext: PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
	#[doc = "Cached function pointer for `vkGetDescriptorEXT`."]
	#[doc(alias = "vkGetDescriptorEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html)"]
	pub get_descriptor_ext: PFN_vkGetDescriptorEXT,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorBuffersEXT`."]
	#[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html)"]
	pub cmd_bind_descriptor_buffers_ext: PFN_vkCmdBindDescriptorBuffersEXT,
	#[doc = "Cached function pointer for `vkCmdSetDescriptorBufferOffsetsEXT`."]
	#[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html)"]
	pub cmd_set_descriptor_buffer_offsets_ext: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorBufferEmbeddedSamplersEXT`."]
	#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html)"]
	pub cmd_bind_descriptor_buffer_embedded_samplers_ext:
		PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
	#[doc = "Cached function pointer for `vkGetBufferOpaqueCaptureDescriptorDataEXT`."]
	#[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html)"]
	pub get_buffer_opaque_capture_descriptor_data_ext: PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
	#[doc = "Cached function pointer for `vkGetImageOpaqueCaptureDescriptorDataEXT`."]
	#[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html)"]
	pub get_image_opaque_capture_descriptor_data_ext: PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
	#[doc = "Cached function pointer for `vkGetImageViewOpaqueCaptureDescriptorDataEXT`."]
	#[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html)"]
	pub get_image_view_opaque_capture_descriptor_data_ext:
		PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
	#[doc = "Cached function pointer for `vkGetSamplerOpaqueCaptureDescriptorDataEXT`."]
	#[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html)"]
	pub get_sampler_opaque_capture_descriptor_data_ext:
		PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
	#[doc = "Cached function pointer for `vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`."]
	#[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)"]
	pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
		PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
	#[doc = "Cached function pointer for `vkSetDeviceMemoryPriorityEXT`."]
	#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html)"]
	pub set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
	#[doc = "Cached function pointer for `vkWaitForPresent2KHR`."]
	#[doc(alias = "vkWaitForPresent2KHR")]
	#[doc = "\n\n[Khronos reference: `vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html)"]
	pub wait_for_present2_khr: PFN_vkWaitForPresent2KHR,
	#[doc = "Cached function pointer for `vkWaitForPresentKHR`."]
	#[doc(alias = "vkWaitForPresentKHR")]
	#[doc = "\n\n[Khronos reference: `vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html)"]
	pub wait_for_present_khr: PFN_vkWaitForPresentKHR,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkCreateBufferCollectionFUCHSIA`."]
	#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html)"]
	pub create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkSetBufferCollectionBufferConstraintsFUCHSIA`."]
	#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)"]
	pub set_buffer_collection_buffer_constraints_fuchsia:
		PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkSetBufferCollectionImageConstraintsFUCHSIA`."]
	#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html)"]
	pub set_buffer_collection_image_constraints_fuchsia:
		PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkDestroyBufferCollectionFUCHSIA`."]
	#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html)"]
	pub destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
	#[cfg(feature = "platform-fuchsia")]
	#[doc = "Cached function pointer for `vkGetBufferCollectionPropertiesFUCHSIA`."]
	#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
	#[doc = "\n\n[Khronos reference: `vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html)"]
	pub get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCreateCudaModuleNV`."]
	#[doc(alias = "vkCreateCudaModuleNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html)"]
	pub create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkGetCudaModuleCacheNV`."]
	#[doc(alias = "vkGetCudaModuleCacheNV")]
	#[doc = "\n\n[Khronos reference: `vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html)"]
	pub get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCreateCudaFunctionNV`."]
	#[doc(alias = "vkCreateCudaFunctionNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html)"]
	pub create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkDestroyCudaModuleNV`."]
	#[doc(alias = "vkDestroyCudaModuleNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html)"]
	pub destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkDestroyCudaFunctionNV`."]
	#[doc(alias = "vkDestroyCudaFunctionNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html)"]
	pub destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCmdCudaLaunchKernelNV`."]
	#[doc(alias = "vkCmdCudaLaunchKernelNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html)"]
	pub cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
	#[doc = "Cached function pointer for `vkCmdBeginRendering`."]
	#[doc(alias = "vkCmdBeginRendering")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html)"]
	pub cmd_begin_rendering: PFN_vkCmdBeginRendering,
	#[doc = "Cached function pointer for `vkCmdBeginRenderingKHR`."]
	#[doc(alias = "vkCmdBeginRenderingKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html)"]
	pub cmd_begin_rendering_khr: PFN_vkCmdBeginRenderingKHR,
	#[doc = "Cached function pointer for `vkCmdEndRendering`."]
	#[doc(alias = "vkCmdEndRendering")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html)"]
	pub cmd_end_rendering: PFN_vkCmdEndRendering,
	#[doc = "Cached function pointer for `vkCmdEndRendering2KHR`."]
	#[doc(alias = "vkCmdEndRendering2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html)"]
	pub cmd_end_rendering2_khr: PFN_vkCmdEndRendering2KHR,
	#[doc = "Cached function pointer for `vkCmdEndRendering2EXT`."]
	#[doc(alias = "vkCmdEndRendering2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html)"]
	pub cmd_end_rendering2_ext: PFN_vkCmdEndRendering2EXT,
	#[doc = "Cached function pointer for `vkCmdEndRenderingKHR`."]
	#[doc(alias = "vkCmdEndRenderingKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html)"]
	pub cmd_end_rendering_khr: PFN_vkCmdEndRenderingKHR,
	#[doc = "Cached function pointer for `vkGetDescriptorSetLayoutHostMappingInfoVALVE`."]
	#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)"]
	pub get_descriptor_set_layout_host_mapping_info_valve:
		PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
	#[doc = "Cached function pointer for `vkGetDescriptorSetHostMappingVALVE`."]
	#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
	#[doc = "\n\n[Khronos reference: `vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)"]
	pub get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
	#[doc = "Cached function pointer for `vkCreateMicromapEXT`."]
	#[doc(alias = "vkCreateMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html)"]
	pub create_micromap_ext: PFN_vkCreateMicromapEXT,
	#[doc = "Cached function pointer for `vkCmdBuildMicromapsEXT`."]
	#[doc(alias = "vkCmdBuildMicromapsEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html)"]
	pub cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
	#[doc = "Cached function pointer for `vkBuildMicromapsEXT`."]
	#[doc(alias = "vkBuildMicromapsEXT")]
	#[doc = "\n\n[Khronos reference: `vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html)"]
	pub build_micromaps_ext: PFN_vkBuildMicromapsEXT,
	#[doc = "Cached function pointer for `vkDestroyMicromapEXT`."]
	#[doc(alias = "vkDestroyMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)"]
	pub destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
	#[doc = "Cached function pointer for `vkCmdCopyMicromapEXT`."]
	#[doc(alias = "vkCmdCopyMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html)"]
	pub cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
	#[doc = "Cached function pointer for `vkCopyMicromapEXT`."]
	#[doc(alias = "vkCopyMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html)"]
	pub copy_micromap_ext: PFN_vkCopyMicromapEXT,
	#[doc = "Cached function pointer for `vkCmdCopyMicromapToMemoryEXT`."]
	#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html)"]
	pub cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
	#[doc = "Cached function pointer for `vkCopyMicromapToMemoryEXT`."]
	#[doc(alias = "vkCopyMicromapToMemoryEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html)"]
	pub copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryToMicromapEXT`."]
	#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html)"]
	pub cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
	#[doc = "Cached function pointer for `vkCopyMemoryToMicromapEXT`."]
	#[doc(alias = "vkCopyMemoryToMicromapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html)"]
	pub copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
	#[doc = "Cached function pointer for `vkCmdWriteMicromapsPropertiesEXT`."]
	#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html)"]
	pub cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
	#[doc = "Cached function pointer for `vkWriteMicromapsPropertiesEXT`."]
	#[doc(alias = "vkWriteMicromapsPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html)"]
	pub write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
	#[doc = "Cached function pointer for `vkGetDeviceMicromapCompatibilityEXT`."]
	#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html)"]
	pub get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
	#[doc = "Cached function pointer for `vkGetMicromapBuildSizesEXT`."]
	#[doc(alias = "vkGetMicromapBuildSizesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html)"]
	pub get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
	#[doc = "Cached function pointer for `vkGetShaderModuleIdentifierEXT`."]
	#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)"]
	pub get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
	#[doc = "Cached function pointer for `vkGetShaderModuleCreateInfoIdentifierEXT`."]
	#[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html)"]
	pub get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
	#[doc = "Cached function pointer for `vkGetImageSubresourceLayout2`."]
	#[doc(alias = "vkGetImageSubresourceLayout2")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)"]
	pub get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
	#[doc = "Cached function pointer for `vkGetImageSubresourceLayout2KHR`."]
	#[doc(alias = "vkGetImageSubresourceLayout2KHR")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html)"]
	pub get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2KHR,
	#[doc = "Cached function pointer for `vkGetImageSubresourceLayout2EXT`."]
	#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
	#[doc = "\n\n[Khronos reference: `vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html)"]
	pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2EXT,
	#[doc = "Cached function pointer for `vkGetPipelinePropertiesEXT`."]
	#[doc(alias = "vkGetPipelinePropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html)"]
	pub get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
	#[cfg(feature = "platform-metal")]
	#[doc = "Cached function pointer for `vkExportMetalObjectsEXT`."]
	#[doc(alias = "vkExportMetalObjectsEXT")]
	#[doc = "\n\n[Khronos reference: `vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html)"]
	pub export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
	#[doc = "Cached function pointer for `vkCmdBindTileMemoryQCOM`."]
	#[doc(alias = "vkCmdBindTileMemoryQCOM")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html)"]
	pub cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
	#[doc = "Cached function pointer for `vkGetFramebufferTilePropertiesQCOM`."]
	#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
	#[doc = "\n\n[Khronos reference: `vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)"]
	pub get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
	#[doc = "Cached function pointer for `vkGetDynamicRenderingTilePropertiesQCOM`."]
	#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
	#[doc = "\n\n[Khronos reference: `vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html)"]
	pub get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
	#[doc = "Cached function pointer for `vkCreateOpticalFlowSessionNV`."]
	#[doc(alias = "vkCreateOpticalFlowSessionNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html)"]
	pub create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
	#[doc = "Cached function pointer for `vkDestroyOpticalFlowSessionNV`."]
	#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html)"]
	pub destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
	#[doc = "Cached function pointer for `vkBindOpticalFlowSessionImageNV`."]
	#[doc(alias = "vkBindOpticalFlowSessionImageNV")]
	#[doc = "\n\n[Khronos reference: `vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html)"]
	pub bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
	#[doc = "Cached function pointer for `vkCmdOpticalFlowExecuteNV`."]
	#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html)"]
	pub cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
	#[doc = "Cached function pointer for `vkGetDeviceFaultInfoEXT`."]
	#[doc(alias = "vkGetDeviceFaultInfoEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html)"]
	pub get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
	#[doc = "Cached function pointer for `vkGetDeviceFaultReportsKHR`."]
	#[doc(alias = "vkGetDeviceFaultReportsKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html)"]
	pub get_device_fault_reports_khr: PFN_vkGetDeviceFaultReportsKHR,
	#[doc = "Cached function pointer for `vkGetDeviceFaultDebugInfoKHR`."]
	#[doc(alias = "vkGetDeviceFaultDebugInfoKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html)"]
	pub get_device_fault_debug_info_khr: PFN_vkGetDeviceFaultDebugInfoKHR,
	#[doc = "Cached function pointer for `vkCmdSetDepthBias2EXT`."]
	#[doc(alias = "vkCmdSetDepthBias2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html)"]
	pub cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
	#[doc = "Cached function pointer for `vkReleaseSwapchainImagesKHR`."]
	#[doc(alias = "vkReleaseSwapchainImagesKHR")]
	#[doc = "\n\n[Khronos reference: `vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html)"]
	pub release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
	#[doc = "Cached function pointer for `vkReleaseSwapchainImagesEXT`."]
	#[doc(alias = "vkReleaseSwapchainImagesEXT")]
	#[doc = "\n\n[Khronos reference: `vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html)"]
	pub release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesEXT,
	#[doc = "Cached function pointer for `vkGetDeviceImageSubresourceLayout`."]
	#[doc(alias = "vkGetDeviceImageSubresourceLayout")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html)"]
	pub get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
	#[doc = "Cached function pointer for `vkGetDeviceImageSubresourceLayoutKHR`."]
	#[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html)"]
	pub get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayoutKHR,
	#[doc = "Cached function pointer for `vkMapMemory2`."]
	#[doc(alias = "vkMapMemory2")]
	#[doc = "\n\n[Khronos reference: `vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html)"]
	pub map_memory2: PFN_vkMapMemory2,
	#[doc = "Cached function pointer for `vkMapMemory2KHR`."]
	#[doc(alias = "vkMapMemory2KHR")]
	#[doc = "\n\n[Khronos reference: `vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html)"]
	pub map_memory2_khr: PFN_vkMapMemory2KHR,
	#[doc = "Cached function pointer for `vkUnmapMemory2`."]
	#[doc(alias = "vkUnmapMemory2")]
	#[doc = "\n\n[Khronos reference: `vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html)"]
	pub unmap_memory2: PFN_vkUnmapMemory2,
	#[doc = "Cached function pointer for `vkUnmapMemory2KHR`."]
	#[doc(alias = "vkUnmapMemory2KHR")]
	#[doc = "\n\n[Khronos reference: `vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html)"]
	pub unmap_memory2_khr: PFN_vkUnmapMemory2KHR,
	#[doc = "Cached function pointer for `vkCreateShadersEXT`."]
	#[doc(alias = "vkCreateShadersEXT")]
	#[doc = "\n\n[Khronos reference: `vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html)"]
	pub create_shaders_ext: PFN_vkCreateShadersEXT,
	#[doc = "Cached function pointer for `vkDestroyShaderEXT`."]
	#[doc(alias = "vkDestroyShaderEXT")]
	#[doc = "\n\n[Khronos reference: `vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html)"]
	pub destroy_shader_ext: PFN_vkDestroyShaderEXT,
	#[doc = "Cached function pointer for `vkGetShaderBinaryDataEXT`."]
	#[doc(alias = "vkGetShaderBinaryDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html)"]
	pub get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
	#[doc = "Cached function pointer for `vkCmdBindShadersEXT`."]
	#[doc(alias = "vkCmdBindShadersEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html)"]
	pub cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
	#[doc = "Cached function pointer for `vkSetSwapchainPresentTimingQueueSizeEXT`."]
	#[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
	#[doc = "\n\n[Khronos reference: `vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html)"]
	pub set_swapchain_present_timing_queue_size_ext: PFN_vkSetSwapchainPresentTimingQueueSizeEXT,
	#[doc = "Cached function pointer for `vkGetSwapchainTimingPropertiesEXT`."]
	#[doc(alias = "vkGetSwapchainTimingPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html)"]
	pub get_swapchain_timing_properties_ext: PFN_vkGetSwapchainTimingPropertiesEXT,
	#[doc = "Cached function pointer for `vkGetSwapchainTimeDomainPropertiesEXT`."]
	#[doc(alias = "vkGetSwapchainTimeDomainPropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html)"]
	pub get_swapchain_time_domain_properties_ext: PFN_vkGetSwapchainTimeDomainPropertiesEXT,
	#[doc = "Cached function pointer for `vkGetPastPresentationTimingEXT`."]
	#[doc(alias = "vkGetPastPresentationTimingEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html)"]
	pub get_past_presentation_timing_ext: PFN_vkGetPastPresentationTimingEXT,
	#[cfg(feature = "platform-screen")]
	#[doc = "Cached function pointer for `vkGetScreenBufferPropertiesQNX`."]
	#[doc(alias = "vkGetScreenBufferPropertiesQNX")]
	#[doc = "\n\n[Khronos reference: `vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html)"]
	pub get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkGetExecutionGraphPipelineScratchSizeAMDX`."]
	#[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
	#[doc = "\n\n[Khronos reference: `vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html)"]
	pub get_execution_graph_pipeline_scratch_size_amdx:
		PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkGetExecutionGraphPipelineNodeIndexAMDX`."]
	#[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
	#[doc = "\n\n[Khronos reference: `vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html)"]
	pub get_execution_graph_pipeline_node_index_amdx: PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCreateExecutionGraphPipelinesAMDX`."]
	#[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
	#[doc = "\n\n[Khronos reference: `vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html)"]
	pub create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCmdInitializeGraphScratchMemoryAMDX`."]
	#[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
	#[doc = "\n\n[Khronos reference: `vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html)"]
	pub cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCmdDispatchGraphAMDX`."]
	#[doc(alias = "vkCmdDispatchGraphAMDX")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html)"]
	pub cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCmdDispatchGraphIndirectAMDX`."]
	#[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html)"]
	pub cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
	#[cfg(feature = "provisional")]
	#[doc = "Cached function pointer for `vkCmdDispatchGraphIndirectCountAMDX`."]
	#[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html)"]
	pub cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
	#[doc = "Cached function pointer for `vkCreateGpaSessionAMD`."]
	#[doc(alias = "vkCreateGpaSessionAMD")]
	#[doc = "\n\n[Khronos reference: `vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html)"]
	pub create_gpa_session_amd: PFN_vkCreateGpaSessionAMD,
	#[doc = "Cached function pointer for `vkDestroyGpaSessionAMD`."]
	#[doc(alias = "vkDestroyGpaSessionAMD")]
	#[doc = "\n\n[Khronos reference: `vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html)"]
	pub destroy_gpa_session_amd: PFN_vkDestroyGpaSessionAMD,
	#[doc = "Cached function pointer for `vkSetGpaDeviceClockModeAMD`."]
	#[doc(alias = "vkSetGpaDeviceClockModeAMD")]
	#[doc = "\n\n[Khronos reference: `vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html)"]
	pub set_gpa_device_clock_mode_amd: PFN_vkSetGpaDeviceClockModeAMD,
	#[doc = "Cached function pointer for `vkGetGpaDeviceClockInfoAMD`."]
	#[doc(alias = "vkGetGpaDeviceClockInfoAMD")]
	#[doc = "\n\n[Khronos reference: `vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html)"]
	pub get_gpa_device_clock_info_amd: PFN_vkGetGpaDeviceClockInfoAMD,
	#[doc = "Cached function pointer for `vkCmdBeginGpaSessionAMD`."]
	#[doc(alias = "vkCmdBeginGpaSessionAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html)"]
	pub cmd_begin_gpa_session_amd: PFN_vkCmdBeginGpaSessionAMD,
	#[doc = "Cached function pointer for `vkCmdEndGpaSessionAMD`."]
	#[doc(alias = "vkCmdEndGpaSessionAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html)"]
	pub cmd_end_gpa_session_amd: PFN_vkCmdEndGpaSessionAMD,
	#[doc = "Cached function pointer for `vkCmdBeginGpaSampleAMD`."]
	#[doc(alias = "vkCmdBeginGpaSampleAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html)"]
	pub cmd_begin_gpa_sample_amd: PFN_vkCmdBeginGpaSampleAMD,
	#[doc = "Cached function pointer for `vkCmdEndGpaSampleAMD`."]
	#[doc(alias = "vkCmdEndGpaSampleAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html)"]
	pub cmd_end_gpa_sample_amd: PFN_vkCmdEndGpaSampleAMD,
	#[doc = "Cached function pointer for `vkGetGpaSessionStatusAMD`."]
	#[doc(alias = "vkGetGpaSessionStatusAMD")]
	#[doc = "\n\n[Khronos reference: `vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html)"]
	pub get_gpa_session_status_amd: PFN_vkGetGpaSessionStatusAMD,
	#[doc = "Cached function pointer for `vkGetGpaSessionResultsAMD`."]
	#[doc(alias = "vkGetGpaSessionResultsAMD")]
	#[doc = "\n\n[Khronos reference: `vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html)"]
	pub get_gpa_session_results_amd: PFN_vkGetGpaSessionResultsAMD,
	#[doc = "Cached function pointer for `vkResetGpaSessionAMD`."]
	#[doc(alias = "vkResetGpaSessionAMD")]
	#[doc = "\n\n[Khronos reference: `vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html)"]
	pub reset_gpa_session_amd: PFN_vkResetGpaSessionAMD,
	#[doc = "Cached function pointer for `vkCmdCopyGpaSessionResultsAMD`."]
	#[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html)"]
	pub cmd_copy_gpa_session_results_amd: PFN_vkCmdCopyGpaSessionResultsAMD,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorSets2`."]
	#[doc(alias = "vkCmdBindDescriptorSets2")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html)"]
	pub cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorSets2KHR`."]
	#[doc(alias = "vkCmdBindDescriptorSets2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html)"]
	pub cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2KHR,
	#[doc = "Cached function pointer for `vkCmdPushConstants2`."]
	#[doc(alias = "vkCmdPushConstants2")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html)"]
	pub cmd_push_constants2: PFN_vkCmdPushConstants2,
	#[doc = "Cached function pointer for `vkCmdPushConstants2KHR`."]
	#[doc(alias = "vkCmdPushConstants2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html)"]
	pub cmd_push_constants2_khr: PFN_vkCmdPushConstants2KHR,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSet2`."]
	#[doc(alias = "vkCmdPushDescriptorSet2")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html)"]
	pub cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSet2KHR`."]
	#[doc(alias = "vkCmdPushDescriptorSet2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html)"]
	pub cmd_push_descriptor_set2_khr: PFN_vkCmdPushDescriptorSet2KHR,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSetWithTemplate2`."]
	#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html)"]
	pub cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
	#[doc = "Cached function pointer for `vkCmdPushDescriptorSetWithTemplate2KHR`."]
	#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html)"]
	pub cmd_push_descriptor_set_with_template2_khr: PFN_vkCmdPushDescriptorSetWithTemplate2KHR,
	#[doc = "Cached function pointer for `vkCmdSetDescriptorBufferOffsets2EXT`."]
	#[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html)"]
	pub cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
	#[doc = "Cached function pointer for `vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`."]
	#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html)"]
	pub cmd_bind_descriptor_buffer_embedded_samplers2_ext:
		PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
	#[doc = "Cached function pointer for `vkSetLatencySleepModeNV`."]
	#[doc(alias = "vkSetLatencySleepModeNV")]
	#[doc = "\n\n[Khronos reference: `vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html)"]
	pub set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
	#[doc = "Cached function pointer for `vkLatencySleepNV`."]
	#[doc(alias = "vkLatencySleepNV")]
	#[doc = "\n\n[Khronos reference: `vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html)"]
	pub latency_sleep_nv: PFN_vkLatencySleepNV,
	#[doc = "Cached function pointer for `vkSetLatencyMarkerNV`."]
	#[doc(alias = "vkSetLatencyMarkerNV")]
	#[doc = "\n\n[Khronos reference: `vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html)"]
	pub set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
	#[doc = "Cached function pointer for `vkGetLatencyTimingsNV`."]
	#[doc(alias = "vkGetLatencyTimingsNV")]
	#[doc = "\n\n[Khronos reference: `vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html)"]
	pub get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
	#[doc = "Cached function pointer for `vkQueueNotifyOutOfBandNV`."]
	#[doc(alias = "vkQueueNotifyOutOfBandNV")]
	#[doc = "\n\n[Khronos reference: `vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html)"]
	pub queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
	#[doc = "Cached function pointer for `vkSetLatencySleepModeLegacyNV`."]
	#[doc(alias = "vkSetLatencySleepModeLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkSetLatencySleepModeLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html)"]
	pub set_latency_sleep_mode_legacy_nv: PFN_vkSetLatencySleepModeLegacyNV,
	#[doc = "Cached function pointer for `vkLatencySleepLegacyNV`."]
	#[doc(alias = "vkLatencySleepLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkLatencySleepLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html)"]
	pub latency_sleep_legacy_nv: PFN_vkLatencySleepLegacyNV,
	#[doc = "Cached function pointer for `vkSetLatencyMarkerLegacyNV`."]
	#[doc(alias = "vkSetLatencyMarkerLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkSetLatencyMarkerLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html)"]
	pub set_latency_marker_legacy_nv: PFN_vkSetLatencyMarkerLegacyNV,
	#[doc = "Cached function pointer for `vkGetLatencyTimingsLegacyNV`."]
	#[doc(alias = "vkGetLatencyTimingsLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkGetLatencyTimingsLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html)"]
	pub get_latency_timings_legacy_nv: PFN_vkGetLatencyTimingsLegacyNV,
	#[doc = "Cached function pointer for `vkQueueNotifyOutOfBandLegacyNV`."]
	#[doc(alias = "vkQueueNotifyOutOfBandLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkQueueNotifyOutOfBandLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html)"]
	pub queue_notify_out_of_band_legacy_nv: PFN_vkQueueNotifyOutOfBandLegacyNV,
	#[doc = "Cached function pointer for `vkGetSleepStatusLegacyNV`."]
	#[doc(alias = "vkGetSleepStatusLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkGetSleepStatusLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html)"]
	pub get_sleep_status_legacy_nv: PFN_vkGetSleepStatusLegacyNV,
	#[doc = "Cached function pointer for `vkShutdownLatencyDeviceLegacyNV`."]
	#[doc(alias = "vkShutdownLatencyDeviceLegacyNV")]
	#[doc = "\n\n[Khronos reference: `vkShutdownLatencyDeviceLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html)"]
	pub shutdown_latency_device_legacy_nv: PFN_vkShutdownLatencyDeviceLegacyNV,
	#[doc = "Cached function pointer for `vkCmdSetRenderingAttachmentLocations`."]
	#[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html)"]
	pub cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
	#[doc = "Cached function pointer for `vkCmdSetRenderingAttachmentLocationsKHR`."]
	#[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html)"]
	pub cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocationsKHR,
	#[doc = "Cached function pointer for `vkCmdSetRenderingInputAttachmentIndices`."]
	#[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html)"]
	pub cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
	#[doc = "Cached function pointer for `vkCmdSetRenderingInputAttachmentIndicesKHR`."]
	#[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html)"]
	pub cmd_set_rendering_input_attachment_indices_khr:
		PFN_vkCmdSetRenderingInputAttachmentIndicesKHR,
	#[doc = "Cached function pointer for `vkCmdSetDepthClampRangeEXT`."]
	#[doc(alias = "vkCmdSetDepthClampRangeEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html)"]
	pub cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
	#[cfg(feature = "platform-metal")]
	#[doc = "Cached function pointer for `vkGetMemoryMetalHandleEXT`."]
	#[doc(alias = "vkGetMemoryMetalHandleEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html)"]
	pub get_memory_metal_handle_ext: PFN_vkGetMemoryMetalHandleEXT,
	#[cfg(feature = "platform-metal")]
	#[doc = "Cached function pointer for `vkGetMemoryMetalHandlePropertiesEXT`."]
	#[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html)"]
	pub get_memory_metal_handle_properties_ext: PFN_vkGetMemoryMetalHandlePropertiesEXT,
	#[doc = "Cached function pointer for `vkConvertCooperativeVectorMatrixNV`."]
	#[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
	#[doc = "\n\n[Khronos reference: `vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html)"]
	pub convert_cooperative_vector_matrix_nv: PFN_vkConvertCooperativeVectorMatrixNV,
	#[doc = "Cached function pointer for `vkCmdConvertCooperativeVectorMatrixNV`."]
	#[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html)"]
	pub cmd_convert_cooperative_vector_matrix_nv: PFN_vkCmdConvertCooperativeVectorMatrixNV,
	#[doc = "Cached function pointer for `vkCmdDispatchTileQCOM`."]
	#[doc(alias = "vkCmdDispatchTileQCOM")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html)"]
	pub cmd_dispatch_tile_qcom: PFN_vkCmdDispatchTileQCOM,
	#[doc = "Cached function pointer for `vkCmdBeginPerTileExecutionQCOM`."]
	#[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html)"]
	pub cmd_begin_per_tile_execution_qcom: PFN_vkCmdBeginPerTileExecutionQCOM,
	#[doc = "Cached function pointer for `vkCmdEndPerTileExecutionQCOM`."]
	#[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html)"]
	pub cmd_end_per_tile_execution_qcom: PFN_vkCmdEndPerTileExecutionQCOM,
	#[doc = "Cached function pointer for `vkCreateExternalComputeQueueNV`."]
	#[doc(alias = "vkCreateExternalComputeQueueNV")]
	#[doc = "\n\n[Khronos reference: `vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html)"]
	pub create_external_compute_queue_nv: PFN_vkCreateExternalComputeQueueNV,
	#[doc = "Cached function pointer for `vkDestroyExternalComputeQueueNV`."]
	#[doc(alias = "vkDestroyExternalComputeQueueNV")]
	#[doc = "\n\n[Khronos reference: `vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html)"]
	pub destroy_external_compute_queue_nv: PFN_vkDestroyExternalComputeQueueNV,
	#[doc = "Cached function pointer for `vkGetExternalComputeQueueDataNV`."]
	#[doc(alias = "vkGetExternalComputeQueueDataNV")]
	#[doc = "\n\n[Khronos reference: `vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html)"]
	pub get_external_compute_queue_data_nv: PFN_vkGetExternalComputeQueueDataNV,
	#[doc = "Cached function pointer for `vkCreateShaderInstrumentationARM`."]
	#[doc(alias = "vkCreateShaderInstrumentationARM")]
	#[doc = "\n\n[Khronos reference: `vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html)"]
	pub create_shader_instrumentation_arm: PFN_vkCreateShaderInstrumentationARM,
	#[doc = "Cached function pointer for `vkDestroyShaderInstrumentationARM`."]
	#[doc(alias = "vkDestroyShaderInstrumentationARM")]
	#[doc = "\n\n[Khronos reference: `vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html)"]
	pub destroy_shader_instrumentation_arm: PFN_vkDestroyShaderInstrumentationARM,
	#[doc = "Cached function pointer for `vkCmdBeginShaderInstrumentationARM`."]
	#[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html)"]
	pub cmd_begin_shader_instrumentation_arm: PFN_vkCmdBeginShaderInstrumentationARM,
	#[doc = "Cached function pointer for `vkCmdEndShaderInstrumentationARM`."]
	#[doc(alias = "vkCmdEndShaderInstrumentationARM")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html)"]
	pub cmd_end_shader_instrumentation_arm: PFN_vkCmdEndShaderInstrumentationARM,
	#[doc = "Cached function pointer for `vkGetShaderInstrumentationValuesARM`."]
	#[doc(alias = "vkGetShaderInstrumentationValuesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html)"]
	pub get_shader_instrumentation_values_arm: PFN_vkGetShaderInstrumentationValuesARM,
	#[doc = "Cached function pointer for `vkClearShaderInstrumentationMetricsARM`."]
	#[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
	#[doc = "\n\n[Khronos reference: `vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html)"]
	pub clear_shader_instrumentation_metrics_arm: PFN_vkClearShaderInstrumentationMetricsARM,
	#[doc = "Cached function pointer for `vkCreateTensorARM`."]
	#[doc(alias = "vkCreateTensorARM")]
	#[doc = "\n\n[Khronos reference: `vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html)"]
	pub create_tensor_arm: PFN_vkCreateTensorARM,
	#[doc = "Cached function pointer for `vkDestroyTensorARM`."]
	#[doc(alias = "vkDestroyTensorARM")]
	#[doc = "\n\n[Khronos reference: `vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html)"]
	pub destroy_tensor_arm: PFN_vkDestroyTensorARM,
	#[doc = "Cached function pointer for `vkCreateTensorViewARM`."]
	#[doc(alias = "vkCreateTensorViewARM")]
	#[doc = "\n\n[Khronos reference: `vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html)"]
	pub create_tensor_view_arm: PFN_vkCreateTensorViewARM,
	#[doc = "Cached function pointer for `vkDestroyTensorViewARM`."]
	#[doc(alias = "vkDestroyTensorViewARM")]
	#[doc = "\n\n[Khronos reference: `vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html)"]
	pub destroy_tensor_view_arm: PFN_vkDestroyTensorViewARM,
	#[doc = "Cached function pointer for `vkGetTensorMemoryRequirementsARM`."]
	#[doc(alias = "vkGetTensorMemoryRequirementsARM")]
	#[doc = "\n\n[Khronos reference: `vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html)"]
	pub get_tensor_memory_requirements_arm: PFN_vkGetTensorMemoryRequirementsARM,
	#[doc = "Cached function pointer for `vkBindTensorMemoryARM`."]
	#[doc(alias = "vkBindTensorMemoryARM")]
	#[doc = "\n\n[Khronos reference: `vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html)"]
	pub bind_tensor_memory_arm: PFN_vkBindTensorMemoryARM,
	#[doc = "Cached function pointer for `vkGetDeviceTensorMemoryRequirementsARM`."]
	#[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
	#[doc = "\n\n[Khronos reference: `vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html)"]
	pub get_device_tensor_memory_requirements_arm: PFN_vkGetDeviceTensorMemoryRequirementsARM,
	#[doc = "Cached function pointer for `vkCmdCopyTensorARM`."]
	#[doc(alias = "vkCmdCopyTensorARM")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html)"]
	pub cmd_copy_tensor_arm: PFN_vkCmdCopyTensorARM,
	#[doc = "Cached function pointer for `vkGetTensorOpaqueCaptureDescriptorDataARM`."]
	#[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
	#[doc = "\n\n[Khronos reference: `vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html)"]
	pub get_tensor_opaque_capture_descriptor_data_arm: PFN_vkGetTensorOpaqueCaptureDescriptorDataARM,
	#[doc = "Cached function pointer for `vkGetTensorViewOpaqueCaptureDescriptorDataARM`."]
	#[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
	#[doc = "\n\n[Khronos reference: `vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html)"]
	pub get_tensor_view_opaque_capture_descriptor_data_arm:
		PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM,
	#[doc = "Cached function pointer for `vkCreateDataGraphPipelinesARM`."]
	#[doc(alias = "vkCreateDataGraphPipelinesARM")]
	#[doc = "\n\n[Khronos reference: `vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html)"]
	pub create_data_graph_pipelines_arm: PFN_vkCreateDataGraphPipelinesARM,
	#[doc = "Cached function pointer for `vkCreateDataGraphPipelineSessionARM`."]
	#[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
	#[doc = "\n\n[Khronos reference: `vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html)"]
	pub create_data_graph_pipeline_session_arm: PFN_vkCreateDataGraphPipelineSessionARM,
	#[doc = "Cached function pointer for `vkGetDataGraphPipelineSessionBindPointRequirementsARM`."]
	#[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
	#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html)"]
	pub get_data_graph_pipeline_session_bind_point_requirements_arm:
		PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM,
	#[doc = "Cached function pointer for `vkGetDataGraphPipelineSessionMemoryRequirementsARM`."]
	#[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
	#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html)"]
	pub get_data_graph_pipeline_session_memory_requirements_arm:
		PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM,
	#[doc = "Cached function pointer for `vkBindDataGraphPipelineSessionMemoryARM`."]
	#[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
	#[doc = "\n\n[Khronos reference: `vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html)"]
	pub bind_data_graph_pipeline_session_memory_arm: PFN_vkBindDataGraphPipelineSessionMemoryARM,
	#[doc = "Cached function pointer for `vkDestroyDataGraphPipelineSessionARM`."]
	#[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
	#[doc = "\n\n[Khronos reference: `vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html)"]
	pub destroy_data_graph_pipeline_session_arm: PFN_vkDestroyDataGraphPipelineSessionARM,
	#[doc = "Cached function pointer for `vkCmdDispatchDataGraphARM`."]
	#[doc(alias = "vkCmdDispatchDataGraphARM")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html)"]
	pub cmd_dispatch_data_graph_arm: PFN_vkCmdDispatchDataGraphARM,
	#[doc = "Cached function pointer for `vkGetDataGraphPipelineAvailablePropertiesARM`."]
	#[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html)"]
	pub get_data_graph_pipeline_available_properties_arm:
		PFN_vkGetDataGraphPipelineAvailablePropertiesARM,
	#[doc = "Cached function pointer for `vkGetDataGraphPipelinePropertiesARM`."]
	#[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
	#[doc = "\n\n[Khronos reference: `vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html)"]
	pub get_data_graph_pipeline_properties_arm: PFN_vkGetDataGraphPipelinePropertiesARM,
	#[cfg(feature = "platform-ohos")]
	#[doc = "Cached function pointer for `vkGetNativeBufferPropertiesOHOS`."]
	#[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
	#[doc = "\n\n[Khronos reference: `vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html)"]
	pub get_native_buffer_properties_ohos: PFN_vkGetNativeBufferPropertiesOHOS,
	#[cfg(feature = "platform-ohos")]
	#[doc = "Cached function pointer for `vkGetMemoryNativeBufferOHOS`."]
	#[doc(alias = "vkGetMemoryNativeBufferOHOS")]
	#[doc = "\n\n[Khronos reference: `vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html)"]
	pub get_memory_native_buffer_ohos: PFN_vkGetMemoryNativeBufferOHOS,
	#[doc = "Cached function pointer for `vkQueueSetPerfHintQCOM`."]
	#[doc(alias = "vkQueueSetPerfHintQCOM")]
	#[doc = "\n\n[Khronos reference: `vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html)"]
	pub queue_set_perf_hint_qcom: PFN_vkQueueSetPerfHintQCOM,
	#[doc = "Cached function pointer for `vkCmdSetComputeOccupancyPriorityNV`."]
	#[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html)"]
	pub cmd_set_compute_occupancy_priority_nv: PFN_vkCmdSetComputeOccupancyPriorityNV,
	#[doc = "Cached function pointer for `vkWriteSamplerDescriptorsEXT`."]
	#[doc(alias = "vkWriteSamplerDescriptorsEXT")]
	#[doc = "\n\n[Khronos reference: `vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html)"]
	pub write_sampler_descriptors_ext: PFN_vkWriteSamplerDescriptorsEXT,
	#[doc = "Cached function pointer for `vkWriteResourceDescriptorsEXT`."]
	#[doc(alias = "vkWriteResourceDescriptorsEXT")]
	#[doc = "\n\n[Khronos reference: `vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html)"]
	pub write_resource_descriptors_ext: PFN_vkWriteResourceDescriptorsEXT,
	#[doc = "Cached function pointer for `vkCmdBindSamplerHeapEXT`."]
	#[doc(alias = "vkCmdBindSamplerHeapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html)"]
	pub cmd_bind_sampler_heap_ext: PFN_vkCmdBindSamplerHeapEXT,
	#[doc = "Cached function pointer for `vkCmdBindResourceHeapEXT`."]
	#[doc(alias = "vkCmdBindResourceHeapEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html)"]
	pub cmd_bind_resource_heap_ext: PFN_vkCmdBindResourceHeapEXT,
	#[doc = "Cached function pointer for `vkCmdPushDataEXT`."]
	#[doc(alias = "vkCmdPushDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html)"]
	pub cmd_push_data_ext: PFN_vkCmdPushDataEXT,
	#[doc = "Cached function pointer for `vkRegisterCustomBorderColorEXT`."]
	#[doc(alias = "vkRegisterCustomBorderColorEXT")]
	#[doc = "\n\n[Khronos reference: `vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html)"]
	pub register_custom_border_color_ext: PFN_vkRegisterCustomBorderColorEXT,
	#[doc = "Cached function pointer for `vkUnregisterCustomBorderColorEXT`."]
	#[doc(alias = "vkUnregisterCustomBorderColorEXT")]
	#[doc = "\n\n[Khronos reference: `vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html)"]
	pub unregister_custom_border_color_ext: PFN_vkUnregisterCustomBorderColorEXT,
	#[doc = "Cached function pointer for `vkGetImageOpaqueCaptureDataEXT`."]
	#[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
	#[doc = "\n\n[Khronos reference: `vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html)"]
	pub get_image_opaque_capture_data_ext: PFN_vkGetImageOpaqueCaptureDataEXT,
	#[doc = "Cached function pointer for `vkGetTensorOpaqueCaptureDataARM`."]
	#[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
	#[doc = "\n\n[Khronos reference: `vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html)"]
	pub get_tensor_opaque_capture_data_arm: PFN_vkGetTensorOpaqueCaptureDataARM,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryKHR`."]
	#[doc(alias = "vkCmdCopyMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html)"]
	pub cmd_copy_memory_khr: PFN_vkCmdCopyMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdCopyMemoryToImageKHR`."]
	#[doc(alias = "vkCmdCopyMemoryToImageKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html)"]
	pub cmd_copy_memory_to_image_khr: PFN_vkCmdCopyMemoryToImageKHR,
	#[doc = "Cached function pointer for `vkCmdCopyImageToMemoryKHR`."]
	#[doc(alias = "vkCmdCopyImageToMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html)"]
	pub cmd_copy_image_to_memory_khr: PFN_vkCmdCopyImageToMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdUpdateMemoryKHR`."]
	#[doc(alias = "vkCmdUpdateMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html)"]
	pub cmd_update_memory_khr: PFN_vkCmdUpdateMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdFillMemoryKHR`."]
	#[doc(alias = "vkCmdFillMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html)"]
	pub cmd_fill_memory_khr: PFN_vkCmdFillMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdCopyQueryPoolResultsToMemoryKHR`."]
	#[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html)"]
	pub cmd_copy_query_pool_results_to_memory_khr: PFN_vkCmdCopyQueryPoolResultsToMemoryKHR,
	#[doc = "Cached function pointer for `vkCmdBeginConditionalRendering2EXT`."]
	#[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html)"]
	pub cmd_begin_conditional_rendering2_ext: PFN_vkCmdBeginConditionalRendering2EXT,
	#[doc = "Cached function pointer for `vkCmdBindTransformFeedbackBuffers2EXT`."]
	#[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html)"]
	pub cmd_bind_transform_feedback_buffers2_ext: PFN_vkCmdBindTransformFeedbackBuffers2EXT,
	#[doc = "Cached function pointer for `vkCmdBeginTransformFeedback2EXT`."]
	#[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html)"]
	pub cmd_begin_transform_feedback2_ext: PFN_vkCmdBeginTransformFeedback2EXT,
	#[doc = "Cached function pointer for `vkCmdEndTransformFeedback2EXT`."]
	#[doc(alias = "vkCmdEndTransformFeedback2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html)"]
	pub cmd_end_transform_feedback2_ext: PFN_vkCmdEndTransformFeedback2EXT,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectByteCount2EXT`."]
	#[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html)"]
	pub cmd_draw_indirect_byte_count2_ext: PFN_vkCmdDrawIndirectByteCount2EXT,
	#[doc = "Cached function pointer for `vkCmdWriteMarkerToMemoryAMD`."]
	#[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
	#[doc = "\n\n[Khronos reference: `vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html)"]
	pub cmd_write_marker_to_memory_amd: PFN_vkCmdWriteMarkerToMemoryAMD,
	#[doc = "Cached function pointer for `vkCmdBindIndexBuffer3KHR`."]
	#[doc(alias = "vkCmdBindIndexBuffer3KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html)"]
	pub cmd_bind_index_buffer3_khr: PFN_vkCmdBindIndexBuffer3KHR,
	#[doc = "Cached function pointer for `vkCmdBindVertexBuffers3KHR`."]
	#[doc(alias = "vkCmdBindVertexBuffers3KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html)"]
	pub cmd_bind_vertex_buffers3_khr: PFN_vkCmdBindVertexBuffers3KHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndirect2KHR`."]
	#[doc(alias = "vkCmdDrawIndirect2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html)"]
	pub cmd_draw_indirect2_khr: PFN_vkCmdDrawIndirect2KHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirect2KHR`."]
	#[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html)"]
	pub cmd_draw_indexed_indirect2_khr: PFN_vkCmdDrawIndexedIndirect2KHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndirectCount2KHR`."]
	#[doc(alias = "vkCmdDrawIndirectCount2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html)"]
	pub cmd_draw_indirect_count2_khr: PFN_vkCmdDrawIndirectCount2KHR,
	#[doc = "Cached function pointer for `vkCmdDrawIndexedIndirectCount2KHR`."]
	#[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html)"]
	pub cmd_draw_indexed_indirect_count2_khr: PFN_vkCmdDrawIndexedIndirectCount2KHR,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirect2EXT`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html)"]
	pub cmd_draw_mesh_tasks_indirect2_ext: PFN_vkCmdDrawMeshTasksIndirect2EXT,
	#[doc = "Cached function pointer for `vkCmdDrawMeshTasksIndirectCount2EXT`."]
	#[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
	#[doc = "\n\n[Khronos reference: `vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html)"]
	pub cmd_draw_mesh_tasks_indirect_count2_ext: PFN_vkCmdDrawMeshTasksIndirectCount2EXT,
	#[doc = "Cached function pointer for `vkCmdDispatchIndirect2KHR`."]
	#[doc(alias = "vkCmdDispatchIndirect2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html)"]
	pub cmd_dispatch_indirect2_khr: PFN_vkCmdDispatchIndirect2KHR,
	#[doc = "Cached function pointer for `vkCreateAccelerationStructure2KHR`."]
	#[doc(alias = "vkCreateAccelerationStructure2KHR")]
	#[doc = "\n\n[Khronos reference: `vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html)"]
	pub create_acceleration_structure2_khr: PFN_vkCreateAccelerationStructure2KHR,
	#[doc = "Cached function pointer for `vkCmdSetDispatchParametersARM`."]
	#[doc(alias = "vkCmdSetDispatchParametersARM")]
	#[doc = "\n\n[Khronos reference: `vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html)"]
	pub cmd_set_dispatch_parameters_arm: PFN_vkCmdSetDispatchParametersARM,
}

impl FnEntry {
	/// Load all commands for this dispatch level.
	///
	/// # Safety
	/// `load` must implement the Vulkan proc-addr contract and remain callable
	/// for the lifetime of this table and all derived raw function pointers.
	pub unsafe fn new_from_resolver(
		load: unsafe extern "system" fn(InstanceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction,
	) -> Self {
		macro_rules! resolve {
			($name:literal, $ty:ty) => {
				unsafe {
					std::mem::transmute::<PFN_vkVoidFunction, $ty>(load(InstanceHandle::NULL, $name.as_ptr()))
				}
			};
		}
		Self {
			create_instance: resolve!(c"vkCreateInstance", PFN_vkCreateInstance),
			get_instance_proc_addr: resolve!(c"vkGetInstanceProcAddr", PFN_vkGetInstanceProcAddr),
			enumerate_instance_version: resolve!(
				c"vkEnumerateInstanceVersion",
				PFN_vkEnumerateInstanceVersion
			),
			enumerate_instance_layer_properties: resolve!(
				c"vkEnumerateInstanceLayerProperties",
				PFN_vkEnumerateInstanceLayerProperties
			),
			enumerate_instance_extension_properties: resolve!(
				c"vkEnumerateInstanceExtensionProperties",
				PFN_vkEnumerateInstanceExtensionProperties
			),
		}
	}
}

impl FnInstance {
	/// Load all commands for this dispatch level.
	///
	/// # Safety
	/// `load` must implement the Vulkan proc-addr contract and remain callable
	/// for the lifetime of this table and all derived raw function pointers.
	pub unsafe fn new_from_resolver(
		load: unsafe extern "system" fn(InstanceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction,
		instance: InstanceHandle,
	) -> Self {
		macro_rules! resolve {
			($name:literal, $ty:ty) => {
				unsafe { std::mem::transmute::<PFN_vkVoidFunction, $ty>(load(instance, $name.as_ptr())) }
			};
		}
		Self {
			destroy_instance: resolve!(c"vkDestroyInstance", PFN_vkDestroyInstance),
			enumerate_physical_devices: resolve!(
				c"vkEnumeratePhysicalDevices",
				PFN_vkEnumeratePhysicalDevices
			),
			get_device_proc_addr: resolve!(c"vkGetDeviceProcAddr", PFN_vkGetDeviceProcAddr),
			get_physical_device_properties: resolve!(
				c"vkGetPhysicalDeviceProperties",
				PFN_vkGetPhysicalDeviceProperties
			),
			get_physical_device_queue_family_properties: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyProperties",
				PFN_vkGetPhysicalDeviceQueueFamilyProperties
			),
			get_physical_device_memory_properties: resolve!(
				c"vkGetPhysicalDeviceMemoryProperties",
				PFN_vkGetPhysicalDeviceMemoryProperties
			),
			get_physical_device_features: resolve!(
				c"vkGetPhysicalDeviceFeatures",
				PFN_vkGetPhysicalDeviceFeatures
			),
			get_physical_device_format_properties: resolve!(
				c"vkGetPhysicalDeviceFormatProperties",
				PFN_vkGetPhysicalDeviceFormatProperties
			),
			get_physical_device_image_format_properties: resolve!(
				c"vkGetPhysicalDeviceImageFormatProperties",
				PFN_vkGetPhysicalDeviceImageFormatProperties
			),
			create_device: resolve!(c"vkCreateDevice", PFN_vkCreateDevice),
			enumerate_device_layer_properties: resolve!(
				c"vkEnumerateDeviceLayerProperties",
				PFN_vkEnumerateDeviceLayerProperties
			),
			enumerate_device_extension_properties: resolve!(
				c"vkEnumerateDeviceExtensionProperties",
				PFN_vkEnumerateDeviceExtensionProperties
			),
			get_physical_device_sparse_image_format_properties: resolve!(
				c"vkGetPhysicalDeviceSparseImageFormatProperties",
				PFN_vkGetPhysicalDeviceSparseImageFormatProperties
			),
			#[cfg(feature = "platform-android")]
			create_android_surface_khr: resolve!(
				c"vkCreateAndroidSurfaceKHR",
				PFN_vkCreateAndroidSurfaceKHR
			),
			#[cfg(feature = "platform-ohos")]
			create_surface_ohos: resolve!(c"vkCreateSurfaceOHOS", PFN_vkCreateSurfaceOHOS),
			get_physical_device_display_properties_khr: resolve!(
				c"vkGetPhysicalDeviceDisplayPropertiesKHR",
				PFN_vkGetPhysicalDeviceDisplayPropertiesKHR
			),
			get_physical_device_display_plane_properties_khr: resolve!(
				c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
				PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR
			),
			get_display_plane_supported_displays_khr: resolve!(
				c"vkGetDisplayPlaneSupportedDisplaysKHR",
				PFN_vkGetDisplayPlaneSupportedDisplaysKHR
			),
			get_display_mode_properties_khr: resolve!(
				c"vkGetDisplayModePropertiesKHR",
				PFN_vkGetDisplayModePropertiesKHR
			),
			create_display_mode_khr: resolve!(c"vkCreateDisplayModeKHR", PFN_vkCreateDisplayModeKHR),
			get_display_plane_capabilities_khr: resolve!(
				c"vkGetDisplayPlaneCapabilitiesKHR",
				PFN_vkGetDisplayPlaneCapabilitiesKHR
			),
			create_display_plane_surface_khr: resolve!(
				c"vkCreateDisplayPlaneSurfaceKHR",
				PFN_vkCreateDisplayPlaneSurfaceKHR
			),
			destroy_surface_khr: resolve!(c"vkDestroySurfaceKHR", PFN_vkDestroySurfaceKHR),
			get_physical_device_surface_support_khr: resolve!(
				c"vkGetPhysicalDeviceSurfaceSupportKHR",
				PFN_vkGetPhysicalDeviceSurfaceSupportKHR
			),
			get_physical_device_surface_capabilities_khr: resolve!(
				c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
				PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR
			),
			get_physical_device_surface_formats_khr: resolve!(
				c"vkGetPhysicalDeviceSurfaceFormatsKHR",
				PFN_vkGetPhysicalDeviceSurfaceFormatsKHR
			),
			get_physical_device_surface_present_modes_khr: resolve!(
				c"vkGetPhysicalDeviceSurfacePresentModesKHR",
				PFN_vkGetPhysicalDeviceSurfacePresentModesKHR
			),
			#[cfg(feature = "platform-vi")]
			create_vi_surface_nn: resolve!(c"vkCreateViSurfaceNN", PFN_vkCreateViSurfaceNN),
			#[cfg(feature = "platform-wayland")]
			create_wayland_surface_khr: resolve!(
				c"vkCreateWaylandSurfaceKHR",
				PFN_vkCreateWaylandSurfaceKHR
			),
			#[cfg(feature = "platform-wayland")]
			get_physical_device_wayland_presentation_support_khr: resolve!(
				c"vkGetPhysicalDeviceWaylandPresentationSupportKHR",
				PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR
			),
			#[cfg(feature = "platform-ubm")]
			create_ubm_surface_sec: resolve!(c"vkCreateUbmSurfaceSEC", PFN_vkCreateUbmSurfaceSEC),
			#[cfg(feature = "platform-ubm")]
			get_physical_device_ubm_presentation_support_sec: resolve!(
				c"vkGetPhysicalDeviceUbmPresentationSupportSEC",
				PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC
			),
			#[cfg(feature = "platform-win32")]
			create_win32_surface_khr: resolve!(c"vkCreateWin32SurfaceKHR", PFN_vkCreateWin32SurfaceKHR),
			#[cfg(feature = "platform-win32")]
			get_physical_device_win32_presentation_support_khr: resolve!(
				c"vkGetPhysicalDeviceWin32PresentationSupportKHR",
				PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR
			),
			#[cfg(feature = "platform-xlib")]
			create_xlib_surface_khr: resolve!(c"vkCreateXlibSurfaceKHR", PFN_vkCreateXlibSurfaceKHR),
			#[cfg(feature = "platform-xlib")]
			get_physical_device_xlib_presentation_support_khr: resolve!(
				c"vkGetPhysicalDeviceXlibPresentationSupportKHR",
				PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR
			),
			#[cfg(feature = "platform-xcb")]
			create_xcb_surface_khr: resolve!(c"vkCreateXcbSurfaceKHR", PFN_vkCreateXcbSurfaceKHR),
			#[cfg(feature = "platform-xcb")]
			get_physical_device_xcb_presentation_support_khr: resolve!(
				c"vkGetPhysicalDeviceXcbPresentationSupportKHR",
				PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR
			),
			#[cfg(feature = "platform-directfb")]
			create_direct_fb_surface_ext: resolve!(
				c"vkCreateDirectFBSurfaceEXT",
				PFN_vkCreateDirectFBSurfaceEXT
			),
			#[cfg(feature = "platform-directfb")]
			get_physical_device_direct_fb_presentation_support_ext: resolve!(
				c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
				PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT
			),
			#[cfg(feature = "platform-fuchsia")]
			create_image_pipe_surface_fuchsia: resolve!(
				c"vkCreateImagePipeSurfaceFUCHSIA",
				PFN_vkCreateImagePipeSurfaceFUCHSIA
			),
			#[cfg(feature = "platform-ggp")]
			create_stream_descriptor_surface_ggp: resolve!(
				c"vkCreateStreamDescriptorSurfaceGGP",
				PFN_vkCreateStreamDescriptorSurfaceGGP
			),
			#[cfg(feature = "platform-screen")]
			create_screen_surface_qnx: resolve!(
				c"vkCreateScreenSurfaceQNX",
				PFN_vkCreateScreenSurfaceQNX
			),
			#[cfg(feature = "platform-screen")]
			get_physical_device_screen_presentation_support_qnx: resolve!(
				c"vkGetPhysicalDeviceScreenPresentationSupportQNX",
				PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX
			),
			create_debug_report_callback_ext: resolve!(
				c"vkCreateDebugReportCallbackEXT",
				PFN_vkCreateDebugReportCallbackEXT
			),
			destroy_debug_report_callback_ext: resolve!(
				c"vkDestroyDebugReportCallbackEXT",
				PFN_vkDestroyDebugReportCallbackEXT
			),
			debug_report_message_ext: resolve!(c"vkDebugReportMessageEXT", PFN_vkDebugReportMessageEXT),
			get_physical_device_external_image_format_properties_nv: resolve!(
				c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
				PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV
			),
			get_physical_device_features2: resolve!(
				c"vkGetPhysicalDeviceFeatures2",
				PFN_vkGetPhysicalDeviceFeatures2
			),
			get_physical_device_features2_khr: resolve!(
				c"vkGetPhysicalDeviceFeatures2KHR",
				PFN_vkGetPhysicalDeviceFeatures2KHR
			),
			get_physical_device_properties2: resolve!(
				c"vkGetPhysicalDeviceProperties2",
				PFN_vkGetPhysicalDeviceProperties2
			),
			get_physical_device_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceProperties2KHR",
				PFN_vkGetPhysicalDeviceProperties2KHR
			),
			get_physical_device_format_properties2: resolve!(
				c"vkGetPhysicalDeviceFormatProperties2",
				PFN_vkGetPhysicalDeviceFormatProperties2
			),
			get_physical_device_format_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceFormatProperties2KHR",
				PFN_vkGetPhysicalDeviceFormatProperties2KHR
			),
			get_physical_device_image_format_properties2: resolve!(
				c"vkGetPhysicalDeviceImageFormatProperties2",
				PFN_vkGetPhysicalDeviceImageFormatProperties2
			),
			get_physical_device_image_format_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceImageFormatProperties2KHR",
				PFN_vkGetPhysicalDeviceImageFormatProperties2KHR
			),
			get_physical_device_queue_family_properties2: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyProperties2",
				PFN_vkGetPhysicalDeviceQueueFamilyProperties2
			),
			get_physical_device_queue_family_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyProperties2KHR",
				PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR
			),
			get_physical_device_memory_properties2: resolve!(
				c"vkGetPhysicalDeviceMemoryProperties2",
				PFN_vkGetPhysicalDeviceMemoryProperties2
			),
			get_physical_device_memory_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceMemoryProperties2KHR",
				PFN_vkGetPhysicalDeviceMemoryProperties2KHR
			),
			get_physical_device_sparse_image_format_properties2: resolve!(
				c"vkGetPhysicalDeviceSparseImageFormatProperties2",
				PFN_vkGetPhysicalDeviceSparseImageFormatProperties2
			),
			get_physical_device_sparse_image_format_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
				PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR
			),
			get_physical_device_external_buffer_properties: resolve!(
				c"vkGetPhysicalDeviceExternalBufferProperties",
				PFN_vkGetPhysicalDeviceExternalBufferProperties
			),
			get_physical_device_external_buffer_properties_khr: resolve!(
				c"vkGetPhysicalDeviceExternalBufferPropertiesKHR",
				PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR
			),
			get_physical_device_external_semaphore_properties: resolve!(
				c"vkGetPhysicalDeviceExternalSemaphoreProperties",
				PFN_vkGetPhysicalDeviceExternalSemaphoreProperties
			),
			get_physical_device_external_semaphore_properties_khr: resolve!(
				c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
				PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
			),
			get_physical_device_external_fence_properties: resolve!(
				c"vkGetPhysicalDeviceExternalFenceProperties",
				PFN_vkGetPhysicalDeviceExternalFenceProperties
			),
			get_physical_device_external_fence_properties_khr: resolve!(
				c"vkGetPhysicalDeviceExternalFencePropertiesKHR",
				PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR
			),
			release_display_ext: resolve!(c"vkReleaseDisplayEXT", PFN_vkReleaseDisplayEXT),
			#[cfg(feature = "platform-xlib-xrandr")]
			acquire_xlib_display_ext: resolve!(c"vkAcquireXlibDisplayEXT", PFN_vkAcquireXlibDisplayEXT),
			#[cfg(feature = "platform-xlib-xrandr")]
			get_rand_r_output_display_ext: resolve!(
				c"vkGetRandROutputDisplayEXT",
				PFN_vkGetRandROutputDisplayEXT
			),
			#[cfg(feature = "platform-win32")]
			acquire_winrt_display_nv: resolve!(c"vkAcquireWinrtDisplayNV", PFN_vkAcquireWinrtDisplayNV),
			#[cfg(feature = "platform-win32")]
			get_winrt_display_nv: resolve!(c"vkGetWinrtDisplayNV", PFN_vkGetWinrtDisplayNV),
			get_physical_device_surface_capabilities2_ext: resolve!(
				c"vkGetPhysicalDeviceSurfaceCapabilities2EXT",
				PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT
			),
			enumerate_physical_device_groups: resolve!(
				c"vkEnumeratePhysicalDeviceGroups",
				PFN_vkEnumeratePhysicalDeviceGroups
			),
			enumerate_physical_device_groups_khr: resolve!(
				c"vkEnumeratePhysicalDeviceGroupsKHR",
				PFN_vkEnumeratePhysicalDeviceGroupsKHR
			),
			get_physical_device_present_rectangles_khr: resolve!(
				c"vkGetPhysicalDevicePresentRectanglesKHR",
				PFN_vkGetPhysicalDevicePresentRectanglesKHR
			),
			#[cfg(feature = "platform-ios")]
			create_ios_surface_mvk: resolve!(c"vkCreateIOSSurfaceMVK", PFN_vkCreateIOSSurfaceMVK),
			#[cfg(feature = "platform-macos")]
			create_mac_os_surface_mvk: resolve!(c"vkCreateMacOSSurfaceMVK", PFN_vkCreateMacOSSurfaceMVK),
			#[cfg(feature = "platform-metal")]
			create_metal_surface_ext: resolve!(c"vkCreateMetalSurfaceEXT", PFN_vkCreateMetalSurfaceEXT),
			get_physical_device_multisample_properties_ext: resolve!(
				c"vkGetPhysicalDeviceMultisamplePropertiesEXT",
				PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT
			),
			get_physical_device_surface_capabilities2_khr: resolve!(
				c"vkGetPhysicalDeviceSurfaceCapabilities2KHR",
				PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR
			),
			get_physical_device_surface_formats2_khr: resolve!(
				c"vkGetPhysicalDeviceSurfaceFormats2KHR",
				PFN_vkGetPhysicalDeviceSurfaceFormats2KHR
			),
			get_physical_device_display_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceDisplayProperties2KHR",
				PFN_vkGetPhysicalDeviceDisplayProperties2KHR
			),
			get_physical_device_display_plane_properties2_khr: resolve!(
				c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
				PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR
			),
			get_display_mode_properties2_khr: resolve!(
				c"vkGetDisplayModeProperties2KHR",
				PFN_vkGetDisplayModeProperties2KHR
			),
			get_display_plane_capabilities2_khr: resolve!(
				c"vkGetDisplayPlaneCapabilities2KHR",
				PFN_vkGetDisplayPlaneCapabilities2KHR
			),
			get_physical_device_calibrateable_time_domains_khr: resolve!(
				c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
				PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR
			),
			get_physical_device_calibrateable_time_domains_ext: resolve!(
				c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
				PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
			),
			create_debug_utils_messenger_ext: resolve!(
				c"vkCreateDebugUtilsMessengerEXT",
				PFN_vkCreateDebugUtilsMessengerEXT
			),
			destroy_debug_utils_messenger_ext: resolve!(
				c"vkDestroyDebugUtilsMessengerEXT",
				PFN_vkDestroyDebugUtilsMessengerEXT
			),
			submit_debug_utils_message_ext: resolve!(
				c"vkSubmitDebugUtilsMessageEXT",
				PFN_vkSubmitDebugUtilsMessageEXT
			),
			get_physical_device_cooperative_matrix_properties_nv: resolve!(
				c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
				PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
			),
			#[cfg(feature = "platform-win32")]
			get_physical_device_surface_present_modes2_ext: resolve!(
				c"vkGetPhysicalDeviceSurfacePresentModes2EXT",
				PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT
			),
			enumerate_physical_device_queue_family_performance_query_counters_khr: resolve!(
				c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
				PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
			),
			get_physical_device_queue_family_performance_query_passes_khr: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
				PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
			),
			create_headless_surface_ext: resolve!(
				c"vkCreateHeadlessSurfaceEXT",
				PFN_vkCreateHeadlessSurfaceEXT
			),
			get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: resolve!(
				c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
				PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
			),
			get_physical_device_tool_properties: resolve!(
				c"vkGetPhysicalDeviceToolProperties",
				PFN_vkGetPhysicalDeviceToolProperties
			),
			get_physical_device_tool_properties_ext: resolve!(
				c"vkGetPhysicalDeviceToolPropertiesEXT",
				PFN_vkGetPhysicalDeviceToolPropertiesEXT
			),
			get_physical_device_fragment_shading_rates_khr: resolve!(
				c"vkGetPhysicalDeviceFragmentShadingRatesKHR",
				PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR
			),
			get_physical_device_video_capabilities_khr: resolve!(
				c"vkGetPhysicalDeviceVideoCapabilitiesKHR",
				PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR
			),
			get_physical_device_video_format_properties_khr: resolve!(
				c"vkGetPhysicalDeviceVideoFormatPropertiesKHR",
				PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR
			),
			get_physical_device_video_encode_quality_level_properties_khr: resolve!(
				c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
				PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
			),
			acquire_drm_display_ext: resolve!(c"vkAcquireDrmDisplayEXT", PFN_vkAcquireDrmDisplayEXT),
			get_drm_display_ext: resolve!(c"vkGetDrmDisplayEXT", PFN_vkGetDrmDisplayEXT),
			get_physical_device_optical_flow_image_formats_nv: resolve!(
				c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
				PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV
			),
			get_physical_device_cooperative_matrix_properties_khr: resolve!(
				c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
				PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR
			),
			get_physical_device_cooperative_matrix_properties2_ext: resolve!(
				c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT",
				PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT
			),
			get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv: resolve!(
				c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV",
				PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
			),
			get_physical_device_cooperative_vector_properties_nv: resolve!(
				c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
				PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV
			),
			enumerate_physical_device_shader_instrumentation_metrics_arm: resolve!(
				c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
				PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
			),
			get_physical_device_external_tensor_properties_arm: resolve!(
				c"vkGetPhysicalDeviceExternalTensorPropertiesARM",
				PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM
			),
			get_physical_device_queue_family_data_graph_properties_arm: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
				PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM
			),
			get_physical_device_queue_family_data_graph_processing_engine_properties_arm: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
				PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
			),
			enumerate_physical_device_queue_family_performance_counters_by_region_arm: resolve!(
				c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
				PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
			),
			get_physical_device_descriptor_size_ext: resolve!(
				c"vkGetPhysicalDeviceDescriptorSizeEXT",
				PFN_vkGetPhysicalDeviceDescriptorSizeEXT
			),
			get_physical_device_queue_family_data_graph_engine_operation_properties_arm: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
				PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
			),
			get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm: resolve!(
				c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM",
				PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
			),
		}
	}

	#[cfg(feature = "loader")]
	pub(crate) fn retain_enabled(
		&mut self,
		mut enabled: impl FnMut(&super::extensions::CommandMetadata) -> bool,
	) {
		if !enabled(&super::extensions::COMMANDS[1]) {
			self.destroy_instance = None;
		}
		if !enabled(&super::extensions::COMMANDS[2]) {
			self.enumerate_physical_devices = None;
		}
		if !enabled(&super::extensions::COMMANDS[3]) {
			self.get_device_proc_addr = None;
		}
		if !enabled(&super::extensions::COMMANDS[5]) {
			self.get_physical_device_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[6]) {
			self.get_physical_device_queue_family_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[7]) {
			self.get_physical_device_memory_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[8]) {
			self.get_physical_device_features = None;
		}
		if !enabled(&super::extensions::COMMANDS[9]) {
			self.get_physical_device_format_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[10]) {
			self.get_physical_device_image_format_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[11]) {
			self.create_device = None;
		}
		if !enabled(&super::extensions::COMMANDS[16]) {
			self.enumerate_device_layer_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[17]) {
			self.enumerate_device_extension_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[34]) {
			self.get_physical_device_sparse_image_format_properties = None;
		}

		#[cfg(feature = "platform-android")]
		if !enabled(&super::extensions::COMMANDS[163]) {
			self.create_android_surface_khr = None;
		}

		#[cfg(feature = "platform-ohos")]
		if !enabled(&super::extensions::COMMANDS[164]) {
			self.create_surface_ohos = None;
		}
		if !enabled(&super::extensions::COMMANDS[165]) {
			self.get_physical_device_display_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[166]) {
			self.get_physical_device_display_plane_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[167]) {
			self.get_display_plane_supported_displays_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[168]) {
			self.get_display_mode_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[169]) {
			self.create_display_mode_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[170]) {
			self.get_display_plane_capabilities_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[171]) {
			self.create_display_plane_surface_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[173]) {
			self.destroy_surface_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[174]) {
			self.get_physical_device_surface_support_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[175]) {
			self.get_physical_device_surface_capabilities_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[176]) {
			self.get_physical_device_surface_formats_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[177]) {
			self.get_physical_device_surface_present_modes_khr = None;
		}

		#[cfg(feature = "platform-vi")]
		if !enabled(&super::extensions::COMMANDS[183]) {
			self.create_vi_surface_nn = None;
		}

		#[cfg(feature = "platform-wayland")]
		if !enabled(&super::extensions::COMMANDS[184]) {
			self.create_wayland_surface_khr = None;
		}

		#[cfg(feature = "platform-wayland")]
		if !enabled(&super::extensions::COMMANDS[185]) {
			self.get_physical_device_wayland_presentation_support_khr = None;
		}

		#[cfg(feature = "platform-ubm")]
		if !enabled(&super::extensions::COMMANDS[186]) {
			self.create_ubm_surface_sec = None;
		}

		#[cfg(feature = "platform-ubm")]
		if !enabled(&super::extensions::COMMANDS[187]) {
			self.get_physical_device_ubm_presentation_support_sec = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[188]) {
			self.create_win32_surface_khr = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[189]) {
			self.get_physical_device_win32_presentation_support_khr = None;
		}

		#[cfg(feature = "platform-xlib")]
		if !enabled(&super::extensions::COMMANDS[190]) {
			self.create_xlib_surface_khr = None;
		}

		#[cfg(feature = "platform-xlib")]
		if !enabled(&super::extensions::COMMANDS[191]) {
			self.get_physical_device_xlib_presentation_support_khr = None;
		}

		#[cfg(feature = "platform-xcb")]
		if !enabled(&super::extensions::COMMANDS[192]) {
			self.create_xcb_surface_khr = None;
		}

		#[cfg(feature = "platform-xcb")]
		if !enabled(&super::extensions::COMMANDS[193]) {
			self.get_physical_device_xcb_presentation_support_khr = None;
		}

		#[cfg(feature = "platform-directfb")]
		if !enabled(&super::extensions::COMMANDS[194]) {
			self.create_direct_fb_surface_ext = None;
		}

		#[cfg(feature = "platform-directfb")]
		if !enabled(&super::extensions::COMMANDS[195]) {
			self.get_physical_device_direct_fb_presentation_support_ext = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[196]) {
			self.create_image_pipe_surface_fuchsia = None;
		}

		#[cfg(feature = "platform-ggp")]
		if !enabled(&super::extensions::COMMANDS[197]) {
			self.create_stream_descriptor_surface_ggp = None;
		}

		#[cfg(feature = "platform-screen")]
		if !enabled(&super::extensions::COMMANDS[198]) {
			self.create_screen_surface_qnx = None;
		}

		#[cfg(feature = "platform-screen")]
		if !enabled(&super::extensions::COMMANDS[199]) {
			self.get_physical_device_screen_presentation_support_qnx = None;
		}
		if !enabled(&super::extensions::COMMANDS[200]) {
			self.create_debug_report_callback_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[201]) {
			self.destroy_debug_report_callback_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[202]) {
			self.debug_report_message_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[208]) {
			self.get_physical_device_external_image_format_properties_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[225]) {
			self.get_physical_device_features2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[226]) {
			self.get_physical_device_features2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[227]) {
			self.get_physical_device_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[228]) {
			self.get_physical_device_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[229]) {
			self.get_physical_device_format_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[230]) {
			self.get_physical_device_format_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[231]) {
			self.get_physical_device_image_format_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[232]) {
			self.get_physical_device_image_format_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[233]) {
			self.get_physical_device_queue_family_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[234]) {
			self.get_physical_device_queue_family_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[235]) {
			self.get_physical_device_memory_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[236]) {
			self.get_physical_device_memory_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[237]) {
			self.get_physical_device_sparse_image_format_properties2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[238]) {
			self.get_physical_device_sparse_image_format_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[243]) {
			self.get_physical_device_external_buffer_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[244]) {
			self.get_physical_device_external_buffer_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[252]) {
			self.get_physical_device_external_semaphore_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[253]) {
			self.get_physical_device_external_semaphore_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[260]) {
			self.get_physical_device_external_fence_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[261]) {
			self.get_physical_device_external_fence_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[266]) {
			self.release_display_ext = None;
		}

		#[cfg(feature = "platform-xlib-xrandr")]
		if !enabled(&super::extensions::COMMANDS[267]) {
			self.acquire_xlib_display_ext = None;
		}

		#[cfg(feature = "platform-xlib-xrandr")]
		if !enabled(&super::extensions::COMMANDS[268]) {
			self.get_rand_r_output_display_ext = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[269]) {
			self.acquire_winrt_display_nv = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[270]) {
			self.get_winrt_display_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[275]) {
			self.get_physical_device_surface_capabilities2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[276]) {
			self.enumerate_physical_device_groups = None;
		}
		if !enabled(&super::extensions::COMMANDS[277]) {
			self.enumerate_physical_device_groups_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[291]) {
			self.get_physical_device_present_rectangles_khr = None;
		}

		#[cfg(feature = "platform-ios")]
		if !enabled(&super::extensions::COMMANDS[304]) {
			self.create_ios_surface_mvk = None;
		}

		#[cfg(feature = "platform-macos")]
		if !enabled(&super::extensions::COMMANDS[305]) {
			self.create_mac_os_surface_mvk = None;
		}

		#[cfg(feature = "platform-metal")]
		if !enabled(&super::extensions::COMMANDS[306]) {
			self.create_metal_surface_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[312]) {
			self.get_physical_device_multisample_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[313]) {
			self.get_physical_device_surface_capabilities2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[314]) {
			self.get_physical_device_surface_formats2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[315]) {
			self.get_physical_device_display_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[316]) {
			self.get_physical_device_display_plane_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[317]) {
			self.get_display_mode_properties2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[318]) {
			self.get_display_plane_capabilities2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[344]) {
			self.get_physical_device_calibrateable_time_domains_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[345]) {
			self.get_physical_device_calibrateable_time_domains_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[356]) {
			self.create_debug_utils_messenger_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[357]) {
			self.destroy_debug_utils_messenger_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[358]) {
			self.submit_debug_utils_message_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[428]) {
			self.get_physical_device_cooperative_matrix_properties_nv = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[440]) {
			self.get_physical_device_surface_present_modes2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[444]) {
			self.enumerate_physical_device_queue_family_performance_query_counters_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[445]) {
			self.get_physical_device_queue_family_performance_query_passes_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[454]) {
			self.create_headless_surface_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[455]) {
			self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[473]) {
			self.get_physical_device_tool_properties = None;
		}
		if !enabled(&super::extensions::COMMANDS[474]) {
			self.get_physical_device_tool_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[574]) {
			self.get_physical_device_fragment_shading_rates_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[601]) {
			self.get_physical_device_video_capabilities_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[602]) {
			self.get_physical_device_video_format_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[603]) {
			self.get_physical_device_video_encode_quality_level_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[640]) {
			self.acquire_drm_display_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[641]) {
			self.get_drm_display_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[687]) {
			self.get_physical_device_optical_flow_image_formats_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[713]) {
			self.get_physical_device_cooperative_matrix_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[714]) {
			self.get_physical_device_cooperative_matrix_properties2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[761]) {
			self.get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[764]) {
			self.get_physical_device_cooperative_vector_properties_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[773]) {
			self.enumerate_physical_device_shader_instrumentation_metrics_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[790]) {
			self.get_physical_device_external_tensor_properties_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[800]) {
			self.get_physical_device_queue_family_data_graph_properties_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[801]) {
			self.get_physical_device_queue_family_data_graph_processing_engine_properties_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[805]) {
			self.enumerate_physical_device_queue_family_performance_counters_by_region_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[815]) {
			self.get_physical_device_descriptor_size_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[839]) {
			self.get_physical_device_queue_family_data_graph_engine_operation_properties_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[841]) {
			self.get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm = None;
		}
	}
}

impl FnDevice {
	/// Load all commands for this dispatch level.
	///
	/// # Safety
	/// `load` must implement the Vulkan proc-addr contract and remain callable
	/// for the lifetime of this table and all derived raw function pointers.
	pub unsafe fn new_from_resolver(
		load: unsafe extern "system" fn(DeviceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction,
		device: DeviceHandle,
	) -> Self {
		macro_rules! resolve {
			($name:literal, $ty:ty) => {
				unsafe { std::mem::transmute::<PFN_vkVoidFunction, $ty>(load(device, $name.as_ptr())) }
			};
		}
		Self {
			destroy_device: resolve!(c"vkDestroyDevice", PFN_vkDestroyDevice),
			get_device_queue: resolve!(c"vkGetDeviceQueue", PFN_vkGetDeviceQueue),
			queue_submit: resolve!(c"vkQueueSubmit", PFN_vkQueueSubmit),
			queue_wait_idle: resolve!(c"vkQueueWaitIdle", PFN_vkQueueWaitIdle),
			device_wait_idle: resolve!(c"vkDeviceWaitIdle", PFN_vkDeviceWaitIdle),
			allocate_memory: resolve!(c"vkAllocateMemory", PFN_vkAllocateMemory),
			free_memory: resolve!(c"vkFreeMemory", PFN_vkFreeMemory),
			map_memory: resolve!(c"vkMapMemory", PFN_vkMapMemory),
			unmap_memory: resolve!(c"vkUnmapMemory", PFN_vkUnmapMemory),
			flush_mapped_memory_ranges: resolve!(
				c"vkFlushMappedMemoryRanges",
				PFN_vkFlushMappedMemoryRanges
			),
			invalidate_mapped_memory_ranges: resolve!(
				c"vkInvalidateMappedMemoryRanges",
				PFN_vkInvalidateMappedMemoryRanges
			),
			get_device_memory_commitment: resolve!(
				c"vkGetDeviceMemoryCommitment",
				PFN_vkGetDeviceMemoryCommitment
			),
			get_buffer_memory_requirements: resolve!(
				c"vkGetBufferMemoryRequirements",
				PFN_vkGetBufferMemoryRequirements
			),
			bind_buffer_memory: resolve!(c"vkBindBufferMemory", PFN_vkBindBufferMemory),
			get_image_memory_requirements: resolve!(
				c"vkGetImageMemoryRequirements",
				PFN_vkGetImageMemoryRequirements
			),
			bind_image_memory: resolve!(c"vkBindImageMemory", PFN_vkBindImageMemory),
			get_image_sparse_memory_requirements: resolve!(
				c"vkGetImageSparseMemoryRequirements",
				PFN_vkGetImageSparseMemoryRequirements
			),
			queue_bind_sparse: resolve!(c"vkQueueBindSparse", PFN_vkQueueBindSparse),
			create_fence: resolve!(c"vkCreateFence", PFN_vkCreateFence),
			destroy_fence: resolve!(c"vkDestroyFence", PFN_vkDestroyFence),
			reset_fences: resolve!(c"vkResetFences", PFN_vkResetFences),
			get_fence_status: resolve!(c"vkGetFenceStatus", PFN_vkGetFenceStatus),
			wait_for_fences: resolve!(c"vkWaitForFences", PFN_vkWaitForFences),
			create_semaphore: resolve!(c"vkCreateSemaphore", PFN_vkCreateSemaphore),
			destroy_semaphore: resolve!(c"vkDestroySemaphore", PFN_vkDestroySemaphore),
			create_event: resolve!(c"vkCreateEvent", PFN_vkCreateEvent),
			destroy_event: resolve!(c"vkDestroyEvent", PFN_vkDestroyEvent),
			get_event_status: resolve!(c"vkGetEventStatus", PFN_vkGetEventStatus),
			set_event: resolve!(c"vkSetEvent", PFN_vkSetEvent),
			reset_event: resolve!(c"vkResetEvent", PFN_vkResetEvent),
			create_query_pool: resolve!(c"vkCreateQueryPool", PFN_vkCreateQueryPool),
			destroy_query_pool: resolve!(c"vkDestroyQueryPool", PFN_vkDestroyQueryPool),
			get_query_pool_results: resolve!(c"vkGetQueryPoolResults", PFN_vkGetQueryPoolResults),
			reset_query_pool: resolve!(c"vkResetQueryPool", PFN_vkResetQueryPool),
			reset_query_pool_ext: resolve!(c"vkResetQueryPoolEXT", PFN_vkResetQueryPoolEXT),
			create_buffer: resolve!(c"vkCreateBuffer", PFN_vkCreateBuffer),
			destroy_buffer: resolve!(c"vkDestroyBuffer", PFN_vkDestroyBuffer),
			create_buffer_view: resolve!(c"vkCreateBufferView", PFN_vkCreateBufferView),
			destroy_buffer_view: resolve!(c"vkDestroyBufferView", PFN_vkDestroyBufferView),
			create_image: resolve!(c"vkCreateImage", PFN_vkCreateImage),
			destroy_image: resolve!(c"vkDestroyImage", PFN_vkDestroyImage),
			get_image_subresource_layout: resolve!(
				c"vkGetImageSubresourceLayout",
				PFN_vkGetImageSubresourceLayout
			),
			create_image_view: resolve!(c"vkCreateImageView", PFN_vkCreateImageView),
			destroy_image_view: resolve!(c"vkDestroyImageView", PFN_vkDestroyImageView),
			create_shader_module: resolve!(c"vkCreateShaderModule", PFN_vkCreateShaderModule),
			destroy_shader_module: resolve!(c"vkDestroyShaderModule", PFN_vkDestroyShaderModule),
			create_pipeline_cache: resolve!(c"vkCreatePipelineCache", PFN_vkCreatePipelineCache),
			destroy_pipeline_cache: resolve!(c"vkDestroyPipelineCache", PFN_vkDestroyPipelineCache),
			get_pipeline_cache_data: resolve!(c"vkGetPipelineCacheData", PFN_vkGetPipelineCacheData),
			merge_pipeline_caches: resolve!(c"vkMergePipelineCaches", PFN_vkMergePipelineCaches),
			create_pipeline_binaries_khr: resolve!(
				c"vkCreatePipelineBinariesKHR",
				PFN_vkCreatePipelineBinariesKHR
			),
			destroy_pipeline_binary_khr: resolve!(
				c"vkDestroyPipelineBinaryKHR",
				PFN_vkDestroyPipelineBinaryKHR
			),
			get_pipeline_key_khr: resolve!(c"vkGetPipelineKeyKHR", PFN_vkGetPipelineKeyKHR),
			get_pipeline_binary_data_khr: resolve!(
				c"vkGetPipelineBinaryDataKHR",
				PFN_vkGetPipelineBinaryDataKHR
			),
			release_captured_pipeline_data_khr: resolve!(
				c"vkReleaseCapturedPipelineDataKHR",
				PFN_vkReleaseCapturedPipelineDataKHR
			),
			create_graphics_pipelines: resolve!(
				c"vkCreateGraphicsPipelines",
				PFN_vkCreateGraphicsPipelines
			),
			create_compute_pipelines: resolve!(c"vkCreateComputePipelines", PFN_vkCreateComputePipelines),
			get_device_subpass_shading_max_workgroup_size_huawei: resolve!(
				c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI",
				PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI
			),
			destroy_pipeline: resolve!(c"vkDestroyPipeline", PFN_vkDestroyPipeline),
			create_pipeline_layout: resolve!(c"vkCreatePipelineLayout", PFN_vkCreatePipelineLayout),
			destroy_pipeline_layout: resolve!(c"vkDestroyPipelineLayout", PFN_vkDestroyPipelineLayout),
			create_sampler: resolve!(c"vkCreateSampler", PFN_vkCreateSampler),
			destroy_sampler: resolve!(c"vkDestroySampler", PFN_vkDestroySampler),
			create_descriptor_set_layout: resolve!(
				c"vkCreateDescriptorSetLayout",
				PFN_vkCreateDescriptorSetLayout
			),
			destroy_descriptor_set_layout: resolve!(
				c"vkDestroyDescriptorSetLayout",
				PFN_vkDestroyDescriptorSetLayout
			),
			create_descriptor_pool: resolve!(c"vkCreateDescriptorPool", PFN_vkCreateDescriptorPool),
			destroy_descriptor_pool: resolve!(c"vkDestroyDescriptorPool", PFN_vkDestroyDescriptorPool),
			reset_descriptor_pool: resolve!(c"vkResetDescriptorPool", PFN_vkResetDescriptorPool),
			allocate_descriptor_sets: resolve!(c"vkAllocateDescriptorSets", PFN_vkAllocateDescriptorSets),
			free_descriptor_sets: resolve!(c"vkFreeDescriptorSets", PFN_vkFreeDescriptorSets),
			update_descriptor_sets: resolve!(c"vkUpdateDescriptorSets", PFN_vkUpdateDescriptorSets),
			create_framebuffer: resolve!(c"vkCreateFramebuffer", PFN_vkCreateFramebuffer),
			destroy_framebuffer: resolve!(c"vkDestroyFramebuffer", PFN_vkDestroyFramebuffer),
			create_render_pass: resolve!(c"vkCreateRenderPass", PFN_vkCreateRenderPass),
			destroy_render_pass: resolve!(c"vkDestroyRenderPass", PFN_vkDestroyRenderPass),
			get_render_area_granularity: resolve!(
				c"vkGetRenderAreaGranularity",
				PFN_vkGetRenderAreaGranularity
			),
			get_rendering_area_granularity: resolve!(
				c"vkGetRenderingAreaGranularity",
				PFN_vkGetRenderingAreaGranularity
			),
			get_rendering_area_granularity_khr: resolve!(
				c"vkGetRenderingAreaGranularityKHR",
				PFN_vkGetRenderingAreaGranularityKHR
			),
			create_command_pool: resolve!(c"vkCreateCommandPool", PFN_vkCreateCommandPool),
			destroy_command_pool: resolve!(c"vkDestroyCommandPool", PFN_vkDestroyCommandPool),
			reset_command_pool: resolve!(c"vkResetCommandPool", PFN_vkResetCommandPool),
			allocate_command_buffers: resolve!(c"vkAllocateCommandBuffers", PFN_vkAllocateCommandBuffers),
			free_command_buffers: resolve!(c"vkFreeCommandBuffers", PFN_vkFreeCommandBuffers),
			begin_command_buffer: resolve!(c"vkBeginCommandBuffer", PFN_vkBeginCommandBuffer),
			end_command_buffer: resolve!(c"vkEndCommandBuffer", PFN_vkEndCommandBuffer),
			reset_command_buffer: resolve!(c"vkResetCommandBuffer", PFN_vkResetCommandBuffer),
			cmd_bind_pipeline: resolve!(c"vkCmdBindPipeline", PFN_vkCmdBindPipeline),
			cmd_set_primitive_restart_index_ext: resolve!(
				c"vkCmdSetPrimitiveRestartIndexEXT",
				PFN_vkCmdSetPrimitiveRestartIndexEXT
			),
			cmd_set_attachment_feedback_loop_enable_ext: resolve!(
				c"vkCmdSetAttachmentFeedbackLoopEnableEXT",
				PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT
			),
			cmd_set_viewport: resolve!(c"vkCmdSetViewport", PFN_vkCmdSetViewport),
			cmd_set_scissor: resolve!(c"vkCmdSetScissor", PFN_vkCmdSetScissor),
			cmd_set_line_width: resolve!(c"vkCmdSetLineWidth", PFN_vkCmdSetLineWidth),
			cmd_set_depth_bias: resolve!(c"vkCmdSetDepthBias", PFN_vkCmdSetDepthBias),
			cmd_set_blend_constants: resolve!(c"vkCmdSetBlendConstants", PFN_vkCmdSetBlendConstants),
			cmd_set_depth_bounds: resolve!(c"vkCmdSetDepthBounds", PFN_vkCmdSetDepthBounds),
			cmd_set_stencil_compare_mask: resolve!(
				c"vkCmdSetStencilCompareMask",
				PFN_vkCmdSetStencilCompareMask
			),
			cmd_set_stencil_write_mask: resolve!(
				c"vkCmdSetStencilWriteMask",
				PFN_vkCmdSetStencilWriteMask
			),
			cmd_set_stencil_reference: resolve!(
				c"vkCmdSetStencilReference",
				PFN_vkCmdSetStencilReference
			),
			cmd_bind_descriptor_sets: resolve!(c"vkCmdBindDescriptorSets", PFN_vkCmdBindDescriptorSets),
			cmd_bind_index_buffer: resolve!(c"vkCmdBindIndexBuffer", PFN_vkCmdBindIndexBuffer),
			cmd_bind_vertex_buffers: resolve!(c"vkCmdBindVertexBuffers", PFN_vkCmdBindVertexBuffers),
			cmd_draw: resolve!(c"vkCmdDraw", PFN_vkCmdDraw),
			cmd_draw_indexed: resolve!(c"vkCmdDrawIndexed", PFN_vkCmdDrawIndexed),
			cmd_draw_multi_ext: resolve!(c"vkCmdDrawMultiEXT", PFN_vkCmdDrawMultiEXT),
			cmd_draw_multi_indexed_ext: resolve!(
				c"vkCmdDrawMultiIndexedEXT",
				PFN_vkCmdDrawMultiIndexedEXT
			),
			cmd_draw_indirect: resolve!(c"vkCmdDrawIndirect", PFN_vkCmdDrawIndirect),
			cmd_draw_indexed_indirect: resolve!(
				c"vkCmdDrawIndexedIndirect",
				PFN_vkCmdDrawIndexedIndirect
			),
			cmd_dispatch: resolve!(c"vkCmdDispatch", PFN_vkCmdDispatch),
			cmd_dispatch_indirect: resolve!(c"vkCmdDispatchIndirect", PFN_vkCmdDispatchIndirect),
			cmd_subpass_shading_huawei: resolve!(
				c"vkCmdSubpassShadingHUAWEI",
				PFN_vkCmdSubpassShadingHUAWEI
			),
			cmd_draw_cluster_huawei: resolve!(c"vkCmdDrawClusterHUAWEI", PFN_vkCmdDrawClusterHUAWEI),
			cmd_draw_cluster_indirect_huawei: resolve!(
				c"vkCmdDrawClusterIndirectHUAWEI",
				PFN_vkCmdDrawClusterIndirectHUAWEI
			),
			cmd_update_pipeline_indirect_buffer_nv: resolve!(
				c"vkCmdUpdatePipelineIndirectBufferNV",
				PFN_vkCmdUpdatePipelineIndirectBufferNV
			),
			cmd_copy_buffer: resolve!(c"vkCmdCopyBuffer", PFN_vkCmdCopyBuffer),
			cmd_copy_image: resolve!(c"vkCmdCopyImage", PFN_vkCmdCopyImage),
			cmd_blit_image: resolve!(c"vkCmdBlitImage", PFN_vkCmdBlitImage),
			cmd_copy_buffer_to_image: resolve!(c"vkCmdCopyBufferToImage", PFN_vkCmdCopyBufferToImage),
			cmd_copy_image_to_buffer: resolve!(c"vkCmdCopyImageToBuffer", PFN_vkCmdCopyImageToBuffer),
			cmd_copy_memory_indirect_nv: resolve!(
				c"vkCmdCopyMemoryIndirectNV",
				PFN_vkCmdCopyMemoryIndirectNV
			),
			cmd_copy_memory_indirect_khr: resolve!(
				c"vkCmdCopyMemoryIndirectKHR",
				PFN_vkCmdCopyMemoryIndirectKHR
			),
			cmd_copy_memory_to_image_indirect_nv: resolve!(
				c"vkCmdCopyMemoryToImageIndirectNV",
				PFN_vkCmdCopyMemoryToImageIndirectNV
			),
			cmd_copy_memory_to_image_indirect_khr: resolve!(
				c"vkCmdCopyMemoryToImageIndirectKHR",
				PFN_vkCmdCopyMemoryToImageIndirectKHR
			),
			cmd_update_buffer: resolve!(c"vkCmdUpdateBuffer", PFN_vkCmdUpdateBuffer),
			cmd_fill_buffer: resolve!(c"vkCmdFillBuffer", PFN_vkCmdFillBuffer),
			cmd_clear_color_image: resolve!(c"vkCmdClearColorImage", PFN_vkCmdClearColorImage),
			cmd_clear_depth_stencil_image: resolve!(
				c"vkCmdClearDepthStencilImage",
				PFN_vkCmdClearDepthStencilImage
			),
			cmd_clear_attachments: resolve!(c"vkCmdClearAttachments", PFN_vkCmdClearAttachments),
			cmd_resolve_image: resolve!(c"vkCmdResolveImage", PFN_vkCmdResolveImage),
			cmd_set_event: resolve!(c"vkCmdSetEvent", PFN_vkCmdSetEvent),
			cmd_reset_event: resolve!(c"vkCmdResetEvent", PFN_vkCmdResetEvent),
			cmd_wait_events: resolve!(c"vkCmdWaitEvents", PFN_vkCmdWaitEvents),
			cmd_pipeline_barrier: resolve!(c"vkCmdPipelineBarrier", PFN_vkCmdPipelineBarrier),
			cmd_begin_query: resolve!(c"vkCmdBeginQuery", PFN_vkCmdBeginQuery),
			cmd_end_query: resolve!(c"vkCmdEndQuery", PFN_vkCmdEndQuery),
			cmd_begin_conditional_rendering_ext: resolve!(
				c"vkCmdBeginConditionalRenderingEXT",
				PFN_vkCmdBeginConditionalRenderingEXT
			),
			cmd_end_conditional_rendering_ext: resolve!(
				c"vkCmdEndConditionalRenderingEXT",
				PFN_vkCmdEndConditionalRenderingEXT
			),
			cmd_begin_custom_resolve_ext: resolve!(
				c"vkCmdBeginCustomResolveEXT",
				PFN_vkCmdBeginCustomResolveEXT
			),
			cmd_reset_query_pool: resolve!(c"vkCmdResetQueryPool", PFN_vkCmdResetQueryPool),
			cmd_write_timestamp: resolve!(c"vkCmdWriteTimestamp", PFN_vkCmdWriteTimestamp),
			cmd_copy_query_pool_results: resolve!(
				c"vkCmdCopyQueryPoolResults",
				PFN_vkCmdCopyQueryPoolResults
			),
			cmd_push_constants: resolve!(c"vkCmdPushConstants", PFN_vkCmdPushConstants),
			cmd_begin_render_pass: resolve!(c"vkCmdBeginRenderPass", PFN_vkCmdBeginRenderPass),
			cmd_next_subpass: resolve!(c"vkCmdNextSubpass", PFN_vkCmdNextSubpass),
			cmd_end_render_pass: resolve!(c"vkCmdEndRenderPass", PFN_vkCmdEndRenderPass),
			cmd_execute_commands: resolve!(c"vkCmdExecuteCommands", PFN_vkCmdExecuteCommands),
			create_shared_swapchains_khr: resolve!(
				c"vkCreateSharedSwapchainsKHR",
				PFN_vkCreateSharedSwapchainsKHR
			),
			create_swapchain_khr: resolve!(c"vkCreateSwapchainKHR", PFN_vkCreateSwapchainKHR),
			destroy_swapchain_khr: resolve!(c"vkDestroySwapchainKHR", PFN_vkDestroySwapchainKHR),
			get_swapchain_images_khr: resolve!(c"vkGetSwapchainImagesKHR", PFN_vkGetSwapchainImagesKHR),
			acquire_next_image_khr: resolve!(c"vkAcquireNextImageKHR", PFN_vkAcquireNextImageKHR),
			queue_present_khr: resolve!(c"vkQueuePresentKHR", PFN_vkQueuePresentKHR),
			debug_marker_set_object_name_ext: resolve!(
				c"vkDebugMarkerSetObjectNameEXT",
				PFN_vkDebugMarkerSetObjectNameEXT
			),
			debug_marker_set_object_tag_ext: resolve!(
				c"vkDebugMarkerSetObjectTagEXT",
				PFN_vkDebugMarkerSetObjectTagEXT
			),
			cmd_debug_marker_begin_ext: resolve!(
				c"vkCmdDebugMarkerBeginEXT",
				PFN_vkCmdDebugMarkerBeginEXT
			),
			cmd_debug_marker_end_ext: resolve!(c"vkCmdDebugMarkerEndEXT", PFN_vkCmdDebugMarkerEndEXT),
			cmd_debug_marker_insert_ext: resolve!(
				c"vkCmdDebugMarkerInsertEXT",
				PFN_vkCmdDebugMarkerInsertEXT
			),
			#[cfg(feature = "platform-win32")]
			get_memory_win32_handle_nv: resolve!(
				c"vkGetMemoryWin32HandleNV",
				PFN_vkGetMemoryWin32HandleNV
			),
			cmd_execute_generated_commands_nv: resolve!(
				c"vkCmdExecuteGeneratedCommandsNV",
				PFN_vkCmdExecuteGeneratedCommandsNV
			),
			cmd_preprocess_generated_commands_nv: resolve!(
				c"vkCmdPreprocessGeneratedCommandsNV",
				PFN_vkCmdPreprocessGeneratedCommandsNV
			),
			cmd_bind_pipeline_shader_group_nv: resolve!(
				c"vkCmdBindPipelineShaderGroupNV",
				PFN_vkCmdBindPipelineShaderGroupNV
			),
			get_generated_commands_memory_requirements_nv: resolve!(
				c"vkGetGeneratedCommandsMemoryRequirementsNV",
				PFN_vkGetGeneratedCommandsMemoryRequirementsNV
			),
			create_indirect_commands_layout_nv: resolve!(
				c"vkCreateIndirectCommandsLayoutNV",
				PFN_vkCreateIndirectCommandsLayoutNV
			),
			destroy_indirect_commands_layout_nv: resolve!(
				c"vkDestroyIndirectCommandsLayoutNV",
				PFN_vkDestroyIndirectCommandsLayoutNV
			),
			cmd_execute_generated_commands_ext: resolve!(
				c"vkCmdExecuteGeneratedCommandsEXT",
				PFN_vkCmdExecuteGeneratedCommandsEXT
			),
			cmd_preprocess_generated_commands_ext: resolve!(
				c"vkCmdPreprocessGeneratedCommandsEXT",
				PFN_vkCmdPreprocessGeneratedCommandsEXT
			),
			get_generated_commands_memory_requirements_ext: resolve!(
				c"vkGetGeneratedCommandsMemoryRequirementsEXT",
				PFN_vkGetGeneratedCommandsMemoryRequirementsEXT
			),
			create_indirect_commands_layout_ext: resolve!(
				c"vkCreateIndirectCommandsLayoutEXT",
				PFN_vkCreateIndirectCommandsLayoutEXT
			),
			destroy_indirect_commands_layout_ext: resolve!(
				c"vkDestroyIndirectCommandsLayoutEXT",
				PFN_vkDestroyIndirectCommandsLayoutEXT
			),
			create_indirect_execution_set_ext: resolve!(
				c"vkCreateIndirectExecutionSetEXT",
				PFN_vkCreateIndirectExecutionSetEXT
			),
			destroy_indirect_execution_set_ext: resolve!(
				c"vkDestroyIndirectExecutionSetEXT",
				PFN_vkDestroyIndirectExecutionSetEXT
			),
			update_indirect_execution_set_pipeline_ext: resolve!(
				c"vkUpdateIndirectExecutionSetPipelineEXT",
				PFN_vkUpdateIndirectExecutionSetPipelineEXT
			),
			update_indirect_execution_set_shader_ext: resolve!(
				c"vkUpdateIndirectExecutionSetShaderEXT",
				PFN_vkUpdateIndirectExecutionSetShaderEXT
			),
			cmd_push_descriptor_set: resolve!(c"vkCmdPushDescriptorSet", PFN_vkCmdPushDescriptorSet),
			cmd_push_descriptor_set_khr: resolve!(
				c"vkCmdPushDescriptorSetKHR",
				PFN_vkCmdPushDescriptorSetKHR
			),
			trim_command_pool: resolve!(c"vkTrimCommandPool", PFN_vkTrimCommandPool),
			trim_command_pool_khr: resolve!(c"vkTrimCommandPoolKHR", PFN_vkTrimCommandPoolKHR),
			#[cfg(feature = "platform-win32")]
			get_memory_win32_handle_khr: resolve!(
				c"vkGetMemoryWin32HandleKHR",
				PFN_vkGetMemoryWin32HandleKHR
			),
			#[cfg(feature = "platform-win32")]
			get_memory_win32_handle_properties_khr: resolve!(
				c"vkGetMemoryWin32HandlePropertiesKHR",
				PFN_vkGetMemoryWin32HandlePropertiesKHR
			),
			get_memory_fd_khr: resolve!(c"vkGetMemoryFdKHR", PFN_vkGetMemoryFdKHR),
			get_memory_fd_properties_khr: resolve!(
				c"vkGetMemoryFdPropertiesKHR",
				PFN_vkGetMemoryFdPropertiesKHR
			),
			#[cfg(feature = "platform-fuchsia")]
			get_memory_zircon_handle_fuchsia: resolve!(
				c"vkGetMemoryZirconHandleFUCHSIA",
				PFN_vkGetMemoryZirconHandleFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			get_memory_zircon_handle_properties_fuchsia: resolve!(
				c"vkGetMemoryZirconHandlePropertiesFUCHSIA",
				PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA
			),
			get_memory_remote_address_nv: resolve!(
				c"vkGetMemoryRemoteAddressNV",
				PFN_vkGetMemoryRemoteAddressNV
			),
			#[cfg(feature = "platform-win32")]
			get_semaphore_win32_handle_khr: resolve!(
				c"vkGetSemaphoreWin32HandleKHR",
				PFN_vkGetSemaphoreWin32HandleKHR
			),
			#[cfg(feature = "platform-win32")]
			import_semaphore_win32_handle_khr: resolve!(
				c"vkImportSemaphoreWin32HandleKHR",
				PFN_vkImportSemaphoreWin32HandleKHR
			),
			get_semaphore_fd_khr: resolve!(c"vkGetSemaphoreFdKHR", PFN_vkGetSemaphoreFdKHR),
			import_semaphore_fd_khr: resolve!(c"vkImportSemaphoreFdKHR", PFN_vkImportSemaphoreFdKHR),
			#[cfg(feature = "platform-fuchsia")]
			get_semaphore_zircon_handle_fuchsia: resolve!(
				c"vkGetSemaphoreZirconHandleFUCHSIA",
				PFN_vkGetSemaphoreZirconHandleFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			import_semaphore_zircon_handle_fuchsia: resolve!(
				c"vkImportSemaphoreZirconHandleFUCHSIA",
				PFN_vkImportSemaphoreZirconHandleFUCHSIA
			),
			#[cfg(feature = "platform-win32")]
			get_fence_win32_handle_khr: resolve!(
				c"vkGetFenceWin32HandleKHR",
				PFN_vkGetFenceWin32HandleKHR
			),
			#[cfg(feature = "platform-win32")]
			import_fence_win32_handle_khr: resolve!(
				c"vkImportFenceWin32HandleKHR",
				PFN_vkImportFenceWin32HandleKHR
			),
			get_fence_fd_khr: resolve!(c"vkGetFenceFdKHR", PFN_vkGetFenceFdKHR),
			import_fence_fd_khr: resolve!(c"vkImportFenceFdKHR", PFN_vkImportFenceFdKHR),
			display_power_control_ext: resolve!(
				c"vkDisplayPowerControlEXT",
				PFN_vkDisplayPowerControlEXT
			),
			register_device_event_ext: resolve!(
				c"vkRegisterDeviceEventEXT",
				PFN_vkRegisterDeviceEventEXT
			),
			register_display_event_ext: resolve!(
				c"vkRegisterDisplayEventEXT",
				PFN_vkRegisterDisplayEventEXT
			),
			get_swapchain_counter_ext: resolve!(
				c"vkGetSwapchainCounterEXT",
				PFN_vkGetSwapchainCounterEXT
			),
			get_device_group_peer_memory_features: resolve!(
				c"vkGetDeviceGroupPeerMemoryFeatures",
				PFN_vkGetDeviceGroupPeerMemoryFeatures
			),
			get_device_group_peer_memory_features_khr: resolve!(
				c"vkGetDeviceGroupPeerMemoryFeaturesKHR",
				PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR
			),
			bind_buffer_memory2: resolve!(c"vkBindBufferMemory2", PFN_vkBindBufferMemory2),
			bind_buffer_memory2_khr: resolve!(c"vkBindBufferMemory2KHR", PFN_vkBindBufferMemory2KHR),
			bind_image_memory2: resolve!(c"vkBindImageMemory2", PFN_vkBindImageMemory2),
			bind_image_memory2_khr: resolve!(c"vkBindImageMemory2KHR", PFN_vkBindImageMemory2KHR),
			cmd_set_device_mask: resolve!(c"vkCmdSetDeviceMask", PFN_vkCmdSetDeviceMask),
			cmd_set_device_mask_khr: resolve!(c"vkCmdSetDeviceMaskKHR", PFN_vkCmdSetDeviceMaskKHR),
			get_device_group_present_capabilities_khr: resolve!(
				c"vkGetDeviceGroupPresentCapabilitiesKHR",
				PFN_vkGetDeviceGroupPresentCapabilitiesKHR
			),
			get_device_group_surface_present_modes_khr: resolve!(
				c"vkGetDeviceGroupSurfacePresentModesKHR",
				PFN_vkGetDeviceGroupSurfacePresentModesKHR
			),
			acquire_next_image2_khr: resolve!(c"vkAcquireNextImage2KHR", PFN_vkAcquireNextImage2KHR),
			cmd_dispatch_base: resolve!(c"vkCmdDispatchBase", PFN_vkCmdDispatchBase),
			cmd_dispatch_base_khr: resolve!(c"vkCmdDispatchBaseKHR", PFN_vkCmdDispatchBaseKHR),
			create_descriptor_update_template: resolve!(
				c"vkCreateDescriptorUpdateTemplate",
				PFN_vkCreateDescriptorUpdateTemplate
			),
			create_descriptor_update_template_khr: resolve!(
				c"vkCreateDescriptorUpdateTemplateKHR",
				PFN_vkCreateDescriptorUpdateTemplateKHR
			),
			destroy_descriptor_update_template: resolve!(
				c"vkDestroyDescriptorUpdateTemplate",
				PFN_vkDestroyDescriptorUpdateTemplate
			),
			destroy_descriptor_update_template_khr: resolve!(
				c"vkDestroyDescriptorUpdateTemplateKHR",
				PFN_vkDestroyDescriptorUpdateTemplateKHR
			),
			update_descriptor_set_with_template: resolve!(
				c"vkUpdateDescriptorSetWithTemplate",
				PFN_vkUpdateDescriptorSetWithTemplate
			),
			update_descriptor_set_with_template_khr: resolve!(
				c"vkUpdateDescriptorSetWithTemplateKHR",
				PFN_vkUpdateDescriptorSetWithTemplateKHR
			),
			cmd_push_descriptor_set_with_template: resolve!(
				c"vkCmdPushDescriptorSetWithTemplate",
				PFN_vkCmdPushDescriptorSetWithTemplate
			),
			cmd_push_descriptor_set_with_template_khr: resolve!(
				c"vkCmdPushDescriptorSetWithTemplateKHR",
				PFN_vkCmdPushDescriptorSetWithTemplateKHR
			),
			set_hdr_metadata_ext: resolve!(c"vkSetHdrMetadataEXT", PFN_vkSetHdrMetadataEXT),
			get_swapchain_status_khr: resolve!(c"vkGetSwapchainStatusKHR", PFN_vkGetSwapchainStatusKHR),
			get_refresh_cycle_duration_google: resolve!(
				c"vkGetRefreshCycleDurationGOOGLE",
				PFN_vkGetRefreshCycleDurationGOOGLE
			),
			get_past_presentation_timing_google: resolve!(
				c"vkGetPastPresentationTimingGOOGLE",
				PFN_vkGetPastPresentationTimingGOOGLE
			),
			cmd_set_viewport_w_scaling_nv: resolve!(
				c"vkCmdSetViewportWScalingNV",
				PFN_vkCmdSetViewportWScalingNV
			),
			cmd_set_discard_rectangle_ext: resolve!(
				c"vkCmdSetDiscardRectangleEXT",
				PFN_vkCmdSetDiscardRectangleEXT
			),
			cmd_set_discard_rectangle_enable_ext: resolve!(
				c"vkCmdSetDiscardRectangleEnableEXT",
				PFN_vkCmdSetDiscardRectangleEnableEXT
			),
			cmd_set_discard_rectangle_mode_ext: resolve!(
				c"vkCmdSetDiscardRectangleModeEXT",
				PFN_vkCmdSetDiscardRectangleModeEXT
			),
			cmd_set_sample_locations_ext: resolve!(
				c"vkCmdSetSampleLocationsEXT",
				PFN_vkCmdSetSampleLocationsEXT
			),
			get_buffer_memory_requirements2: resolve!(
				c"vkGetBufferMemoryRequirements2",
				PFN_vkGetBufferMemoryRequirements2
			),
			get_buffer_memory_requirements2_khr: resolve!(
				c"vkGetBufferMemoryRequirements2KHR",
				PFN_vkGetBufferMemoryRequirements2KHR
			),
			get_image_memory_requirements2: resolve!(
				c"vkGetImageMemoryRequirements2",
				PFN_vkGetImageMemoryRequirements2
			),
			get_image_memory_requirements2_khr: resolve!(
				c"vkGetImageMemoryRequirements2KHR",
				PFN_vkGetImageMemoryRequirements2KHR
			),
			get_image_sparse_memory_requirements2: resolve!(
				c"vkGetImageSparseMemoryRequirements2",
				PFN_vkGetImageSparseMemoryRequirements2
			),
			get_image_sparse_memory_requirements2_khr: resolve!(
				c"vkGetImageSparseMemoryRequirements2KHR",
				PFN_vkGetImageSparseMemoryRequirements2KHR
			),
			get_device_buffer_memory_requirements: resolve!(
				c"vkGetDeviceBufferMemoryRequirements",
				PFN_vkGetDeviceBufferMemoryRequirements
			),
			get_device_buffer_memory_requirements_khr: resolve!(
				c"vkGetDeviceBufferMemoryRequirementsKHR",
				PFN_vkGetDeviceBufferMemoryRequirementsKHR
			),
			get_device_image_memory_requirements: resolve!(
				c"vkGetDeviceImageMemoryRequirements",
				PFN_vkGetDeviceImageMemoryRequirements
			),
			get_device_image_memory_requirements_khr: resolve!(
				c"vkGetDeviceImageMemoryRequirementsKHR",
				PFN_vkGetDeviceImageMemoryRequirementsKHR
			),
			get_device_image_sparse_memory_requirements: resolve!(
				c"vkGetDeviceImageSparseMemoryRequirements",
				PFN_vkGetDeviceImageSparseMemoryRequirements
			),
			get_device_image_sparse_memory_requirements_khr: resolve!(
				c"vkGetDeviceImageSparseMemoryRequirementsKHR",
				PFN_vkGetDeviceImageSparseMemoryRequirementsKHR
			),
			create_sampler_ycbcr_conversion: resolve!(
				c"vkCreateSamplerYcbcrConversion",
				PFN_vkCreateSamplerYcbcrConversion
			),
			create_sampler_ycbcr_conversion_khr: resolve!(
				c"vkCreateSamplerYcbcrConversionKHR",
				PFN_vkCreateSamplerYcbcrConversionKHR
			),
			destroy_sampler_ycbcr_conversion: resolve!(
				c"vkDestroySamplerYcbcrConversion",
				PFN_vkDestroySamplerYcbcrConversion
			),
			destroy_sampler_ycbcr_conversion_khr: resolve!(
				c"vkDestroySamplerYcbcrConversionKHR",
				PFN_vkDestroySamplerYcbcrConversionKHR
			),
			get_device_queue2: resolve!(c"vkGetDeviceQueue2", PFN_vkGetDeviceQueue2),
			create_validation_cache_ext: resolve!(
				c"vkCreateValidationCacheEXT",
				PFN_vkCreateValidationCacheEXT
			),
			destroy_validation_cache_ext: resolve!(
				c"vkDestroyValidationCacheEXT",
				PFN_vkDestroyValidationCacheEXT
			),
			get_validation_cache_data_ext: resolve!(
				c"vkGetValidationCacheDataEXT",
				PFN_vkGetValidationCacheDataEXT
			),
			merge_validation_caches_ext: resolve!(
				c"vkMergeValidationCachesEXT",
				PFN_vkMergeValidationCachesEXT
			),
			get_descriptor_set_layout_support: resolve!(
				c"vkGetDescriptorSetLayoutSupport",
				PFN_vkGetDescriptorSetLayoutSupport
			),
			get_descriptor_set_layout_support_khr: resolve!(
				c"vkGetDescriptorSetLayoutSupportKHR",
				PFN_vkGetDescriptorSetLayoutSupportKHR
			),
			get_shader_info_amd: resolve!(c"vkGetShaderInfoAMD", PFN_vkGetShaderInfoAMD),
			set_local_dimming_amd: resolve!(c"vkSetLocalDimmingAMD", PFN_vkSetLocalDimmingAMD),
			get_calibrated_timestamps_khr: resolve!(
				c"vkGetCalibratedTimestampsKHR",
				PFN_vkGetCalibratedTimestampsKHR
			),
			get_calibrated_timestamps_ext: resolve!(
				c"vkGetCalibratedTimestampsEXT",
				PFN_vkGetCalibratedTimestampsEXT
			),
			set_debug_utils_object_name_ext: resolve!(
				c"vkSetDebugUtilsObjectNameEXT",
				PFN_vkSetDebugUtilsObjectNameEXT
			),
			set_debug_utils_object_tag_ext: resolve!(
				c"vkSetDebugUtilsObjectTagEXT",
				PFN_vkSetDebugUtilsObjectTagEXT
			),
			queue_begin_debug_utils_label_ext: resolve!(
				c"vkQueueBeginDebugUtilsLabelEXT",
				PFN_vkQueueBeginDebugUtilsLabelEXT
			),
			queue_end_debug_utils_label_ext: resolve!(
				c"vkQueueEndDebugUtilsLabelEXT",
				PFN_vkQueueEndDebugUtilsLabelEXT
			),
			queue_insert_debug_utils_label_ext: resolve!(
				c"vkQueueInsertDebugUtilsLabelEXT",
				PFN_vkQueueInsertDebugUtilsLabelEXT
			),
			cmd_begin_debug_utils_label_ext: resolve!(
				c"vkCmdBeginDebugUtilsLabelEXT",
				PFN_vkCmdBeginDebugUtilsLabelEXT
			),
			cmd_end_debug_utils_label_ext: resolve!(
				c"vkCmdEndDebugUtilsLabelEXT",
				PFN_vkCmdEndDebugUtilsLabelEXT
			),
			cmd_insert_debug_utils_label_ext: resolve!(
				c"vkCmdInsertDebugUtilsLabelEXT",
				PFN_vkCmdInsertDebugUtilsLabelEXT
			),
			get_memory_host_pointer_properties_ext: resolve!(
				c"vkGetMemoryHostPointerPropertiesEXT",
				PFN_vkGetMemoryHostPointerPropertiesEXT
			),
			cmd_write_buffer_marker_amd: resolve!(
				c"vkCmdWriteBufferMarkerAMD",
				PFN_vkCmdWriteBufferMarkerAMD
			),
			create_render_pass2: resolve!(c"vkCreateRenderPass2", PFN_vkCreateRenderPass2),
			create_render_pass2_khr: resolve!(c"vkCreateRenderPass2KHR", PFN_vkCreateRenderPass2KHR),
			cmd_begin_render_pass2: resolve!(c"vkCmdBeginRenderPass2", PFN_vkCmdBeginRenderPass2),
			cmd_begin_render_pass2_khr: resolve!(
				c"vkCmdBeginRenderPass2KHR",
				PFN_vkCmdBeginRenderPass2KHR
			),
			cmd_next_subpass2: resolve!(c"vkCmdNextSubpass2", PFN_vkCmdNextSubpass2),
			cmd_next_subpass2_khr: resolve!(c"vkCmdNextSubpass2KHR", PFN_vkCmdNextSubpass2KHR),
			cmd_end_render_pass2: resolve!(c"vkCmdEndRenderPass2", PFN_vkCmdEndRenderPass2),
			cmd_end_render_pass2_khr: resolve!(c"vkCmdEndRenderPass2KHR", PFN_vkCmdEndRenderPass2KHR),
			get_semaphore_counter_value: resolve!(
				c"vkGetSemaphoreCounterValue",
				PFN_vkGetSemaphoreCounterValue
			),
			get_semaphore_counter_value_khr: resolve!(
				c"vkGetSemaphoreCounterValueKHR",
				PFN_vkGetSemaphoreCounterValueKHR
			),
			wait_semaphores: resolve!(c"vkWaitSemaphores", PFN_vkWaitSemaphores),
			wait_semaphores_khr: resolve!(c"vkWaitSemaphoresKHR", PFN_vkWaitSemaphoresKHR),
			signal_semaphore: resolve!(c"vkSignalSemaphore", PFN_vkSignalSemaphore),
			signal_semaphore_khr: resolve!(c"vkSignalSemaphoreKHR", PFN_vkSignalSemaphoreKHR),
			#[cfg(feature = "platform-android")]
			get_android_hardware_buffer_properties_android: resolve!(
				c"vkGetAndroidHardwareBufferPropertiesANDROID",
				PFN_vkGetAndroidHardwareBufferPropertiesANDROID
			),
			#[cfg(feature = "platform-android")]
			get_memory_android_hardware_buffer_android: resolve!(
				c"vkGetMemoryAndroidHardwareBufferANDROID",
				PFN_vkGetMemoryAndroidHardwareBufferANDROID
			),
			cmd_draw_indirect_count: resolve!(c"vkCmdDrawIndirectCount", PFN_vkCmdDrawIndirectCount),
			cmd_draw_indirect_count_khr: resolve!(
				c"vkCmdDrawIndirectCountKHR",
				PFN_vkCmdDrawIndirectCountKHR
			),
			cmd_draw_indirect_count_amd: resolve!(
				c"vkCmdDrawIndirectCountAMD",
				PFN_vkCmdDrawIndirectCountAMD
			),
			cmd_draw_indexed_indirect_count: resolve!(
				c"vkCmdDrawIndexedIndirectCount",
				PFN_vkCmdDrawIndexedIndirectCount
			),
			cmd_draw_indexed_indirect_count_khr: resolve!(
				c"vkCmdDrawIndexedIndirectCountKHR",
				PFN_vkCmdDrawIndexedIndirectCountKHR
			),
			cmd_draw_indexed_indirect_count_amd: resolve!(
				c"vkCmdDrawIndexedIndirectCountAMD",
				PFN_vkCmdDrawIndexedIndirectCountAMD
			),
			cmd_set_checkpoint_nv: resolve!(c"vkCmdSetCheckpointNV", PFN_vkCmdSetCheckpointNV),
			get_queue_checkpoint_data_nv: resolve!(
				c"vkGetQueueCheckpointDataNV",
				PFN_vkGetQueueCheckpointDataNV
			),
			cmd_bind_transform_feedback_buffers_ext: resolve!(
				c"vkCmdBindTransformFeedbackBuffersEXT",
				PFN_vkCmdBindTransformFeedbackBuffersEXT
			),
			cmd_begin_transform_feedback_ext: resolve!(
				c"vkCmdBeginTransformFeedbackEXT",
				PFN_vkCmdBeginTransformFeedbackEXT
			),
			cmd_end_transform_feedback_ext: resolve!(
				c"vkCmdEndTransformFeedbackEXT",
				PFN_vkCmdEndTransformFeedbackEXT
			),
			cmd_begin_query_indexed_ext: resolve!(
				c"vkCmdBeginQueryIndexedEXT",
				PFN_vkCmdBeginQueryIndexedEXT
			),
			cmd_end_query_indexed_ext: resolve!(c"vkCmdEndQueryIndexedEXT", PFN_vkCmdEndQueryIndexedEXT),
			cmd_draw_indirect_byte_count_ext: resolve!(
				c"vkCmdDrawIndirectByteCountEXT",
				PFN_vkCmdDrawIndirectByteCountEXT
			),
			cmd_set_exclusive_scissor_nv: resolve!(
				c"vkCmdSetExclusiveScissorNV",
				PFN_vkCmdSetExclusiveScissorNV
			),
			cmd_set_exclusive_scissor_enable_nv: resolve!(
				c"vkCmdSetExclusiveScissorEnableNV",
				PFN_vkCmdSetExclusiveScissorEnableNV
			),
			cmd_bind_shading_rate_image_nv: resolve!(
				c"vkCmdBindShadingRateImageNV",
				PFN_vkCmdBindShadingRateImageNV
			),
			cmd_set_viewport_shading_rate_palette_nv: resolve!(
				c"vkCmdSetViewportShadingRatePaletteNV",
				PFN_vkCmdSetViewportShadingRatePaletteNV
			),
			cmd_set_coarse_sample_order_nv: resolve!(
				c"vkCmdSetCoarseSampleOrderNV",
				PFN_vkCmdSetCoarseSampleOrderNV
			),
			cmd_draw_mesh_tasks_nv: resolve!(c"vkCmdDrawMeshTasksNV", PFN_vkCmdDrawMeshTasksNV),
			cmd_draw_mesh_tasks_indirect_nv: resolve!(
				c"vkCmdDrawMeshTasksIndirectNV",
				PFN_vkCmdDrawMeshTasksIndirectNV
			),
			cmd_draw_mesh_tasks_indirect_count_nv: resolve!(
				c"vkCmdDrawMeshTasksIndirectCountNV",
				PFN_vkCmdDrawMeshTasksIndirectCountNV
			),
			cmd_draw_mesh_tasks_ext: resolve!(c"vkCmdDrawMeshTasksEXT", PFN_vkCmdDrawMeshTasksEXT),
			cmd_draw_mesh_tasks_indirect_ext: resolve!(
				c"vkCmdDrawMeshTasksIndirectEXT",
				PFN_vkCmdDrawMeshTasksIndirectEXT
			),
			cmd_draw_mesh_tasks_indirect_count_ext: resolve!(
				c"vkCmdDrawMeshTasksIndirectCountEXT",
				PFN_vkCmdDrawMeshTasksIndirectCountEXT
			),
			compile_deferred_nv: resolve!(c"vkCompileDeferredNV", PFN_vkCompileDeferredNV),
			create_acceleration_structure_nv: resolve!(
				c"vkCreateAccelerationStructureNV",
				PFN_vkCreateAccelerationStructureNV
			),
			cmd_bind_invocation_mask_huawei: resolve!(
				c"vkCmdBindInvocationMaskHUAWEI",
				PFN_vkCmdBindInvocationMaskHUAWEI
			),
			destroy_acceleration_structure_khr: resolve!(
				c"vkDestroyAccelerationStructureKHR",
				PFN_vkDestroyAccelerationStructureKHR
			),
			destroy_acceleration_structure_nv: resolve!(
				c"vkDestroyAccelerationStructureNV",
				PFN_vkDestroyAccelerationStructureNV
			),
			get_acceleration_structure_memory_requirements_nv: resolve!(
				c"vkGetAccelerationStructureMemoryRequirementsNV",
				PFN_vkGetAccelerationStructureMemoryRequirementsNV
			),
			bind_acceleration_structure_memory_nv: resolve!(
				c"vkBindAccelerationStructureMemoryNV",
				PFN_vkBindAccelerationStructureMemoryNV
			),
			cmd_copy_acceleration_structure_nv: resolve!(
				c"vkCmdCopyAccelerationStructureNV",
				PFN_vkCmdCopyAccelerationStructureNV
			),
			cmd_copy_acceleration_structure_khr: resolve!(
				c"vkCmdCopyAccelerationStructureKHR",
				PFN_vkCmdCopyAccelerationStructureKHR
			),
			copy_acceleration_structure_khr: resolve!(
				c"vkCopyAccelerationStructureKHR",
				PFN_vkCopyAccelerationStructureKHR
			),
			cmd_copy_acceleration_structure_to_memory_khr: resolve!(
				c"vkCmdCopyAccelerationStructureToMemoryKHR",
				PFN_vkCmdCopyAccelerationStructureToMemoryKHR
			),
			copy_acceleration_structure_to_memory_khr: resolve!(
				c"vkCopyAccelerationStructureToMemoryKHR",
				PFN_vkCopyAccelerationStructureToMemoryKHR
			),
			cmd_copy_memory_to_acceleration_structure_khr: resolve!(
				c"vkCmdCopyMemoryToAccelerationStructureKHR",
				PFN_vkCmdCopyMemoryToAccelerationStructureKHR
			),
			copy_memory_to_acceleration_structure_khr: resolve!(
				c"vkCopyMemoryToAccelerationStructureKHR",
				PFN_vkCopyMemoryToAccelerationStructureKHR
			),
			cmd_write_acceleration_structures_properties_khr: resolve!(
				c"vkCmdWriteAccelerationStructuresPropertiesKHR",
				PFN_vkCmdWriteAccelerationStructuresPropertiesKHR
			),
			cmd_write_acceleration_structures_properties_nv: resolve!(
				c"vkCmdWriteAccelerationStructuresPropertiesNV",
				PFN_vkCmdWriteAccelerationStructuresPropertiesNV
			),
			cmd_build_acceleration_structure_nv: resolve!(
				c"vkCmdBuildAccelerationStructureNV",
				PFN_vkCmdBuildAccelerationStructureNV
			),
			write_acceleration_structures_properties_khr: resolve!(
				c"vkWriteAccelerationStructuresPropertiesKHR",
				PFN_vkWriteAccelerationStructuresPropertiesKHR
			),
			cmd_trace_rays_khr: resolve!(c"vkCmdTraceRaysKHR", PFN_vkCmdTraceRaysKHR),
			cmd_trace_rays_nv: resolve!(c"vkCmdTraceRaysNV", PFN_vkCmdTraceRaysNV),
			get_ray_tracing_shader_group_handles_khr: resolve!(
				c"vkGetRayTracingShaderGroupHandlesKHR",
				PFN_vkGetRayTracingShaderGroupHandlesKHR
			),
			get_ray_tracing_shader_group_handles_nv: resolve!(
				c"vkGetRayTracingShaderGroupHandlesNV",
				PFN_vkGetRayTracingShaderGroupHandlesNV
			),
			get_ray_tracing_capture_replay_shader_group_handles_khr: resolve!(
				c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR",
				PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR
			),
			get_acceleration_structure_handle_nv: resolve!(
				c"vkGetAccelerationStructureHandleNV",
				PFN_vkGetAccelerationStructureHandleNV
			),
			create_ray_tracing_pipelines_nv: resolve!(
				c"vkCreateRayTracingPipelinesNV",
				PFN_vkCreateRayTracingPipelinesNV
			),
			create_ray_tracing_pipelines_khr: resolve!(
				c"vkCreateRayTracingPipelinesKHR",
				PFN_vkCreateRayTracingPipelinesKHR
			),
			cmd_trace_rays_indirect_khr: resolve!(
				c"vkCmdTraceRaysIndirectKHR",
				PFN_vkCmdTraceRaysIndirectKHR
			),
			cmd_trace_rays_indirect2_khr: resolve!(
				c"vkCmdTraceRaysIndirect2KHR",
				PFN_vkCmdTraceRaysIndirect2KHR
			),
			get_cluster_acceleration_structure_build_sizes_nv: resolve!(
				c"vkGetClusterAccelerationStructureBuildSizesNV",
				PFN_vkGetClusterAccelerationStructureBuildSizesNV
			),
			cmd_build_cluster_acceleration_structure_indirect_nv: resolve!(
				c"vkCmdBuildClusterAccelerationStructureIndirectNV",
				PFN_vkCmdBuildClusterAccelerationStructureIndirectNV
			),
			get_device_acceleration_structure_compatibility_khr: resolve!(
				c"vkGetDeviceAccelerationStructureCompatibilityKHR",
				PFN_vkGetDeviceAccelerationStructureCompatibilityKHR
			),
			get_ray_tracing_shader_group_stack_size_khr: resolve!(
				c"vkGetRayTracingShaderGroupStackSizeKHR",
				PFN_vkGetRayTracingShaderGroupStackSizeKHR
			),
			cmd_set_ray_tracing_pipeline_stack_size_khr: resolve!(
				c"vkCmdSetRayTracingPipelineStackSizeKHR",
				PFN_vkCmdSetRayTracingPipelineStackSizeKHR
			),
			get_image_view_handle_nvx: resolve!(c"vkGetImageViewHandleNVX", PFN_vkGetImageViewHandleNVX),
			get_image_view_handle64_nvx: resolve!(
				c"vkGetImageViewHandle64NVX",
				PFN_vkGetImageViewHandle64NVX
			),
			get_image_view_address_nvx: resolve!(
				c"vkGetImageViewAddressNVX",
				PFN_vkGetImageViewAddressNVX
			),
			get_device_combined_image_sampler_index_nvx: resolve!(
				c"vkGetDeviceCombinedImageSamplerIndexNVX",
				PFN_vkGetDeviceCombinedImageSamplerIndexNVX
			),
			#[cfg(feature = "platform-win32")]
			get_device_group_surface_present_modes2_ext: resolve!(
				c"vkGetDeviceGroupSurfacePresentModes2EXT",
				PFN_vkGetDeviceGroupSurfacePresentModes2EXT
			),
			#[cfg(feature = "platform-win32")]
			acquire_full_screen_exclusive_mode_ext: resolve!(
				c"vkAcquireFullScreenExclusiveModeEXT",
				PFN_vkAcquireFullScreenExclusiveModeEXT
			),
			#[cfg(feature = "platform-win32")]
			release_full_screen_exclusive_mode_ext: resolve!(
				c"vkReleaseFullScreenExclusiveModeEXT",
				PFN_vkReleaseFullScreenExclusiveModeEXT
			),
			acquire_profiling_lock_khr: resolve!(
				c"vkAcquireProfilingLockKHR",
				PFN_vkAcquireProfilingLockKHR
			),
			release_profiling_lock_khr: resolve!(
				c"vkReleaseProfilingLockKHR",
				PFN_vkReleaseProfilingLockKHR
			),
			get_image_drm_format_modifier_properties_ext: resolve!(
				c"vkGetImageDrmFormatModifierPropertiesEXT",
				PFN_vkGetImageDrmFormatModifierPropertiesEXT
			),
			get_buffer_opaque_capture_address: resolve!(
				c"vkGetBufferOpaqueCaptureAddress",
				PFN_vkGetBufferOpaqueCaptureAddress
			),
			get_buffer_opaque_capture_address_khr: resolve!(
				c"vkGetBufferOpaqueCaptureAddressKHR",
				PFN_vkGetBufferOpaqueCaptureAddressKHR
			),
			get_buffer_device_address: resolve!(
				c"vkGetBufferDeviceAddress",
				PFN_vkGetBufferDeviceAddress
			),
			get_buffer_device_address_khr: resolve!(
				c"vkGetBufferDeviceAddressKHR",
				PFN_vkGetBufferDeviceAddressKHR
			),
			get_buffer_device_address_ext: resolve!(
				c"vkGetBufferDeviceAddressEXT",
				PFN_vkGetBufferDeviceAddressEXT
			),
			initialize_performance_api_intel: resolve!(
				c"vkInitializePerformanceApiINTEL",
				PFN_vkInitializePerformanceApiINTEL
			),
			uninitialize_performance_api_intel: resolve!(
				c"vkUninitializePerformanceApiINTEL",
				PFN_vkUninitializePerformanceApiINTEL
			),
			cmd_set_performance_marker_intel: resolve!(
				c"vkCmdSetPerformanceMarkerINTEL",
				PFN_vkCmdSetPerformanceMarkerINTEL
			),
			cmd_set_performance_stream_marker_intel: resolve!(
				c"vkCmdSetPerformanceStreamMarkerINTEL",
				PFN_vkCmdSetPerformanceStreamMarkerINTEL
			),
			cmd_set_performance_override_intel: resolve!(
				c"vkCmdSetPerformanceOverrideINTEL",
				PFN_vkCmdSetPerformanceOverrideINTEL
			),
			acquire_performance_configuration_intel: resolve!(
				c"vkAcquirePerformanceConfigurationINTEL",
				PFN_vkAcquirePerformanceConfigurationINTEL
			),
			release_performance_configuration_intel: resolve!(
				c"vkReleasePerformanceConfigurationINTEL",
				PFN_vkReleasePerformanceConfigurationINTEL
			),
			queue_set_performance_configuration_intel: resolve!(
				c"vkQueueSetPerformanceConfigurationINTEL",
				PFN_vkQueueSetPerformanceConfigurationINTEL
			),
			get_performance_parameter_intel: resolve!(
				c"vkGetPerformanceParameterINTEL",
				PFN_vkGetPerformanceParameterINTEL
			),
			get_device_memory_opaque_capture_address: resolve!(
				c"vkGetDeviceMemoryOpaqueCaptureAddress",
				PFN_vkGetDeviceMemoryOpaqueCaptureAddress
			),
			get_device_memory_opaque_capture_address_khr: resolve!(
				c"vkGetDeviceMemoryOpaqueCaptureAddressKHR",
				PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR
			),
			get_pipeline_executable_properties_khr: resolve!(
				c"vkGetPipelineExecutablePropertiesKHR",
				PFN_vkGetPipelineExecutablePropertiesKHR
			),
			get_pipeline_executable_statistics_khr: resolve!(
				c"vkGetPipelineExecutableStatisticsKHR",
				PFN_vkGetPipelineExecutableStatisticsKHR
			),
			get_pipeline_executable_internal_representations_khr: resolve!(
				c"vkGetPipelineExecutableInternalRepresentationsKHR",
				PFN_vkGetPipelineExecutableInternalRepresentationsKHR
			),
			cmd_set_line_stipple: resolve!(c"vkCmdSetLineStipple", PFN_vkCmdSetLineStipple),
			cmd_set_line_stipple_khr: resolve!(c"vkCmdSetLineStippleKHR", PFN_vkCmdSetLineStippleKHR),
			cmd_set_line_stipple_ext: resolve!(c"vkCmdSetLineStippleEXT", PFN_vkCmdSetLineStippleEXT),
			create_acceleration_structure_khr: resolve!(
				c"vkCreateAccelerationStructureKHR",
				PFN_vkCreateAccelerationStructureKHR
			),
			cmd_build_acceleration_structures_khr: resolve!(
				c"vkCmdBuildAccelerationStructuresKHR",
				PFN_vkCmdBuildAccelerationStructuresKHR
			),
			cmd_build_acceleration_structures_indirect_khr: resolve!(
				c"vkCmdBuildAccelerationStructuresIndirectKHR",
				PFN_vkCmdBuildAccelerationStructuresIndirectKHR
			),
			build_acceleration_structures_khr: resolve!(
				c"vkBuildAccelerationStructuresKHR",
				PFN_vkBuildAccelerationStructuresKHR
			),
			get_acceleration_structure_device_address_khr: resolve!(
				c"vkGetAccelerationStructureDeviceAddressKHR",
				PFN_vkGetAccelerationStructureDeviceAddressKHR
			),
			create_deferred_operation_khr: resolve!(
				c"vkCreateDeferredOperationKHR",
				PFN_vkCreateDeferredOperationKHR
			),
			destroy_deferred_operation_khr: resolve!(
				c"vkDestroyDeferredOperationKHR",
				PFN_vkDestroyDeferredOperationKHR
			),
			get_deferred_operation_max_concurrency_khr: resolve!(
				c"vkGetDeferredOperationMaxConcurrencyKHR",
				PFN_vkGetDeferredOperationMaxConcurrencyKHR
			),
			get_deferred_operation_result_khr: resolve!(
				c"vkGetDeferredOperationResultKHR",
				PFN_vkGetDeferredOperationResultKHR
			),
			deferred_operation_join_khr: resolve!(
				c"vkDeferredOperationJoinKHR",
				PFN_vkDeferredOperationJoinKHR
			),
			get_pipeline_indirect_memory_requirements_nv: resolve!(
				c"vkGetPipelineIndirectMemoryRequirementsNV",
				PFN_vkGetPipelineIndirectMemoryRequirementsNV
			),
			get_pipeline_indirect_device_address_nv: resolve!(
				c"vkGetPipelineIndirectDeviceAddressNV",
				PFN_vkGetPipelineIndirectDeviceAddressNV
			),
			anti_lag_update_amd: resolve!(c"vkAntiLagUpdateAMD", PFN_vkAntiLagUpdateAMD),
			cmd_set_cull_mode: resolve!(c"vkCmdSetCullMode", PFN_vkCmdSetCullMode),
			cmd_set_cull_mode_ext: resolve!(c"vkCmdSetCullModeEXT", PFN_vkCmdSetCullModeEXT),
			cmd_set_front_face: resolve!(c"vkCmdSetFrontFace", PFN_vkCmdSetFrontFace),
			cmd_set_front_face_ext: resolve!(c"vkCmdSetFrontFaceEXT", PFN_vkCmdSetFrontFaceEXT),
			cmd_set_primitive_topology: resolve!(
				c"vkCmdSetPrimitiveTopology",
				PFN_vkCmdSetPrimitiveTopology
			),
			cmd_set_primitive_topology_ext: resolve!(
				c"vkCmdSetPrimitiveTopologyEXT",
				PFN_vkCmdSetPrimitiveTopologyEXT
			),
			cmd_set_viewport_with_count: resolve!(
				c"vkCmdSetViewportWithCount",
				PFN_vkCmdSetViewportWithCount
			),
			cmd_set_viewport_with_count_ext: resolve!(
				c"vkCmdSetViewportWithCountEXT",
				PFN_vkCmdSetViewportWithCountEXT
			),
			cmd_set_scissor_with_count: resolve!(
				c"vkCmdSetScissorWithCount",
				PFN_vkCmdSetScissorWithCount
			),
			cmd_set_scissor_with_count_ext: resolve!(
				c"vkCmdSetScissorWithCountEXT",
				PFN_vkCmdSetScissorWithCountEXT
			),
			cmd_bind_index_buffer2: resolve!(c"vkCmdBindIndexBuffer2", PFN_vkCmdBindIndexBuffer2),
			cmd_bind_index_buffer2_khr: resolve!(
				c"vkCmdBindIndexBuffer2KHR",
				PFN_vkCmdBindIndexBuffer2KHR
			),
			cmd_bind_vertex_buffers2: resolve!(c"vkCmdBindVertexBuffers2", PFN_vkCmdBindVertexBuffers2),
			cmd_bind_vertex_buffers2_ext: resolve!(
				c"vkCmdBindVertexBuffers2EXT",
				PFN_vkCmdBindVertexBuffers2EXT
			),
			cmd_set_depth_test_enable: resolve!(c"vkCmdSetDepthTestEnable", PFN_vkCmdSetDepthTestEnable),
			cmd_set_depth_test_enable_ext: resolve!(
				c"vkCmdSetDepthTestEnableEXT",
				PFN_vkCmdSetDepthTestEnableEXT
			),
			cmd_set_depth_write_enable: resolve!(
				c"vkCmdSetDepthWriteEnable",
				PFN_vkCmdSetDepthWriteEnable
			),
			cmd_set_depth_write_enable_ext: resolve!(
				c"vkCmdSetDepthWriteEnableEXT",
				PFN_vkCmdSetDepthWriteEnableEXT
			),
			cmd_set_depth_compare_op: resolve!(c"vkCmdSetDepthCompareOp", PFN_vkCmdSetDepthCompareOp),
			cmd_set_depth_compare_op_ext: resolve!(
				c"vkCmdSetDepthCompareOpEXT",
				PFN_vkCmdSetDepthCompareOpEXT
			),
			cmd_set_depth_bounds_test_enable: resolve!(
				c"vkCmdSetDepthBoundsTestEnable",
				PFN_vkCmdSetDepthBoundsTestEnable
			),
			cmd_set_depth_bounds_test_enable_ext: resolve!(
				c"vkCmdSetDepthBoundsTestEnableEXT",
				PFN_vkCmdSetDepthBoundsTestEnableEXT
			),
			cmd_set_stencil_test_enable: resolve!(
				c"vkCmdSetStencilTestEnable",
				PFN_vkCmdSetStencilTestEnable
			),
			cmd_set_stencil_test_enable_ext: resolve!(
				c"vkCmdSetStencilTestEnableEXT",
				PFN_vkCmdSetStencilTestEnableEXT
			),
			cmd_set_stencil_op: resolve!(c"vkCmdSetStencilOp", PFN_vkCmdSetStencilOp),
			cmd_set_stencil_op_ext: resolve!(c"vkCmdSetStencilOpEXT", PFN_vkCmdSetStencilOpEXT),
			cmd_set_patch_control_points_ext: resolve!(
				c"vkCmdSetPatchControlPointsEXT",
				PFN_vkCmdSetPatchControlPointsEXT
			),
			cmd_set_rasterizer_discard_enable: resolve!(
				c"vkCmdSetRasterizerDiscardEnable",
				PFN_vkCmdSetRasterizerDiscardEnable
			),
			cmd_set_rasterizer_discard_enable_ext: resolve!(
				c"vkCmdSetRasterizerDiscardEnableEXT",
				PFN_vkCmdSetRasterizerDiscardEnableEXT
			),
			cmd_set_depth_bias_enable: resolve!(c"vkCmdSetDepthBiasEnable", PFN_vkCmdSetDepthBiasEnable),
			cmd_set_depth_bias_enable_ext: resolve!(
				c"vkCmdSetDepthBiasEnableEXT",
				PFN_vkCmdSetDepthBiasEnableEXT
			),
			cmd_set_logic_op_ext: resolve!(c"vkCmdSetLogicOpEXT", PFN_vkCmdSetLogicOpEXT),
			cmd_set_primitive_restart_enable: resolve!(
				c"vkCmdSetPrimitiveRestartEnable",
				PFN_vkCmdSetPrimitiveRestartEnable
			),
			cmd_set_primitive_restart_enable_ext: resolve!(
				c"vkCmdSetPrimitiveRestartEnableEXT",
				PFN_vkCmdSetPrimitiveRestartEnableEXT
			),
			cmd_set_tessellation_domain_origin_ext: resolve!(
				c"vkCmdSetTessellationDomainOriginEXT",
				PFN_vkCmdSetTessellationDomainOriginEXT
			),
			cmd_set_depth_clamp_enable_ext: resolve!(
				c"vkCmdSetDepthClampEnableEXT",
				PFN_vkCmdSetDepthClampEnableEXT
			),
			cmd_set_polygon_mode_ext: resolve!(c"vkCmdSetPolygonModeEXT", PFN_vkCmdSetPolygonModeEXT),
			cmd_set_rasterization_samples_ext: resolve!(
				c"vkCmdSetRasterizationSamplesEXT",
				PFN_vkCmdSetRasterizationSamplesEXT
			),
			cmd_set_sample_mask_ext: resolve!(c"vkCmdSetSampleMaskEXT", PFN_vkCmdSetSampleMaskEXT),
			cmd_set_alpha_to_coverage_enable_ext: resolve!(
				c"vkCmdSetAlphaToCoverageEnableEXT",
				PFN_vkCmdSetAlphaToCoverageEnableEXT
			),
			cmd_set_alpha_to_one_enable_ext: resolve!(
				c"vkCmdSetAlphaToOneEnableEXT",
				PFN_vkCmdSetAlphaToOneEnableEXT
			),
			cmd_set_logic_op_enable_ext: resolve!(
				c"vkCmdSetLogicOpEnableEXT",
				PFN_vkCmdSetLogicOpEnableEXT
			),
			cmd_set_color_blend_enable_ext: resolve!(
				c"vkCmdSetColorBlendEnableEXT",
				PFN_vkCmdSetColorBlendEnableEXT
			),
			cmd_set_color_blend_equation_ext: resolve!(
				c"vkCmdSetColorBlendEquationEXT",
				PFN_vkCmdSetColorBlendEquationEXT
			),
			cmd_set_color_write_mask_ext: resolve!(
				c"vkCmdSetColorWriteMaskEXT",
				PFN_vkCmdSetColorWriteMaskEXT
			),
			cmd_set_rasterization_stream_ext: resolve!(
				c"vkCmdSetRasterizationStreamEXT",
				PFN_vkCmdSetRasterizationStreamEXT
			),
			cmd_set_conservative_rasterization_mode_ext: resolve!(
				c"vkCmdSetConservativeRasterizationModeEXT",
				PFN_vkCmdSetConservativeRasterizationModeEXT
			),
			cmd_set_extra_primitive_overestimation_size_ext: resolve!(
				c"vkCmdSetExtraPrimitiveOverestimationSizeEXT",
				PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT
			),
			cmd_set_depth_clip_enable_ext: resolve!(
				c"vkCmdSetDepthClipEnableEXT",
				PFN_vkCmdSetDepthClipEnableEXT
			),
			cmd_set_sample_locations_enable_ext: resolve!(
				c"vkCmdSetSampleLocationsEnableEXT",
				PFN_vkCmdSetSampleLocationsEnableEXT
			),
			cmd_set_color_blend_advanced_ext: resolve!(
				c"vkCmdSetColorBlendAdvancedEXT",
				PFN_vkCmdSetColorBlendAdvancedEXT
			),
			cmd_set_provoking_vertex_mode_ext: resolve!(
				c"vkCmdSetProvokingVertexModeEXT",
				PFN_vkCmdSetProvokingVertexModeEXT
			),
			cmd_set_line_rasterization_mode_ext: resolve!(
				c"vkCmdSetLineRasterizationModeEXT",
				PFN_vkCmdSetLineRasterizationModeEXT
			),
			cmd_set_line_stipple_enable_ext: resolve!(
				c"vkCmdSetLineStippleEnableEXT",
				PFN_vkCmdSetLineStippleEnableEXT
			),
			cmd_set_depth_clip_negative_one_to_one_ext: resolve!(
				c"vkCmdSetDepthClipNegativeOneToOneEXT",
				PFN_vkCmdSetDepthClipNegativeOneToOneEXT
			),
			cmd_set_viewport_w_scaling_enable_nv: resolve!(
				c"vkCmdSetViewportWScalingEnableNV",
				PFN_vkCmdSetViewportWScalingEnableNV
			),
			cmd_set_viewport_swizzle_nv: resolve!(
				c"vkCmdSetViewportSwizzleNV",
				PFN_vkCmdSetViewportSwizzleNV
			),
			cmd_set_coverage_to_color_enable_nv: resolve!(
				c"vkCmdSetCoverageToColorEnableNV",
				PFN_vkCmdSetCoverageToColorEnableNV
			),
			cmd_set_coverage_to_color_location_nv: resolve!(
				c"vkCmdSetCoverageToColorLocationNV",
				PFN_vkCmdSetCoverageToColorLocationNV
			),
			cmd_set_coverage_modulation_mode_nv: resolve!(
				c"vkCmdSetCoverageModulationModeNV",
				PFN_vkCmdSetCoverageModulationModeNV
			),
			cmd_set_coverage_modulation_table_enable_nv: resolve!(
				c"vkCmdSetCoverageModulationTableEnableNV",
				PFN_vkCmdSetCoverageModulationTableEnableNV
			),
			cmd_set_coverage_modulation_table_nv: resolve!(
				c"vkCmdSetCoverageModulationTableNV",
				PFN_vkCmdSetCoverageModulationTableNV
			),
			cmd_set_shading_rate_image_enable_nv: resolve!(
				c"vkCmdSetShadingRateImageEnableNV",
				PFN_vkCmdSetShadingRateImageEnableNV
			),
			cmd_set_coverage_reduction_mode_nv: resolve!(
				c"vkCmdSetCoverageReductionModeNV",
				PFN_vkCmdSetCoverageReductionModeNV
			),
			cmd_set_representative_fragment_test_enable_nv: resolve!(
				c"vkCmdSetRepresentativeFragmentTestEnableNV",
				PFN_vkCmdSetRepresentativeFragmentTestEnableNV
			),
			create_private_data_slot: resolve!(c"vkCreatePrivateDataSlot", PFN_vkCreatePrivateDataSlot),
			create_private_data_slot_ext: resolve!(
				c"vkCreatePrivateDataSlotEXT",
				PFN_vkCreatePrivateDataSlotEXT
			),
			destroy_private_data_slot: resolve!(
				c"vkDestroyPrivateDataSlot",
				PFN_vkDestroyPrivateDataSlot
			),
			destroy_private_data_slot_ext: resolve!(
				c"vkDestroyPrivateDataSlotEXT",
				PFN_vkDestroyPrivateDataSlotEXT
			),
			set_private_data: resolve!(c"vkSetPrivateData", PFN_vkSetPrivateData),
			set_private_data_ext: resolve!(c"vkSetPrivateDataEXT", PFN_vkSetPrivateDataEXT),
			get_private_data: resolve!(c"vkGetPrivateData", PFN_vkGetPrivateData),
			get_private_data_ext: resolve!(c"vkGetPrivateDataEXT", PFN_vkGetPrivateDataEXT),
			cmd_copy_buffer2: resolve!(c"vkCmdCopyBuffer2", PFN_vkCmdCopyBuffer2),
			cmd_copy_buffer2_khr: resolve!(c"vkCmdCopyBuffer2KHR", PFN_vkCmdCopyBuffer2KHR),
			cmd_copy_image2: resolve!(c"vkCmdCopyImage2", PFN_vkCmdCopyImage2),
			cmd_copy_image2_khr: resolve!(c"vkCmdCopyImage2KHR", PFN_vkCmdCopyImage2KHR),
			cmd_blit_image2: resolve!(c"vkCmdBlitImage2", PFN_vkCmdBlitImage2),
			cmd_blit_image2_khr: resolve!(c"vkCmdBlitImage2KHR", PFN_vkCmdBlitImage2KHR),
			cmd_copy_buffer_to_image2: resolve!(c"vkCmdCopyBufferToImage2", PFN_vkCmdCopyBufferToImage2),
			cmd_copy_buffer_to_image2_khr: resolve!(
				c"vkCmdCopyBufferToImage2KHR",
				PFN_vkCmdCopyBufferToImage2KHR
			),
			cmd_copy_image_to_buffer2: resolve!(c"vkCmdCopyImageToBuffer2", PFN_vkCmdCopyImageToBuffer2),
			cmd_copy_image_to_buffer2_khr: resolve!(
				c"vkCmdCopyImageToBuffer2KHR",
				PFN_vkCmdCopyImageToBuffer2KHR
			),
			cmd_resolve_image2: resolve!(c"vkCmdResolveImage2", PFN_vkCmdResolveImage2),
			cmd_resolve_image2_khr: resolve!(c"vkCmdResolveImage2KHR", PFN_vkCmdResolveImage2KHR),
			cmd_set_fragment_shading_rate_khr: resolve!(
				c"vkCmdSetFragmentShadingRateKHR",
				PFN_vkCmdSetFragmentShadingRateKHR
			),
			cmd_set_fragment_shading_rate_enum_nv: resolve!(
				c"vkCmdSetFragmentShadingRateEnumNV",
				PFN_vkCmdSetFragmentShadingRateEnumNV
			),
			get_acceleration_structure_build_sizes_khr: resolve!(
				c"vkGetAccelerationStructureBuildSizesKHR",
				PFN_vkGetAccelerationStructureBuildSizesKHR
			),
			cmd_set_vertex_input_ext: resolve!(c"vkCmdSetVertexInputEXT", PFN_vkCmdSetVertexInputEXT),
			cmd_set_color_write_enable_ext: resolve!(
				c"vkCmdSetColorWriteEnableEXT",
				PFN_vkCmdSetColorWriteEnableEXT
			),
			cmd_set_event2: resolve!(c"vkCmdSetEvent2", PFN_vkCmdSetEvent2),
			cmd_set_event2_khr: resolve!(c"vkCmdSetEvent2KHR", PFN_vkCmdSetEvent2KHR),
			cmd_reset_event2: resolve!(c"vkCmdResetEvent2", PFN_vkCmdResetEvent2),
			cmd_reset_event2_khr: resolve!(c"vkCmdResetEvent2KHR", PFN_vkCmdResetEvent2KHR),
			cmd_wait_events2: resolve!(c"vkCmdWaitEvents2", PFN_vkCmdWaitEvents2),
			cmd_wait_events2_khr: resolve!(c"vkCmdWaitEvents2KHR", PFN_vkCmdWaitEvents2KHR),
			cmd_pipeline_barrier2: resolve!(c"vkCmdPipelineBarrier2", PFN_vkCmdPipelineBarrier2),
			cmd_pipeline_barrier2_khr: resolve!(
				c"vkCmdPipelineBarrier2KHR",
				PFN_vkCmdPipelineBarrier2KHR
			),
			queue_submit2: resolve!(c"vkQueueSubmit2", PFN_vkQueueSubmit2),
			queue_submit2_khr: resolve!(c"vkQueueSubmit2KHR", PFN_vkQueueSubmit2KHR),
			cmd_write_timestamp2: resolve!(c"vkCmdWriteTimestamp2", PFN_vkCmdWriteTimestamp2),
			cmd_write_timestamp2_khr: resolve!(c"vkCmdWriteTimestamp2KHR", PFN_vkCmdWriteTimestamp2KHR),
			cmd_write_buffer_marker2_amd: resolve!(
				c"vkCmdWriteBufferMarker2AMD",
				PFN_vkCmdWriteBufferMarker2AMD
			),
			get_queue_checkpoint_data2_nv: resolve!(
				c"vkGetQueueCheckpointData2NV",
				PFN_vkGetQueueCheckpointData2NV
			),
			copy_memory_to_image: resolve!(c"vkCopyMemoryToImage", PFN_vkCopyMemoryToImage),
			copy_memory_to_image_ext: resolve!(c"vkCopyMemoryToImageEXT", PFN_vkCopyMemoryToImageEXT),
			copy_image_to_memory: resolve!(c"vkCopyImageToMemory", PFN_vkCopyImageToMemory),
			copy_image_to_memory_ext: resolve!(c"vkCopyImageToMemoryEXT", PFN_vkCopyImageToMemoryEXT),
			copy_image_to_image: resolve!(c"vkCopyImageToImage", PFN_vkCopyImageToImage),
			copy_image_to_image_ext: resolve!(c"vkCopyImageToImageEXT", PFN_vkCopyImageToImageEXT),
			transition_image_layout: resolve!(c"vkTransitionImageLayout", PFN_vkTransitionImageLayout),
			transition_image_layout_ext: resolve!(
				c"vkTransitionImageLayoutEXT",
				PFN_vkTransitionImageLayoutEXT
			),
			create_video_session_khr: resolve!(c"vkCreateVideoSessionKHR", PFN_vkCreateVideoSessionKHR),
			destroy_video_session_khr: resolve!(
				c"vkDestroyVideoSessionKHR",
				PFN_vkDestroyVideoSessionKHR
			),
			create_video_session_parameters_khr: resolve!(
				c"vkCreateVideoSessionParametersKHR",
				PFN_vkCreateVideoSessionParametersKHR
			),
			update_video_session_parameters_khr: resolve!(
				c"vkUpdateVideoSessionParametersKHR",
				PFN_vkUpdateVideoSessionParametersKHR
			),
			get_encoded_video_session_parameters_khr: resolve!(
				c"vkGetEncodedVideoSessionParametersKHR",
				PFN_vkGetEncodedVideoSessionParametersKHR
			),
			destroy_video_session_parameters_khr: resolve!(
				c"vkDestroyVideoSessionParametersKHR",
				PFN_vkDestroyVideoSessionParametersKHR
			),
			get_video_session_memory_requirements_khr: resolve!(
				c"vkGetVideoSessionMemoryRequirementsKHR",
				PFN_vkGetVideoSessionMemoryRequirementsKHR
			),
			bind_video_session_memory_khr: resolve!(
				c"vkBindVideoSessionMemoryKHR",
				PFN_vkBindVideoSessionMemoryKHR
			),
			cmd_decode_video_khr: resolve!(c"vkCmdDecodeVideoKHR", PFN_vkCmdDecodeVideoKHR),
			cmd_begin_video_coding_khr: resolve!(
				c"vkCmdBeginVideoCodingKHR",
				PFN_vkCmdBeginVideoCodingKHR
			),
			cmd_control_video_coding_khr: resolve!(
				c"vkCmdControlVideoCodingKHR",
				PFN_vkCmdControlVideoCodingKHR
			),
			cmd_end_video_coding_khr: resolve!(c"vkCmdEndVideoCodingKHR", PFN_vkCmdEndVideoCodingKHR),
			cmd_encode_video_khr: resolve!(c"vkCmdEncodeVideoKHR", PFN_vkCmdEncodeVideoKHR),
			cmd_decompress_memory_nv: resolve!(c"vkCmdDecompressMemoryNV", PFN_vkCmdDecompressMemoryNV),
			cmd_decompress_memory_indirect_count_nv: resolve!(
				c"vkCmdDecompressMemoryIndirectCountNV",
				PFN_vkCmdDecompressMemoryIndirectCountNV
			),
			get_partitioned_acceleration_structures_build_sizes_nv: resolve!(
				c"vkGetPartitionedAccelerationStructuresBuildSizesNV",
				PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV
			),
			cmd_build_partitioned_acceleration_structures_nv: resolve!(
				c"vkCmdBuildPartitionedAccelerationStructuresNV",
				PFN_vkCmdBuildPartitionedAccelerationStructuresNV
			),
			cmd_decompress_memory_ext: resolve!(
				c"vkCmdDecompressMemoryEXT",
				PFN_vkCmdDecompressMemoryEXT
			),
			cmd_decompress_memory_indirect_count_ext: resolve!(
				c"vkCmdDecompressMemoryIndirectCountEXT",
				PFN_vkCmdDecompressMemoryIndirectCountEXT
			),
			create_cu_module_nvx: resolve!(c"vkCreateCuModuleNVX", PFN_vkCreateCuModuleNVX),
			create_cu_function_nvx: resolve!(c"vkCreateCuFunctionNVX", PFN_vkCreateCuFunctionNVX),
			destroy_cu_module_nvx: resolve!(c"vkDestroyCuModuleNVX", PFN_vkDestroyCuModuleNVX),
			destroy_cu_function_nvx: resolve!(c"vkDestroyCuFunctionNVX", PFN_vkDestroyCuFunctionNVX),
			cmd_cu_launch_kernel_nvx: resolve!(c"vkCmdCuLaunchKernelNVX", PFN_vkCmdCuLaunchKernelNVX),
			get_descriptor_set_layout_size_ext: resolve!(
				c"vkGetDescriptorSetLayoutSizeEXT",
				PFN_vkGetDescriptorSetLayoutSizeEXT
			),
			get_descriptor_set_layout_binding_offset_ext: resolve!(
				c"vkGetDescriptorSetLayoutBindingOffsetEXT",
				PFN_vkGetDescriptorSetLayoutBindingOffsetEXT
			),
			get_descriptor_ext: resolve!(c"vkGetDescriptorEXT", PFN_vkGetDescriptorEXT),
			cmd_bind_descriptor_buffers_ext: resolve!(
				c"vkCmdBindDescriptorBuffersEXT",
				PFN_vkCmdBindDescriptorBuffersEXT
			),
			cmd_set_descriptor_buffer_offsets_ext: resolve!(
				c"vkCmdSetDescriptorBufferOffsetsEXT",
				PFN_vkCmdSetDescriptorBufferOffsetsEXT
			),
			cmd_bind_descriptor_buffer_embedded_samplers_ext: resolve!(
				c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT",
				PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT
			),
			get_buffer_opaque_capture_descriptor_data_ext: resolve!(
				c"vkGetBufferOpaqueCaptureDescriptorDataEXT",
				PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT
			),
			get_image_opaque_capture_descriptor_data_ext: resolve!(
				c"vkGetImageOpaqueCaptureDescriptorDataEXT",
				PFN_vkGetImageOpaqueCaptureDescriptorDataEXT
			),
			get_image_view_opaque_capture_descriptor_data_ext: resolve!(
				c"vkGetImageViewOpaqueCaptureDescriptorDataEXT",
				PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT
			),
			get_sampler_opaque_capture_descriptor_data_ext: resolve!(
				c"vkGetSamplerOpaqueCaptureDescriptorDataEXT",
				PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT
			),
			get_acceleration_structure_opaque_capture_descriptor_data_ext: resolve!(
				c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
				PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT
			),
			set_device_memory_priority_ext: resolve!(
				c"vkSetDeviceMemoryPriorityEXT",
				PFN_vkSetDeviceMemoryPriorityEXT
			),
			wait_for_present2_khr: resolve!(c"vkWaitForPresent2KHR", PFN_vkWaitForPresent2KHR),
			wait_for_present_khr: resolve!(c"vkWaitForPresentKHR", PFN_vkWaitForPresentKHR),
			#[cfg(feature = "platform-fuchsia")]
			create_buffer_collection_fuchsia: resolve!(
				c"vkCreateBufferCollectionFUCHSIA",
				PFN_vkCreateBufferCollectionFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			set_buffer_collection_buffer_constraints_fuchsia: resolve!(
				c"vkSetBufferCollectionBufferConstraintsFUCHSIA",
				PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			set_buffer_collection_image_constraints_fuchsia: resolve!(
				c"vkSetBufferCollectionImageConstraintsFUCHSIA",
				PFN_vkSetBufferCollectionImageConstraintsFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			destroy_buffer_collection_fuchsia: resolve!(
				c"vkDestroyBufferCollectionFUCHSIA",
				PFN_vkDestroyBufferCollectionFUCHSIA
			),
			#[cfg(feature = "platform-fuchsia")]
			get_buffer_collection_properties_fuchsia: resolve!(
				c"vkGetBufferCollectionPropertiesFUCHSIA",
				PFN_vkGetBufferCollectionPropertiesFUCHSIA
			),
			#[cfg(feature = "provisional")]
			create_cuda_module_nv: resolve!(c"vkCreateCudaModuleNV", PFN_vkCreateCudaModuleNV),
			#[cfg(feature = "provisional")]
			get_cuda_module_cache_nv: resolve!(c"vkGetCudaModuleCacheNV", PFN_vkGetCudaModuleCacheNV),
			#[cfg(feature = "provisional")]
			create_cuda_function_nv: resolve!(c"vkCreateCudaFunctionNV", PFN_vkCreateCudaFunctionNV),
			#[cfg(feature = "provisional")]
			destroy_cuda_module_nv: resolve!(c"vkDestroyCudaModuleNV", PFN_vkDestroyCudaModuleNV),
			#[cfg(feature = "provisional")]
			destroy_cuda_function_nv: resolve!(c"vkDestroyCudaFunctionNV", PFN_vkDestroyCudaFunctionNV),
			#[cfg(feature = "provisional")]
			cmd_cuda_launch_kernel_nv: resolve!(c"vkCmdCudaLaunchKernelNV", PFN_vkCmdCudaLaunchKernelNV),
			cmd_begin_rendering: resolve!(c"vkCmdBeginRendering", PFN_vkCmdBeginRendering),
			cmd_begin_rendering_khr: resolve!(c"vkCmdBeginRenderingKHR", PFN_vkCmdBeginRenderingKHR),
			cmd_end_rendering: resolve!(c"vkCmdEndRendering", PFN_vkCmdEndRendering),
			cmd_end_rendering2_khr: resolve!(c"vkCmdEndRendering2KHR", PFN_vkCmdEndRendering2KHR),
			cmd_end_rendering2_ext: resolve!(c"vkCmdEndRendering2EXT", PFN_vkCmdEndRendering2EXT),
			cmd_end_rendering_khr: resolve!(c"vkCmdEndRenderingKHR", PFN_vkCmdEndRenderingKHR),
			get_descriptor_set_layout_host_mapping_info_valve: resolve!(
				c"vkGetDescriptorSetLayoutHostMappingInfoVALVE",
				PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE
			),
			get_descriptor_set_host_mapping_valve: resolve!(
				c"vkGetDescriptorSetHostMappingVALVE",
				PFN_vkGetDescriptorSetHostMappingVALVE
			),
			create_micromap_ext: resolve!(c"vkCreateMicromapEXT", PFN_vkCreateMicromapEXT),
			cmd_build_micromaps_ext: resolve!(c"vkCmdBuildMicromapsEXT", PFN_vkCmdBuildMicromapsEXT),
			build_micromaps_ext: resolve!(c"vkBuildMicromapsEXT", PFN_vkBuildMicromapsEXT),
			destroy_micromap_ext: resolve!(c"vkDestroyMicromapEXT", PFN_vkDestroyMicromapEXT),
			cmd_copy_micromap_ext: resolve!(c"vkCmdCopyMicromapEXT", PFN_vkCmdCopyMicromapEXT),
			copy_micromap_ext: resolve!(c"vkCopyMicromapEXT", PFN_vkCopyMicromapEXT),
			cmd_copy_micromap_to_memory_ext: resolve!(
				c"vkCmdCopyMicromapToMemoryEXT",
				PFN_vkCmdCopyMicromapToMemoryEXT
			),
			copy_micromap_to_memory_ext: resolve!(
				c"vkCopyMicromapToMemoryEXT",
				PFN_vkCopyMicromapToMemoryEXT
			),
			cmd_copy_memory_to_micromap_ext: resolve!(
				c"vkCmdCopyMemoryToMicromapEXT",
				PFN_vkCmdCopyMemoryToMicromapEXT
			),
			copy_memory_to_micromap_ext: resolve!(
				c"vkCopyMemoryToMicromapEXT",
				PFN_vkCopyMemoryToMicromapEXT
			),
			cmd_write_micromaps_properties_ext: resolve!(
				c"vkCmdWriteMicromapsPropertiesEXT",
				PFN_vkCmdWriteMicromapsPropertiesEXT
			),
			write_micromaps_properties_ext: resolve!(
				c"vkWriteMicromapsPropertiesEXT",
				PFN_vkWriteMicromapsPropertiesEXT
			),
			get_device_micromap_compatibility_ext: resolve!(
				c"vkGetDeviceMicromapCompatibilityEXT",
				PFN_vkGetDeviceMicromapCompatibilityEXT
			),
			get_micromap_build_sizes_ext: resolve!(
				c"vkGetMicromapBuildSizesEXT",
				PFN_vkGetMicromapBuildSizesEXT
			),
			get_shader_module_identifier_ext: resolve!(
				c"vkGetShaderModuleIdentifierEXT",
				PFN_vkGetShaderModuleIdentifierEXT
			),
			get_shader_module_create_info_identifier_ext: resolve!(
				c"vkGetShaderModuleCreateInfoIdentifierEXT",
				PFN_vkGetShaderModuleCreateInfoIdentifierEXT
			),
			get_image_subresource_layout2: resolve!(
				c"vkGetImageSubresourceLayout2",
				PFN_vkGetImageSubresourceLayout2
			),
			get_image_subresource_layout2_khr: resolve!(
				c"vkGetImageSubresourceLayout2KHR",
				PFN_vkGetImageSubresourceLayout2KHR
			),
			get_image_subresource_layout2_ext: resolve!(
				c"vkGetImageSubresourceLayout2EXT",
				PFN_vkGetImageSubresourceLayout2EXT
			),
			get_pipeline_properties_ext: resolve!(
				c"vkGetPipelinePropertiesEXT",
				PFN_vkGetPipelinePropertiesEXT
			),
			#[cfg(feature = "platform-metal")]
			export_metal_objects_ext: resolve!(c"vkExportMetalObjectsEXT", PFN_vkExportMetalObjectsEXT),
			cmd_bind_tile_memory_qcom: resolve!(c"vkCmdBindTileMemoryQCOM", PFN_vkCmdBindTileMemoryQCOM),
			get_framebuffer_tile_properties_qcom: resolve!(
				c"vkGetFramebufferTilePropertiesQCOM",
				PFN_vkGetFramebufferTilePropertiesQCOM
			),
			get_dynamic_rendering_tile_properties_qcom: resolve!(
				c"vkGetDynamicRenderingTilePropertiesQCOM",
				PFN_vkGetDynamicRenderingTilePropertiesQCOM
			),
			create_optical_flow_session_nv: resolve!(
				c"vkCreateOpticalFlowSessionNV",
				PFN_vkCreateOpticalFlowSessionNV
			),
			destroy_optical_flow_session_nv: resolve!(
				c"vkDestroyOpticalFlowSessionNV",
				PFN_vkDestroyOpticalFlowSessionNV
			),
			bind_optical_flow_session_image_nv: resolve!(
				c"vkBindOpticalFlowSessionImageNV",
				PFN_vkBindOpticalFlowSessionImageNV
			),
			cmd_optical_flow_execute_nv: resolve!(
				c"vkCmdOpticalFlowExecuteNV",
				PFN_vkCmdOpticalFlowExecuteNV
			),
			get_device_fault_info_ext: resolve!(c"vkGetDeviceFaultInfoEXT", PFN_vkGetDeviceFaultInfoEXT),
			get_device_fault_reports_khr: resolve!(
				c"vkGetDeviceFaultReportsKHR",
				PFN_vkGetDeviceFaultReportsKHR
			),
			get_device_fault_debug_info_khr: resolve!(
				c"vkGetDeviceFaultDebugInfoKHR",
				PFN_vkGetDeviceFaultDebugInfoKHR
			),
			cmd_set_depth_bias2_ext: resolve!(c"vkCmdSetDepthBias2EXT", PFN_vkCmdSetDepthBias2EXT),
			release_swapchain_images_khr: resolve!(
				c"vkReleaseSwapchainImagesKHR",
				PFN_vkReleaseSwapchainImagesKHR
			),
			release_swapchain_images_ext: resolve!(
				c"vkReleaseSwapchainImagesEXT",
				PFN_vkReleaseSwapchainImagesEXT
			),
			get_device_image_subresource_layout: resolve!(
				c"vkGetDeviceImageSubresourceLayout",
				PFN_vkGetDeviceImageSubresourceLayout
			),
			get_device_image_subresource_layout_khr: resolve!(
				c"vkGetDeviceImageSubresourceLayoutKHR",
				PFN_vkGetDeviceImageSubresourceLayoutKHR
			),
			map_memory2: resolve!(c"vkMapMemory2", PFN_vkMapMemory2),
			map_memory2_khr: resolve!(c"vkMapMemory2KHR", PFN_vkMapMemory2KHR),
			unmap_memory2: resolve!(c"vkUnmapMemory2", PFN_vkUnmapMemory2),
			unmap_memory2_khr: resolve!(c"vkUnmapMemory2KHR", PFN_vkUnmapMemory2KHR),
			create_shaders_ext: resolve!(c"vkCreateShadersEXT", PFN_vkCreateShadersEXT),
			destroy_shader_ext: resolve!(c"vkDestroyShaderEXT", PFN_vkDestroyShaderEXT),
			get_shader_binary_data_ext: resolve!(
				c"vkGetShaderBinaryDataEXT",
				PFN_vkGetShaderBinaryDataEXT
			),
			cmd_bind_shaders_ext: resolve!(c"vkCmdBindShadersEXT", PFN_vkCmdBindShadersEXT),
			set_swapchain_present_timing_queue_size_ext: resolve!(
				c"vkSetSwapchainPresentTimingQueueSizeEXT",
				PFN_vkSetSwapchainPresentTimingQueueSizeEXT
			),
			get_swapchain_timing_properties_ext: resolve!(
				c"vkGetSwapchainTimingPropertiesEXT",
				PFN_vkGetSwapchainTimingPropertiesEXT
			),
			get_swapchain_time_domain_properties_ext: resolve!(
				c"vkGetSwapchainTimeDomainPropertiesEXT",
				PFN_vkGetSwapchainTimeDomainPropertiesEXT
			),
			get_past_presentation_timing_ext: resolve!(
				c"vkGetPastPresentationTimingEXT",
				PFN_vkGetPastPresentationTimingEXT
			),
			#[cfg(feature = "platform-screen")]
			get_screen_buffer_properties_qnx: resolve!(
				c"vkGetScreenBufferPropertiesQNX",
				PFN_vkGetScreenBufferPropertiesQNX
			),
			#[cfg(feature = "provisional")]
			get_execution_graph_pipeline_scratch_size_amdx: resolve!(
				c"vkGetExecutionGraphPipelineScratchSizeAMDX",
				PFN_vkGetExecutionGraphPipelineScratchSizeAMDX
			),
			#[cfg(feature = "provisional")]
			get_execution_graph_pipeline_node_index_amdx: resolve!(
				c"vkGetExecutionGraphPipelineNodeIndexAMDX",
				PFN_vkGetExecutionGraphPipelineNodeIndexAMDX
			),
			#[cfg(feature = "provisional")]
			create_execution_graph_pipelines_amdx: resolve!(
				c"vkCreateExecutionGraphPipelinesAMDX",
				PFN_vkCreateExecutionGraphPipelinesAMDX
			),
			#[cfg(feature = "provisional")]
			cmd_initialize_graph_scratch_memory_amdx: resolve!(
				c"vkCmdInitializeGraphScratchMemoryAMDX",
				PFN_vkCmdInitializeGraphScratchMemoryAMDX
			),
			#[cfg(feature = "provisional")]
			cmd_dispatch_graph_amdx: resolve!(c"vkCmdDispatchGraphAMDX", PFN_vkCmdDispatchGraphAMDX),
			#[cfg(feature = "provisional")]
			cmd_dispatch_graph_indirect_amdx: resolve!(
				c"vkCmdDispatchGraphIndirectAMDX",
				PFN_vkCmdDispatchGraphIndirectAMDX
			),
			#[cfg(feature = "provisional")]
			cmd_dispatch_graph_indirect_count_amdx: resolve!(
				c"vkCmdDispatchGraphIndirectCountAMDX",
				PFN_vkCmdDispatchGraphIndirectCountAMDX
			),
			create_gpa_session_amd: resolve!(c"vkCreateGpaSessionAMD", PFN_vkCreateGpaSessionAMD),
			destroy_gpa_session_amd: resolve!(c"vkDestroyGpaSessionAMD", PFN_vkDestroyGpaSessionAMD),
			set_gpa_device_clock_mode_amd: resolve!(
				c"vkSetGpaDeviceClockModeAMD",
				PFN_vkSetGpaDeviceClockModeAMD
			),
			get_gpa_device_clock_info_amd: resolve!(
				c"vkGetGpaDeviceClockInfoAMD",
				PFN_vkGetGpaDeviceClockInfoAMD
			),
			cmd_begin_gpa_session_amd: resolve!(c"vkCmdBeginGpaSessionAMD", PFN_vkCmdBeginGpaSessionAMD),
			cmd_end_gpa_session_amd: resolve!(c"vkCmdEndGpaSessionAMD", PFN_vkCmdEndGpaSessionAMD),
			cmd_begin_gpa_sample_amd: resolve!(c"vkCmdBeginGpaSampleAMD", PFN_vkCmdBeginGpaSampleAMD),
			cmd_end_gpa_sample_amd: resolve!(c"vkCmdEndGpaSampleAMD", PFN_vkCmdEndGpaSampleAMD),
			get_gpa_session_status_amd: resolve!(
				c"vkGetGpaSessionStatusAMD",
				PFN_vkGetGpaSessionStatusAMD
			),
			get_gpa_session_results_amd: resolve!(
				c"vkGetGpaSessionResultsAMD",
				PFN_vkGetGpaSessionResultsAMD
			),
			reset_gpa_session_amd: resolve!(c"vkResetGpaSessionAMD", PFN_vkResetGpaSessionAMD),
			cmd_copy_gpa_session_results_amd: resolve!(
				c"vkCmdCopyGpaSessionResultsAMD",
				PFN_vkCmdCopyGpaSessionResultsAMD
			),
			cmd_bind_descriptor_sets2: resolve!(
				c"vkCmdBindDescriptorSets2",
				PFN_vkCmdBindDescriptorSets2
			),
			cmd_bind_descriptor_sets2_khr: resolve!(
				c"vkCmdBindDescriptorSets2KHR",
				PFN_vkCmdBindDescriptorSets2KHR
			),
			cmd_push_constants2: resolve!(c"vkCmdPushConstants2", PFN_vkCmdPushConstants2),
			cmd_push_constants2_khr: resolve!(c"vkCmdPushConstants2KHR", PFN_vkCmdPushConstants2KHR),
			cmd_push_descriptor_set2: resolve!(c"vkCmdPushDescriptorSet2", PFN_vkCmdPushDescriptorSet2),
			cmd_push_descriptor_set2_khr: resolve!(
				c"vkCmdPushDescriptorSet2KHR",
				PFN_vkCmdPushDescriptorSet2KHR
			),
			cmd_push_descriptor_set_with_template2: resolve!(
				c"vkCmdPushDescriptorSetWithTemplate2",
				PFN_vkCmdPushDescriptorSetWithTemplate2
			),
			cmd_push_descriptor_set_with_template2_khr: resolve!(
				c"vkCmdPushDescriptorSetWithTemplate2KHR",
				PFN_vkCmdPushDescriptorSetWithTemplate2KHR
			),
			cmd_set_descriptor_buffer_offsets2_ext: resolve!(
				c"vkCmdSetDescriptorBufferOffsets2EXT",
				PFN_vkCmdSetDescriptorBufferOffsets2EXT
			),
			cmd_bind_descriptor_buffer_embedded_samplers2_ext: resolve!(
				c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT",
				PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT
			),
			set_latency_sleep_mode_nv: resolve!(c"vkSetLatencySleepModeNV", PFN_vkSetLatencySleepModeNV),
			latency_sleep_nv: resolve!(c"vkLatencySleepNV", PFN_vkLatencySleepNV),
			set_latency_marker_nv: resolve!(c"vkSetLatencyMarkerNV", PFN_vkSetLatencyMarkerNV),
			get_latency_timings_nv: resolve!(c"vkGetLatencyTimingsNV", PFN_vkGetLatencyTimingsNV),
			queue_notify_out_of_band_nv: resolve!(
				c"vkQueueNotifyOutOfBandNV",
				PFN_vkQueueNotifyOutOfBandNV
			),
			set_latency_sleep_mode_legacy_nv: resolve!(
				c"vkSetLatencySleepModeLegacyNV",
				PFN_vkSetLatencySleepModeLegacyNV
			),
			latency_sleep_legacy_nv: resolve!(c"vkLatencySleepLegacyNV", PFN_vkLatencySleepLegacyNV),
			set_latency_marker_legacy_nv: resolve!(
				c"vkSetLatencyMarkerLegacyNV",
				PFN_vkSetLatencyMarkerLegacyNV
			),
			get_latency_timings_legacy_nv: resolve!(
				c"vkGetLatencyTimingsLegacyNV",
				PFN_vkGetLatencyTimingsLegacyNV
			),
			queue_notify_out_of_band_legacy_nv: resolve!(
				c"vkQueueNotifyOutOfBandLegacyNV",
				PFN_vkQueueNotifyOutOfBandLegacyNV
			),
			get_sleep_status_legacy_nv: resolve!(
				c"vkGetSleepStatusLegacyNV",
				PFN_vkGetSleepStatusLegacyNV
			),
			shutdown_latency_device_legacy_nv: resolve!(
				c"vkShutdownLatencyDeviceLegacyNV",
				PFN_vkShutdownLatencyDeviceLegacyNV
			),
			cmd_set_rendering_attachment_locations: resolve!(
				c"vkCmdSetRenderingAttachmentLocations",
				PFN_vkCmdSetRenderingAttachmentLocations
			),
			cmd_set_rendering_attachment_locations_khr: resolve!(
				c"vkCmdSetRenderingAttachmentLocationsKHR",
				PFN_vkCmdSetRenderingAttachmentLocationsKHR
			),
			cmd_set_rendering_input_attachment_indices: resolve!(
				c"vkCmdSetRenderingInputAttachmentIndices",
				PFN_vkCmdSetRenderingInputAttachmentIndices
			),
			cmd_set_rendering_input_attachment_indices_khr: resolve!(
				c"vkCmdSetRenderingInputAttachmentIndicesKHR",
				PFN_vkCmdSetRenderingInputAttachmentIndicesKHR
			),
			cmd_set_depth_clamp_range_ext: resolve!(
				c"vkCmdSetDepthClampRangeEXT",
				PFN_vkCmdSetDepthClampRangeEXT
			),
			#[cfg(feature = "platform-metal")]
			get_memory_metal_handle_ext: resolve!(
				c"vkGetMemoryMetalHandleEXT",
				PFN_vkGetMemoryMetalHandleEXT
			),
			#[cfg(feature = "platform-metal")]
			get_memory_metal_handle_properties_ext: resolve!(
				c"vkGetMemoryMetalHandlePropertiesEXT",
				PFN_vkGetMemoryMetalHandlePropertiesEXT
			),
			convert_cooperative_vector_matrix_nv: resolve!(
				c"vkConvertCooperativeVectorMatrixNV",
				PFN_vkConvertCooperativeVectorMatrixNV
			),
			cmd_convert_cooperative_vector_matrix_nv: resolve!(
				c"vkCmdConvertCooperativeVectorMatrixNV",
				PFN_vkCmdConvertCooperativeVectorMatrixNV
			),
			cmd_dispatch_tile_qcom: resolve!(c"vkCmdDispatchTileQCOM", PFN_vkCmdDispatchTileQCOM),
			cmd_begin_per_tile_execution_qcom: resolve!(
				c"vkCmdBeginPerTileExecutionQCOM",
				PFN_vkCmdBeginPerTileExecutionQCOM
			),
			cmd_end_per_tile_execution_qcom: resolve!(
				c"vkCmdEndPerTileExecutionQCOM",
				PFN_vkCmdEndPerTileExecutionQCOM
			),
			create_external_compute_queue_nv: resolve!(
				c"vkCreateExternalComputeQueueNV",
				PFN_vkCreateExternalComputeQueueNV
			),
			destroy_external_compute_queue_nv: resolve!(
				c"vkDestroyExternalComputeQueueNV",
				PFN_vkDestroyExternalComputeQueueNV
			),
			get_external_compute_queue_data_nv: resolve!(
				c"vkGetExternalComputeQueueDataNV",
				PFN_vkGetExternalComputeQueueDataNV
			),
			create_shader_instrumentation_arm: resolve!(
				c"vkCreateShaderInstrumentationARM",
				PFN_vkCreateShaderInstrumentationARM
			),
			destroy_shader_instrumentation_arm: resolve!(
				c"vkDestroyShaderInstrumentationARM",
				PFN_vkDestroyShaderInstrumentationARM
			),
			cmd_begin_shader_instrumentation_arm: resolve!(
				c"vkCmdBeginShaderInstrumentationARM",
				PFN_vkCmdBeginShaderInstrumentationARM
			),
			cmd_end_shader_instrumentation_arm: resolve!(
				c"vkCmdEndShaderInstrumentationARM",
				PFN_vkCmdEndShaderInstrumentationARM
			),
			get_shader_instrumentation_values_arm: resolve!(
				c"vkGetShaderInstrumentationValuesARM",
				PFN_vkGetShaderInstrumentationValuesARM
			),
			clear_shader_instrumentation_metrics_arm: resolve!(
				c"vkClearShaderInstrumentationMetricsARM",
				PFN_vkClearShaderInstrumentationMetricsARM
			),
			create_tensor_arm: resolve!(c"vkCreateTensorARM", PFN_vkCreateTensorARM),
			destroy_tensor_arm: resolve!(c"vkDestroyTensorARM", PFN_vkDestroyTensorARM),
			create_tensor_view_arm: resolve!(c"vkCreateTensorViewARM", PFN_vkCreateTensorViewARM),
			destroy_tensor_view_arm: resolve!(c"vkDestroyTensorViewARM", PFN_vkDestroyTensorViewARM),
			get_tensor_memory_requirements_arm: resolve!(
				c"vkGetTensorMemoryRequirementsARM",
				PFN_vkGetTensorMemoryRequirementsARM
			),
			bind_tensor_memory_arm: resolve!(c"vkBindTensorMemoryARM", PFN_vkBindTensorMemoryARM),
			get_device_tensor_memory_requirements_arm: resolve!(
				c"vkGetDeviceTensorMemoryRequirementsARM",
				PFN_vkGetDeviceTensorMemoryRequirementsARM
			),
			cmd_copy_tensor_arm: resolve!(c"vkCmdCopyTensorARM", PFN_vkCmdCopyTensorARM),
			get_tensor_opaque_capture_descriptor_data_arm: resolve!(
				c"vkGetTensorOpaqueCaptureDescriptorDataARM",
				PFN_vkGetTensorOpaqueCaptureDescriptorDataARM
			),
			get_tensor_view_opaque_capture_descriptor_data_arm: resolve!(
				c"vkGetTensorViewOpaqueCaptureDescriptorDataARM",
				PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM
			),
			create_data_graph_pipelines_arm: resolve!(
				c"vkCreateDataGraphPipelinesARM",
				PFN_vkCreateDataGraphPipelinesARM
			),
			create_data_graph_pipeline_session_arm: resolve!(
				c"vkCreateDataGraphPipelineSessionARM",
				PFN_vkCreateDataGraphPipelineSessionARM
			),
			get_data_graph_pipeline_session_bind_point_requirements_arm: resolve!(
				c"vkGetDataGraphPipelineSessionBindPointRequirementsARM",
				PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM
			),
			get_data_graph_pipeline_session_memory_requirements_arm: resolve!(
				c"vkGetDataGraphPipelineSessionMemoryRequirementsARM",
				PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM
			),
			bind_data_graph_pipeline_session_memory_arm: resolve!(
				c"vkBindDataGraphPipelineSessionMemoryARM",
				PFN_vkBindDataGraphPipelineSessionMemoryARM
			),
			destroy_data_graph_pipeline_session_arm: resolve!(
				c"vkDestroyDataGraphPipelineSessionARM",
				PFN_vkDestroyDataGraphPipelineSessionARM
			),
			cmd_dispatch_data_graph_arm: resolve!(
				c"vkCmdDispatchDataGraphARM",
				PFN_vkCmdDispatchDataGraphARM
			),
			get_data_graph_pipeline_available_properties_arm: resolve!(
				c"vkGetDataGraphPipelineAvailablePropertiesARM",
				PFN_vkGetDataGraphPipelineAvailablePropertiesARM
			),
			get_data_graph_pipeline_properties_arm: resolve!(
				c"vkGetDataGraphPipelinePropertiesARM",
				PFN_vkGetDataGraphPipelinePropertiesARM
			),
			#[cfg(feature = "platform-ohos")]
			get_native_buffer_properties_ohos: resolve!(
				c"vkGetNativeBufferPropertiesOHOS",
				PFN_vkGetNativeBufferPropertiesOHOS
			),
			#[cfg(feature = "platform-ohos")]
			get_memory_native_buffer_ohos: resolve!(
				c"vkGetMemoryNativeBufferOHOS",
				PFN_vkGetMemoryNativeBufferOHOS
			),
			queue_set_perf_hint_qcom: resolve!(c"vkQueueSetPerfHintQCOM", PFN_vkQueueSetPerfHintQCOM),
			cmd_set_compute_occupancy_priority_nv: resolve!(
				c"vkCmdSetComputeOccupancyPriorityNV",
				PFN_vkCmdSetComputeOccupancyPriorityNV
			),
			write_sampler_descriptors_ext: resolve!(
				c"vkWriteSamplerDescriptorsEXT",
				PFN_vkWriteSamplerDescriptorsEXT
			),
			write_resource_descriptors_ext: resolve!(
				c"vkWriteResourceDescriptorsEXT",
				PFN_vkWriteResourceDescriptorsEXT
			),
			cmd_bind_sampler_heap_ext: resolve!(c"vkCmdBindSamplerHeapEXT", PFN_vkCmdBindSamplerHeapEXT),
			cmd_bind_resource_heap_ext: resolve!(
				c"vkCmdBindResourceHeapEXT",
				PFN_vkCmdBindResourceHeapEXT
			),
			cmd_push_data_ext: resolve!(c"vkCmdPushDataEXT", PFN_vkCmdPushDataEXT),
			register_custom_border_color_ext: resolve!(
				c"vkRegisterCustomBorderColorEXT",
				PFN_vkRegisterCustomBorderColorEXT
			),
			unregister_custom_border_color_ext: resolve!(
				c"vkUnregisterCustomBorderColorEXT",
				PFN_vkUnregisterCustomBorderColorEXT
			),
			get_image_opaque_capture_data_ext: resolve!(
				c"vkGetImageOpaqueCaptureDataEXT",
				PFN_vkGetImageOpaqueCaptureDataEXT
			),
			get_tensor_opaque_capture_data_arm: resolve!(
				c"vkGetTensorOpaqueCaptureDataARM",
				PFN_vkGetTensorOpaqueCaptureDataARM
			),
			cmd_copy_memory_khr: resolve!(c"vkCmdCopyMemoryKHR", PFN_vkCmdCopyMemoryKHR),
			cmd_copy_memory_to_image_khr: resolve!(
				c"vkCmdCopyMemoryToImageKHR",
				PFN_vkCmdCopyMemoryToImageKHR
			),
			cmd_copy_image_to_memory_khr: resolve!(
				c"vkCmdCopyImageToMemoryKHR",
				PFN_vkCmdCopyImageToMemoryKHR
			),
			cmd_update_memory_khr: resolve!(c"vkCmdUpdateMemoryKHR", PFN_vkCmdUpdateMemoryKHR),
			cmd_fill_memory_khr: resolve!(c"vkCmdFillMemoryKHR", PFN_vkCmdFillMemoryKHR),
			cmd_copy_query_pool_results_to_memory_khr: resolve!(
				c"vkCmdCopyQueryPoolResultsToMemoryKHR",
				PFN_vkCmdCopyQueryPoolResultsToMemoryKHR
			),
			cmd_begin_conditional_rendering2_ext: resolve!(
				c"vkCmdBeginConditionalRendering2EXT",
				PFN_vkCmdBeginConditionalRendering2EXT
			),
			cmd_bind_transform_feedback_buffers2_ext: resolve!(
				c"vkCmdBindTransformFeedbackBuffers2EXT",
				PFN_vkCmdBindTransformFeedbackBuffers2EXT
			),
			cmd_begin_transform_feedback2_ext: resolve!(
				c"vkCmdBeginTransformFeedback2EXT",
				PFN_vkCmdBeginTransformFeedback2EXT
			),
			cmd_end_transform_feedback2_ext: resolve!(
				c"vkCmdEndTransformFeedback2EXT",
				PFN_vkCmdEndTransformFeedback2EXT
			),
			cmd_draw_indirect_byte_count2_ext: resolve!(
				c"vkCmdDrawIndirectByteCount2EXT",
				PFN_vkCmdDrawIndirectByteCount2EXT
			),
			cmd_write_marker_to_memory_amd: resolve!(
				c"vkCmdWriteMarkerToMemoryAMD",
				PFN_vkCmdWriteMarkerToMemoryAMD
			),
			cmd_bind_index_buffer3_khr: resolve!(
				c"vkCmdBindIndexBuffer3KHR",
				PFN_vkCmdBindIndexBuffer3KHR
			),
			cmd_bind_vertex_buffers3_khr: resolve!(
				c"vkCmdBindVertexBuffers3KHR",
				PFN_vkCmdBindVertexBuffers3KHR
			),
			cmd_draw_indirect2_khr: resolve!(c"vkCmdDrawIndirect2KHR", PFN_vkCmdDrawIndirect2KHR),
			cmd_draw_indexed_indirect2_khr: resolve!(
				c"vkCmdDrawIndexedIndirect2KHR",
				PFN_vkCmdDrawIndexedIndirect2KHR
			),
			cmd_draw_indirect_count2_khr: resolve!(
				c"vkCmdDrawIndirectCount2KHR",
				PFN_vkCmdDrawIndirectCount2KHR
			),
			cmd_draw_indexed_indirect_count2_khr: resolve!(
				c"vkCmdDrawIndexedIndirectCount2KHR",
				PFN_vkCmdDrawIndexedIndirectCount2KHR
			),
			cmd_draw_mesh_tasks_indirect2_ext: resolve!(
				c"vkCmdDrawMeshTasksIndirect2EXT",
				PFN_vkCmdDrawMeshTasksIndirect2EXT
			),
			cmd_draw_mesh_tasks_indirect_count2_ext: resolve!(
				c"vkCmdDrawMeshTasksIndirectCount2EXT",
				PFN_vkCmdDrawMeshTasksIndirectCount2EXT
			),
			cmd_dispatch_indirect2_khr: resolve!(
				c"vkCmdDispatchIndirect2KHR",
				PFN_vkCmdDispatchIndirect2KHR
			),
			create_acceleration_structure2_khr: resolve!(
				c"vkCreateAccelerationStructure2KHR",
				PFN_vkCreateAccelerationStructure2KHR
			),
			cmd_set_dispatch_parameters_arm: resolve!(
				c"vkCmdSetDispatchParametersARM",
				PFN_vkCmdSetDispatchParametersARM
			),
		}
	}

	#[cfg(feature = "loader")]
	pub(crate) fn retain_enabled(
		&mut self,
		mut enabled: impl FnMut(&super::extensions::CommandMetadata) -> bool,
	) {
		if !enabled(&super::extensions::COMMANDS[12]) {
			self.destroy_device = None;
		}
		if !enabled(&super::extensions::COMMANDS[18]) {
			self.get_device_queue = None;
		}
		if !enabled(&super::extensions::COMMANDS[19]) {
			self.queue_submit = None;
		}
		if !enabled(&super::extensions::COMMANDS[20]) {
			self.queue_wait_idle = None;
		}
		if !enabled(&super::extensions::COMMANDS[21]) {
			self.device_wait_idle = None;
		}
		if !enabled(&super::extensions::COMMANDS[22]) {
			self.allocate_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[23]) {
			self.free_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[24]) {
			self.map_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[25]) {
			self.unmap_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[26]) {
			self.flush_mapped_memory_ranges = None;
		}
		if !enabled(&super::extensions::COMMANDS[27]) {
			self.invalidate_mapped_memory_ranges = None;
		}
		if !enabled(&super::extensions::COMMANDS[28]) {
			self.get_device_memory_commitment = None;
		}
		if !enabled(&super::extensions::COMMANDS[29]) {
			self.get_buffer_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[30]) {
			self.bind_buffer_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[31]) {
			self.get_image_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[32]) {
			self.bind_image_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[33]) {
			self.get_image_sparse_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[35]) {
			self.queue_bind_sparse = None;
		}
		if !enabled(&super::extensions::COMMANDS[36]) {
			self.create_fence = None;
		}
		if !enabled(&super::extensions::COMMANDS[37]) {
			self.destroy_fence = None;
		}
		if !enabled(&super::extensions::COMMANDS[38]) {
			self.reset_fences = None;
		}
		if !enabled(&super::extensions::COMMANDS[39]) {
			self.get_fence_status = None;
		}
		if !enabled(&super::extensions::COMMANDS[40]) {
			self.wait_for_fences = None;
		}
		if !enabled(&super::extensions::COMMANDS[41]) {
			self.create_semaphore = None;
		}
		if !enabled(&super::extensions::COMMANDS[42]) {
			self.destroy_semaphore = None;
		}
		if !enabled(&super::extensions::COMMANDS[43]) {
			self.create_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[44]) {
			self.destroy_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[45]) {
			self.get_event_status = None;
		}
		if !enabled(&super::extensions::COMMANDS[46]) {
			self.set_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[47]) {
			self.reset_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[48]) {
			self.create_query_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[49]) {
			self.destroy_query_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[50]) {
			self.get_query_pool_results = None;
		}
		if !enabled(&super::extensions::COMMANDS[51]) {
			self.reset_query_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[52]) {
			self.reset_query_pool_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[53]) {
			self.create_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[54]) {
			self.destroy_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[55]) {
			self.create_buffer_view = None;
		}
		if !enabled(&super::extensions::COMMANDS[56]) {
			self.destroy_buffer_view = None;
		}
		if !enabled(&super::extensions::COMMANDS[57]) {
			self.create_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[58]) {
			self.destroy_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[59]) {
			self.get_image_subresource_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[60]) {
			self.create_image_view = None;
		}
		if !enabled(&super::extensions::COMMANDS[61]) {
			self.destroy_image_view = None;
		}
		if !enabled(&super::extensions::COMMANDS[62]) {
			self.create_shader_module = None;
		}
		if !enabled(&super::extensions::COMMANDS[63]) {
			self.destroy_shader_module = None;
		}
		if !enabled(&super::extensions::COMMANDS[64]) {
			self.create_pipeline_cache = None;
		}
		if !enabled(&super::extensions::COMMANDS[65]) {
			self.destroy_pipeline_cache = None;
		}
		if !enabled(&super::extensions::COMMANDS[66]) {
			self.get_pipeline_cache_data = None;
		}
		if !enabled(&super::extensions::COMMANDS[67]) {
			self.merge_pipeline_caches = None;
		}
		if !enabled(&super::extensions::COMMANDS[68]) {
			self.create_pipeline_binaries_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[69]) {
			self.destroy_pipeline_binary_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[70]) {
			self.get_pipeline_key_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[71]) {
			self.get_pipeline_binary_data_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[72]) {
			self.release_captured_pipeline_data_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[73]) {
			self.create_graphics_pipelines = None;
		}
		if !enabled(&super::extensions::COMMANDS[74]) {
			self.create_compute_pipelines = None;
		}
		if !enabled(&super::extensions::COMMANDS[75]) {
			self.get_device_subpass_shading_max_workgroup_size_huawei = None;
		}
		if !enabled(&super::extensions::COMMANDS[76]) {
			self.destroy_pipeline = None;
		}
		if !enabled(&super::extensions::COMMANDS[77]) {
			self.create_pipeline_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[78]) {
			self.destroy_pipeline_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[79]) {
			self.create_sampler = None;
		}
		if !enabled(&super::extensions::COMMANDS[80]) {
			self.destroy_sampler = None;
		}
		if !enabled(&super::extensions::COMMANDS[81]) {
			self.create_descriptor_set_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[82]) {
			self.destroy_descriptor_set_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[83]) {
			self.create_descriptor_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[84]) {
			self.destroy_descriptor_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[85]) {
			self.reset_descriptor_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[86]) {
			self.allocate_descriptor_sets = None;
		}
		if !enabled(&super::extensions::COMMANDS[87]) {
			self.free_descriptor_sets = None;
		}
		if !enabled(&super::extensions::COMMANDS[88]) {
			self.update_descriptor_sets = None;
		}
		if !enabled(&super::extensions::COMMANDS[89]) {
			self.create_framebuffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[90]) {
			self.destroy_framebuffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[91]) {
			self.create_render_pass = None;
		}
		if !enabled(&super::extensions::COMMANDS[92]) {
			self.destroy_render_pass = None;
		}
		if !enabled(&super::extensions::COMMANDS[93]) {
			self.get_render_area_granularity = None;
		}
		if !enabled(&super::extensions::COMMANDS[94]) {
			self.get_rendering_area_granularity = None;
		}
		if !enabled(&super::extensions::COMMANDS[95]) {
			self.get_rendering_area_granularity_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[96]) {
			self.create_command_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[97]) {
			self.destroy_command_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[98]) {
			self.reset_command_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[99]) {
			self.allocate_command_buffers = None;
		}
		if !enabled(&super::extensions::COMMANDS[100]) {
			self.free_command_buffers = None;
		}
		if !enabled(&super::extensions::COMMANDS[101]) {
			self.begin_command_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[102]) {
			self.end_command_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[103]) {
			self.reset_command_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[104]) {
			self.cmd_bind_pipeline = None;
		}
		if !enabled(&super::extensions::COMMANDS[105]) {
			self.cmd_set_primitive_restart_index_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[106]) {
			self.cmd_set_attachment_feedback_loop_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[107]) {
			self.cmd_set_viewport = None;
		}
		if !enabled(&super::extensions::COMMANDS[108]) {
			self.cmd_set_scissor = None;
		}
		if !enabled(&super::extensions::COMMANDS[109]) {
			self.cmd_set_line_width = None;
		}
		if !enabled(&super::extensions::COMMANDS[110]) {
			self.cmd_set_depth_bias = None;
		}
		if !enabled(&super::extensions::COMMANDS[111]) {
			self.cmd_set_blend_constants = None;
		}
		if !enabled(&super::extensions::COMMANDS[112]) {
			self.cmd_set_depth_bounds = None;
		}
		if !enabled(&super::extensions::COMMANDS[113]) {
			self.cmd_set_stencil_compare_mask = None;
		}
		if !enabled(&super::extensions::COMMANDS[114]) {
			self.cmd_set_stencil_write_mask = None;
		}
		if !enabled(&super::extensions::COMMANDS[115]) {
			self.cmd_set_stencil_reference = None;
		}
		if !enabled(&super::extensions::COMMANDS[116]) {
			self.cmd_bind_descriptor_sets = None;
		}
		if !enabled(&super::extensions::COMMANDS[117]) {
			self.cmd_bind_index_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[118]) {
			self.cmd_bind_vertex_buffers = None;
		}
		if !enabled(&super::extensions::COMMANDS[119]) {
			self.cmd_draw = None;
		}
		if !enabled(&super::extensions::COMMANDS[120]) {
			self.cmd_draw_indexed = None;
		}
		if !enabled(&super::extensions::COMMANDS[121]) {
			self.cmd_draw_multi_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[122]) {
			self.cmd_draw_multi_indexed_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[123]) {
			self.cmd_draw_indirect = None;
		}
		if !enabled(&super::extensions::COMMANDS[124]) {
			self.cmd_draw_indexed_indirect = None;
		}
		if !enabled(&super::extensions::COMMANDS[125]) {
			self.cmd_dispatch = None;
		}
		if !enabled(&super::extensions::COMMANDS[126]) {
			self.cmd_dispatch_indirect = None;
		}
		if !enabled(&super::extensions::COMMANDS[127]) {
			self.cmd_subpass_shading_huawei = None;
		}
		if !enabled(&super::extensions::COMMANDS[128]) {
			self.cmd_draw_cluster_huawei = None;
		}
		if !enabled(&super::extensions::COMMANDS[129]) {
			self.cmd_draw_cluster_indirect_huawei = None;
		}
		if !enabled(&super::extensions::COMMANDS[130]) {
			self.cmd_update_pipeline_indirect_buffer_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[131]) {
			self.cmd_copy_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[132]) {
			self.cmd_copy_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[133]) {
			self.cmd_blit_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[134]) {
			self.cmd_copy_buffer_to_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[135]) {
			self.cmd_copy_image_to_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[136]) {
			self.cmd_copy_memory_indirect_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[137]) {
			self.cmd_copy_memory_indirect_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[138]) {
			self.cmd_copy_memory_to_image_indirect_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[139]) {
			self.cmd_copy_memory_to_image_indirect_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[140]) {
			self.cmd_update_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[141]) {
			self.cmd_fill_buffer = None;
		}
		if !enabled(&super::extensions::COMMANDS[142]) {
			self.cmd_clear_color_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[143]) {
			self.cmd_clear_depth_stencil_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[144]) {
			self.cmd_clear_attachments = None;
		}
		if !enabled(&super::extensions::COMMANDS[145]) {
			self.cmd_resolve_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[146]) {
			self.cmd_set_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[147]) {
			self.cmd_reset_event = None;
		}
		if !enabled(&super::extensions::COMMANDS[148]) {
			self.cmd_wait_events = None;
		}
		if !enabled(&super::extensions::COMMANDS[149]) {
			self.cmd_pipeline_barrier = None;
		}
		if !enabled(&super::extensions::COMMANDS[150]) {
			self.cmd_begin_query = None;
		}
		if !enabled(&super::extensions::COMMANDS[151]) {
			self.cmd_end_query = None;
		}
		if !enabled(&super::extensions::COMMANDS[152]) {
			self.cmd_begin_conditional_rendering_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[153]) {
			self.cmd_end_conditional_rendering_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[154]) {
			self.cmd_begin_custom_resolve_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[155]) {
			self.cmd_reset_query_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[156]) {
			self.cmd_write_timestamp = None;
		}
		if !enabled(&super::extensions::COMMANDS[157]) {
			self.cmd_copy_query_pool_results = None;
		}
		if !enabled(&super::extensions::COMMANDS[158]) {
			self.cmd_push_constants = None;
		}
		if !enabled(&super::extensions::COMMANDS[159]) {
			self.cmd_begin_render_pass = None;
		}
		if !enabled(&super::extensions::COMMANDS[160]) {
			self.cmd_next_subpass = None;
		}
		if !enabled(&super::extensions::COMMANDS[161]) {
			self.cmd_end_render_pass = None;
		}
		if !enabled(&super::extensions::COMMANDS[162]) {
			self.cmd_execute_commands = None;
		}
		if !enabled(&super::extensions::COMMANDS[172]) {
			self.create_shared_swapchains_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[178]) {
			self.create_swapchain_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[179]) {
			self.destroy_swapchain_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[180]) {
			self.get_swapchain_images_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[181]) {
			self.acquire_next_image_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[182]) {
			self.queue_present_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[203]) {
			self.debug_marker_set_object_name_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[204]) {
			self.debug_marker_set_object_tag_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[205]) {
			self.cmd_debug_marker_begin_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[206]) {
			self.cmd_debug_marker_end_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[207]) {
			self.cmd_debug_marker_insert_ext = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[209]) {
			self.get_memory_win32_handle_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[210]) {
			self.cmd_execute_generated_commands_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[211]) {
			self.cmd_preprocess_generated_commands_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[212]) {
			self.cmd_bind_pipeline_shader_group_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[213]) {
			self.get_generated_commands_memory_requirements_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[214]) {
			self.create_indirect_commands_layout_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[215]) {
			self.destroy_indirect_commands_layout_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[216]) {
			self.cmd_execute_generated_commands_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[217]) {
			self.cmd_preprocess_generated_commands_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[218]) {
			self.get_generated_commands_memory_requirements_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[219]) {
			self.create_indirect_commands_layout_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[220]) {
			self.destroy_indirect_commands_layout_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[221]) {
			self.create_indirect_execution_set_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[222]) {
			self.destroy_indirect_execution_set_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[223]) {
			self.update_indirect_execution_set_pipeline_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[224]) {
			self.update_indirect_execution_set_shader_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[239]) {
			self.cmd_push_descriptor_set = None;
		}
		if !enabled(&super::extensions::COMMANDS[240]) {
			self.cmd_push_descriptor_set_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[241]) {
			self.trim_command_pool = None;
		}
		if !enabled(&super::extensions::COMMANDS[242]) {
			self.trim_command_pool_khr = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[245]) {
			self.get_memory_win32_handle_khr = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[246]) {
			self.get_memory_win32_handle_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[247]) {
			self.get_memory_fd_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[248]) {
			self.get_memory_fd_properties_khr = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[249]) {
			self.get_memory_zircon_handle_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[250]) {
			self.get_memory_zircon_handle_properties_fuchsia = None;
		}
		if !enabled(&super::extensions::COMMANDS[251]) {
			self.get_memory_remote_address_nv = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[254]) {
			self.get_semaphore_win32_handle_khr = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[255]) {
			self.import_semaphore_win32_handle_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[256]) {
			self.get_semaphore_fd_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[257]) {
			self.import_semaphore_fd_khr = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[258]) {
			self.get_semaphore_zircon_handle_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[259]) {
			self.import_semaphore_zircon_handle_fuchsia = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[262]) {
			self.get_fence_win32_handle_khr = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[263]) {
			self.import_fence_win32_handle_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[264]) {
			self.get_fence_fd_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[265]) {
			self.import_fence_fd_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[271]) {
			self.display_power_control_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[272]) {
			self.register_device_event_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[273]) {
			self.register_display_event_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[274]) {
			self.get_swapchain_counter_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[278]) {
			self.get_device_group_peer_memory_features = None;
		}
		if !enabled(&super::extensions::COMMANDS[279]) {
			self.get_device_group_peer_memory_features_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[280]) {
			self.bind_buffer_memory2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[281]) {
			self.bind_buffer_memory2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[282]) {
			self.bind_image_memory2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[283]) {
			self.bind_image_memory2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[284]) {
			self.cmd_set_device_mask = None;
		}
		if !enabled(&super::extensions::COMMANDS[285]) {
			self.cmd_set_device_mask_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[286]) {
			self.get_device_group_present_capabilities_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[287]) {
			self.get_device_group_surface_present_modes_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[288]) {
			self.acquire_next_image2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[289]) {
			self.cmd_dispatch_base = None;
		}
		if !enabled(&super::extensions::COMMANDS[290]) {
			self.cmd_dispatch_base_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[292]) {
			self.create_descriptor_update_template = None;
		}
		if !enabled(&super::extensions::COMMANDS[293]) {
			self.create_descriptor_update_template_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[294]) {
			self.destroy_descriptor_update_template = None;
		}
		if !enabled(&super::extensions::COMMANDS[295]) {
			self.destroy_descriptor_update_template_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[296]) {
			self.update_descriptor_set_with_template = None;
		}
		if !enabled(&super::extensions::COMMANDS[297]) {
			self.update_descriptor_set_with_template_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[298]) {
			self.cmd_push_descriptor_set_with_template = None;
		}
		if !enabled(&super::extensions::COMMANDS[299]) {
			self.cmd_push_descriptor_set_with_template_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[300]) {
			self.set_hdr_metadata_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[301]) {
			self.get_swapchain_status_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[302]) {
			self.get_refresh_cycle_duration_google = None;
		}
		if !enabled(&super::extensions::COMMANDS[303]) {
			self.get_past_presentation_timing_google = None;
		}
		if !enabled(&super::extensions::COMMANDS[307]) {
			self.cmd_set_viewport_w_scaling_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[308]) {
			self.cmd_set_discard_rectangle_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[309]) {
			self.cmd_set_discard_rectangle_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[310]) {
			self.cmd_set_discard_rectangle_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[311]) {
			self.cmd_set_sample_locations_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[319]) {
			self.get_buffer_memory_requirements2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[320]) {
			self.get_buffer_memory_requirements2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[321]) {
			self.get_image_memory_requirements2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[322]) {
			self.get_image_memory_requirements2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[323]) {
			self.get_image_sparse_memory_requirements2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[324]) {
			self.get_image_sparse_memory_requirements2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[325]) {
			self.get_device_buffer_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[326]) {
			self.get_device_buffer_memory_requirements_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[327]) {
			self.get_device_image_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[328]) {
			self.get_device_image_memory_requirements_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[329]) {
			self.get_device_image_sparse_memory_requirements = None;
		}
		if !enabled(&super::extensions::COMMANDS[330]) {
			self.get_device_image_sparse_memory_requirements_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[331]) {
			self.create_sampler_ycbcr_conversion = None;
		}
		if !enabled(&super::extensions::COMMANDS[332]) {
			self.create_sampler_ycbcr_conversion_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[333]) {
			self.destroy_sampler_ycbcr_conversion = None;
		}
		if !enabled(&super::extensions::COMMANDS[334]) {
			self.destroy_sampler_ycbcr_conversion_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[335]) {
			self.get_device_queue2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[336]) {
			self.create_validation_cache_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[337]) {
			self.destroy_validation_cache_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[338]) {
			self.get_validation_cache_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[339]) {
			self.merge_validation_caches_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[340]) {
			self.get_descriptor_set_layout_support = None;
		}
		if !enabled(&super::extensions::COMMANDS[341]) {
			self.get_descriptor_set_layout_support_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[342]) {
			self.get_shader_info_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[343]) {
			self.set_local_dimming_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[346]) {
			self.get_calibrated_timestamps_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[347]) {
			self.get_calibrated_timestamps_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[348]) {
			self.set_debug_utils_object_name_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[349]) {
			self.set_debug_utils_object_tag_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[350]) {
			self.queue_begin_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[351]) {
			self.queue_end_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[352]) {
			self.queue_insert_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[353]) {
			self.cmd_begin_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[354]) {
			self.cmd_end_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[355]) {
			self.cmd_insert_debug_utils_label_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[359]) {
			self.get_memory_host_pointer_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[360]) {
			self.cmd_write_buffer_marker_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[361]) {
			self.create_render_pass2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[362]) {
			self.create_render_pass2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[363]) {
			self.cmd_begin_render_pass2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[364]) {
			self.cmd_begin_render_pass2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[365]) {
			self.cmd_next_subpass2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[366]) {
			self.cmd_next_subpass2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[367]) {
			self.cmd_end_render_pass2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[368]) {
			self.cmd_end_render_pass2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[369]) {
			self.get_semaphore_counter_value = None;
		}
		if !enabled(&super::extensions::COMMANDS[370]) {
			self.get_semaphore_counter_value_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[371]) {
			self.wait_semaphores = None;
		}
		if !enabled(&super::extensions::COMMANDS[372]) {
			self.wait_semaphores_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[373]) {
			self.signal_semaphore = None;
		}
		if !enabled(&super::extensions::COMMANDS[374]) {
			self.signal_semaphore_khr = None;
		}

		#[cfg(feature = "platform-android")]
		if !enabled(&super::extensions::COMMANDS[375]) {
			self.get_android_hardware_buffer_properties_android = None;
		}

		#[cfg(feature = "platform-android")]
		if !enabled(&super::extensions::COMMANDS[376]) {
			self.get_memory_android_hardware_buffer_android = None;
		}
		if !enabled(&super::extensions::COMMANDS[377]) {
			self.cmd_draw_indirect_count = None;
		}
		if !enabled(&super::extensions::COMMANDS[378]) {
			self.cmd_draw_indirect_count_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[379]) {
			self.cmd_draw_indirect_count_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[380]) {
			self.cmd_draw_indexed_indirect_count = None;
		}
		if !enabled(&super::extensions::COMMANDS[381]) {
			self.cmd_draw_indexed_indirect_count_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[382]) {
			self.cmd_draw_indexed_indirect_count_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[383]) {
			self.cmd_set_checkpoint_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[384]) {
			self.get_queue_checkpoint_data_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[385]) {
			self.cmd_bind_transform_feedback_buffers_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[386]) {
			self.cmd_begin_transform_feedback_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[387]) {
			self.cmd_end_transform_feedback_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[388]) {
			self.cmd_begin_query_indexed_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[389]) {
			self.cmd_end_query_indexed_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[390]) {
			self.cmd_draw_indirect_byte_count_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[391]) {
			self.cmd_set_exclusive_scissor_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[392]) {
			self.cmd_set_exclusive_scissor_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[393]) {
			self.cmd_bind_shading_rate_image_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[394]) {
			self.cmd_set_viewport_shading_rate_palette_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[395]) {
			self.cmd_set_coarse_sample_order_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[396]) {
			self.cmd_draw_mesh_tasks_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[397]) {
			self.cmd_draw_mesh_tasks_indirect_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[398]) {
			self.cmd_draw_mesh_tasks_indirect_count_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[399]) {
			self.cmd_draw_mesh_tasks_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[400]) {
			self.cmd_draw_mesh_tasks_indirect_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[401]) {
			self.cmd_draw_mesh_tasks_indirect_count_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[402]) {
			self.compile_deferred_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[403]) {
			self.create_acceleration_structure_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[404]) {
			self.cmd_bind_invocation_mask_huawei = None;
		}
		if !enabled(&super::extensions::COMMANDS[405]) {
			self.destroy_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[406]) {
			self.destroy_acceleration_structure_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[407]) {
			self.get_acceleration_structure_memory_requirements_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[408]) {
			self.bind_acceleration_structure_memory_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[409]) {
			self.cmd_copy_acceleration_structure_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[410]) {
			self.cmd_copy_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[411]) {
			self.copy_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[412]) {
			self.cmd_copy_acceleration_structure_to_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[413]) {
			self.copy_acceleration_structure_to_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[414]) {
			self.cmd_copy_memory_to_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[415]) {
			self.copy_memory_to_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[416]) {
			self.cmd_write_acceleration_structures_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[417]) {
			self.cmd_write_acceleration_structures_properties_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[418]) {
			self.cmd_build_acceleration_structure_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[419]) {
			self.write_acceleration_structures_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[420]) {
			self.cmd_trace_rays_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[421]) {
			self.cmd_trace_rays_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[422]) {
			self.get_ray_tracing_shader_group_handles_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[423]) {
			self.get_ray_tracing_shader_group_handles_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[424]) {
			self.get_ray_tracing_capture_replay_shader_group_handles_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[425]) {
			self.get_acceleration_structure_handle_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[426]) {
			self.create_ray_tracing_pipelines_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[427]) {
			self.create_ray_tracing_pipelines_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[429]) {
			self.cmd_trace_rays_indirect_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[430]) {
			self.cmd_trace_rays_indirect2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[431]) {
			self.get_cluster_acceleration_structure_build_sizes_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[432]) {
			self.cmd_build_cluster_acceleration_structure_indirect_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[433]) {
			self.get_device_acceleration_structure_compatibility_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[434]) {
			self.get_ray_tracing_shader_group_stack_size_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[435]) {
			self.cmd_set_ray_tracing_pipeline_stack_size_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[436]) {
			self.get_image_view_handle_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[437]) {
			self.get_image_view_handle64_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[438]) {
			self.get_image_view_address_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[439]) {
			self.get_device_combined_image_sampler_index_nvx = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[441]) {
			self.get_device_group_surface_present_modes2_ext = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[442]) {
			self.acquire_full_screen_exclusive_mode_ext = None;
		}

		#[cfg(feature = "platform-win32")]
		if !enabled(&super::extensions::COMMANDS[443]) {
			self.release_full_screen_exclusive_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[446]) {
			self.acquire_profiling_lock_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[447]) {
			self.release_profiling_lock_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[448]) {
			self.get_image_drm_format_modifier_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[449]) {
			self.get_buffer_opaque_capture_address = None;
		}
		if !enabled(&super::extensions::COMMANDS[450]) {
			self.get_buffer_opaque_capture_address_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[451]) {
			self.get_buffer_device_address = None;
		}
		if !enabled(&super::extensions::COMMANDS[452]) {
			self.get_buffer_device_address_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[453]) {
			self.get_buffer_device_address_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[456]) {
			self.initialize_performance_api_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[457]) {
			self.uninitialize_performance_api_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[458]) {
			self.cmd_set_performance_marker_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[459]) {
			self.cmd_set_performance_stream_marker_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[460]) {
			self.cmd_set_performance_override_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[461]) {
			self.acquire_performance_configuration_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[462]) {
			self.release_performance_configuration_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[463]) {
			self.queue_set_performance_configuration_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[464]) {
			self.get_performance_parameter_intel = None;
		}
		if !enabled(&super::extensions::COMMANDS[465]) {
			self.get_device_memory_opaque_capture_address = None;
		}
		if !enabled(&super::extensions::COMMANDS[466]) {
			self.get_device_memory_opaque_capture_address_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[467]) {
			self.get_pipeline_executable_properties_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[468]) {
			self.get_pipeline_executable_statistics_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[469]) {
			self.get_pipeline_executable_internal_representations_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[470]) {
			self.cmd_set_line_stipple = None;
		}
		if !enabled(&super::extensions::COMMANDS[471]) {
			self.cmd_set_line_stipple_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[472]) {
			self.cmd_set_line_stipple_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[475]) {
			self.create_acceleration_structure_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[476]) {
			self.cmd_build_acceleration_structures_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[477]) {
			self.cmd_build_acceleration_structures_indirect_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[478]) {
			self.build_acceleration_structures_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[479]) {
			self.get_acceleration_structure_device_address_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[480]) {
			self.create_deferred_operation_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[481]) {
			self.destroy_deferred_operation_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[482]) {
			self.get_deferred_operation_max_concurrency_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[483]) {
			self.get_deferred_operation_result_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[484]) {
			self.deferred_operation_join_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[485]) {
			self.get_pipeline_indirect_memory_requirements_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[486]) {
			self.get_pipeline_indirect_device_address_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[487]) {
			self.anti_lag_update_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[488]) {
			self.cmd_set_cull_mode = None;
		}
		if !enabled(&super::extensions::COMMANDS[489]) {
			self.cmd_set_cull_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[490]) {
			self.cmd_set_front_face = None;
		}
		if !enabled(&super::extensions::COMMANDS[491]) {
			self.cmd_set_front_face_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[492]) {
			self.cmd_set_primitive_topology = None;
		}
		if !enabled(&super::extensions::COMMANDS[493]) {
			self.cmd_set_primitive_topology_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[494]) {
			self.cmd_set_viewport_with_count = None;
		}
		if !enabled(&super::extensions::COMMANDS[495]) {
			self.cmd_set_viewport_with_count_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[496]) {
			self.cmd_set_scissor_with_count = None;
		}
		if !enabled(&super::extensions::COMMANDS[497]) {
			self.cmd_set_scissor_with_count_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[498]) {
			self.cmd_bind_index_buffer2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[499]) {
			self.cmd_bind_index_buffer2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[500]) {
			self.cmd_bind_vertex_buffers2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[501]) {
			self.cmd_bind_vertex_buffers2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[502]) {
			self.cmd_set_depth_test_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[503]) {
			self.cmd_set_depth_test_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[504]) {
			self.cmd_set_depth_write_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[505]) {
			self.cmd_set_depth_write_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[506]) {
			self.cmd_set_depth_compare_op = None;
		}
		if !enabled(&super::extensions::COMMANDS[507]) {
			self.cmd_set_depth_compare_op_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[508]) {
			self.cmd_set_depth_bounds_test_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[509]) {
			self.cmd_set_depth_bounds_test_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[510]) {
			self.cmd_set_stencil_test_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[511]) {
			self.cmd_set_stencil_test_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[512]) {
			self.cmd_set_stencil_op = None;
		}
		if !enabled(&super::extensions::COMMANDS[513]) {
			self.cmd_set_stencil_op_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[514]) {
			self.cmd_set_patch_control_points_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[515]) {
			self.cmd_set_rasterizer_discard_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[516]) {
			self.cmd_set_rasterizer_discard_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[517]) {
			self.cmd_set_depth_bias_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[518]) {
			self.cmd_set_depth_bias_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[519]) {
			self.cmd_set_logic_op_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[520]) {
			self.cmd_set_primitive_restart_enable = None;
		}
		if !enabled(&super::extensions::COMMANDS[521]) {
			self.cmd_set_primitive_restart_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[522]) {
			self.cmd_set_tessellation_domain_origin_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[523]) {
			self.cmd_set_depth_clamp_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[524]) {
			self.cmd_set_polygon_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[525]) {
			self.cmd_set_rasterization_samples_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[526]) {
			self.cmd_set_sample_mask_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[527]) {
			self.cmd_set_alpha_to_coverage_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[528]) {
			self.cmd_set_alpha_to_one_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[529]) {
			self.cmd_set_logic_op_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[530]) {
			self.cmd_set_color_blend_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[531]) {
			self.cmd_set_color_blend_equation_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[532]) {
			self.cmd_set_color_write_mask_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[533]) {
			self.cmd_set_rasterization_stream_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[534]) {
			self.cmd_set_conservative_rasterization_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[535]) {
			self.cmd_set_extra_primitive_overestimation_size_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[536]) {
			self.cmd_set_depth_clip_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[537]) {
			self.cmd_set_sample_locations_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[538]) {
			self.cmd_set_color_blend_advanced_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[539]) {
			self.cmd_set_provoking_vertex_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[540]) {
			self.cmd_set_line_rasterization_mode_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[541]) {
			self.cmd_set_line_stipple_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[542]) {
			self.cmd_set_depth_clip_negative_one_to_one_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[543]) {
			self.cmd_set_viewport_w_scaling_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[544]) {
			self.cmd_set_viewport_swizzle_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[545]) {
			self.cmd_set_coverage_to_color_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[546]) {
			self.cmd_set_coverage_to_color_location_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[547]) {
			self.cmd_set_coverage_modulation_mode_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[548]) {
			self.cmd_set_coverage_modulation_table_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[549]) {
			self.cmd_set_coverage_modulation_table_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[550]) {
			self.cmd_set_shading_rate_image_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[551]) {
			self.cmd_set_coverage_reduction_mode_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[552]) {
			self.cmd_set_representative_fragment_test_enable_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[553]) {
			self.create_private_data_slot = None;
		}
		if !enabled(&super::extensions::COMMANDS[554]) {
			self.create_private_data_slot_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[555]) {
			self.destroy_private_data_slot = None;
		}
		if !enabled(&super::extensions::COMMANDS[556]) {
			self.destroy_private_data_slot_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[557]) {
			self.set_private_data = None;
		}
		if !enabled(&super::extensions::COMMANDS[558]) {
			self.set_private_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[559]) {
			self.get_private_data = None;
		}
		if !enabled(&super::extensions::COMMANDS[560]) {
			self.get_private_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[561]) {
			self.cmd_copy_buffer2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[562]) {
			self.cmd_copy_buffer2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[563]) {
			self.cmd_copy_image2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[564]) {
			self.cmd_copy_image2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[565]) {
			self.cmd_blit_image2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[566]) {
			self.cmd_blit_image2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[567]) {
			self.cmd_copy_buffer_to_image2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[568]) {
			self.cmd_copy_buffer_to_image2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[569]) {
			self.cmd_copy_image_to_buffer2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[570]) {
			self.cmd_copy_image_to_buffer2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[571]) {
			self.cmd_resolve_image2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[572]) {
			self.cmd_resolve_image2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[573]) {
			self.cmd_set_fragment_shading_rate_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[575]) {
			self.cmd_set_fragment_shading_rate_enum_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[576]) {
			self.get_acceleration_structure_build_sizes_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[577]) {
			self.cmd_set_vertex_input_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[578]) {
			self.cmd_set_color_write_enable_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[579]) {
			self.cmd_set_event2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[580]) {
			self.cmd_set_event2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[581]) {
			self.cmd_reset_event2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[582]) {
			self.cmd_reset_event2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[583]) {
			self.cmd_wait_events2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[584]) {
			self.cmd_wait_events2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[585]) {
			self.cmd_pipeline_barrier2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[586]) {
			self.cmd_pipeline_barrier2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[587]) {
			self.queue_submit2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[588]) {
			self.queue_submit2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[589]) {
			self.cmd_write_timestamp2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[590]) {
			self.cmd_write_timestamp2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[591]) {
			self.cmd_write_buffer_marker2_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[592]) {
			self.get_queue_checkpoint_data2_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[593]) {
			self.copy_memory_to_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[594]) {
			self.copy_memory_to_image_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[595]) {
			self.copy_image_to_memory = None;
		}
		if !enabled(&super::extensions::COMMANDS[596]) {
			self.copy_image_to_memory_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[597]) {
			self.copy_image_to_image = None;
		}
		if !enabled(&super::extensions::COMMANDS[598]) {
			self.copy_image_to_image_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[599]) {
			self.transition_image_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[600]) {
			self.transition_image_layout_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[604]) {
			self.create_video_session_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[605]) {
			self.destroy_video_session_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[606]) {
			self.create_video_session_parameters_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[607]) {
			self.update_video_session_parameters_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[608]) {
			self.get_encoded_video_session_parameters_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[609]) {
			self.destroy_video_session_parameters_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[610]) {
			self.get_video_session_memory_requirements_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[611]) {
			self.bind_video_session_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[612]) {
			self.cmd_decode_video_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[613]) {
			self.cmd_begin_video_coding_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[614]) {
			self.cmd_control_video_coding_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[615]) {
			self.cmd_end_video_coding_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[616]) {
			self.cmd_encode_video_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[617]) {
			self.cmd_decompress_memory_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[618]) {
			self.cmd_decompress_memory_indirect_count_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[619]) {
			self.get_partitioned_acceleration_structures_build_sizes_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[620]) {
			self.cmd_build_partitioned_acceleration_structures_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[621]) {
			self.cmd_decompress_memory_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[622]) {
			self.cmd_decompress_memory_indirect_count_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[623]) {
			self.create_cu_module_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[624]) {
			self.create_cu_function_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[625]) {
			self.destroy_cu_module_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[626]) {
			self.destroy_cu_function_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[627]) {
			self.cmd_cu_launch_kernel_nvx = None;
		}
		if !enabled(&super::extensions::COMMANDS[628]) {
			self.get_descriptor_set_layout_size_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[629]) {
			self.get_descriptor_set_layout_binding_offset_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[630]) {
			self.get_descriptor_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[631]) {
			self.cmd_bind_descriptor_buffers_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[632]) {
			self.cmd_set_descriptor_buffer_offsets_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[633]) {
			self.cmd_bind_descriptor_buffer_embedded_samplers_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[634]) {
			self.get_buffer_opaque_capture_descriptor_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[635]) {
			self.get_image_opaque_capture_descriptor_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[636]) {
			self.get_image_view_opaque_capture_descriptor_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[637]) {
			self.get_sampler_opaque_capture_descriptor_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[638]) {
			self.get_acceleration_structure_opaque_capture_descriptor_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[639]) {
			self.set_device_memory_priority_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[642]) {
			self.wait_for_present2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[643]) {
			self.wait_for_present_khr = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[644]) {
			self.create_buffer_collection_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[645]) {
			self.set_buffer_collection_buffer_constraints_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[646]) {
			self.set_buffer_collection_image_constraints_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[647]) {
			self.destroy_buffer_collection_fuchsia = None;
		}

		#[cfg(feature = "platform-fuchsia")]
		if !enabled(&super::extensions::COMMANDS[648]) {
			self.get_buffer_collection_properties_fuchsia = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[649]) {
			self.create_cuda_module_nv = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[650]) {
			self.get_cuda_module_cache_nv = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[651]) {
			self.create_cuda_function_nv = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[652]) {
			self.destroy_cuda_module_nv = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[653]) {
			self.destroy_cuda_function_nv = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[654]) {
			self.cmd_cuda_launch_kernel_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[655]) {
			self.cmd_begin_rendering = None;
		}
		if !enabled(&super::extensions::COMMANDS[656]) {
			self.cmd_begin_rendering_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[657]) {
			self.cmd_end_rendering = None;
		}
		if !enabled(&super::extensions::COMMANDS[658]) {
			self.cmd_end_rendering2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[659]) {
			self.cmd_end_rendering2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[660]) {
			self.cmd_end_rendering_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[661]) {
			self.get_descriptor_set_layout_host_mapping_info_valve = None;
		}
		if !enabled(&super::extensions::COMMANDS[662]) {
			self.get_descriptor_set_host_mapping_valve = None;
		}
		if !enabled(&super::extensions::COMMANDS[663]) {
			self.create_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[664]) {
			self.cmd_build_micromaps_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[665]) {
			self.build_micromaps_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[666]) {
			self.destroy_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[667]) {
			self.cmd_copy_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[668]) {
			self.copy_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[669]) {
			self.cmd_copy_micromap_to_memory_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[670]) {
			self.copy_micromap_to_memory_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[671]) {
			self.cmd_copy_memory_to_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[672]) {
			self.copy_memory_to_micromap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[673]) {
			self.cmd_write_micromaps_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[674]) {
			self.write_micromaps_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[675]) {
			self.get_device_micromap_compatibility_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[676]) {
			self.get_micromap_build_sizes_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[677]) {
			self.get_shader_module_identifier_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[678]) {
			self.get_shader_module_create_info_identifier_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[679]) {
			self.get_image_subresource_layout2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[680]) {
			self.get_image_subresource_layout2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[681]) {
			self.get_image_subresource_layout2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[682]) {
			self.get_pipeline_properties_ext = None;
		}

		#[cfg(feature = "platform-metal")]
		if !enabled(&super::extensions::COMMANDS[683]) {
			self.export_metal_objects_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[684]) {
			self.cmd_bind_tile_memory_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[685]) {
			self.get_framebuffer_tile_properties_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[686]) {
			self.get_dynamic_rendering_tile_properties_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[688]) {
			self.create_optical_flow_session_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[689]) {
			self.destroy_optical_flow_session_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[690]) {
			self.bind_optical_flow_session_image_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[691]) {
			self.cmd_optical_flow_execute_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[692]) {
			self.get_device_fault_info_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[693]) {
			self.get_device_fault_reports_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[694]) {
			self.get_device_fault_debug_info_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[695]) {
			self.cmd_set_depth_bias2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[696]) {
			self.release_swapchain_images_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[697]) {
			self.release_swapchain_images_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[698]) {
			self.get_device_image_subresource_layout = None;
		}
		if !enabled(&super::extensions::COMMANDS[699]) {
			self.get_device_image_subresource_layout_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[700]) {
			self.map_memory2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[701]) {
			self.map_memory2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[702]) {
			self.unmap_memory2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[703]) {
			self.unmap_memory2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[704]) {
			self.create_shaders_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[705]) {
			self.destroy_shader_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[706]) {
			self.get_shader_binary_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[707]) {
			self.cmd_bind_shaders_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[708]) {
			self.set_swapchain_present_timing_queue_size_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[709]) {
			self.get_swapchain_timing_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[710]) {
			self.get_swapchain_time_domain_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[711]) {
			self.get_past_presentation_timing_ext = None;
		}

		#[cfg(feature = "platform-screen")]
		if !enabled(&super::extensions::COMMANDS[712]) {
			self.get_screen_buffer_properties_qnx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[715]) {
			self.get_execution_graph_pipeline_scratch_size_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[716]) {
			self.get_execution_graph_pipeline_node_index_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[717]) {
			self.create_execution_graph_pipelines_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[718]) {
			self.cmd_initialize_graph_scratch_memory_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[719]) {
			self.cmd_dispatch_graph_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[720]) {
			self.cmd_dispatch_graph_indirect_amdx = None;
		}

		#[cfg(feature = "provisional")]
		if !enabled(&super::extensions::COMMANDS[721]) {
			self.cmd_dispatch_graph_indirect_count_amdx = None;
		}
		if !enabled(&super::extensions::COMMANDS[722]) {
			self.create_gpa_session_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[723]) {
			self.destroy_gpa_session_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[724]) {
			self.set_gpa_device_clock_mode_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[725]) {
			self.get_gpa_device_clock_info_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[726]) {
			self.cmd_begin_gpa_session_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[727]) {
			self.cmd_end_gpa_session_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[728]) {
			self.cmd_begin_gpa_sample_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[729]) {
			self.cmd_end_gpa_sample_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[730]) {
			self.get_gpa_session_status_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[731]) {
			self.get_gpa_session_results_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[732]) {
			self.reset_gpa_session_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[733]) {
			self.cmd_copy_gpa_session_results_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[734]) {
			self.cmd_bind_descriptor_sets2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[735]) {
			self.cmd_bind_descriptor_sets2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[736]) {
			self.cmd_push_constants2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[737]) {
			self.cmd_push_constants2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[738]) {
			self.cmd_push_descriptor_set2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[739]) {
			self.cmd_push_descriptor_set2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[740]) {
			self.cmd_push_descriptor_set_with_template2 = None;
		}
		if !enabled(&super::extensions::COMMANDS[741]) {
			self.cmd_push_descriptor_set_with_template2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[742]) {
			self.cmd_set_descriptor_buffer_offsets2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[743]) {
			self.cmd_bind_descriptor_buffer_embedded_samplers2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[744]) {
			self.set_latency_sleep_mode_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[745]) {
			self.latency_sleep_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[746]) {
			self.set_latency_marker_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[747]) {
			self.get_latency_timings_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[748]) {
			self.queue_notify_out_of_band_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[749]) {
			self.set_latency_sleep_mode_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[750]) {
			self.latency_sleep_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[751]) {
			self.set_latency_marker_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[752]) {
			self.get_latency_timings_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[753]) {
			self.queue_notify_out_of_band_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[754]) {
			self.get_sleep_status_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[755]) {
			self.shutdown_latency_device_legacy_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[756]) {
			self.cmd_set_rendering_attachment_locations = None;
		}
		if !enabled(&super::extensions::COMMANDS[757]) {
			self.cmd_set_rendering_attachment_locations_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[758]) {
			self.cmd_set_rendering_input_attachment_indices = None;
		}
		if !enabled(&super::extensions::COMMANDS[759]) {
			self.cmd_set_rendering_input_attachment_indices_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[760]) {
			self.cmd_set_depth_clamp_range_ext = None;
		}

		#[cfg(feature = "platform-metal")]
		if !enabled(&super::extensions::COMMANDS[762]) {
			self.get_memory_metal_handle_ext = None;
		}

		#[cfg(feature = "platform-metal")]
		if !enabled(&super::extensions::COMMANDS[763]) {
			self.get_memory_metal_handle_properties_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[765]) {
			self.convert_cooperative_vector_matrix_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[766]) {
			self.cmd_convert_cooperative_vector_matrix_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[767]) {
			self.cmd_dispatch_tile_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[768]) {
			self.cmd_begin_per_tile_execution_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[769]) {
			self.cmd_end_per_tile_execution_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[770]) {
			self.create_external_compute_queue_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[771]) {
			self.destroy_external_compute_queue_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[772]) {
			self.get_external_compute_queue_data_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[774]) {
			self.create_shader_instrumentation_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[775]) {
			self.destroy_shader_instrumentation_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[776]) {
			self.cmd_begin_shader_instrumentation_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[777]) {
			self.cmd_end_shader_instrumentation_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[778]) {
			self.get_shader_instrumentation_values_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[779]) {
			self.clear_shader_instrumentation_metrics_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[780]) {
			self.create_tensor_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[781]) {
			self.destroy_tensor_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[782]) {
			self.create_tensor_view_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[783]) {
			self.destroy_tensor_view_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[784]) {
			self.get_tensor_memory_requirements_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[785]) {
			self.bind_tensor_memory_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[786]) {
			self.get_device_tensor_memory_requirements_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[787]) {
			self.cmd_copy_tensor_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[788]) {
			self.get_tensor_opaque_capture_descriptor_data_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[789]) {
			self.get_tensor_view_opaque_capture_descriptor_data_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[791]) {
			self.create_data_graph_pipelines_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[792]) {
			self.create_data_graph_pipeline_session_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[793]) {
			self.get_data_graph_pipeline_session_bind_point_requirements_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[794]) {
			self.get_data_graph_pipeline_session_memory_requirements_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[795]) {
			self.bind_data_graph_pipeline_session_memory_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[796]) {
			self.destroy_data_graph_pipeline_session_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[797]) {
			self.cmd_dispatch_data_graph_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[798]) {
			self.get_data_graph_pipeline_available_properties_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[799]) {
			self.get_data_graph_pipeline_properties_arm = None;
		}

		#[cfg(feature = "platform-ohos")]
		if !enabled(&super::extensions::COMMANDS[802]) {
			self.get_native_buffer_properties_ohos = None;
		}

		#[cfg(feature = "platform-ohos")]
		if !enabled(&super::extensions::COMMANDS[803]) {
			self.get_memory_native_buffer_ohos = None;
		}
		if !enabled(&super::extensions::COMMANDS[804]) {
			self.queue_set_perf_hint_qcom = None;
		}
		if !enabled(&super::extensions::COMMANDS[806]) {
			self.cmd_set_compute_occupancy_priority_nv = None;
		}
		if !enabled(&super::extensions::COMMANDS[807]) {
			self.write_sampler_descriptors_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[808]) {
			self.write_resource_descriptors_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[809]) {
			self.cmd_bind_sampler_heap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[810]) {
			self.cmd_bind_resource_heap_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[811]) {
			self.cmd_push_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[812]) {
			self.register_custom_border_color_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[813]) {
			self.unregister_custom_border_color_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[814]) {
			self.get_image_opaque_capture_data_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[816]) {
			self.get_tensor_opaque_capture_data_arm = None;
		}
		if !enabled(&super::extensions::COMMANDS[817]) {
			self.cmd_copy_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[818]) {
			self.cmd_copy_memory_to_image_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[819]) {
			self.cmd_copy_image_to_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[820]) {
			self.cmd_update_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[821]) {
			self.cmd_fill_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[822]) {
			self.cmd_copy_query_pool_results_to_memory_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[823]) {
			self.cmd_begin_conditional_rendering2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[824]) {
			self.cmd_bind_transform_feedback_buffers2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[825]) {
			self.cmd_begin_transform_feedback2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[826]) {
			self.cmd_end_transform_feedback2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[827]) {
			self.cmd_draw_indirect_byte_count2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[828]) {
			self.cmd_write_marker_to_memory_amd = None;
		}
		if !enabled(&super::extensions::COMMANDS[829]) {
			self.cmd_bind_index_buffer3_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[830]) {
			self.cmd_bind_vertex_buffers3_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[831]) {
			self.cmd_draw_indirect2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[832]) {
			self.cmd_draw_indexed_indirect2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[833]) {
			self.cmd_draw_indirect_count2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[834]) {
			self.cmd_draw_indexed_indirect_count2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[835]) {
			self.cmd_draw_mesh_tasks_indirect2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[836]) {
			self.cmd_draw_mesh_tasks_indirect_count2_ext = None;
		}
		if !enabled(&super::extensions::COMMANDS[837]) {
			self.cmd_dispatch_indirect2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[838]) {
			self.create_acceleration_structure2_khr = None;
		}
		if !enabled(&super::extensions::COMMANDS[840]) {
			self.cmd_set_dispatch_parameters_arm = None;
		}
	}
}
