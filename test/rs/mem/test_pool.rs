//! Fixed-type pool contracts derived from pinned VMA CreatePool/BlockVector rules.
use super::*;
use crate::mem::{AllocationCreateInfo, PoolCreateInfo, PoolStats};

fn info() -> PoolCreateInfo {
	PoolCreateInfo {
		block_size: 1024,
		max_block_count: 2,
		..Default::default()
	}
}

fn configure() {
	STATE.with_borrow_mut(|s| s.pooled = true);
}

#[test]
fn minimum_blocks_are_preallocated_retained_reused_and_explicitly_destroyed() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic ordinary memory; explicit cleanup, no host/GPU accesses.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 2,
				..info()
			})
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 1024), (0, 1024)]
		);
		let empty = PoolStats {
			block_count: 2,
			block_bytes: 2048,
			free_bytes: 2048,
			largest_free_region: 1024,
			..Default::default()
		};
		assert_eq!(allocator.pool_statistics(&pool).unwrap(), empty);
		assert_eq!(allocator.close(), Err(AllocError::LivePools));
		allocator.allocation_limit = 2;
		let opts = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		clear();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert!(!events().contains(&"allocate"));
		assert_eq!(
			allocator.pool_statistics(&pool).unwrap().reserved_bytes,
			320
		);
		assert_eq!(
			allocator.pool_statistics(&pool).unwrap().allocation_bytes,
			257
		);
		assert_eq!(
			allocator.destroy_pool(&mut pool),
			Err(AllocError::LiveAllocations)
		);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap(), empty);
		clear();
		crate::mem::destroy_pool(&mut allocator, &mut pool).unwrap();
		assert_eq!(events(), ["free", "free"]);
		assert!(pool.is_destroyed());
		assert_eq!(
			allocator.pool_statistics(&pool),
			Err(AllocError::UnknownPool)
		);
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		allocator.close().unwrap();
	}
}

#[test]
fn fixed_blocks_enforce_maximum_extra_alignment_and_no_outside_fallback() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: each mock region is explicitly destroyed; pointer-free placement checks.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 1024,
				max_block_count: 1,
				min_allocation_alignment: 512,
				..Default::default()
			})
			.unwrap();
		let opts = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert_eq!((x.offset(), y.offset()), (0, 512));
		assert_eq!(x.handle(), y.handle());
		let before = allocator.statistics();
		clear();
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::OutOfSpace)
		));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(allocator.statistics(), before);
		allocator.destroy_buffer(a, &mut x).unwrap();
		allocator.destroy_buffer(b, &mut y).unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 1);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn minimum_block_failure_rolls_back_only_the_new_pool_and_keeps_error() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: mock limit forces partial acquisition rollback; no external native use.
	unsafe {
		let mut existing = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				..info()
			})
			.unwrap();
		let before = allocator.statistics();
		allocator.allocation_limit = 2;
		clear();
		assert!(matches!(
			allocator.create_pool(&PoolCreateInfo {
				min_block_count: 2,
				..info()
			}),
			Err(AllocError::AllocationLimit)
		));
		assert_eq!(events(), ["allocate", "free"]);
		assert_eq!(allocator.statistics(), before);
		assert_eq!(allocator.pool_statistics(&existing).unwrap().block_count, 1);
		allocator.destroy_pool(&mut existing).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn native_failure_during_minimum_acquisition_releases_prior_blocks_and_allows_retry() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|s| s.fail_allocate_at = Some(2));
	// SAFETY: the exact-PFN driver fails the second allocation; no object is published.
	unsafe {
		assert!(matches!(
			allocator.create_pool(&PoolCreateInfo {
				min_block_count: 2,
				..info()
			}),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_OUT_OF_DEVICE_MEMORY
			)))
		));
		assert_eq!(events(), ["allocate", "allocate", "free"]);
		assert!(allocator.pools.is_empty());
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		STATE.with_borrow_mut(|s| s.fail_allocate_at = None);
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 2,
				..info()
			})
			.unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 2);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn invalid_foreign_and_stale_pools_fail_before_resource_creation() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let mut other = fixture.allocator();
	// SAFETY: invalid policies/identities must produce no synthetic native effects.
	unsafe {
		for invalid in [
			PoolCreateInfo {
				memory_type_index: 32,
				..info()
			},
			PoolCreateInfo {
				min_block_count: 3,
				..info()
			},
			PoolCreateInfo {
				min_allocation_alignment: 3,
				..info()
			},
		] {
			clear();
			assert!(allocator.create_pool(&invalid).is_err());
			assert!(events().is_empty());
		}
		let mut pool = allocator.create_pool(&info()).unwrap();
		let opts = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		clear();
		assert!(matches!(
			other.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::WrongAllocator)
		));
		assert_eq!(
			other.destroy_pool(&mut pool),
			Err(AllocError::WrongAllocator)
		);
		assert!(events().is_empty());
		allocator.destroy_pool(&mut pool).unwrap();
		clear();
		assert!(matches!(
			allocator.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				}
			),
			Err(AllocError::UnknownPool)
		));
		assert!(events().is_empty());
		allocator.close().unwrap();
		other.close().unwrap();
	}
}

#[test]
fn ordinary_pools_and_distinct_custom_pools_never_share_native_blocks() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: three separately tracked mock storage domains; explicitly released.
	unsafe {
		let mut first = allocator.create_pool(&info()).unwrap();
		let mut second = allocator.create_pool(&info()).unwrap();
		let (a, mut x) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&first),
					..Default::default()
				},
			)
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&second),
					..Default::default()
				},
			)
			.unwrap();
		let (c, mut z) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					policy: AllocationPolicy::Pooled { block_size: 1024 },
					..Default::default()
				},
			)
			.unwrap();
		assert_ne!(x.handle(), y.handle());
		assert_ne!(x.handle(), z.handle());
		assert_ne!(y.handle(), z.handle());
		allocator.destroy_buffer(a, &mut x).unwrap();
		allocator.destroy_buffer(b, &mut y).unwrap();
		allocator.destroy_buffer(c, &mut z).unwrap();
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		allocator.destroy_pool(&mut first).unwrap();
		allocator.destroy_pool(&mut second).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn persistent_maps_balance_before_pool_destruction_and_empty_cache_is_bounded() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: mock pointers never dereferenced, no native host/GPU use remains at cleanup.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 512,
				..info()
			})
			.unwrap();
		let opts = AllocationCreateInfo {
			pool: Some(&pool),
			persistent_map: true,
			..Default::default()
		};
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert_ne!(x.handle(), y.handle());
		clear();
		allocator.destroy_buffer(a, &mut x).unwrap();
		allocator.destroy_buffer(b, &mut y).unwrap();
		assert_eq!(
			events(),
			["destroy_buffer", "unmap", "destroy_buffer", "unmap", "free"]
		);
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 1);
		let (c, mut z) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				},
			)
			.unwrap();
		assert_eq!(allocator.mapped_pointer(&z).unwrap(), None);
		allocator.destroy_buffer(c, &mut z).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		allocator.close().unwrap();
	}
}

#[test]
fn pool_type_is_fixed_required_properties_stay_mandatory_and_dedication_is_exact() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: independently injected requirements/type mismatches; new resource rollback only.
	unsafe {
		let mut pool = allocator.create_pool(&info()).unwrap();
		for options in [
			AllocationCreateInfo {
				memory_type_bits: 0,
				..Default::default()
			},
			AllocationCreateInfo {
				required_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
				..Default::default()
			},
		] {
			clear();
			assert!(matches!(
				allocator.create_buffer_with_options(
					&buffer_info(),
					&AllocationCreateInfo {
						pool: Some(&pool),
						..options
					}
				),
				Err(AllocError::NoCompatibleMemoryType)
			));
			assert!(!events().contains(&"allocate"));
			assert_eq!(events().last(), Some(&"destroy_buffer"));
		}
		STATE.with_borrow_mut(|s| s.required = true);
		assert!(matches!(
			allocator.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				}
			),
			Err(AllocError::UnsupportedCapability(_))
		));
		STATE.with_borrow_mut(|s| {
			s.required = false;
			s.preferred = true;
		});
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				},
			)
			.unwrap();
		assert!(STATE.with_borrow(|s| s.dedicated).is_null());
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn mixed_images_share_preallocated_blocks_and_release_to_the_minimum() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic page-separated resources; native cleanup is explicit.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				..info()
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		let (buffer, mut buffer_memory) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		let (linear, mut linear_memory) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::LINEAR), &options)
			.unwrap();
		assert_eq!(buffer_memory.handle(), linear_memory.handle());
		let reusable = buffer_memory.handle();
		allocator
			.destroy_buffer(buffer, &mut buffer_memory)
			.unwrap();
		let (optimal, mut optimal_memory) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		assert_eq!(optimal_memory.handle(), reusable);
		assert_eq!(optimal_memory.handle(), linear_memory.handle());
		allocator.destroy_image(linear, &mut linear_memory).unwrap();
		allocator
			.destroy_image(optimal, &mut optimal_memory)
			.unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 1);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn map_and_bind_failure_preserve_error_and_leave_only_the_pools_empty_cache() {
	for failure in ["map", "bind"] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		// SAFETY: failed creation publishes no resource; retained empty block belongs to pool.
		unsafe {
			let mut pool = allocator.create_pool(&info()).unwrap();
			STATE.with_borrow_mut(|s| s.fail = Some(failure));
			assert!(matches!(
				allocator.create_buffer_with_options(
					&buffer_info(),
					&AllocationCreateInfo {
						pool: Some(&pool),
						persistent_map: true,
						..Default::default()
					}
				),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			let stats = allocator.pool_statistics(&pool).unwrap();
			assert_eq!(
				(
					stats.allocation_count,
					stats.allocation_bytes,
					stats.reserved_bytes
				),
				(0, 0, 0)
			);
			assert_eq!(stats.block_count, 1);
			assert_eq!(allocator.statistics().mapped_native_count, 0);
			assert_eq!(
				events()
					.iter()
					.filter(|&&name| name == "destroy_buffer")
					.count(),
				1
			);
			STATE.with_borrow_mut(|s| s.fail = None);
			let (buffer, mut allocation) = allocator
				.create_buffer_with_options(
					&buffer_info(),
					&AllocationCreateInfo {
						pool: Some(&pool),
						..Default::default()
					},
				)
				.unwrap();
			assert_eq!(allocator.statistics().native_allocation_count, 1);
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn metadata_oom_rejects_pool_publication_before_native_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	clear();
	FAIL_NEXT_HOST_ALLOCATION.set(true);
	// SAFETY: test-only actual host allocation rejection; validated mock context.
	let result = unsafe { allocator.create_pool(&info()) };
	FAIL_NEXT_HOST_ALLOCATION.set(false);
	assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
	assert!(events().is_empty());
	assert!(allocator.pools.is_empty());
	assert_eq!(allocator.statistics(), AllocatorStats::default());
	allocator.close().unwrap();
}
