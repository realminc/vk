//! Real pooled-resource relocation with GPU copy, completion and host readback.
use vk::{
	self,
	mem::{
		Allocation, AllocationAlgorithm, AllocationCreateInfo, DefragmentationAlgorithm,
		DefragmentationContext, DefragmentationInfo, DefragmentationResource, MemoryUsage,
		PoolCreateInfo,
	},
};

const BUFFER_SIZE: usize = 16 * 1024;
const IMAGE_EXTENT: vk::Extent3D = vk::Extent3D {
	width: 7,
	height: 9,
	depth: 1,
};
const IMAGE_BYTES: usize = 7 * 9 * 4;

struct Resident {
	buffer: vk::Buffer,
	allocation: Allocation,
	pattern: u8,
}

struct ImageResident {
	image: vk::Image,
	allocation: Allocation,
	clear: [f32; 4],
	pixel: [u8; 4],
}

unsafe fn write_pattern(allocator: &mut vk::Allocator<'_>, resident: &Resident) {
	// SAFETY: the allocation is host-visible, no GPU operation is pending, and the
	// mapped byte extent is bounded by the managed buffer's allocation.
	unsafe {
		let pointer = allocator.map(&resident.allocation).unwrap();
		std::slice::from_raw_parts_mut(pointer.cast::<u8>(), BUFFER_SIZE).fill(resident.pattern);
		allocator
			.flush(&resident.allocation, 0, BUFFER_SIZE as u64)
			.unwrap();
		allocator.unmap(&resident.allocation).unwrap();
	}
}

unsafe fn assert_pattern(allocator: &mut vk::Allocator<'_>, resident: &Resident) {
	// SAFETY: the copy fence completed, the allocation is host-visible, and the
	// invalidated byte extent is bounded by the managed buffer's allocation.
	unsafe {
		let pointer = allocator.map(&resident.allocation).unwrap();
		allocator
			.invalidate(&resident.allocation, 0, BUFFER_SIZE as u64)
			.unwrap();
		assert!(
			std::slice::from_raw_parts(pointer.cast::<u8>(), BUFFER_SIZE)
				.iter()
				.all(|&byte| byte == resident.pattern)
		);
		allocator.unmap(&resident.allocation).unwrap();
	}
}

#[test]
fn native_buffer_copy_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this gated profile owns every native parent, pooled allocation,
	// command and replacement. The copy fence completes before old-resource
	// destruction/commit, and every child is explicitly destroyed before parents.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::TRANSFER_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 transfer queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let buffer_info = vk::BufferCreateInfo {
			size: BUFFER_SIZE as u64,
			usage: vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_buffer_info(&buffer_info, &selection)
			.unwrap();
		let probe = device.create_buffer(&buffer_info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(probe).unwrap();
		device.destroy_buffer(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let reserved_size = requirements
			.size
			.checked_add(atom - 1)
			.map(|size| size & !(atom - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(2).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 2,
				..Default::default()
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let mut residents = Vec::new();
		for pattern in [0x31, 0x72, 0xB4] {
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&buffer_info, &options)
				.unwrap();
			residents.push(Resident {
				buffer,
				allocation,
				pattern,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		for resident in &residents {
			write_pattern(&mut allocator, resident);
		}
		let mut removed = residents.remove(0);
		allocator
			.destroy_buffer(removed.buffer, &mut removed.allocation)
			.unwrap();

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let command = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 1,
				..Default::default()
			})
			.unwrap()[0];
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();

		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				max_allocations: 1,
				algorithm: DefragmentationAlgorithm::Fast,
				pool: Some(&pool),
				..Default::default()
			})
			.unwrap();
		assert_eq!(
			pass.len(),
			1,
			"fragmented two-block layout must yield one move"
		);
		let movement = *pass.movement(0).unwrap();
		let DefragmentationResource::Buffer(old_buffer) = movement.resource else {
			panic!("managed buffer move expected");
		};
		let moved = residents
			.iter_mut()
			.find(|resident| resident.buffer == old_buffer)
			.expect("proposal identifies a live resident");
		let replacement = pass.device().create_buffer(&buffer_info, None).unwrap();
		pass.bind_buffer(0, replacement).unwrap();
		pass
			.device()
			.begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())
			.unwrap();
		pass
			.device()
			.cmd_copy_buffer(
				command,
				old_buffer,
				replacement,
				&[vk::BufferCopy {
					size: BUFFER_SIZE as u64,
					..Default::default()
				}],
			)
			.unwrap();
		let barrier = vk::MemoryBarrier {
			src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
			dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
			..Default::default()
		};
		pass
			.device()
			.cmd_pipeline_barrier(
				command,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlags::empty(),
				&[barrier],
				&[],
				&[],
			)
			.unwrap();
		pass.device().end_command_buffer(command).unwrap();
		pass
			.device()
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &command,
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			pass
				.device()
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);
		pass.device().destroy_buffer(old_buffer, None).unwrap();
		pass.commit_move(0, &mut moved.allocation).unwrap();
		moved.buffer = replacement;
		let stats = pass.finish().unwrap();
		assert_eq!(stats.moved_allocations, 1);
		assert_eq!(stats.moved_bytes, requirements.size);
		assert_eq!(stats.freed_blocks, 0, "pool retains one empty spare block");
		drop(pass);
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		let trimmed = allocator.trim().unwrap();
		assert_eq!(trimmed.block_count, 1);
		assert_eq!(trimmed.block_bytes, block_size);
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		for resident in &residents {
			assert_pattern(&mut allocator, resident);
		}
		for resident in &mut residents {
			allocator
				.destroy_buffer(resident.buffer, &mut resident.allocation)
				.unwrap();
		}
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &[command])
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn native_image_copy_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this gated profile owns every native parent, pooled image, command,
	// readback buffer and replacement. Fences complete before old-resource
	// destruction/commit and host reads, and every child is destroyed first.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::TRANSFER_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 transfer queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let image_info = vk::ImageCreateInfo {
			image_type: vk::ImageType::_2D,
			format: vk::Format::R8G8B8A8_UNORM,
			extent: IMAGE_EXTENT,
			mip_levels: 1,
			array_layers: 1,
			samples: vk::SampleCountFlagBits::_1_BIT,
			tiling: vk::ImageTiling::OPTIMAL,
			usage: vk::ImageUsageFlagBits::TRANSFER_SRC_BIT | vk::ImageUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_image_info(&image_info, &selection)
			.unwrap();
		let probe = device.create_image(&image_info, None).unwrap();
		let mut requirements = vk::MemoryRequirements::default();
		device
			.get_image_memory_requirements(probe, &mut requirements)
			.unwrap();
		device.destroy_image(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let separation = requirements.alignment.max(atom);
		let reserved_size = requirements
			.size
			.checked_add(separation - 1)
			.map(|size| size & !(separation - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(2).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 2,
				..Default::default()
			})
			.unwrap();
		let readback_info = vk::BufferCreateInfo {
			size: 2 * IMAGE_BYTES as u64,
			usage: vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let readback_options = AllocationCreateInfo {
			usage: MemoryUsage::PreferHost,
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			persistent_map: true,
			..Default::default()
		};
		let (readback, mut readback_memory) = allocator
			.create_buffer_with_options(&readback_info, &readback_options)
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let colors = [
			([1.0, 0.0, 0.0, 1.0], [255, 0, 0, 255]),
			([0.0, 1.0, 0.0, 1.0], [0, 255, 0, 255]),
			([0.0, 0.0, 1.0, 1.0], [0, 0, 255, 255]),
		];
		let mut residents = Vec::new();
		for (clear, pixel) in colors {
			let (image, allocation) = allocator
				.create_image_with_options(&image_info, &options)
				.unwrap();
			residents.push(ImageResident {
				image,
				allocation,
				clear,
				pixel,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 3);

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let commands = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 2,
				..Default::default()
			})
			.unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();
		let range = vk::ImageSubresourceRange {
			aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		};
		let layers = vk::ImageSubresourceLayers {
			aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
			mip_level: 0,
			base_array_layer: 0,
			layer_count: 1,
		};
		device
			.begin_command_buffer(commands[0], &vk::CommandBufferBeginInfo::default())
			.unwrap();
		for resident in &residents {
			device
				.cmd_pipeline_barrier(
					commands[0],
					vk::PipelineStageFlagBits::TOP_OF_PIPE_BIT,
					vk::PipelineStageFlagBits::TRANSFER_BIT,
					vk::DependencyFlags::empty(),
					&[],
					&[],
					&[vk::ImageMemoryBarrier {
						dst_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
						old_layout: vk::ImageLayout::UNDEFINED,
						new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
						src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						image: resident.image,
						subresource_range: range,
						..Default::default()
					}],
				)
				.unwrap();
			device
				.cmd_clear_color_image(
					commands[0],
					resident.image,
					vk::ImageLayout::TRANSFER_DST_OPTIMAL,
					&vk::ClearColorValue {
						float32: resident.clear,
					},
					&[range],
				)
				.unwrap();
			device
				.cmd_pipeline_barrier(
					commands[0],
					vk::PipelineStageFlagBits::TRANSFER_BIT,
					vk::PipelineStageFlagBits::TRANSFER_BIT,
					vk::DependencyFlags::empty(),
					&[],
					&[],
					&[vk::ImageMemoryBarrier {
						src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
						dst_access_mask: vk::AccessFlagBits::TRANSFER_READ_BIT,
						old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
						new_layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
						src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						image: resident.image,
						subresource_range: range,
						..Default::default()
					}],
				)
				.unwrap();
		}
		device.end_command_buffer(commands[0]).unwrap();
		device
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &commands[0],
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			device
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);

		let mut removed = residents.remove(0);
		allocator
			.destroy_image(removed.image, &mut removed.allocation)
			.unwrap();

		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				max_allocations: 1,
				algorithm: DefragmentationAlgorithm::Fast,
				pool: Some(&pool),
				..Default::default()
			})
			.unwrap();
		assert_eq!(
			pass.len(),
			1,
			"fragmented two-block layout must yield one move"
		);
		let movement = *pass.movement(0).unwrap();
		let DefragmentationResource::Image(old_image) = movement.resource else {
			panic!("managed image move expected");
		};
		let moved_index = residents
			.iter()
			.position(|resident| resident.image == old_image)
			.expect("proposal identifies a live resident");
		let other_index = 1 - moved_index;
		let replacement = pass.device().create_image(&image_info, None).unwrap();
		pass.bind_image(0, replacement).unwrap();
		device.reset_fences(&[fence]).unwrap();
		pass
			.device()
			.begin_command_buffer(commands[1], &vk::CommandBufferBeginInfo::default())
			.unwrap();
		pass
			.device()
			.cmd_pipeline_barrier(
				commands[1],
				vk::PipelineStageFlagBits::TOP_OF_PIPE_BIT,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::DependencyFlags::empty(),
				&[],
				&[],
				&[vk::ImageMemoryBarrier {
					dst_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
					old_layout: vk::ImageLayout::UNDEFINED,
					new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
					src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
					dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
					image: replacement,
					subresource_range: range,
					..Default::default()
				}],
			)
			.unwrap();
		pass
			.device()
			.cmd_copy_image(
				commands[1],
				old_image,
				vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
				replacement,
				vk::ImageLayout::TRANSFER_DST_OPTIMAL,
				&[vk::ImageCopy {
					src_subresource: layers,
					src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
					dst_subresource: layers,
					dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
					extent: IMAGE_EXTENT,
				}],
			)
			.unwrap();
		pass
			.device()
			.cmd_pipeline_barrier(
				commands[1],
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::DependencyFlags::empty(),
				&[],
				&[],
				&[vk::ImageMemoryBarrier {
					src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
					dst_access_mask: vk::AccessFlagBits::TRANSFER_READ_BIT,
					old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
					new_layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
					src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
					dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
					image: replacement,
					subresource_range: range,
					..Default::default()
				}],
			)
			.unwrap();
		for (index, image) in [
			(0usize, replacement),
			(1usize, residents[other_index].image),
		] {
			pass
				.device()
				.cmd_copy_image_to_buffer(
					commands[1],
					image,
					vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
					readback,
					&[vk::BufferImageCopy {
						image_subresource: layers,
						image_extent: IMAGE_EXTENT,
						buffer_offset: index as u64 * IMAGE_BYTES as u64,
						buffer_row_length: 0,
						buffer_image_height: 0,
						image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
					}],
				)
				.unwrap();
		}
		pass
			.device()
			.cmd_pipeline_barrier(
				commands[1],
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlags::empty(),
				&[vk::MemoryBarrier {
					src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
					dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
					..Default::default()
				}],
				&[],
				&[],
			)
			.unwrap();
		pass.device().end_command_buffer(commands[1]).unwrap();
		pass
			.device()
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &commands[1],
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			pass
				.device()
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);
		pass.device().destroy_image(old_image, None).unwrap();
		pass
			.commit_move(0, &mut residents[moved_index].allocation)
			.unwrap();
		residents[moved_index].image = replacement;
		let stats = pass.finish().unwrap();
		assert_eq!(stats.moved_allocations, 1);
		assert_eq!(stats.moved_bytes, requirements.size);
		assert_eq!(stats.freed_blocks, 0, "pool retains one empty spare block");
		drop(pass);
		assert_eq!(allocator.statistics().native_allocation_count, 3);
		let trimmed = allocator.trim().unwrap();
		assert_eq!(trimmed.block_count, 1);
		assert_eq!(trimmed.block_bytes, block_size);
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		allocator
			.invalidate(&readback_memory, 0, vk::WHOLE_SIZE)
			.unwrap();
		let snapshot = allocator.allocation_info(&readback_memory).unwrap();
		let pointer = snapshot.mapped_pointer.unwrap();
		let data = std::slice::from_raw_parts(pointer.cast::<u8>(), 2 * IMAGE_BYTES);
		for pixel in data[..IMAGE_BYTES].as_chunks::<4>().0 {
			assert_eq!(*pixel, residents[moved_index].pixel);
		}
		for pixel in data[IMAGE_BYTES..].as_chunks::<4>().0 {
			assert_eq!(*pixel, residents[other_index].pixel);
		}
		for resident in &mut residents {
			allocator
				.destroy_image(resident.image, &mut resident.allocation)
				.unwrap();
		}
		allocator
			.destroy_buffer(readback, &mut readback_memory)
			.unwrap();
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &commands)
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn native_multi_pass_extensive_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this gated profile owns every native parent, pooled allocation,
	// command and replacement across bounded relocation passes. Each pass is
	// finished before the next, copy fences complete before old-resource
	// destruction/commit, and every child is explicitly destroyed first.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::TRANSFER_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 transfer queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let buffer_info = vk::BufferCreateInfo {
			size: BUFFER_SIZE as u64,
			usage: vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_buffer_info(&buffer_info, &selection)
			.unwrap();
		let probe = device.create_buffer(&buffer_info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(probe).unwrap();
		device.destroy_buffer(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let separation = requirements.alignment.max(atom);
		let reserved_size = requirements
			.size
			.checked_add(separation - 1)
			.map(|size| size & !(separation - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(2).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 4,
				..Default::default()
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let mut residents = Vec::new();
		for pattern in [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77] {
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&buffer_info, &options)
				.unwrap();
			residents.push(Resident {
				buffer,
				allocation,
				pattern,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 4);
		for resident in &residents {
			write_pattern(&mut allocator, resident);
		}
		for index in [4usize, 2, 0] {
			let mut removed = residents.remove(index);
			allocator
				.destroy_buffer(removed.buffer, &mut removed.allocation)
				.unwrap();
		}
		assert_eq!(residents.len(), 4);

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let commands = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 8,
				..Default::default()
			})
			.unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();

		let mut context = DefragmentationContext::new(
			&mut allocator,
			&DefragmentationInfo {
				pool: Some(&pool),
				max_allocations: 2,
				algorithm: DefragmentationAlgorithm::Extensive,
				..Default::default()
			},
		)
		.unwrap();
		let mut committed = 0usize;
		let mut ignored_once = false;
		let mut converged = false;
		for command in &commands {
			let mut pass = context.begin_pass().unwrap();
			if pass.is_empty() {
				pass.finish().unwrap();
				converged = true;
				break;
			}
			let mut resolved = Vec::new();
			for index in 0..pass.len() {
				if !ignored_once {
					pass.ignore_move(index).unwrap();
					ignored_once = true;
					continue;
				}
				let movement = *pass.movement(index).unwrap();
				let DefragmentationResource::Buffer(old_buffer) = movement.resource else {
					panic!("managed buffer move expected");
				};
				let resident = residents
					.iter()
					.position(|resident| resident.buffer == old_buffer)
					.expect("proposal identifies a live resident");
				let replacement = pass.device().create_buffer(&buffer_info, None).unwrap();
				pass.bind_buffer(index, replacement).unwrap();
				resolved.push((index, old_buffer, resident, replacement));
			}
			if !resolved.is_empty() {
				pass
					.device()
					.begin_command_buffer(*command, &vk::CommandBufferBeginInfo::default())
					.unwrap();
				for &(_, old_buffer, _, replacement) in &resolved {
					pass
						.device()
						.cmd_copy_buffer(
							*command,
							old_buffer,
							replacement,
							&[vk::BufferCopy {
								size: BUFFER_SIZE as u64,
								..Default::default()
							}],
						)
						.unwrap();
				}
				pass
					.device()
					.cmd_pipeline_barrier(
						*command,
						vk::PipelineStageFlagBits::TRANSFER_BIT,
						vk::PipelineStageFlagBits::HOST_BIT,
						vk::DependencyFlags::empty(),
						&[vk::MemoryBarrier {
							src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
							dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
							..Default::default()
						}],
						&[],
						&[],
					)
					.unwrap();
				pass.device().end_command_buffer(*command).unwrap();
				device.reset_fences(&[fence]).unwrap();
				pass
					.device()
					.queue_submit(
						queue,
						&[vk::SubmitInfo {
							command_buffer_count: 1,
							p_command_buffers: command,
							..Default::default()
						}],
						fence,
					)
					.unwrap();
				assert!(
					pass
						.device()
						.wait_for_fences(&[fence], true, 10_000_000_000)
						.unwrap()
				);
			}
			for (index, old_buffer, resident, replacement) in resolved {
				pass.device().destroy_buffer(old_buffer, None).unwrap();
				pass
					.commit_move(index, &mut residents[resident].allocation)
					.unwrap();
				residents[resident].buffer = replacement;
				committed += 1;
			}
			pass.finish().unwrap();
		}
		assert!(converged, "bounded extensive passes must converge");
		assert!(
			committed >= 1,
			"extensive passes must commit at least one move"
		);
		assert_eq!(context.ignored_allocations(), 1);
		let stats = context.finish().unwrap();
		assert_eq!(stats.moved_allocations, committed);
		assert_eq!(stats.ignored_allocations, 1);
		assert!(
			stats.freed_blocks >= 1,
			"evacuated held blocks must be released"
		);
		drop(context);
		assert_eq!(
			allocator.statistics().native_allocation_count,
			4 - stats.freed_blocks
		);
		assert_eq!(allocator.trim().unwrap(), vk::mem::TrimStats::default());
		for resident in &residents {
			assert_pattern(&mut allocator, resident);
		}
		for resident in &mut residents {
			allocator
				.destroy_buffer(resident.buffer, &mut resident.allocation)
				.unwrap();
		}
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &commands)
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}

fn intra_block(algorithm: DefragmentationAlgorithm) {
	// SAFETY: this gated profile owns every native parent, single-block pool,
	// command and replacement. Copy fences complete before old-resource
	// destruction/commit, and every child is explicitly destroyed first.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::TRANSFER_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 transfer queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let buffer_info = vk::BufferCreateInfo {
			size: BUFFER_SIZE as u64,
			usage: vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_buffer_info(&buffer_info, &selection)
			.unwrap();
		let probe = device.create_buffer(&buffer_info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(probe).unwrap();
		device.destroy_buffer(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let separation = requirements.alignment.max(atom);
		let reserved_size = requirements
			.size
			.checked_add(separation - 1)
			.map(|size| size & !(separation - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(3).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 1,
				..Default::default()
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let mut residents = Vec::new();
		for pattern in [0x21, 0x47, 0x8C] {
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&buffer_info, &options)
				.unwrap();
			residents.push(Resident {
				buffer,
				allocation,
				pattern,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		for resident in &residents {
			write_pattern(&mut allocator, resident);
		}
		let mut removed = residents.remove(0);
		allocator
			.destroy_buffer(removed.buffer, &mut removed.allocation)
			.unwrap();

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let commands = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 4,
				..Default::default()
			})
			.unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();

		let mut committed = 0usize;
		let mut converged = false;
		for command in &commands {
			let mut pass = allocator
				.begin_defragmentation(&DefragmentationInfo {
					pool: Some(&pool),
					max_allocations: 2,
					algorithm,
					..Default::default()
				})
				.unwrap();
			if pass.is_empty() {
				pass.finish().unwrap();
				converged = true;
				break;
			}
			let mut resolved = Vec::new();
			for index in 0..pass.len() {
				let movement = *pass.movement(index).unwrap();
				assert_eq!(
					movement.destination.device_memory, movement.source.device_memory,
					"intra-block compaction must stay inside the single block"
				);
				assert!(
					movement.destination.offset < movement.source.offset,
					"intra-block compaction must move to a strictly lower offset"
				);
				let DefragmentationResource::Buffer(old_buffer) = movement.resource else {
					panic!("managed buffer move expected");
				};
				let resident = residents
					.iter()
					.position(|resident| resident.buffer == old_buffer)
					.expect("proposal identifies a live resident");
				let replacement = pass.device().create_buffer(&buffer_info, None).unwrap();
				pass.bind_buffer(index, replacement).unwrap();
				resolved.push((index, old_buffer, resident, replacement));
			}
			pass
				.device()
				.begin_command_buffer(*command, &vk::CommandBufferBeginInfo::default())
				.unwrap();
			for &(_, old_buffer, _, replacement) in &resolved {
				pass
					.device()
					.cmd_copy_buffer(
						*command,
						old_buffer,
						replacement,
						&[vk::BufferCopy {
							size: BUFFER_SIZE as u64,
							..Default::default()
						}],
					)
					.unwrap();
			}
			pass
				.device()
				.cmd_pipeline_barrier(
					*command,
					vk::PipelineStageFlagBits::TRANSFER_BIT,
					vk::PipelineStageFlagBits::HOST_BIT,
					vk::DependencyFlags::empty(),
					&[vk::MemoryBarrier {
						src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
						dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
						..Default::default()
					}],
					&[],
					&[],
				)
				.unwrap();
			pass.device().end_command_buffer(*command).unwrap();
			device.reset_fences(&[fence]).unwrap();
			pass
				.device()
				.queue_submit(
					queue,
					&[vk::SubmitInfo {
						command_buffer_count: 1,
						p_command_buffers: command,
						..Default::default()
					}],
					fence,
				)
				.unwrap();
			assert!(
				pass
					.device()
					.wait_for_fences(&[fence], true, 10_000_000_000)
					.unwrap()
			);
			for (index, old_buffer, resident, replacement) in resolved {
				pass.device().destroy_buffer(old_buffer, None).unwrap();
				pass
					.commit_move(index, &mut residents[resident].allocation)
					.unwrap();
				residents[resident].buffer = replacement;
				committed += 1;
			}
			pass.finish().unwrap();
		}
		assert!(converged, "bounded intra-block passes must converge");
		assert_eq!(committed, 2, "both survivors compact to the front hole");
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		assert_eq!(allocator.trim().unwrap(), vk::mem::TrimStats::default());
		for resident in &residents {
			assert_pattern(&mut allocator, resident);
		}
		for resident in &mut residents {
			allocator
				.destroy_buffer(resident.buffer, &mut resident.allocation)
				.unwrap();
		}
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &commands)
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn native_intra_block_balanced_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	intra_block(DefragmentationAlgorithm::Balanced);
}

#[test]
fn native_intra_block_full_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	intra_block(DefragmentationAlgorithm::Full);
}

#[test]
fn native_alias_group_commit_and_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this gated profile owns every native parent, pooled allocation,
	// alias, command and replacement. Old aliases are destroyed before commit,
	// the copy fence completes first, and every child is destroyed early.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::TRANSFER_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 transfer queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let buffer_info = vk::BufferCreateInfo {
			size: BUFFER_SIZE as u64,
			usage: vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_buffer_info(&buffer_info, &selection)
			.unwrap();
		let probe = device.create_buffer(&buffer_info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(probe).unwrap();
		device.destroy_buffer(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let separation = requirements.alignment.max(atom);
		let reserved_size = requirements
			.size
			.checked_add(separation - 1)
			.map(|size| size & !(separation - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(2).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 2,
				..Default::default()
			})
			.unwrap();
		let mut residents = Vec::new();
		let mut aliases = Vec::new();
		for pattern in [0x51, 0xA3, 0xC9] {
			let aliased = pattern != 0x51;
			let options = AllocationCreateInfo {
				pool: Some(&pool),
				required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
				can_alias: aliased,
				..Default::default()
			};
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&buffer_info, &options)
				.unwrap();
			if aliased {
				aliases.push(
					allocator
						.create_aliasing_buffer(&allocation, 0, &buffer_info)
						.unwrap(),
				);
			} else {
				aliases.push(vk::Buffer::NULL);
			}
			residents.push(Resident {
				buffer,
				allocation,
				pattern,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		for resident in &residents {
			write_pattern(&mut allocator, resident);
		}
		let mut removed = residents.remove(0);
		allocator
			.destroy_buffer(removed.buffer, &mut removed.allocation)
			.unwrap();
		aliases.remove(0);

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let command = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 1,
				..Default::default()
			})
			.unwrap()[0];
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();

		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				max_allocations: 1,
				algorithm: DefragmentationAlgorithm::Fast,
				pool: Some(&pool),
				..Default::default()
			})
			.unwrap();
		assert_eq!(pass.len(), 1);
		let movement = *pass.movement(0).unwrap();
		assert!(
			movement.can_alias,
			"the moved allocation carries an alias group"
		);
		let DefragmentationResource::Buffer(old_buffer) = movement.resource else {
			panic!("managed buffer move expected");
		};
		let moved_index = residents
			.iter()
			.position(|resident| resident.buffer == old_buffer)
			.expect("proposal identifies a live resident");
		let replacement = pass.device().create_buffer(&buffer_info, None).unwrap();
		pass.bind_buffer(0, replacement).unwrap();
		let new_alias = pass.create_aliasing_buffer(0, 0, &buffer_info).unwrap();
		pass
			.device()
			.begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())
			.unwrap();
		pass
			.device()
			.cmd_copy_buffer(
				command,
				old_buffer,
				replacement,
				&[vk::BufferCopy {
					size: BUFFER_SIZE as u64,
					..Default::default()
				}],
			)
			.unwrap();
		pass
			.device()
			.cmd_pipeline_barrier(
				command,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlags::empty(),
				&[vk::MemoryBarrier {
					src_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
					dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
					..Default::default()
				}],
				&[],
				&[],
			)
			.unwrap();
		pass.device().end_command_buffer(command).unwrap();
		pass
			.device()
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &command,
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			pass
				.device()
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);
		pass
			.device()
			.destroy_buffer(aliases[moved_index], pass.allocation_callbacks())
			.unwrap();
		pass.device().destroy_buffer(old_buffer, None).unwrap();
		pass
			.commit_move(0, &mut residents[moved_index].allocation)
			.unwrap();
		residents[moved_index].buffer = replacement;
		aliases[moved_index] = new_alias;
		let stats = pass.finish().unwrap();
		assert_eq!(stats.moved_allocations, 1);
		assert_eq!(stats.moved_bytes, requirements.size);
		drop(pass);
		for resident in &residents {
			assert_pattern(&mut allocator, resident);
		}
		for (resident, &alias) in residents.iter_mut().zip(&aliases) {
			if !alias.is_null() {
				device.destroy_buffer(alias, None).unwrap();
			}
			allocator
				.destroy_buffer(resident.buffer, &mut resident.allocation)
				.unwrap();
		}
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &[command])
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}

const SHADER_WORDS: usize = 257;

unsafe fn assert_shader_payload(allocator: &mut vk::Allocator<'_>, resident: &Resident) {
	// SAFETY: the dispatch/copy fences completed, the allocation is host-visible,
	// and the invalidated byte extent is bounded by the managed buffer.
	unsafe {
		let pointer = allocator.map(&resident.allocation).unwrap();
		allocator
			.invalidate(&resident.allocation, 0, BUFFER_SIZE as u64)
			.unwrap();
		let words = std::slice::from_raw_parts(pointer.cast::<u32>(), SHADER_WORDS);
		for (index, &word) in words.iter().enumerate() {
			assert_eq!(
				word,
				(index as u32)
					.wrapping_mul(1664525)
					.wrapping_add(1013904223)
			);
		}
		let tail = std::slice::from_raw_parts(
			pointer.cast::<u8>().add(SHADER_WORDS * 4),
			BUFFER_SIZE - SHADER_WORDS * 4,
		);
		assert!(tail.iter().all(|&byte| byte == resident.pattern));
		allocator.unmap(&resident.allocation).unwrap();
	}
}

#[test]
fn native_descriptor_rebind_after_commit() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this gated profile owns every native parent, pooled allocation,
	// descriptor set, pipeline, command and replacement. Each fence completes
	// before dependent dispatch/copy/host uses, old resources are destroyed
	// before commit, and every child is explicitly destroyed first.
	unsafe {
		let entry = vk::Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo {
				p_application_info: &app,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				(instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1)
					.then(|| {
						instance
							.get_physical_device_queue_family_properties(physical)
							.unwrap()
							.iter()
							.position(|properties| {
								properties.queue_count > 0
									&& properties.queue_flags.as_raw() & vk::QueueFlagBits::COMPUTE_BIT.as_raw() != 0
							})
							.map(|family| (physical, u32::try_from(family).unwrap()))
					})
					.flatten()
			})
			.expect("Vulkan 1.1 compute queue");
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let buffer_info = vk::BufferCreateInfo {
			size: BUFFER_SIZE as u64,
			usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT
				| vk::BufferUsageFlagBits::TRANSFER_SRC_BIT
				| vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let selection = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let memory_type = allocator
			.find_memory_type_for_buffer_info(&buffer_info, &selection)
			.unwrap();
		let probe = device.create_buffer(&buffer_info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(probe).unwrap();
		device.destroy_buffer(probe, None).unwrap();
		let native = instance.get_physical_device_properties(physical).unwrap();
		let memory = instance
			.get_physical_device_memory_properties(physical)
			.unwrap();
		let flags = memory.memory_types[memory_type as usize].property_flags;
		let atom = if flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
		{
			native.limits.non_coherent_atom_size
		} else {
			1
		};
		let separation = requirements.alignment.max(atom);
		let reserved_size = requirements
			.size
			.checked_add(separation - 1)
			.map(|size| size & !(separation - 1))
			.unwrap();
		let block_size = reserved_size.checked_mul(2).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: memory_type,
				algorithm: AllocationAlgorithm::Tlsf,
				block_size,
				max_block_count: 2,
				..Default::default()
			})
			.unwrap();
		let options = AllocationCreateInfo {
			pool: Some(&pool),
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let mut residents = Vec::new();
		for pattern in [0x31, 0x72, 0xB4] {
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&buffer_info, &options)
				.unwrap();
			residents.push(Resident {
				buffer,
				allocation,
				pattern,
			});
		}
		assert_eq!(allocator.statistics().native_allocation_count, 2);
		for resident in &residents {
			write_pattern(&mut allocator, resident);
		}

		let binding = vk::DescriptorSetLayoutBinding {
			binding: 0,
			descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
			descriptor_count: 1,
			stage_flags: vk::ShaderStageFlagBits::COMPUTE_BIT,
			p_immutable_samplers: std::ptr::null(),
			..Default::default()
		};
		let set_layout = device
			.create_descriptor_set_layout(
				&vk::DescriptorSetLayoutCreateInfo {
					binding_count: 1,
					p_bindings: &binding,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let pipeline_layout = device
			.create_pipeline_layout(
				&vk::PipelineLayoutCreateInfo {
					set_layout_count: 1,
					p_set_layouts: &set_layout,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let code: Vec<u32> = include_bytes!("fixtures/storage.comp.spv")
			.as_chunks::<4>()
			.0
			.iter()
			.map(|bytes| u32::from_le_bytes(*bytes))
			.collect();
		let shader = device
			.create_shader_module(
				&vk::ShaderModuleCreateInfo {
					code_size: code.len() * 4,
					p_code: code.as_ptr(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut pipelines = [vk::Pipeline::NULL];
		let status = device
			.create_compute_pipelines(
				vk::PipelineCache::NULL,
				&[vk::ComputePipelineCreateInfo {
					stage: vk::PipelineShaderStageCreateInfo {
						stage: vk::ShaderStageFlagBits::COMPUTE_BIT,
						module: shader,
						p_name: c"main".as_ptr(),
						..Default::default()
					},
					layout: pipeline_layout,
					..Default::default()
				}],
				None,
				&mut pipelines,
			)
			.unwrap();
		assert_eq!(status, vk::Result::SUCCESS);
		let pipeline = pipelines[0];
		let pool_size = vk::DescriptorPoolSize {
			r#type: vk::DescriptorType::STORAGE_BUFFER,
			descriptor_count: 3,
		};
		let descriptor_pool = device
			.create_descriptor_pool(
				&vk::DescriptorPoolCreateInfo {
					max_sets: residents.len() as u32,
					pool_size_count: 1,
					p_pool_sizes: &pool_size,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let layouts = [set_layout; 3];
		let mut sets = [vk::DescriptorSet::NULL; 3];
		device
			.allocate_descriptor_sets(
				&vk::DescriptorSetAllocateInfo {
					descriptor_pool,
					descriptor_set_count: 3,
					p_set_layouts: layouts.as_ptr(),
					..Default::default()
				},
				&mut sets,
			)
			.unwrap();
		let buffer_descriptors: Vec<_> = residents
			.iter()
			.map(|resident| vk::DescriptorBufferInfo {
				buffer: resident.buffer,
				offset: 0,
				range: BUFFER_SIZE as u64,
			})
			.collect();
		for (index, set) in sets.iter().enumerate() {
			device
				.update_descriptor_sets(
					&[vk::WriteDescriptorSet {
						dst_set: *set,
						dst_binding: 0,
						descriptor_count: 1,
						descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
						p_buffer_info: &buffer_descriptors[index],
						..Default::default()
					}],
					&[],
				)
				.unwrap();
		}

		let command_pool = device
			.create_command_pool(
				&vk::CommandPoolCreateInfo {
					queue_family_index: family,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let commands = device
			.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
				command_pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 3,
				..Default::default()
			})
			.unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();
		device
			.begin_command_buffer(commands[0], &vk::CommandBufferBeginInfo::default())
			.unwrap();
		device
			.cmd_bind_pipeline(commands[0], vk::PipelineBindPoint::COMPUTE, pipeline)
			.unwrap();
		for (index, resident) in residents.iter().enumerate() {
			device
				.cmd_bind_descriptor_sets(
					commands[0],
					vk::PipelineBindPoint::COMPUTE,
					pipeline_layout,
					0,
					&[sets[index]],
					&[],
				)
				.unwrap();
			device.cmd_dispatch(commands[0], 5, 1, 1).unwrap();
			let _ = resident;
		}
		device.end_command_buffer(commands[0]).unwrap();
		device
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &commands[0],
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			device
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);

		let mut removed = residents.remove(0);
		allocator
			.destroy_buffer(removed.buffer, &mut removed.allocation)
			.unwrap();
		let mut pass = allocator
			.begin_defragmentation(&DefragmentationInfo {
				max_allocations: 1,
				algorithm: DefragmentationAlgorithm::Fast,
				pool: Some(&pool),
				..Default::default()
			})
			.unwrap();
		assert_eq!(pass.len(), 1);
		let movement = *pass.movement(0).unwrap();
		let DefragmentationResource::Buffer(old_buffer) = movement.resource else {
			panic!("managed buffer move expected");
		};
		let moved_index = residents
			.iter()
			.position(|resident| resident.buffer == old_buffer)
			.expect("proposal identifies a live resident");
		let moved_set = sets[moved_index + 1];
		let replacement = pass.device().create_buffer(&buffer_info, None).unwrap();
		pass.bind_buffer(0, replacement).unwrap();
		pass
			.device()
			.begin_command_buffer(commands[1], &vk::CommandBufferBeginInfo::default())
			.unwrap();
		pass
			.device()
			.cmd_copy_buffer(
				commands[1],
				old_buffer,
				replacement,
				&[vk::BufferCopy {
					size: BUFFER_SIZE as u64,
					..Default::default()
				}],
			)
			.unwrap();
		pass.device().end_command_buffer(commands[1]).unwrap();
		device.reset_fences(&[fence]).unwrap();
		pass
			.device()
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &commands[1],
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			pass
				.device()
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);
		pass.device().destroy_buffer(old_buffer, None).unwrap();
		pass
			.commit_move(0, &mut residents[moved_index].allocation)
			.unwrap();
		residents[moved_index].buffer = replacement;
		let stats = pass.finish().unwrap();
		assert_eq!(stats.moved_allocations, 1);
		drop(pass);

		let moved_descriptor = vk::DescriptorBufferInfo {
			buffer: replacement,
			offset: 0,
			range: BUFFER_SIZE as u64,
		};
		device
			.update_descriptor_sets(
				&[vk::WriteDescriptorSet {
					dst_set: moved_set,
					dst_binding: 0,
					descriptor_count: 1,
					descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
					p_buffer_info: &moved_descriptor,
					..Default::default()
				}],
				&[],
			)
			.unwrap();
		device
			.begin_command_buffer(commands[2], &vk::CommandBufferBeginInfo::default())
			.unwrap();
		device
			.cmd_bind_pipeline(commands[2], vk::PipelineBindPoint::COMPUTE, pipeline)
			.unwrap();
		device
			.cmd_bind_descriptor_sets(
				commands[2],
				vk::PipelineBindPoint::COMPUTE,
				pipeline_layout,
				0,
				&[moved_set],
				&[],
			)
			.unwrap();
		device.cmd_dispatch(commands[2], 5, 1, 1).unwrap();
		device
			.cmd_pipeline_barrier(
				commands[2],
				vk::PipelineStageFlagBits::COMPUTE_SHADER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlags::empty(),
				&[vk::MemoryBarrier {
					src_access_mask: vk::AccessFlagBits::SHADER_WRITE_BIT,
					dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
					..Default::default()
				}],
				&[],
				&[],
			)
			.unwrap();
		device.end_command_buffer(commands[2]).unwrap();
		device.reset_fences(&[fence]).unwrap();
		device
			.queue_submit(
				queue,
				&[vk::SubmitInfo {
					command_buffer_count: 1,
					p_command_buffers: &commands[2],
					..Default::default()
				}],
				fence,
			)
			.unwrap();
		assert!(
			device
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);

		for resident in &residents {
			assert_shader_payload(&mut allocator, resident);
		}
		for resident in &mut residents {
			allocator
				.destroy_buffer(resident.buffer, &mut resident.allocation)
				.unwrap();
		}
		device
			.destroy_descriptor_pool(descriptor_pool, None)
			.unwrap();
		device.destroy_pipeline(pipeline, None).unwrap();
		device.destroy_shader_module(shader, None).unwrap();
		device
			.destroy_pipeline_layout(pipeline_layout, None)
			.unwrap();
		device
			.destroy_descriptor_set_layout(set_layout, None)
			.unwrap();
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &commands)
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}
