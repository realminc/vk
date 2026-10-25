//! Exact-ABI fault injection; no driver, allocation callback or GPU is invoked.
use super::*;
use crate::load::{CallError, CommandSupport, Entry, LoadError};
use std::{
	cell::RefCell,
	ffi::{CStr, c_char},
};

#[derive(Debug, PartialEq)]
struct Event(&'static str, u64, usize);

struct State {
	instance_status: vk::Result,
	device_status: vk::Result,
	null_instance: bool,
	null_device: bool,
	destroy_instance: bool,
	gdpa: bool,
	destroy_device: bool,
	fallback_destroy: bool,
	physical_version: u32,
	loader_version: u32,
	version_status: vk::Result,
	queries: usize,
	devices: u64,
	events: Vec<Event>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			instance_status: vk::Result::SUCCESS,
			device_status: vk::Result::SUCCESS,
			null_instance: false,
			null_device: false,
			destroy_instance: true,
			gdpa: true,
			destroy_device: true,
			fallback_destroy: true,
			physical_version: vk::API_VERSION_1_1,
			loader_version: vk::API_VERSION_1_1,
			version_status: vk::Result::SUCCESS,
			queries: 0,
			devices: 0,
			events: Vec::new(),
		}
	}
}

thread_local! { static STATE: RefCell<State> = RefCell::new(State::default()); }

fn reset() {
	STATE.with_borrow_mut(|state| *state = State::default());
}

macro_rules! erase {
	($ty:ty, $function:ident) => {
		// SAFETY: each match arm names the exact Vulkan command prototype.
		unsafe { std::mem::transmute::<$ty, vk::PFN_vkVoidFunction>(Some($function)) }
	};
}

unsafe extern "system" fn gipa(
	handle: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	STATE.with_borrow_mut(|state| {
		state.queries += 1;
		if !handle.is_null() {
			state
				.events
				.push(Event("instance resolve", handle.as_raw(), 0));
		}
	});
	// SAFETY: table loading/recovery supplies a static terminated command name.
	match unsafe { CStr::from_ptr(name) }.to_bytes() {
		b"vkCreateInstance" => erase!(vk::PFN_vkCreateInstance, create_instance),
		b"vkEnumerateInstanceVersion" => erase!(vk::PFN_vkEnumerateInstanceVersion, version),
		b"vkEnumeratePhysicalDeviceGroups" => erase!(vk::PFN_vkEnumeratePhysicalDeviceGroups, groups),
		b"vkEnumerateInstanceExtensionProperties" => {
			erase!(vk::PFN_vkEnumerateInstanceExtensionProperties, extensions)
		}
		b"vkEnumerateInstanceLayerProperties" => {
			erase!(vk::PFN_vkEnumerateInstanceLayerProperties, layers)
		}
		b"vkDestroyInstance" if STATE.with_borrow(|state| state.destroy_instance) => {
			erase!(vk::PFN_vkDestroyInstance, destroy_instance)
		}
		b"vkCreateDevice" => erase!(vk::PFN_vkCreateDevice, create_device),
		b"vkGetDeviceProcAddr" if STATE.with_borrow(|state| state.gdpa) => {
			erase!(vk::PFN_vkGetDeviceProcAddr, gdpa)
		}
		b"vkDestroyDevice" if STATE.with_borrow(|state| state.fallback_destroy) => {
			erase!(vk::PFN_vkDestroyDevice, destroy_device)
		}
		b"vkGetPhysicalDeviceProperties" => erase!(vk::PFN_vkGetPhysicalDeviceProperties, properties),
		b"vkGetPhysicalDeviceFeatures2" | b"vkGetPhysicalDeviceFeatures2KHR" => {
			erase!(vk::PFN_vkGetPhysicalDeviceFeatures2, features)
		}
		b"vkGetPhysicalDeviceVideoCapabilitiesKHR" => erase!(
			vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
			video_capabilities
		),
		_ => None,
	}
}

unsafe extern "system" fn gdpa(
	handle: vk::DeviceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	STATE.with_borrow_mut(|state| {
		state.queries += 1;
		state
			.events
			.push(Event("device resolve", handle.as_raw(), 0));
	});
	// SAFETY: loading/recovery supplies a valid terminated command name.
	match unsafe { CStr::from_ptr(name) }.to_bytes() {
		b"vkDestroyDevice" if STATE.with_borrow(|state| state.destroy_device) => {
			erase!(vk::PFN_vkDestroyDevice, destroy_device)
		}
		b"vkGetBufferMemoryRequirements2" | b"vkGetBufferMemoryRequirements2KHR" => {
			erase!(vk::PFN_vkGetBufferMemoryRequirements2, requirements)
		}
		_ => None,
	}
}

unsafe extern "system" fn create_instance(
	_: *const vk::InstanceCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::InstanceHandle,
) -> vk::Result {
	STATE.with_borrow_mut(|state| {
		state
			.events
			.push(Event("create instance", 0, callbacks as usize));
		// SAFETY: canonical creation supplies one writable native output slot.
		unsafe {
			*output = if state.null_instance {
				vk::InstanceHandle::NULL
			} else {
				vk::InstanceHandle::from_raw(0x1000)
			};
		}
		state.instance_status
	})
}

unsafe extern "system" fn create_device(
	_: vk::PhysicalDevice,
	_: *const vk::DeviceCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::DeviceHandle,
) -> vk::Result {
	STATE.with_borrow_mut(|state| {
		state.devices += 1;
		state
			.events
			.push(Event("create device", 0, callbacks as usize));
		// SAFETY: canonical creation supplies one writable output slot. On error
		// this deliberately poisons it; the caller must not adopt or destroy it.
		unsafe {
			*output = if state.null_device {
				vk::DeviceHandle::NULL
			} else {
				vk::DeviceHandle::from_raw(0x2000 + state.devices)
			};
		}
		state.device_status
	})
}

unsafe extern "system" fn destroy_instance(
	handle: vk::InstanceHandle,
	callbacks: *const vk::AllocationCallbacks,
) {
	STATE.with_borrow_mut(|state| {
		state.events.push(Event(
			"destroy instance",
			handle.as_raw(),
			callbacks as usize,
		))
	});
}

unsafe extern "system" fn destroy_device(
	handle: vk::DeviceHandle,
	callbacks: *const vk::AllocationCallbacks,
) {
	STATE.with_borrow_mut(|state| {
		state
			.events
			.push(Event("destroy device", handle.as_raw(), callbacks as usize))
	});
}

unsafe extern "system" fn properties(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceProperties,
) {
	// SAFETY: the wrapper supplies an initialized writable record.
	unsafe {
		(*output).api_version = STATE.with_borrow(|state| state.physical_version);
	}
}

unsafe extern "system" fn requirements(
	handle: vk::DeviceHandle,
	_: *const vk::BufferMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	STATE.with_borrow_mut(|state| state.events.push(Event("requirements", handle.as_raw(), 0)));
	// SAFETY: tests supply one initialized writable output record.
	unsafe {
		(*output).memory_requirements.size = 512;
	}
}

unsafe extern "system" fn features(_: vk::PhysicalDevice, _: *mut vk::PhysicalDeviceFeatures2) {}

unsafe extern "system" fn version(output: *mut u32) -> vk::Result {
	STATE.with_borrow(|state| {
		// SAFETY: Entry supplies one writable version output slot.
		unsafe {
			*output = state.loader_version;
		}
		state.version_status
	})
}

unsafe extern "system" fn groups(
	_: vk::InstanceHandle,
	count: *mut u32,
	_: *mut vk::PhysicalDeviceGroupProperties,
) -> vk::Result {
	// SAFETY: the enumeration wrapper supplies a writable count slot.
	unsafe {
		*count = 0;
	}
	vk::Result::SUCCESS
}
unsafe extern "system" fn video_capabilities(
	_: vk::PhysicalDevice,
	_: *const vk::VideoProfileInfoKHR,
	_: *mut vk::VideoCapabilitiesKHR,
) -> vk::Result {
	vk::Result::SUCCESS
}
unsafe extern "system" fn extensions(
	_: *const c_char,
	count: *mut u32,
	_: *mut vk::ExtensionProperties,
) -> vk::Result {
	// SAFETY: the query protocol supplies a writable count.
	unsafe {
		*count = 0;
	}
	vk::Result::SUCCESS
}
unsafe extern "system" fn layers(count: *mut u32, _: *mut vk::LayerProperties) -> vk::Result {
	// SAFETY: the query protocol supplies a writable count.
	unsafe {
		*count = 0;
	}
	vk::Result::SUCCESS
}

fn callbacks() -> vk::AllocationCallbacks {
	// The mock records pointer identity without invoking callbacks.
	vk::AllocationCallbacks {
		p_user_data: std::ptr::null_mut(),
		pfn_allocation: None,
		pfn_reallocation: None,
		pfn_free: None,
		pfn_internal_allocation: None,
		pfn_internal_free: None,
	}
}

fn count(action: &str) -> usize {
	STATE.with_borrow(|state| {
		state
			.events
			.iter()
			.filter(|event| event.0 == action)
			.count()
	})
}

unsafe fn parent(version: u32) -> (Entry, vk::Instance) {
	// SAFETY: static mocks accept these fully live local create-info records.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		let app = vk::ApplicationInfo {
			api_version: version,
			..Default::default()
		};
		let instance = entry
			.create_instance(
				&vk::InstanceCreateInfo {
					p_application_info: &app,
					..Default::default()
				},
				None,
			)
			.unwrap();
		(entry, instance)
	}
}

#[test]
fn instance_driver_error_and_null_success_never_load_or_destroy_outputs() {
	reset();
	// SAFETY: exact-ABI mocks deliberately return failures/invalid outputs.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		STATE.with_borrow_mut(|state| state.instance_status = vk::Result::ERROR_OUT_OF_HOST_MEMORY);
		assert!(matches!(
			entry.create_instance(&vk::InstanceCreateInfo::default(), None),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		));
		STATE.with_borrow_mut(|state| {
			state.instance_status = vk::Result::SUCCESS;
			state.null_instance = true;
		});
		assert!(matches!(
			entry.create_instance(&vk::InstanceCreateInfo::default(), None),
			Err(CallError::InvalidOutput(_))
		));
	}
	assert_eq!(count("instance resolve"), 0);
	assert_eq!(count("destroy instance"), 0);
}

#[test]
fn missing_instance_destruction_returns_recoverable_owner_and_original_callbacks() {
	reset();
	// SAFETY: static exact-ABI mocks accept inert callbacks and fake handles.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		STATE.with_borrow_mut(|state| state.destroy_instance = false);
		let callbacks = callbacks();
		let Err(CallError::InitializationFailed(failure)) =
			entry.create_instance(&vk::InstanceCreateInfo::default(), Some(&callbacks))
		else {
			panic!("expected retained native instance");
		};
		assert_eq!(
			failure.handle(),
			CreatedHandle::Instance(vk::InstanceHandle::from_raw(0x1000))
		);
		assert_eq!(failure.missing_command(), "vkDestroyInstance");
		let failure = failure.destroy(Some(&callbacks)).unwrap_err();
		drop(entry);
		STATE.with_borrow_mut(|state| state.destroy_instance = true);
		failure.destroy(Some(&callbacks)).unwrap();
		assert_eq!(count("destroy instance"), 1);
		assert!(STATE.with_borrow(|state| state.events.contains(&Event(
			"destroy instance",
			0x1000,
			&callbacks as *const _ as usize
		))));
	}
}

#[test]
fn missing_device_resolver_prevents_native_creation() {
	reset();
	STATE.with_borrow_mut(|state| state.gdpa = false);
	// SAFETY: exact-ABI mock context has no native resources.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_0);
		assert!(matches!(
			instance.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo::default(),
				None
			),
			Err(CallError::MissingCommand("vkGetDeviceProcAddr"))
		));
		assert_eq!(count("create device"), 0);
		instance.destroy(None);
	}
}

#[test]
fn invalid_metadata_or_version_query_failure_prevents_native_creation() {
	reset();
	// SAFETY: null arrays are rejected before dereference; version errors have
	// no native creation side effects in this exact-ABI mock.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		let invalid = vk::InstanceCreateInfo {
			enabled_extension_count: 1,
			..Default::default()
		};
		assert!(matches!(
			entry.create_instance(&invalid, None),
			Err(CallError::InvalidOutput(_))
		));
		STATE.with_borrow_mut(|state| state.version_status = vk::Result::ERROR_OUT_OF_HOST_MEMORY);
		assert!(matches!(
			entry.create_instance(&vk::InstanceCreateInfo::default(), None),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		));
		assert_eq!(count("create instance"), 0);
	}
}

#[test]
fn instance_commands_use_loader_version_without_capping_physical_device_queries() {
	reset();
	STATE.with_borrow_mut(|state| state.loader_version = vk::API_VERSION_1_0);
	// SAFETY: exact-ABI mocks model a requested API above the instance ceiling,
	// with independently resolvable physical-device command addresses.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_1);
		assert_eq!(
			instance.capabilities().unwrap().instance_api_version(),
			vk::API_VERSION_1_0
		);
		assert!(instance.fp().enumerate_physical_device_groups.is_none());
		assert!(instance.fp().get_physical_device_features2.is_some());
		instance.destroy(None);
	}
}

#[test]
fn device_post_creation_failure_rolls_back_through_instance_dispatch_once() {
	reset();
	STATE.with_borrow_mut(|state| state.destroy_device = false);
	// SAFETY: exact-ABI mocks record, but never invoke, inert callbacks.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_0);
		let callbacks = callbacks();
		assert!(matches!(
			instance.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo::default(),
				Some(&callbacks)
			),
			Err(CallError::MissingCommand("vkDestroyDevice"))
		));
		assert_eq!(count("destroy device"), 1);
		assert!(STATE.with_borrow(|state| state.events.contains(&Event(
			"destroy device",
			0x2001,
			&callbacks as *const _ as usize
		))));
		instance.destroy(None);
		assert_eq!(count("destroy device"), 1);
	}
}

#[test]
fn unavailable_rollback_preserves_device_until_explicit_retry() {
	reset();
	STATE.with_borrow_mut(|state| {
		state.destroy_device = false;
		state.fallback_destroy = false;
	});
	// SAFETY: no native resources or work; parent remains live through recovery.
	unsafe {
		let (entry, instance) = parent(vk::API_VERSION_1_0);
		let Err(CallError::InitializationFailed(failure)) = instance.create_device(
			vk::PhysicalDevice::NULL,
			&vk::DeviceCreateInfo::default(),
			None,
		) else {
			panic!("expected retained native device");
		};
		assert_eq!(
			failure.handle(),
			CreatedHandle::Device(vk::DeviceHandle::from_raw(0x2001))
		);
		let failure = failure.destroy(None).unwrap_err();
		assert_eq!(count("destroy device"), 0);
		drop(entry);
		STATE.with_borrow_mut(|state| state.destroy_device = true);
		failure.destroy(None).unwrap();
		assert_eq!(count("destroy device"), 1);
		instance.destroy(None);
	}
}

#[test]
fn device_driver_error_and_null_success_never_resolve_or_destroy_poisoned_outputs() {
	reset();
	// SAFETY: mocks return deliberately invalid outputs without native side effects.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_0);
		STATE.with_borrow_mut(|state| state.device_status = vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
		assert!(matches!(
			instance.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo::default(),
				None
			),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY))
		));
		STATE.with_borrow_mut(|state| {
			state.device_status = vk::Result::SUCCESS;
			state.null_device = true;
		});
		assert!(matches!(
			instance.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo::default(),
				None
			),
			Err(CallError::InvalidOutput(_))
		));
		assert_eq!(count("device resolve"), 0);
		assert_eq!(count("destroy device"), 0);
		instance.destroy(None);
	}
}

#[test]
fn imports_reject_null_without_lookup_and_preserve_unknown_provenance() {
	reset();
	// SAFETY: fake opaque tokens are accepted only by these exact-ABI mocks.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		let before = STATE.with_borrow(|state| state.queries);
		assert!(matches!(
			vk::Instance::new_from_handle(&entry, vk::InstanceHandle::NULL),
			Err(LoadError::NullHandle("instance"))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), before);
		let instance =
			vk::Instance::new_from_handle(&entry, vk::InstanceHandle::from_raw(0x1000)).unwrap();
		assert!(instance.capabilities().is_none());
		let before = STATE.with_borrow(|state| state.queries);
		assert!(matches!(
			vk::Device::new_from_handle(&instance, vk::DeviceHandle::NULL),
			Err(LoadError::NullHandle("device"))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), before);
		STATE.with_borrow_mut(|state| state.destroy_device = false);
		let handle = vk::DeviceHandle::from_raw(0x2000);
		assert!(matches!(
			vk::Device::new_from_handle(&instance, handle),
			Err(LoadError::MissingSymbol("vkDestroyDevice"))
		));
		assert_eq!(count("destroy device"), 0);
		STATE.with_borrow_mut(|state| state.destroy_device = true);
		let device = vk::Device::new_from_handle(&instance, handle).unwrap();
		assert!(device.capabilities().is_none());
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn device_version_extensions_aliases_and_cache_are_context_specific() {
	reset();
	STATE.with_borrow_mut(|state| state.physical_version = vk::API_VERSION_1_0);
	// SAFETY: the mock accepts all local records and opaque physical-device tokens.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_1);
		assert!(instance.fp().get_physical_device_features2.is_some());
		assert!(instance.fp().get_physical_device_features2_khr.is_none());
		assert!(
			instance
				.fp()
				.get_physical_device_video_capabilities_khr
				.is_some()
		);
		assert_eq!(
			instance
				.capabilities()
				.unwrap()
				.command_support(c"vkGetPhysicalDeviceVideoCapabilitiesKHR"),
			Some(CommandSupport::RequiresPhysicalDevice)
		);
		let names = [c"VK_KHR_get_memory_requirements2".as_ptr()];
		let info = vk::DeviceCreateInfo {
			enabled_extension_count: 1,
			pp_enabled_extension_names: names.as_ptr(),
			..Default::default()
		};
		let extension = instance
			.create_device(vk::PhysicalDevice::NULL, &info, None)
			.unwrap();
		assert_eq!(
			extension.capabilities().unwrap().api_version(),
			vk::API_VERSION_1_0
		);
		assert!(extension.fp().get_buffer_memory_requirements2.is_none());
		assert!(extension.fp().get_buffer_memory_requirements2_khr.is_some());
		let baseline = instance
			.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo::default(),
				None,
			)
			.unwrap();
		assert!(baseline.fp().get_buffer_memory_requirements2_khr.is_none());
		assert_eq!(
			baseline.cmd_dispatch(vk::CommandBuffer::NULL, 1, 1, 1),
			Err(CallError::MissingCommand("vkCmdDispatch"))
		);
		let queries = STATE.with_borrow(|state| state.queries);
		let mut output = vk::MemoryRequirements2::default();
		assert_eq!(
			extension.get_buffer_memory_requirements2(
				&vk::BufferMemoryRequirementsInfo2::default(),
				&mut output
			),
			Err(CallError::MissingCommand("vkGetBufferMemoryRequirements2"))
		);
		extension
			.get_buffer_memory_requirements2_khr(
				&vk::BufferMemoryRequirementsInfo2::default(),
				&mut output,
			)
			.unwrap();
		assert_eq!(output.memory_requirements.size, 512);
		assert_eq!(STATE.with_borrow(|state| state.queries), queries);
		assert_eq!(count("requirements"), 1);
		extension.destroy(None);
		baseline.destroy(None);
		instance.destroy(None);
	}
}

#[cfg(unix)]
#[test]
fn failed_initialization_retains_actual_library_owner_through_recovery() {
	reset();
	// SAFETY: this opens the current process, not a driver; resolver mocks own all
	// native behavior. The library Arc serves only as a real lifetime sentinel.
	unsafe {
		let library: Library = libloading::os::unix::Library::this().into();
		let library = Arc::new(library);
		let weak = Arc::downgrade(&library);
		let mut entry = Entry::new_from_resolver(gipa).unwrap();
		entry.library = Some(library);
		STATE.with_borrow_mut(|state| state.destroy_instance = false);
		let Err(CallError::InitializationFailed(failure)) =
			entry.create_instance(&vk::InstanceCreateInfo::default(), None)
		else {
			panic!("expected failure");
		};
		drop(entry);
		assert!(weak.upgrade().is_some());
		STATE.with_borrow_mut(|state| state.destroy_instance = true);
		failure.destroy(None).unwrap();
		assert!(weak.upgrade().is_none());
	}
}

#[test]
fn metadata_imports_filter_once_and_keep_owned_features_after_input_drop() {
	use crate::load::Capabilities;
	reset();
	// SAFETY: exact-ABI resolver accepts these opaque tokens; local metadata is valid.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		let instance_extensions = [c"VK_KHR_get_physical_device_properties2".as_ptr()];
		let parent = Capabilities::new_from_instance_create_info(
			&vk::InstanceCreateInfo {
				enabled_extension_count: 1,
				pp_enabled_extension_names: instance_extensions.as_ptr(),
				..Default::default()
			},
			vk::API_VERSION_1_1,
		)
		.unwrap();
		let instance = vk::Instance::new_from_handle_with_capabilities(
			&entry,
			vk::InstanceHandle::from_raw(0x1000),
			parent,
		)
		.unwrap();
		assert!(instance.fp().get_physical_device_features2.is_none());
		let metadata = {
			let sync = vk::PhysicalDeviceSynchronization2Features {
				synchronization2: vk::TRUE,
				..Default::default()
			};
			let names = [c"VK_KHR_synchronization2".as_ptr()];
			instance
				.capabilities()
				.unwrap()
				.new_from_device_create_info(
					&vk::DeviceCreateInfo {
						p_next: (&sync as *const vk::PhysicalDeviceSynchronization2Features).cast(),
						enabled_extension_count: 1,
						pp_enabled_extension_names: names.as_ptr(),
						..Default::default()
					},
					vk::API_VERSION_1_1,
				)
				.unwrap()
		};
		let device = vk::Device::new_from_handle_with_capabilities(
			&instance,
			vk::DeviceHandle::from_raw(0x2000),
			metadata,
		)
		.unwrap();
		assert!(device.fp().get_buffer_memory_requirements2.is_none());
		let capabilities = device.capabilities().unwrap();
		assert_eq!(
			capabilities.features().unwrap().enabled(
				vk::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
				"synchronization2"
			),
			Some(true)
		);
		assert_eq!(
			capabilities.command_support(c"vkQueueSubmit2KHR"),
			Some(CommandSupport::Enabled)
		);
		assert_eq!(
			capabilities.command_support(c"vkQueueSubmit2"),
			Some(CommandSupport::Disabled)
		);
		let queries = STATE.with_borrow(|state| state.queries);
		assert!(matches!(
			device.get_buffer_memory_requirements2(
				&vk::BufferMemoryRequirementsInfo2::default(),
				&mut vk::MemoryRequirements2::default()
			),
			Err(CallError::MissingCommand(_))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), queries);
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn metadata_import_rejection_has_no_resolution_destruction_or_ownership_transfer() {
	use crate::load::Capabilities;
	reset();
	// SAFETY: initialized local metadata and opaque tokens understood only by mocks.
	unsafe {
		let entry = Entry::new_from_resolver(gipa).unwrap();
		let capture = || {
			Capabilities::new_from_instance_create_info(
				&vk::InstanceCreateInfo::default(),
				vk::API_VERSION_1_1,
			)
			.unwrap()
		};
		let device_metadata = capture()
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_1)
			.unwrap();
		let queries = STATE.with_borrow(|state| state.queries);
		assert!(matches!(
			vk::Instance::new_from_handle_with_capabilities(
				&entry,
				vk::InstanceHandle::from_raw(0x1000),
				device_metadata
			),
			Err(LoadError::InvalidMetadata(_))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), queries);
		let instance = vk::Instance::new_from_handle_with_capabilities(
			&entry,
			vk::InstanceHandle::from_raw(0x1000),
			capture(),
		)
		.unwrap();
		let queries = STATE.with_borrow(|state| state.queries);
		assert!(matches!(
			vk::Device::new_from_handle_with_capabilities(
				&instance,
				vk::DeviceHandle::from_raw(0x2000),
				capture()
			),
			Err(LoadError::InvalidMetadata(_))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), queries);
		let names = [c"VK_KHR_surface".as_ptr()];
		let foreign_parent = Capabilities::new_from_instance_create_info(
			&vk::InstanceCreateInfo {
				enabled_extension_count: 1,
				pp_enabled_extension_names: names.as_ptr(),
				..Default::default()
			},
			vk::API_VERSION_1_1,
		)
		.unwrap();
		let foreign_device = foreign_parent
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_1)
			.unwrap();
		assert!(matches!(
			vk::Device::new_from_handle_with_capabilities(
				&instance,
				vk::DeviceHandle::from_raw(0x2000),
				foreign_device
			),
			Err(LoadError::InvalidMetadata(_))
		));
		assert_eq!(STATE.with_borrow(|state| state.queries), queries);
		let metadata = capture()
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_1)
			.unwrap();
		STATE.with_borrow_mut(|state| state.destroy_device = false);
		assert!(matches!(
			vk::Device::new_from_handle_with_capabilities(
				&instance,
				vk::DeviceHandle::from_raw(0x2000),
				metadata
			),
			Err(LoadError::MissingSymbol("vkDestroyDevice"))
		));
		assert_eq!(count("destroy device"), 0);
		assert_eq!(count("create device"), 0);
		STATE.with_borrow_mut(|state| state.destroy_device = true);
		// Failed imports leave the raw resource with the caller, who can retry.
		let device =
			vk::Device::new_from_handle(&instance, vk::DeviceHandle::from_raw(0x2000)).unwrap();
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn invalid_feature_chain_prevents_native_creation() {
	reset();
	// SAFETY: readable local records; the duplicate feature combination is rejected.
	unsafe {
		let (_, instance) = parent(vk::API_VERSION_1_0);
		let tail = vk::PhysicalDeviceSynchronization2Features::default();
		let head = vk::PhysicalDeviceSynchronization2Features {
			p_next: (&tail as *const vk::PhysicalDeviceSynchronization2Features)
				.cast_mut()
				.cast(),
			..Default::default()
		};
		assert!(matches!(
			instance.create_device(
				vk::PhysicalDevice::NULL,
				&vk::DeviceCreateInfo {
					p_next: (&head as *const vk::PhysicalDeviceSynchronization2Features).cast(),
					..Default::default()
				},
				None
			),
			Err(CallError::InvalidMetadata(_))
		));
		assert_eq!(count("create device"), 0);
		instance.destroy(None);
	}
}
