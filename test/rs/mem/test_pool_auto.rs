//! Pinned VMA automatic custom-pool sizing, dedication, ownership and rollback.
use super::*;
use crate::mem::{AllocationCreateInfo, PoolCreateInfo};

unsafe fn allocator(fixture: &Fixture, heap_size: u64) -> Allocator<'_> {
	let mut props = properties();
	props.memory_heaps[0].size = heap_size;
	STATE.with_borrow_mut(|s| s.pooled = true);
	// SAFETY: exact synthetic memory metadata, no actual driver storage.
	unsafe { Allocator::new(&fixture.device, props, 64) }.unwrap()
}

fn options(pool: &crate::Pool) -> AllocationCreateInfo<'_> {
	AllocationCreateInfo {
		pool: Some(pool),
		..Default::default()
	}
}

#[test]
fn default_pool_is_lazy_and_preferred_size_follows_exact_heap_boundary_and_rounding() {
	for (heap_size, preferred) in [
		(512 * 1024, 64 * 1024),
		(1024 * 1024 * 1024, 128 * 1024 * 1024),
		(1024 * 1024 * 1024 + 1, 256 * 1024 * 1024),
		(8193, 1024),
		(8455, 1056),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		// SAFETY: only synthetic minimum acquisition; no byte backing is allocated.
		unsafe {
			let mut allocator = allocator(&fixture, heap_size);
			let mut lazy = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
			assert_eq!(
				allocator.pools[&lazy.serial].preferred_block_size,
				preferred
			);
			assert!(events().is_empty());
			assert_eq!(
				allocator.pool_statistics(&lazy).unwrap(),
				crate::mem::PoolStats::default()
			);
			allocator.destroy_pool(&mut lazy).unwrap();
			let mut minimum = allocator
				.create_pool(&PoolCreateInfo {
					min_block_count: 1,
					max_block_count: 1,
					..Default::default()
				})
				.unwrap();
			assert_eq!(STATE.with_borrow(|s| s.allocated.clone()), [(0, preferred)]);
			allocator.destroy_pool(&mut minimum).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn blocks_start_at_one_eighth_grow_after_existing_maximum_and_reuse_actual_size() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: two synthetic pool blocks; all regions released before destruction.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		let mut resources = Vec::new();
		for _ in 0..26 {
			resources.push(
				allocator
					.create_buffer_with_options(&buffer_info(), &options(&pool))
					.unwrap(),
			);
		}
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 8192), (0, 16384)]
		);
		let first = &resources[0].1;
		let second = &resources[25].1;
		assert_eq!(first.block_size, 8192);
		assert_eq!(second.block_size, 16384);
		assert_ne!(first.handle(), second.handle());
		for (buffer, mut allocation) in resources {
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		}
		clear();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert!(!events().contains(&"allocate"));
		assert_eq!(allocation.block_size, 8192);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		allocator.close().unwrap();
	}
}

#[test]
fn minimum_blocks_are_full_size_and_partial_failure_rolls_back_only_new_pool() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: failed preallocation publishes no pool; successful retry explicitly cleaned up.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut existing = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		STATE.with_borrow_mut(|s| s.fail_allocate_at = Some(2));
		let info = PoolCreateInfo {
			min_block_count: 2,
			max_block_count: 2,
			..Default::default()
		};
		assert!(matches!(
			allocator.create_pool(&info),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_OUT_OF_DEVICE_MEMORY
			)))
		));
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 65536), (0, 65536)]
		);
		assert_eq!(events().iter().filter(|&&e| e == "free").count(), 1);
		assert_eq!(allocator.pools.len(), 1);
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		STATE.with_borrow_mut(|s| s.fail_allocate_at = None);
		let mut pool = allocator.create_pool(&info).unwrap();
		clear();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(allocation.block_size, 65536);
		assert!(!events().contains(&"allocate"));
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 2);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.destroy_pool(&mut existing).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn required_dedication_stays_owned_by_pool_for_buffers_images_and_checked_shutdown() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: exact required dedication is recorded by mock PFNs; no actual GPU uses.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				max_block_count: 1,
				..Default::default()
			})
			.unwrap();
		STATE.with_borrow_mut(|s| s.required = true);
		let (buffer, mut a) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(STATE.with_borrow(|s| s.dedicated), buffer);
		let (image, mut b) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &options(&pool))
			.unwrap();
		assert_eq!(STATE.with_borrow(|s| s.dedicated_image), image);
		assert!(allocator.blocks.is_empty());
		let stats = allocator.pool_statistics(&pool).unwrap();
		assert_eq!(stats.allocation_count, 2);
		assert_eq!(stats.block_count, 2);
		assert_eq!(stats.block_bytes, a.size() + b.size());
		assert_eq!(stats.free_bytes, 0);
		assert_eq!(
			allocator.destroy_pool(&mut pool),
			Err(AllocError::LiveAllocations)
		);
		assert_eq!(allocator.close(), Err(AllocError::LiveAllocations));
		allocator.destroy_buffer(buffer, &mut a).unwrap();
		allocator.destroy_image(image, &mut b).unwrap();
		assert_eq!(
			allocator.pool_statistics(&pool).unwrap(),
			crate::mem::PoolStats::default()
		);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn automatic_pool_can_dedicate_above_block_ceiling_without_type_or_identity_escape() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: native count limit checked independently from the pool block ceiling.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				max_block_count: 1,
				..Default::default()
			})
			.unwrap();
		STATE.with_borrow_mut(|s| s.requirements_size = Some(40000));
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 65536), (0, 40000)]
		);
		assert!(allocator.live[&allocation.serial].placement.is_none());
		assert_eq!(allocator.live[&allocation.serial].pool, Some(pool.serial));
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 2);
		assert_eq!(allocator.pool_statistics(&pool).unwrap().free_bytes, 65536);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		STATE.with_borrow_mut(|s| s.requirements_bits = Some(2));
		clear();
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(&pool)),
			Err(AllocError::NoCompatibleMemoryType)
		));
		assert!(!events().contains(&"allocate"));
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn native_oom_shrinks_only_within_bound_and_dedicated_fallback_is_pool_owned() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: bounded failure-injected native acquisitions, with explicit cleanup.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		STATE.with_borrow_mut(|s| {
			s.requirements_size = Some(5000);
			s.fail_above = Some(8192);
		});
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 16384), (0, 8192)]
		);
		assert_eq!(allocation.block_size, 8192);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		STATE.with_borrow_mut(|s| {
			s.allocated.clear();
			s.requirements_size = Some(257);
			s.fail_above = Some(512);
		});
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 8192), (0, 257)]
		);
		assert!(allocator.live[&allocation.serial].placement.is_none());
		assert_eq!(allocator.live[&allocation.serial].pool, Some(pool.serial));
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn high_native_count_prefers_existing_pool_space_but_required_dedication_stays_required() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: the one native block is already at the supplied synthetic count ceiling.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				max_block_count: 1,
				..Default::default()
			})
			.unwrap();
		allocator.allocation_limit = 1;
		STATE.with_borrow_mut(|s| {
			s.preferred = true;
			s.requirements_size = Some(40000);
		});
		clear();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(allocation.block_size, 65536);
		assert!(!events().contains(&"allocate"));
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		STATE.with_borrow_mut(|s| s.required = true);
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(&pool)),
			Err(AllocError::AllocationLimit)
		));
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn device_loss_and_binding_failures_do_not_trigger_storage_fallback() {
	for site in ["allocate", "bind", "map"] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		// SAFETY: every unpublished synthetic object is rolled back; no real pointer access.
		unsafe {
			let mut allocator = allocator(&fixture, 512 * 1024);
			let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
			STATE.with_borrow_mut(|s| s.fail = Some(site));
			let options = AllocationCreateInfo {
				persistent_map: true,
				..options(&pool)
			};
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer_info(), &options),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			assert_eq!(STATE.with_borrow(|s| s.allocated.len()), 1);
			assert_eq!(
				allocator.pool_statistics(&pool).unwrap().allocation_count,
				0
			);
			assert_eq!(
				events().iter().filter(|&&e| e == "destroy_buffer").count(),
				1
			);
			STATE.with_borrow_mut(|s| s.fail = None);
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn metadata_failure_after_requirements_rolls_back_without_native_storage_or_retry() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: the failing host allocation is the real native-block metadata reservation.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		STATE.with_borrow_mut(|s| s.fail = Some("host_after_requirements"));
		let result = allocator.create_buffer_with_options(&buffer_info(), &options(&pool));
		assert!(
			!FAIL_NEXT_HOST_ALLOCATION.replace(false),
			"actual host allocator was exercised"
		);
		assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(
			allocator.pool_statistics(&pool).unwrap(),
			crate::mem::PoolStats::default()
		);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn automatic_minimum_obeys_allocation_size_limit_while_lazy_growth_can_shrink() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: no driver block may exceed the explicit synthetic native size ceiling.
	unsafe {
		let mut allocator = allocator(&fixture, 512 * 1024);
		allocator.allocation_size_limit = Some(8192);
		assert!(matches!(
			allocator.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				..Default::default()
			}),
			Err(AllocError::AllocationTooLarge {
				requested: 65536,
				limit: 8192
			})
		));
		assert!(events().is_empty());
		let mut pool = allocator.create_pool(&PoolCreateInfo::default()).unwrap();
		STATE.with_borrow_mut(|s| s.requirements_size = Some(5000));
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(allocation.block_size, 8192);
		assert_eq!(STATE.with_borrow(|s| s.allocated.clone()), [(0, 8192)]);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		clear();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		assert_eq!(allocation.block_size, 8192);
		assert!(!events().contains(&"allocate"));
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}
