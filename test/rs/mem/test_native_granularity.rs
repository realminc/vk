//! Native admission, atom/page separation and partial-failure rollback.
use super::*;
use crate::mem::{AllocationAlgorithm, PoolCreateInfo};

#[test]
fn supplied_and_queried_limits_are_validated_before_native_allocation() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	for granularity in [0, 3, 255] {
		clear();
		// SAFETY: deliberately invalid scalar metadata; rejected before native effects.
		assert!(matches!(
			unsafe { Allocator::new_with_granularity(&fixture.device, properties(), 64, granularity) },
			Err(AllocError::InvalidProperties(_))
		));
		assert!(events().is_empty());
	}
	// SAFETY: exact mock physical-device metadata, no live native allocations.
	let mut queried = unsafe {
		Allocator::new_from_physical_device(
			&fixture.instance,
			vk::PhysicalDevice::from_raw(0x3000),
			&fixture.device,
		)
	}
	.unwrap();
	assert_eq!(queried.granularity.0, Some(64));
	queried.close().unwrap();
	for granularity in [0, 3] {
		STATE.with_borrow_mut(|state| state.granularity = Some(granularity));
		clear();
		// SAFETY: invalid queried limit is rejected before native allocation.
		assert!(matches!(
			unsafe {
				Allocator::new_from_physical_device(
					&fixture.instance,
					vk::PhysicalDevice::from_raw(0x3000),
					&fixture.device,
				)
			},
			Err(AllocError::InvalidProperties(_))
		));
		assert!(!events().contains(&"allocate"));
	}
}

#[test]
fn mixed_resources_use_one_block_with_exact_binding_and_cache_offsets() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| s.pooled = true);
		// SAFETY: synthetic physical metadata and synchronous PFNs, no GPU access.
		unsafe {
			let mut allocator =
				Allocator::new_with_granularity(&fixture.device, properties(), 64, 1024).unwrap();
			let mut pool = allocator
				.create_pool(&PoolCreateInfo {
					algorithm,
					block_size: 4096,
					min_block_count: 1,
					max_block_count: 1,
					..Default::default()
				})
				.unwrap();
			let opts = AllocationCreateInfo {
				pool: Some(&pool),
				persistent_map: true,
				..Default::default()
			};
			let (buffer, mut b) = allocator
				.create_buffer_with_options(&buffer_info(), &opts)
				.unwrap();
			let (linear, mut l) = allocator
				.create_image_with_options(&image::info(vk::ImageTiling::LINEAR), &opts)
				.unwrap();
			let (optimal, mut o) = allocator
				.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &opts)
				.unwrap();
			let (second, mut s) = allocator
				.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &opts)
				.unwrap();
			assert_eq!(
				[b.offset(), l.offset(), o.offset(), s.offset()],
				[0, 320, 1024, 1344]
			);
			assert_eq!([l.handle(), o.handle(), s.handle()], [b.handle(); 3]);
			assert_eq!(allocator.statistics().native_allocation_count, 1);
			assert_eq!(allocator.statistics().mapped_native_count, 1);
			let stats = allocator.pool_statistics(&pool).unwrap();
			assert_eq!(
				(
					stats.allocation_bytes,
					stats.reserved_bytes,
					stats.free_bytes
				),
				(1028, 1280, 2816)
			);
			allocator.flush(&o, 256, 1).unwrap();
			assert_eq!(
				STATE.with_borrow(|s| s.ranges.last().copied()),
				Some((1280, 64))
			);
			for (resource, allocation) in [(optimal, &mut o), (second, &mut s), (linear, &mut l)] {
				allocator.destroy_image(resource, allocation).unwrap();
			}
			allocator.destroy_buffer(buffer, &mut b).unwrap();
			assert_eq!(allocator.pool_statistics(&pool).unwrap().free_bytes, 4096);
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn mixed_image_failure_releases_page_constraints_and_only_its_own_mapping_hold() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		for failure in ["create_image", "map", "bind_image"] {
			let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
			STATE.with_borrow_mut(|s| s.pooled = true);
			// SAFETY: injected mock errors, no outstanding native uses or pointer dereferences.
			unsafe {
				let mut allocator =
					Allocator::new_with_granularity(&fixture.device, properties(), 64, 1024).unwrap();
				let mut pool = allocator
					.create_pool(&PoolCreateInfo {
						algorithm,
						block_size: 4096,
						min_block_count: 1,
						max_block_count: 1,
						..Default::default()
					})
					.unwrap();
				let opts = AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				};
				let (buffer, mut b) = allocator
					.create_buffer_with_options(&buffer_info(), &opts)
					.unwrap();
				let before = allocator.statistics();
				STATE.with_borrow_mut(|s| s.fail = Some(failure));
				let failed = allocator.create_image_with_options(
					&image::info(vk::ImageTiling::OPTIMAL),
					&AllocationCreateInfo {
						persistent_map: true,
						..opts
					},
				);
				assert!(matches!(
					failed,
					Err(AllocError::Call(CallError::Vulkan(
						vk::Result::ERROR_DEVICE_LOST
					)))
				));
				assert_eq!(allocator.statistics(), before);
				STATE.with_borrow_mut(|s| s.fail = None);
				let (next, mut n) = allocator
					.create_buffer_with_options(&buffer_info(), &opts)
					.unwrap();
				assert_eq!(n.offset(), 320);
				assert_eq!(n.handle(), b.handle());
				let (optimal, mut o) = allocator
					.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &opts)
					.unwrap();
				assert_eq!(o.offset(), 1024);
				allocator.destroy_image(optimal, &mut o).unwrap();
				allocator.destroy_buffer(next, &mut n).unwrap();
				allocator.destroy_buffer(buffer, &mut b).unwrap();
				allocator.destroy_pool(&mut pool).unwrap();
				allocator.close().unwrap();
			}
		}
	}
}

#[test]
fn manual_unknown_limit_is_conservative_and_does_not_invent_device_properties() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	// SAFETY: unknown-granularity route is conservative; mock children explicitly destroyed.
	unsafe {
		let mut allocator = Allocator::new(&fixture.device, properties(), 64).unwrap();
		let opts = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 4096 },
			..Default::default()
		};
		let (buffer, mut b) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		let (linear, mut l) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::LINEAR), &opts)
			.unwrap();
		let (optimal, mut o) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &opts)
			.unwrap();
		assert_eq!(b.handle(), l.handle());
		assert_ne!(b.handle(), o.handle());
		allocator.destroy_image(optimal, &mut o).unwrap();
		allocator.destroy_image(linear, &mut l).unwrap();
		allocator.destroy_buffer(buffer, &mut b).unwrap();
		allocator.close().unwrap();
	}
}
