//! Native allocator metadata ownership and failure boundaries with mock PFNs.
use super::*;
use crate::mem::{
	DefragmentationAlgorithm, DefragmentationContext, DefragmentationInfo, PoolCreateInfo,
};

use crate::mem::host::test_callbacks::Tracker;

fn configure(callbacks: &vk::AllocationCallbacks) {
	STATE.with_borrow_mut(|state| {
		state.pooled = true;
		state.expected_callbacks = std::ptr::from_ref(callbacks).addr();
		state.events.reserve(4096);
		state.allocated.reserve(64);
		state.allocation_flags.reserve(64);
		state.bindings.reserve(128);
		state.ranges.reserve(128);
	});
}

#[test]
fn complete_native_pool_name_map_diagnostics_and_relocation_path_avoids_global_metadata() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	configure(&callbacks);
	FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
	let result = (|| -> Result<usize, AllocError> {
		// SAFETY: exact synthetic native metadata/PFNs and synchronized callback
		// fixture. No synthetic mapping pointer is dereferenced; native uses are complete.
		unsafe {
			let mut allocator =
				Allocator::new_with_callbacks(&fixture.device, properties(), 64, &callbacks)?;
			let mut pool = allocator.create_pool(&PoolCreateInfo {
				block_size: 1024,
				min_block_count: 1,
				max_block_count: 2,
				..Default::default()
			})?;
			allocator.set_pool_name(&pool, Some("pool λ"))?;
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			};
			let (first, mut a) = allocator.create_buffer_with_options(&buffer_info(), &options)?;
			let (old, mut b) = allocator.create_buffer_with_options(&buffer_info(), &options)?;
			allocator.set_allocation_name(&b, Some("buffer 雪"))?;
			allocator.set_allocation_user_data(&b, 17)?;
			allocator.map(&b)?;
			allocator.flush_many(&[crate::mem::CacheRange {
				allocation: &b,
				offset: 0,
				size: 64,
			}])?;
			allocator.unmap(&b)?;
			allocator.write_statistics_json(&mut std::io::sink(), true)?;
			allocator.destroy_buffer(first, &mut a)?;
			let mut context = DefragmentationContext::new(
				&mut allocator,
				&DefragmentationInfo {
					algorithm: DefragmentationAlgorithm::Full,
					..Default::default()
				},
			)?;
			let mut pass = context.begin_pass()?;
			if pass.len() != 1 {
				return Err(AllocError::InvalidRequest("expected one relocation"));
			}
			let replacement = pass
				.device()
				.create_buffer(&buffer_info(), pass.allocation_callbacks())?;
			pass.bind_buffer(0, replacement)?;
			pass
				.device()
				.destroy_buffer(old, pass.allocation_callbacks())?;
			pass.commit_move(0, &mut b)?;
			pass.finish()?;
			drop(pass);
			let moved = context.finish()?.moved_allocations;
			drop(context);
			if allocator.allocation_name(&b)? != Some("buffer 雪")
				|| allocator.allocation_info(&b)?.user_data != 17
				|| allocator.pool_name(&pool)? != Some("pool λ")
			{
				return Err(AllocError::InvalidRequest("relocation lost metadata"));
			}
			allocator.destroy_buffer(replacement, &mut b)?;
			allocator.destroy_pool(&mut pool)?;
			allocator.close()?;
			Ok(moved)
		}
	})();
	let untouched = FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.replace(false));
	assert_eq!(result, Ok(1));
	assert!(
		untouched,
		"allocator-owned metadata used the global Rust allocator"
	);
	let after = tracker.snapshot();
	assert_eq!((after.live, after.bytes, after.unexpected), (0, 0, 0));
	assert_eq!(after.successes, after.frees);
}

#[test]
fn host_registry_pool_and_placement_failures_precede_native_acquisition() {
	// Sweep every allocator-owned host allocation before the first managed buffer
	// is published. Each failure must balance acquired host storage and preserve
	// the exact native resource count. Native objects are explicitly cleaned up.
	let mut completed = false;
	for failure in 1..=32 {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		configure(&callbacks);
		tracker.fail_at(failure);
		// SAFETY: exact synthetic requirements/callbacks and completed mock resources.
		unsafe {
			let mut allocator =
				Allocator::new_with_callbacks(&fixture.device, properties(), 64, &callbacks).unwrap();
			let result = allocator.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					policy: AllocationPolicy::Pooled { block_size: 1024 },
					..Default::default()
				},
			);
			match result {
				Ok((buffer, mut allocation)) => {
					assert!(tracker.snapshot().attempts < failure);
					allocator.destroy_buffer(buffer, &mut allocation).unwrap();
					completed = true;
				}
				Err(error) => {
					assert_eq!(error, AllocError::HostAllocationFailed);
					assert_eq!(allocator.statistics().allocation_count, 0);
					assert_eq!(allocator.statistics().native_allocation_count, 0);
				}
			}
			allocator.close().unwrap();
		}
		let after = tracker.snapshot();
		assert_eq!((after.live, after.unexpected), (0, 0), "failure {failure}");
		assert_eq!(after.successes, after.frees);
		if completed {
			break;
		}
	}
	assert!(completed, "bounded construction sweep never completed");
}

#[test]
fn name_and_diagnostic_failures_are_transactional_and_default_callbacks_stay_global() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	configure(&callbacks);
	// SAFETY: exact mock resource/callback lifetimes, explicit native cleanup.
	unsafe {
		let mut allocator = Allocator::new_from_physical_device_with_callbacks(
			&fixture.instance,
			vk::PhysicalDevice::from_raw(0x3000),
			&fixture.device,
			&callbacks,
		)
		.unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 1024,
				..Default::default()
			})
			.unwrap();
		allocator
			.set_pool_name(&pool, Some("original pool"))
			.unwrap();
		let (buffer, mut allocation) = allocator
			.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					pool: Some(&pool),
					..Default::default()
				},
			)
			.unwrap();
		allocator
			.set_allocation_name(&allocation, Some("original allocation"))
			.unwrap();
		for pool_name in [false, true] {
			tracker.fail_at(tracker.snapshot().attempts + 1);
			let result = if pool_name {
				allocator.set_pool_name(&pool, Some("replacement"))
			} else {
				allocator.set_allocation_name(&allocation, Some("replacement"))
			};
			assert_eq!(result, Err(AllocError::HostAllocationFailed));
			assert_eq!(allocator.pool_name(&pool).unwrap(), Some("original pool"));
			assert_eq!(
				allocator.allocation_name(&allocation).unwrap(),
				Some("original allocation")
			);
		}
		let mut output = b"prefix".to_vec();
		tracker.fail_at(tracker.snapshot().attempts + 1);
		assert_eq!(
			allocator.write_statistics_json(&mut output, true),
			Err(AllocError::HostAllocationFailed)
		);
		assert_eq!(output, b"prefix");
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
	assert_eq!(tracker.snapshot().live, 0);
	// A default allocator retains global fallible reservation, rather than
	// inheriting any callback from an earlier allocator on this thread.
	STATE.with_borrow_mut(|state| state.expected_callbacks = 0);
	let mut allocator = fixture.allocator();
	FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(true));
	// SAFETY: injected global metadata failure precedes native allocation.
	let result = unsafe { allocator.allocate(&request()) };
	FAIL_NEXT_HOST_ALLOCATION.with(|flag| flag.set(false));
	assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
	allocator.close().unwrap();
}

#[test]
fn callback_host_drop_releases_metadata_without_native_retirement_and_tokens_stay_nonowning() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	configure(&callbacks);
	let token = {
		// SAFETY: deliberate mock native leak has no physical resources; CPU metadata still drops with matching callbacks.
		let mut allocator =
			unsafe { Allocator::new_with_callbacks(&fixture.device, properties(), 64, &callbacks) }
				.unwrap();
		// SAFETY: synthetic ordinary buffer requirements and explicit callback lifetime.
		let (_, allocation) = unsafe {
			allocator.create_buffer_with_options(
				&buffer_info(),
				&AllocationCreateInfo {
					policy: AllocationPolicy::Pooled { block_size: 1024 },
					..Default::default()
				},
			)
		}
		.unwrap();
		clear();
		allocation
	};
	assert!(events().is_empty());
	assert_eq!(tracker.snapshot().live, 0);
	STATE.with_borrow_mut(|state| state.expected_callbacks = 0);
	let allocator = fixture.allocator();
	assert_eq!(
		allocator.allocation_info(&token),
		Err(AllocError::WrongAllocator)
	);
}

#[test]
fn every_minimum_pool_metadata_failure_rolls_back_partial_native_blocks() {
	let mut completed = false;
	for failure in 1..=32 {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		configure(&callbacks);
		tracker.fail_at(failure);
		// SAFETY: synthetic native blocks and exact retained callback family.
		unsafe {
			let mut allocator =
				Allocator::new_with_callbacks(&fixture.device, properties(), 64, &callbacks).unwrap();
			match allocator.create_pool(&PoolCreateInfo {
				block_size: 1024,
				min_block_count: 2,
				max_block_count: 2,
				..Default::default()
			}) {
				Ok(mut pool) => {
					assert!(tracker.snapshot().attempts < failure);
					allocator.destroy_pool(&mut pool).unwrap();
					completed = true;
				}
				Err(error) => {
					assert_eq!(error, AllocError::HostAllocationFailed);
					assert_eq!(allocator.statistics().native_allocation_count, 0);
					assert!(allocator.pools.is_empty());
				}
			}
			allocator.close().unwrap();
		}
		let after = tracker.snapshot();
		assert_eq!((after.live, after.unexpected), (0, 0), "failure {failure}");
		assert_eq!(after.successes, after.frees);
		if completed {
			break;
		}
	}
	assert!(completed, "bounded pool construction sweep never completed");
}

#[test]
fn missing_host_callback_pairs_are_rejected_before_acquiring_metadata_or_native_memory() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let tracker = Tracker::new();
	for missing_allocation in [false, true] {
		let mut callbacks = tracker.callbacks();
		if missing_allocation {
			callbacks.pfn_allocation = None;
		} else {
			callbacks.pfn_free = None;
		}
		clear();
		// SAFETY: function-pair shape is deliberately invalid and rejected before invocation.
		assert!(matches!(
			unsafe { Allocator::new_with_callbacks(&fixture.device, properties(), 64, &callbacks) },
			Err(AllocError::InvalidRequest(_))
		));
		assert!(events().is_empty());
	}
	assert_eq!(tracker.snapshot().attempts, 0);
}
