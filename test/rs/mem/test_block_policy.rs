//! Pinned VMA block-vector selection, mapping-group and incremental-order contracts.
use super::*;
use crate::mem::{HostAccess, MemoryUsage, PlacementStrategy, PoolCreateInfo};

fn requirements(size: u64) -> vk::MemoryRequirements {
	vk::MemoryRequirements {
		size,
		alignment: 64,
		memory_type_bits: 1,
	}
}

fn pool_info(count: usize) -> PoolCreateInfo {
	PoolCreateInfo {
		block_size: 1024,
		min_block_count: count,
		max_block_count: count,
		..Default::default()
	}
}

#[test]
fn strategies_select_forward_or_reverse_compatible_vectors_without_native_growth() {
	for visible in [false, true] {
		for strategy in [
			PlacementStrategy::Default,
			PlacementStrategy::MinMemory,
			PlacementStrategy::MinOffset,
			PlacementStrategy::MinTime,
		] {
			let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
			STATE.with_borrow_mut(|state| state.pooled = true);
			let mut properties = properties();
			if !visible {
				properties.memory_types[0].property_flags = vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT;
			}
			// SAFETY: fixed synthetic native memory and no host/GPU dereferences.
			unsafe {
				let mut allocator =
					Allocator::new_with_granularity(&fixture.device, properties, 64, 64).unwrap();
				let mut pool = allocator.create_pool(&pool_info(3)).unwrap();
				let options = AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				};
				let mut live = Vec::new();
				for size in [768, 640, 512] {
					live.push(
						allocator
							.allocate_with_options(&requirements(size), &options)
							.unwrap(),
					);
				}
				let expected = if strategy == PlacementStrategy::MinTime {
					2
				} else {
					0
				};
				clear();
				let allocation = allocator
					.allocate_with_options(
						&requirements(128),
						&AllocationCreateInfo {
							strategy,
							never_allocate: true,
							..options
						},
					)
					.unwrap();
				assert_eq!(
					allocation.handle(),
					live[expected].handle(),
					"{strategy:?}, visible={visible}"
				);
				assert!(events().is_empty());
				live.push(allocation);
				for mut allocation in live {
					allocator.free(&mut allocation).unwrap();
				}
				allocator.destroy_pool(&mut pool).unwrap();
				allocator.close().unwrap();
			}
		}
	}
}

#[test]
fn host_intent_prefers_mapped_or_unmapped_blocks_without_changing_mapping_holds() {
	for (usage, host_access, persistent, strategy, mapped_expected) in [
		(
			MemoryUsage::Explicit,
			HostAccess::None,
			false,
			PlacementStrategy::Default,
			true,
		),
		(
			MemoryUsage::Auto,
			HostAccess::None,
			false,
			PlacementStrategy::Default,
			false,
		),
		(
			MemoryUsage::Auto,
			HostAccess::Random,
			false,
			PlacementStrategy::MinMemory,
			true,
		),
		(
			MemoryUsage::PreferHost,
			HostAccess::SequentialWrite,
			false,
			PlacementStrategy::MinOffset,
			true,
		),
		(
			MemoryUsage::Auto,
			HostAccess::None,
			true,
			PlacementStrategy::Default,
			true,
		),
		(
			MemoryUsage::Auto,
			HostAccess::None,
			false,
			PlacementStrategy::MinTime,
			true,
		),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|state| state.pooled = true);
		let mut allocator = fixture.allocator();
		// SAFETY: synthetic buffers and mappings; no returned pointer is dereferenced.
		unsafe {
			let mut pool = allocator.create_pool(&pool_info(2)).unwrap();
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			};
			let mut a = allocator
				.allocate_with_options(&requirements(768), &options)
				.unwrap();
			let mut b = allocator
				.allocate_with_options(&requirements(768), &options)
				.unwrap();
			allocator.map(&b).unwrap();
			let info = vk::BufferCreateInfo {
				size: 128,
				..buffer_info()
			};
			STATE.with_borrow_mut(|state| state.requirements_size = Some(128));
			clear();
			let (buffer, mut result) = allocator
				.create_buffer_with_options(
					&info,
					&AllocationCreateInfo {
						usage,
						host_access,
						persistent_map: persistent,
						strategy,
						never_allocate: true,
						..options
					},
				)
				.unwrap();
			assert_eq!(
				result.handle(),
				if mapped_expected {
					b.handle()
				} else {
					a.handle()
				},
				"{usage:?}, {host_access:?}, persistent={persistent}, {strategy:?}"
			);
			assert_eq!(allocator.allocation_info(&b).unwrap().map_count, 1);
			assert!(
				!events()
					.iter()
					.any(|event| ["allocate", "map", "unmap"].contains(event))
			);
			allocator.destroy_buffer(buffer, &mut result).unwrap();
			allocator.unmap(&b).unwrap();
			allocator.free(&mut a).unwrap();
			allocator.free(&mut b).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn incompatible_preferred_mapping_group_falls_back_to_the_other_group() {
	for prefer_mapped in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|state| state.pooled = true);
		let mut allocator = fixture.allocator();
		// SAFETY: synthetic native storage; complete explicit cleanup and no pointer accesses.
		unsafe {
			let mut pool = allocator.create_pool(&pool_info(2)).unwrap();
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			};
			let mut full = allocator
				.allocate_with_options(&requirements(960), &options)
				.unwrap();
			let mut available = allocator
				.allocate_with_options(&requirements(768), &options)
				.unwrap();
			let mapped = if prefer_mapped { &full } else { &available };
			allocator.map(mapped).unwrap();
			STATE.with_borrow_mut(|state| state.requirements_size = Some(128));
			let (buffer, mut allocation) = allocator
				.create_buffer_with_options(
					&vk::BufferCreateInfo {
						size: 128,
						..buffer_info()
					},
					&AllocationCreateInfo {
						usage: MemoryUsage::Auto,
						host_access: if prefer_mapped {
							HostAccess::Random
						} else {
							HostAccess::None
						},
						never_allocate: true,
						..options
					},
				)
				.unwrap();
			assert_eq!(allocation.handle(), available.handle());
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.unmap(mapped).unwrap();
			allocator.free(&mut full).unwrap();
			allocator.free(&mut available).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn cached_native_mapping_is_preferred_without_granting_an_allocation_hold() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|state| state.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic mapping pointer, never dereferenced; cleanup explicitly unmaps cache.
	unsafe {
		let mut pool = allocator.create_pool(&pool_info(2)).unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		let mut a = allocator
			.allocate_with_options(&requirements(768), &options)
			.unwrap();
		let mut b = allocator
			.allocate_with_options(&requirements(768), &options)
			.unwrap();
		for _ in 0..8 {
			allocator.map(&b).unwrap();
			allocator.unmap(&b).unwrap();
		}
		let cached = allocator
			.blocks
			.iter()
			.find(|block| block.handle == b.handle())
			.unwrap();
		assert_eq!(cached.mapping.read().references, 0);
		assert!(!cached.mapping.read().pointer.is_null());
		clear();
		let mut allocation = allocator
			.allocate_with_options(&requirements(128), &options)
			.unwrap();
		assert_eq!(allocation.handle(), b.handle());
		let info = allocator.allocation_info(&allocation).unwrap();
		assert_eq!(info.map_count, 0);
		assert!(info.mapped_pointer.is_none());
		assert!(events().is_empty());
		allocator.free(&mut allocation).unwrap();
		allocator.free(&mut a).unwrap();
		allocator.free(&mut b).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn partial_free_reorders_the_vector_and_next_request_uses_the_fuller_block() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|state| state.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic requirements and native storage, with no host/GPU uses.
	unsafe {
		let mut pool = allocator.create_pool(&pool_info(2)).unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		let mut a = allocator
			.allocate_with_options(&requirements(256), &options)
			.unwrap();
		let mut b = allocator
			.allocate_with_options(&requirements(512), &options)
			.unwrap();
		let mut c = allocator
			.allocate_with_options(&requirements(640), &options)
			.unwrap();
		assert_eq!(a.handle(), b.handle());
		assert_ne!(b.handle(), c.handle());
		allocator.free(&mut b).unwrap();
		assert_eq!(allocator.blocks[0].handle, c.handle());
		let mut next = allocator
			.allocate_with_options(&requirements(128), &options)
			.unwrap();
		assert_eq!(next.handle(), c.handle());
		allocator.free(&mut next).unwrap();
		allocator.free(&mut a).unwrap();
		allocator.free(&mut c).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn partial_free_keeps_a_nontrailing_spare_until_incremental_order_reaches_it() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|state| state.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic native blocks; all regions are explicitly retired before pool destruction.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 0,
				..pool_info(3)
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			..Default::default()
		};
		let mut a = allocator
			.allocate_with_options(&requirements(1024), &options)
			.unwrap();
		let mut b = allocator
			.allocate_with_options(&requirements(512), &options)
			.unwrap();
		let mut c = allocator
			.allocate_with_options(&requirements(512), &options)
			.unwrap();
		let mut d = allocator
			.allocate_with_options(&requirements(768), &options)
			.unwrap();
		assert_eq!(b.handle(), c.handle());
		assert_ne!(a.handle(), b.handle());
		assert_ne!(b.handle(), d.handle());
		allocator.free(&mut a).unwrap();
		assert!(allocator.blocks[1].metadata.is_empty());
		assert!(!allocator.blocks[2].metadata.is_empty());
		clear();
		allocator.free(&mut b).unwrap();
		assert!(events().is_empty());
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 3);
		assert!(allocator.blocks[2].metadata.is_empty());
		allocator.free(&mut c).unwrap();
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 2);
		allocator.free(&mut d).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}
