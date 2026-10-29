//! Mixed-resource evacuation/packing against explicit synthetic native lifetimes.
use super::*;
use crate::mem::{
	DefragmentationMove, DefragmentationPass, DefragmentationResource, DefragmentationStats,
	PoolCreateInfo,
};

struct Resident {
	resource: DefragmentationResource,
	allocation: Allocation,
}

unsafe fn mixed_layout(
	allocator: &mut Allocator<'_>,
	options: &AllocationCreateInfo,
) -> Vec<Resident> {
	let mut resources = Vec::new();
	// Three 1024-byte blocks: buffer/image halves. Delete the first image and
	// second buffer, leaving two half-full blocks and one full mixed block.
	// SAFETY: mock requirements/identities only, all native cleanup remains explicit.
	unsafe {
		for _ in 0..3 {
			let (buffer, allocation) = create(allocator, 512, options);
			resources.push(Resident {
				resource: DefragmentationResource::Buffer(buffer),
				allocation,
			});
			let (image, allocation) = allocator
				.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), options)
				.unwrap();
			resources.push(Resident {
				resource: DefragmentationResource::Image(image),
				allocation,
			});
		}
		if let DefragmentationResource::Image(image) = resources[1].resource {
			allocator
				.destroy_image(image, &mut resources[1].allocation)
				.unwrap();
		}
		if let DefragmentationResource::Buffer(buffer) = resources[2].resource {
			allocator
				.destroy_buffer(buffer, &mut resources[2].allocation)
				.unwrap();
		}
	}
	assert_eq!(allocator.statistics().native_allocation_count, 3);
	assert_eq!(allocator.statistics().allocation_count, 4);
	resources
}

unsafe fn resolve(
	pass: &mut DefragmentationPass<'_, '_>,
	residents: &mut [Resident],
) -> Vec<DefragmentationMove> {
	let mut trace = Vec::new();
	for index in 0..pass.len() {
		let movement = *pass.movement(index).unwrap();
		let resident = residents
			.iter_mut()
			.find(|resident| {
				resident.resource == movement.resource && !resident.allocation.handle().is_null()
			})
			.unwrap();
		STATE.with_borrow_mut(|state| state.requirements_size = Some(resident.allocation.size()));
		// SAFETY: matching mock requirements and native identities. There is no
		// GPU data here; this proves explicit protocol transitions, not copying.
		unsafe {
			let replacement = match resident.resource {
				DefragmentationResource::Buffer(old) => {
					let new = pass
						.device()
						.create_buffer(
							&vk::BufferCreateInfo {
								size: resident.allocation.size(),
								..buffer_info()
							},
							None,
						)
						.unwrap();
					pass.bind_buffer(index, new).unwrap();
					pass.device().destroy_buffer(old, None).unwrap();
					DefragmentationResource::Buffer(new)
				}
				DefragmentationResource::Image(old) => {
					let new = pass
						.device()
						.create_image(&image::info(vk::ImageTiling::OPTIMAL), None)
						.unwrap();
					pass.bind_image(index, new).unwrap();
					pass.device().destroy_image(old, None).unwrap();
					DefragmentationResource::Image(new)
				}
				DefragmentationResource::Unassociated => unreachable!("fixture owns each resource"),
			};
			pass.commit_move(index, &mut resident.allocation).unwrap();
			resident.resource = replacement;
		}
		trace.push(movement);
	}
	pass.finish().unwrap();
	trace
}

unsafe fn cleanup(allocator: &mut Allocator<'_>, resources: &mut [Resident]) {
	for resident in resources {
		if resident.allocation.handle().is_null() {
			continue;
		}
		// SAFETY: every mock resource remains owned by this test and has no users.
		unsafe {
			match resident.resource {
				DefragmentationResource::Buffer(buffer) => allocator
					.destroy_buffer(buffer, &mut resident.allocation)
					.unwrap(),
				DefragmentationResource::Image(image) => allocator
					.destroy_image(image, &mut resident.allocation)
					.unwrap(),
				DefragmentationResource::Unassociated => unreachable!("fixture owns each resource"),
			}
		}
	}
}

fn info() -> DefragmentationInfo<'static> {
	DefragmentationInfo {
		algorithm: DefragmentationAlgorithm::Extensive,
		max_allocations: 1,
		..Default::default()
	}
}

unsafe fn run(
	context: &mut DefragmentationContext<'_, '_>,
	resources: &mut [Resident],
) -> Vec<DefragmentationMove> {
	let mut trace = Vec::new();
	for _ in 0..32 {
		// SAFETY: fixture sources stay live; every proposed mock move is resolved.
		let mut pass = unsafe { context.begin_pass() }.unwrap();
		if pass.is_empty() {
			pass.finish().unwrap();
			return trace;
		}
		assert_eq!(pass.len(), 1);
		// SAFETY: mock resource replacement, no actual copy/completion work exists.
		trace.extend(unsafe { resolve(&mut pass, resources) });
	}
	panic!("Extensive planning failed to terminate within the fixture's finite phases");
}

unsafe fn run_with_stop(stop_at: Option<usize>) -> usize {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: the same independently checked mixed-class fixture, with simulated
	// completed copies and explicit resource retirement on every proposed move.
	unsafe {
		let mut residents = mixed_layout(
			&mut allocator,
			&AllocationCreateInfo {
				policy: AllocationPolicy::Pooled { block_size: 1024 },
				..Default::default()
			},
		);
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		let mut polls = 0;
		let mut interrupted = false;
		let mut complete = false;
		for _ in 0..64 {
			let mut pass = context
				.begin_pass_with_break(&mut || {
					let stop = stop_at == Some(polls);
					polls += 1;
					stop
				})
				.unwrap();
			interrupted |= pass.was_interrupted();
			if pass.is_empty() {
				let exhausted = !pass.was_interrupted();
				pass.finish().unwrap();
				if exhausted {
					complete = true;
					break;
				}
			} else {
				resolve(&mut pass, &mut residents);
			}
		}
		assert!(complete, "stop boundary {stop_at:?}");
		assert!(
			stop_at.is_none() || interrupted,
			"stop boundary was not reached"
		);
		let stats = context.finish().unwrap();
		assert_eq!((stats.freed_blocks, stats.freed_bytes), (1, 1024));
		drop(context);
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		let mut buffer_memory = None;
		let mut image_memory = None;
		for resident in &residents {
			if resident.allocation.is_freed() {
				continue;
			}
			let group = match resident.resource {
				DefragmentationResource::Buffer(_) => &mut buffer_memory,
				DefragmentationResource::Image(_) => &mut image_memory,
				DefragmentationResource::Unassociated => unreachable!("fixture owns resources"),
			};
			assert_eq!(
				*group.get_or_insert(resident.allocation.handle()),
				resident.allocation.handle()
			);
		}
		assert_ne!(buffer_memory, image_memory);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
		polls
	}
}

#[test]
fn every_extensive_stop_boundary_resumes_to_the_same_packed_resource_groups() {
	// SAFETY: the helper owns the complete synthetic lifecycle of each replay.
	unsafe {
		let polls = run_with_stop(None);
		assert!(polls > 1);
		for boundary in 0..polls {
			run_with_stop(Some(boundary));
		}
	}
}

#[test]
fn extensive_requires_a_persistent_context_before_any_native_effect() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	clear();
	// SAFETY: empty mock context, rejected before planning/native effects.
	assert!(matches!(
		unsafe { allocator.begin_defragmentation(&info()) },
		Err(AllocError::UnsupportedCapability(_))
	));
	assert!(events().is_empty());
	allocator.close().unwrap();
}

#[test]
fn extensive_evacuates_then_packs_classes_and_frees_surplus_only_at_finish() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	// SAFETY: explicit synthetic resource lifetime transitions without GPU work.
	unsafe {
		let mut residents = mixed_layout(&mut allocator, &options);
		let original_first = residents[4].allocation.handle();
		// Partial-free incremental swaps put the first-created half-full
		// block last among equal-free blocks. Context ordering retains that tie.
		let original_last = residents[0].allocation.handle();
		clear();
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		let trace = run(&mut context, &mut residents);
		assert!(trace.len() >= 6);
		assert!(trace.iter().any(|movement| matches!(
			movement.resource,
			DefragmentationResource::Image(_)
		) && movement.source.device_memory == original_first
			&& movement.destination.device_memory == original_last));
		assert!(!events().contains(&"allocate"));
		assert!(!events().contains(&"free"));
		assert_eq!(context.statistics().freed_blocks, 0);
		let stats = context.finish().unwrap();
		assert_eq!((stats.freed_blocks, stats.freed_bytes), (1, 1024));
		assert_eq!(stats.moved_allocations, trace.len());
		assert_eq!(stats.moved_bytes, trace.len() as u64 * 512);
		assert_eq!(context.finish().unwrap(), stats);
		drop(context);
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		let buffers: Vec<_> = residents
			.iter()
			.filter(|resident| {
				!resident.allocation.handle().is_null()
					&& matches!(resident.resource, DefragmentationResource::Buffer(_))
			})
			.collect();
		let images: Vec<_> = residents
			.iter()
			.filter(|resident| {
				!resident.allocation.handle().is_null()
					&& matches!(resident.resource, DefragmentationResource::Image(_))
			})
			.collect();
		assert_eq!(
			buffers[0].allocation.handle(),
			buffers[1].allocation.handle()
		);
		assert_eq!(images[0].allocation.handle(), images[1].allocation.handle());
		assert_ne!(
			buffers[0].allocation.handle(),
			images[0].allocation.handle()
		);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
	}
}

#[test]
fn extensive_finish_preserves_a_custom_pool_block_floor() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic fixed-type pool and explicit resource/pool destruction.
	unsafe {
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 1024,
				min_block_count: 3,
				max_block_count: 3,
				..Default::default()
			})
			.unwrap();
		let mut residents = mixed_layout(
			&mut allocator,
			&AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			},
		);
		clear();
		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				pool: Some(&pool),
				..info()
			},
		)
		.unwrap();
		assert!(!run(&mut context, &mut residents).is_empty());
		assert_eq!(context.finish().unwrap().freed_blocks, 0);
		drop(context);
		assert!(!events().contains(&"free"));
		assert_eq!(allocator.pool_statistics(&pool).unwrap().block_count, 3);
		cleanup(&mut allocator, &mut residents);
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn dropping_a_resolved_context_does_no_native_work_and_explicit_trim_recovers_empty_blocks() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	// SAFETY: complete mock moves precede dropping the host-only context.
	unsafe {
		let mut residents = mixed_layout(&mut allocator, &options);
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		run(&mut context, &mut residents);
		clear();
		drop(context);
		assert!(events().is_empty());
		assert!(
			allocator
				.blocks
				.iter()
				.all(|block| !block.defragmentation_hold)
		);
		assert_eq!(allocator.statistics().native_allocation_count, 3);
		assert_eq!(allocator.trim().unwrap().block_count, 1);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
	}
}

#[test]
fn ignored_evacuation_sources_are_not_repeated_and_do_not_leak_destinations() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	// SAFETY: no replacements are created; every proposal is explicitly ignored.
	unsafe {
		let mut residents = mixed_layout(&mut allocator, &options);
		let before = allocator.statistics();
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		let mut ignored = Vec::new();
		for _ in 0..8 {
			let mut pass = context.begin_pass().unwrap();
			if pass.is_empty() {
				pass.finish().unwrap();
				break;
			}
			let movement = *pass.movement(0).unwrap();
			assert!(!ignored.contains(&movement.resource));
			ignored.push(movement.resource);
			pass.ignore_move(0).unwrap();
			pass.finish().unwrap();
		}
		assert!(!ignored.is_empty());
		assert_eq!(context.finish().unwrap().ignored_allocations, ignored.len());
		drop(context);
		assert_eq!(allocator.statistics(), before);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
	}
}

#[test]
fn unit_granularity_uses_full_compaction_without_cross_class_phase_work() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	// SAFETY: synthetic queried granularity is exactly one, as in donor fallback.
	unsafe {
		let mut allocator =
			Allocator::new_with_granularity(&fixture.device, properties(), 64, 1).unwrap();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 256 },
			..Default::default()
		};
		let mut first = create(&mut allocator, 64, &options);
		let (buffer, allocation) = create(&mut allocator, 64, &options);
		destroy(&mut allocator, &mut first);
		let mut residents = vec![Resident {
			resource: DefragmentationResource::Buffer(buffer),
			allocation,
		}];
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		let trace = run(&mut context, &mut residents);
		assert_eq!(trace.len(), 1);
		assert_eq!(
			(trace[0].source.offset, trace[0].destination.offset),
			(64, 0)
		);
		assert_eq!(context.finish().unwrap().moved_allocations, 1);
		drop(context);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
	}
}

#[test]
fn zero_limits_leave_sources_unchanged_and_clear_holds_at_finish() {
	for limits in [(0, usize::MAX), (u64::MAX, 0)] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 1024 },
			..Default::default()
		};
		// SAFETY: zero-copy passes perform no native work on synthetic sources.
		unsafe {
			let mut residents = mixed_layout(&mut allocator, &options);
			let before = allocator.statistics();
			clear();
			let mut context = DefragmentationContext::new(
				&mut allocator,
				&DefragmentationInfo {
					max_bytes: limits.0,
					max_allocations: limits.1,
					..info()
				},
			)
			.unwrap();
			let mut pass = context.begin_pass().unwrap();
			assert!(pass.is_empty());
			pass.finish().unwrap();
			drop(pass);
			assert_eq!(context.finish().unwrap(), DefragmentationStats::default());
			drop(context);
			assert_eq!(allocator.statistics(), before);
			assert!(
				allocator
					.blocks
					.iter()
					.all(|block| !block.defragmentation_hold)
			);
			assert!(events().is_empty());
			cleanup(&mut allocator, &mut residents);
			allocator.close().unwrap();
		}
	}
}

#[test]
fn host_failure_before_context_or_pass_publication_is_retryable() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	// SAFETY: test-only allocation failures, no native resource accesses.
	unsafe {
		let mut residents = mixed_layout(&mut allocator, &options);
		let before = allocator.statistics();
		clear();
		FAIL_NEXT_HOST_ALLOCATION.set(true);
		let failed = DefragmentationContext::new(&mut allocator, &info());
		FAIL_NEXT_HOST_ALLOCATION.set(false);
		assert!(matches!(failed, Err(AllocError::HostAllocationFailed)));
		drop(failed);
		assert!(
			allocator
				.blocks
				.iter()
				.all(|block| !block.defragmentation_hold)
		);
		let mut context = DefragmentationContext::new(&mut allocator, &info()).unwrap();
		FAIL_NEXT_HOST_ALLOCATION.set(true);
		let failed = context.begin_pass();
		FAIL_NEXT_HOST_ALLOCATION.set(false);
		assert!(matches!(failed, Err(AllocError::HostAllocationFailed)));
		drop(failed);
		assert!(events().is_empty());
		assert_eq!(context.statistics(), DefragmentationStats::default());
		assert_eq!(context.finish().unwrap(), DefragmentationStats::default());
		drop(context);
		assert_eq!(allocator.statistics(), before);
		cleanup(&mut allocator, &mut residents);
		allocator.close().unwrap();
	}
}
