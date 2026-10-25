//! Immutable creation provenance and registry command admission.
use super::{CallError, Features};
use crate::core::{self as vk, CapabilityRequirement, CommandMetadata};
use std::{
	ffi::{CStr, c_char},
	sync::Arc,
};

/// Version/extension admission for one exact command spelling.
/// This does not validate enabled feature bits, parameters or Vulkan valid usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSupport {
	Enabled,
	Disabled,
	/// An instance-level query depends on support on the selected physical device.
	RequiresPhysicalDevice,
}

/// Owned snapshot of the API version and extension names used at creation.
/// Device snapshots inherit instance extensions and cap the API version by the
/// selected physical device. Feature records are owned; no input pNext pointers are retained.
#[derive(Debug)]
pub struct Capabilities {
	api_version: u32,
	instance_api_version: u32,
	instance_extensions: Arc<Vec<Vec<u8>>>,
	device_extensions: Option<Vec<Vec<u8>>>,
	features: Option<Features>,
}

impl Capabilities {
	/// Application API ceiling, capped by physical-device API for device snapshots.
	/// The ignored patch component is removed.
	pub fn api_version(&self) -> u32 {
		self.api_version
	}

	/// Application API ceiling capped by the queried instance implementation.
	/// Physical-device commands use their own API support, independently.
	pub fn instance_api_version(&self) -> u32 {
		self.instance_api_version
	}

	pub(crate) fn limit_instance_version(&mut self, version: u32) -> Result<(), CallError> {
		self.instance_api_version = self.api_version.min(normalize_version(version)?);
		Ok(())
	}

	/// Exact enabled instance extension names, independently owned from create info.
	pub fn instance_extensions(&self) -> impl ExactSizeIterator<Item = &CStr> {
		self.instance_extensions.iter().map(|bytes| {
			// SAFETY: capture copies one complete valid CStr including its terminator.
			unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
		})
	}

	/// Enabled device extensions; absent on an instance-only snapshot.
	pub fn device_extensions(&self) -> Option<impl ExactSizeIterator<Item = &CStr>> {
		self.device_extensions.as_ref().map(|extensions| {
			extensions.iter().map(|bytes| {
				// SAFETY: capture copies one complete valid CStr including its terminator.
				unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
			})
		})
	}

	/// Exact enabled feature records; absent on an instance-only snapshot.
	pub fn features(&self) -> Option<&Features> {
		self.features.as_ref()
	}

	/// Capture external instance creation metadata without adopting a native handle.
	///
	/// # Errors
	/// Reports invalid versions/name arrays or host allocation failure.
	///
	/// # Safety
	/// Every reachable create-info pointer must be valid for its declared length.
	/// When importing, this must describe the actual instance's creation and
	/// `implementation_version` must be the version of its instance implementation.
	pub unsafe fn new_from_instance_create_info(
		info: &vk::InstanceCreateInfo,
		implementation_version: u32,
	) -> Result<Self, CallError> {
		// SAFETY: caller provides the original valid metadata graph.
		let mut capabilities = unsafe { Self::instance(info) }?;
		capabilities.limit_instance_version(implementation_version)?;
		Ok(capabilities)
	}

	/// Capture external device creation metadata without adopting a native handle.
	///
	/// # Errors
	/// Reports a device snapshot used as parent, invalid metadata/chains or host
	/// allocation failure. No native allocation or feature query is performed.
	///
	/// # Safety
	/// This snapshot must describe the actual parent instance. The create info
	/// must describe the actual logical device, and physical_version its selected
	/// physical device. All reachable records must be readable, aligned and valid
	/// for their sType; pointers are borrowed only until this constructor returns.
	pub unsafe fn new_from_device_create_info(
		&self,
		info: &vk::DeviceCreateInfo,
		physical_version: u32,
	) -> Result<Self, CallError> {
		if self.device_extensions.is_some() {
			return Err(CallError::InvalidMetadata(
				"device capabilities cannot be an instance parent",
			));
		}
		// SAFETY: caller provides the original valid metadata graph.
		unsafe { self.device(info, physical_version) }
	}

	pub(crate) fn matches_instance(&self, parent: &Self) -> bool {
		!parent.is_device()
			&& self.api_version <= parent.api_version
			&& self.instance_api_version == parent.instance_api_version
			&& self
				.instance_extensions()
				.all(|name| parent.instance_extensions().any(|other| name == other))
			&& parent
				.instance_extensions()
				.all(|name| self.instance_extensions().any(|other| name == other))
	}

	pub(crate) fn is_device(&self) -> bool {
		self.device_extensions.is_some()
	}

	/// Evaluate registry version/extension requirements for an exact native name.
	/// Unknown names return None. Alias spellings have their own requirements;
	/// promotion never silently enables an extension spelling or replaces a PFN.
	/// Enabled does not guarantee a loaded pointer or feature-bit validity.
	pub fn command_support(&self, name: &CStr) -> Option<CommandSupport> {
		vk::COMMANDS
			.iter()
			.find(|command| command.name.as_bytes() == name.to_bytes())
			.map(|command| self.support(command))
	}

	pub(crate) fn support(&self, command: &CommandMetadata) -> CommandSupport {
		use CommandSupport::*;
		let api_version = if command.dispatch == "Instance" && !command.physical_device {
			self.instance_api_version
		} else {
			self.api_version
		};
		let mut result = Disabled;
		for clause in command.requirements {
			let mut support = Enabled;
			for atom in *clause {
				let value = match atom {
					CapabilityRequirement::Core(version) => {
						if api_version >= *version {
							if command.physical_device
								&& self.device_extensions.is_none()
								&& *version > vk::API_VERSION_1_0
							{
								RequiresPhysicalDevice
							} else {
								Enabled
							}
						} else {
							Disabled
						}
					}
					CapabilityRequirement::InstanceExtension(name) => {
						if self.instance_extensions().any(|enabled| enabled == *name) {
							Enabled
						} else {
							Disabled
						}
					}
					CapabilityRequirement::DeviceExtension(name) => match self.device_extensions() {
						Some(mut extensions) => {
							if extensions.any(|enabled| enabled == *name) {
								Enabled
							} else {
								Disabled
							}
						}
						None => RequiresPhysicalDevice,
					},
				};
				if value == Disabled {
					support = Disabled;
					break;
				}
				if value == RequiresPhysicalDevice {
					support = RequiresPhysicalDevice;
				}
			}
			if support == Enabled {
				return Enabled;
			}
			if support == RequiresPhysicalDevice {
				result = RequiresPhysicalDevice;
			}
		}
		result
	}

	/// # Safety
	/// All reachable create-info pointers must be valid for their declared lengths.
	pub(crate) unsafe fn instance(info: &vk::InstanceCreateInfo) -> Result<Self, CallError> {
		// SAFETY: caller guarantees the optional application-info pointer is valid.
		let requested = unsafe { info.p_application_info.as_ref() }.map_or(0, |app| app.api_version);
		let api_version = normalize_version(if requested == 0 {
			vk::API_VERSION_1_0
		} else {
			requested
		})?;
		// SAFETY: caller supplies the enabled-name array and valid terminated strings.
		let extensions = unsafe {
			capture(
				info.enabled_extension_count,
				info.pp_enabled_extension_names,
			)
		}?;
		Ok(Self {
			api_version,
			instance_api_version: api_version,
			instance_extensions: Arc::new(extensions),
			device_extensions: None,
			features: None,
		})
	}

	/// # Safety
	/// All reachable create-info pointers must be valid for their declared lengths.
	pub(crate) unsafe fn device(
		&self,
		info: &vk::DeviceCreateInfo,
		physical_version: u32,
	) -> Result<Self, CallError> {
		let api_version = self.api_version.min(normalize_version(physical_version)?);
		// SAFETY: caller supplies the enabled-name array and valid terminated strings.
		let extensions = unsafe {
			capture(
				info.enabled_extension_count,
				info.pp_enabled_extension_names,
			)
		}?;
		Ok(Self {
			api_version,
			instance_api_version: self.instance_api_version,
			instance_extensions: Arc::clone(&self.instance_extensions),
			device_extensions: Some(extensions),
			// SAFETY: caller guarantees the complete readable feature chain.
			features: Some(unsafe { Features::capture(info) }?),
		})
	}
}

fn normalize_version(version: u32) -> Result<u32, CallError> {
	if vk::api_version_variant(version) != 0 || version < vk::API_VERSION_1_0 {
		return Err(CallError::InvalidOutput("unsupported Vulkan API version"));
	}
	Ok(version & !0xfff)
}

unsafe fn capture(count: u32, names: *const *const c_char) -> Result<Vec<Vec<u8>>, CallError> {
	if count != 0 && names.is_null() {
		return Err(CallError::InvalidOutput("null enabled-extension array"));
	}
	let mut result = Vec::new();
	result
		.try_reserve_exact(count as usize)
		.map_err(|_| CallError::HostAllocationFailed)?;
	for index in 0..count as usize {
		// SAFETY: caller guarantees a readable array of count pointers.
		let name = unsafe { *names.add(index) };
		if name.is_null() {
			return Err(CallError::InvalidOutput("null enabled-extension name"));
		}
		// SAFETY: caller guarantees each non-null entry is a valid terminated CStr.
		let bytes = unsafe { CStr::from_ptr(name) }.to_bytes_with_nul();
		let mut owned = Vec::new();
		owned
			.try_reserve_exact(bytes.len())
			.map_err(|_| CallError::HostAllocationFailed)?;
		owned.extend_from_slice(bytes);
		result.push(owned);
	}
	Ok(result)
}

#[cfg(test)]
#[path = "../../../test/rs/load/test_capability.rs"]
mod tests;
