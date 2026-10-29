//! Real optimal-image clear, transfer and exact host readback through the native allocator.
use vk::{
	self,
	mem::{AllocationAlgorithm, AllocationCreateInfo, AllocationPolicy, HostAccess, MemoryUsage},
};

#[test]
fn managed_image_transfer_readback() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	image_transfer(vk::API_VERSION_1_0, AllocationPolicy::Separate, None);
}

#[test]
fn pooled_image_transfer_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	image_transfer(
		vk::API_VERSION_1_1,
		AllocationPolicy::Pooled { block_size: 65536 },
		None,
	);
}

#[test]
fn adaptive_image_transfer_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	image_transfer(
		vk::API_VERSION_1_1,
		AllocationPolicy::Adaptive {
			block_size: 65536,
			min_block_size: 4096,
		},
		None,
	);
}

#[test]
fn custom_pool_image_transfer_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	image_transfer(
		vk::API_VERSION_1_1,
		AllocationPolicy::Pooled { block_size: 65536 },
		Some(AllocationAlgorithm::Tlsf),
	);
}

#[test]
fn linear_mixed_upper_image_transfer_readback() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	image_transfer(
		vk::API_VERSION_1_1,
		AllocationPolicy::Pooled { block_size: 65536 },
		Some(AllocationAlgorithm::Linear),
	);
}

fn image_transfer(
	version: u32,
	policy: AllocationPolicy,
	custom_pool: Option<AllocationAlgorithm>,
) {
	// SAFETY: this explicit native profile owns every resource, records exact layout/access
	// transitions, waits for its fence, invalidates host cache and releases children first.
	unsafe {
		let entry = vk::Entry::new().unwrap();
		let app = vk::ApplicationInfo {
			api_version: version,
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
				instance
					.get_physical_device_queue_family_properties(physical)
					.unwrap()
					.iter()
					.position(|q| {
						q.queue_count > 0
							&& q.queue_flags.as_raw() & vk::QueueFlagBits::COMPUTE_BIT.as_raw() != 0
					})
					.map(|index| (physical, index as u32))
			})
			.expect("compute/transfer device");
		let budget_enabled = version >= vk::API_VERSION_1_1
			&& instance
				.enumerate_device_extension_properties(physical, None)
				.unwrap()
				.iter()
				.any(|extension| {
					std::ffi::CStr::from_ptr(extension.extension_name.as_ptr()) == c"VK_EXT_memory_budget"
				});
		let extensions = [c"VK_EXT_memory_budget".as_ptr()];
		let priorities = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priorities,
			..Default::default()
		};
		let device = vk::Device::new(
			&instance,
			physical,
			&vk::DeviceCreateInfo {
				queue_create_info_count: 1,
				p_queue_create_infos: &queue_info,
				enabled_extension_count: u32::from(budget_enabled),
				pp_enabled_extension_names: extensions.as_ptr(),
				..Default::default()
			},
			None,
		)
		.unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let budget = allocator.query_budget(&instance, physical).unwrap();
		assert_eq!(
			budget.source == vk::mem::BudgetSource::Extension,
			budget_enabled
		);
		eprintln!(
			"image allocator profile: api={version}, policy={policy:?}, budget={:?}",
			budget.source
		);
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
			usage: vk::ImageUsageFlagBits::TRANSFER_SRC_BIT | vk::ImageUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let mut memory_pool = if let Some(algorithm) = custom_pool {
			let probe = device.create_image(&image_info, None).unwrap();
			let mut requirements = vk::MemoryRequirements::default();
			device
				.get_image_memory_requirements(probe, &mut requirements)
				.unwrap();
			let properties = instance
				.get_physical_device_memory_properties(physical)
				.unwrap();
			let native = instance.get_physical_device_properties(physical).unwrap();
			let selector =
				vk::mem::MemoryTypeSelector::new(properties, native.limits.non_coherent_atom_size).unwrap();
			let index = selector
				.find_memory_type(&vk::AllocRequest {
					size: requirements.size,
					alignment: requirements.alignment,
					memory_type_bits: requirements.memory_type_bits,
					required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
					preferred_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
				})
				.unwrap();
			device.destroy_image(probe, None).unwrap();
			Some(
				allocator
					.create_pool(&vk::mem::PoolCreateInfo {
						memory_type_index: index,
						algorithm,
						block_size: 65536,
						min_block_count: 0,
						max_block_count: 1,
						..Default::default()
					})
					.unwrap(),
			)
		} else {
			None
		};
		let image_options = AllocationCreateInfo {
			pool: memory_pool.as_ref(),
			upper_address: custom_pool == Some(AllocationAlgorithm::Linear),
			usage: if custom_pool.is_some() {
				MemoryUsage::Explicit
			} else {
				MemoryUsage::PreferDevice
			},
			preferred_flags: vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT,
			policy,
			..Default::default()
		};
		let (image, mut image_memory) = if custom_pool.is_some() {
			let image = device.create_image(&image_info, None).unwrap();
			let selected = allocator
				.find_memory_type_for_image_info(&image_info, &image_options)
				.unwrap();
			let mut requirements = vk::MemoryRequirements::default();
			device
				.get_image_memory_requirements(image, &mut requirements)
				.unwrap();
			let allocation = allocator
				.allocate_with_options(&requirements, &image_options)
				.unwrap();
			assert_eq!(allocation.memory_type_index(), selected);
			vk::mem::bind_image_memory(&mut allocator, image, &allocation).unwrap();
			(image, allocation)
		} else {
			allocator
				.create_image_with_options(&image_info, &image_options)
				.unwrap()
		};
		const BYTES: u64 = 7 * 9 * 4;
		let buffer_info = vk::BufferCreateInfo {
			size: BYTES,
			usage: vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
			..Default::default()
		};
		let buffer_options = AllocationCreateInfo {
			pool: memory_pool.as_ref(),
			usage: MemoryUsage::PreferHost,
			host_access: HostAccess::Random,
			policy,
			persistent_map: true,
			..Default::default()
		};
		let (buffer, mut buffer_memory) = allocator
			.create_buffer_with_options(&buffer_info, &buffer_options)
			.unwrap();
		if custom_pool.is_some() {
			assert_eq!(
				image_memory.handle(),
				buffer_memory.handle(),
				"mixed custom-pool resources must share one native block"
			);
		}
		if image_memory.handle() == buffer_memory.handle() {
			let native = instance.get_physical_device_properties(physical).unwrap();
			let page = native.limits.buffer_image_granularity;
			assert!(
				(image_memory.offset() + image_memory.reserved_size() - 1) / page
					< buffer_memory.offset() / page
					|| (buffer_memory.offset() + buffer_memory.reserved_size() - 1) / page
						< image_memory.offset() / page
			);
		}
		assert_eq!(
			allocator.trim_with_budget(&instance, physical).unwrap(),
			vk::mem::TrimStats::default()
		);

		let pool = device
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
				command_pool: pool,
				level: vk::CommandBufferLevel::PRIMARY,
				command_buffer_count: 1,
				..Default::default()
			})
			.unwrap()[0];
		device
			.begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())
			.unwrap();
		let range = vk::ImageSubresourceRange {
			aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		};
		let mut barrier = vk::ImageMemoryBarrier {
			dst_access_mask: vk::AccessFlagBits::TRANSFER_WRITE_BIT,
			old_layout: vk::ImageLayout::UNDEFINED,
			new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
			src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
			dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
			image,
			subresource_range: range,
			..Default::default()
		};
		device
			.cmd_pipeline_barrier(
				command,
				vk::PipelineStageFlagBits::TOP_OF_PIPE_BIT,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::DependencyFlags::empty(),
				&[],
				&[],
				std::slice::from_ref(&barrier),
			)
			.unwrap();
		device
			.cmd_clear_color_image(
				command,
				image,
				vk::ImageLayout::TRANSFER_DST_OPTIMAL,
				&vk::ClearColorValue {
					float32: [1.0, 0.0, 1.0, 1.0],
				},
				&[range],
			)
			.unwrap();
		barrier.src_access_mask = vk::AccessFlagBits::TRANSFER_WRITE_BIT;
		barrier.dst_access_mask = vk::AccessFlagBits::TRANSFER_READ_BIT;
		barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
		barrier.new_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
		device
			.cmd_pipeline_barrier(
				command,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::PipelineStageFlagBits::TRANSFER_BIT,
				vk::DependencyFlags::empty(),
				&[],
				&[],
				std::slice::from_ref(&barrier),
			)
			.unwrap();
		device
			.cmd_copy_image_to_buffer(
				command,
				image,
				vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
				buffer,
				&[vk::BufferImageCopy {
					image_subresource: vk::ImageSubresourceLayers {
						aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
						mip_level: 0,
						base_array_layer: 0,
						layer_count: 1,
					},
					image_extent: image_info.extent,
					buffer_offset: 0,
					buffer_row_length: 0,
					buffer_image_height: 0,
					image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
				}],
			)
			.unwrap();
		device
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
		device.end_command_buffer(command).unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();
		device
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
			device
				.wait_for_fences(&[fence], true, 10_000_000_000)
				.unwrap()
		);
		allocator
			.invalidate(&buffer_memory, 0, vk::WHOLE_SIZE)
			.unwrap();
		let snapshot = allocator.allocation_info(&buffer_memory).unwrap();
		assert_eq!(snapshot.device_memory, buffer_memory.handle());
		assert_eq!(snapshot.offset, buffer_memory.offset());
		assert!(snapshot.size >= BYTES);
		let pointer = snapshot.mapped_pointer.unwrap();
		let data = std::slice::from_raw_parts(pointer.cast::<u8>(), BYTES as usize);
		for pixel in data.as_chunks::<4>().0 {
			assert_eq!(*pixel, [255u8, 0, 255, 255]);
		}
		device.destroy_fence(fence, None).unwrap();
		device.destroy_command_pool(pool, None).unwrap();
		allocator.destroy_image(image, &mut image_memory).unwrap();
		allocator
			.destroy_buffer(buffer, &mut buffer_memory)
			.unwrap();
		if let Some(pool) = memory_pool.as_mut() {
			assert_eq!(
				vk::mem::trim(&mut allocator).unwrap(),
				vk::mem::TrimStats {
					block_count: 1,
					block_bytes: 65536
				}
			);
			assert_eq!(allocator.pool_statistics(pool).unwrap().block_count, 0);
			allocator.destroy_pool(pool).unwrap();
		}
		assert_eq!(allocator.statistics().native_bytes, 0);
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
	}
}
