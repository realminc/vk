//! Real storage-buffer allocation, compute dispatch and readback.
#![cfg(feature = "allocator")]
use ::vk::{self as vk, memory::AllocError};

#[test]
fn compute_shader_storage_buffer_round_trip() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_0, false, false, false);
}

#[test]
fn descriptor_template_and_base_dispatch_storage_buffer_round_trip() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_1, false, false, false);
}

#[test]
fn khr_dedication_and_allocation_size_storage_buffer_round_trip() {
	if std::env::var("VULKAN_MEMORY_KHR_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_0, true, false, false);
}

#[test]
fn pooled_legacy_storage_buffer_round_trip() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_0, false, true, false);
}

#[test]
fn pooled_core_persistent_storage_buffer_round_trip() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_1, false, true, true);
}

#[test]
fn pooled_khr_persistent_storage_buffer_round_trip() {
	if std::env::var("VULKAN_MEMORY_KHR_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	compute_buffer(vk::API_VERSION_1_0, true, true, true);
}

fn compute_buffer(api_version: u32, khr_memory: bool, pooled: bool, persistent: bool) {
	use ::vk::{Allocator, Entry};
	// SAFETY: explicit driver gate; this test creates all objects, waits for exact
	// completion, makes shader writes visible to the host and destroys children first.
	unsafe {
		let entry = Entry::new().expect("load Vulkan");
		let app = vk::ApplicationInfo {
			api_version,
			..Default::default()
		};
		let instance_extensions = [c"VK_KHR_get_physical_device_properties2".as_ptr()];
		let instance = entry
			.create_instance(
				&vk::InstanceCreateInfo {
					p_application_info: &app,
					enabled_extension_count: u32::from(khr_memory),
					pp_enabled_extension_names: instance_extensions.as_ptr(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		let (physical, family) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|p| {
				instance
					.get_physical_device_queue_family_properties(p)
					.unwrap()
					.iter()
					.position(|q| {
						q.queue_count > 0
							&& q.queue_flags.as_raw() & vk::QueueFlagBits::COMPUTE_BIT.as_raw() != 0
					})
					.map(|i| (p, i as u32))
			})
			.expect("compute-capable device");
		let props = instance.get_physical_device_properties(physical).unwrap();
		assert!(
			props.api_version >= api_version,
			"profile requires the requested device version"
		);
		let priority = 1.0;
		let queues = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device_extensions = [
			c"VK_KHR_get_memory_requirements2".as_ptr(),
			c"VK_KHR_dedicated_allocation".as_ptr(),
			c"VK_KHR_maintenance3".as_ptr(),
		];
		let device = instance
			.create_device(
				physical,
				&vk::DeviceCreateInfo {
					queue_create_info_count: 1,
					p_queue_create_infos: &queues,
					enabled_extension_count: if khr_memory {
						device_extensions.len() as u32
					} else {
						0
					},
					pp_enabled_extension_names: device_extensions.as_ptr(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		let queue = if api_version >= vk::API_VERSION_1_1 {
			let mut queue = vk::Queue::NULL;
			device
				.get_device_queue2(
					&vk::DeviceQueueInfo2 {
						queue_family_index: family,
						..Default::default()
					},
					&mut queue,
				)
				.unwrap();
			assert!(!queue.is_null());
			queue
		} else {
			device.get_device_queue(family, 0).unwrap()
		};
		let mut allocator = Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		if api_version >= vk::API_VERSION_1_1 || khr_memory {
			let limit = allocator
				.max_allocation_size()
				.expect("queried native size limit");
			// If the native ceiling is u64::MAX, no larger request is representable.
			if let Some(size) = limit.checked_add(1) {
				let oversized = ::vk::AllocRequest {
					size,
					alignment: 1,
					memory_type_bits: u32::MAX,
					required_flags: vk::MemoryPropertyFlags::empty(),
					preferred_flags: vk::MemoryPropertyFlags::empty(),
				};
				assert!(
					matches!(allocator.allocate(&oversized), Err(AllocError::AllocationTooLarge { requested, limit: ceiling }) if requested == oversized.size && ceiling == limit)
				);
			}
		} else {
			assert_eq!(allocator.max_allocation_size(), None);
		}
		const COUNT: usize = 257;
		let info = vk::BufferCreateInfo {
			size: (COUNT * 4) as u64,
			usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
			sharing_mode: vk::SharingMode::EXCLUSIVE,
			..Default::default()
		};
		let options = ::vk::memory::AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			preferred_flags: vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT,
			policy: ::vk::memory::AllocationPolicy::Pooled {
				block_size: 64 * 1024,
			},
			persistent_map: persistent,
			..Default::default()
		};
		let sentinel = pooled.then(|| {
			let (buffer, allocation) = allocator
				.create_buffer_with_options(&info, &options)
				.unwrap();
			let pointer = allocator.map(&allocation).unwrap();
			pointer.cast::<u8>().write_bytes(0xA5, COUNT * 4);
			allocator.flush(&allocation, 0, vk::WHOLE_SIZE).unwrap();
			(buffer, allocation, pointer)
		});
		let (buffer, mut allocation) = if pooled {
			allocator.create_buffer_with_options(&info, &options)
		} else {
			allocator.create_buffer(&info, options.required_flags, options.preferred_flags)
		}
		.unwrap();
		if let Some((_, neighbor, _)) = &sentinel {
			assert_eq!(
				neighbor.handle(),
				allocation.handle(),
				"ordinary buffer pair must share this qualified device's block"
			);
			assert!(allocation.offset() >= neighbor.offset() + neighbor.reserved_size());
			assert_eq!(allocator.statistics().native_allocation_count, 1);
		}

		let binding = vk::DescriptorSetLayoutBinding {
			binding: 0,
			descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
			descriptor_count: 1,
			stage_flags: vk::ShaderStageFlagBits::COMPUTE_BIT,
			p_immutable_samplers: std::ptr::null(),
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
		let pipeline_layout = vk::create_pipeline_layout(
			&device,
			&vk::PipelineLayoutCreateInfo {
				set_layout_count: 1,
				p_set_layouts: &set_layout,
				..Default::default()
			},
			None,
		)
		.unwrap();
		let code: Vec<u32> = include_bytes!("fixtures/storage.comp.spv")
			.chunks_exact(4)
			.map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
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
		let stage = vk::PipelineShaderStageCreateInfo {
			stage: vk::ShaderStageFlagBits::COMPUTE_BIT,
			module: shader,
			p_name: c"main".as_ptr(),
			..Default::default()
		};
		let mut pipelines = [vk::Pipeline::NULL];
		let status = device
			.create_compute_pipelines(
				vk::PipelineCache::NULL,
				&[vk::ComputePipelineCreateInfo {
					flags: if api_version >= vk::API_VERSION_1_1 {
						vk::PipelineCreateFlagBits::DISPATCH_BASE_BIT
					} else {
						vk::PipelineCreateFlags::empty()
					},
					stage,
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
			descriptor_count: 1,
		};
		let descriptor_pool = device
			.create_descriptor_pool(
				&vk::DescriptorPoolCreateInfo {
					max_sets: 1,
					pool_size_count: 1,
					p_pool_sizes: &pool_size,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut sets = [vk::DescriptorSet::NULL];
		device
			.allocate_descriptor_sets(
				&vk::DescriptorSetAllocateInfo {
					descriptor_pool,
					descriptor_set_count: 1,
					p_set_layouts: &set_layout,
					..Default::default()
				},
				&mut sets,
			)
			.unwrap();
		let set = sets[0];
		let buffer_info = vk::DescriptorBufferInfo {
			buffer,
			offset: 0,
			range: (COUNT * 4) as u64,
		};
		let write = vk::WriteDescriptorSet {
			dst_set: set,
			dst_binding: 0,
			descriptor_count: 1,
			descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
			p_buffer_info: &buffer_info,
			..Default::default()
		};
		if api_version >= vk::API_VERSION_1_1 {
			let layout_info = vk::DescriptorSetLayoutCreateInfo {
				binding_count: 1,
				p_bindings: &binding,
				..Default::default()
			};
			let mut support = vk::DescriptorSetLayoutSupport::default();
			device
				.get_descriptor_set_layout_support(&layout_info, &mut support)
				.unwrap();
			assert_eq!(support.supported, vk::TRUE);
			let entry = vk::DescriptorUpdateTemplateEntry {
				dst_binding: 0,
				dst_array_element: 0,
				descriptor_count: 1,
				descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
				offset: 0,
				stride: std::mem::size_of::<vk::DescriptorBufferInfo>(),
			};
			let template = device
				.create_descriptor_update_template(
					&vk::DescriptorUpdateTemplateCreateInfo {
						descriptor_update_entry_count: 1,
						p_descriptor_update_entries: &entry,
						template_type: vk::DescriptorUpdateTemplateType::DESCRIPTOR_SET,
						descriptor_set_layout: set_layout,
						..Default::default()
					},
					None,
				)
				.unwrap();
			device
				.update_descriptor_set_with_template(
					set,
					template,
					(&buffer_info as *const vk::DescriptorBufferInfo).cast(),
				)
				.unwrap();
			device
				.destroy_descriptor_update_template(template, None)
				.unwrap();
		} else {
			device.update_descriptor_sets(&[write], &[]).unwrap();
		}
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
		vk::cmd_bind_pipeline(&device, command, vk::PipelineBindPoint::COMPUTE, pipeline).unwrap();
		device
			.cmd_bind_descriptor_sets(
				command,
				vk::PipelineBindPoint::COMPUTE,
				pipeline_layout,
				0,
				&[set],
				&[],
			)
			.unwrap();
		if api_version >= vk::API_VERSION_1_1 {
			device.cmd_set_device_mask(command, 1).unwrap();
			device.cmd_dispatch_base(command, 0, 0, 0, 1, 1, 1).unwrap();
			device
				.cmd_dispatch_base(command, 1, 0, 0, (COUNT as u32).div_ceil(64) - 1, 1, 1)
				.unwrap();
		} else {
			device
				.cmd_dispatch(command, (COUNT as u32).div_ceil(64), 1, 1)
				.unwrap();
		}
		let barrier = vk::MemoryBarrier {
			src_access_mask: vk::AccessFlagBits::SHADER_WRITE_BIT,
			dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT,
			..Default::default()
		};
		device
			.cmd_pipeline_barrier(
				command,
				vk::PipelineStageFlagBits::COMPUTE_SHADER_BIT,
				vk::PipelineStageFlagBits::HOST_BIT,
				vk::DependencyFlags::empty(),
				&[barrier],
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
		let mapped = if pooled && persistent {
			allocator.mapped_pointer(&allocation).unwrap().unwrap()
		} else {
			allocator.map(&allocation).unwrap()
		};
		allocator
			.invalidate(&allocation, 0, vk::WHOLE_SIZE)
			.unwrap();
		for (i, &actual) in std::slice::from_raw_parts(mapped.cast::<u32>(), COUNT)
			.iter()
			.enumerate()
		{
			assert_eq!(
				actual,
				(i as u32).wrapping_mul(1664525).wrapping_add(1013904223)
			);
		}
		if !pooled || !persistent {
			allocator.unmap(&allocation).unwrap();
		}
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		if let Some((buffer, mut neighbor, pointer)) = sentinel {
			assert_eq!(allocator.statistics().native_allocation_count, 1);
			allocator.invalidate(&neighbor, 0, vk::WHOLE_SIZE).unwrap();
			assert!(
				std::slice::from_raw_parts(pointer.cast::<u8>(), COUNT * 4)
					.iter()
					.all(|&byte| byte == 0xA5)
			);
			allocator.unmap(&neighbor).unwrap();
			allocator.destroy_buffer(buffer, &mut neighbor).unwrap();
		}
		assert_eq!(
			allocator.statistics(),
			::vk::memory::AllocatorStats::default()
		);
		assert_eq!(allocator.live_count(), 0);
		assert_eq!(
			allocator.free(&mut allocation),
			Err(AllocError::UnknownAllocation)
		);
		allocator.close().unwrap();
		drop(allocator);
		device.destroy_fence(fence, None).unwrap();
		if api_version >= vk::API_VERSION_1_1 {
			// Exact fence completion above permits resetting and trimming this pool.
			device
				.reset_command_pool(pool, vk::CommandPoolResetFlags::empty())
				.unwrap();
			device
				.trim_command_pool(pool, vk::CommandPoolTrimFlags::default())
				.unwrap();
		}
		device.destroy_command_pool(pool, None).unwrap();
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
		drop(entry); // the device and instance retain the native library.
		device.destroy(None);
		instance.destroy(None);
	}
}
