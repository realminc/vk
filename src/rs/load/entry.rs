//! Dynamic library ownership and checked Vulkan global dispatch.
use crate::Instance;
use crate::{self as vk, make_api_version};
use libloading::Library;
use std::{
	ffi::{CStr, OsStr},
	sync::Arc,
};

/// Loading or required-command resolution failure.
#[derive(Debug, PartialEq)]
pub enum LoadError {
	InvalidMetadata(&'static str),
	/// Native library opening or symbol lookup failed.
	LibraryOpen(String),
	/// A required Vulkan function could not be resolved.
	MissingSymbol(&'static str),
	/// Import never transfers ownership of a null native handle.
	NullHandle(&'static str),
}

impl std::fmt::Display for LoadError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMetadata(s) => write!(f, "invalid Vulkan import metadata: {s}"),
			Self::LibraryOpen(s) => write!(f, "Vulkan loader: {s}"),
			Self::MissingSymbol(s) => write!(f, "missing Vulkan command: {s}"),
			Self::NullHandle(s) => write!(f, "null Vulkan {s} handle"),
		}
	}
}

impl std::error::Error for LoadError {}

/// A Vulkan driver failure or a missing command (entry, instance or device level).
#[derive(Debug, PartialEq)]
pub enum CallError {
	/// The native enumeration did not stabilize within the bounded retry budget.
	EnumerationUnstable {
		attempts: u32,
	},
	/// Native creation succeeded but cleanup could not be resolved. Handle this
	/// variant explicitly to recover the still-live object and retained library.
	InitializationFailed(super::InitializationFailure),
	Vulkan(vk::Result),
	MissingCommand(&'static str),
	/// Invalid caller-supplied creation/import metadata; no native effect occurred.
	InvalidMetadata(&'static str),
	/// A required exact feature-record member is disabled or unrecorded.
	FeatureNotEnabled {
		structure: vk::StructureType,
		member: &'static str,
	},
	HostAllocationFailed,
	InvalidOutput(&'static str),
	CountOverflow(usize),
	/// An output/parallel slice does not match its native input count.
	SliceLengthMismatch {
		parameter: &'static str,
		expected: usize,
		actual: usize,
	},
}

impl std::fmt::Display for CallError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::EnumerationUnstable { attempts } => write!(
				f,
				"Vulkan enumeration did not stabilize after {attempts} attempts"
			),
			Self::InitializationFailed(failure) => failure.fmt(f),
			Self::Vulkan(r) => write!(f, "Vulkan result {}", r.as_raw()),
			Self::MissingCommand(n) => write!(f, "missing Vulkan command: {n}"),
			Self::InvalidMetadata(reason) => write!(f, "invalid Vulkan metadata: {reason}"),
			Self::FeatureNotEnabled { structure, member } => write!(
				f,
				"Vulkan feature {structure:?}::{member} is not recorded as enabled"
			),
			Self::HostAllocationFailed => f.write_str("Vulkan host allocation failed"),
			Self::InvalidOutput(reason) => write!(f, "invalid Vulkan output: {reason}"),
			Self::CountOverflow(count) => write!(f, "Vulkan count exceeds u32: {count}"),
			Self::SliceLengthMismatch {
				parameter,
				expected,
				actual,
			} => {
				write!(f, "Vulkan {parameter} length {actual}, expected {expected}")
			}
		}
	}
}

impl std::error::Error for CallError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::InitializationFailed(failure) => Some(failure),
			_ => None,
		}
	}
}

impl From<vk::Result> for CallError {
	/// Lift a non-success `VkResult` into a `CallError`.
	///
	/// Callers that only expect `VK_SUCCESS` can use `?` on a `vk::Result` once
	/// it is inside a `Result<_, CallError>` context.
	fn from(r: vk::Result) -> Self {
		Self::Vulkan(r)
	}
}

/// Global dispatch. Instances created here retain the native library independently.
#[derive(Clone)]
pub struct Entry {
	pub(crate) library: Option<Arc<Library>>,
	pub(crate) gipa: unsafe extern "system" fn(
		vk::InstanceHandle,
		*const std::ffi::c_char,
	) -> vk::PFN_vkVoidFunction,
	fp: vk::FnEntry,
}

impl Entry {
	/// Load the platform's default Vulkan library.
	///
	/// # Safety
	/// The library and its initialization routines must be safe to load, and it
	/// must implement the Vulkan ABI. Raw copied PFNs must not outlive all owners.
	pub unsafe fn new() -> Result<Self, LoadError> {
		#[cfg(windows)]
		let path = "vulkan-1.dll";
		#[cfg(any(target_os = "macos", target_os = "ios"))]
		let path = "libvulkan.dylib";
		#[cfg(any(target_os = "android", target_os = "fuchsia"))]
		let path = "libvulkan.so";
		#[cfg(not(any(
			windows,
			target_os = "macos",
			target_os = "ios",
			target_os = "android",
			target_os = "fuchsia"
		)))]
		let path = "libvulkan.so.1";
		// SAFETY: caller provides the native-loading guarantees above.
		unsafe { Self::new_from_path(path) }
	}

	/// Load a Vulkan implementation at the supplied native path.
	///
	/// # Safety
	/// The library's initialization/finalization must be safe, and its exports
	/// must implement Vulkan. Raw copied PFNs must not outlive all library owners.
	pub unsafe fn new_from_path(path: impl AsRef<OsStr>) -> Result<Self, LoadError> {
		// SAFETY: caller guarantees the library and its ABI are valid.
		let library =
			Arc::new(unsafe { Library::new(path) }.map_err(|e| LoadError::LibraryOpen(e.to_string()))?);
		// SAFETY: the Vulkan ABI specifies this exact export and signature.
		let gipa = unsafe {
			library.get::<unsafe extern "system" fn(
				vk::InstanceHandle,
				*const std::ffi::c_char,
			) -> vk::PFN_vkVoidFunction>(b"vkGetInstanceProcAddr\0")
		}
		.map_err(|_| LoadError::MissingSymbol("vkGetInstanceProcAddr"))?;
		// SAFETY: the resolved function belongs to the retained native library.
		let mut entry = unsafe { Self::new_from_resolver(*gipa) }?;
		entry.library = Some(library);
		Ok(entry)
	}

	/// Construct dispatch using an externally owned resolver.
	///
	/// # Safety
	/// The resolver must implement Vulkan and remain callable for this entry and
	/// all instances and raw PFNs derived from it, including any entry clones.
	pub unsafe fn new_from_resolver(
		gipa: unsafe extern "system" fn(
			vk::InstanceHandle,
			*const std::ffi::c_char,
		) -> vk::PFN_vkVoidFunction,
	) -> Result<Self, LoadError> {
		// SAFETY: caller guarantees a Vulkan-conformant resolver.
		let fp = unsafe { vk::FnEntry::new_from_resolver(gipa) };
		for (present, name) in [
			(fp.create_instance.is_some(), "vkCreateInstance"),
			(
				fp.enumerate_instance_extension_properties.is_some(),
				"vkEnumerateInstanceExtensionProperties",
			),
			(
				fp.enumerate_instance_layer_properties.is_some(),
				"vkEnumerateInstanceLayerProperties",
			),
		] {
			if !present {
				return Err(LoadError::MissingSymbol(name));
			}
		}
		Ok(Self {
			library: None,
			gipa,
			fp,
		})
	}

	/// Borrow the immutable global dispatch table. Copied functions remain unsafe.
	pub fn fp(&self) -> &vk::FnEntry {
		&self.fp
	}

	/// Resolve a command for a live instance, or a global command with NULL.
	///
	/// # Safety
	/// `instance` must be NULL or a live instance from this implementation.
	/// The caller must retain the library while using the returned function.
	#[doc(alias = "vkGetInstanceProcAddr")]
	#[doc = "\n\n[Khronos reference: `vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)"]
	pub unsafe fn get_instance_proc_addr(
		&self,
		instance: vk::InstanceHandle,
		name: &CStr,
	) -> vk::PFN_vkVoidFunction {
		// SAFETY: caller guarantees the instance belongs to this implementation.
		unsafe { (self.gipa)(instance, name.as_ptr()) }
	}

	/// Create a headless or extension-configured Vulkan instance.
	///
	/// # Safety
	/// The create info, its reachable pointers and allocator callbacks must meet
	/// Vulkan valid usage. Allocator state must remain alive until destruction.
	#[doc(alias = "vkCreateInstance")]
	#[doc = "\n\n[Khronos reference: `vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html)"]
	pub unsafe fn create_instance(
		&self,
		create_info: &vk::InstanceCreateInfo,
		allocator: Option<&vk::AllocationCallbacks>,
	) -> Result<Instance, CallError> {
		// SAFETY: the caller guarantees the canonical creation contract.
		unsafe { crate::create_instance(self, create_info, allocator) }
	}
}

use crate::command::commands;
commands! { Entry, entry;
	/// Enumerate instance extensions available without any layer.
	///
	/// Pass `layer_name = None` for the implicit layer (ICD + loader). Pass a
	/// layer name to query extensions exposed by that specific layer.
	///
	/// # Errors
	/// Returns `CallError::MissingCommand` if the driver predates Vulkan 1.0
	/// (practically impossible) or `CallError::Vulkan` on a driver failure.
	#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)"]
	pub fn enumerate_instance_extension_properties(
		layer_name: Option<&std::ffi::CStr>,
	) -> Result<Vec<vk::ExtensionProperties>, CallError> {
		let f = entry
			.fp
			.enumerate_instance_extension_properties
			.ok_or(CallError::MissingCommand(
				"vkEnumerateInstanceExtensionProperties",
			))?;
		let layer_ptr = layer_name.map_or(std::ptr::null(), |l| l.as_ptr());
		// SAFETY: validated Vulkan PFN; the optional layer name remains live.
		unsafe { super::enumerate::enumerate(|count, data| f(layer_ptr, count, data)) }
	}

	/// Enumerate all implicit and explicit Vulkan layers installed on this system.
	///
	/// # Errors
	/// Returns `CallError::MissingCommand` if the command is absent or
	/// `CallError::Vulkan` on a driver failure.
	#[doc(alias = "vkEnumerateInstanceLayerProperties")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)"]
	pub fn enumerate_instance_layer_properties() -> Result<Vec<vk::LayerProperties>, CallError> {
		let f = entry
			.fp
			.enumerate_instance_layer_properties
			.ok_or(CallError::MissingCommand(
				"vkEnumerateInstanceLayerProperties",
			))?;
		// SAFETY: validated Vulkan PFN implements the count/data protocol.
		unsafe { super::enumerate::enumerate(|count, data| f(count, data)) }
	}

	/// Query the loader API version. A Vulkan 1.0 loader has no version command.
	#[doc(alias = "vkEnumerateInstanceVersion")]
	#[doc = "\n\n[Khronos reference: `vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)"]
	pub fn enumerate_instance_version() -> Result<u32, vk::Result> {
		let Some(f) = entry.fp.enumerate_instance_version else {
			return Ok(make_api_version(0, 1, 0, 0));
		};
		let mut version = 0;
		// SAFETY: the table was checked on construction and the output is writable.
		let result = unsafe { f(&mut version) };
		if result == vk::Result::SUCCESS {
			Ok(version)
		} else {
			Err(result)
		}
	}
}
