//! Default-vector isolation and same-pool block mapping with independent PFN evidence.
use super::*;

fn default_options() -> AllocationCreateInfo<'static> {
	AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 4096 },
		..Default::default()
	}
}

fn info() -> vk::BufferCreateInfo<'static> {
	vk::BufferCreateInfo {
		size: 64,
		usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
		..Default::default()
	}
}

#[test]
fn default_type_scopes_overlap_and_never_escape_native_type_selection() {
	let _test = reset();
	let (_instance, device) = contexts();
	let mut properties = properties_coherent();
	properties.memory_type_count = 2;
	properties.memory_types[1] = properties.memory_types[0];
	REQ_BITS.store(3, Ordering::SeqCst);
	// SAFETY: two fixture types share one checked heap. All workers retain exact
	// callback/device lifetimes, and all returned objects are explicitly retired.
	unsafe {
		let mut allocator = Allocator::new(&device, properties, 64).unwrap();
		let mut untouched_pool = new_pool(&mut allocator);
		let (untouched_buffer, mut untouched) = create(&mut allocator, &untouched_pool).unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(2);
		let mut values = shared
			.with_memory_types(&[0, 1], |children| {
				std::thread::scope(|scope| {
					let workers: Vec<_> = children
						.iter()
						.enumerate()
						.map(|(index, child)| {
							scope.spawn(move || {
								child
									.with_allocator(|allocator| {
										assert!(matches!(
											allocator.close(),
											Err(AllocError::InvalidRequest(_))
										));
										let options = default_options();
										assert!(matches!(
											allocator.create_buffer_with_options(
												&info(),
												&AllocationCreateInfo {
													memory_type_bits: 1 << (1 - index),
													..options
												}
											),
											Err(AllocError::NoCompatibleMemoryType)
										));
										let (buffer, allocation) = allocator
											.create_buffer_with_options(&info(), &options)
											.unwrap();
										assert_eq!(allocation.memory_type_index(), index as u32);
										assert_eq!(
											NATIVE.lock().unwrap().types[&allocation.handle().as_raw()],
											index as u32
										);
										let mut request = AllocRequest {
											size: 64,
											alignment: 64,
											memory_type_bits: 1 << (1 - index),
											required_flags: vk::MemoryPropertyFlags::empty(),
											preferred_flags: vk::MemoryPropertyFlags::empty(),
										};
										assert!(matches!(
											allocator.allocate(&request),
											Err(AllocError::NoCompatibleMemoryType)
										));
										request.memory_type_bits = 3;
										let mut raw = allocator.allocate(&request).unwrap();
										assert_eq!(raw.memory_type_index(), index as u32);
										assert_eq!(
											NATIVE.lock().unwrap().types[&raw.handle().as_raw()],
											index as u32
										);
										allocator.free(&mut raw).unwrap();
										let pointer = allocator.map(&allocation).unwrap().cast::<u64>();
										pointer.write_unaligned(0x1234 + index as u64);
										(buffer, allocation)
									})
									.unwrap()
							})
						})
						.collect();
					workers
						.into_iter()
						.map(|worker| worker.join().unwrap())
						.collect::<Vec<_>>()
				})
			})
			.unwrap();
		assert!(!gate.0.lock().unwrap().timed_out);
		*GATE.lock().unwrap() = None;
		assert!(OVERLAP.load(Ordering::SeqCst) > 0);
		let mut allocator = shared.into_inner().unwrap();
		assert_eq!(allocator.statistics().allocation_count, 3);
		assert_eq!(
			allocator
				.pool_statistics(&untouched_pool)
				.unwrap()
				.allocation_count,
			1
		);
		for (index, (buffer, allocation)) in values.iter_mut().enumerate() {
			assert_eq!(
				allocator
					.mapped_pointer(allocation)
					.unwrap()
					.unwrap()
					.cast::<u64>()
					.read_unaligned(),
				0x1234 + index as u64
			);
			allocator.unmap(allocation).unwrap();
			allocator.destroy_buffer(*buffer, allocation).unwrap();
		}
		allocator
			.destroy_buffer(untouched_buffer, &mut untouched)
			.unwrap();
		allocator.destroy_pool(&mut untouched_pool).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn same_pool_blocks_map_concurrently_keep_all_regions_and_reject_placement_changes() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: same pool's independent native blocks own stable real host backing.
	// Negative placement operations must reject before any native effect.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 128,
				..Default::default()
			})
			.unwrap();
		let (first_buffer, first) = create(&mut allocator, &pool).unwrap();
		let (extra_buffer, mut extra) = create(&mut allocator, &pool).unwrap();
		let (second_buffer, second) = create(&mut allocator, &pool).unwrap();
		assert_eq!(first.handle(), extra.handle());
		assert_ne!(first.handle(), second.handle());
		let original_order = allocator
			.blocks
			.iter()
			.map(|block| block.handle)
			.collect::<Vec<_>>();
		let original_budget = allocator.budget_snapshot();
		allocator.budget_cache = Some(original_budget);
		allocator.budget_operations = 7;
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(2);
		*GATE.lock().unwrap() = None;
		*MAP_GATE.lock().unwrap() = Some(Arc::clone(&gate));
		shared
			.with_blocks(&[&first, &second], |children| {
				std::thread::scope(|scope| {
					let first_worker = scope.spawn(|| {
						children[0]
							.with_allocator(|allocator| {
								assert!(matches!(
									allocator.free(&mut extra),
									Err(AllocError::InvalidRequest(_))
								));
								assert!(matches!(
									allocator.destroy_buffer(extra_buffer, &mut extra),
									Err(AllocError::InvalidRequest(_))
								));
								assert!(matches!(
									allocator.trim(),
									Err(AllocError::InvalidRequest(_))
								));
								assert!(matches!(
									allocator.create_buffer_with_options(&info(), &default_options()),
									Err(AllocError::InvalidRequest(_))
								));
								let pointer = allocator.map(&first).unwrap().cast::<u64>();
								pointer.write_unaligned(0x1111);
								let extra_pointer = allocator.map(&extra).unwrap().cast::<u64>();
								extra_pointer.write_unaligned(0x3333);
								assert_eq!(allocator.statistics().allocation_count, 2);
							})
							.unwrap()
					});
					let second_worker = scope.spawn(|| {
						children[1]
							.with_allocator(|allocator| {
								assert!(matches!(
									allocator.map(&first),
									Err(AllocError::UnknownAllocation)
								));
								allocator
									.map(&second)
									.unwrap()
									.cast::<u64>()
									.write_unaligned(0x2222);
							})
							.unwrap()
					});
					first_worker.join().unwrap();
					second_worker.join().unwrap();
				});
			})
			.unwrap();
		assert!(!gate.0.lock().unwrap().timed_out);
		*MAP_GATE.lock().unwrap() = None;
		let mut allocator = shared.into_inner().unwrap();
		assert_eq!(allocator.budget_cache, Some(original_budget));
		assert_eq!(allocator.budget_operations, 7);
		assert_eq!(
			allocator
				.blocks
				.iter()
				.map(|block| block.handle)
				.collect::<Vec<_>>(),
			original_order
		);
		for (buffer, mut allocation, expected) in [
			(first_buffer, first, 0x1111),
			(extra_buffer, extra, 0x3333),
			(second_buffer, second, 0x2222),
		] {
			assert_eq!(
				allocator
					.mapped_pointer(&allocation)
					.unwrap()
					.unwrap()
					.cast::<u64>()
					.read_unaligned(),
				expected
			);
			allocator.unmap(&allocation).unwrap();
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn invalid_domains_and_block_scope_unwind_preserve_linear_order_and_poison() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: panic is before native work; every owner is recovered and retired.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 64,
				min_block_count: 3,
				algorithm: crate::mem::AllocationAlgorithm::Linear,
				..Default::default()
			})
			.unwrap();
		let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
		let (later_buffer, mut later_allocation) = create(&mut allocator, &pool).unwrap();
		let original = allocator
			.blocks
			.iter()
			.map(|block| block.handle)
			.collect::<Vec<_>>();
		let shared = SynchronizedAllocator::new(allocator);
		assert!(matches!(
			shared.with_memory_types(&[0, 0], |_| ()),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(matches!(
			shared.with_memory_types(&[1], |_| ()),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(matches!(
			shared.with_blocks(&[&allocation, &allocation], |_| ()),
			Err(AllocError::InvalidRequest(_))
		));
		let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
			shared.with_blocks::<()>(&[&allocation], |_| panic!("block phase unwind"))
		}));
		assert!(result.is_err());
		let mut allocator = shared.into_inner().err().unwrap().into_inner();
		assert_eq!(
			allocator
				.blocks
				.iter()
				.map(|block| block.handle)
				.collect::<Vec<_>>(),
			original
		);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator
			.destroy_buffer(later_buffer, &mut later_allocation)
			.unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}
