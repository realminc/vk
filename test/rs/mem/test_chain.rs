//! Borrowed chain contracts independent of any Vulkan driver.
use super::*;

#[test]
fn registered_chain_retains_exact_native_storage_without_modifying_links() {
	let export = vk::ExportMemoryAllocateInfo {
		handle_types: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		..Default::default()
	};
	// SAFETY: complete immutable native record, no linked storage or external handles.
	let chain = unsafe { AllocationChain::new_from_record(&export) }.unwrap();
	assert_eq!(chain.as_ptr(), ptr::from_ref(&export).cast());
	assert_eq!(chain.data.exports, export.handle_types);
	assert!(!chain.data.imports);
	assert!(export.p_next.is_null());
}

#[test]
fn duplicate_tags_and_cycles_are_rejected_before_native_allocation() {
	let tail = vk::ExportMemoryAllocateInfo::default();
	let head = vk::ExportMemoryAllocateInfo {
		p_next: ptr::from_ref(&tail).cast(),
		..Default::default()
	};
	// SAFETY: both complete records are readable for this synchronous inspection.
	assert!(matches!(
		unsafe { AllocationChain::new_from_record(&head) },
		Err(AllocError::InvalidRequest(_))
	));
	let mut cycle = vk::ExportMemoryAllocateInfo::default();
	cycle.p_next = ptr::from_ref(&cycle).cast();
	// SAFETY: self-linked complete record remains readable; inspection must reject the cycle.
	assert!(matches!(
		unsafe { AllocationChain::new_from_record(&cycle) },
		Err(AllocError::InvalidRequest(_))
	));
}

#[test]
fn caller_cannot_duplicate_allocator_owned_priority_or_dedication() {
	let priority = vk::MemoryPriorityAllocateInfoEXT::default();
	let dedicated = vk::MemoryDedicatedAllocateInfo::default();
	// SAFETY: complete readable native records without external effects.
	unsafe {
		assert!(matches!(
			AllocationChain::new_from_record(&priority),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(matches!(
			AllocationChain::new_from_record(&dedicated),
			Err(AllocError::InvalidRequest(_))
		));
	}
}

#[test]
fn valid_import_is_identified_and_invalid_fd_never_reaches_native_code() {
	let import = vk::ImportMemoryFdInfoKHR {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		fd: 7,
		..Default::default()
	};
	// SAFETY: inspection only reads the complete record; no allocation or OS access occurs.
	let chain = unsafe { AllocationChain::new_from_record(&import) }.unwrap();
	assert!(chain.data.imports);
	assert_eq!(
		chain.data.import_extension,
		Some(c"VK_KHR_external_memory_fd")
	);
	let bad = vk::ImportMemoryFdInfoKHR {
		fd: -1,
		..Default::default()
	};
	// SAFETY: complete readable invalid payload is inspected and rejected, never imported.
	assert!(matches!(
		unsafe { AllocationChain::new_from_record(&bad) },
		Err(AllocError::InvalidRequest(_))
	));
}

#[test]
fn explicit_device_mask_flags_are_retained_and_capture_replay_is_rejected() {
	let flags = vk::MemoryAllocateFlagsInfo {
		flags: vk::MemoryAllocateFlagBits::DEVICE_MASK_BIT,
		device_mask: 3,
		..Default::default()
	};
	// SAFETY: complete readable native record; no allocation or device operation occurs.
	let chain = unsafe { AllocationChain::new_from_record(&flags) }.unwrap();
	assert_eq!(
		chain.data.flags,
		Some(vk::MemoryAllocateFlagBits::DEVICE_MASK_BIT)
	);
	assert_eq!(flags.device_mask, 3);
	let replay = vk::MemoryAllocateFlagsInfo {
		flags: vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
		..Default::default()
	};
	// SAFETY: complete readable unsupported payload is rejected before native effects.
	assert!(matches!(
		unsafe { AllocationChain::new_from_record(&replay) },
		Err(AllocError::UnsupportedCapability(_))
	));
}

#[test]
fn zero_export_record_is_still_retained_for_duplicate_header_detection() {
	let export = vk::ExportMemoryAllocateInfo::default();
	// SAFETY: complete native record, no links/handles or native effects.
	let chain = unsafe { AllocationChain::new_from_record(&export) }.unwrap();
	assert!(chain.data.has_export);
	assert_eq!(chain.data.exports.as_raw(), 0);
}

#[test]
fn import_accounting_requires_one_import_and_nonempty_exact_metadata() {
	use crate::mem::{ImportMemoryInfo, ImportResource};
	let export = vk::ExportMemoryAllocateInfo::default();
	let fd = vk::ImportMemoryFdInfoKHR {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		fd: 7,
		..Default::default()
	};
	let info = ImportMemoryInfo {
		allocation_size: 4096,
		memory_type_bits: 1,
		resource: None,
	};
	// SAFETY: complete records inspected without native or external-object access.
	unsafe {
		assert!(matches!(
			AllocationChain::new_from_record_with_import(&export, info),
			Err(AllocError::InvalidRequest(_))
		));
		for invalid in [
			ImportMemoryInfo {
				allocation_size: 0,
				..info
			},
			ImportMemoryInfo {
				memory_type_bits: 0,
				..info
			},
			ImportMemoryInfo {
				resource: Some(ImportResource::Buffer(vk::Buffer::NULL)),
				..info
			},
			ImportMemoryInfo {
				resource: Some(ImportResource::Image(vk::Image::NULL)),
				..info
			},
		] {
			assert!(matches!(
				AllocationChain::new_from_record_with_import(&fd, invalid),
				Err(AllocError::InvalidRequest(_))
			));
		}
		let host = vk::ImportMemoryHostPointerInfoEXT {
			handle_type: vk::ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_BIT_EXT,
			p_host_pointer: std::ptr::dangling_mut(),
			..Default::default()
		};
		assert!(matches!(
			AllocationChain::new_from_record_with_import(
				&host,
				ImportMemoryInfo {
					resource: Some(ImportResource::Buffer(vk::Buffer::from_raw(11))),
					..info
				}
			),
			Err(AllocError::InvalidRequest(_))
		));
		let flags = vk::MemoryAllocateFlagsInfo {
			flags: vk::MemoryAllocateFlagBits::ZERO_INITIALIZE_BIT_EXT,
			p_next: std::ptr::from_ref(&fd).cast(),
			..Default::default()
		};
		assert!(matches!(
			AllocationChain::new_from_record(&flags),
			Err(AllocError::InvalidRequest(_))
		));
		let two_imports = vk::ImportMemoryHostPointerInfoEXT {
			p_next: std::ptr::from_ref(&fd).cast(),
			..host
		};
		assert!(matches!(
			AllocationChain::new_from_record(&two_imports),
			Err(AllocError::InvalidRequest(_))
		));
	}
}

#[cfg(feature = "platform-metal")]
#[test]
fn metal_texture_and_platform_records_validate_resource_and_external_identity() {
	use crate::mem::{ImportMemoryInfo, ImportResource};
	let texture = vk::ImportMemoryMetalHandleInfoEXT {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_BIT_EXT,
		handle: std::ptr::dangling_mut(),
		..Default::default()
	};
	let info = ImportMemoryInfo {
		allocation_size: 4096,
		memory_type_bits: 1,
		resource: None,
	};
	// SAFETY: complete records; no synthetic external object is dereferenced.
	unsafe {
		for resource in [None, Some(ImportResource::Buffer(vk::Buffer::from_raw(1)))] {
			assert!(matches!(
				AllocationChain::new_from_record_with_import(
					&texture,
					ImportMemoryInfo { resource, ..info }
				),
				Err(AllocError::InvalidRequest(_))
			));
		}
		let resource = Some(ImportResource::Image(vk::Image::from_raw(1)));
		assert!(
			AllocationChain::new_from_record_with_import(&texture, ImportMemoryInfo { resource, ..info })
				.is_ok()
		);
		let null = vk::ImportMemoryMetalHandleInfoEXT {
			handle: std::ptr::null_mut(),
			..texture
		};
		assert!(matches!(
			AllocationChain::new_from_record_with_import(&null, ImportMemoryInfo { resource, ..info }),
			Err(AllocError::InvalidRequest(_))
		));
	}
}
