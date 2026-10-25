//! Admission for allocation-neutral native creation chains. Pointers are borrowed
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
