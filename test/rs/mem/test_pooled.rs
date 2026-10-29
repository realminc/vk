//! Native policy proofs use exact cached mock PFNs; independent byte/range checks.
use super::*;

#[test]
fn adaptive_blocks_shrink_on_oom_and_reuse_smaller_blocks_before_growing() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	STATE.with_borrow_mut(|s| s.fail_above = Some(1024));
	let mut allocator = fixture.allocator();
	let opts = AllocationCreateInfo {
		policy: AllocationPolicy::Adaptive {
			block_size: 4096,
			min_block_size: 512,
		},
		..Default::default()
	};
	// SAFETY: exact mock resource pairs, no host/GPU use and explicit destruction.
	unsafe {
		let (x, mut a) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 4096), (0, 2048), (0, 1024)]
		);
		let (y, mut b) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert_eq!(a.handle(), b.handle());
		assert_eq!(STATE.with_borrow(|s| s.allocated.len()), 3);
		allocator.destroy_buffer(x, &mut a).unwrap();
		allocator.destroy_buffer(y, &mut b).unwrap();
	}
}

#[test]
fn adaptive_fallback_keeps_exact_dedication_and_does_not_retry_device_loss() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	STATE.with_borrow_mut(|s| s.fail_above = Some(300));
	let mut allocator = fixture.allocator();
	let opts = AllocationCreateInfo {
		policy: AllocationPolicy::Adaptive {
			block_size: 1024,
			min_block_size: 512,
		},
		..Default::default()
	};
	// SAFETY: only synthetic unpublished native resources and matching cleanup.
	unsafe {
		let (buffer, mut a) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.allocated.clone()),
			[(0, 1024), (0, 512), (0, 257)]
		);
		assert_eq!(STATE.with_borrow(|s| s.dedicated), buffer);
		assert_eq!(allocator.statistics().pooled_block_count, 0);
		allocator.destroy_buffer(buffer, &mut a).unwrap();
		clear();
		STATE.with_borrow_mut(|s| {
			s.fail = Some("allocate");
			s.fail_above = None;
		});
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_DEVICE_LOST
			)))
		));
		assert_eq!(
			events()
				.iter()
				.filter(|&&event| event == "allocate")
				.count(),
			1
		);
		assert_eq!(allocator.live_count(), 0);
	}
}
use crate::mem::{AllocationCreateInfo, AllocationPolicy, AllocatorStats, PlacementStrategy};

fn options(persistent_map: bool) -> AllocationCreateInfo<'static> {
	AllocationCreateInfo {
		required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
		policy: AllocationPolicy::Pooled { block_size: 4096 },
		persistent_map,
		strategy: PlacementStrategy::MinOffset,
		..Default::default()
	}
}

fn configure() {
	STATE.with_borrow_mut(|s| {
		s.pooled = true;
		s.events.reserve(1024);
		s.allocated.reserve(1024);
		s.bindings.reserve(1024);
		s.ranges.reserve(1024);
	});
}

#[test]
fn regions_share_one_native_block_and_isolate_noncoherent_atoms() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	allocator.allocation_limit = 1;
	// SAFETY: synthetic native objects only; explicit resource/placement cleanup.
	unsafe {
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		let (b, mut y) =
			create_buffer_with_options(&mut allocator, &buffer_info(), &options(false)).unwrap();
		assert_eq!(x.handle(), y.handle());
		assert_eq!(
			(x.offset(), y.offset(), x.size(), y.size()),
			(0, 320, 257, 257)
		);
		assert_eq!((x.reserved_size(), y.reserved_size()), (320, 320));
		assert_eq!(
			allocator.statistics(),
			AllocatorStats {
				allocation_count: 2,
				native_allocation_count: 1,
				pooled_block_count: 1,
				allocation_bytes: 514,
				reserved_bytes: 640,
				native_bytes: 4096,
				mapped_native_count: 0
			}
		);
		allocator.map(&y).unwrap();
		allocator.flush(&y, 255, 2).unwrap();
		allocator.invalidate(&y, 65, 2).unwrap();
		assert_eq!(
			STATE.with_borrow(|s| s.ranges.clone()),
			[(512, 128), (384, 64)]
		);
		// No cache operation may enter another region's private atom extent.
		for (offset, size) in STATE.with_borrow(|s| s.ranges.clone()) {
			assert!(offset >= y.offset() && offset + size <= y.offset() + y.reserved_size());
		}
		allocator.unmap(&y).unwrap();
		clear();
		allocator.destroy_buffer(a, &mut x).unwrap();
		assert_eq!(events(), ["destroy_buffer"]);
		assert_eq!(allocator.statistics().native_bytes, 4096);
		let (c, mut z) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		assert_eq!(z.offset(), 0);
		allocator.destroy_buffer(b, &mut y).unwrap();
		allocator.destroy_buffer(c, &mut z).unwrap();
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		assert_eq!(events().iter().filter(|&&e| e == "free").count(), 1);
		allocator.close().unwrap();
	}
}

#[test]
fn explicit_and_persistent_holds_share_mapping_without_granting_neighbor_access() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic pointers are only compared, never dereferenced.
	unsafe {
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options(true))
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		assert_eq!(
			allocator.mapped_pointer(&x).unwrap().unwrap().addr(),
			0x4000
		);
		assert_eq!(mapped_pointer(&allocator, &y).unwrap(), None);
		assert_eq!(allocator.flush(&y, 0, 1), Err(AllocError::NotMapped));
		assert_eq!(allocator.unmap(&x), Err(AllocError::NotMapped));
		let p = allocator.map(&y).unwrap();
		assert_eq!(p.addr(), 0x4000 + 320);
		assert_eq!(allocator.map(&y).unwrap(), p);
		assert_eq!(allocator.allocation_info(&y).unwrap().map_count, 2);
		assert_eq!(allocator.free(&mut y), Err(AllocError::AlreadyMapped));
		allocator.map(&x).unwrap();
		allocator.unmap(&x).unwrap();
		assert_eq!(events().iter().filter(|&&e| e == "map").count(), 1);
		assert_eq!(events().iter().filter(|&&e| e == "unmap").count(), 0);
		clear();
		allocator.destroy_buffer(a, &mut x).unwrap();
		assert_eq!(events(), ["destroy_buffer"]);
		assert_eq!(allocator.mapped_pointer(&y).unwrap(), Some(p));
		allocator.unmap(&y).unwrap();
		assert_eq!(allocator.mapped_pointer(&y).unwrap(), Some(p));
		assert_eq!(events(), ["destroy_buffer"]);
		allocator.unmap(&y).unwrap();
		assert_eq!(events(), ["destroy_buffer", "unmap"]);
		allocator.destroy_buffer(b, &mut y).unwrap();
		assert_eq!(allocator.statistics(), AllocatorStats::default());
	}
}

#[test]
fn dedication_requirements_and_preferences_override_pooling_on_exact_routes() {
	for (version, extensions, route) in [
		(vk::API_VERSION_1_1, vec![], "requirements_core"),
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_get_memory_requirements2",
				c"VK_KHR_dedicated_allocation",
			],
			"requirements_khr",
		),
	] {
		for required in [false, true] {
			let fixture = Fixture::new(version, &extensions, true, None);
			configure();
			STATE.with_borrow_mut(|s| {
				s.required = required;
				s.preferred = !required;
			});
			let mut allocator = fixture.allocator();
			// SAFETY: mock requirements and exact chain checks, no driver.
			unsafe {
				let (buffer, mut allocation) = allocator
					.create_buffer_with_options(&buffer_info(), &options(true))
					.unwrap();
				assert!(events().contains(&route));
				assert_eq!(STATE.with_borrow(|s| s.dedicated), buffer);
				assert_eq!(allocator.statistics().pooled_block_count, 0);
				assert_eq!(allocation.reserved_size(), 257);
				assert!(allocator.mapped_pointer(&allocation).unwrap().is_some());
				clear();
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
				assert_eq!(events(), ["destroy_buffer", "unmap", "free"]);
			}
		}
	}
}

#[test]
fn only_native_oom_retries_other_compatible_types_and_never_relaxes_required_flags() {
	for loss in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut properties = properties();
		properties.memory_type_count = 3;
		properties.memory_types[1] = properties.memory_types[0];
		properties.memory_types[2] = properties.memory_types[0];
		properties.memory_types[2].property_flags = vk::MemoryPropertyFlags::empty();
		// SAFETY: matching mock memory properties.
		let mut allocator = unsafe { Allocator::new(&fixture.device, properties, 64) }.unwrap();
		STATE.with_borrow_mut(|s| {
			s.requirements_bits = Some(7);
			if loss {
				s.fail = Some("allocate");
			} else {
				s.fail_type = Some(0);
			}
		});
		// SAFETY: no real objects; mock allocs emit native OOM or loss.
		unsafe {
			let result = allocator.create_buffer_with_options(&buffer_info(), &options(false));
			if loss {
				assert!(matches!(
					result,
					Err(AllocError::Call(CallError::Vulkan(
						vk::Result::ERROR_DEVICE_LOST
					)))
				));
				assert_eq!(STATE.with_borrow(|s| s.allocated.clone()), [(0, 4096)]);
			} else {
				let (buffer, mut allocation) = result.unwrap();
				assert_eq!(allocation.memory_type_index(), 1);
				assert_eq!(
					STATE.with_borrow(|s| s.allocated.clone()),
					[(0, 4096), (1, 4096)]
				);
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			}
			assert_eq!(allocator.statistics(), AllocatorStats::default());
		}
	}
}

#[test]
fn bind_failure_rolls_back_new_or_reused_storage_and_persistent_holds() {
	for existing in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		// SAFETY: mock-only failures and cleanup; no external native use.
		unsafe {
			let retained = existing.then(|| {
				allocator
					.create_buffer_with_options(&buffer_info(), &options(true))
					.unwrap()
			});
			let before = allocator.statistics();
			clear();
			STATE.with_borrow_mut(|s| s.fail = Some("bind"));
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer_info(), &options(true)),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			assert_eq!(allocator.statistics(), before);
			if existing {
				assert_eq!(
					events(),
					["create", "requirements_core", "bind", "destroy_buffer"]
				);
			} else {
				assert_eq!(
					events(),
					[
						"create",
						"requirements_core",
						"allocate",
						"map",
						"bind",
						"destroy_buffer",
						"unmap",
						"free"
					]
				);
			}
			STATE.with_borrow_mut(|s| s.fail = None);
			let (b, mut y) = allocator
				.create_buffer_with_options(&buffer_info(), &options(true))
				.unwrap();
			assert_eq!(y.offset(), if existing { 320 } else { 0 });
			allocator.destroy_buffer(b, &mut y).unwrap();
			if let Some((a, mut x)) = retained {
				allocator.destroy_buffer(a, &mut x).unwrap();
			}
			assert_eq!(allocator.statistics(), AllocatorStats::default());
		}
	}
}

#[test]
fn persistent_map_failures_and_missing_inverse_preserve_existing_regions() {
	for failure in ["map", "null_map", "inverse"] {
		let fixture = Fixture::new(
			vk::API_VERSION_1_1,
			&[],
			true,
			(failure == "inverse").then_some(b"vkUnmapMemory".as_slice()),
		);
		configure();
		let mut allocator = fixture.allocator();
		// SAFETY: only mock mappings; null success is explicitly rolled back.
		unsafe {
			let (a, mut x) = allocator
				.create_buffer_with_options(&buffer_info(), &options(false))
				.unwrap();
			let before = allocator.statistics();
			clear();
			STATE.with_borrow_mut(|s| s.fail = Some(failure));
			assert!(
				allocator
					.create_buffer_with_options(&buffer_info(), &options(true))
					.is_err()
			);
			assert_eq!(allocator.statistics(), before);
			assert!(!events().contains(&"free"));
			assert_eq!(events().contains(&"unmap"), failure == "null_map");
			if failure == "inverse" {
				assert!(events().is_empty());
			}
			STATE.with_borrow_mut(|s| s.fail = None);
			allocator.destroy_buffer(a, &mut x).unwrap();
		}
	}
}

#[test]
fn policy_mask_heap_size_native_count_and_closed_state_fail_without_leaks() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic metadata and native resources only.
	unsafe {
		let mut opts = options(false);
		opts.memory_type_bits = 0;
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::NoCompatibleMemoryType)
		));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		clear();
		opts = options(false);
		opts.policy = AllocationPolicy::Pooled { block_size: 0 };
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(events().is_empty());
		allocator.allocation_size_limit = Some(1024);
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(false)),
			Err(AllocError::AllocationTooLarge {
				requested: 4096,
				limit: 1024
			})
		));
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		allocator.allocation_size_limit = None;
		allocator.selector.properties.memory_heaps[0].size = 4096;
		allocator.allocation_limit = 1;
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		assert_eq!(allocator.close(), Err(AllocError::LiveAllocations));
		assert!(matches!(
			allocator.allocate(&request()),
			Err(AllocError::AllocationLimit)
		));
		allocator.allocation_limit = 2;
		assert!(matches!(
			allocator.allocate(&request()),
			Err(AllocError::OutOfSpace)
		));
		allocator.destroy_buffer(a, &mut x).unwrap();
		allocator.close().unwrap();
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(false)),
			Err(AllocError::Closed)
		));
	}
}

#[test]
fn shared_map_counter_overflow_and_wrong_owners_precede_native_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let mut other = fixture.allocator();
	// SAFETY: synthetic pointers and checked counter states only.
	unsafe {
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		clear();
		assert_eq!(other.map(&x), Err(AllocError::WrongAllocator));
		assert_eq!(other.free(&mut x), Err(AllocError::WrongAllocator));
		allocator.blocks[0].mapping.references = u32::MAX;
		assert_eq!(allocator.map(&x), Err(AllocError::InvalidRange));
		assert!(events().is_empty());
		allocator.blocks[0].mapping.references = 0;
		allocator.destroy_buffer(a, &mut x).unwrap();
		clear();
		assert_eq!(allocator.map(&x), Err(AllocError::UnknownAllocation));
		assert!(events().is_empty());
	}
}

#[test]
fn real_host_oom_after_query_rolls_back_buffer_before_native_block_acquisition() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: real metadata OOM, synthetic native PFNs; vectors pre-reserved.
	unsafe {
		STATE.with_borrow_mut(|s| s.fail = Some("host_after_requirements"));
		let result = allocator.create_buffer_with_options(&buffer_info(), &options(false));
		assert!(
			!FAIL_NEXT_HOST_ALLOCATION.replace(false),
			"host reservation failure must be consumed"
		);
		assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		STATE.with_borrow_mut(|s| s.fail = None);
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		allocator.destroy_buffer(a, &mut x).unwrap();
	}
}

#[test]
fn seeded_native_region_trace_matches_independent_ranges_and_accounting() {
	for strategy in [
		PlacementStrategy::Default,
		PlacementStrategy::MinMemory,
		PlacementStrategy::MinTime,
		PlacementStrategy::MinOffset,
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let mut live: Vec<(vk::Buffer, Allocation)> = Vec::new();
		let mut random = 0x10fe_cafe_56aa_0041u64;
		let mut opts = options(false);
		opts.strategy = strategy;
		// SAFETY: synthetic native regions only; independent intervals/accounting checked.
		unsafe {
			for _ in 0..4000 {
				random ^= random << 13;
				random ^= random >> 7;
				random ^= random << 17;
				if !live.is_empty() && (random & 3 == 0 || live.len() >= 48) {
					let (buffer, mut allocation) = live.swap_remove(random as usize % live.len());
					allocator.destroy_buffer(buffer, &mut allocation).unwrap();
				} else {
					STATE.with_borrow_mut(|s| s.requirements_size = Some(257 + (random % 7) * 61));
					live.push(
						allocator
							.create_buffer_with_options(&buffer_info(), &opts)
							.unwrap(),
					);
				}
				let stats = allocator.statistics();
				let mut handles = Vec::new();
				let mut total = 0;
				let mut reserved = 0;
				for (i, (_, a)) in live.iter().enumerate() {
					assert_eq!(a.offset() % 64, 0);
					assert_eq!(a.reserved_size() % 64, 0);
					assert!(a.offset() + a.reserved_size() <= 4096);
					total += a.size();
					reserved += a.reserved_size();
					if !handles.contains(&a.handle()) {
						handles.push(a.handle());
					}
					for (_, b) in &live[i + 1..] {
						assert!(
							a.handle() != b.handle()
								|| a.offset() + a.reserved_size() <= b.offset()
								|| b.offset() + b.reserved_size() <= a.offset()
						);
					}
				}
				assert_eq!(
					(
						stats.allocation_count,
						stats.allocation_bytes,
						stats.reserved_bytes
					),
					(live.len(), total, reserved)
				);
				assert_eq!(
					(stats.native_allocation_count, stats.native_bytes),
					(handles.len(), handles.len() as u64 * 4096)
				);
			}
			for (buffer, mut a) in live {
				allocator.destroy_buffer(buffer, &mut a).unwrap();
			}
			assert_eq!(allocator.statistics(), AllocatorStats::default());
		}
	}
}

#[test]
fn unsupported_tile_storage_and_oversized_host_mapping_are_rejected() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: mock storage only; the large virtual extent is never mapped natively.
	unsafe {
		let mut info = buffer_info();
		info.usage = vk::BufferUsageFlagBits::TILE_MEMORY_BIT_QCOM;
		assert!(matches!(
			allocator.create_buffer_with_options(&info, &options(false)),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(events().is_empty());
		allocator.selector.properties.memory_heaps[0].flags =
			vk::MemoryHeapFlagBits::TILE_MEMORY_BIT_QCOM;
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(false)),
			Err(AllocError::NoCompatibleMemoryType)
		));
		allocator.selector.properties.memory_heaps[0].flags = vk::MemoryHeapFlags::empty();
		allocator.selector.properties.memory_heaps[0].size = u64::MAX;
		let opts = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled {
				block_size: isize::MAX as u64 + 1,
			},
			..options(false)
		};
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &opts)
			.unwrap();
		clear();
		assert_eq!(allocator.map(&allocation), Err(AllocError::InvalidRange));
		assert!(events().is_empty());
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
	}
}

#[test]
fn dropping_pooled_allocator_performs_no_native_cleanup_even_with_persistent_maps() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: mock-only lifetime leak proof; no real native allocation is leaked.
	{
		let _pair =
			unsafe { allocator.create_buffer_with_options(&buffer_info(), &options(true)) }.unwrap();
	}
	clear();
	drop(allocator);
	assert!(events().is_empty());
}

#[test]
fn native_count_ceiling_still_allows_reuse_of_a_lower_ranked_compatible_type() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut properties = properties();
	properties.memory_type_count = 2;
	properties.memory_types[1] = properties.memory_types[0];
	// SAFETY: matching synthetic property snapshot.
	let mut allocator = unsafe { Allocator::new(&fixture.device, properties, 64) }.unwrap();
	allocator.allocation_limit = 2;
	STATE.with_borrow_mut(|s| {
		s.requirements_bits = Some(3);
		s.requirements_size = Some(2048);
	});
	// SAFETY: mock buffer and memory objects, with explicit cleanup.
	unsafe {
		let first = AllocationCreateInfo {
			memory_type_bits: 1,
			..options(false)
		};
		let (a, mut x) = allocator
			.create_buffer_with_options(&buffer_info(), &first)
			.unwrap();
		let (b, mut y) = allocator
			.create_buffer_with_options(&buffer_info(), &first)
			.unwrap();
		STATE.with_borrow_mut(|s| s.requirements_size = Some(257));
		let second = AllocationCreateInfo {
			memory_type_bits: 2,
			..options(false)
		};
		let (c, mut z) = allocator
			.create_buffer_with_options(&buffer_info(), &second)
			.unwrap();
		clear();
		let (d, mut w) = allocator
			.create_buffer_with_options(&buffer_info(), &options(false))
			.unwrap();
		assert_eq!(w.memory_type_index(), 1);
		assert_eq!(w.handle(), z.handle());
		assert_eq!(events(), ["create", "requirements_core", "bind"]);
		for (b, a) in [(a, &mut x), (b, &mut y), (c, &mut z), (d, &mut w)] {
			allocator.destroy_buffer(b, a).unwrap();
		}
		assert_eq!(allocator.statistics(), AllocatorStats::default());
	}
}
