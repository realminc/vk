//! Native library loading and instance lifetime proof.

/// Explicit opt-in: a requested driver test must not silently skip failures.
#[cfg(feature = "loader")]
#[test]
fn headless_instance_create_destroy() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: this opt-in test trusts the installed platform Vulkan loader.
	let entry = unsafe { ::vk::load::Entry::new() }.expect("load Vulkan");
	let version = entry
		.enumerate_instance_version()
		.expect("query Vulkan version");
	assert!(vk::api_version_major(version) >= 1);
	let app = vk::ApplicationInfo {
		api_version: vk::API_VERSION_1_0,
		..Default::default()
	};
	let info = vk::InstanceCreateInfo {
		p_application_info: &app,
		..Default::default()
	};
	// SAFETY: pointers refer to live inputs; no extensions or callbacks are used.
	let instance = unsafe { entry.create_instance(&info, None) }.expect("create headless instance");
	assert!(!instance.handle().is_null());
	assert_eq!(
		instance.capabilities().unwrap().api_version(),
		vk::API_VERSION_1_0
	);
	assert!(instance.fp().get_physical_device_features2.is_none());
	assert!(instance.fp().get_physical_device_features2_khr.is_none());
	// This must remain callable after Entry drops: Instance retains its library.
	drop(entry);
	// SAFETY: this instance has no children; allocator matches its creation.
	unsafe { instance.destroy(None) };
}

#[test]
fn native_device_dispatch_caps_requested_version_by_selected_physical_device() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: explicit installed-driver opt-in; all child objects are destroyed
	// before the instance, with no submitted work or custom allocator callbacks.
	unsafe {
		let entry = ::vk::Entry::new().expect("load Vulkan");
		let requested = entry.enumerate_instance_version().unwrap() & !0xfff;
		let app = vk::ApplicationInfo {
			api_version: requested,
			..Default::default()
		};
		let instance = entry
			.create_instance(
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
					.position(|properties| properties.queue_count > 0)
					.map(|family| (physical, u32::try_from(family).unwrap()))
			})
			.expect("physical device with a queue family");
		let physical_version = instance
			.get_physical_device_properties(physical)
			.unwrap()
			.api_version
			& !0xfff;
		let priority = 1.0;
		let queue = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = instance
			.create_device(
				physical,
				&vk::DeviceCreateInfo {
					queue_create_info_count: 1,
					p_queue_create_infos: &queue,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let effective = requested.min(physical_version);
		assert_eq!(device.capabilities().unwrap().api_version(), effective);
		assert!(device.fp().get_buffer_memory_requirements2_khr.is_none());
		assert_eq!(
			device.fp().get_buffer_memory_requirements2.is_some(),
			effective >= vk::API_VERSION_1_1
		);
		assert_eq!(
			device.fp().cmd_pipeline_barrier2.is_some(),
			effective >= vk::API_VERSION_1_3
		);
		native_core_1_0_objects(&device);
		device.destroy(None);
		instance.destroy(None);
	}
}

/// Exercise newly exposed native object/query operations without submitted work.
/// This does not qualify drawing, pipeline creation or presentation.
unsafe fn native_core_1_0_objects(device: &vk::Device) {
	// SAFETY: the opt-in caller provides a live device. Inputs remain initialized
	// and borrowed through synchronous calls; all children are explicitly destroyed.
	unsafe {
		let cache = device
			.create_pipeline_cache(&vk::PipelineCacheCreateInfo::default(), None)
			.unwrap();
		let data = device.get_pipeline_cache_data(cache).unwrap();
		assert!(
			!data.is_empty(),
			"pipeline cache serialization includes its header"
		);
		let imported = device
			.create_pipeline_cache(
				&vk::PipelineCacheCreateInfo {
					initial_data_size: data.len(),
					p_initial_data: data.as_ptr().cast(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		device.merge_pipeline_caches(cache, &[imported]).unwrap();
		device.destroy_pipeline_cache(imported, None).unwrap();
		device.destroy_pipeline_cache(cache, None).unwrap();

		let event = device
			.create_event(&vk::EventCreateInfo::default(), None)
			.unwrap();
		assert_eq!(
			device.get_event_status(event).unwrap(),
			vk::Result::EVENT_RESET
		);
		device.set_event(event).unwrap();
		assert_eq!(
			device.get_event_status(event).unwrap(),
			vk::Result::EVENT_SET
		);
		device.reset_event(event).unwrap();
		assert_eq!(
			device.get_event_status(event).unwrap(),
			vk::Result::EVENT_RESET
		);
		device.destroy_event(event, None).unwrap();
		let fence = device
			.create_fence(&vk::FenceCreateInfo::default(), None)
			.unwrap();
		assert_eq!(
			device.get_fence_status(fence).unwrap(),
			vk::Result::NOT_READY
		);
		device.destroy_fence(fence, None).unwrap();

		// All-zero fields are valid host representations for this pointer/integer
		// record; a graphics subpass with zero attachments requires no extra features.
		let subpass = vk::SubpassDescription {
			pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
			..std::mem::zeroed()
		};
		let pass = device
			.create_render_pass(
				&vk::RenderPassCreateInfo {
					subpass_count: 1,
					p_subpasses: &subpass,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let framebuffer = device
			.create_framebuffer(
				&vk::FramebufferCreateInfo {
					render_pass: pass,
					width: 3,
					height: 5,
					layers: 1,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut granularity = vk::Extent2D {
			width: 0,
			height: 0,
		};
		device
			.get_render_area_granularity(pass, &mut granularity)
			.unwrap();
		assert!(granularity.width > 0 && granularity.height > 0);
		device.destroy_framebuffer(framebuffer, None).unwrap();
		device.destroy_render_pass(pass, None).unwrap();
	}
}

/// This profile requires Vulkan 1.2 and supported timelineSemaphore. A requested
/// profile fails if those prerequisites are absent instead of reporting a skip.
#[test]
fn native_timeline_and_metadata_import_round_trip() {
	if std::env::var("VULKAN_TIMELINE_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: explicit installed-driver opt-in; all chains remain live during
	// capture/calls. Each native handle has one wrapper owner at a time and is
	// destroyed explicitly after its children. No work is submitted.
	unsafe {
		let entry = ::vk::Entry::new().unwrap();
		let implementation = entry.enumerate_instance_version().unwrap();
		assert!(implementation >= vk::API_VERSION_1_2);
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_2,
			..Default::default()
		};
		let instance_info = vk::InstanceCreateInfo {
			p_application_info: &app,
			..Default::default()
		};
		let created = entry.create_instance(&instance_info, None).unwrap();
		let instance_handle = created.handle();
		// Drop only releases Rust metadata/library references, never the native handle.
		drop(created);
		let metadata =
			::vk::Capabilities::new_from_instance_create_info(&instance_info, implementation).unwrap();
		let instance =
			vk::Instance::new_from_handle_with_capabilities(&entry, instance_handle, metadata).unwrap();
		let (physical, family, physical_version) = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				let version = instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version;
				if version < vk::API_VERSION_1_2 {
					return None;
				}
				instance
					.get_physical_device_queue_family_properties(physical)
					.unwrap()
					.iter()
					.position(|q| q.queue_count > 0)
					.map(|family| (physical, u32::try_from(family).unwrap(), version))
			})
			.expect("timeline profile requires a Vulkan 1.2 physical device and queue");
		let mut supported = vk::PhysicalDeviceTimelineSemaphoreFeatures::default();
		let mut query = vk::PhysicalDeviceFeatures2 {
			p_next: (&mut supported as *mut vk::PhysicalDeviceTimelineSemaphoreFeatures).cast(),
			..Default::default()
		};
		instance
			.get_physical_device_features2(physical, &mut query)
			.unwrap();
		assert_ne!(
			supported.timeline_semaphore,
			vk::FALSE,
			"timeline profile requires timelineSemaphore"
		);
		let enabled = vk::PhysicalDeviceTimelineSemaphoreFeatures {
			timeline_semaphore: vk::TRUE,
			..Default::default()
		};
		let priority = 1.0;
		let queue = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device_info = vk::DeviceCreateInfo {
			p_next: (&enabled as *const vk::PhysicalDeviceTimelineSemaphoreFeatures).cast(),
			queue_create_info_count: 1,
			p_queue_create_infos: &queue,
			..Default::default()
		};
		let created = instance
			.create_device(physical, &device_info, None)
			.unwrap();
		let device_handle = created.handle();
		drop(created);
		let metadata = instance
			.capabilities()
			.unwrap()
			.new_from_device_create_info(&device_info, physical_version)
			.unwrap();
		let device =
			vk::Device::new_from_handle_with_capabilities(&instance, device_handle, metadata).unwrap();
		device
			.capabilities()
			.unwrap()
			.features()
			.unwrap()
			.require(&[(
				vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
				"timelineSemaphore",
			)])
			.unwrap();
		assert!(device.fp().wait_semaphores_khr.is_none());
		let kind = vk::SemaphoreTypeCreateInfo {
			semaphore_type: vk::SemaphoreType::TIMELINE,
			initial_value: 7,
			..Default::default()
		};
		let semaphore = device
			.create_semaphore(
				&vk::SemaphoreCreateInfo {
					p_next: (&kind as *const vk::SemaphoreTypeCreateInfo).cast(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut counter = 0;
		device
			.get_semaphore_counter_value(semaphore, &mut counter)
			.unwrap();
		assert_eq!(counter, 7);
		let expected = 11;
		let wait = vk::SemaphoreWaitInfo {
			semaphore_count: 1,
			p_semaphores: &semaphore,
			p_values: &expected,
			..Default::default()
		};
		assert_eq!(
			device.wait_semaphores(&wait, 0).unwrap(),
			vk::Result::TIMEOUT
		);
		::vk::signal_semaphore(
			&device,
			&vk::SemaphoreSignalInfo {
				semaphore,
				value: expected,
				..Default::default()
			},
		)
		.unwrap();
		assert_eq!(
			device.wait_semaphores(&wait, 0).unwrap(),
			vk::Result::SUCCESS
		);
		device
			.get_semaphore_counter_value(semaphore, &mut counter)
			.unwrap();
		assert_eq!(counter, expected);
		device.destroy_semaphore(semaphore, None).unwrap();
		device.destroy(None);
		instance.destroy(None);
	}
}

#[test]
fn native_vulkan_1_1_feature_chain_and_output_queries() {
	if std::env::var("VULKAN_1_1_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: opt-in installed-driver profile. All complete chains remain live
	// through calls. Only queried supported features are enabled; no work is
	// submitted here and all native children are explicitly destroyed first.
	unsafe {
		let entry = ::vk::Entry::new().unwrap();
		assert!(entry.enumerate_instance_version().unwrap() >= vk::API_VERSION_1_1);
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = entry
			.create_instance(
				&vk::InstanceCreateInfo {
					p_application_info: &app,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut count = 0;
		let mut groups = None;
		for _ in 0..8 {
			assert_eq!(
				instance
					.enumerate_physical_device_groups(&mut count, None)
					.unwrap(),
				vk::Result::SUCCESS
			);
			let mut values = vec![vk::PhysicalDeviceGroupProperties::default(); count as usize];
			let status = instance
				.enumerate_physical_device_groups(&mut count, Some(&mut values))
				.unwrap();
			if status == vk::Result::SUCCESS {
				values.truncate(count as usize);
				groups = Some(values);
				break;
			}
		}
		let groups = groups.expect("device-group enumeration must stabilize");
		assert!(!groups.is_empty());
		let physical = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find(|&physical| {
				instance
					.get_physical_device_properties(physical)
					.unwrap()
					.api_version
					>= vk::API_VERSION_1_1
			})
			.expect("profile requires a Vulkan 1.1 physical device");
		assert!(groups.iter().any(|group| {
			group.physical_devices[..group.physical_device_count as usize].contains(&physical)
		}));
		instance
			.get_physical_device_queue_family_properties2(physical, &mut count, None)
			.unwrap();
		let mut families = vec![vk::QueueFamilyProperties2::default(); count as usize];
		instance
			.get_physical_device_queue_family_properties2(physical, &mut count, Some(&mut families))
			.unwrap();
		families.truncate(count as usize);
		let legacy = instance
			.get_physical_device_queue_family_properties(physical)
			.unwrap();
		assert_eq!(families.len(), legacy.len());
		for (new, old) in families.iter().zip(&legacy) {
			assert_eq!(new.queue_family_properties.queue_count, old.queue_count);
			assert_eq!(new.queue_family_properties.queue_flags, old.queue_flags);
		}
		let family = families
			.iter()
			.position(|q| q.queue_family_properties.queue_count > 0)
			.expect("queue family") as u32;

		// These six individual records are the 1.1 API. Vulkan11Features was
		// introduced in 1.2 and must not replace them in a 1.1 profile.
		let mut draw = vk::PhysicalDeviceShaderDrawParametersFeatures::default();
		let mut ycbcr = vk::PhysicalDeviceSamplerYcbcrConversionFeatures {
			p_next: (&mut draw as *mut vk::PhysicalDeviceShaderDrawParametersFeatures).cast(),
			..Default::default()
		};
		let mut protected = vk::PhysicalDeviceProtectedMemoryFeatures {
			p_next: (&mut ycbcr as *mut vk::PhysicalDeviceSamplerYcbcrConversionFeatures).cast(),
			..Default::default()
		};
		let mut variable = vk::PhysicalDeviceVariablePointersFeatures {
			p_next: (&mut protected as *mut vk::PhysicalDeviceProtectedMemoryFeatures).cast(),
			..Default::default()
		};
		let mut multiview = vk::PhysicalDeviceMultiviewFeatures {
			p_next: (&mut variable as *mut vk::PhysicalDeviceVariablePointersFeatures).cast(),
			..Default::default()
		};
		let mut storage = vk::PhysicalDevice16BitStorageFeatures {
			p_next: (&mut multiview as *mut vk::PhysicalDeviceMultiviewFeatures).cast(),
			..Default::default()
		};
		let mut supported = vk::PhysicalDeviceFeatures2 {
			p_next: (&mut storage as *mut vk::PhysicalDevice16BitStorageFeatures).cast(),
			..Default::default()
		};
		instance
			.get_physical_device_features2(physical, &mut supported)
			.unwrap();
		let expected = [
			(
				storage.s_type,
				"storageBuffer16BitAccess",
				storage.storage_buffer16_bit_access,
			),
			(
				storage.s_type,
				"uniformAndStorageBuffer16BitAccess",
				storage.uniform_and_storage_buffer16_bit_access,
			),
			(
				storage.s_type,
				"storagePushConstant16",
				storage.storage_push_constant16,
			),
			(
				storage.s_type,
				"storageInputOutput16",
				storage.storage_input_output16,
			),
			(multiview.s_type, "multiview", multiview.multiview),
			(
				multiview.s_type,
				"multiviewGeometryShader",
				multiview.multiview_geometry_shader,
			),
			(
				multiview.s_type,
				"multiviewTessellationShader",
				multiview.multiview_tessellation_shader,
			),
			(
				variable.s_type,
				"variablePointersStorageBuffer",
				variable.variable_pointers_storage_buffer,
			),
			(
				variable.s_type,
				"variablePointers",
				variable.variable_pointers,
			),
			(
				protected.s_type,
				"protectedMemory",
				protected.protected_memory,
			),
			(
				ycbcr.s_type,
				"samplerYcbcrConversion",
				ycbcr.sampler_ycbcr_conversion,
			),
			(
				draw.s_type,
				"shaderDrawParameters",
				draw.shader_draw_parameters,
			),
		];
		let priority = 1.0;
		let queue = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = instance
			.create_device(
				physical,
				&vk::DeviceCreateInfo {
					p_next: (&supported as *const vk::PhysicalDeviceFeatures2).cast(),
					queue_create_info_count: 1,
					p_queue_create_infos: &queue,
					..Default::default()
				},
				None,
			)
			.unwrap();
		assert_eq!(
			device.capabilities().unwrap().api_version(),
			vk::API_VERSION_1_1
		);
		let captured = device.capabilities().unwrap().features().unwrap();
		for (structure, member, value) in expected {
			assert_eq!(
				captured.enabled(structure, member),
				Some(value != vk::FALSE),
				"{member}"
			);
			assert_eq!(
				captured.require(&[(structure, member)]).is_ok(),
				value != vk::FALSE,
				"{member}"
			);
			eprintln!(
				"Vulkan 1.1 queried/enabled {member}: {}",
				value != vk::FALSE
			);
		}
		assert_eq!(
			captured.enabled(
				vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
				"multiview"
			),
			None
		);
		assert!(device.fp().create_descriptor_update_template.is_some());
		assert!(device.fp().create_descriptor_update_template_khr.is_none());
		assert!(device.fp().cmd_set_device_mask.is_some());
		assert!(
			device.fp().wait_semaphores.is_none(),
			"1.2 core stays disabled at 1.1"
		);

		let mut ids = vk::PhysicalDeviceIDProperties::default();
		let mut subgroup = vk::PhysicalDeviceSubgroupProperties {
			p_next: (&mut ids as *mut vk::PhysicalDeviceIDProperties).cast(),
			..Default::default()
		};
		let mut maintenance = vk::PhysicalDeviceMaintenance3Properties {
			p_next: (&mut subgroup as *mut vk::PhysicalDeviceSubgroupProperties).cast(),
			..Default::default()
		};
		let mut properties = vk::PhysicalDeviceProperties2 {
			p_next: (&mut maintenance as *mut vk::PhysicalDeviceMaintenance3Properties).cast(),
			..Default::default()
		};
		instance
			.get_physical_device_properties2(physical, &mut properties)
			.unwrap();
		assert!(properties.properties.api_version >= vk::API_VERSION_1_1);
		assert!(subgroup.subgroup_size > 0);
		assert!(maintenance.max_memory_allocation_size > 0);
		let mut memory = vk::PhysicalDeviceMemoryProperties2::default();
		instance
			.get_physical_device_memory_properties2(physical, &mut memory)
			.unwrap();
		assert!(memory.memory_properties.memory_type_count > 0);
		let format = vk::Format::R8G8B8A8_UNORM;
		let mut format_properties = vk::FormatProperties2::default();
		instance
			.get_physical_device_format_properties2(physical, format, &mut format_properties)
			.unwrap();
		let mut old = std::mem::zeroed::<vk::FormatProperties>();
		instance
			.get_physical_device_format_properties(physical, format, &mut old)
			.unwrap();
		assert_eq!(
			format_properties.format_properties.optimal_tiling_features,
			old.optimal_tiling_features
		);
		let mut image_properties = vk::ImageFormatProperties2::default();
		instance
			.get_physical_device_image_format_properties2(
				physical,
				&vk::PhysicalDeviceImageFormatInfo2 {
					format,
					r#type: vk::ImageType::_2D,
					tiling: vk::ImageTiling::OPTIMAL,
					usage: vk::ImageUsageFlagBits::SAMPLED_BIT,
					..Default::default()
				},
				&mut image_properties,
			)
			.unwrap();
		assert!(image_properties.image_format_properties.max_extent.width > 0);
		let sparse_info = vk::PhysicalDeviceSparseImageFormatInfo2 {
			format,
			r#type: vk::ImageType::_2D,
			samples: vk::SampleCountFlagBits::_1_BIT,
			usage: vk::ImageUsageFlagBits::SAMPLED_BIT,
			tiling: vk::ImageTiling::OPTIMAL,
			..Default::default()
		};
		instance
			.get_physical_device_sparse_image_format_properties2(physical, &sparse_info, &mut count, None)
			.unwrap();
		let mut sparse = vec![vk::SparseImageFormatProperties2::default(); count as usize];
		instance
			.get_physical_device_sparse_image_format_properties2(
				physical,
				&sparse_info,
				&mut count,
				Some(&mut sparse),
			)
			.unwrap();
		eprintln!("Vulkan 1.1 sparse format records: {count}");
		let mut buffer_external = vk::ExternalBufferProperties::default();
		instance
			.get_physical_device_external_buffer_properties(
				physical,
				&vk::PhysicalDeviceExternalBufferInfo {
					usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
					handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
					..Default::default()
				},
				&mut buffer_external,
			)
			.unwrap();
		let mut fence_external = vk::ExternalFenceProperties::default();
		instance
			.get_physical_device_external_fence_properties(
				physical,
				&vk::PhysicalDeviceExternalFenceInfo {
					handle_type: vk::ExternalFenceHandleTypeFlagBits::OPAQUE_FD_BIT,
					..Default::default()
				},
				&mut fence_external,
			)
			.unwrap();
		let mut semaphore_external = vk::ExternalSemaphoreProperties::default();
		instance
			.get_physical_device_external_semaphore_properties(
				physical,
				&vk::PhysicalDeviceExternalSemaphoreInfo {
					handle_type: vk::ExternalSemaphoreHandleTypeFlagBits::OPAQUE_FD_BIT,
					..Default::default()
				},
				&mut semaphore_external,
			)
			.unwrap();
		native_requirements_and_bind_memory2(&device, &memory.memory_properties);
		device.destroy(None);
		instance.destroy(None);
	}
}

/// Exercise 1.1 requirements2/dedication and bind-memory2 without submitted work.
unsafe fn native_requirements_and_bind_memory2(
	device: &vk::Device,
	properties: &vk::PhysicalDeviceMemoryProperties,
) {
	// SAFETY: caller owns a live device and matching memory properties; resources
	// are never used by the GPU and are destroyed before their allocations.
	unsafe {
		let buffer = device
			.create_buffer(
				&vk::BufferCreateInfo {
					size: 64,
					usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
					sharing_mode: vk::SharingMode::EXCLUSIVE,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut dedicated = vk::MemoryDedicatedRequirements::default();
		let mut requirements = vk::MemoryRequirements2 {
			p_next: (&mut dedicated as *mut vk::MemoryDedicatedRequirements).cast(),
			..Default::default()
		};
		device
			.get_buffer_memory_requirements2(
				&vk::BufferMemoryRequirementsInfo2 {
					buffer,
					..Default::default()
				},
				&mut requirements,
			)
			.unwrap();
		let memory_type_index = (0..properties.memory_type_count)
			.find(|index| requirements.memory_requirements.memory_type_bits & (1 << index) != 0)
			.unwrap();
		let dedication = vk::MemoryDedicatedAllocateInfo {
			buffer,
			..Default::default()
		};
		let allocation = device
			.allocate_memory(
				&vk::MemoryAllocateInfo {
					p_next: (&dedication as *const vk::MemoryDedicatedAllocateInfo).cast(),
					allocation_size: requirements.memory_requirements.size,
					memory_type_index,
					..Default::default()
				},
				None,
			)
			.unwrap();
		device
			.bind_buffer_memory2(&[vk::BindBufferMemoryInfo {
				buffer,
				memory: allocation,
				..Default::default()
			}])
			.unwrap();
		device.destroy_buffer(buffer, None).unwrap();
		device.free_memory(allocation, None).unwrap();

		let image = device
			.create_image(
				&vk::ImageCreateInfo {
					image_type: vk::ImageType::_2D,
					format: vk::Format::R8G8B8A8_UNORM,
					extent: vk::Extent3D {
						width: 4,
						height: 4,
						depth: 1,
					},
					mip_levels: 1,
					array_layers: 1,
					samples: vk::SampleCountFlagBits::_1_BIT,
					tiling: vk::ImageTiling::OPTIMAL,
					usage: vk::ImageUsageFlagBits::SAMPLED_BIT,
					sharing_mode: vk::SharingMode::EXCLUSIVE,
					initial_layout: vk::ImageLayout::UNDEFINED,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let mut dedicated = vk::MemoryDedicatedRequirements::default();
		let mut requirements = vk::MemoryRequirements2 {
			p_next: (&mut dedicated as *mut vk::MemoryDedicatedRequirements).cast(),
			..Default::default()
		};
		device
			.get_image_memory_requirements2(
				&vk::ImageMemoryRequirementsInfo2 {
					image,
					..Default::default()
				},
				&mut requirements,
			)
			.unwrap();
		let memory_type_index = (0..properties.memory_type_count)
			.find(|index| requirements.memory_requirements.memory_type_bits & (1 << index) != 0)
			.unwrap();
		let dedication = vk::MemoryDedicatedAllocateInfo {
			image,
			..Default::default()
		};
		let allocation = device
			.allocate_memory(
				&vk::MemoryAllocateInfo {
					p_next: (&dedication as *const vk::MemoryDedicatedAllocateInfo).cast(),
					allocation_size: requirements.memory_requirements.size,
					memory_type_index,
					..Default::default()
				},
				None,
			)
			.unwrap();
		device
			.bind_image_memory2(&[vk::BindImageMemoryInfo {
				image,
				memory: allocation,
				..Default::default()
			}])
			.unwrap();
		let sparse_info = vk::ImageSparseMemoryRequirementsInfo2 {
			image,
			..Default::default()
		};
		let mut count = u32::MAX;
		device
			.get_image_sparse_memory_requirements2(&sparse_info, &mut count, None)
			.unwrap();
		assert_eq!(count, 0, "ordinary image has no sparse requirements");
		device
			.get_image_sparse_memory_requirements2(&sparse_info, &mut count, Some(&mut []))
			.unwrap();
		assert_eq!(count, 0);
		device.destroy_image(image, None).unwrap();
		device.free_memory(allocation, None).unwrap();
	}
}

#[test]
fn native_vulkan_1_2_host_query_reset_and_render_pass2_submission() {
	if std::env::var("VULKAN_1_2_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: explicit driver opt-in; live input records, one graphics queue,
	// exact fence completion, then explicit child-first destruction. No draws,
	// attachments, address queries or unsatisfied optional features are used.
	unsafe {
		let entry = ::vk::Entry::new().unwrap();
		assert!(entry.enumerate_instance_version().unwrap() >= vk::API_VERSION_1_2);
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_2,
			..Default::default()
		};
		let instance = entry
			.create_instance(
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
				let properties = instance.get_physical_device_properties(physical).unwrap();
				if properties.api_version < vk::API_VERSION_1_2 {
					return None;
				}
				instance
					.get_physical_device_queue_family_properties(physical)
					.unwrap()
					.iter()
					.position(|q| {
						q.queue_count > 0
							&& q.queue_flags.as_raw() & vk::QueueFlagBits::GRAPHICS_BIT.as_raw() != 0
					})
					.map(|family| (physical, u32::try_from(family).unwrap()))
			})
			.expect("1.2 profile requires a Vulkan 1.2 graphics queue");
		let mut supported = vk::PhysicalDeviceVulkan12Features::default();
		instance
			.get_physical_device_features2(
				physical,
				&mut vk::PhysicalDeviceFeatures2 {
					p_next: (&mut supported as *mut vk::PhysicalDeviceVulkan12Features).cast(),
					..Default::default()
				},
			)
			.unwrap();
		assert_eq!(
			supported.host_query_reset,
			vk::TRUE,
			"profile requires hostQueryReset"
		);
		eprintln!(
			"1.2 queried features: hostQueryReset={}, drawIndirectCount={}, bufferDeviceAddress={}, bufferDeviceAddressCaptureReplay={}; this profile executes host reset and attachment-free render-pass2 only",
			supported.host_query_reset,
			supported.draw_indirect_count,
			supported.buffer_device_address,
			supported.buffer_device_address_capture_replay
		);
		let enabled = vk::PhysicalDeviceVulkan12Features {
			host_query_reset: vk::TRUE,
			..Default::default()
		};
		let priority = 1.0;
		let queue_info = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let device = instance
			.create_device(
				physical,
				&vk::DeviceCreateInfo {
					p_next: (&enabled as *const vk::PhysicalDeviceVulkan12Features).cast(),
					queue_create_info_count: 1,
					p_queue_create_infos: &queue_info,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let features = device.capabilities().unwrap().features().unwrap();
		features
			.require(&[(
				vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
				"hostQueryReset",
			)])
			.unwrap();
		assert_eq!(
			features.enabled(
				vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
				"bufferDeviceAddress"
			),
			Some(false)
		);
		assert!(device.fp().reset_query_pool_ext.is_none());
		assert!(device.fp().create_render_pass2_khr.is_none());
		assert!(device.fp().get_buffer_device_address_khr.is_none());
		assert!(device.fp().get_buffer_device_address_ext.is_none());
		assert!(device.fp().cmd_draw_indirect_count_amd.is_none());
		let pool = device
			.create_query_pool(
				&vk::QueryPoolCreateInfo {
					query_type: vk::QueryType::OCCLUSION,
					query_count: 1,
					..Default::default()
				},
				None,
			)
			.unwrap();
		::vk::reset_query_pool(&device, pool, 0, 1).unwrap();
		let mut results = [u64::MAX; 2];
		// A byte-slice API retains the caller's native eight-byte alignment.
		let data = std::slice::from_raw_parts_mut(results.as_mut_ptr().cast::<u8>(), 16);
		let flags = vk::QueryResultFlags::from_raw(
			vk::QueryResultFlagBits::_64_BIT.as_raw()
				| vk::QueryResultFlagBits::WITH_AVAILABILITY_BIT.as_raw(),
		);
		assert_eq!(
			device
				.get_query_pool_results(pool, 0, 1, data, 16, flags)
				.unwrap(),
			vk::Result::NOT_READY
		);
		assert_eq!(results[1], 0);
		device.destroy_query_pool(pool, None).unwrap();

		let subpasses = [vk::SubpassDescription2 {
			pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
			..Default::default()
		}; 2];
		let render = device
			.create_render_pass2(
				&vk::RenderPassCreateInfo2 {
					subpass_count: 2,
					p_subpasses: subpasses.as_ptr(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		let framebuffer = device
			.create_framebuffer(
				&vk::FramebufferCreateInfo {
					render_pass: render,
					width: 1,
					height: 1,
					layers: 1,
					..Default::default()
				},
				None,
			)
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
		device
			.begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())
			.unwrap();
		let begin = vk::SubpassBeginInfo {
			contents: vk::SubpassContents::INLINE,
			..Default::default()
		};
		let end = vk::SubpassEndInfo::default();
		device
			.cmd_begin_render_pass2(
				command,
				&vk::RenderPassBeginInfo {
					render_pass: render,
					framebuffer,
					render_area: vk::Rect2D {
						offset: vk::Offset2D { x: 0, y: 0 },
						extent: vk::Extent2D {
							width: 1,
							height: 1,
						},
					},
					..Default::default()
				},
				&begin,
			)
			.unwrap();
		::vk::cmd_next_subpass2(&device, command, &begin, &end).unwrap();
		device.cmd_end_render_pass2(command, &end).unwrap();
		device.end_command_buffer(command).unwrap();
		let queue = device.get_device_queue(family, 0).unwrap();
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
		assert!(device.wait_for_fences(&[fence], true, u64::MAX).unwrap());
		device.destroy_fence(fence, None).unwrap();
		device
			.free_command_buffers(command_pool, &[command])
			.unwrap();
		device.destroy_command_pool(command_pool, None).unwrap();
		device.destroy_framebuffer(framebuffer, None).unwrap();
		device.destroy_render_pass(render, None).unwrap();
		device.destroy(None);
		instance.destroy(None);
	}
}
