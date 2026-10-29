//! Direct `&self` operation surface: mixed-phase calls on one serialized owner.
use super::*;
use crate::mem::{PlacementStrategy, SynchronizedVirtualBlock, VirtualAllocation};

fn access_options() -> AllocationCreateInfo<'static> {
	AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 4096 },
		..Default::default()
	}
}

#[test]
fn direct_operations_support_mixed_phases_and_close_from_any_thread() {
	let _test = reset();
	let (_instance, device) = contexts();
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: the fixed native ledger and callback tracker outlive every worker,
	// and all returned objects are explicitly retired before close.
	unsafe {
		let shared = SynchronizedAllocator::new(
			Allocator::new_with_callbacks(&device, properties_coherent(), 64, &callbacks).unwrap(),
		);
		let mut pool = shared
			.create_pool(&PoolCreateInfo {
				block_size: 4096,
				..Default::default()
			})
			.unwrap();
		// Different workers exercise different phases on the same owner without
		// any borrowed partition: managed buffers in a pool, raw allocations with
		// mapping, and allocator-wide statistics/budget/configuration queries.
		std::thread::scope(|scope| {
			scope.spawn(|| {
				for iteration in 0..32_u64 {
					let (buffer, mut allocation) = shared
						.create_buffer_with_options(
							&vk::BufferCreateInfo {
								size: 64,
								usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
								..Default::default()
							},
							&AllocationCreateInfo {
								pool: Some(&pool),
								..access_options()
							},
						)
						.unwrap();
					shared
						.set_allocation_name(&allocation, Some("pooled worker"))
						.unwrap();
					let pointer = shared.map(&allocation).unwrap().cast::<u64>();
					pointer.write_unaligned(iteration);
					assert_eq!(pointer.read_unaligned(), iteration);
					shared.unmap(&allocation).unwrap();
					assert!(shared.mapped_pointer(&allocation).unwrap().is_none());
					shared.destroy_buffer(buffer, &mut allocation).unwrap();
				}
			});
			scope.spawn(|| {
				for iteration in 0..32_u64 {
					let mut allocation = shared
						.allocate(&AllocRequest {
							size: 64,
							alignment: 64,
							memory_type_bits: 1,
							required_flags: vk::MemoryPropertyFlags::empty(),
							preferred_flags: vk::MemoryPropertyFlags::empty(),
						})
						.unwrap();
					assert_eq!(shared.allocation_info(&allocation).unwrap().size, 64);
					shared
						.set_allocation_user_data(&allocation, iteration as usize)
						.unwrap();
					let mut bytes = [0_u8; 16];
					let pointer = shared.map(&allocation).unwrap().cast::<u64>();
					pointer.write_unaligned(iteration);
					shared
						.copy_from_allocation(&allocation, 0, &mut bytes)
						.unwrap();
					assert_eq!(
						u64::from_ne_bytes(bytes[..8].try_into().unwrap()),
						iteration
					);
					shared.unmap(&allocation).unwrap();
					shared.free(&mut allocation).unwrap();
				}
			});
			scope.spawn(|| {
				for _ in 0..32 {
					assert!(shared.statistics().is_ok());
					assert!(shared.budget_snapshot().is_ok());
					assert!(shared.pool_statistics(&pool).is_ok());
					shared.set_current_frame_index(0).unwrap();
					assert_eq!(shared.current_frame_index().unwrap(), 0);
					assert!(shared.live_count().is_ok());
				}
			});
		});
		// Overlap across independent backings is permitted; native ownership is
		// checked independently by the ledger, not by a global exclusion rule.
		shared.destroy_pool(&mut pool).unwrap();
		assert_eq!(shared.live_count().unwrap(), 0);
		std::thread::scope(|scope| {
			scope.spawn(|| shared.close().unwrap()).join().unwrap();
		});
		assert!(shared.is_closed().unwrap());
		assert!(matches!(
			shared.allocate(&AllocRequest {
				size: 64,
				alignment: 64,
				memory_type_bits: 1,
				required_flags: vk::MemoryPropertyFlags::empty(),
				preferred_flags: vk::MemoryPropertyFlags::empty(),
			}),
			Err(AllocError::Closed)
		));
	}
	empty_native();
}

#[test]
fn direct_operations_refuse_access_after_worker_panic() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: fixed fixture; the panic occurs before any native state change.
	unsafe {
		let shared =
			SynchronizedAllocator::new(Allocator::new(&device, properties_coherent(), 64).unwrap());
		let mut allocation = shared
			.allocate(&AllocRequest {
				size: 64,
				alignment: 64,
				memory_type_bits: 1,
				required_flags: vk::MemoryPropertyFlags::empty(),
				preferred_flags: vk::MemoryPropertyFlags::empty(),
			})
			.unwrap();
		std::thread::scope(|scope| {
			let worker = scope.spawn(|| {
				let _ = shared.with_allocator::<()>(|_| panic!("injected closure failure"));
			});
			assert!(worker.join().is_err());
		});
		assert_eq!(
			shared.allocation_info(&allocation),
			Err(AllocError::SynchronizationPoisoned)
		);
		assert!(matches!(
			shared.allocate(&AllocRequest {
				size: 64,
				alignment: 64,
				memory_type_bits: 1,
				required_flags: vk::MemoryPropertyFlags::empty(),
				preferred_flags: vk::MemoryPropertyFlags::empty(),
			}),
			Err(AllocError::SynchronizationPoisoned)
		));
		assert_eq!(
			shared.budget_snapshot(),
			Err(AllocError::SynchronizationPoisoned)
		);
		let mut allocator = match shared.into_inner() {
			Err(poison) => poison.into_inner(),
			Ok(_) => panic!("poison lost"),
		};
		allocator.free(&mut allocation).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn synchronized_virtual_block_serves_concurrent_placement_and_recovery() {
	let _test = reset();
	fn send_sync<T: Send + Sync>() {}
	send_sync::<SynchronizedVirtualBlock<'_>>();
	fn send<T: Send>() {}
	send::<VirtualAllocation>();
	let block = SynchronizedVirtualBlock::new(4096).unwrap();
	std::thread::scope(|scope| {
		for worker in 0..4_u64 {
			let block = &block;
			scope.spawn(move || {
				let mut tokens = Vec::new();
				for index in 0..16_u64 {
					let token = block.allocate(64, 64, PlacementStrategy::Default).unwrap();
					block
						.set_user_data(&token, ((worker << 8) | index) as usize)
						.unwrap();
					tokens.push(token);
				}
				for mut token in tokens {
					let info = block.allocation_info(&token).unwrap();
					assert_eq!(info.size, 64);
					block.free(&mut token).unwrap();
				}
			});
		}
	});
	assert!(block.is_empty().unwrap());
	// A token moves to another thread, which observes and frees it.
	let mut token = block
		.allocate(128, 128, PlacementStrategy::Default)
		.unwrap();
	std::thread::scope(|scope| {
		scope.spawn(|| {
			assert_eq!(block.allocation_info(&token).unwrap().offset, 0);
			block.free(&mut token).unwrap();
		});
	});
	let stats = block.statistics().unwrap();
	assert_eq!((stats.allocation_count, stats.allocated_bytes), (0, 0));
	let mut block = block.into_inner().unwrap();
	let mut token = block.allocate(64, 64, PlacementStrategy::Default).unwrap();
	assert_eq!(block.allocation_info(&token).unwrap().offset, 0);
	block.free(&mut token).unwrap();
}

#[test]
fn synchronized_virtual_block_poison_retains_regions_for_explicit_recovery() {
	struct PanickingWriter;

	impl std::io::Write for PanickingWriter {
		fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
			panic!("injected writer failure");
		}

		fn flush(&mut self) -> std::io::Result<()> {
			Ok(())
		}
	}

	let block = SynchronizedVirtualBlock::new(256).unwrap();
	let mut token = block.allocate(64, 8, PlacementStrategy::Default).unwrap();
	let result = std::panic::catch_unwind(|| {
		let _ = block.write_statistics_json(&mut PanickingWriter, false);
	});
	assert!(result.is_err());
	assert_eq!(
		block.allocation_info(&token),
		Err(AllocError::SynchronizationPoisoned)
	);
	assert!(matches!(
		block.allocate(16, 8, PlacementStrategy::Default),
		Err(AllocError::SynchronizationPoisoned)
	));
	let mut recovered = match block.into_inner() {
		Err(poison) => poison.into_inner(),
		Ok(_) => panic!("poison lost"),
	};
	assert_eq!(recovered.allocation_info(&token).unwrap().size, 64);
	recovered.free(&mut token).unwrap();
	assert!(recovered.is_empty());
}

#[test]
fn synchronized_virtual_block_writer_error_does_not_poison_access() {
	struct FailingWriter;

	impl std::io::Write for FailingWriter {
		fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
			Err(std::io::ErrorKind::BrokenPipe.into())
		}

		fn flush(&mut self) -> std::io::Result<()> {
			Ok(())
		}
	}

	let block = SynchronizedVirtualBlock::new(256).unwrap();
	assert!(matches!(
		block.write_statistics_json(&mut FailingWriter, true),
		Err(AllocError::Output { .. })
	));
	let mut token = block.allocate(64, 8, PlacementStrategy::Default).unwrap();
	block.free(&mut token).unwrap();
	assert!(block.is_empty().unwrap());
}
