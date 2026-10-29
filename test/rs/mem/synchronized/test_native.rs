//! Dynamic same-vector overlap, backing exclusion and reservation rollback.
use super::*;
use std::sync::{Barrier, mpsc};

fn info() -> vk::BufferCreateInfo<'static> {
	vk::BufferCreateInfo {
		size: 64,
		usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
		..Default::default()
	}
}

#[test]
fn blocked_backing_map_and_bind_allow_same_vector_placement_and_free() {
	for binding in [false, true] {
		let _test = reset();
		let (_instance, device) = contexts();
		// SAFETY: exact fixture resources, real stable backing, joined workers;
		// no GPU work and every native object is explicitly retired.
		unsafe {
			let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
			let mut pool = allocator
				.create_pool(&PoolCreateInfo {
					block_size: 128,
					..Default::default()
				})
				.unwrap();
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			};
			let first_buffer = device.create_buffer(&info(), None).unwrap();
			let mut first = allocator
				.allocate_for_buffer(first_buffer, &options)
				.unwrap();
			let (neighbor_buffer, mut neighbor) = create(&mut allocator, &pool).unwrap();
			let (second_buffer, mut second) = create(&mut allocator, &pool).unwrap();
			let (spare_buffer, mut spare) = create(&mut allocator, &pool).unwrap();
			assert_eq!(first.handle(), neighbor.handle());
			assert_eq!(second.handle(), spare.handle());
			assert_ne!(first.handle(), second.handle());
			allocator.destroy_buffer(spare_buffer, &mut spare).unwrap();
			let shared = SynchronizedAllocator::new(allocator);
			let gate = Gate::new(usize::MAX);
			*GATE.lock().unwrap() = None;
			if binding {
				BIND_TARGET.store(first.handle().as_raw(), Ordering::SeqCst);
				*BIND_GATE.lock().unwrap() = Some(Arc::clone(&gate));
			} else {
				*MAP_GATE.lock().unwrap() = Some(Arc::clone(&gate));
			}
			std::thread::scope(|scope| {
				let native = scope.spawn(|| {
					if binding {
						shared.bind_buffer_memory(first_buffer, &first).unwrap();
					} else {
						shared.map(&first).unwrap();
					}
				});
				gate.wait_for_entry();
				// Keep the first backing blocked until another thread has both
				// placed AND freed on the second backing of this exact pool/vector.
				let (send, receive) = mpsc::channel();
				let shared_ref = &shared;
				let pool_ref = &pool;
				let second_handle = second.handle();
				let placement = scope.spawn(move || {
					let shared = shared_ref;
					let pool = pool_ref;
					let options = AllocationCreateInfo {
						pool: Some(pool),
						..Default::default()
					};
					let (buffer, mut allocation) = shared
						.create_buffer_with_options(&info(), &options)
						.unwrap();
					assert_eq!(allocation.handle(), second_handle);
					shared.destroy_buffer(buffer, &mut allocation).unwrap();
					send.send(()).unwrap();
				});
				let progressed = receive.recv_timeout(Duration::from_secs(3)).is_ok();
				gate.release();
				native.join().unwrap();
				placement.join().unwrap();
				assert!(
					progressed,
					"same-vector placement/free waited for the unrelated backing"
				);
			});
			assert!(!gate.0.lock().unwrap().timed_out);
			assert!(OVERLAP.load(Ordering::SeqCst) > 0);
			*BIND_GATE.lock().unwrap() = None;
			*MAP_GATE.lock().unwrap() = None;
			if !binding {
				shared.unmap(&first).unwrap();
			}
			shared.destroy_buffer(first_buffer, &mut first).unwrap();
			shared
				.destroy_buffer(neighbor_buffer, &mut neighbor)
				.unwrap();
			shared.destroy_buffer(second_buffer, &mut second).unwrap();
			shared.destroy_pool(&mut pool).unwrap();
			shared.close().unwrap();
		}
		empty_native();
		println!(
			"\nCONCURRENCY independent-{} 1 0 0",
			if binding { "bind" } else { "map" }
		);
	}
}

#[test]
fn same_backing_map_and_bind_serialize_and_repeated_maps_keep_exact_counts() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: exact host-only fixture with explicitly joined threads and cleanup.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		allocator.set_mapping_hysteresis(false).unwrap();
		let mut pool = new_pool(&mut allocator);
		let (first_buffer, mut first) = create(&mut allocator, &pool).unwrap();
		let second_buffer = device.create_buffer(&info(), None).unwrap();
		let mut second = allocator
			.allocate_for_buffer(
				second_buffer,
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				},
			)
			.unwrap();
		assert_eq!(first.handle(), second.handle());
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(usize::MAX);
		*GATE.lock().unwrap() = None;
		*MAP_GATE.lock().unwrap() = Some(Arc::clone(&gate));
		std::thread::scope(|scope| {
			let mapping = scope.spawn(|| {
				shared.map(&first).unwrap();
			});
			gate.wait_for_entry();
			let binding = scope.spawn(|| shared.bind_buffer_memory(second_buffer, &second).unwrap());
			// Observe both leases, proving the bind worker reached the backing
			// mutex rather than depending on thread startup timing.
			let deadline = std::time::Instant::now() + Duration::from_secs(3);
			let mut admitted = false;
			while std::time::Instant::now() < deadline {
				if shared.inner.lock().unwrap().active == 2 {
					admitted = true;
					break;
				}
				std::thread::yield_now();
			}
			let bound_early = NATIVE.lock().unwrap().buffers[&second_buffer.as_raw()].is_some();
			gate.release();
			mapping.join().unwrap();
			binding.join().unwrap();
			assert!(admitted);
			assert!(!bound_early, "bind bypassed the active backing map mutex");
		});
		*MAP_GATE.lock().unwrap() = None;
		shared.unmap(&first).unwrap();
		let barrier = Barrier::new(5);
		std::thread::scope(|scope| {
			for _ in 0..4 {
				scope.spawn(|| {
					let pointer = shared.map(&first).unwrap();
					assert!(!pointer.is_null());
					barrier.wait();
					barrier.wait();
					shared.unmap(&first).unwrap();
				});
			}
			barrier.wait();
			let count = shared.allocation_info(&first).unwrap().map_count;
			barrier.wait();
			assert_eq!(count, 4);
		});
		assert_eq!(shared.allocation_info(&first).unwrap().map_count, 0);
		assert_eq!(NATIVE.lock().unwrap().maps, 2);
		assert_eq!(NATIVE.lock().unwrap().unmaps, 2);
		shared.destroy_buffer(first_buffer, &mut first).unwrap();
		shared.destroy_buffer(second_buffer, &mut second).unwrap();
		shared.destroy_pool(&mut pool).unwrap();
		shared.close().unwrap();
	}
	empty_native();
	println!("\nCONCURRENCY same-backing 1 2 2");
}

#[test]
fn callback_oom_and_native_map_bind_failures_preserve_retry_and_cleanup() {
	let _test = reset();
	let (_instance, device) = contexts();
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: synchronized callbacks and native ledger outlive every operation.
	unsafe {
		let mut allocator =
			Allocator::new_with_callbacks(&device, properties_coherent(), 64, &callbacks).unwrap();
		let mut pool = new_pool(&mut allocator);
		let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
		let original_pointer = allocator.map(&allocation).unwrap();
		original_pointer.cast::<u64>().write_unaligned(0xabc123);
		let shared = SynchronizedAllocator::new(allocator);
		tracker.fail_at(tracker.snapshot().attempts + 1);
		assert!(matches!(
			shared.map(&allocation),
			Err(AllocError::HostAllocationFailed)
		));
		assert_eq!(NATIVE.lock().unwrap().maps, 1);
		assert_eq!(shared.allocation_info(&allocation).unwrap().map_count, 1);
		assert_eq!(original_pointer.cast::<u64>().read_unaligned(), 0xabc123);
		// Successful promotion preserves the old native hold through explicit release.
		shared.unmap(&allocation).unwrap();
		assert_eq!(NATIVE.lock().unwrap().unmaps, 1);
		FAIL_MAP.store(true, Ordering::SeqCst);
		assert!(matches!(shared.map(&allocation), Err(AllocError::Call(_))));
		assert_eq!(shared.allocation_info(&allocation).unwrap().map_count, 0);
		let pointer = shared.map(&allocation).unwrap();
		pointer.cast::<u64>().write_unaligned(0x1234);
		shared.unmap(&allocation).unwrap();
		let second_buffer = device.create_buffer(&info(), Some(&callbacks)).unwrap();
		let mut second = shared
			.allocate_for_buffer(
				second_buffer,
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				},
			)
			.unwrap();
		FAIL_BIND.store(true, Ordering::SeqCst);
		assert!(matches!(
			shared.bind_buffer_memory(second_buffer, &second),
			Err(AllocError::Call(_))
		));
		assert!(NATIVE.lock().unwrap().buffers[&second_buffer.as_raw()].is_none());
		shared.bind_buffer_memory(second_buffer, &second).unwrap();
		shared.destroy_buffer(buffer, &mut allocation).unwrap();
		shared.destroy_buffer(second_buffer, &mut second).unwrap();
		shared.destroy_pool(&mut pool).unwrap();
		shared.close().unwrap();
	}
	empty_native();
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn pending_raw_bind_claim_rejects_competitors_and_close_waits_for_publication() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: two distinct raw allocations, one live ordinary buffer and joined
	// host calls; a failed second claim must never reach the native bind.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let request = AllocRequest {
			size: 64,
			alignment: 64,
			memory_type_bits: 1,
			required_flags: vk::MemoryPropertyFlags::empty(),
			preferred_flags: vk::MemoryPropertyFlags::empty(),
		};
		let mut first = allocator.allocate(&request).unwrap();
		let mut second = allocator.allocate(&request).unwrap();
		let buffer = device.create_buffer(&info(), None).unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(usize::MAX);
		*GATE.lock().unwrap() = None;
		*BIND_GATE.lock().unwrap() = Some(Arc::clone(&gate));
		std::thread::scope(|scope| {
			let binding = scope.spawn(|| shared.bind_buffer_memory(buffer, &first).unwrap());
			gate.wait_for_entry();
			assert_eq!(
				shared.bind_buffer_memory(buffer, &second),
				Err(AllocError::WrongResource)
			);
			assert!(matches!(
				shared.allocate_for_buffer(buffer, &AllocationCreateInfo::default()),
				Err(AllocError::InvalidRequest(
					"resource already has an allocation"
				))
			));
			let (started, receive) = mpsc::channel();
			let shared_ref = &shared;
			let closing = scope.spawn(move || {
				started.send(()).unwrap();
				shared_ref.close()
			});
			receive.recv().unwrap();
			assert_eq!(shared.try_with_allocator(|_| ()), Ok(None));
			gate.release();
			binding.join().unwrap();
			assert_eq!(closing.join().unwrap(), Err(AllocError::LiveAllocations));
		});
		*BIND_GATE.lock().unwrap() = None;
		shared.destroy_buffer(buffer, &mut first).unwrap();
		shared.free(&mut second).unwrap();
		shared.close().unwrap();
	}
	empty_native();
}

#[test]
fn free_trim_and_pool_destruction_cannot_retire_an_active_native_backing() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: joined workers use a private identity snapshot only to check that
	// competing retirement waits and rejects the newly acquired explicit map.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut pool = new_pool(&mut allocator);
		let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
		let mut snapshot = allocator.token(allocation.serial).unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(usize::MAX);
		*GATE.lock().unwrap() = None;
		*MAP_GATE.lock().unwrap() = Some(Arc::clone(&gate));
		std::thread::scope(|scope| {
			let mapping = scope.spawn(|| {
				shared.map(&allocation).unwrap();
			});
			gate.wait_for_entry();
			let (send, receive) = mpsc::channel();
			let shared_ref = &shared;
			let retiring = scope.spawn(move || {
				send.send(()).unwrap();
				shared_ref.free(&mut snapshot)
			});
			let trimming = scope.spawn(|| shared.trim());
			let destroying = scope.spawn(|| shared.destroy_pool(&mut pool));
			receive.recv().unwrap();
			assert!(
				NATIVE
					.lock()
					.unwrap()
					.memory
					.contains_key(&allocation.handle().as_raw())
			);
			gate.release();
			mapping.join().unwrap();
			assert_eq!(retiring.join().unwrap(), Err(AllocError::AlreadyMapped));
			assert_eq!(trimming.join().unwrap().unwrap().block_count, 0);
			assert_eq!(destroying.join().unwrap(), Err(AllocError::LiveAllocations));
		});
		*MAP_GATE.lock().unwrap() = None;
		shared.unmap(&allocation).unwrap();
		shared.destroy_buffer(buffer, &mut allocation).unwrap();
		shared.destroy_pool(&mut pool).unwrap();
		shared.close().unwrap();
	}
	empty_native();
}
