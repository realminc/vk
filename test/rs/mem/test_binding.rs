//! Existing-resource ownership, binding state and native rollback are independent of creation.
use super::*;
use crate::mem::{AllocationAlgorithm, MemoryUsage, PoolCreateInfo};

#[test]
fn existing_buffer_queries_exact_routes_and_never_creates_or_destroys_the_handle() {
	for (version, extensions, query) in [
		(vk::API_VERSION_1_0, vec![], "requirements_legacy"),
		(vk::API_VERSION_1_1, vec![], "requirements_core"),
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_dedicated_allocation",
				c"VK_KHR_get_memory_requirements2",
			],
			"requirements_khr",
		),
	] {
		let fixture = Fixture::new(version, &extensions, true, None);
		let mut allocator = fixture.allocator();
		let buffer = vk::Buffer::from_raw(700);
		// SAFETY: caller-owned live mock buffer; no GPU uses, explicit destruction outside allocator.
		unsafe {
			let mut allocation = allocator
				.allocate_for_buffer(buffer, &AllocationCreateInfo::default())
				.unwrap();
			assert_eq!(events(), [query, "allocate"]);
			assert!(matches!(
				allocator.allocate_for_buffer(buffer, &AllocationCreateInfo::default()),
				Err(AllocError::InvalidRequest(_))
			));
			clear();
			allocator.bind_buffer_memory(buffer, &allocation).unwrap();
			assert_eq!(events(), ["bind"]);
			clear();
			assert!(matches!(
				allocator.bind_buffer_memory(buffer, &allocation),
				Err(AllocError::InvalidRequest(_))
			));
			assert!(events().is_empty());
			fixture.device.destroy_buffer(buffer, None).unwrap();
			allocator.free(&mut allocation).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn existing_image_allocation_does_not_guess_tiling_and_can_share_only_separate_pages() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| s.pooled = true);
		// SAFETY: exact synthetic properties/ordinary image handles; caller explicitly releases everything.
		unsafe {
			let mut allocator =
				Allocator::new_with_granularity(&fixture.device, properties(), 64, 1024).unwrap();
			let mut pool = allocator
				.create_pool(&PoolCreateInfo {
					algorithm,
					block_size: 4096,
					max_block_count: 1,
					..Default::default()
				})
				.unwrap();
			let opts = AllocationCreateInfo {
				pool: Some(&pool),
				persistent_map: true,
				..Default::default()
			};
			let a = vk::Image::from_raw(701);
			let b = vk::Image::from_raw(702);
			let mut x = allocator.allocate_for_image(a, &opts).unwrap();
			let mut y = allocator.allocate_for_image(b, &opts).unwrap();
			assert_eq!((x.offset(), y.offset()), (0, 1024));
			assert_eq!(x.handle(), y.handle());
			assert_eq!(allocator.statistics().mapped_native_count, 1);
			allocator.bind_image_memory(a, &x).unwrap();
			allocator.bind_image_memory(b, &y).unwrap();
			allocator.destroy_image(a, &mut x).unwrap();
			allocator.destroy_image(b, &mut y).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn allocation_or_mapping_failures_preserve_existing_handles_and_binding_failure_can_retry() {
	for image in [false, true] {
		for failure in ["allocate", "map"] {
			let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
			let mut allocator = fixture.allocator();
			STATE.with_borrow_mut(|s| s.fail = Some(failure));
			let opts = AllocationCreateInfo {
				persistent_map: true,
				..Default::default()
			};
			// SAFETY: injected mock failure; no caller resource is ever destroyed by allocation.
			unsafe {
				let failed = if image {
					allocator.allocate_for_image(vk::Image::from_raw(700), &opts)
				} else {
					allocator.allocate_for_buffer(vk::Buffer::from_raw(700), &opts)
				};
				assert!(matches!(
					failed,
					Err(AllocError::Call(CallError::Vulkan(
						vk::Result::ERROR_DEVICE_LOST
					)))
				));
				assert!(!events().iter().any(|event| event.starts_with("destroy")));
				assert_eq!(allocator.statistics(), AllocatorStats::default());
				allocator.close().unwrap();
			}
		}
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		// SAFETY: live unbound mock handles; failed native binding leaves them unbound for retry.
		unsafe {
			let mut token = if image {
				allocator.allocate_for_image(vk::Image::from_raw(700), &AllocationCreateInfo::default())
			} else {
				allocator.allocate_for_buffer(vk::Buffer::from_raw(700), &AllocationCreateInfo::default())
			}
			.unwrap();
			let before = allocator.statistics();
			STATE.with_borrow_mut(|s| s.fail = Some(if image { "bind_image" } else { "bind" }));
			clear();
			let failed = if image {
				allocator.bind_image_memory(vk::Image::from_raw(700), &token)
			} else {
				allocator.bind_buffer_memory(vk::Buffer::from_raw(700), &token)
			};
			assert!(failed.is_err());
			assert_eq!(allocator.statistics(), before);
			assert!(
				!events()
					.iter()
					.any(|event| event.starts_with("destroy") || *event == "free")
			);
			STATE.with_borrow_mut(|s| s.fail = None);
			if image {
				allocator
					.bind_image_memory(vk::Image::from_raw(700), &token)
					.unwrap();
				allocator
					.destroy_image(vk::Image::from_raw(700), &mut token)
					.unwrap();
			} else {
				allocator
					.bind_buffer_memory(vk::Buffer::from_raw(700), &token)
					.unwrap();
				allocator
					.destroy_buffer(vk::Buffer::from_raw(700), &mut token)
					.unwrap();
			}
			allocator.close().unwrap();
		}
	}
}

#[test]
fn invalid_policy_null_target_and_wrong_association_fail_before_native_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: malformed metadata/handles rejected before any native effect.
	unsafe {
		for usage in [
			MemoryUsage::Auto,
			MemoryUsage::PreferDevice,
			MemoryUsage::PreferHost,
		] {
			clear();
			assert!(
				allocator
					.allocate_for_buffer(
						vk::Buffer::from_raw(1),
						&AllocationCreateInfo {
							usage,
							..Default::default()
						}
					)
					.is_err()
			);
			assert!(events().is_empty());
		}
		clear();
		assert!(
			allocator
				.allocate_for_image(vk::Image::NULL, &AllocationCreateInfo::default())
				.is_err()
		);
		assert!(events().is_empty());
		let mut token = allocator
			.allocate_for_buffer(vk::Buffer::from_raw(700), &AllocationCreateInfo::default())
			.unwrap();
		clear();
		assert_eq!(
			allocator.bind_buffer_memory(vk::Buffer::from_raw(701), &token),
			Err(AllocError::WrongResource)
		);
		assert_eq!(
			allocator.bind_image_memory(vk::Image::from_raw(700), &token),
			Err(AllocError::WrongResource)
		);
		assert!(events().is_empty());
		allocator.free(&mut token).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn raw_separate_binding_checks_requirements_type_and_required_dedication() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: ordinary synthetic handles; unbound allocation retained across invalid binding attempts.
	unsafe {
		let mut token = allocator.allocate(&request()).unwrap();
		STATE.with_borrow_mut(|s| s.requirements_size = Some(258));
		clear();
		assert_eq!(
			allocator.bind_buffer_memory(vk::Buffer::from_raw(700), &token),
			Err(AllocError::InvalidRange)
		);
		assert!(!events().contains(&"bind"));
		STATE.with_borrow_mut(|s| {
			s.requirements_size = None;
			s.requirements_bits = Some(2);
		});
		assert_eq!(
			allocator.bind_image_memory(vk::Image::from_raw(700), &token),
			Err(AllocError::NoCompatibleMemoryType)
		);
		STATE.with_borrow_mut(|s| {
			s.requirements_bits = None;
			s.required = true;
		});
		assert!(matches!(
			allocator.bind_buffer_memory(vk::Buffer::from_raw(700), &token),
			Err(AllocError::UnsupportedCapability(_))
		));
		STATE.with_borrow_mut(|s| s.required = false);
		allocator
			.bind_buffer_memory(vk::Buffer::from_raw(700), &token)
			.unwrap();
		allocator
			.destroy_buffer(vk::Buffer::from_raw(700), &mut token)
			.unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn missing_inverse_or_binding_entry_points_do_not_destroy_caller_resources() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, Some(b"vkBindImageMemory"));
	let mut allocator = fixture.allocator();
	// SAFETY: unbound mock image and separate storage; binding PFN deliberately absent.
	unsafe {
		let mut token = allocator
			.allocate_for_image(vk::Image::from_raw(700), &AllocationCreateInfo::default())
			.unwrap();
		clear();
		assert!(matches!(
			allocator.bind_image_memory(vk::Image::from_raw(700), &token),
			Err(AllocError::Call(CallError::MissingCommand(
				"vkBindImageMemory"
			)))
		));
		assert!(events().is_empty());
		allocator.free(&mut token).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn duplicated_native_create_outputs_never_destroy_the_existing_live_object() {
	for image in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		// SAFETY: intentionally invalid mock output; only the first resource is actually live/owned.
		unsafe {
			let (handle, mut token) = if image {
				let (resource, allocation) = allocator
					.create_image(
						&image::info(vk::ImageTiling::OPTIMAL),
						request().required_flags,
						request().preferred_flags,
					)
					.unwrap();
				(resource.as_raw(), allocation)
			} else {
				let (resource, allocation) = allocator
					.create_buffer(
						&buffer_info(),
						request().required_flags,
						request().preferred_flags,
					)
					.unwrap();
				(resource.as_raw(), allocation)
			};
			STATE.with_borrow_mut(|s| s.next_buffer = handle);
			clear();
			let result = if image {
				allocator
					.create_image(
						&image::info(vk::ImageTiling::OPTIMAL),
						request().required_flags,
						request().preferred_flags,
					)
					.map(|_| ())
			} else {
				allocator
					.create_buffer(
						&buffer_info(),
						request().required_flags,
						request().preferred_flags,
					)
					.map(|_| ())
			};
			assert!(matches!(
				result,
				Err(AllocError::Call(CallError::InvalidOutput(_)))
			));
			assert!(!events().iter().any(|e| e.starts_with("destroy")));
			if image {
				allocator
					.destroy_image(vk::Image::from_raw(handle), &mut token)
					.unwrap();
			} else {
				allocator
					.destroy_buffer(vk::Buffer::from_raw(handle), &mut token)
					.unwrap();
			}
			allocator.close().unwrap();
		}
	}
}
