//! Native resource-policy contracts following pinned VMA memory preferences,
//! dedicated allocation and explicit requirements/binding behavior.
use super::*;
use crate::mem::{AllocationCreateInfo, AllocationPolicy, HostAccess, MemoryUsage};

fn memory_properties(unified: bool) -> vk::PhysicalDeviceMemoryProperties {
	use vk::MemoryPropertyFlagBits as Flag;
	let mut properties = properties();
	properties.memory_heap_count = 2;
	properties.memory_heaps[1].size = 1 << 30;
	properties.memory_type_count = 3 + u32::from(unified);
	properties.memory_types[0] = vk::MemoryType {
		property_flags: Flag::DEVICE_LOCAL_BIT,
		heap_index: 0,
	};
	properties.memory_types[1] = vk::MemoryType {
		property_flags: Flag::HOST_VISIBLE_BIT | Flag::HOST_COHERENT_BIT,
		heap_index: 1,
	};
	properties.memory_types[2] = vk::MemoryType {
		property_flags: Flag::HOST_VISIBLE_BIT | Flag::HOST_CACHED_BIT,
		heap_index: 1,
	};
	properties.memory_types[3] = vk::MemoryType {
		property_flags: Flag::DEVICE_LOCAL_BIT | Flag::HOST_VISIBLE_BIT | Flag::HOST_CACHED_BIT,
		heap_index: 0,
	};
	properties
}

fn allocator(fixture: &Fixture, unified: bool) -> Allocator<'_> {
	STATE.with_borrow_mut(|s| {
		s.pooled = true;
		s.requirements_bits = Some(0xf);
	});
	// SAFETY: synthetic memory types match the exact-PFN driver's admitted storage.
	unsafe { Allocator::new(&fixture.device, memory_properties(unified), 64) }.unwrap()
}

fn pooled() -> AllocationPolicy {
	AllocationPolicy::Pooled { block_size: 4096 }
}

#[test]
fn random_host_storage_preserves_required_visibility_and_uses_available_locality() {
	use vk::MemoryPropertyFlagBits as Flag;
	for unified in [false, true] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = allocator(&fixture, unified);
		let options = AllocationCreateInfo {
			usage: MemoryUsage::PreferDevice,
			host_access: HostAccess::Random,
			required_flags: Flag::HOST_VISIBLE_BIT,
			preferred_flags: Flag::HOST_CACHED_BIT | Flag::DEVICE_LOCAL_BIT,
			policy: pooled(),
			..Default::default()
		};
		// SAFETY: independent mock native buffer pairs, no actual CPU/GPU use.
		unsafe {
			let (a, mut x) = allocator
				.create_buffer_with_options(&buffer_info(), &options)
				.unwrap();
			let (b, mut y) = allocator
				.create_buffer_with_options(&buffer_info(), &options)
				.unwrap();
			let info = allocator.allocation_info(&y).unwrap();
			assert_eq!(info.memory_type_index, if unified { 3 } else { 2 });
			assert_eq!(info.heap_index, u32::from(!unified));
			assert_eq!(
				info.memory_properties,
				memory_properties(unified).memory_types[info.memory_type_index as usize].property_flags
			);
			assert!(x.is_host_visible());
			assert_eq!(x.handle(), y.handle());
			assert!(y.offset() >= x.offset() + x.reserved_size());
			allocator.map(&y).unwrap();
			allocator.flush(&y, 1, 255).unwrap();
			allocator.invalidate(&y, 1, 255).unwrap();
			assert_eq!(
				STATE.with_borrow(|s| s.ranges.clone()),
				[(320, 256), (320, 256)]
			);
			allocator.unmap(&y).unwrap();
			allocator.destroy_buffer(a, &mut x).unwrap();
			allocator.destroy_buffer(b, &mut y).unwrap();
			allocator.close().unwrap();
		}
	}
}

#[test]
fn sequential_persistent_upload_and_random_readback_choose_distinct_host_policies() {
	for (host_access, persistent_map, expected_type) in [
		(HostAccess::SequentialWrite, true, 1),
		(HostAccess::Random, false, 2),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = allocator(&fixture, false);
		let options = AllocationCreateInfo {
			usage: if persistent_map {
				MemoryUsage::PreferDevice
			} else {
				MemoryUsage::PreferHost
			},
			host_access,
			persistent_map,
			policy: pooled(),
			..Default::default()
		};
		// SAFETY: pointers are synthetic and compared only; explicitly matched cleanup.
		unsafe {
			let (buffer, mut allocation) = allocator
				.create_buffer_with_options(&buffer_info(), &options)
				.unwrap();
			let info = allocator.allocation_info(&allocation).unwrap();
			assert_eq!(info.memory_type_index, expected_type);
			assert_eq!(info.mapped_pointer.is_some(), persistent_map);
			let pointer = if persistent_map {
				info.mapped_pointer.unwrap()
			} else {
				allocator.map(&allocation).unwrap()
			};
			assert_eq!(
				allocator
					.allocation_info(&allocation)
					.unwrap()
					.mapped_pointer,
				Some(pointer)
			);
			allocator
				.invalidate(&allocation, 0, vk::WHOLE_SIZE)
				.unwrap();
			if !persistent_map {
				allocator.unmap(&allocation).unwrap();
			}
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			assert_eq!(allocator.statistics(), AllocatorStats::default());
			allocator.close().unwrap();
		}
	}
}

#[test]
fn separate_device_and_host_storage_never_join_an_ordinary_pool() {
	use vk::MemoryPropertyFlagBits as Flag;
	for (version, extensions) in [
		(vk::API_VERSION_1_0, vec![]),
		(vk::API_VERSION_1_1, vec![]),
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_get_memory_requirements2",
				c"VK_KHR_dedicated_allocation",
			],
		),
	] {
		let fixture = Fixture::new(version, &extensions, true, None);
		let mut allocator = allocator(&fixture, false);
		let options = AllocationCreateInfo {
			usage: MemoryUsage::PreferDevice,
			required_flags: Flag::DEVICE_LOCAL_BIT,
			policy: pooled(),
			..Default::default()
		};
		// SAFETY: isolated fake resources with no pending accesses; complete native cleanup.
		unsafe {
			let (ordinary, mut shared) = allocator
				.create_buffer_with_options(&buffer_info(), &options)
				.unwrap();
			let mut separate = Vec::new();
			for host in [false, false, true] {
				let options = AllocationCreateInfo {
					host_access: if host {
						HostAccess::SequentialWrite
					} else {
						HostAccess::None
					},
					required_flags: if host {
						Flag::HOST_VISIBLE_BIT
					} else {
						Flag::DEVICE_LOCAL_BIT
					},
					policy: AllocationPolicy::Separate,
					..options
				};
				let (buffer, allocation) = allocator
					.create_buffer_with_options(&buffer_info(), &options)
					.unwrap();
				assert_ne!(allocation.handle(), shared.handle());
				assert!(
					separate
						.iter()
						.all(|(_, other): &(vk::Buffer, Allocation)| other.handle() != allocation.handle())
				);
				assert_eq!(allocation.offset(), 0);
				assert_eq!(
					allocator
						.allocation_info(&allocation)
						.unwrap()
						.mapped_pointer,
					None
				);
				if !host {
					assert_eq!(allocator.map(&allocation), Err(AllocError::NotHostVisible));
				}
				if version != vk::API_VERSION_1_0 || !extensions.is_empty() {
					assert_eq!(STATE.with_borrow(|s| s.dedicated), buffer);
				} else {
					assert!(STATE.with_borrow(|s| s.dedicated).is_null());
				}
				separate.push((buffer, allocation));
			}
			assert_eq!(allocator.statistics().pooled_block_count, 1);
			for (buffer, mut allocation) in separate {
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			}
			allocator.destroy_buffer(ordinary, &mut shared).unwrap();
			assert_eq!(allocator.statistics(), AllocatorStats::default());
			allocator.close().unwrap();
		}
	}
}

#[test]
fn device_local_render_images_share_only_compatible_image_storage() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = allocator(&fixture, false);
	let options = AllocationCreateInfo {
		usage: MemoryUsage::PreferDevice,
		required_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
		policy: pooled(),
		..Default::default()
	};
	let image_info = vk::ImageCreateInfo {
		image_type: vk::ImageType::_2D,
		format: vk::Format::R8G8B8A8_UNORM,
		extent: vk::Extent3D {
			width: 7,
			height: 9,
			depth: 1,
		},
		mip_levels: 1,
		array_layers: 1,
		samples: vk::SampleCountFlagBits::_1_BIT,
		tiling: vk::ImageTiling::OPTIMAL,
		usage: vk::ImageUsageFlagBits::SAMPLED_BIT | vk::ImageUsageFlagBits::TRANSFER_DST_BIT,
		..Default::default()
	};
	// SAFETY: ordinary synthetic images/buffer; matching explicit destruction, no views/work.
	unsafe {
		let (buffer, mut buffer_allocation) = allocator
			.create_buffer_with_options(&buffer_info(), &options)
			.unwrap();
		let (first, mut x) = allocator
			.create_image_with_options(&image_info, &options)
			.unwrap();
		let (second, mut y) = allocator
			.create_image_with_options(&image_info, &options)
			.unwrap();
		let info = allocator.allocation_info(&y).unwrap();
		assert_eq!(info.memory_type_index, 0);
		assert_eq!(
			info.memory_properties,
			vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT
		);
		assert_eq!(info.mapped_pointer, None);
		assert_eq!(x.handle(), y.handle());
		assert_ne!(x.handle(), buffer_allocation.handle());
		assert!(y.offset() >= x.offset() + x.reserved_size());
		assert_eq!(allocator.statistics().pooled_block_count, 2);
		allocator.destroy_image(first, &mut x).unwrap();
		allocator.destroy_image(second, &mut y).unwrap();
		allocator
			.destroy_buffer(buffer, &mut buffer_allocation)
			.unwrap();
		assert_eq!(allocator.statistics(), AllocatorStats::default());
		allocator.close().unwrap();
	}
}

#[test]
fn raw_session_storage_preserves_native_mask_alignment_and_explicit_binding_metadata() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = allocator(&fixture, false);
	// SAFETY: stand-in ordinary session requirements; no codec execution or GPU binding.
	unsafe {
		for bits in [1, 4] {
			let request = AllocRequest {
				required_flags: vk::MemoryPropertyFlags::empty(),
				preferred_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
				memory_type_bits: bits,
				..request()
			};
			let mut allocation = allocator.allocate(&request).unwrap();
			let info = allocator.allocation_info(&allocation).unwrap();
			assert_ne!(bits & (1 << info.memory_type_index), 0);
			assert!(info.offset.is_multiple_of(request.alignment));
			assert_eq!(info.size, request.size);
			assert_eq!(info.device_memory, allocation.handle());
			assert_eq!(info.mapped_pointer, None);
			assert_eq!(allocator.statistics().pooled_block_count, 0);
			assert!(STATE.with_borrow(|s| s.dedicated).is_null());
			allocator.free(&mut allocation).unwrap();
		}
		allocator.close().unwrap();
	}
}
