//! Native linear custom-pool identities, atom padding, mapping and failure contracts.
use super::*;
use crate::mem::{AllocationAlgorithm, AllocationCreateInfo, PoolCreateInfo};

fn pool_info() -> PoolCreateInfo {
	PoolCreateInfo {
		algorithm: AllocationAlgorithm::Linear,
		block_size: 1024,
		max_block_count: 1,
		..Default::default()
	}
}

fn options(pool: &crate::Pool, upper_address: bool) -> AllocationCreateInfo<'_> {
	AllocationCreateInfo {
		pool: Some(pool),
		upper_address,
		..Default::default()
	}
}

#[test]
fn double_stack_publishes_actual_upper_binding_and_private_atom_ranges() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: mock native memory, no GPU/host dereferences; exact explicit cleanup.
	unsafe {
		let mut pool = allocator.create_pool(&pool_info()).unwrap();
		let (low, mut a) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, false))
			.unwrap();
		let (high, mut b) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, true))
			.unwrap();
		assert_eq!((a.offset(), b.offset()), (0, 704));
		assert_eq!((a.reserved_size(), b.reserved_size()), (320, 320));
		assert_eq!(a.handle(), b.handle());
		assert_eq!(
			STATE.with_borrow(|s| s.bindings.clone()),
			[(low.as_raw(), 0), (high.as_raw(), 704)]
		);
		assert_eq!(b.mapped_range(256, 1).unwrap().unwrap().offset, 960);
		let (inner, mut c) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, true))
			.unwrap();
		assert_eq!(c.offset(), 384);
		let before = allocator.statistics();
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(&pool, true)),
			Err(AllocError::OutOfSpace)
		));
		assert_eq!(allocator.statistics(), before);
		allocator.destroy_buffer(inner, &mut c).unwrap();
		allocator.destroy_buffer(high, &mut b).unwrap();
		allocator.destroy_buffer(low, &mut a).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn ring_reuses_completed_prefixes_and_keeps_live_tail_generation() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: native uses are synthetic and complete before explicit region release.
	unsafe {
		let mut pool = allocator.create_pool(&pool_info()).unwrap();
		let mut live = Vec::new();
		for _ in 0..3 {
			live.push(
				allocator
					.create_buffer_with_options(&buffer_info(), &options(&pool, false))
					.unwrap(),
			);
		}
		for expected in [0, 320, 640, 0, 320, 640] {
			let (buffer, mut old) = live.remove(0);
			assert_eq!(old.offset(), expected);
			allocator.destroy_buffer(buffer, &mut old).unwrap();
			let next = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool, false))
				.unwrap();
			assert_eq!(next.1.offset(), expected);
			assert_eq!(allocator.free(&mut old), Err(AllocError::UnknownAllocation));
			live.push(next);
		}
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		assert_eq!(
			allocator.pool_statistics(&pool).unwrap().reserved_bytes,
			960
		);
		for (buffer, mut token) in live {
			allocator.destroy_buffer(buffer, &mut token).unwrap();
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn multiblock_stack_uses_newest_block_and_never_wraps_old_holes() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: two synthetic block domains, no accesses survive region destruction.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				max_block_count: 2,
				..pool_info()
			})
			.unwrap();
		let mut first = Vec::new();
		for _ in 0..3 {
			first.push(
				allocator
					.create_buffer_with_options(&buffer_info(), &options(&pool, false))
					.unwrap(),
			);
		}
		let (buffer, mut token) = first.remove(0);
		allocator.destroy_buffer(buffer, &mut token).unwrap();
		let newest = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, false))
			.unwrap();
		assert_ne!(newest.1.handle(), first[0].1.handle());
		assert_eq!(newest.1.offset(), 0);
		// An older empty block cannot be selected instead of the newest stack tail.
		for (buffer, mut token) in first {
			allocator.destroy_buffer(buffer, &mut token).unwrap();
		}
		let second = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, false))
			.unwrap();
		assert_eq!(second.1.handle(), newest.1.handle());
		assert_eq!(second.1.offset(), 320);
		for (buffer, mut token) in [newest, second] {
			allocator.destroy_buffer(buffer, &mut token).unwrap();
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn invalid_upper_routes_reject_before_native_resource_creation() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: invalid routes acquire no native state; empty pools explicitly destroyed.
	unsafe {
		clear();
		let opts = AllocationCreateInfo {
			upper_address: true,
			..Default::default()
		};
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &opts),
			Err(AllocError::UnsupportedCapability(_))
		));
		assert!(events().is_empty());
		for info in [
			PoolCreateInfo {
				algorithm: AllocationAlgorithm::Tlsf,
				..pool_info()
			},
			PoolCreateInfo {
				max_block_count: 2,
				..pool_info()
			},
			PoolCreateInfo {
				max_block_count: 0,
				..pool_info()
			},
		] {
			let mut pool = allocator.create_pool(&info).unwrap();
			clear();
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer_info(), &options(&pool, true)),
				Err(AllocError::UnsupportedCapability(_))
			));
			assert!(matches!(
				allocator.create_image_with_options(
					&image::info(vk::ImageTiling::OPTIMAL),
					&options(&pool, true)
				),
				Err(AllocError::UnsupportedCapability(_))
			));
			assert!(events().is_empty());
			allocator.destroy_pool(&mut pool).unwrap();
		}
		allocator.close().unwrap();
	}
}

#[test]
fn bind_map_and_native_failures_restore_upper_stack_before_retry() {
	for operation in ["allocate", "map", "bind"] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| {
			s.pooled = true;
			s.fail = Some(operation);
		});
		let mut allocator = fixture.allocator();
		// SAFETY: injected native failure, inverse operations remain available.
		unsafe {
			let mut pool = allocator.create_pool(&pool_info()).unwrap();
			let opts = AllocationCreateInfo {
				persistent_map: true,
				..options(&pool, true)
			};
			assert!(matches!(
				allocator.create_buffer_with_options(&buffer_info(), &opts),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			assert_eq!(allocator.statistics().allocation_count, 0);
			assert_eq!(allocator.statistics().mapped_native_count, 0);
			STATE.with_borrow_mut(|s| s.fail = None);
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &opts)
				.unwrap();
			assert_eq!(token.offset(), 704);
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn automatic_upper_uses_actual_grown_block_and_never_escapes_to_dedication() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut props = properties();
	props.memory_heaps[0].size = 512 * 1024;
	// SAFETY: matching synthetic properties; all resources explicitly released.
	unsafe {
		let mut allocator = Allocator::new(&fixture.device, props, 64).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 0,
				..pool_info()
			})
			.unwrap();
		let (buffer, mut token) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, true))
			.unwrap();
		assert_eq!(token.block_size, 8192);
		assert_eq!(token.offset(), 7872);
		STATE.with_borrow_mut(|s| s.requirements_size = Some(8192));
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(&pool, true)),
			Err(AllocError::OutOfSpace)
		));
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		STATE.with_borrow_mut(|s| {
			s.required = true;
			s.requirements_size = None;
		});
		assert!(matches!(
			allocator.create_buffer_with_options(&buffer_info(), &options(&pool, true)),
			Err(AllocError::UnsupportedCapability(_))
		));
		assert_eq!(allocator.statistics().allocation_count, 1);
		allocator.destroy_buffer(buffer, &mut token).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn metadata_failure_in_preallocated_block_has_no_native_storage_or_cursor_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.pooled = true);
	let mut allocator = fixture.allocator();
	// SAFETY: actual host allocation injection after synthetic requirements; no external access.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				min_block_count: 1,
				..pool_info()
			})
			.unwrap();
		let before = allocator.statistics();
		clear();
		STATE.with_borrow_mut(|s| s.fail = Some("host_after_requirements"));
		let failure = allocator.create_buffer_with_options(&buffer_info(), &options(&pool, true));
		assert!(!FAIL_NEXT_HOST_ALLOCATION.replace(false));
		assert!(matches!(failure, Err(AllocError::HostAllocationFailed)));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(allocator.statistics(), before);
		STATE.with_borrow_mut(|s| s.fail = None);
		let (buffer, mut token) = allocator
			.create_buffer_with_options(&buffer_info(), &options(&pool, true))
			.unwrap();
		assert_eq!(token.offset(), 704);
		allocator.destroy_buffer(buffer, &mut token).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn image_stacks_share_page_separated_blocks_with_buffers() {
	for tiling in [vk::ImageTiling::LINEAR, vk::ImageTiling::OPTIMAL] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		STATE.with_borrow_mut(|s| s.pooled = true);
		let mut allocator = fixture.allocator();
		// SAFETY: synthetic ordinary images, no mapping/pixel-layout assumptions or GPU uses.
		unsafe {
			let mut pool = allocator.create_pool(&pool_info()).unwrap();
			let (low, mut a) = allocator
				.create_image_with_options(&image::info(tiling), &options(&pool, false))
				.unwrap();
			let (high, mut b) = allocator
				.create_image_with_options(&image::info(tiling), &options(&pool, true))
				.unwrap();
			assert_eq!((a.offset(), b.offset()), (0, 704));
			assert_eq!(a.handle(), b.handle());
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool, false))
				.unwrap();
			assert_eq!(token.offset(), 320);
			assert_eq!(token.handle(), a.handle());
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			allocator.destroy_image(high, &mut b).unwrap();
			assert_eq!(
				allocator.destroy_pool(&mut pool),
				Err(AllocError::LiveAllocations)
			);
			let memory = a.handle();
			allocator.destroy_image(low, &mut a).unwrap();
			let (buffer, mut token) = allocator
				.create_buffer_with_options(&buffer_info(), &options(&pool, true))
				.unwrap();
			assert_eq!(token.offset(), 704);
			assert_eq!(token.handle(), memory);
			allocator.destroy_buffer(buffer, &mut token).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
	}
}
