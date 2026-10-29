//! Platform chain classification; Vulkan external-memory import contracts.
//! VMA donor: generic pMemoryAllocateNext forwarding. Explicit payload accounting,
//! dedication policy and no-query imports are Rust-specific safety adaptations.
use super::{AllocError, ChainData};
use crate::{self as vk, mem::ImportResource};
use std::ffi::CStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::mem) enum ImportKind {
	#[default]
	Ordinary,
	Host,
	Platform,
	MetalTexture,
	Collection,
}

impl ImportKind {
	pub(in crate::mem) fn requires_info(self) -> bool {
		self == Self::Platform || self == Self::MetalTexture || self == Self::Collection
	}
}

pub(super) fn validate(data: &ChainData) -> Result<(), AllocError> {
	if let Some(info) = data.import_info {
		if !data.imports || info.allocation_size == 0 || info.memory_type_bits == 0 {
			return Err(AllocError::InvalidRequest(
				"invalid external import accounting",
			));
		}
		if info.resource.is_some_and(|resource| match resource {
			ImportResource::Buffer(buffer) => buffer.is_null(),
			ImportResource::Image(image) => image.is_null(),
		}) {
			return Err(AllocError::InvalidRequest("null imported resource"));
		}
		if data.import_kind == ImportKind::Host && info.resource.is_some() {
			return Err(AllocError::InvalidRequest(
				"host imports forbid resource dedication",
			));
		}
		if data.import_kind == ImportKind::MetalTexture
			&& !matches!(info.resource, Some(ImportResource::Image(_)))
		{
			return Err(AllocError::InvalidRequest(
				"Metal texture imports require an image",
			));
		}
		if data.import_kind == ImportKind::Collection && info.resource.is_none() {
			return Err(AllocError::InvalidRequest(
				"collection imports require a resource",
			));
		}
	} else if data.import_kind.requires_info() {
		return Err(AllocError::InvalidRequest(
			"platform imports require external accounting",
		));
	}
	if data.imports
		&& data.flags.is_some_and(|flags| {
			flags.as_raw() & vk::MemoryAllocateFlagBits::ZERO_INITIALIZE_BIT_EXT.as_raw() != 0
		}) {
		return Err(AllocError::InvalidRequest(
			"imports cannot zero-initialize memory",
		));
	}
	Ok(())
}

pub(super) unsafe fn inspect(
	cursor: *const vk::BaseInStructure,
) -> Result<(&'static CStr, ImportKind), AllocError> {
	// SAFETY: caller supplies a readable, correctly tagged complete native record.
	let tag = unsafe { (*cursor).s_type };
	// SAFETY: each selected cast matches its native sType; only inline fields are
	// read. External OS objects, names and pointed-to payloads are never dereferenced.
	#[cfg(any(
		feature = "platform-win32",
		feature = "platform-android",
		feature = "platform-metal",
		feature = "platform-ohos",
		feature = "platform-screen",
		feature = "platform-fuchsia"
	))]
	unsafe {
		match tag {
			#[cfg(feature = "platform-win32")]
			vk::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
				let record = &*cursor.cast::<vk::ImportMemoryWin32HandleInfoKHR>();
				use vk::ExternalMemoryHandleTypeFlagBits as F;
				let kind = match record.handle_type {
					F::OPAQUE_WIN32_BIT | F::OPAQUE_WIN32_KMT_BIT | F::D3D12_HEAP_BIT => ImportKind::Ordinary,
					F::D3D11_TEXTURE_BIT | F::D3D11_TEXTURE_KMT_BIT | F::D3D12_RESOURCE_BIT => {
						ImportKind::Platform
					}
					_ => return Err(AllocError::InvalidRequest("invalid Win32 import type")),
				};
				if record.handle.is_null() && record.name.is_null() {
					return Err(AllocError::InvalidRequest(
						"missing imported Win32 handle/name",
					));
				}
				Ok((c"VK_KHR_external_memory_win32", kind))
			}
			#[cfg(feature = "platform-win32")]
			vk::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => {
				let record = &*cursor.cast::<vk::ImportMemoryWin32HandleInfoNV>();
				use vk::ExternalMemoryHandleTypeFlagBitsNV as F;
				if record.handle.is_null()
					|| !matches!(
						record.handle_type,
						F::OPAQUE_WIN32_BIT_NV
							| F::OPAQUE_WIN32_KMT_BIT_NV
							| F::D3D11_IMAGE_BIT_NV
							| F::D3D11_IMAGE_KMT_BIT_NV
					) {
					return Err(AllocError::InvalidRequest("invalid NV Win32 import"));
				}
				Ok((c"VK_NV_external_memory_win32", ImportKind::Platform))
			}
			#[cfg(feature = "platform-android")]
			vk::StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
				if (*cursor.cast::<vk::ImportAndroidHardwareBufferInfoANDROID>())
					.buffer
					.is_null()
				{
					return Err(AllocError::InvalidRequest("null Android hardware buffer"));
				}
				Ok((
					c"VK_ANDROID_external_memory_android_hardware_buffer",
					ImportKind::Platform,
				))
			}
			#[cfg(feature = "platform-metal")]
			vk::StructureType::IMPORT_METAL_BUFFER_INFO_EXT => {
				if (*cursor.cast::<vk::ImportMetalBufferInfoEXT>())
					.mtl_buffer
					.is_null()
				{
					return Err(AllocError::InvalidRequest("null Metal buffer"));
				}
				Ok((c"VK_EXT_metal_objects", ImportKind::Platform))
			}
			#[cfg(feature = "platform-metal")]
			vk::StructureType::IMPORT_MEMORY_METAL_HANDLE_INFO_EXT => {
				let record = &*cursor.cast::<vk::ImportMemoryMetalHandleInfoEXT>();
				use vk::ExternalMemoryHandleTypeFlagBits as F;
				let kind = match record.handle_type {
					F::MTLBUFFER_BIT_EXT | F::MTLHEAP_BIT_EXT => ImportKind::Platform,
					F::MTLTEXTURE_BIT_EXT => ImportKind::MetalTexture,
					_ => return Err(AllocError::InvalidRequest("invalid Metal import type")),
				};
				if record.handle.is_null() {
					return Err(AllocError::InvalidRequest("null Metal handle"));
				}
				Ok((c"VK_EXT_external_memory_metal", kind))
			}
			#[cfg(feature = "platform-ohos")]
			vk::StructureType::IMPORT_NATIVE_BUFFER_INFO_OHOS => {
				if (*cursor.cast::<vk::ImportNativeBufferInfoOHOS>())
					.buffer
					.is_null()
				{
					return Err(AllocError::InvalidRequest("null OHOS native buffer"));
				}
				Ok((c"VK_OHOS_external_memory", ImportKind::Platform))
			}
			#[cfg(feature = "platform-screen")]
			vk::StructureType::IMPORT_SCREEN_BUFFER_INFO_QNX => {
				if (*cursor.cast::<vk::ImportScreenBufferInfoQNX>())
					.buffer
					.is_null()
				{
					return Err(AllocError::InvalidRequest("null QNX Screen buffer"));
				}
				Ok((
					c"VK_QNX_external_memory_screen_buffer",
					ImportKind::Platform,
				))
			}
			#[cfg(feature = "platform-fuchsia")]
			vk::StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => {
				let record = &*cursor.cast::<vk::ImportMemoryZirconHandleInfoFUCHSIA>();
				if record.handle == 0
					|| record.handle_type != vk::ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_BIT_FUCHSIA
				{
					return Err(AllocError::InvalidRequest("invalid Zircon VMO import"));
				}
				Ok((c"VK_FUCHSIA_external_memory", ImportKind::Platform))
			}
			#[cfg(feature = "platform-fuchsia")]
			vk::StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => {
				if (*cursor.cast::<vk::ImportMemoryBufferCollectionFUCHSIA>())
					.collection
					.is_null()
				{
					return Err(AllocError::InvalidRequest("null Fuchsia buffer collection"));
				}
				Ok((c"VK_FUCHSIA_buffer_collection", ImportKind::Collection))
			}
			_ => Err(AllocError::UnsupportedCapability(
				"native allocation chain record",
			)),
		}
	}
	#[cfg(not(any(
		feature = "platform-win32",
		feature = "platform-android",
		feature = "platform-metal",
		feature = "platform-ohos",
		feature = "platform-screen",
		feature = "platform-fuchsia"
	)))]
	{
		let _ = tag;
		Err(AllocError::UnsupportedCapability(
			"native allocation chain record",
		))
	}
}
