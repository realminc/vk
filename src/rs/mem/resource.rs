//! Enabled allocation features and allocation-neutral native creation chains. Pointers are borrowed
//! only for the create call; no allocator object retains caller chain storage.
use super::AllocError;
use crate as vk;
use std::{
	ffi::{CStr, c_void},
	ptr,
};

pub(super) fn extension(device: &vk::Device, name: &CStr) -> bool {
	device
		.capabilities()
		.and_then(|c| c.device_extensions())
		.is_some_and(|mut names| names.any(|candidate| candidate == name))
}

// Donor: VMA's allocator-wide buffer-device-address admission. Read the exact
// core/unified or standalone feature representation; never infer enablement
// from driver support, a resolved PFN, or the incompatible EXT representation.
pub(super) fn buffer_device_address(device: &vk::Device) -> bool {
	let Some(capabilities) = device.capabilities() else {
		return false;
	};
	let Some(features) = capabilities.features() else {
		return false;
	};
	let core = capabilities.api_version() >= vk::API_VERSION_1_2;
	let khr = capabilities.command_support(c"vkGetBufferDeviceAddressKHR")
		== Some(vk::load::CommandSupport::Enabled);
	(core || khr)
		&& (features.enabled(
			vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
			"bufferDeviceAddress",
		) == Some(true)
			|| (core
				&& features.enabled(
					vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
					"bufferDeviceAddress",
				) == Some(true)))
}

/// The unsafe caller still establishes all pointed-to extents, profile feature
/// requirements and native valid usage; this is not a replacement validation layer.
pub(super) unsafe fn validate_chain(
	device: &vk::Device,
	chain: *const c_void,
	image: bool,
) -> Result<(), AllocError> {
	let mut seen = [ptr::null::<vk::BaseInStructure>(); 64];
	let mut kinds = [vk::StructureType::from_raw(0); 64];
	let mut count = 0;
	let mut next = chain.cast::<vk::BaseInStructure>();
	while !next.is_null() {
		if count == seen.len() || seen[..count].contains(&next) {
			return Err(AllocError::InvalidRequest(
				"cyclic or excessive resource chain",
			));
		}
		// SAFETY: caller guarantees every reachable native header is readable.
		let node = unsafe { &*next };
		if kinds[..count].contains(&node.s_type) {
			return Err(AllocError::InvalidRequest(
				"duplicate resource chain record",
			));
		}
		seen[count] = next;
		kinds[count] = node.s_type;
		count += 1;
		match node.s_type {
			vk::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO if !image => {
				// SAFETY: caller supplies the complete correctly tagged record.
				let types = unsafe { (*next.cast::<vk::ExternalMemoryBufferCreateInfo>()).handle_types };
				validate_external_types(device, types)?;
			}
			vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO if image => {
				// SAFETY: caller supplies the complete correctly tagged record.
				let types = unsafe { (*next.cast::<vk::ExternalMemoryImageCreateInfo>()).handle_types };
				validate_external_types(device, types)?;
			}

			vk::StructureType::VIDEO_PROFILE_LIST_INFO_KHR => {
				if !extension(device, c"VK_KHR_video_queue") {
					return Err(AllocError::UnsupportedCapability(
						"video_queue is not enabled",
					));
				}
				// SAFETY: caller supplies a complete record matching the header tag.
				let list = unsafe { &*next.cast::<vk::VideoProfileListInfoKHR>() };
				if list.profile_count == 0 || list.p_profiles.is_null() {
					return Err(AllocError::InvalidRequest(
						"empty video resource profile list",
					));
				}
			}
			vk::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO if image => {
				if !device
					.capabilities()
					.is_some_and(|c| c.api_version() >= vk::API_VERSION_1_2)
					&& !extension(device, c"VK_KHR_image_format_list")
				{
					return Err(AllocError::UnsupportedCapability(
						"image format list is not enabled",
					));
				}
				// SAFETY: caller supplies a complete record matching the header tag.
				let list = unsafe { &*next.cast::<vk::ImageFormatListCreateInfo>() };
				if list.view_format_count != 0 && list.p_view_formats.is_null() {
					return Err(AllocError::InvalidRequest("null image view format array"));
				}
			}
			_ => {
				return Err(AllocError::UnsupportedCapability(
					"resource allocation chain",
				));
			}
		}
		next = node.p_next;
	}
	Ok(())
}

pub(super) fn memory_priority(device: &vk::Device) -> bool {
	extension(device, c"VK_EXT_memory_priority")
		&& device
			.capabilities()
			.and_then(|caps| caps.features())
			.is_some_and(|features| {
				features.enabled(
					vk::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
					"memoryPriority",
				) == Some(true)
			})
}

pub(super) fn device_coherent_memory(device: &vk::Device) -> bool {
	extension(device, c"VK_AMD_device_coherent_memory")
		&& device
			.capabilities()
			.and_then(|caps| caps.features())
			.is_some_and(|features| {
				features.enabled(
					vk::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
					"deviceCoherentMemory",
				) == Some(true)
			})
}

pub(super) fn protected_memory(device: &vk::Device) -> bool {
	device.capabilities().is_some_and(|caps| {
		caps.api_version() >= vk::API_VERSION_1_1
			&& caps.features().is_some_and(|features| {
				features.enabled(
					vk::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
					"protectedMemory",
				) == Some(true)
					|| features.enabled(
						vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
						"protectedMemory",
					) == Some(true)
			})
	})
}

pub(super) fn validate_external_types(
	device: &vk::Device,
	types: vk::ExternalMemoryHandleTypeFlags,
) -> Result<(), AllocError> {
	use vk::ExternalMemoryHandleTypeFlagBits as F;
	if types.as_raw() == 0 {
		return Ok(());
	}
	if !device
		.capabilities()
		.is_some_and(|caps| caps.api_version() >= vk::API_VERSION_1_1)
		&& !extension(device, c"VK_KHR_external_memory")
	{
		return Err(AllocError::UnsupportedCapability(
			"external memory is not enabled",
		));
	}
	let fd = F::OPAQUE_FD_BIT | F::DMA_BUF_BIT_EXT;
	let win = F::OPAQUE_WIN32_BIT
		| F::OPAQUE_WIN32_KMT_BIT
		| F::D3D11_TEXTURE_BIT
		| F::D3D11_TEXTURE_KMT_BIT
		| F::D3D12_HEAP_BIT
		| F::D3D12_RESOURCE_BIT;
	if types.as_raw() & !(fd | win).as_raw() != 0 {
		return Err(AllocError::UnsupportedCapability(
			"external handle type accounting",
		));
	}
	if types.as_raw() & fd.as_raw() != 0 && !extension(device, c"VK_KHR_external_memory_fd") {
		return Err(AllocError::UnsupportedCapability(
			"external_memory_fd is not enabled",
		));
	}
	if types.as_raw() & F::DMA_BUF_BIT_EXT.as_raw() != 0
		&& !extension(device, c"VK_EXT_external_memory_dma_buf")
	{
		return Err(AllocError::UnsupportedCapability(
			"external_memory_dma_buf is not enabled",
		));
	}
	if types.as_raw() & win.as_raw() != 0 {
		#[cfg(feature = "platform-win32")]
		if !extension(device, c"VK_KHR_external_memory_win32") {
			return Err(AllocError::UnsupportedCapability(
				"external_memory_win32 is not enabled",
			));
		}
		#[cfg(not(feature = "platform-win32"))]
		return Err(AllocError::UnsupportedCapability(
			"platform-win32 is disabled",
		));
	}
	Ok(())
}
