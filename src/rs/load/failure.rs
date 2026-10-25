//! Recovery when a driver creates an object but does not expose its destruction command.
use crate::core as vk;
use libloading::Library;
use std::{ffi::CStr, sync::Arc};

/// A native object returned successfully before dispatch initialization failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatedHandle {
	Instance(vk::InstanceHandle),
	Device(vk::DeviceHandle),
}

/// Retains a created handle and its library when automatic rollback is impossible.
/// The native object still needs explicit cleanup; dropping this error performs
/// no destruction. Keep its parent and original allocator callback state alive.
#[derive(Debug)]
#[must_use = "a created native object still needs explicit recovery"]
pub struct InitializationFailure {
	pub(crate) handle: CreatedHandle,
	pub(crate) missing_command: &'static str,
	pub(crate) library: Option<Arc<Library>>,
	pub(crate) instance: vk::InstanceHandle,
	pub(crate) gipa: unsafe extern "system" fn(
		vk::InstanceHandle,
		*const std::ffi::c_char,
	) -> vk::PFN_vkVoidFunction,
	pub(crate) gdpa: vk::PFN_vkGetDeviceProcAddr,
}

impl PartialEq for InitializationFailure {
	fn eq(&self, other: &Self) -> bool {
		// Error equality identifies the same native resource and missing command.
		self.handle == other.handle
			&& self.instance == other.instance
			&& self.missing_command == other.missing_command
	}
}

impl std::fmt::Display for InitializationFailure {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"missing {}; {:?} requires recovery",
			self.missing_command, self.handle
		)
	}
}

impl std::error::Error for InitializationFailure {}

impl InitializationFailure {
	fn exported(&self, name: &CStr) -> vk::PFN_vkVoidFunction {
		// SAFETY: the retained library was loaded under Entry's Vulkan ABI contract.
		// The erased pointer is restored only to the command's exact prototype.
		unsafe {
			self.library.as_ref().and_then(|library| {
				library
					.get::<unsafe extern "system" fn()>(name.to_bytes_with_nul())
					.ok()
					.map(|symbol| *symbol)
			})
		}
	}

	/// Native object retained by this failure. Do not create duplicate owners.
	pub fn handle(&self) -> CreatedHandle {
		self.handle
	}

	/// Required command whose absence prevented wrapper initialization.
	pub fn missing_command(&self) -> &'static str {
		self.missing_command
	}

	/// Retry explicit cleanup; on failure return the same owner for further recovery.
	/// No submission or wait is performed.
	///
	/// # Safety
	/// The native object and its parents must still be live, with no children or
	/// outstanding operations. Callbacks must match creation and remain valid.
	/// Do not separately destroy the exposed handle before calling this method.
	pub unsafe fn destroy(self, allocator: Option<&vk::AllocationCallbacks>) -> Result<(), Self> {
		let allocator = allocator.map_or(std::ptr::null(), |value| value);
		// Keep the library loaded through resolution and native cleanup.
		let _retained = &self.library;
		match self.handle {
			CreatedHandle::Instance(handle) => {
				// SAFETY: caller guarantees the instance and its resolver remain valid.
				let function = unsafe { (self.gipa)(handle, c"vkDestroyInstance".as_ptr()) }
					.or_else(|| self.exported(c"vkDestroyInstance"));
				// SAFETY: this exact command has the Vulkan destruction prototype.
				let function = unsafe {
					std::mem::transmute::<vk::PFN_vkVoidFunction, vk::PFN_vkDestroyInstance>(function)
				};
				let Some(function) = function else {
					return Err(self);
				};
				// SAFETY: caller supplies native destruction and callback guarantees.
				unsafe { function(handle, allocator) };
			}
			CreatedHandle::Device(handle) => {
				// SAFETY: the retained resolver and caller-owned parent remain valid.
				let function = unsafe {
					self
						.gdpa
						.and_then(|load| load(handle, c"vkDestroyDevice".as_ptr()))
						.or_else(|| (self.gipa)(self.instance, c"vkDestroyDevice".as_ptr()))
						.or_else(|| self.exported(c"vkDestroyDevice"))
				};
				// SAFETY: this exact command has the Vulkan destruction prototype.
				let function = unsafe {
					std::mem::transmute::<vk::PFN_vkVoidFunction, vk::PFN_vkDestroyDevice>(function)
				};
				let Some(function) = function else {
					return Err(self);
				};
				// SAFETY: caller supplies native destruction and callback guarantees.
				unsafe { function(handle, allocator) };
			}
		}
		Ok(())
	}
}

#[cfg(test)]
#[path = "../../../test/rs/load/test_failure.rs"]
mod tests;
