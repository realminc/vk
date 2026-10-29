//! BDA admission and native flag chains against independent exact-PFN records.
use super::*;
use crate::mem::{AllocationCreateInfo, AllocationPolicy, PoolCreateInfo};

fn buffer() -> vk::BufferCreateInfo<'static> {
	vk::BufferCreateInfo {
		usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT
			| vk::BufferUsageFlagBits::SHADER_DEVICE_ADDRESS_BIT,
		..buffer_info()
	}
}

fn flags() -> Vec<u32> {
	STATE.with_borrow(|s| s.allocation_flags.clone())
}

fn address_bit() -> u32 {
	vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_BIT.as_raw()
}

unsafe fn fixture(version: u32, chain: *const c_void) -> Fixture {
	// SAFETY: caller supplies a live typed feature record for synchronous capture.
	unsafe { Fixture::new_with_feature_chain(version, &[], &[], true, None, chain) }
}

#[test]
fn core_unified_and_standalone_records_admit_exact_enabled_feature() {
	let unified = vk::PhysicalDeviceVulkan12Features {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	let standalone = vk::PhysicalDeviceBufferDeviceAddressFeatures {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	for chain in [
		std::ptr::from_ref(&unified).cast(),
		std::ptr::from_ref(&standalone).cast(),
	] {
		// SAFETY: synchronous capture of the live typed record and synthetic resources.
		unsafe {
			let fixture = fixture(vk::API_VERSION_1_2, chain);
			let mut allocator = fixture.allocator();
			let (buffer, mut allocation) = allocator
				.create_buffer_with_options(&buffer(), &AllocationCreateInfo::default())
				.unwrap();
			assert_eq!(flags(), [address_bit()]);
			assert_eq!(STATE.with_borrow(|s| s.dedicated), buffer);
			assert_eq!(
				events(),
				["create", "requirements_core", "allocate", "bind"]
			);
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn khr_admission_requires_registry_dependencies_and_standalone_enabled_bit() {
	let feature = vk::PhysicalDeviceBufferDeviceAddressFeatures {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	for (version, device_extensions, instance_extensions, admitted) in [
		(
			vk::API_VERSION_1_1,
			vec![c"VK_KHR_buffer_device_address"],
			vec![],
			true,
		),
		(
			vk::API_VERSION_1_0,
			vec![c"VK_KHR_buffer_device_address", c"VK_KHR_device_group"],
			vec![c"VK_KHR_get_physical_device_properties2"],
			true,
		),
		(
			vk::API_VERSION_1_0,
			vec![c"VK_KHR_buffer_device_address"],
			vec![c"VK_KHR_get_physical_device_properties2"],
			false,
		),
		(
			vk::API_VERSION_1_0,
			vec![c"VK_KHR_buffer_device_address", c"VK_KHR_device_group"],
			vec![],
			false,
		),
		(vk::API_VERSION_1_1, vec![], vec![], false),
	] {
		// SAFETY: matching synthetic metadata; feature pointers remain live during capture.
		unsafe {
			let fixture = Fixture::new_with_feature_chain(
				version,
				&device_extensions,
				&instance_extensions,
				true,
				None,
				std::ptr::from_ref(&feature).cast(),
			);
			let mut allocator = fixture.allocator();
			let result =
				allocator.create_buffer_with_options(&buffer(), &AllocationCreateInfo::default());
			if admitted {
				let (buffer, mut allocation) = result.unwrap();
				assert_eq!(flags(), [address_bit()]);
				assert!(events().contains(&if version == vk::API_VERSION_1_0 {
					"requirements_legacy"
				} else {
					"requirements_core"
				}));
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			} else {
				assert!(matches!(result, Err(AllocError::UnsupportedCapability(_))));
				assert!(events().is_empty());
			}
			allocator.close().unwrap();
		}
	}
}

#[test]
fn absent_false_ext_and_unadmitted_unified_features_never_enable_address_storage() {
	let disabled = vk::PhysicalDeviceBufferDeviceAddressFeatures::default();
	let ext = vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	let unified = vk::PhysicalDeviceVulkan12Features {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	for (version, chain, metadata) in [
		(vk::API_VERSION_1_2, std::ptr::null(), true),
		(
			vk::API_VERSION_1_2,
			std::ptr::from_ref(&disabled).cast(),
			true,
		),
		(vk::API_VERSION_1_2, std::ptr::from_ref(&ext).cast(), true),
		(
			vk::API_VERSION_1_1,
			std::ptr::from_ref(&unified).cast(),
			true,
		),
		(
			vk::API_VERSION_1_2,
			std::ptr::from_ref(&unified).cast(),
			false,
		),
	] {
		// SAFETY: exact mock PFNs and complete live typed feature records only.
		unsafe {
			let fixture = Fixture::new_with_feature_chain(
				version,
				&[c"VK_KHR_buffer_device_address"],
				&[],
				metadata,
				None,
				chain,
			);
			let mut allocator = fixture.allocator();
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer(), &AllocationCreateInfo::default()),
				Err(AllocError::UnsupportedCapability(_))
			));
			assert!(events().is_empty());
			let mut raw = allocator.allocate(&request()).unwrap();
			assert_eq!(flags(), [0]);
			allocator.free(&mut raw).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn ordinary_then_address_buffer_reuses_flagged_default_and_preallocated_custom_blocks() {
	let feature = vk::PhysicalDeviceVulkan12Features {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	for custom in [false, true] {
		// SAFETY: synthetic metadata; every region and pool explicitly released.
		unsafe {
			let fixture = fixture(vk::API_VERSION_1_2, std::ptr::from_ref(&feature).cast());
			STATE.with_borrow_mut(|s| s.pooled = true);
			let mut allocator = fixture.allocator();
			let mut pool = custom.then(|| {
				allocator
					.create_pool(&PoolCreateInfo {
						block_size: 1024,
						min_block_count: 1,
						max_block_count: 1,
						..Default::default()
					})
					.unwrap()
			});
			let options = AllocationCreateInfo {
				pool: pool.as_ref(),
				policy: AllocationPolicy::Pooled { block_size: 1024 },
				..Default::default()
			};
			let (ordinary, mut first) = allocator
				.create_buffer_with_options(&buffer_info(), &options)
				.unwrap();
			let (address, mut second) = allocator
				.create_buffer_with_options(&buffer(), &options)
				.unwrap();
			assert_eq!(first.handle(), second.handle());
			assert!(second.offset() > 0);
			assert_eq!(flags(), [address_bit()]);
			assert!(STATE.with_borrow(|s| s.dedicated.is_null()));
			allocator.destroy_buffer(address, &mut second).unwrap();
			allocator.destroy_buffer(ordinary, &mut first).unwrap();
			if let Some(pool) = pool.as_mut() {
				allocator.destroy_pool(pool).unwrap();
			}
			assert_eq!(allocator.statistics(), AllocatorStats::default());
			allocator.close().unwrap();
		}
	}
}

#[test]
fn dedicated_address_chain_coexists_with_native_dedication_and_ordinary_images_omit_flags() {
	let feature = vk::PhysicalDeviceBufferDeviceAddressFeatures {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	// SAFETY: exact synthetic resource requirements and live metadata, explicit cleanup.
	unsafe {
		let fixture = fixture(vk::API_VERSION_1_2, std::ptr::from_ref(&feature).cast());
		let mut allocator = fixture.allocator();
		STATE.with_borrow_mut(|s| s.required = true);
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 1024 },
			..Default::default()
		};
		let (address, mut allocation) = allocator
			.create_buffer_with_options(&buffer(), &options)
			.unwrap();
		assert_eq!(flags(), [address_bit()]);
		assert_eq!(STATE.with_borrow(|s| s.dedicated), address);
		allocator.destroy_buffer(address, &mut allocation).unwrap();
		let (ordinary, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		assert_eq!(flags(), [address_bit(), 0]);
		allocator.destroy_buffer(ordinary, &mut allocation).unwrap();
		let (image, mut allocation) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		assert_eq!(flags(), [address_bit(), 0, 0]);
		assert_eq!(STATE.with_borrow(|s| s.dedicated_image), image);
		allocator.destroy_image(image, &mut allocation).unwrap();
		let mut raw = allocator.allocate(&request()).unwrap();
		assert_eq!(flags(), [address_bit(), 0, 0, address_bit()]);
		allocator.free(&mut raw).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn address_allocation_map_and_bind_failures_preserve_error_and_retry_cleanly() {
	let feature = vk::PhysicalDeviceVulkan12Features {
		buffer_device_address: vk::TRUE,
		..Default::default()
	};
	for site in ["allocate", "map", "bind"] {
		// SAFETY: failure-injected mock objects are unpublished and rolled back synchronously.
		unsafe {
			let fixture = fixture(vk::API_VERSION_1_2, std::ptr::from_ref(&feature).cast());
			let mut allocator = fixture.allocator();
			let options = AllocationCreateInfo {
				persistent_map: true,
				..Default::default()
			};
			STATE.with_borrow_mut(|s| s.fail = Some(site));
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer(), &options),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			assert_eq!(allocator.statistics(), AllocatorStats::default());
			assert_eq!(
				events().iter().filter(|&&e| e == "destroy_buffer").count(),
				1
			);
			assert_eq!(
				events().iter().filter(|&&e| e == "free").count(),
				usize::from(site != "allocate")
			);
			STATE.with_borrow_mut(|s| s.fail = None);
			let (buffer, mut allocation) = allocator
				.create_buffer_with_options(&buffer(), &options)
				.unwrap();
			assert_eq!(flags(), [address_bit(), address_bit()]);
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn capture_replay_remains_rejected_before_native_creation() {
	let feature = vk::PhysicalDeviceVulkan12Features {
		buffer_device_address: vk::TRUE,
		buffer_device_address_capture_replay: vk::TRUE,
		..Default::default()
	};
	// SAFETY: typed feature metadata only; invalid managed requests have no effects.
	unsafe {
		let fixture = fixture(vk::API_VERSION_1_2, std::ptr::from_ref(&feature).cast());
		let mut allocator = fixture.allocator();
		let info = vk::BufferCreateInfo {
			flags: vk::BufferCreateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
			..buffer()
		};
		assert!(matches!(
			allocator.create_buffer_with_options(&info, &AllocationCreateInfo::default()),
			Err(AllocError::InvalidRequest(_))
		));
		let address = vk::BufferOpaqueCaptureAddressCreateInfo::default();
		let info = vk::BufferCreateInfo {
			p_next: std::ptr::from_ref(&address).cast(),
			..buffer()
		};
		assert!(matches!(
			allocator.create_buffer_with_options(&info, &AllocationCreateInfo::default()),
			Err(AllocError::UnsupportedCapability(_))
		));
		assert!(events().is_empty());
		allocator.close().unwrap();
	}
}
