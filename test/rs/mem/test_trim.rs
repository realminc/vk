//! VMA-derived empty-cache retention and explicit pressure maintenance.
use super::*;
use crate::mem::{AllocationAlgorithm, PoolCreateInfo, TrimStats};

fn info(algorithm: AllocationAlgorithm, minimum: usize) -> PoolCreateInfo {
	PoolCreateInfo {
		algorithm,
		block_size: 1024,
		min_block_count: minimum,
		max_block_count: 3,
		..Default::default()
	}
}

fn options(pool: &crate::Pool) -> AllocationCreateInfo<'_> {
	AllocationCreateInfo {
		pool: Some(pool),
		..Default::default()
	}
}

#[test]
fn explicit_trim_releases_only_surplus_empty_blocks_and_preserves_pool_identity() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| {
			s.pooled = true;
			s.requirements_size = Some(768);
		});
		let mut allocator = fixture.allocator();
		// SAFETY: synthetic live allocations/maps; trim must not disturb either.
		unsafe {
			let mut pool = allocator.create_pool(&info(algorithm, 0)).unwrap();
			let (a, mut x) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			let (b, mut y) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			allocator.map(&y).unwrap();
			allocator.destroy_buffer(a, &mut x).unwrap();
			clear();
			assert_eq!(
				allocator.trim().unwrap(),
				TrimStats {
					block_count: 1,
					block_bytes: 1024
				}
			);
			assert_eq!(events(), ["free"]);
			assert!(!pool.is_destroyed());
			assert_eq!(allocator.statistics().mapped_native_count, 1);
			assert!(allocator.mapped_pointer(&y).unwrap().is_some());
			assert_eq!(allocator.trim().unwrap(), TrimStats::default());
			allocator.unmap(&y).unwrap();
			allocator.destroy_buffer(b, &mut y).unwrap();
			assert_eq!(crate::mem::trim(&mut allocator).unwrap().block_bytes, 1024);
			assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 0);
			let (c, mut z) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			assert_eq!(z.offset(), 0);
			allocator.destroy_buffer(c, &mut z).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
			assert_eq!(allocator.trim(), Err(AllocError::Closed));
		}
	}
}

#[test]
fn local_pressure_drops_the_last_spare_but_never_the_minimum() {
	for (minimum, algorithm) in [
		(0, AllocationAlgorithm::Tlsf),
		(1, AllocationAlgorithm::Tlsf),
		(0, AllocationAlgorithm::Linear),
		(1, AllocationAlgorithm::Linear),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| s.pooled = true);
		let mut allocator = fixture.allocator();
		allocator.set_heap_limit(0, 1024).unwrap();
		// SAFETY: exact mock resources explicitly destroyed; no driver budget is queried.
		unsafe {
			let mut pool = allocator.create_pool(&info(algorithm, minimum)).unwrap();
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			clear();
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			assert_eq!(
				events(),
				if minimum == 0 {
					vec!["destroy_buffer", "free"]
				} else {
					vec!["destroy_buffer"]
				}
			);
			assert_eq!(
				allocator.pool_statistics(&pool).unwrap().block_count,
				minimum
			);
			assert_eq!(allocator.trim().unwrap(), TrimStats::default());
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn partially_freeing_a_live_block_discards_a_surplus_empty_spare_like_vma() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: independent synthetic pairs; the last live region remains valid throughout.
	unsafe {
		let mut pool = allocator
			.create_pool(&info(AllocationAlgorithm::Tlsf, 0))
			.unwrap();
		let mut live = Vec::new();
		for _ in 0..4 {
			live.push(
				allocator
					.create_buffer_with_options(&buffer_info(), &options(&pool))
					.unwrap(),
			);
		}
		assert_ne!(live[0].1.handle(), live[3].1.handle());
		let (buffer, mut allocation) = live.pop().unwrap();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 2);
		let (buffer, mut allocation) = live.pop().unwrap();
		clear();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		assert_eq!(events(), ["destroy_buffer", "free"]);
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 1);
		for (buffer, mut allocation) in live {
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn queried_pressure_is_explicit_route_exact_and_not_cached_into_free() {
	for khr in [false, true] {
		let fixture = Fixture::new_with_instance_extensions(
			if khr {
				vk::API_VERSION_1_0
			} else {
				vk::API_VERSION_1_1
			},
			&[c"VK_EXT_memory_budget"],
			if khr {
				&[c"VK_KHR_get_physical_device_properties2"]
			} else {
				&[]
			},
			true,
			None,
		);
		STATE.with_borrow_mut(|s| {
			s.pooled = true;
			s.budget = 1024;
			s.usage = 1024;
		});
		let mut allocator = fixture.allocator();
		// SAFETY: exact owned mock parent metadata and physical-device identity.
		unsafe {
			let mut pool = allocator
				.create_pool(&info(AllocationAlgorithm::Tlsf, 0))
				.unwrap();
			for _ in 0..2 {
				let (buffer, mut token) = allocator
					.create_buffer_with_options(&buffer_info(), &options(&pool))
					.unwrap();
				allocator.destroy_buffer(buffer, &mut token).unwrap();
				assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 1);
				clear();
				assert_eq!(
					allocator
						.trim_with_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
						.unwrap(),
					TrimStats {
						block_count: 1,
						block_bytes: 1024
					}
				);
				assert_eq!(
					events(),
					[if khr { "budget_khr" } else { "budget_core" }, "free"]
				);
			}
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn pressure_stops_after_projected_relief_and_always_preserves_each_pool_floor() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[c"VK_EXT_memory_budget"], true, None);
	STATE.with_borrow_mut(|s| {
		s.pooled = true;
		s.budget = 2048;
		s.usage = 2048;
	});
	let mut allocator = fixture.allocator();
	// SAFETY: separate empty mock pool caches, no native resource is live.
	unsafe {
		let mut pools = Vec::new();
		for _ in 0..3 {
			let pool = allocator
				.create_pool(&info(AllocationAlgorithm::Tlsf, 0))
				.unwrap();
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			pools.push(pool);
		}
		clear();
		assert_eq!(
			allocator
				.trim_with_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
				.unwrap()
				.block_count,
			1
		);
		assert_eq!(allocator.statistics().native_bytes, 2048);
		assert_eq!(events(), ["budget_core", "free"]);
		// With no pressure even empty surplus storage stays cached.
		STATE.with_borrow_mut(|s| {
			s.budget = 4096;
			s.usage = 0;
		});
		assert_eq!(
			allocator
				.trim_with_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
				.unwrap(),
			TrimStats::default()
		);
		assert_eq!(allocator.trim().unwrap().block_count, 2);
		for pool in &mut pools {
			allocator.destroy_pool(pool).unwrap();
		}
		allocator.close().unwrap();
	}
}

#[test]
fn invalid_budget_context_and_missing_query_pfn_free_nothing() {
	for blocked in [
		None,
		Some(b"vkGetPhysicalDeviceMemoryProperties2".as_slice()),
	] {
		let fixture = Fixture::new(
			vk::API_VERSION_1_1,
			&[c"VK_EXT_memory_budget"],
			true,
			blocked,
		);
		STATE.with_borrow_mut(|s| s.pooled = true);
		let mut allocator = fixture.allocator();
		// SAFETY: empty mock pool with no resources; malformed queries cannot free it.
		unsafe {
			let mut pool = allocator
				.create_pool(&info(AllocationAlgorithm::Tlsf, 1))
				.unwrap();
			let before = allocator.statistics();
			STATE.with_borrow_mut(|s| s.budget = 0);
			clear();
			assert!(
				allocator
					.trim_with_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
					.is_err()
			);
			assert_eq!(allocator.statistics(), before);
			assert!(!events().contains(&"free"));
			clear();
			assert!(
				allocator
					.trim_with_budget(&fixture.instance, vk::PhysicalDevice::NULL)
					.is_err()
			);
			assert!(events().is_empty());
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn trimming_does_not_allocate_host_metadata_or_acquire_a_mapping() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: only an empty synthetic native cache remains; mock event storage pre-reserved.
	unsafe {
		let mut pool = allocator
			.create_pool(&info(AllocationAlgorithm::Tlsf, 0))
			.unwrap();
		let (buffer, mut token) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool))
			.unwrap();
		allocator.destroy_buffer(buffer, &mut token).unwrap();
		clear();
		STATE.with_borrow_mut(|s| s.events.reserve(8));
		FAIL_NEXT_HOST_ALLOCATION.set(true);
		let result = allocator.trim();
		assert!(
			FAIL_NEXT_HOST_ALLOCATION.replace(false),
			"trim attempted host allocation"
		);
		assert_eq!(
			result.unwrap(),
			TrimStats {
				block_count: 1,
				block_bytes: 1024
			}
		);
		assert_eq!(events(), ["free"]);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn heap_pressure_does_not_reclaim_a_different_heaps_cache() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| {
		s.pooled = true;
		s.requirements_bits = Some(3);
	});
	let mut p = properties();
	p.memory_heap_count = 2;
	p.memory_type_count = 2;
	p.memory_heaps[1].size = 1 << 30;
	p.memory_types[1] = p.memory_types[0];
	p.memory_types[1].heap_index = 1;
	// SAFETY: synthetic two-heap properties; every resource completed before releasing its region.
	unsafe {
		let mut allocator = Allocator::new_with_granularity(&fixture.device, p, 64, 64).unwrap();
		let mut pools = Vec::new();
		for index in 0..2 {
			let pool = allocator
				.create_pool(&PoolCreateInfo {
					memory_type_index: index,
					..info(AllocationAlgorithm::Tlsf, 0)
				})
				.unwrap();
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool))
				.unwrap();
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			pools.push(pool);
		}
		let mut snapshot = allocator.budget_snapshot();
		snapshot.heaps[0].budget = 1024;
		snapshot.heaps[1].budget = 2048;
		clear();
		assert_eq!(
			super::super::pool::trim_blocks(&mut allocator, &mut snapshot, false).block_bytes,
			1024
		);
		assert_eq!(allocator.pool_statistics(&pools[0]).unwrap().block_count, 0);
		assert_eq!(allocator.pool_statistics(&pools[1]).unwrap().block_count, 1);
		assert_eq!(allocator.budget_snapshot().heaps[1].native_bytes, 1024);
		assert_eq!(events(), ["free"]);
		assert_eq!(allocator.trim().unwrap().block_count, 1);
		for pool in &mut pools {
			allocator.destroy_pool(pool).unwrap();
		}
		allocator.close().unwrap();
	}
}
