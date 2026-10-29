//! Independent move-policy and explicit-lifecycle traces against cached mock PFNs.
#[path = "defragmentation/test_extensive.rs"]
mod extensive;

use super::*;
use crate::mem::{DefragmentationAlgorithm, DefragmentationContext, DefragmentationInfo};

unsafe fn replace_buffer(
	pass: &mut crate::mem::DefragmentationPass<'_, '_>,
	index: usize,
	resource: &mut (vk::Buffer, Allocation),
) {
	// SAFETY: the fixture supplies exact mock requirements and owns all resources;
	// it simulates completion and never dereferences synthetic mapped pointers.
	unsafe {
		let replacement = pass
			.device()
			.create_buffer(
				&vk::BufferCreateInfo {
					size: resource.1.size(),
					..buffer_info()
				},
				pass.allocation_callbacks(),
			)
			.unwrap();
		pass.bind_buffer(index, replacement).unwrap();
		pass
			.device()
			.destroy_buffer(resource.0, pass.allocation_callbacks())
			.unwrap();
		pass.commit_move(index, &mut resource.1).unwrap();
		resource.0 = replacement;
	}
}

#[test]
fn cooperative_stop_is_distinct_from_completion_and_resumes_every_algorithm() {
	for algorithm in [
		DefragmentationAlgorithm::Fast,
		DefragmentationAlgorithm::Balanced,
		DefragmentationAlgorithm::Full,
		DefragmentationAlgorithm::Extensive,
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 256 },
			..Default::default()
		};
		// SAFETY: mock resources only, each pass explicitly finished/aborted.
		unsafe {
			let mut resources: Vec<_> = (0..7)
				.map(|_| create(&mut allocator, 64, &options))
				.collect();
			for resource in &mut resources[..3] {
				destroy(&mut allocator, resource);
			}
			let mut context = DefragmentationContext::new(
				&mut allocator,
				&DefragmentationInfo {
					algorithm,
					..Default::default()
				},
			)
			.unwrap();
			clear();
			let mut calls = 0;
			let mut pass = context
				.begin_pass_with_break(&mut || {
					calls += 1;
					true
				})
				.unwrap();
			assert!(pass.is_empty());
			assert!(pass.was_interrupted());
			assert_eq!(calls, 1);
			assert!(events().is_empty());
			pass.finish().unwrap();
			drop(pass);
			let mut resumed = context.begin_pass().unwrap();
			assert!(!resumed.is_empty(), "{algorithm:?}");
			assert!(!resumed.was_interrupted());
			resumed.abort().unwrap();
			drop(resumed);
			context.finish().unwrap();
			drop(context);
			destroy_all(&mut allocator, &mut resources);
		}
	}
}

#[test]
fn stop_after_one_proposal_keeps_it_resolvable_then_resumes_the_next_source() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: explicit mock replacement/commit and simulated completion.
	unsafe {
		let mut resources: Vec<_> = (0..4)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		for resource in &mut resources[..2] {
			destroy(&mut allocator, resource);
		}
		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			},
		)
		.unwrap();
		let mut calls = 0;
		let mut pass = context
			.begin_pass_with_break(&mut || {
				calls += 1;
				calls == 2
			})
			.unwrap();
		assert_eq!(pass.len(), 1);
		assert!(pass.was_interrupted());
		replace_buffer(&mut pass, 0, &mut resources[2]);
		pass.finish().unwrap();
		drop(pass);
		let mut next = context.begin_pass().unwrap();
		assert_eq!(next.len(), 1);
		replace_buffer(&mut next, 0, &mut resources[3]);
		next.finish().unwrap();
		drop(next);
		assert_eq!(context.finish().unwrap().moved_allocations, 2);
		drop(context);
		assert_eq!((resources[2].1.offset(), resources[3].1.offset()), (0, 64));
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn panicking_stop_predicate_rolls_back_unpublished_mapped_destinations() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		persistent_map: true,
		..Default::default()
	};
	// SAFETY: synthetic pointers are never dereferenced; the unwound pass was unpublished.
	unsafe {
		let mut resources: Vec<_> = (0..4)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		for resource in &mut resources[..2] {
			destroy(&mut allocator, resource);
		}
		let before = allocator.statistics();
		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			},
		)
		.unwrap();
		let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
			let mut calls = 0;
			let _ = context.begin_pass_with_break(&mut || {
				calls += 1;
				assert_ne!(calls, 2, "stop panic");
				false
			});
		}));
		assert!(result.is_err());
		assert_eq!(context.statistics(), Default::default());
		let mut retry = context.begin_pass().unwrap();
		assert_eq!(retry.len(), 2);
		assert_eq!(retry.movement(0).unwrap().destination.offset, 0);
		retry.abort().unwrap();
		drop(retry);
		context.finish().unwrap();
		drop(context);
		assert_eq!(allocator.statistics(), before);
		assert_eq!(allocator.blocks[0].mapping.references, 2);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn raw_storage_commits_without_an_invented_resource_or_binding() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	let requirements = vk::MemoryRequirements {
		size: 64,
		alignment: 64,
		memory_type_bits: 1,
	};
	// SAFETY: no external resource bindings or consumers exist for these raw regions.
	unsafe {
		let mut first = allocator
			.allocate_with_options(&requirements, &options)
			.unwrap();
		let mut source = allocator
			.allocate_with_options(&requirements, &options)
			.unwrap();
		allocator.free(&mut first).unwrap();
		allocator.set_allocation_name(&source, Some("raw")).unwrap();
		allocator.set_allocation_user_data(&source, 73).unwrap();
		clear();
		let mut pass = crate::mem::begin_defragmentation_with_break(
			&mut allocator,
			&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			},
			&mut || false,
		)
		.unwrap();
		assert_eq!(pass.len(), 1);
		assert_eq!(
			pass.movement(0).unwrap().resource,
			crate::mem::DefragmentationResource::Unassociated
		);
		pass.commit_move(0, &mut source).unwrap();
		pass.finish().unwrap();
		drop(pass);
		assert!(events().is_empty());
		assert_eq!(source.offset(), 0);
		assert_eq!(allocator.allocation_name(&source).unwrap(), Some("raw"));
		assert_eq!(allocator.allocation_info(&source).unwrap().user_data, 73);
		assert_eq!(
			allocator.validate(&source).unwrap().class,
			ResourceClass::Unknown
		);
		assert!(!allocator.validate(&source).unwrap().bound);
		allocator.free(&mut source).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn raw_destination_can_adopt_a_checked_buffer_or_image_binding() {
	for image in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 256 },
			..Default::default()
		};
		let requirements = vk::MemoryRequirements {
			size: 64,
			alignment: 64,
			memory_type_bits: 1,
		};
		// SAFETY: no old resource, matching mock replacement requirements and no GPU uses.
		unsafe {
			let mut first = allocator
				.allocate_with_options(&requirements, &options)
				.unwrap();
			let mut source = allocator
				.allocate_with_options(&requirements, &options)
				.unwrap();
			allocator.free(&mut first).unwrap();
			STATE.with_borrow_mut(|state| state.requirements_size = Some(64));
			let mut pass = allocator
				.begin_defragmentation(&DefragmentationInfo {
					algorithm: DefragmentationAlgorithm::Full,
					..Default::default()
				})
				.unwrap();
			let buffer = if !image {
				let buffer = pass.device().create_buffer(&buffer_info(), None).unwrap();
				pass.bind_buffer(0, buffer).unwrap();
				Some(buffer)
			} else {
				None
			};
			let image = if image {
				let image = pass
					.device()
					.create_image(&super::image::info(vk::ImageTiling::OPTIMAL), None)
					.unwrap();
				pass.bind_image(0, image).unwrap();
				Some(image)
			} else {
				None
			};
			pass.commit_move(0, &mut source).unwrap();
			pass.finish().unwrap();
			drop(pass);
			assert_eq!(
				allocator.validate(&source).unwrap().class,
				ResourceClass::Unknown
			);
			if let Some(buffer) = buffer {
				allocator.destroy_buffer(buffer, &mut source).unwrap();
			}
			if let Some(image) = image {
				allocator.destroy_image(image, &mut source).unwrap();
			}
			allocator.close().unwrap();
		}
	}
}

#[test]
fn mapped_move_preserves_explicit_and_persistent_holds_at_the_new_offset() {
	for persistent in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 256 },
			persistent_map: persistent,
			..Default::default()
		};
		// SAFETY: counted mock mappings; no synthetic pointer is dereferenced.
		unsafe {
			let mut resources: Vec<_> = (0..2)
				.map(|_| create(&mut allocator, 64, &options))
				.collect();
			destroy(&mut allocator, &mut resources[0]);
			let old_pointer = allocator.map(&resources[1].1).unwrap();
			assert_eq!(allocator.map(&resources[1].1).unwrap(), old_pointer);
			clear();
			let mut pass = allocator
				.begin_defragmentation(&DefragmentationInfo {
					algorithm: DefragmentationAlgorithm::Full,
					..Default::default()
				})
				.unwrap();
			let proposal = *pass.movement(0).unwrap();
			assert_eq!(
				(proposal.source.map_count, proposal.destination.map_count),
				(2, 2)
			);
			assert_eq!(proposal.source.mapped_pointer, Some(old_pointer));
			assert_ne!(proposal.destination.mapped_pointer, Some(old_pointer));
			replace_buffer(&mut pass, 0, &mut resources[1]);
			pass.finish().unwrap();
			drop(pass);
			let moved = &resources[1].1;
			assert_eq!(
				allocator.mapped_pointer(moved).unwrap(),
				proposal.destination.mapped_pointer
			);
			assert_eq!(
				allocator.blocks[0].mapping.references,
				2 + u32::from(persistent)
			);
			assert!(!events().contains(&"map"));
			assert!(!events().contains(&"unmap"));
			assert_eq!(
				allocator.free(&mut resources[1].1),
				Err(AllocError::AlreadyMapped)
			);
			allocator.unmap(&resources[1].1).unwrap();
			allocator.unmap(&resources[1].1).unwrap();
			assert_eq!(
				allocator.mapped_pointer(&resources[1].1).unwrap().is_some(),
				persistent
			);
			destroy_all(&mut allocator, &mut resources);
			assert_eq!(
				events().iter().filter(|&&event| event == "unmap").count(),
				1
			);
		}
	}
}

#[test]
fn mapped_ignore_and_destroy_release_exactly_their_owned_holds() {
	for destroy_source in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 256 },
			persistent_map: true,
			..Default::default()
		};
		// SAFETY: mock pointers have no uses; destroy action explicitly retires the old object.
		unsafe {
			let mut resources: Vec<_> = (0..2)
				.map(|_| create(&mut allocator, 64, &options))
				.collect();
			destroy(&mut allocator, &mut resources[0]);
			let old = allocator.map(&resources[1].1).unwrap();
			let mut pass = allocator
				.begin_defragmentation(&DefragmentationInfo {
					algorithm: DefragmentationAlgorithm::Full,
					..Default::default()
				})
				.unwrap();
			if destroy_source {
				pass.device().destroy_buffer(resources[1].0, None).unwrap();
				pass.destroy_move(0, &mut resources[1].1).unwrap();
			} else {
				pass.ignore_move(0).unwrap();
			}
			let stats = pass.finish().unwrap();
			drop(pass);
			if destroy_source {
				assert_eq!(stats.destroyed_allocations, 1);
				assert!(resources[1].1.is_freed());
			} else {
				assert_eq!(stats.ignored_allocations, 1);
				assert_eq!(allocator.blocks[0].mapping.references, 2);
				assert_eq!(
					allocator.mapped_pointer(&resources[1].1).unwrap(),
					Some(old)
				);
				allocator.unmap(&resources[1].1).unwrap();
			}
			destroy_all(&mut allocator, &mut resources);
		}
	}
}

#[test]
fn relocation_guard_metadata_survives_and_failed_destroy_keeps_both_regions_live() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut backing = Box::new([0_u8; 256]);
	STATE.with_borrow_mut(|state| state.mapping_pointer = Some(backing.as_mut_ptr() as usize));
	let mut properties = properties();
	properties.memory_types[0].property_flags =
		vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	// SAFETY: the mock mapping is a real stable 256-byte host buffer, with the
	// exact coherent native extent used below; all references end before Drop.
	let mut allocator =
		unsafe { Allocator::new_with_granularity(&fixture.device, properties, 64, 64) }.unwrap();
	allocator.set_mapping_hysteresis(false).unwrap();
	allocator
		.set_debug_memory_settings(crate::mem::DebugMemorySettings {
			margin: 16,
			..Default::default()
		})
		.unwrap();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: no GPU uses; test writes only separately owned initialized guard bytes.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		assert_eq!(resources[1].1.offset(), 128);
		allocator.map(&resources[1].1).unwrap();
		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			})
			.unwrap();
		assert_eq!(pass.movement(0).unwrap().destination.offset, 0);
		assert_eq!(
			u32::from_ne_bytes(backing[64..68].try_into().unwrap()),
			0x7F84_E666
		);
		// Destroy is allowed only after resource retirement. A corrupt source
		// must not discard the destination or either region's counted holds.
		pass.device().destroy_buffer(resources[1].0, None).unwrap();
		backing[192] ^= 1;
		assert!(matches!(
			pass.destroy_move(0, &mut resources[1].1),
			Err(AllocError::CorruptionDetected { .. })
		));
		backing[192] ^= 1;
		assert_eq!(pass.finish(), Err(AllocError::LiveAllocations));
		assert!(!resources[1].1.is_freed());
		// The old mock buffer is already destroyed. Commit a checked replacement;
		// no actual contents are needed for this guard/lifecycle oracle.
		let new = pass
			.device()
			.create_buffer(
				&vk::BufferCreateInfo {
					size: 64,
					..buffer_info()
				},
				None,
			)
			.unwrap();
		pass.bind_buffer(0, new).unwrap();
		pass.commit_move(0, &mut resources[1].1).unwrap();
		resources[1].0 = new;
		pass.finish().unwrap();
		drop(pass);
		assert_eq!(allocator.blocks[0].mapping.references, 1);
		assert_eq!(allocator.validate(&resources[1].1).unwrap().guard_size, 16);
		assert_eq!(
			allocator.validate(&resources[1].1).unwrap().guard_offset,
			64
		);
		backing[64] ^= 1;
		assert!(matches!(
			allocator.check_corruption(1),
			Err(AllocError::CorruptionDetected { .. })
		));
		backing[64] ^= 1;
		allocator.check_corruption(1).unwrap();
		allocator.unmap(&resources[1].1).unwrap();
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn destination_reference_overflow_is_atomic_and_does_not_drop_source_holds() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: synthetic mappings, no pointer dereferences. Directly seed the
	// valid maximum count instead of making billions of identical mock maps.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		allocator.map(&resources[1].1).unwrap();
		allocator
			.live
			.get_mut(&resources[1].1.serial)
			.unwrap()
			.map_count = u32::MAX;
		allocator.blocks[0].mapping.references = u32::MAX;
		let before = allocator.statistics();
		clear();
		assert!(matches!(
			allocator.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			}),
			Err(AllocError::InvalidRange)
		));
		assert_eq!(allocator.statistics(), before);
		assert_eq!(
			allocator
				.allocation_info(&resources[1].1)
				.unwrap()
				.map_count,
			u32::MAX
		);
		assert_eq!(allocator.blocks[0].mapping.references, u32::MAX);
		assert!(events().is_empty());
		allocator
			.live
			.get_mut(&resources[1].1.serial)
			.unwrap()
			.map_count = 1;
		allocator.blocks[0].mapping.references = 1;
		allocator.unmap(&resources[1].1).unwrap();
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn destination_map_failure_retains_source_counts_pointer_and_all_placements() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: source is the only mapped native block; destination map failure precedes publication.
	unsafe {
		let mut resources: Vec<_> = (0..7)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		for resource in &mut resources[..3] {
			destroy(&mut allocator, resource);
		}
		let pointer = allocator.map(&resources[3].1).unwrap();
		let before = allocator.statistics();
		STATE.with_borrow_mut(|state| state.fail = Some("map"));
		assert!(matches!(
			allocator.begin_defragmentation(&DefragmentationInfo::default()),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_DEVICE_LOST
			)))
		));
		STATE.with_borrow_mut(|state| state.fail = None);
		assert_eq!(allocator.statistics(), before);
		assert_eq!(
			allocator
				.allocation_info(&resources[3].1)
				.unwrap()
				.map_count,
			1
		);
		assert_eq!(
			allocator.mapped_pointer(&resources[3].1).unwrap(),
			Some(pointer)
		);
		let mut retry = allocator
			.begin_defragmentation(&DefragmentationInfo::default())
			.unwrap();
		assert_eq!(retry.len(), 1);
		retry.abort().unwrap();
		drop(retry);
		assert_eq!(allocator.statistics(), before);
		allocator.unmap(&resources[3].1).unwrap();
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn alias_group_recreation_uses_checked_buffer_and_image_services() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		can_alias: true,
		..Default::default()
	};
	// SAFETY: aliases are synthetic, have no GPU uses, and every old/new object is retired explicitly.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		let alias_info = vk::ImageCreateInfo {
			flags: vk::ImageCreateFlagBits::ALIAS_BIT,
			..super::image::info(vk::ImageTiling::OPTIMAL)
		};
		let alias_buffer = allocator
			.create_aliasing_buffer(&resources[1].1, 0, &buffer_info())
			.unwrap();
		let alias_image = allocator
			.create_aliasing_image(&resources[1].1, 0, &alias_info)
			.unwrap();
		destroy(&mut allocator, &mut resources[0]);
		let native_count = allocator.statistics().native_allocation_count;
		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			})
			.unwrap();
		assert!(pass.movement(0).unwrap().can_alias);
		let new_alias_buffer = pass.create_aliasing_buffer(0, 0, &buffer_info()).unwrap();
		let new_alias_image = pass.create_aliasing_image(0, 0, &alias_info).unwrap();
		pass
			.device()
			.destroy_buffer(alias_buffer, pass.allocation_callbacks())
			.unwrap();
		pass
			.device()
			.destroy_image(alias_image, pass.allocation_callbacks())
			.unwrap();
		replace_buffer(&mut pass, 0, &mut resources[1]);
		pass.finish().unwrap();
		drop(pass);
		assert_eq!(allocator.statistics().native_allocation_count, native_count);
		assert_eq!(
			allocator.validate(&resources[1].1).unwrap().class,
			ResourceClass::Unknown
		);
		assert!(allocator.validate(&resources[1].1).unwrap().can_alias);
		let extra = allocator
			.create_aliasing_buffer(&resources[1].1, 0, &buffer_info())
			.unwrap();
		allocator
			.device
			.destroy_buffer(extra, allocator.callbacks)
			.unwrap();
		allocator
			.device
			.destroy_buffer(new_alias_buffer, allocator.callbacks)
			.unwrap();
		allocator
			.device
			.destroy_image(new_alias_image, allocator.callbacks)
			.unwrap();
		destroy_all(&mut allocator, &mut resources);
	}
}

fn configure() {
	STATE.with_borrow_mut(|state| state.pooled = true);
}

unsafe fn create(
	allocator: &mut Allocator<'_>,
	size: u64,
	options: &AllocationCreateInfo,
) -> (vk::Buffer, Allocation) {
	STATE.with_borrow_mut(|state| state.requirements_size = Some(size));
	// SAFETY: synthetic ordinary requirements and matching cached mock PFNs.
	unsafe {
		allocator.create_buffer_with_options(
			&vk::BufferCreateInfo {
				size,
				..buffer_info()
			},
			options,
		)
	}
	.unwrap()
}

unsafe fn destroy(allocator: &mut Allocator<'_>, resource: &mut (vk::Buffer, Allocation)) {
	// SAFETY: matching mock pair, no host/GPU use or outstanding mappings.
	unsafe { allocator.destroy_buffer(resource.0, &mut resource.1) }.unwrap();
}

unsafe fn destroy_all(allocator: &mut Allocator<'_>, resources: &mut [(vk::Buffer, Allocation)]) {
	for resource in resources {
		if !resource.1.handle().is_null() {
			// SAFETY: every still-live mock pair is owned by this test.
			unsafe { destroy(allocator, resource) };
		}
	}
	allocator.close().unwrap();
}

#[test]
fn fast_consolidates_by_free_bytes_instead_of_block_creation_order() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: synthetic resources, all proposals explicitly aborted before cleanup.
	unsafe {
		let mut resources: Vec<_> = (0..7)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		for resource in &mut resources[..3] {
			destroy(&mut allocator, resource);
		}
		let sparse = resources[3].1.handle();
		let fuller = resources[4].1.handle();
		clear();
		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Fast,
				max_allocations: 1,
				..Default::default()
			})
			.unwrap();
		assert_eq!(pass.len(), 1);
		let movement = pass.movement(0).unwrap();
		assert_eq!(
			movement.resource,
			crate::mem::DefragmentationResource::Buffer(resources[3].0)
		);
		assert_eq!(movement.source.device_memory, sparse);
		assert_eq!(movement.destination.device_memory, fuller);
		assert_eq!(movement.destination.offset, 192);
		assert!(events().is_empty());
		assert_eq!(pass.abort().unwrap().ignored_allocations, 1);
		drop(pass);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn full_compacts_one_block_while_fast_leaves_the_same_hole_untouched() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: mock placements have no real consumers; every pass is resolved.
	unsafe {
		let mut resources: Vec<_> = (0..3)
			.map(|_| create(&mut allocator, 65, &options))
			.collect();
		// Noncoherent atom padding makes each 65-byte request occupy 128 bytes.
		assert_eq!(resources[0].1.reserved_size(), 128);
		destroy(&mut allocator, &mut resources[0]);
		// The later block is pinned so this compares only same-block placement.
		allocator
			.set_allocation_pinned(&resources[2].1, true)
			.unwrap();
		let mut fast = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Fast,
				..Default::default()
			})
			.unwrap();
		assert!(fast.is_empty());
		fast.finish().unwrap();
		drop(fast);
		let before = allocator.statistics();
		let mut full = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			})
			.unwrap();
		assert_eq!(full.len(), 1);
		let movement = full.movement(0).unwrap();
		assert_eq!(
			movement.source.device_memory,
			movement.destination.device_memory
		);
		assert_eq!(
			(movement.source.offset, movement.destination.offset),
			(128, 0)
		);
		assert_eq!(
			(
				movement.destination.size,
				movement.destination.reserved_size
			),
			(65, 128)
		);
		full.abort().unwrap();
		drop(full);
		assert_eq!(allocator.statistics(), before);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn balanced_avoids_a_large_isolated_copy_that_full_can_propose() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	assert_eq!(
		DefragmentationInfo::default().algorithm,
		DefragmentationAlgorithm::Balanced
	);
	// SAFETY: complete synthetic placement layout, explicit pass rollback/cleanup.
	unsafe {
		let mut resources: Vec<_> = [256, 64, 256, 64, 64, 64, 64, 64]
			.into_iter()
			.map(|size| create(&mut allocator, size, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		for (index, (_, allocation)) in resources.iter().enumerate().skip(1) {
			if index != 2 {
				allocator.set_allocation_pinned(allocation, true).unwrap();
			}
		}
		// Free spans: 256 and 128, average 192. Seven allocations average <256.
		// Neither immediate neighbor of the selected 256-byte allocation is free.
		let mut balanced = allocator
			.begin_defragmentation(&DefragmentationInfo::default())
			.unwrap();
		assert!(balanced.is_empty());
		balanced.finish().unwrap();
		drop(balanced);
		let mut full = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			})
			.unwrap();
		assert_eq!(full.len(), 1);
		let movement = full.movement(0).unwrap();
		assert_eq!(
			(movement.source.offset, movement.destination.offset),
			(320, 0)
		);
		full.abort().unwrap();
		drop(full);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn full_preserves_each_explicit_block_size_and_custom_pool_domain() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	// SAFETY: independent mock domains; native lifetime cleanup is explicit.
	unsafe {
		let mut pool = allocator
			.create_pool(&crate::mem::PoolCreateInfo {
				block_size: 256,
				..Default::default()
			})
			.unwrap();
		let mut resources = Vec::new();
		for options in [
			AllocationCreateInfo {
				policy: AllocationPolicy::Pooled { block_size: 256 },
				..Default::default()
			},
			AllocationCreateInfo {
				policy: AllocationPolicy::Pooled { block_size: 512 },
				..Default::default()
			},
			AllocationCreateInfo {
				pool: Some(&pool),
				..Default::default()
			},
		] {
			resources.push(create(&mut allocator, 64, &options));
			resources.push(create(&mut allocator, 64, &options));
		}
		for index in [0, 2, 4] {
			destroy(&mut allocator, &mut resources[index]);
		}
		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				..Default::default()
			})
			.unwrap();
		assert_eq!(pass.len(), 3);
		for index in 0..pass.len() {
			let movement = pass.movement(index).unwrap();
			assert_eq!(
				movement.source.device_memory,
				movement.destination.device_memory
			);
			assert_eq!(
				movement.source.native_size,
				movement.destination.native_size
			);
			assert_eq!(
				(movement.source.offset, movement.destination.offset),
				(64, 0)
			);
		}
		pass.abort().unwrap();
		drop(pass);
		for resource in &mut resources {
			if !resource.1.handle().is_null() {
				destroy(&mut allocator, resource);
			}
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
}

#[test]
fn context_retains_ignored_blocks_as_destinations_and_stops_proposing_their_sources() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: proposals never bind or copy; ignored reservations are explicitly freed.
	unsafe {
		let mut resources: Vec<_> = (0..3)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				max_allocations: 1,
				..Default::default()
			},
		)
		.unwrap();
		let mut first = context.begin_pass().unwrap();
		assert_eq!(first.len(), 1);
		first.ignore_move(0).unwrap();
		first.finish().unwrap();
		drop(first);
		let mut next = context.begin_pass().unwrap();
		assert!(next.is_empty());
		next.finish().unwrap();
		drop(next);
		assert_eq!(context.ignored_allocations(), 1);
		assert_eq!(context.finish().unwrap().ignored_allocations, 1);
		drop(context);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn committed_replacement_retains_metadata_and_context_does_not_move_backwards() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: mock copies have no GPU work; binding/destruction/commit are explicit.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		allocator
			.set_allocation_name(&resources[1].1, Some("retained"))
			.unwrap();
		allocator
			.set_allocation_user_data(&resources[1].1, 79)
			.unwrap();
		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				algorithm: DefragmentationAlgorithm::Full,
				max_allocations: 1,
				..Default::default()
			},
		)
		.unwrap();
		let mut pass = context.begin_pass().unwrap();
		assert_eq!(pass.len(), 1);
		let replacement = pass
			.device()
			.create_buffer(
				&vk::BufferCreateInfo {
					size: 64,
					..buffer_info()
				},
				None,
			)
			.unwrap();
		pass.bind_buffer(0, replacement).unwrap();
		pass.device().destroy_buffer(resources[1].0, None).unwrap();
		pass.commit_move(0, &mut resources[1].1).unwrap();
		resources[1].0 = replacement;
		let stats = pass.finish().unwrap();
		assert_eq!((stats.moved_allocations, stats.moved_bytes), (1, 64));
		drop(pass);
		let mut next = context.begin_pass().unwrap();
		assert!(next.is_empty());
		next.finish().unwrap();
		drop(next);
		assert_eq!(context.finish().unwrap(), stats);
		drop(context);
		assert_eq!(resources[1].1.offset(), 0);
		assert_eq!(
			allocator.allocation_name(&resources[1].1).unwrap(),
			Some("retained")
		);
		assert_eq!(
			allocator
				.allocation_info(&resources[1].1)
				.unwrap()
				.user_data,
			79
		);
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn zero_copy_limits_leave_all_metadata_and_native_effects_unchanged() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: explicit mock cleanup, no reservations should pass either limit.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		let before = allocator.statistics();
		clear();
		for info in [
			DefragmentationInfo {
				max_bytes: 0,
				..Default::default()
			},
			DefragmentationInfo {
				max_allocations: 0,
				..Default::default()
			},
		] {
			let mut pass = allocator
				.begin_defragmentation_with_break(&info, &mut || {
					panic!("zero copy limits must not poll the stop predicate")
				})
				.unwrap();
			assert!(pass.is_empty());
			pass.finish().unwrap();
		}
		assert_eq!(allocator.statistics(), before);
		assert!(events().is_empty());
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn host_metadata_failure_precedes_any_destination_or_native_effect() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let mut allocator = fixture.allocator();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 256 },
		..Default::default()
	};
	// SAFETY: test-only host failure occurs at the initial scheduling snapshot.
	unsafe {
		let mut resources: Vec<_> = (0..2)
			.map(|_| create(&mut allocator, 64, &options))
			.collect();
		destroy(&mut allocator, &mut resources[0]);
		let before = allocator.statistics();
		clear();
		FAIL_NEXT_HOST_ALLOCATION.set(true);
		let result = allocator.begin_defragmentation(&DefragmentationInfo::default());
		FAIL_NEXT_HOST_ALLOCATION.set(false);
		assert!(matches!(result, Err(AllocError::HostAllocationFailed)));
		assert_eq!(allocator.statistics(), before);
		assert!(events().is_empty());
		destroy_all(&mut allocator, &mut resources);
	}
}

#[test]
fn consecutive_oversized_sources_stop_at_the_donor_sixteen_source_limit() {
	for oversized in [15, 16] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		configure();
		let mut allocator = fixture.allocator();
		let options = AllocationCreateInfo {
			policy: AllocationPolicy::Pooled { block_size: 4096 },
			..Default::default()
		};
		// SAFETY: synthetic layout; every proposal is aborted, every resource destroyed.
		unsafe {
			let mut resources: Vec<_> = (0..=oversized)
				.map(|_| create(&mut allocator, 128, &options))
				.collect();
			resources.push(create(&mut allocator, 64, &options));
			destroy(&mut allocator, &mut resources[0]);
			let mut pass = allocator
				.begin_defragmentation(&DefragmentationInfo {
					algorithm: DefragmentationAlgorithm::Full,
					max_bytes: 64,
					..Default::default()
				})
				.unwrap();
			assert_eq!(pass.len(), usize::from(oversized == 15));
			if !pass.is_empty() {
				assert_eq!(pass.movement(0).unwrap().source.size, 64);
			}
			pass.abort().unwrap();
			drop(pass);
			destroy_all(&mut allocator, &mut resources);
		}
	}
}

#[test]
fn full_respects_live_buffer_image_pages_before_accepting_a_lower_image_offset() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	configure();
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Pooled { block_size: 1024 },
		..Default::default()
	};
	// SAFETY: mock properties use 256-byte granularity, no native consumer work.
	unsafe {
		let mut allocator =
			Allocator::new_with_granularity(&fixture.device, properties(), 64, 256).unwrap();
		let mut first = create(&mut allocator, 64, &options);
		let mut second = create(&mut allocator, 64, &options);
		let (image, mut allocation) = allocator
			.create_image_with_options(&image::info(vk::ImageTiling::OPTIMAL), &options)
			.unwrap();
		assert_eq!((second.1.offset(), allocation.offset()), (64, 256));
		destroy(&mut allocator, &mut first);
		allocator.set_allocation_pinned(&second.1, true).unwrap();
		let info = DefragmentationInfo {
			algorithm: DefragmentationAlgorithm::Full,
			..Default::default()
		};
		let mut blocked = allocator.begin_defragmentation(&info).unwrap();
		assert!(blocked.is_empty());
		blocked.finish().unwrap();
		drop(blocked);
		destroy(&mut allocator, &mut second);
		let mut available = allocator.begin_defragmentation(&info).unwrap();
		assert_eq!(available.len(), 1);
		let movement = available.movement(0).unwrap();
		assert_eq!(
			movement.resource,
			crate::mem::DefragmentationResource::Image(image)
		);
		assert_eq!(
			(movement.source.offset, movement.destination.offset),
			(256, 0)
		);
		available.abort().unwrap();
		drop(available);
		allocator.destroy_image(image, &mut allocation).unwrap();
		allocator.close().unwrap();
	}
}
