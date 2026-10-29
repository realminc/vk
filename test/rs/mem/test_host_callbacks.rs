//! Public callback-controlled virtual placement and failure/cleanup contracts.
use vk::{
	self, AllocError, VirtualBlock,
	mem::{AllocationAlgorithm, PlacementStrategy, SynchronizedVirtualBlock},
};

#[path = "host/test_callbacks.rs"]
mod callbacks;
use callbacks::Tracker;

#[test]
fn tlsf_and_linear_metadata_growth_clear_json_and_drop_use_the_matching_callbacks() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		{
			// SAFETY: synchronized exact-layout callback state/code retained through block Drop.
			let mut block =
				unsafe { VirtualBlock::new_with_callbacks(65536, algorithm, &callbacks) }.unwrap();
			let mut regions = Vec::new();
			for index in 0..96 {
				let region = block
					.allocate(17 + index % 7, 16, PlacementStrategy::Default)
					.unwrap();
				block.set_user_data(&region, index as usize).unwrap();
				regions.push(region);
			}
			let mut json = Vec::new();
			let before = tracker.snapshot();
			block.write_statistics_json(&mut json, true).unwrap();
			let after = tracker.snapshot();
			assert_eq!(after.attempts, before.attempts + 1);
			assert_eq!((after.live, after.bytes), (before.live, before.bytes));
			assert!(
				String::from_utf8(json)
					.unwrap()
					.contains("\"allocation_count\":96")
			);
			for mut region in regions {
				block.free(&mut region).unwrap();
			}
			assert!(block.is_empty());
			let before = tracker.snapshot();
			block.clear();
			assert_eq!(tracker.snapshot(), before);
			let mut region = block
				.allocate(65536, 1, PlacementStrategy::Default)
				.unwrap();
			block.free(&mut region).unwrap();
		}
		let after = tracker.snapshot();
		assert!(after.successes > 2);
		assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
		assert_eq!(after.successes, after.frees);
	}
}

#[test]
fn partial_tlsf_construction_and_growth_failures_return_all_owned_storage() {
	for failure in [1, 2] {
		let tracker = Tracker::new();
		tracker.fail_at(failure);
		let callbacks = tracker.callbacks();
		// SAFETY: the fixture supplies a legal null-allocation failure and remains live through cleanup.
		assert!(matches!(
			unsafe { VirtualBlock::new_with_callbacks(1024, AllocationAlgorithm::Tlsf, &callbacks) },
			Err(AllocError::HostAllocationFailed)
		));
		let after = tracker.snapshot();
		assert_eq!((after.live, after.unexpected), (0, 0));
		assert_eq!(after.successes, after.frees);
	}
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		// SAFETY: exact paired fixture callbacks with failure injection only before acquiring storage.
		let mut block =
			unsafe { VirtualBlock::new_with_callbacks(4096, algorithm, &callbacks) }.unwrap();
		let before = block.statistics();
		tracker.fail_at(tracker.snapshot().attempts + 1);
		assert!(matches!(
			block.allocate(32, 16, PlacementStrategy::Default),
			Err(AllocError::HostAllocationFailed)
		));
		assert_eq!(block.statistics(), before);
		let mut region = block.allocate(32, 16, PlacementStrategy::Default).unwrap();
		assert_eq!(block.allocation_info(&region).unwrap().offset, 0);
		block.free(&mut region).unwrap();
		drop(block);
		assert_eq!(tracker.snapshot().live, 0);
	}
}

#[test]
fn detailed_json_failure_precedes_output_and_preserves_placement() {
	for algorithm in [AllocationAlgorithm::Tlsf, AllocationAlgorithm::Linear] {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		// SAFETY: live exact-layout callback fixture, no native resources involved.
		let mut block =
			unsafe { VirtualBlock::new_with_callbacks(4096, algorithm, &callbacks) }.unwrap();
		let mut region = block.allocate(64, 16, PlacementStrategy::Default).unwrap();
		let before = block.statistics();
		let mut output = b"prefix".to_vec();
		tracker.fail_at(tracker.snapshot().attempts + 1);
		assert_eq!(
			block.write_statistics_json(&mut output, true),
			Err(AllocError::HostAllocationFailed)
		);
		assert_eq!(output, b"prefix");
		assert_eq!(block.statistics(), before);
		block.free(&mut region).unwrap();
		drop(block);
		assert_eq!(tracker.snapshot().live, 0);
	}
}

#[test]
fn missing_callback_pairs_are_rejected_before_allocation_and_tokens_can_outlive_the_block() {
	let tracker = Tracker::new();
	for missing_allocation in [false, true] {
		let mut callbacks = tracker.callbacks();
		if missing_allocation {
			callbacks.pfn_allocation = None;
		} else {
			callbacks.pfn_free = None;
		}
		// SAFETY: malformed function-pair shape is rejected without invoking any callback.
		assert!(matches!(
			unsafe { VirtualBlock::new_with_callbacks(1024, AllocationAlgorithm::Tlsf, &callbacks) },
			Err(AllocError::InvalidRequest(_))
		));
	}
	assert_eq!(tracker.snapshot().attempts, 0);
	let old = {
		let callbacks = tracker.callbacks();
		// SAFETY: callback state and record outlive the block, but need not outlive the non-owning token.
		let mut block =
			unsafe { VirtualBlock::new_with_callbacks(1024, AllocationAlgorithm::Tlsf, &callbacks) }
				.unwrap();
		block.allocate(64, 1, PlacementStrategy::Default).unwrap()
	};
	assert_eq!(tracker.snapshot().live, 0);
	let mut current = VirtualBlock::new(1024).unwrap();
	assert_eq!(
		current.allocation_info(&old),
		Err(AllocError::WrongAllocator)
	);
	let mut region = current.allocate(64, 1, PlacementStrategy::Default).unwrap();
	current.free(&mut region).unwrap();
}

#[test]
fn callback_owned_blocks_move_between_threads_and_share_only_synchronized_user_state() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: the shared callback fixture serializes its ledger and retains all code/user data until scoped threads and owners finish.
	let blocks: Vec<_> = (0..8)
		.map(|_| {
			unsafe { VirtualBlock::new_with_callbacks(8192, AllocationAlgorithm::Tlsf, &callbacks) }
				.unwrap()
		})
		.collect();
	std::thread::scope(|scope| {
		for mut block in blocks {
			scope.spawn(move || {
				for _ in 0..64 {
					let mut region = block.allocate(32, 8, PlacementStrategy::Default).unwrap();
					block.free(&mut region).unwrap();
				}
			});
		}
	});
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn shared_virtual_diagnostics_use_synchronized_callbacks_without_mutating_placements() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: callback records/state outlive the block and all scoped readers;
	// its mutex serializes every independent diagnostic scratch allocation/free.
	let mut block =
		unsafe { VirtualBlock::new_with_callbacks(8192, AllocationAlgorithm::Tlsf, &callbacks) }
			.unwrap();
	let mut regions = Vec::new();
	for _ in 0..32 {
		regions.push(block.allocate(32, 8, PlacementStrategy::Default).unwrap());
	}
	let expected = block.statistics();
	std::thread::scope(|scope| {
		for _ in 0..8 {
			let shared = &block;
			scope.spawn(move || {
				for _ in 0..32 {
					assert_eq!(shared.statistics(), expected);
					shared
						.write_statistics_json(&mut std::io::sink(), true)
						.unwrap();
				}
			});
		}
	});
	assert_eq!(block.statistics(), expected);
	for mut region in regions {
		block.free(&mut region).unwrap();
	}
	drop(block);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn synchronized_virtual_block_callbacks_survive_worker_cleanup() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: the tracker synchronizes callback state and outlives the block;
	// its allocation/free code supports all participating threads through Drop.
	let block = unsafe {
		SynchronizedVirtualBlock::new_with_callbacks(8192, AllocationAlgorithm::Tlsf, &callbacks)
	}
	.unwrap();
	std::thread::scope(|scope| {
		for _ in 0..4 {
			let block = &block;
			scope.spawn(move || {
				for _ in 0..32 {
					let mut region = block.allocate(32, 8, PlacementStrategy::Default).unwrap();
					block
						.write_statistics_json(&mut std::io::sink(), true)
						.unwrap();
					block.free(&mut region).unwrap();
				}
			});
		}
	});
	assert!(block.is_empty().unwrap());
	std::thread::scope(|scope| {
		scope.spawn(move || drop(block)).join().unwrap();
	});
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn synchronized_virtual_block_adopts_live_callback_backed_regions() {
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: the synchronized tracker and callback record remain valid through
	// adoption, worker access and final host cleanup.
	let mut ordinary =
		unsafe { VirtualBlock::new_with_callbacks(4096, AllocationAlgorithm::Tlsf, &callbacks) }
			.unwrap();
	let mut region = ordinary
		.allocate(64, 16, PlacementStrategy::Default)
		.unwrap();
	// SAFETY: the same callback/thread contract holds after transferring the block.
	let block = unsafe { SynchronizedVirtualBlock::new_from_block(ordinary) };
	std::thread::scope(|scope| {
		scope.spawn(|| {
			assert_eq!(block.allocation_info(&region).unwrap().size, 64);
			block.free(&mut region).unwrap();
		});
	});
	assert!(block.is_empty().unwrap());
	drop(block);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}
