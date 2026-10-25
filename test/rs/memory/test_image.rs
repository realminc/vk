//! Independent exact-PFN image lifecycle, storage-domain and chain-admission traces.
use super::*;

fn info(tiling: vk::ImageTiling) -> vk::ImageCreateInfo {
	vk::ImageCreateInfo {
		image_type: vk::ImageType::_2D,
		format: vk::Format::R8G8B8A8_UNORM,
		extent: vk::Extent3D {
			width: 7,
			height: 9,
			depth: 1,
		},
		mip_levels: 1,
		array_layers: 1,
		samples: vk::SampleCountFlagBits::_1_BIT,
		tiling,
		usage: vk::ImageUsageFlagBits::TRANSFER_DST_BIT,
		..Default::default()
	}
}

pub(super) unsafe extern "system" fn create_native_image(
	_: vk::DeviceHandle,
	info: *const vk::ImageCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::Image,
) -> vk::Result {
	assert!(callbacks.is_null());
	let status = event("create_image");
	// SAFETY: wrapper supplies valid synchronous input/output records.
	unsafe {
		assert_eq!((*info).extent.width, 7);
		STATE.with_borrow_mut(|s| {
			s.create_chain = (*info).p_next as usize;
			if status == vk::Result::SUCCESS {
				*output = if s.fail == Some("null_image") {
					vk::Image::NULL
				} else {
					vk::Image::from_raw(s.next_buffer)
				};
				s.next_buffer += 1;
			}
		});
	}
	status
}

pub(super) unsafe extern "system" fn destroy_native_image(
	_: vk::DeviceHandle,
	image: vk::Image,
	callbacks: *const vk::AllocationCallbacks,
) {
	assert!(!image.is_null());
	assert!(callbacks.is_null());
	event("destroy_image");
}

pub(super) unsafe extern "system" fn requirements_legacy(
	_: vk::DeviceHandle,
	_: vk::Image,
	output: *mut vk::MemoryRequirements,
) {
	event("image_requirements_legacy");
	// SAFETY: wrapper supplies a writable requirements record.
	unsafe {
		*output = native_requirements();
	}
}

pub(super) unsafe extern "system" fn requirements_core(
	_: vk::DeviceHandle,
	query: *const vk::ImageMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	event("image_requirements_core");
	// SAFETY: exact live native inputs/output chain forwarded unchanged.
	unsafe {
		requirements2(query, output);
	}
}

pub(super) unsafe extern "system" fn requirements_khr(
	_: vk::DeviceHandle,
	query: *const vk::ImageMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	event("image_requirements_khr");
	// SAFETY: exact live native inputs/output chain forwarded unchanged.
	unsafe {
		requirements2(query, output);
	}
}

unsafe fn requirements2(
	query: *const vk::ImageMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	// SAFETY: complete initialized requirements/dedication output chain from allocator.
	unsafe {
		assert_eq!(
			(*query).s_type,
			vk::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2
		);
		assert!(!(*query).image.is_null());
		assert_eq!((*output).s_type, vk::StructureType::MEMORY_REQUIREMENTS_2);
		let dedication = &mut *(*output).p_next.cast::<vk::MemoryDedicatedRequirements>();
		assert_eq!(
			dedication.s_type,
			vk::StructureType::MEMORY_DEDICATED_REQUIREMENTS
		);
		assert!(dedication.p_next.is_null());
		dedication.requires_dedicated_allocation = STATE.with_borrow(|s| u32::from(s.required));
		dedication.prefers_dedicated_allocation = STATE.with_borrow(|s| u32::from(s.preferred));
		(*output).memory_requirements = native_requirements();
	}
}

pub(super) unsafe extern "system" fn bind_image(
	_: vk::DeviceHandle,
	image: vk::Image,
	_: vk::DeviceMemory,
	offset: u64,
) -> vk::Result {
	assert!(!image.is_null());
	assert!(offset.is_multiple_of(64));
	STATE.with_borrow_mut(|s| s.bindings.push((image.as_raw(), offset)));
	event("bind_image")
}

#[test]
fn exact_legacy_core_and_khr_queries_dedicate_and_release_the_matching_image() {
	for (version, extensions, query) in [
		(vk::API_VERSION_1_0, vec![], "image_requirements_legacy"),
		(vk::API_VERSION_1_1, vec![], "image_requirements_core"),
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_dedicated_allocation",
				c"VK_KHR_get_memory_requirements2",
			],
			"image_requirements_khr",
		),
	] {
		let fixture = Fixture::new(version, &extensions, true, None);
		let mut allocator = fixture.allocator();
		// SAFETY: static exact-PFN mock objects have no native resource use.
		unsafe {
			let (image, mut allocation) = allocator
				.create_image(
					&info(vk::ImageTiling::OPTIMAL),
					request().required_flags,
					request().preferred_flags,
				)
				.unwrap();
			assert_eq!(events(), ["create_image", query, "allocate", "bind_image"]);
			if version != vk::API_VERSION_1_0 || !extensions.is_empty() {
				assert_eq!(STATE.with_borrow(|s| s.dedicated_image), image);
				assert!(STATE.with_borrow(|s| s.dedicated).is_null());
			}
			assert_eq!(
				allocator.destroy_buffer(vk::Buffer::from_raw(image.as_raw()), &mut allocation),
				Err(AllocError::WrongResource)
			);
			allocator.destroy_image(image, &mut allocation).unwrap();
			assert!(allocation.is_freed());
			assert_eq!(&events()[4..], ["destroy_image", "free"]);
			allocator.close().unwrap();
		}
	}
}

#[test]
fn image_failures_roll_back_only_new_resources_and_keep_original_error() {
	for failure in [
		"create_image",
		"allocate",
		"bind_image",
		"null_image",
		"null_memory",
		"bad_requirements",
		"map",
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		STATE.with_borrow_mut(|s| {
			s.fail = Some(failure);
			s.bad_requirements = failure == "bad_requirements";
		});
		let options = AllocationCreateInfo {
			persistent_map: failure == "map",
			..Default::default()
		};
		// SAFETY: independent mock failures affect only unpublished synthetic resources.
		let result =
			unsafe { allocator.create_image_with_options(&info(vk::ImageTiling::OPTIMAL), &options) };
		assert!(result.is_err(), "{failure}");
		assert_eq!(allocator.statistics().allocation_count, 0, "{failure}");
		assert_eq!(
			allocator.statistics().native_allocation_count,
			0,
			"{failure}"
		);
		if ["create_image", "allocate", "bind_image", "map"].contains(&failure) {
			assert!(
				matches!(
					result,
					Err(AllocError::Call(CallError::Vulkan(
						vk::Result::ERROR_DEVICE_LOST
					)))
				),
				"{failure}"
			);
		}
		let trace = events();
		assert_eq!(
			trace.iter().filter(|&&e| e == "destroy_image").count(),
			usize::from(!["create_image", "null_image"].contains(&failure))
		);
		assert_eq!(
			trace.iter().filter(|&&e| e == "free").count(),
			usize::from(["bind_image", "map"].contains(&failure))
		);
	}
}

#[test]
fn image_and_buffer_block_domains_are_disjoint_and_homogeneous_images_share() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 4096 },
		persistent_map: true,
		..Default::default()
	};
	// SAFETY: synthetic compatible resources; each exact managed pair is released explicitly.
	unsafe {
		let (buffer, mut b) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		let (optimal, mut o) = allocator
			.create_image_with_options(&info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		let (second, mut s) = allocator
			.create_image_with_options(&info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		let (linear, mut l) = allocator
			.create_image_with_options(&info(vk::ImageTiling::LINEAR), &options)
			.unwrap();
		assert_ne!(b.handle(), o.handle());
		assert_ne!(b.handle(), l.handle());
		assert_ne!(o.handle(), l.handle());
		assert_eq!(o.handle(), s.handle());
		assert!(s.offset() >= o.offset() + o.reserved_size());
		assert_eq!(allocator.statistics().pooled_block_count, 3);
		assert!(allocator.mapped_pointer(&s).unwrap().is_some());
		allocator.map(&o).unwrap();
		assert_eq!(
			allocator.destroy_image(optimal, &mut o),
			Err(AllocError::AlreadyMapped)
		);
		allocator.unmap(&o).unwrap();
		allocator.destroy_image(optimal, &mut o).unwrap();
		allocator.destroy_image(second, &mut s).unwrap();
		allocator.destroy_image(linear, &mut l).unwrap();
		allocator.destroy_buffer(buffer, &mut b).unwrap();
		assert_eq!(allocator.statistics().mapped_native_count, 0);
		allocator.close().unwrap();
	}
}

#[test]
fn required_and_preferred_dedication_override_image_pooling() {
	for preferred in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| {
			s.required = !preferred;
			s.preferred = preferred;
		});
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 4096 },
			..Default::default()
		};
		// SAFETY: synthetic exact requirements and native resource pair only.
		unsafe {
			let (image, mut a) = allocator
				.create_image_with_options(&info(vk::ImageTiling::OPTIMAL), &options)
				.unwrap();
			assert_eq!(allocator.statistics().pooled_block_count, 0);
			assert_eq!(STATE.with_borrow(|s| s.dedicated_image), image);
			allocator.destroy_image(image, &mut a).unwrap();
		}
	}
}

#[test]
fn unknown_chains_sparse_and_disjoint_images_are_rejected_before_native_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	for flag in [
		vk::ImageCreateFlagBits::DISJOINT_BIT,
		vk::ImageCreateFlagBits::SPARSE_BINDING_BIT,
		vk::ImageCreateFlagBits::ALIAS_BIT,
		vk::ImageCreateFlagBits::PROTECTED_BIT,
	] {
		let mut info = info(vk::ImageTiling::OPTIMAL);
		info.flags = flag;
		// SAFETY: malformed requests are readable and rejected before native effects.
		assert!(
			unsafe { allocator.create_image_with_options(&info, &AllocationCreateInfo::default()) }
				.is_err()
		);
	}
	let node = vk::BaseInStructure {
		s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
		p_next: std::ptr::null(),
	};
	let mut info = info(vk::ImageTiling::OPTIMAL);
	info.p_next = std::ptr::from_ref(&node).cast();
	// SAFETY: complete readable unsupported native header.
	assert!(
		unsafe { allocator.create_image_with_options(&info, &AllocationCreateInfo::default()) }
			.is_err()
	);
	assert!(events().is_empty());
}

#[test]
fn video_and_format_list_chains_are_forwarded_without_retaining_borrowed_storage() {
	let fixture = Fixture::new(vk::API_VERSION_1_2, &[c"VK_KHR_video_queue"], true, None);
	let mut allocator = fixture.allocator();
	let profile = vk::VideoProfileInfoKHR::default();
	let format = vk::Format::R8G8B8A8_UNORM;
	let formats = vk::ImageFormatListCreateInfo {
		view_format_count: 1,
		p_view_formats: &format,
		..Default::default()
	};
	let mut profiles = vk::VideoProfileListInfoKHR {
		profile_count: 1,
		p_profiles: &profile,
		p_next: std::ptr::from_ref(&formats).cast(),
		..Default::default()
	};
	let mut info = info(vk::ImageTiling::OPTIMAL);
	info.p_next = std::ptr::from_ref(&profiles).cast();
	// SAFETY: structural-admission mocks; profile codec semantics are not a native video proof.
	unsafe {
		let (image, mut a) = allocator
			.create_image_with_options(&info, &AllocationCreateInfo::default())
			.unwrap();
		assert_eq!(STATE.with_borrow(|s| s.create_chain), info.p_next as usize);
		allocator.destroy_image(image, &mut a).unwrap();
		profiles.p_next = std::ptr::from_ref(&profiles).cast();
		info.p_next = std::ptr::from_ref(&profiles).cast();
		clear();
		assert!(
			allocator
				.create_image_with_options(&info, &AllocationCreateInfo::default())
				.is_err()
		);
		assert!(events().is_empty());
	}
}

#[test]
fn video_buffer_admission_preserves_the_chain_and_rejects_disabled_or_duplicate_records() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[c"VK_KHR_video_queue"], true, None);
	let mut allocator = fixture.allocator();
	let profile = vk::VideoProfileInfoKHR::default();
	let mut profiles = vk::VideoProfileListInfoKHR {
		profile_count: 1,
		p_profiles: &profile,
		..Default::default()
	};
	let mut info = buffer_info();
	info.p_next = std::ptr::from_ref(&profiles).cast();
	// SAFETY: complete synchronous chain for structural-admission mocks, no native video use.
	unsafe {
		let (buffer, mut a) = allocator
			.create_buffer_with_options(&info, &AllocationCreateInfo::default())
			.unwrap();
		assert_eq!(STATE.with_borrow(|s| s.create_chain), info.p_next as usize);
		allocator.destroy_buffer(buffer, &mut a).unwrap();
		let duplicate = vk::VideoProfileListInfoKHR {
			profile_count: 1,
			p_profiles: &profile,
			..Default::default()
		};
		profiles.p_next = std::ptr::from_ref(&duplicate).cast();
		info.p_next = std::ptr::from_ref(&profiles).cast();
		clear();
		assert!(
			allocator
				.create_buffer_with_options(&info, &AllocationCreateInfo::default())
				.is_err()
		);
		assert!(events().is_empty());
	}
	profiles.p_next = std::ptr::null();
	info.p_next = std::ptr::from_ref(&profiles).cast();
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: readable chain rejected before traversing or forwarding the disabled profile list.
	assert!(
		unsafe { allocator.create_buffer_with_options(&info, &AllocationCreateInfo::default()) }
			.is_err()
	);
	assert!(events().is_empty());
}
