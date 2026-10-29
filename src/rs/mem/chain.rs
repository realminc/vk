//! Borrowed native allocation chains. Donor: pinned VMA 3.4.0
//! `vmaAllocateDedicatedMemory` and `VmaPoolCreateInfo::pMemoryAllocateNext`.
//! Adaptation: bounded validation, borrowed storage and single-attempt imports.
mod import;
pub(super) use import::ImportKind;

use super::{AllocError, ImportMemoryInfo};
use crate as vk;
use std::{ffi::c_void, marker::PhantomData, ptr};

/// A caller-owned `VkMemoryAllocateInfo::pNext` chain.
///
/// The allocator prepends its dedication and admitted priority/address records.
/// Explicit flags are preserved, including a device mask; resource dedication
/// and priority belong to the allocator and must not also appear in this chain.
/// No native handle is owned by this borrowed descriptor.
#[derive(Clone, Copy, Debug)]
pub struct AllocationChain<'a> {
	pub(super) data: ChainData,
	borrow: PhantomData<&'a c_void>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ChainData {
	pub next: *const c_void,
	pub flags: Option<vk::MemoryAllocateFlags>,
	pub exports: vk::ExternalMemoryHandleTypeFlags,
	pub has_export: bool,
	pub imports: bool,
	pub import_extension: Option<&'static std::ffi::CStr>,
	pub import_kind: ImportKind,
	pub import_info: Option<ImportMemoryInfo>,
}

impl<'a> AllocationChain<'a> {
	/// Borrow a registered allocation extension and its linked records.
	/// # Errors
	/// Rejects cycles, duplicate tags, overlong chains, allocator-owned records and
	/// native records without an admitted allocator ownership/accounting contract.
	/// # Safety
	/// Every reachable header and full payload must be valid, correctly tagged,
	/// immutable and readable for `'a`; all nested pointers/handles and enabled
	/// capabilities must meet native valid usage at allocation. Imports must use
	/// an explicitly compatible memory type/size. Native import handle transfer
	/// occurs on successful `vkAllocateMemory`, even if later mapping/binding fails.
	pub unsafe fn new_from_record<T: vk::Extends<vk::MemoryAllocateInfo<'a>>>(
		record: &'a T,
	) -> Result<Self, AllocError> {
		// SAFETY: the sealed registry relationship supplies a repr(C) native header;
		// caller establishes complete chain storage and payload validity for 'a.
		let data = unsafe { inspect(ptr::from_ref(record).cast(), None) }?;
		Ok(Self {
			data,
			borrow: PhantomData,
		})
	}

	/// Borrow an import chain with explicit external payload accounting.
	///
	/// Use this for platform imports whose native allocation size is ignored or
	/// whose compatible types come from external properties. No OS query is guessed.
	/// # Errors
	/// Same chain validation as `new_from_record`, plus absent/invalid import
	/// metadata, null resource identities and incompatible dedication policy.
	/// # Safety
	/// The complete `new_from_record` contract applies. `info` must describe this
	/// exact payload and device: obtain size/types from the platform Vulkan query
	/// or authoritative OS/export metadata. Include all backing bytes, even when
	/// native allocationSize is ignored/zero. A supplied resource must be live,
	/// unbound, compatible, and have requirements covered by this size/type mask.
	/// Every native dedication rule still applies, including an exact requirements
	/// size wherever required. Fuchsia collection resource/index/size must match.
	/// Android/QNX/OHOS image profiles and Metal textures require their image.
	/// External host/OS objects must survive every native use, not just this borrow.
	pub unsafe fn new_from_record_with_import<T: vk::Extends<vk::MemoryAllocateInfo<'a>>>(
		record: &'a T,
		info: ImportMemoryInfo,
	) -> Result<Self, AllocError> {
		// SAFETY: caller supplies the complete immutable chain and truthful payload
		// metadata; inspection validates shape without dereferencing external objects.
		let data = unsafe { inspect(ptr::from_ref(record).cast(), Some(info)) }?;
		Ok(Self {
			data,
			borrow: PhantomData,
		})
	}

	pub fn as_ptr(&self) -> *const c_void {
		self.data.next
	}
}

unsafe fn inspect(
	next: *const c_void,
	import_info: Option<ImportMemoryInfo>,
) -> Result<ChainData, AllocError> {
	let mut data = ChainData {
		next,
		import_info,
		..Default::default()
	};
	let mut seen = [ptr::null::<vk::BaseInStructure>(); 64];
	let mut tags = [vk::StructureType::from_raw(0); 64];
	let mut count = 0;
	let mut cursor = next.cast::<vk::BaseInStructure>();
	while !cursor.is_null() {
		if count == seen.len() || seen[..count].contains(&cursor) {
			return Err(AllocError::InvalidRequest(
				"cyclic or excessive allocation chain",
			));
		}
		// SAFETY: construction contract guarantees readable complete native records.
		let node = unsafe { &*cursor };
		if tags[..count].contains(&node.s_type) {
			return Err(AllocError::InvalidRequest(
				"duplicate allocation chain record",
			));
		}
		seen[count] = cursor;
		tags[count] = node.s_type;
		count += 1;
		match node.s_type {
			vk::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO
			| vk::StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV
			| vk::StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT
			| vk::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM => {
				return Err(AllocError::InvalidRequest(
					"dedication and priority are allocator-owned",
				));
			}
			vk::StructureType::MEMORY_ALLOCATE_FLAGS_INFO => {
				// SAFETY: caller supplies the complete record matching this tag.
				let flags = unsafe { &*cursor.cast::<vk::MemoryAllocateFlagsInfo>() };
				if flags.flags.as_raw() & vk::MemoryAllocateFlagBits::DEVICE_MASK_BIT.as_raw() != 0
					&& flags.device_mask == 0
				{
					return Err(AllocError::InvalidRequest(
						"device-mask allocation requires a nonzero mask",
					));
				}
				if flags.flags.as_raw()
					& vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT.as_raw()
					!= 0
				{
					return Err(AllocError::UnsupportedCapability(
						"allocation capture/replay",
					));
				}
				data.flags = Some(flags.flags);
			}
			vk::StructureType::EXPORT_MEMORY_ALLOCATE_INFO => {
				// SAFETY: caller supplies the complete record matching this tag.
				data.exports = unsafe { (*cursor.cast::<vk::ExportMemoryAllocateInfo>()).handle_types };
				data.has_export = true;
			}
			vk::StructureType::IMPORT_MEMORY_FD_INFO_KHR => {
				if data.imports {
					return Err(AllocError::InvalidRequest(
						"multiple memory import operations",
					));
				}
				// SAFETY: complete native record is guaranteed by construction.
				let import = unsafe { &*cursor.cast::<vk::ImportMemoryFdInfoKHR>() };
				if import.fd < 0
					|| (import.handle_type != vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT
						&& import.handle_type != vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_BIT_EXT)
				{
					return Err(AllocError::InvalidRequest("invalid memory fd import"));
				}
				data.imports = true;
				data.import_extension = Some(c"VK_KHR_external_memory_fd");
			}
			vk::StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => {
				if data.imports {
					return Err(AllocError::InvalidRequest(
						"multiple memory import operations",
					));
				}
				// SAFETY: complete native record is guaranteed by construction.
				let import = unsafe { &*cursor.cast::<vk::ImportMemoryHostPointerInfoEXT>() };
				if import.p_host_pointer.is_null()
					|| (import.handle_type != vk::ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_BIT_EXT
						&& import.handle_type
							!= vk::ExternalMemoryHandleTypeFlagBits::HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT)
				{
					return Err(AllocError::InvalidRequest("invalid host pointer import"));
				}
				data.imports = true;
				data.import_kind = ImportKind::Host;
				data.import_extension = Some(c"VK_EXT_external_memory_host");
			}
			vk::StructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => {
				return Err(AllocError::UnsupportedCapability(
					"allocation capture/replay",
				));
			}
			#[cfg(feature = "platform-win32")]
			vk::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {}
			_ => {
				// SAFETY: same complete tagged-record guarantee as the core cases.
				let (extension, kind) = unsafe { import::inspect(cursor) }?;
				if data.imports {
					return Err(AllocError::InvalidRequest(
						"multiple memory import operations",
					));
				}
				data.imports = true;
				data.import_extension = Some(extension);
				data.import_kind = kind;
			}
		}
		cursor = node.p_next;
	}
	import::validate(&data)?;
	Ok(data)
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_chain.rs"]
mod tests;
