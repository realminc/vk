//! Public selection/raw requests against independent native effects and page models.
use super::*;
use crate::mem::{AllocationAlgorithm, HostAccess, MemoryUsage, PoolCreateInfo};

pub(super) unsafe extern "system" fn buffer_core(
	_: vk::DeviceHandle,
	info: *const vk::DeviceBufferMemoryRequirements,
	output: *mut vk::MemoryRequirements2,
) {
	event("device_buffer_core");
	// SAFETY: exact wrapper query/output records; native create info remains borrowed for this call.
	unsafe {
		assert!(!(*info).p_create_info.is_null());
		assert!((*info).p_next.is_null());
		(*output).memory_requirements = native_requirements();
	}
}

pub(super) unsafe extern "system" fn buffer_khr(
	device: vk::DeviceHandle,
	info: *const vk::DeviceBufferMemoryRequirements,
	output: *mut vk::MemoryRequirements2,
) {
	// SAFETY: identical native aliases with valid initialized records.
	unsafe { buffer_core(device, info, output) };
	STATE.with_borrow_mut(|s| *s.events.last_mut().unwrap() = "device_buffer_khr");
}

pub(super) unsafe extern "system" fn image_core(
	_: vk::DeviceHandle,
	info: *const vk::DeviceImageMemoryRequirements,
	output: *mut vk::MemoryRequirements2,
) {
	event("device_image_core");
	// SAFETY: exact non-disjoint image query and initialized output supplied by wrapper.
	unsafe {
		assert!(!(*info).p_create_info.is_null());
		assert_eq!((*info).plane_aspect.as_raw(), 0);
		(*output).memory_requirements = native_requirements();
	}
}

pub(super) unsafe extern "system" fn image_khr(
	device: vk::DeviceHandle,
	info: *const vk::DeviceImageMemoryRequirements,
	output: *mut vk::MemoryRequirements2,
) {
	// SAFETY: identical native aliases and live records.
	unsafe { image_core(device, info, output) };
	STATE.with_borrow_mut(|s| *s.events.last_mut().unwrap() = "device_image_khr");
}

#[test]
fn creation_queries_use_exact_core_khr_or_temporary_routes_without_allocating() {
	for (version, extensions, buffer_events, image_events) in [
		(
			vk::API_VERSION_1_0,
			vec![],
			vec!["create", "requirements_legacy", "destroy_buffer"],
			vec!["create_image", "image_requirements_legacy", "destroy_image"],
		),
		(
			vk::API_VERSION_1_1,
			vec![],
			vec!["create", "requirements_legacy", "destroy_buffer"],
			vec!["create_image", "image_requirements_legacy", "destroy_image"],
		),
		(
			vk::API_VERSION_1_3,
			vec![],
			vec!["device_buffer_core"],
			vec!["device_image_core"],
		),
		(
			vk::API_VERSION_1_1,
			vec![c"VK_KHR_maintenance4"],
			vec!["device_buffer_khr"],
			vec!["device_image_khr"],
		),
	] {
		let fixture = Fixture::new(version, &extensions, true, None);
		let allocator = fixture.allocator();
		// SAFETY: ordinary synthetic creation metadata; no real native objects or GPU access.
		unsafe {
			assert_eq!(
				allocator
					.find_memory_type_for_buffer_info(&buffer_info(), &AllocationCreateInfo::default()),
				Ok(0)
			);
			assert_eq!(events(), buffer_events);
			clear();
			assert_eq!(
				allocator.find_memory_type_for_image_info(
					&image::info(vk::ImageTiling::OPTIMAL),
					&AllocationCreateInfo::default()
				),
				Ok(0)
			);
			assert_eq!(events(), image_events);
		}
		assert_eq!(allocator.statistics(), AllocatorStats::default());
	}
}

#[test]
fn temporary_queries_clean_up_on_invalid_requirements_or_no_compatible_type() {
	for is_image in [false, true] {
		for bad_output in [false, true] {
			let fixture = Fixture::new(vk::API_VERSION_1_0, &[], true, None);
			let allocator = fixture.allocator();
			STATE.with_borrow_mut(|s| {
				s.bad_requirements = bad_output;
				s.requirements_bits = Some(0);
			});
			// SAFETY: independently injected invalid native requirements; temporaries have no uses.
			let result = unsafe {
				if is_image {
					allocator.find_memory_type_for_image_info(
						&image::info(vk::ImageTiling::LINEAR),
						&AllocationCreateInfo::default(),
					)
				} else {
					allocator
						.find_memory_type_for_buffer_info(&buffer_info(), &AllocationCreateInfo::default())
				}
			};
			assert!(result.is_err());
			assert_eq!(
				events().last(),
				Some(&if is_image {
					"destroy_image"
				} else {
					"destroy_buffer"
				})
			);
			assert!(!events().contains(&"allocate"));
		}
	}
}

#[test]
fn creation_queries_preflight_missing_inverse_and_never_fall_back_from_admitted_direct_spelling() {
	for (version, blocked) in [
		(vk::API_VERSION_1_0, b"vkDestroyBuffer".as_slice()),
		(
			vk::API_VERSION_1_3,
			b"vkGetDeviceBufferMemoryRequirements".as_slice(),
		),
	] {
		let fixture = Fixture::new(version, &[], true, Some(blocked));
		let allocator = fixture.allocator();
		// SAFETY: ordinary mock create info; absent immutable PFN fails before native effects.
		assert!(
			unsafe {
				allocator.find_memory_type_for_buffer_info(&buffer_info(), &AllocationCreateInfo::default())
			}
			.is_err()
		);
		assert!(events().is_empty());
	}
}

#[test]
fn selection_uses_usage_and_masks_but_does_not_admit_or_reserve_storage() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut props = properties();
	props.memory_type_count = 2;
	props.memory_types[1].property_flags = vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT;
	// SAFETY: immutable synthetic memory properties belonging to this mock device.
	let mut allocator = unsafe { Allocator::new(&fixture.device, props, 64) }.unwrap();
	allocator.allocation_limit = 0;
	allocator.next_serial = u64::MAX;
	STATE.with_borrow_mut(|s| s.requirements_bits = Some(3));
	let options = AllocationCreateInfo {
		usage: MemoryUsage::Auto,
		..Default::default()
	};
	// SAFETY: ordinary buffer creation input with device access; temporary is explicitly destroyed.
	assert_eq!(
		unsafe { allocator.find_memory_type_for_buffer_info(&buffer_info(), &options) },
		Ok(1)
	);
	let host = AllocationCreateInfo {
		host_access: HostAccess::Random,
		..options
	};
	// SAFETY: same ordinary input with checked required host-visible preference.
	assert_eq!(
		unsafe { allocator.find_memory_type_for_buffer_info(&buffer_info(), &host) },
		Ok(0)
	);
	assert_eq!(
		allocator.find_memory_type(
			3,
			&AllocationCreateInfo {
				preferred_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
				..Default::default()
			}
		),
		Ok(1)
	);
	assert_eq!(
		allocator.find_memory_type(
			3,
			&AllocationCreateInfo {
				memory_type_bits: 0,
				..Default::default()
			}
		),
		Err(AllocError::NoCompatibleMemoryType)
	);
	assert!(allocator.find_memory_type(3, &options).is_err());
	assert_eq!(allocator.statistics(), AllocatorStats::default());
	assert!(!events().contains(&"allocate"));
}

#[test]
fn raw_pool_requests_remain_unclassified_after_binding_and_never_share_conflicting_pages() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| s.pooled = true);
		// SAFETY: ordinary mock storage with checked granularity and explicit completion-free cleanup.
		unsafe {
			let mut allocator =
				Allocator::new_with_granularity(&fixture.device, properties(), 64, 1024).unwrap();
			let mut pool = allocator
				.create_pool(&PoolCreateInfo {
					algorithm,
					block_size: 8192,
					max_block_count: 1,
					..Default::default()
				})
				.unwrap();
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				persistent_map: true,
				min_allocation_alignment: 2048,
				..Default::default()
			};
			let mut a = allocator
				.allocate_with_options(&native_requirements(), &options)
				.unwrap();
			allocator
				.bind_buffer_memory(vk::Buffer::from_raw(700), &a)
				.unwrap();
			let mut b = allocator
				.allocate_with_options(&native_requirements(), &options)
				.unwrap();
			allocator
				.bind_image_memory(vk::Image::from_raw(701), &b)
				.unwrap();
			assert_eq!((a.offset(), b.offset()), (0, 2048));
			assert_eq!(a.handle(), b.handle());
			assert_eq!(allocator.statistics().mapped_native_count, 1);
			assert!(!events().contains(&"create"));
			assert!(!events().contains(&"create_image"));
			allocator
				.destroy_buffer(vk::Buffer::from_raw(700), &mut a)
				.unwrap();
			allocator
				.destroy_image(vk::Image::from_raw(701), &mut b)
				.unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn raw_requests_reject_automatic_usage_invalid_alignment_and_required_dedication() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: malformed policy and synthetic native dedication rejected without binding.
	unsafe {
		for options in [
			AllocationCreateInfo {
				usage: MemoryUsage::Auto,
				..Default::default()
			},
			AllocationCreateInfo {
				min_allocation_alignment: 3,
				..Default::default()
			},
		] {
			assert!(
				allocator
					.allocate_with_options(&native_requirements(), &options)
					.is_err()
			);
			assert!(events().is_empty());
		}
		let mut a = allocator
			.allocate_with_options(&native_requirements(), &AllocationCreateInfo::default())
			.unwrap();
		STATE.with_borrow_mut(|s| s.required = true);
		clear();
		assert!(
			allocator
				.bind_buffer_memory(vk::Buffer::from_raw(700), &a)
				.is_err()
		);
		assert!(!events().contains(&"bind"));
		allocator.free(&mut a).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn raw_persistent_map_failure_releases_storage_without_creating_a_resource() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|s| s.fail = Some("map"));
	// SAFETY: new unbound synthetic storage; map fails before publishing any pointer.
	assert!(
		unsafe {
			allocator.allocate_with_options(
				&native_requirements(),
				&AllocationCreateInfo {
					persistent_map: true,
					..Default::default()
				},
			)
		}
		.is_err()
	);
	assert_eq!(events(), ["allocate", "map", "free"]);
	assert_eq!(allocator.statistics(), AllocatorStats::default());
}

#[test]
fn minimum_alignment_applies_to_managed_and_existing_resource_paths_before_any_creation() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: ordinary synthetic resources with independent explicit release and no GPU access.
	unsafe {
		let options = AllocationCreateInfo {
			min_allocation_alignment: 1024,
			policy: AllocationPolicy::Pooled { block_size: 8192 },
			..Default::default()
		};
		let (buffer, mut a) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		let (image, mut b) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		let mut c = allocator
			.allocate_for_buffer(vk::Buffer::from_raw(700), &options)
			.unwrap();
		assert_eq!((a.offset(), b.offset(), c.offset()), (0, 1024, 2048));
		clear();
		let invalid = AllocationCreateInfo {
			min_allocation_alignment: 3,
			..options
		};
		assert!(
			allocator
				.create_buffer_with_options(&buffer_info(), &invalid)
				.is_err()
		);
		assert!(
			allocator
				.create_image_with_options(&image::info(vk::ImageTiling::LINEAR), &invalid)
				.is_err()
		);
		assert!(events().is_empty());
		allocator.destroy_buffer(buffer, &mut a).unwrap();
		allocator.destroy_image(image, &mut b).unwrap();
		allocator.free(&mut c).unwrap();
		allocator.close().unwrap();
	}
}
