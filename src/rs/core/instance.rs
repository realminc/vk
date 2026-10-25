//! Explicit instance destruction with retained native-library ownership.
use crate::core as vk;
use crate::load::{
	CallError, Capabilities, CommandSupport, CreatedHandle, Entry, InitializationFailure, LoadError,
};
use libloading::Library;
use std::sync::Arc;

/// Instance handle and dispatch. Call `destroy` explicitly; Drop never destroys Vulkan objects.
pub struct Instance {
	handle: vk::InstanceHandle,
	fp: vk::FnInstance,
	pub(super) gipa: unsafe extern "system" fn(
		vk::InstanceHandle,
		*const std::ffi::c_char,
	) -> vk::PFN_vkVoidFunction,
	pub(crate) library: Option<Arc<Library>>,
	pub(crate) capabilities: Option<Capabilities>,
}

impl Instance {
	/// Associate a live raw instance with the entry that created it.
	///
	/// # Safety
	/// `handle` must be live and belong to `entry`. The caller transfers the
	/// responsibility to destroy it once and must not create duplicate owners.
	/// Ownership remains with the caller on error. Creation metadata is unknown
	/// on this import path; command admission remains the caller's obligation.
	pub unsafe fn new_from_handle(
		entry: &Entry,
		handle: vk::InstanceHandle,
	) -> Result<Self, LoadError> {
		if handle.is_null() {
			return Err(LoadError::NullHandle("instance"));
		}
		// SAFETY: caller guarantees a compatible live instance; gipa resolves by name.
		let fp = unsafe { vk::FnInstance::new_from_resolver(entry.gipa, handle) };
		if fp.destroy_instance.is_none() {
			return Err(LoadError::MissingSymbol("vkDestroyInstance"));
		}
		Ok(Self {
			handle,
			fp,
			gipa: entry.gipa,
			library: entry.library.clone(),
			capabilities: None,
		})
	}

	/// Import a native instance with an owned creation snapshot and filtered dispatch.
	///
	/// # Errors
	/// Reports wrong snapshot scope, a null handle or unavailable destruction.
	/// Every error leaves native ownership with the caller.
	///
	/// # Safety
	/// The handle must be live, belong to entry and have no other wrapper owner.
	/// Capabilities must truthfully describe this exact instance's creation and
	/// implementation. Caller retains parent/callback lifetime obligations.
	pub unsafe fn new_from_handle_with_capabilities(
		entry: &Entry,
		handle: vk::InstanceHandle,
		capabilities: Capabilities,
	) -> Result<Self, LoadError> {
		if capabilities.is_device() {
			return Err(LoadError::InvalidMetadata(
				"instance import needs instance capabilities",
			));
		}
		// SAFETY: caller guarantees the live compatible handle and exclusive import.
		let mut instance = unsafe { Self::new_from_handle(entry, handle) }?;
		instance
			.fp
			.retain_enabled(|command| capabilities.support(command) != CommandSupport::Disabled);
		instance.capabilities = Some(capabilities);
		Ok(instance)
	}

	/// Return the raw handle. Its use must respect this instance's lifetime.
	pub fn handle(&self) -> vk::InstanceHandle {
		self.handle
	}

	/// Borrow the full instance dispatch table.
	pub fn fp(&self) -> &vk::FnInstance {
		&self.fp
	}

	/// Creation version/extension provenance, or None for an imported raw handle.
	pub fn capabilities(&self) -> Option<&Capabilities> {
		self.capabilities.as_ref()
	}

	/// Destroy once, using callbacks compatible with instance creation.
	///
	/// # Safety
	/// All children and outstanding instance operations must be complete. The
	/// allocator must match creation, with its user data and functions still live.
	pub unsafe fn destroy(self, allocator: Option<&vk::AllocationCallbacks>) {
		// SAFETY: caller supplies the canonical destruction guarantees.
		unsafe { destroy_instance(self, allocator) };
	}
}

/// Create a headless or extension-configured Vulkan instance.
///
/// # Errors
/// Reports host metadata/version-query failures before creation, driver errors,
/// invalid native outputs and unavailable destruction. If rollback is impossible,
/// `CallError::InitializationFailed` retains the native object for explicit recovery.
///
/// # Safety
/// The create info, its reachable pointers and allocator callbacks must meet
/// Vulkan valid usage. Allocator state must remain alive until destruction.
pub unsafe fn create_instance(
	entry: &Entry,
	create_info: &vk::InstanceCreateInfo,
	allocator: Option<&vk::AllocationCallbacks>,
) -> Result<Instance, CallError> {
	// Capture all fallible host metadata before creating the native object.
	// SAFETY: caller guarantees validity of all reachable create-info pointers.
	let mut capabilities = unsafe { Capabilities::instance(create_info) }?;
	capabilities.limit_instance_version(entry.enumerate_instance_version()?)?;
	let allocator_ptr = allocator.map_or(std::ptr::null(), |a| a);
	let mut handle = vk::InstanceHandle::NULL;
	// SAFETY: caller guarantees create-info validity; global PFN is validated.
	let result =
		unsafe { entry.fp().create_instance.unwrap()(create_info, allocator_ptr, &mut handle) };
	if result != vk::Result::SUCCESS {
		return Err(CallError::Vulkan(result));
	}
	if handle.is_null() {
		return Err(CallError::InvalidOutput(
			"vkCreateInstance returned a null handle",
		));
	}
	// SAFETY: native success supplied a non-null instance from this entry.
	let mut fp = unsafe { vk::FnInstance::new_from_resolver(entry.gipa, handle) };
	if fp.destroy_instance.is_none() {
		let failure = InitializationFailure {
			handle: CreatedHandle::Instance(handle),
			missing_command: "vkDestroyInstance",
			library: entry.library.clone(),
			instance: handle,
			gipa: entry.gipa,
			gdpa: None,
		};
		// SAFETY: the new instance has no children/work; callbacks match creation.
		return match unsafe { failure.destroy(allocator) } {
			Ok(()) => Err(CallError::MissingCommand("vkDestroyInstance")),
			Err(failure) => Err(CallError::InitializationFailed(failure)),
		};
	}
	// Device-extension physical-device queries cannot be admitted from an
	// instance's enabled extensions alone. Retain them for the unsafe query API.
	fp.retain_enabled(|command| capabilities.support(command) != CommandSupport::Disabled);
	Ok(Instance {
		handle,
		fp,
		gipa: entry.gipa,
		library: entry.library.clone(),
		capabilities: Some(capabilities),
	})
}

impl Instance {
	/// Create an instance using cached entry dispatch.
	///
	/// # Safety
	/// The create info and allocator must satisfy Vulkan instance creation and destruction requirements.
	pub unsafe fn new(
		entry: &Entry,
		create_info: &vk::InstanceCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<Self, CallError> {
		// SAFETY: caller provides the canonical creation guarantees.
		unsafe { create_instance(entry, create_info, allocator) }
	}
}

use super::command::commands;
commands! { Instance, instance;
	/// Resolve an instance command using this instance's retained loader.
	///
	/// # Safety
	/// Cast to the command's exact ABI, obey its enabled-version/extension
	/// requirements and keep the instance and native library alive during use.
	pub unsafe fn get_instance_proc_addr(name: &std::ffi::CStr) -> vk::PFN_vkVoidFunction {
		// SAFETY: this wrapper retains the library and caller guarantees instance lifetime.
		unsafe { (instance.gipa)(instance.handle, name.as_ptr()) }
	}

	/// Enumerate physical devices available to this instance.
	///
	/// # Safety
	/// The instance must be live and not destroyed.
	pub unsafe fn enumerate_physical_devices() -> Result<Vec<vk::PhysicalDevice>, CallError> {
		let f = instance
			.fp
			.enumerate_physical_devices
			.ok_or(CallError::MissingCommand("vkEnumeratePhysicalDevices"))?;
		// SAFETY: caller guarantees a live instance; PFN implements enumeration.
		unsafe { crate::load::enumerate::enumerate(|count, data| f(instance.handle, count, data)) }
	}

	/// Query properties of a physical device (name, driver version, limits, …).
	///
	/// # Safety
	/// The instance must be live and `physical_device` must belong to it.
	pub unsafe fn get_physical_device_properties(
		physical_device: vk::PhysicalDevice,
	) -> Result<vk::PhysicalDeviceProperties, CallError> {
		let f = instance
			.fp
			.get_physical_device_properties
			.ok_or(CallError::MissingCommand("vkGetPhysicalDeviceProperties"))?;
		// SAFETY: PhysicalDeviceProperties is a C struct of integers, enum and arrays; zeroed is valid.
		let mut props = unsafe { std::mem::zeroed::<vk::PhysicalDeviceProperties>() };
		// SAFETY: caller guarantees the physical device and instance are live.
		unsafe { f(physical_device, &mut props) };
		Ok(props)
	}

	/// Query the optional features supported by a physical device.
	///
	/// # Safety
	/// The instance must be live and `physical_device` must belong to it.
	pub unsafe fn get_physical_device_features(
		physical_device: vk::PhysicalDevice,
	) -> Result<vk::PhysicalDeviceFeatures, CallError> {
		let f = instance
			.fp
			.get_physical_device_features
			.ok_or(CallError::MissingCommand("vkGetPhysicalDeviceFeatures"))?;
		// SAFETY: PhysicalDeviceFeatures is a C struct of VkBool32 (u32) fields; zeroed is valid.
		let mut features = unsafe { std::mem::zeroed::<vk::PhysicalDeviceFeatures>() };
		// SAFETY: caller guarantees the physical device and instance are live.
		unsafe { f(physical_device, &mut features) };
		Ok(features)
	}

	/// Enumerate queue families and their properties for a physical device.
	///
	/// # Safety
	/// The instance must be live and `physical_device` must belong to it.
	pub unsafe fn get_physical_device_queue_family_properties(
		physical_device: vk::PhysicalDevice,
	) -> Result<Vec<vk::QueueFamilyProperties>, CallError> {
		let f = instance
			.fp
			.get_physical_device_queue_family_properties
			.ok_or(CallError::MissingCommand(
				"vkGetPhysicalDeviceQueueFamilyProperties",
			))?;
		// SAFETY: the live physical device and PFN obey the count/data protocol.
		unsafe { crate::load::enumerate::enumerate_void(|count, data| f(physical_device, count, data)) }
	}

	/// Enumerate extensions supported by a physical device (optionally filtered by layer).
	///
	/// Pass `layer_name = None` to query all extensions from the ICD.
	///
	/// # Safety
	/// The instance must be live and `physical_device` must belong to it.
	pub unsafe fn enumerate_device_extension_properties(
		physical_device: vk::PhysicalDevice,
		layer_name: Option<&std::ffi::CStr>,
	) -> Result<Vec<vk::ExtensionProperties>, CallError> {
		let f = instance
			.fp
			.enumerate_device_extension_properties
			.ok_or(CallError::MissingCommand(
				"vkEnumerateDeviceExtensionProperties",
			))?;
		let layer_ptr = layer_name.map_or(std::ptr::null(), |l| l.as_ptr());
		// SAFETY: caller guarantees a live device and layer name; PFN implements enumeration.
		unsafe {
			crate::load::enumerate::enumerate(|count, data| f(physical_device, layer_ptr, count, data))
		}
	}

	/// Query the memory heaps and memory types available to a physical device.
	///
	/// # Safety
	/// The instance must be live and `physical_device` must belong to it.
	pub unsafe fn get_physical_device_memory_properties(
		physical_device: vk::PhysicalDevice,
	) -> Result<vk::PhysicalDeviceMemoryProperties, CallError> {
		let f = instance
			.fp
			.get_physical_device_memory_properties
			.ok_or(CallError::MissingCommand(
				"vkGetPhysicalDeviceMemoryProperties",
			))?;
		// SAFETY: PhysicalDeviceMemoryProperties is a C struct of integers and arrays; zeroed is valid.
		let mut props = unsafe { std::mem::zeroed::<vk::PhysicalDeviceMemoryProperties>() };
		// SAFETY: caller guarantees the physical device and instance are live.
		unsafe { f(physical_device, &mut props) };
		Ok(props)
	}

}

/// Destroy once, using callbacks compatible with instance creation.
///
/// # Safety
/// All children and outstanding instance operations must be complete. The
/// allocator must match creation, with its user data and functions still live.
pub unsafe fn destroy_instance(instance: Instance, allocator: Option<&vk::AllocationCallbacks>) {
	// SAFETY: caller guarantees all destruction and allocator requirements.
	unsafe {
		instance.fp.destroy_instance.unwrap()(
			instance.handle,
			allocator.map_or(std::ptr::null(), |a| a),
		)
	};
}
