//! Platform import accounting through real allocator owners and exact mock PFNs.
use super::*;
use crate::mem::{AllocationChain, ImportMemoryInfo, ImportResource};

fn metadata() -> ImportMemoryInfo {
	ImportMemoryInfo {
		allocation_size: 4096,
		memory_type_bits: 2,
		resource: None,
	}
}

fn exercise(
	chain: AllocationChain<'_>,
	extension: &CStr,
	tag: vk::StructureType,
	native_zero: bool,
) {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[extension], true, None);
	let mut properties = properties();
	properties.memory_type_count = 2;
	properties.memory_types[1] = properties.memory_types[0];
	// SAFETY: static synthetic PFNs and exact independent two-type fixture.
	let mut allocator = unsafe { Allocator::new(&fixture.device, properties, 64) }.unwrap();
	STATE.with_borrow_mut(|state| {
		state.pooled = true;
		state.import_tag = Some(tag);
	});
	let options = AllocationCreateInfo {
		memory_allocate_next: Some(chain),
		..Default::default()
	};
	let resource = chain.data.import_info.unwrap().resource;
	// SAFETY: synthetic handles are consumed only by exact mock PFNs; explicit
	// metadata represents this mock payload, with no real OS import/ownership transfer.
	unsafe {
		let mut allocation = match resource {
			Some(ImportResource::Buffer(buffer)) => allocator.allocate_for_buffer(buffer, &options),
			Some(ImportResource::Image(image)) => allocator.allocate_for_image(image, &options),
			None => allocator.allocate_dedicated(
				&vk::MemoryRequirements {
					size: 257,
					alignment: 64,
					memory_type_bits: 3,
				},
				&options,
				None,
			),
		}
		.unwrap();
		assert_eq!(allocation.size(), 4096);
		assert_eq!(allocation.reserved_size(), 4096);
		assert_eq!(allocation.memory_type_index(), 1);
		assert_eq!(allocator.statistics().native_bytes, 4096);
		assert_eq!(allocator.statistics().allocation_bytes, 4096);
		assert_eq!(
			STATE.with_borrow(|state| state.allocated.clone()),
			vec![(1, if native_zero { 0 } else { 4096 })]
		);
		assert_eq!(
			STATE.with_borrow(|state| state.observed_import),
			chain.as_ptr() as usize
		);
		assert_eq!(events(), vec!["allocate"]);
		match resource {
			Some(ImportResource::Buffer(buffer)) => {
				assert_eq!(STATE.with_borrow(|state| state.dedicated), buffer);
				allocator.bind_buffer_memory(buffer, &allocation).unwrap();
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			}
			Some(ImportResource::Image(image)) => {
				assert_eq!(STATE.with_borrow(|state| state.dedicated_image), image);
				allocator.bind_image_memory(image, &allocation).unwrap();
				allocator.destroy_image(image, &mut allocation).unwrap();
			}
			None => allocator.free(&mut allocation).unwrap(),
		}
	}
	assert_eq!(allocator.statistics().native_bytes, 0);
	assert_eq!(allocator.statistics().allocation_bytes, 0);
	allocator.close().unwrap();
	assert!(!events().iter().any(|name| name.contains("requirements")));
}

macro_rules! platform {
	($feature:literal, $test:ident, $record:expr, $extension:literal, $tag:ident, $resource:expr, $zero:expr) => {
		#[cfg(feature = $feature)]
		#[test]
		fn $test() {
			let record = $record;
			// SAFETY: complete immutable record, synthetic opaque external objects.
			assert!(matches!(
				unsafe { AllocationChain::new_from_record(&record) },
				Err(AllocError::InvalidRequest(_))
			));
			// SAFETY: exact synthetic payload extent/types/resource for mock PFNs only.
			let chain = unsafe {
				AllocationChain::new_from_record_with_import(
					&record,
					ImportMemoryInfo {
						resource: $resource,
						..metadata()
					},
				)
			}
			.unwrap();
			exercise(chain, $extension, vk::StructureType::$tag, $zero);
		}
	};
}

platform!(
	"platform-android",
	android_image_uses_external_size_and_types_without_prebind_query,
	vk::ImportAndroidHardwareBufferInfoANDROID {
		buffer: std::ptr::dangling_mut(),
		..Default::default()
	},
	c"VK_ANDROID_external_memory_android_hardware_buffer",
	IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
	Some(ImportResource::Image(vk::Image::from_raw(71))),
	false
);

platform!(
	"platform-screen",
	qnx_image_accounts_external_properties,
	vk::ImportScreenBufferInfoQNX {
		buffer: std::ptr::dangling_mut(),
		..Default::default()
	},
	c"VK_QNX_external_memory_screen_buffer",
	IMPORT_SCREEN_BUFFER_INFO_QNX,
	Some(ImportResource::Image(vk::Image::from_raw(72))),
	false
);

platform!(
	"platform-ohos",
	ohos_image_accounts_external_properties,
	vk::ImportNativeBufferInfoOHOS {
		buffer: std::ptr::dangling_mut(),
		..Default::default()
	},
	c"VK_OHOS_external_memory",
	IMPORT_NATIVE_BUFFER_INFO_OHOS,
	Some(ImportResource::Image(vk::Image::from_raw(73))),
	false
);

platform!(
	"platform-metal",
	metal_texture_passes_zero_native_size_but_accounts_all_bytes,
	vk::ImportMemoryMetalHandleInfoEXT {
		handle: std::ptr::dangling_mut(),
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_BIT_EXT,
		..Default::default()
	},
	c"VK_EXT_external_memory_metal",
	IMPORT_MEMORY_METAL_HANDLE_INFO_EXT,
	Some(ImportResource::Image(vk::Image::from_raw(74))),
	true
);

platform!(
	"platform-metal",
	metal_heap_accounts_backing_extent,
	vk::ImportMemoryMetalHandleInfoEXT {
		handle: std::ptr::dangling_mut(),
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::MTLHEAP_BIT_EXT,
		..Default::default()
	},
	c"VK_EXT_external_memory_metal",
	IMPORT_MEMORY_METAL_HANDLE_INFO_EXT,
	None,
	false
);

platform!(
	"platform-metal",
	metal_objects_buffer_accounts_backing_extent,
	vk::ImportMetalBufferInfoEXT {
		mtl_buffer: std::ptr::dangling_mut(),
		..Default::default()
	},
	c"VK_EXT_metal_objects",
	IMPORT_METAL_BUFFER_INFO_EXT,
	None,
	false
);

platform!(
	"platform-fuchsia",
	zircon_vmo_accounts_external_size_and_types,
	vk::ImportMemoryZirconHandleInfoFUCHSIA {
		handle: 17,
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_BIT_FUCHSIA,
		..Default::default()
	},
	c"VK_FUCHSIA_external_memory",
	IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
	None,
	false
);

platform!(
	"platform-fuchsia",
	collection_requires_exact_resource_dedication,
	vk::ImportMemoryBufferCollectionFUCHSIA {
		collection: vk::BufferCollectionFUCHSIA::from_raw(18),
		index: 3,
		..Default::default()
	},
	c"VK_FUCHSIA_buffer_collection",
	IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
	Some(ImportResource::Buffer(vk::Buffer::from_raw(75))),
	false
);

platform!(
	"platform-win32",
	win32_texture_uses_actual_size_for_accounting,
	vk::ImportMemoryWin32HandleInfoKHR {
		handle: std::ptr::dangling_mut(),
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE_BIT,
		..Default::default()
	},
	c"VK_KHR_external_memory_win32",
	IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
	None,
	false
);

platform!(
	"platform-win32",
	nv_win32_image_uses_actual_size_for_accounting,
	vk::ImportMemoryWin32HandleInfoNV {
		handle: std::ptr::dangling_mut(),
		handle_type: vk::ExternalMemoryHandleTypeFlagBitsNV::D3D11_IMAGE_BIT_NV,
		..Default::default()
	},
	c"VK_NV_external_memory_win32",
	IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
	None,
	false
);

#[test]
fn host_imports_omit_resource_dedication_and_reject_required_dedication() {
	let fixture = Fixture::new(
		vk::API_VERSION_1_1,
		&[c"VK_EXT_external_memory_host"],
		true,
		None,
	);
	let record = vk::ImportMemoryHostPointerInfoEXT {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_BIT_EXT,
		p_host_pointer: std::ptr::dangling_mut(),
		..Default::default()
	};
	// SAFETY: immutable record inspected only; external address is never dereferenced by mocks.
	let chain = unsafe { AllocationChain::new_from_record(&record) }.unwrap();
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|state| state.import_tag = Some(record.s_type));
	let options = AllocationCreateInfo {
		memory_allocate_next: Some(chain),
		..Default::default()
	};
	// SAFETY: exact synthetic resource/chain and no native OS memory access.
	unsafe {
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		assert!(STATE.with_borrow(|state| state.dedicated.is_null()));
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		STATE.with_borrow_mut(|state| {
			state.required = true;
			state.observed_import = 0;
		});
		clear();
		assert!(matches!(
			allocator.allocate_for_buffer(vk::Buffer::from_raw(88), &options),
			Err(AllocError::InvalidRequest(_))
		));
		assert_eq!(events(), vec!["requirements_core"]);
	}
	allocator.close().unwrap();
}

#[test]
fn imported_accounting_intersects_types_limits_extent_and_preserves_single_attempt() {
	let fixture = Fixture::new(
		vk::API_VERSION_1_1,
		&[c"VK_KHR_external_memory_fd"],
		true,
		None,
	);
	let record = vk::ImportMemoryFdInfoKHR {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		fd: 7,
		..Default::default()
	};
	// SAFETY: immutable record and synthetic payload accounting, never real OS access.
	let chain = unsafe {
		AllocationChain::new_from_record_with_import(
			&record,
			ImportMemoryInfo {
				allocation_size: 4096,
				memory_type_bits: 1,
				resource: None,
			},
		)
	}
	.unwrap();
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|state| {
		state.pooled = true;
		state.import_tag = Some(record.s_type);
	});
	let options = AllocationCreateInfo {
		memory_allocate_next: Some(chain),
		..Default::default()
	};
	let requirements = vk::MemoryRequirements {
		size: 257,
		alignment: 64,
		memory_type_bits: 1,
	};
	allocator.set_heap_limit(0, 4095).unwrap();
	// SAFETY: mock-only inputs; all rejection precedes native acquisition.
	unsafe {
		assert_eq!(
			allocator
				.allocate_dedicated(&requirements, &options, None)
				.err(),
			Some(AllocError::OutOfSpace)
		);
		assert!(events().is_empty());
		allocator.set_heap_limit(0, 1 << 30).unwrap();
		assert_eq!(
			allocator
				.allocate_dedicated(
					&vk::MemoryRequirements {
						size: 4097,
						..requirements
					},
					&options,
					None
				)
				.err(),
			Some(AllocError::InvalidRequest(
				"resource exceeds imported payload"
			))
		);
		assert_eq!(
			allocator
				.allocate_dedicated(
					&vk::MemoryRequirements {
						memory_type_bits: 2,
						..requirements
					},
					&options,
					None
				)
				.err(),
			Some(AllocError::NoCompatibleMemoryType)
		);
		assert!(events().is_empty());
		STATE.with_borrow_mut(|state| state.fail_above = Some(0));
		assert!(matches!(
			allocator.allocate_dedicated(&requirements, &options, None),
			Err(AllocError::Call(_))
		));
		assert_eq!(events(), vec!["allocate"]);
		assert_eq!(allocator.statistics().native_bytes, 0);
	}
}

#[test]
fn resource_import_admission_retains_owner_across_binding_failure_and_checks_duplicates() {
	let record = vk::ImportMemoryFdInfoKHR {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		fd: 7,
		..Default::default()
	};
	let buffer = vk::Buffer::from_raw(77);
	// SAFETY: synthetic exact native payload/resource, no actual FD transfer.
	let chain = unsafe {
		AllocationChain::new_from_record_with_import(
			&record,
			ImportMemoryInfo {
				allocation_size: 4096,
				memory_type_bits: 1,
				resource: Some(ImportResource::Buffer(buffer)),
			},
		)
	}
	.unwrap();
	let options = AllocationCreateInfo {
		memory_allocate_next: Some(chain),
		..Default::default()
	};
	{
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		// SAFETY: missing exact extension must reject before native import.
		assert!(matches!(
			unsafe { allocator.allocate_for_buffer(buffer, &options) },
			Err(AllocError::UnsupportedCapability(_))
		));
		assert!(events().is_empty());
	}
	let fixture = Fixture::new(
		vk::API_VERSION_1_1,
		&[c"VK_KHR_external_memory_fd"],
		true,
		None,
	);
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|state| {
		state.pooled = true;
		state.import_tag = Some(record.s_type);
	});
	// SAFETY: exact mock identities and retained immutable import record; no GPU uses.
	unsafe {
		assert_eq!(
			allocator
				.allocate_for_buffer(vk::Buffer::from_raw(78), &options)
				.err(),
			Some(AllocError::WrongResource)
		);
		assert!(events().is_empty());
		let mut allocation = allocator.allocate_for_buffer(buffer, &options).unwrap();
		assert!(matches!(
			allocator.allocate_for_buffer(buffer, &options),
			Err(AllocError::InvalidRequest(_))
		));
		assert_eq!(events(), vec!["allocate"]);
		assert_eq!(
			allocator.bind_buffer_memory(vk::Buffer::from_raw(78), &allocation),
			Err(AllocError::WrongResource)
		);
		STATE.with_borrow_mut(|state| state.fail = Some("bind"));
		assert!(matches!(
			allocator.bind_buffer_memory(buffer, &allocation),
			Err(AllocError::Call(_))
		));
		assert_eq!(allocator.statistics().native_bytes, 4096);
		assert_eq!(allocator.statistics().allocation_count, 1);
		assert!(!allocator.live[&allocation.serial].bound);
		STATE.with_borrow_mut(|state| state.fail = None);
		allocator.bind_buffer_memory(buffer, &allocation).unwrap();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
	}
	assert_eq!(allocator.statistics().native_bytes, 0);
	allocator.close().unwrap();
}
