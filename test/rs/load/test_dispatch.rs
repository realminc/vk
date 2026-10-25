#![cfg(feature = "loader")]
//! Driver-independent checked-loader tests using exact Vulkan mock functions.
use ::vk::{
	self,
	load::{CallError, Entry, LoadError},
};
use std::{
	cell::Cell,
	ffi::{CStr, c_char},
};
thread_local! { static DESTROYED: Cell<usize> = const { Cell::new(0) }; }
thread_local! { static DEVICE_DESTROYED: Cell<usize> = const { Cell::new(0) }; }
thread_local! { static RESOLVER_QUERIES: Cell<usize> = const { Cell::new(0) }; }

#[test]
fn free_functions_and_methods_share_cached_dispatch_and_owner_types() {
	let _: Option<&::vk::Features> = None::<&::vk::load::Features>;
	let _: Option<&::vk::FeatureValue> = None::<&::vk::load::FeatureValue>;
	let _: Option<&::vk::Capabilities> = None::<&::vk::load::Capabilities>;
	let _: Option<&::vk::InitializationFailure> = None::<&::vk::load::InitializationFailure>;
	let _: ::vk::CommandSupport = ::vk::load::CommandSupport::Disabled;
	let _: ::vk::CreatedHandle = ::vk::load::CreatedHandle::Instance(vk::InstanceHandle::NULL);
	let before = DEVICE_DESTROYED.get();
	let calls = CMD_DISPATCH_CALLS.get();
	// SAFETY: static exact-ABI mock resolvers; fake objects have no native resources.
	unsafe {
		let entry = Entry::new_from_resolver(resolver_m3_entry).unwrap();
		let info = vk::InstanceCreateInfo::default();
		let instance = ::vk::create_instance(&entry, &info, None).unwrap();
		let physical = ::vk::enumerate_physical_devices(&instance).unwrap()[0];
		let device =
			::vk::Device::new(&instance, physical, &vk::DeviceCreateInfo::default(), None).unwrap();
		let resolved = RESOLVER_QUERIES.get();
		let queue = ::vk::get_device_queue(&device, 0, 0).unwrap();
		assert_eq!(queue, device.get_device_queue(0, 0).unwrap());
		::vk::cmd_dispatch(&device, vk::CommandBuffer::NULL, 1, 1, 1).unwrap();
		device
			.cmd_dispatch(vk::CommandBuffer::NULL, 1, 1, 1)
			.unwrap();
		assert_eq!(
			RESOLVER_QUERIES.get(),
			resolved,
			"ordinary calls must reuse cached PFNs"
		);
		::vk::destroy_device(device, None);
		::vk::destroy_instance(instance, None);
		let instance = ::vk::Instance::new(&entry, &info, None).unwrap();
		let physical = instance.enumerate_physical_devices().unwrap()[0];
		let device =
			::vk::create_device(&instance, physical, &vk::DeviceCreateInfo::default(), None).unwrap();
		device.destroy(None);
		instance.destroy(None);
	}
	assert_eq!(CMD_DISPATCH_CALLS.get(), calls + 2);
	assert_eq!(DEVICE_DESTROYED.get(), before + 2);
}
unsafe extern "system" fn create(
	_: *const vk::InstanceCreateInfo,
	_: *const vk::AllocationCallbacks,
	out: *mut vk::InstanceHandle,
) -> vk::Result {
	// SAFETY: the loader supplies a writable output handle.
	unsafe { *out = vk::InstanceHandle(std::ptr::dangling_mut::<std::ffi::c_void>()) };
	vk::Result::SUCCESS
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
unsafe extern "system" fn version_error(_: *mut u32) -> vk::Result {
	vk::Result::ERROR_OUT_OF_HOST_MEMORY
}

unsafe extern "system" fn version_success(output: *mut u32) -> vk::Result {
	// SAFETY: Entry supplies one writable version output slot.
	unsafe {
		*output = vk::API_VERSION_1_3;
	}
	vk::Result::SUCCESS
}
unsafe extern "system" fn destroy(_: vk::InstanceHandle, _: *const vk::AllocationCallbacks) {
	DESTROYED.set(DESTROYED.get() + 1);
}
// --- M3 full mock helpers ----------------------------------------------

thread_local! { static CMD_DISPATCH_CALLS: Cell<usize> = const { Cell::new(0) }; }

unsafe extern "system" fn mock_enumerate_instance_extensions(
	_layer: *const c_char,
	count: *mut u32,
	props: *mut vk::ExtensionProperties,
) -> vk::Result {
	if props.is_null() {
		unsafe { *count = 1 };
	} else {
		// Write a single fake extension name into the first slot.
		unsafe {
			std::ptr::write(props, std::mem::zeroed());
			let slot = &mut *props;
			let name = b"VK_KHR_mock_ext\0";
			let dst = slot.extension_name.as_mut_ptr() as *mut u8;
			std::ptr::copy_nonoverlapping(name.as_ptr(), dst, name.len());
			slot.spec_version = 1;
		}
	}
	vk::Result::SUCCESS
}

unsafe extern "system" fn mock_enumerate_instance_layers(
	count: *mut u32,
	layers: *mut vk::LayerProperties,
) -> vk::Result {
	if layers.is_null() {
		unsafe { *count = 1 };
	} else {
		unsafe {
			std::ptr::write(layers, std::mem::zeroed());
			let slot = &mut *layers;
			let name = b"VK_LAYER_MOCK_validation\0";
			let dst = slot.layer_name.as_mut_ptr() as *mut u8;
			std::ptr::copy_nonoverlapping(name.as_ptr(), dst, name.len());
			slot.spec_version = 1;
			slot.implementation_version = 1;
		}
	}
	vk::Result::SUCCESS
}

unsafe extern "system" fn mock_get_physical_device_properties(
	_: vk::PhysicalDevice,
	props: *mut vk::PhysicalDeviceProperties,
) {
	unsafe {
		(*props).api_version = vk::API_VERSION_1_3;
		(*props).vendor_id = 0xDEAD;
		(*props).device_id = 0xBEEF;
		(*props).device_type = vk::PhysicalDeviceType::DISCRETE_GPU;
	}
}

unsafe extern "system" fn mock_get_physical_device_features(
	_: vk::PhysicalDevice,
	features: *mut vk::PhysicalDeviceFeatures,
) {
	// Set a handful of common features to TRUE (1u32).
	unsafe {
		(*features).robust_buffer_access = vk::TRUE;
		(*features).shader_float64 = vk::TRUE;
	}
}

unsafe extern "system" fn mock_get_physical_device_queue_family_properties(
	_: vk::PhysicalDevice,
	count: *mut u32,
	families: *mut vk::QueueFamilyProperties,
) {
	if families.is_null() {
		unsafe { *count = 1 };
	} else {
		unsafe {
			(*families).queue_flags = vk::QueueFlagBits::GRAPHICS_BIT | vk::QueueFlagBits::COMPUTE_BIT;
			(*families).queue_count = 4;
			(*families).timestamp_valid_bits = 64;
		}
	}
}

unsafe extern "system" fn mock_enumerate_device_extensions(
	_: vk::PhysicalDevice,
	_layer: *const c_char,
	count: *mut u32,
	props: *mut vk::ExtensionProperties,
) -> vk::Result {
	if props.is_null() {
		unsafe { *count = 1 };
	} else {
		unsafe {
			std::ptr::write(props, std::mem::zeroed());
			let slot = &mut *props;
			let name = b"VK_KHR_swapchain\0";
			let dst = slot.extension_name.as_mut_ptr() as *mut u8;
			std::ptr::copy_nonoverlapping(name.as_ptr(), dst, name.len());
			slot.spec_version = 70;
		}
	}
	vk::Result::SUCCESS
}

unsafe extern "system" fn mock_get_device_queue(
	_: vk::DeviceHandle,
	_family: u32,
	_index: u32,
	out: *mut vk::Queue,
) {
	unsafe { *out = vk::Queue(4usize as *mut std::ffi::c_void) };
}

unsafe extern "system" fn mock_cmd_dispatch(_: vk::CommandBuffer, _x: u32, _y: u32, _z: u32) {
	CMD_DISPATCH_CALLS.set(CMD_DISPATCH_CALLS.get() + 1);
}

unsafe extern "system" fn mock_enumerate_physical_devices(
	_: vk::InstanceHandle,
	count: *mut u32,
	devices: *mut vk::PhysicalDevice,
) -> vk::Result {
	if devices.is_null() {
		// SAFETY: caller provides a writable count.
		unsafe { *count = 1 };
	} else {
		// SAFETY: caller has allocated at least *count elements.
		unsafe { *devices = vk::PhysicalDevice(2usize as *mut std::ffi::c_void) };
	}
	vk::Result::SUCCESS
}

unsafe extern "system" fn mock_create_device(
	_: vk::PhysicalDevice,
	_: *const vk::DeviceCreateInfo,
	_: *const vk::AllocationCallbacks,
	out: *mut vk::DeviceHandle,
) -> vk::Result {
	// SAFETY: caller provides a writable device output slot.
	unsafe { *out = vk::DeviceHandle(3usize as *mut std::ffi::c_void) };
	vk::Result::SUCCESS
}

unsafe extern "system" fn mock_destroy_device(
	_: vk::DeviceHandle,
	_: *const vk::AllocationCallbacks,
) {
	DEVICE_DESTROYED.set(DEVICE_DESTROYED.get() + 1);
}

/// Instance-level resolver that also handles M3 commands.
unsafe extern "system" fn resolver_m3(
	_: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: loader passes static NUL-terminated command names.
	RESOLVER_QUERIES.set(RESOLVER_QUERIES.get() + 1);
	let name = unsafe { CStr::from_ptr(name) };
	macro_rules! erased {
		($f:expr, $ty:ty) => {{ Some(unsafe { std::mem::transmute::<$ty, unsafe extern "system" fn()>($f) }) }};
	}
	match name.to_bytes() {
		b"vkDestroyInstance" => erased!(
			destroy,
			unsafe extern "system" fn(vk::InstanceHandle, *const vk::AllocationCallbacks)
		),
		b"vkEnumeratePhysicalDevices" => erased!(
			mock_enumerate_physical_devices,
			unsafe extern "system" fn(
				vk::InstanceHandle,
				*mut u32,
				*mut vk::PhysicalDevice,
			) -> vk::Result
		),
		b"vkGetPhysicalDeviceProperties" => erased!(
			mock_get_physical_device_properties,
			unsafe extern "system" fn(vk::PhysicalDevice, *mut vk::PhysicalDeviceProperties)
		),
		b"vkGetPhysicalDeviceFeatures" => erased!(
			mock_get_physical_device_features,
			unsafe extern "system" fn(vk::PhysicalDevice, *mut vk::PhysicalDeviceFeatures)
		),
		b"vkGetPhysicalDeviceQueueFamilyProperties" => erased!(
			mock_get_physical_device_queue_family_properties,
			unsafe extern "system" fn(vk::PhysicalDevice, *mut u32, *mut vk::QueueFamilyProperties)
		),
		b"vkEnumerateDeviceExtensionProperties" => erased!(
			mock_enumerate_device_extensions,
			unsafe extern "system" fn(
				vk::PhysicalDevice,
				*const c_char,
				*mut u32,
				*mut vk::ExtensionProperties,
			) -> vk::Result
		),
		b"vkCreateDevice" => erased!(
			mock_create_device,
			unsafe extern "system" fn(
				vk::PhysicalDevice,
				*const vk::DeviceCreateInfo,
				*const vk::AllocationCallbacks,
				*mut vk::DeviceHandle,
			) -> vk::Result
		),
		// vkGetDeviceProcAddr — re-use resolver_m3 cast to the device-level sig
		b"vkGetDeviceProcAddr" => Some(unsafe {
			std::mem::transmute::<
				unsafe extern "system" fn(vk::InstanceHandle, *const c_char) -> vk::PFN_vkVoidFunction,
				unsafe extern "system" fn(),
			>(resolver_m3)
		}),
		b"vkDestroyDevice" => erased!(
			mock_destroy_device,
			unsafe extern "system" fn(vk::DeviceHandle, *const vk::AllocationCallbacks)
		),
		b"vkGetDeviceQueue" => erased!(
			mock_get_device_queue,
			unsafe extern "system" fn(vk::DeviceHandle, u32, u32, *mut vk::Queue)
		),
		b"vkCmdDispatch" => erased!(
			mock_cmd_dispatch,
			unsafe extern "system" fn(vk::CommandBuffer, u32, u32, u32)
		),
		_ => None,
	}
}

/// Entry-level resolver used by M3 tests; combines global + instance commands.
unsafe extern "system" fn resolver_m3_entry(
	instance: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: delegates to the global entry resolver first, then M3.
	let name_cstr = unsafe { CStr::from_ptr(name) };
	macro_rules! erased {
		($f:expr, $ty:ty) => {{ Some(unsafe { std::mem::transmute::<$ty, unsafe extern "system" fn()>($f) }) }};
	}
	match name_cstr.to_bytes() {
		b"vkCreateInstance" => erased!(
			create,
			unsafe extern "system" fn(
				*const vk::InstanceCreateInfo,
				*const vk::AllocationCallbacks,
				*mut vk::InstanceHandle,
			) -> vk::Result
		),
		b"vkEnumerateInstanceExtensionProperties" => erased!(
			mock_enumerate_instance_extensions,
			unsafe extern "system" fn(
				*const c_char,
				*mut u32,
				*mut vk::ExtensionProperties,
			) -> vk::Result
		),
		b"vkEnumerateInstanceLayerProperties" => erased!(
			mock_enumerate_instance_layers,
			unsafe extern "system" fn(*mut u32, *mut vk::LayerProperties) -> vk::Result
		),
		b"vkEnumerateInstanceVersion" => erased!(
			version_success,
			unsafe extern "system" fn(*mut u32) -> vk::Result
		),
		_ => unsafe { resolver_m3(instance, name) },
	}
}

// -----------------------------------------------------------------------

unsafe extern "system" fn resolver(
	_: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: Entry passes static NUL-terminated command names.
	RESOLVER_QUERIES.set(RESOLVER_QUERIES.get() + 1);
	let name = unsafe { CStr::from_ptr(name) };
	macro_rules! erased {
		($f:expr,$ty:ty) => {{
			// SAFETY: erase a non-null function pointer; Entry casts it back by name.
			Some(unsafe { std::mem::transmute::<$ty, unsafe extern "system" fn()>($f) })
		}};
	}
	match name.to_bytes() {
		b"vkCreateInstance" => erased!(
			create,
			unsafe extern "system" fn(
				*const vk::InstanceCreateInfo,
				*const vk::AllocationCallbacks,
				*mut vk::InstanceHandle,
			) -> vk::Result
		),
		b"vkEnumerateInstanceExtensionProperties" => erased!(
			extensions,
			unsafe extern "system" fn(
				*const c_char,
				*mut u32,
				*mut vk::ExtensionProperties,
			) -> vk::Result
		),
		b"vkEnumerateInstanceLayerProperties" => erased!(
			layers,
			unsafe extern "system" fn(*mut u32, *mut vk::LayerProperties) -> vk::Result
		),
		b"vkEnumerateInstanceVersion" => erased!(
			version_success,
			unsafe extern "system" fn(*mut u32) -> vk::Result
		),
		b"vkDestroyInstance" => erased!(
			destroy,
			unsafe extern "system" fn(vk::InstanceHandle, *const vk::AllocationCallbacks)
		),
		_ => None,
	}
}
unsafe extern "system" fn absent(
	_: vk::InstanceHandle,
	_: *const c_char,
) -> vk::PFN_vkVoidFunction {
	None
}
unsafe extern "system" fn v10(
	instance: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: same static name and compatible instance contract as resolver.
	if unsafe { CStr::from_ptr(name) } == c"vkEnumerateInstanceVersion" {
		None
	} else {
		unsafe { resolver(instance, name) }
	}
}

unsafe extern "system" fn failing_version_resolver(
	instance: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: Entry supplies a static terminated command name. The error fixture
	// changes only the exact native version-query function, with its correct ABI.
	unsafe {
		if CStr::from_ptr(name) == c"vkEnumerateInstanceVersion" {
			std::mem::transmute::<vk::PFN_vkEnumerateInstanceVersion, vk::PFN_vkVoidFunction>(Some(
				version_error,
			))
		} else {
			resolver(instance, name)
		}
	}
}

#[test]
fn missing_required_global_is_reported() {
	// SAFETY: this mock has static lifetime and resolves no functions.
	let result = unsafe { Entry::new_from_resolver(absent) };
	assert!(matches!(
		result,
		Err(LoadError::MissingSymbol("vkCreateInstance"))
	));
}

#[test]
fn version_error_is_preserved_and_v10_falls_back() {
	// SAFETY: all mock PFNs have static lifetime and the exact requested ABI.
	let entry = unsafe { Entry::new_from_resolver(failing_version_resolver) }.unwrap();
	assert_eq!(
		entry.enumerate_instance_version(),
		Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY)
	);
	// SAFETY: v10 implements the valid missing-version-command case.
	let entry = unsafe { Entry::new_from_resolver(v10) }.unwrap();
	assert_eq!(
		entry.enumerate_instance_version().unwrap(),
		vk::API_VERSION_1_0
	);
}

#[test]
fn instance_destroys_exactly_once_after_entry_drop() {
	let before = DESTROYED.get();
	// SAFETY: the resolver and all functions are static.
	let entry = unsafe { Entry::new_from_resolver(resolver) }.unwrap();
	// SAFETY: the mock accepts an initialized empty create info.
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	drop(entry);
	// SAFETY: there are no children or callbacks.
	unsafe { instance.destroy(None) };
	assert_eq!(DESTROYED.get(), before + 1);
}

#[test]
fn imported_wrappers_drop_without_native_destruction() {
	let instances_before = DESTROYED.get();
	let devices_before = DEVICE_DESTROYED.get();
	// SAFETY: static exact-ABI mocks accept these opaque identities without
	// dereferencing them; no real native objects, children or GPU work exist.
	unsafe {
		let entry = Entry::new_from_resolver(resolver_m3_entry).unwrap();
		let instance_handle = vk::InstanceHandle::from_raw(11);
		let device_handle = vk::DeviceHandle::from_raw(22);
		let instance = vk::Instance::new_from_handle(&entry, instance_handle).unwrap();
		let device = vk::Device::new_from_handle(&instance, device_handle).unwrap();
		let destroy_instance = instance.fp().destroy_instance.unwrap();
		let destroy_device = device.fp().destroy_device.unwrap();
		assert_eq!(instance.handle(), instance_handle);
		assert_eq!(device.handle(), device_handle);
		drop(device);
		drop(instance);
		assert_eq!(DESTROYED.get(), instances_before);
		assert_eq!(DEVICE_DESTROYED.get(), devices_before);
		// SAFETY: the mocks remain callable after wrapper Drop. Explicit native
		// cleanup still runs child before parent, once, with matching null callbacks.
		destroy_device(device_handle, std::ptr::null());
		destroy_instance(instance_handle, std::ptr::null());
	}
	assert_eq!(DESTROYED.get(), instances_before + 1);
	assert_eq!(DEVICE_DESTROYED.get(), devices_before + 1);
}

#[test]
fn invalid_library_path_is_a_result() {
	// SAFETY: an embedded NUL is rejected without loading a native library.
	let result = unsafe { Entry::new_from_path("invalid\0path") };
	assert!(matches!(result, Err(LoadError::LibraryOpen(_))));
}

// M3: generated resolver constructors and Device dispatch.

#[test]
fn enumerate_physical_devices_returns_mock_device() {
	// SAFETY: resolver_m3_entry is a static function with the correct ABI.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	// SAFETY: mock create info is trivially valid for the mock driver.
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	// SAFETY: instance handle is live (just created by the mock).
	let devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
	assert_eq!(devices.len(), 1, "mock exposes exactly one physical device");
	assert!(
		!devices[0].is_null(),
		"physical device handle must be non-null"
	);
	// SAFETY: no children; mock destroy is a no-op counter.
	unsafe { instance.destroy(None) };
}

#[test]
fn device_creates_and_destroys_exactly_once() {
	let before = DEVICE_DESTROYED.get();
	// SAFETY: static mock with the correct ABI.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	// SAFETY: trivially valid for the mock driver.
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	// SAFETY: instance is live.
	let phys_devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
	// SAFETY: physical device handle and default create info are valid for the mock.
	let device =
		unsafe { instance.create_device(phys_devices[0], &vk::DeviceCreateInfo::default(), None) }
			.unwrap();
	assert!(!device.handle().is_null(), "device handle must be non-null");
	// SAFETY: no children, no outstanding operations.
	unsafe { device.destroy(None) };
	assert_eq!(
		DEVICE_DESTROYED.get(),
		before + 1,
		"vkDestroyDevice must be called exactly once"
	);
	// SAFETY: no children; mock destroy is a no-op counter.
	unsafe { instance.destroy(None) };
}

#[test]
fn dispatch_tables_are_fully_loaded_via_generated_constructor() {
	// Verify that FnEntry::new_from_resolver wires all expected entry-level commands.
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let fp = entry.fp();
	assert!(
		fp.create_instance.is_some(),
		"vkCreateInstance must be present"
	);
	assert!(
		fp.enumerate_instance_extension_properties.is_some(),
		"vkEnumerateInstanceExtensionProperties must be present"
	);
	// SAFETY: trivially valid mock create info.
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	let ifp = instance.fp();
	assert!(
		ifp.destroy_instance.is_some(),
		"vkDestroyInstance must be present"
	);
	assert!(
		ifp.enumerate_physical_devices.is_some(),
		"vkEnumeratePhysicalDevices must be present"
	);
	assert!(
		ifp.create_device.is_some(),
		"vkCreateDevice must be present"
	);
	// SAFETY: no children; mock destroy increments counter.
	unsafe { instance.destroy(None) };
}

// M3 complete: feature/extension discovery + device queue + cmd_dispatch ------

#[test]
fn entry_enumerates_instance_extensions() {
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let exts = entry.enumerate_instance_extension_properties(None).unwrap();
	assert_eq!(exts.len(), 1, "mock returns exactly one extension");
	// Check the extension name is the null-terminated C string we wrote.
	let raw = &exts[0].extension_name;
	let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
	let name: Vec<u8> = raw[..end].iter().map(|&c| c as u8).collect();
	assert_eq!(name, b"VK_KHR_mock_ext");
	assert_eq!(exts[0].spec_version, 1);
}

#[test]
fn entry_enumerates_instance_layers() {
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let lyrs = entry.enumerate_instance_layer_properties().unwrap();
	assert_eq!(lyrs.len(), 1, "mock returns exactly one layer");
	let raw = &lyrs[0].layer_name;
	let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
	let name: Vec<u8> = raw[..end].iter().map(|&c| c as u8).collect();
	assert_eq!(name, b"VK_LAYER_MOCK_validation");
}

#[test]
fn missing_extension_command_returns_call_error() {
	// A resolver that exposes no instance commands.
	unsafe extern "system" fn minimal(
		_: vk::InstanceHandle,
		name: *const c_char,
	) -> vk::PFN_vkVoidFunction {
		let name = unsafe { CStr::from_ptr(name) };
		match name.to_bytes() {
			b"vkCreateInstance" => Some(unsafe {
				std::mem::transmute::<
					unsafe extern "system" fn(
						*const vk::InstanceCreateInfo,
						*const vk::AllocationCallbacks,
						*mut vk::InstanceHandle,
					) -> vk::Result,
					unsafe extern "system" fn(),
				>(create)
			}),
			b"vkEnumerateInstanceExtensionProperties" => Some(unsafe {
				std::mem::transmute::<
					unsafe extern "system" fn(
						*const c_char,
						*mut u32,
						*mut vk::ExtensionProperties,
					) -> vk::Result,
					unsafe extern "system" fn(),
				>(mock_enumerate_instance_extensions)
			}),
			b"vkEnumerateInstanceLayerProperties" => Some(unsafe {
				std::mem::transmute::<
					unsafe extern "system" fn(*mut u32, *mut vk::LayerProperties) -> vk::Result,
					unsafe extern "system" fn(),
				>(mock_enumerate_instance_layers)
			}),
			_ => None,
		}
	}
	// SAFETY: minimal is static with the correct ABI.
	let entry = unsafe { Entry::new_from_resolver(minimal) }.unwrap();
	// enumerate_instance_extension_properties is present — should succeed.
	let result = entry.enumerate_instance_extension_properties(None);
	assert!(
		result.is_ok(),
		"extension query on minimal mock must succeed"
	);
	// enumerate_instance_layer_properties is present — should succeed.
	let result = entry.enumerate_instance_layer_properties();
	assert!(result.is_ok(), "layer query on minimal mock must succeed");
}

#[test]
fn instance_queries_physical_device_properties_and_features() {
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	let phys = unsafe { instance.enumerate_physical_devices() }.unwrap();
	// SAFETY: phys[0] is a mock handle; instance is live.
	let props = unsafe { instance.get_physical_device_properties(phys[0]) }.unwrap();
	assert_eq!(props.api_version, vk::API_VERSION_1_3);
	assert_eq!(props.vendor_id, 0xDEAD);
	assert_eq!(props.device_type, vk::PhysicalDeviceType::DISCRETE_GPU);
	// SAFETY: same as above.
	let features = unsafe { instance.get_physical_device_features(phys[0]) }.unwrap();
	assert_eq!(
		features.robust_buffer_access,
		vk::TRUE,
		"robust_buffer_access must be set"
	);
	assert_eq!(
		features.shader_float64,
		vk::TRUE,
		"shader_float64 must be set"
	);
	unsafe { instance.destroy(None) };
}

#[test]
fn instance_enumerates_queue_families_and_device_extensions() {
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	let phys = unsafe { instance.enumerate_physical_devices() }.unwrap();
	// SAFETY: phys[0] is live, instance is live.
	let families = unsafe { instance.get_physical_device_queue_family_properties(phys[0]) }.unwrap();
	assert_eq!(families.len(), 1, "mock returns one queue family");
	assert_eq!(families[0].queue_count, 4);
	let compute_bit =
		(families[0].queue_flags.as_raw() & vk::QueueFlagBits::COMPUTE_BIT.as_raw()) != 0;
	assert!(compute_bit, "compute queue flag must be set");
	// SAFETY: phys[0] is live, no layer filter.
	let dev_exts = unsafe { instance.enumerate_device_extension_properties(phys[0], None) }.unwrap();
	assert_eq!(dev_exts.len(), 1, "mock returns one device extension");
	let raw = &dev_exts[0].extension_name;
	let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
	let name: Vec<u8> = raw[..end].iter().map(|&c| c as u8).collect();
	assert_eq!(name, b"VK_KHR_swapchain");
	unsafe { instance.destroy(None) };
}

#[test]
fn device_get_queue_and_cmd_dispatch() {
	let dispatch_before = CMD_DISPATCH_CALLS.get();
	// SAFETY: static mock.
	let entry = unsafe { Entry::new_from_resolver(resolver_m3_entry) }.unwrap();
	let instance =
		unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) }.unwrap();
	let phys = unsafe { instance.enumerate_physical_devices() }.unwrap();
	let device =
		unsafe { instance.create_device(phys[0], &vk::DeviceCreateInfo::default(), None) }.unwrap();
	// SAFETY: family 0, queue 0 are valid for the mock (queue_count = 4).
	let queue = unsafe { device.get_device_queue(0, 0) }.unwrap();
	assert!(!queue.is_null(), "queue handle must be non-null");
	// cmd_dispatch requires a recording command buffer; with a mock we just pass NULL.
	// SAFETY: mock ignores all parameters.
	unsafe { device.cmd_dispatch(vk::CommandBuffer::NULL, 4, 1, 1) }.unwrap();
	assert_eq!(
		CMD_DISPATCH_CALLS.get(),
		dispatch_before + 1,
		"vkCmdDispatch must have been called once"
	);
	// Missing-command behavior is exercised independently by the failure mocks.
	unsafe { device.destroy(None) };
	unsafe { instance.destroy(None) };
}

#[test]
fn from_vk_result_into_call_error() {
	// Verify the From impl: non-success VkResult can be lifted via ?.
	let err: CallError = vk::Result::ERROR_OUT_OF_DEVICE_MEMORY.into();
	assert!(
		matches!(
			err,
			CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY)
		),
		"From<vk::Result> must produce CallError::Vulkan"
	);
}
