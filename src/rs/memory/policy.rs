//! Resource-aware memory preferences, ported from pinned VMA 3.4.0
//! `FindMemoryPreferences` and `VmaBufferImageUsage::ContainsDeviceAccess`.
//! See LICENSE-VMA-MIT.
//! Adaptation: typed exclusive host intent, explicit flags by default, no implicit staging.
use super::{AllocError, AllocationCreateInfo};
use crate as vk;

/// Policy for managed resources. Raw allocation always uses explicit flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MemoryUsage {
	#[default]
	Explicit,
	Auto,
	PreferDevice,
	PreferHost,
}

/// CPU access intent used by automatic memory selection; never submits a transfer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HostAccess {
	#[default]
	None,
	SequentialWrite,
	Random,
}

pub(super) fn preferences(
	info: &AllocationCreateInfo,
	device_access: bool,
	integrated: Option<bool>,
) -> Result<
	(
		vk::MemoryPropertyFlags,
		vk::MemoryPropertyFlags,
		vk::MemoryPropertyFlags,
	),
	AllocError,
> {
	use vk::MemoryPropertyFlagBits as Flag;
	let mut required = info.required_flags;
	let mut preferred = info.preferred_flags;
	let mut avoided = info.avoided_flags;
	if info.allow_transfer_instead
		&& (info.usage == MemoryUsage::Explicit || info.host_access == HostAccess::None)
	{
		return Err(AllocError::InvalidRequest(
			"transfer fallback needs automatic host intent",
		));
	}
	if info.usage != MemoryUsage::Explicit {
		let prefer_host = info.usage == MemoryUsage::PreferHost;
		let prefer_device = info.usage == MemoryUsage::PreferDevice;
		let discrete_transfer = info.allow_transfer_instead
			&& !integrated.ok_or(AllocError::UnsupportedCapability(
				"transfer fallback requires queried physical-device type",
			))? && device_access
			&& !prefer_host;
		match info.host_access {
			HostAccess::Random => {
				preferred = preferred | Flag::HOST_CACHED_BIT;
				if discrete_transfer {
					preferred = preferred | Flag::DEVICE_LOCAL_BIT;
				} else if info.allow_transfer_instead {
					preferred = preferred | Flag::HOST_VISIBLE_BIT;
				} else {
					required = required | Flag::HOST_VISIBLE_BIT;
				}
			}
			HostAccess::SequentialWrite => {
				avoided = avoided | Flag::HOST_CACHED_BIT;
				if discrete_transfer {
					preferred = preferred | Flag::DEVICE_LOCAL_BIT | Flag::HOST_VISIBLE_BIT;
				} else {
					if info.allow_transfer_instead {
						preferred = preferred | Flag::HOST_VISIBLE_BIT;
					} else {
						required = required | Flag::HOST_VISIBLE_BIT;
					}
					if (device_access && !prefer_host) || (!device_access && prefer_device) {
						preferred = preferred | Flag::DEVICE_LOCAL_BIT;
					} else {
						avoided = avoided | Flag::DEVICE_LOCAL_BIT;
					}
				}
			}
			HostAccess::None => {
				if prefer_host {
					avoided = avoided | Flag::DEVICE_LOCAL_BIT;
				} else {
					preferred = preferred | Flag::DEVICE_LOCAL_BIT;
				}
			}
		}
	} else if info.host_access != HostAccess::None {
		return Err(AllocError::InvalidRequest(
			"host intent requires automatic memory usage",
		));
	}
	if (required | preferred).as_raw()
		& (Flag::DEVICE_COHERENT_BIT_AMD | Flag::DEVICE_UNCACHED_BIT_AMD).as_raw()
		== 0
	{
		avoided = avoided | Flag::DEVICE_UNCACHED_BIT_AMD;
	}
	if info.persistent_map {
		required = required | Flag::HOST_VISIBLE_BIT;
	}
	Ok((required, preferred, avoided))
}

#[cfg(test)]
#[path = "../../../test/rs/memory/test_policy.rs"]
mod tests;
