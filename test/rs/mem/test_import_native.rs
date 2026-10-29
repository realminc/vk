//! Real POSIX external-memory round trip; no synthetic descriptor admission proof.
#![cfg(target_os = "linux")]
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};
use vk::mem::{AllocationChain, AllocationCreateInfo, ImportMemoryInfo, ImportResource};

#[test]
fn opaque_fd_buffer_export_import_binding_mapping_and_accounting() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: queried same-device ordinary external-memory profile below.
	unsafe { buffer_round_trip(vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT) };
}

#[test]
fn dma_buf_buffer_export_import_properties_and_payload() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: exact exported DMA-BUF, queried type bits and explicit host ownership.
	unsafe { buffer_round_trip(vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_BIT_EXT) };
}

unsafe fn buffer_round_trip(handle_type: vk::ExternalMemoryHandleTypeFlagBits) {
	// SAFETY: same device/driver payload, queried external support and exact type /
	// extent provenance. Host accesses are serialized; no GPU work is submitted.
	// Successful native import consumes the descriptor, and all resources/memory
	// are explicitly destroyed before their device and instance.
	unsafe {
		let Some((instance, physical, device)) = super::synchronized::contexts(
			if handle_type == vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_BIT_EXT {
				&[
					c"VK_KHR_external_memory_fd",
					c"VK_EXT_external_memory_dma_buf",
				]
			} else {
				&[c"VK_KHR_external_memory_fd"]
			},
		) else {
			return;
		};
		let usage =
			vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT;
		let mut external = vk::ExternalBufferProperties::default();
		instance
			.get_physical_device_external_buffer_properties(
				physical,
				&vk::PhysicalDeviceExternalBufferInfo {
					usage,
					handle_type,
					..Default::default()
				},
				&mut external,
			)
			.unwrap();
		let features = external
			.external_memory_properties
			.external_memory_features
			.as_raw();
		let required = (vk::ExternalMemoryFeatureFlagBits::IMPORTABLE_BIT
			| vk::ExternalMemoryFeatureFlagBits::EXPORTABLE_BIT)
			.as_raw();
		if features & required != required {
			eprintln!("SKIP: {handle_type:?} buffer import/export not supported: {features:#x}");
			device.destroy(None);
			instance.destroy(None);
			return;
		}
		let external_info = vk::ExternalMemoryBufferCreateInfo {
			handle_types: handle_type,
			..Default::default()
		};
		let info = vk::BufferCreateInfo {
			p_next: std::ptr::from_ref(&external_info).cast(),
			size: 4096,
			usage,
			..Default::default()
		};
		let mut exporter =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let options = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			preferred_flags: vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT,
			..Default::default()
		};
		let index = match exporter.find_memory_type_for_buffer_info(&info, &options) {
			Ok(index) => index,
			Err(vk::AllocError::NoCompatibleMemoryType) => {
				eprintln!("SKIP: no host-visible external-buffer memory type");
				exporter.close().unwrap();
				drop(exporter);
				device.destroy(None);
				instance.destroy(None);
				return;
			}
			Err(error) => panic!("external memory type query: {error}"),
		};
		let count = exporter.memory_properties().unwrap().memory_type_count as usize;
		let mut types = vec![vk::ExternalMemoryHandleTypeFlags::empty(); count];
		types[index as usize] = handle_type;
		exporter.set_external_memory_types(&types).unwrap();
		let (source, mut source_allocation) = exporter
			.create_buffer_with_options(
				&info,
				&AllocationCreateInfo {
					memory_type_bits: 1 << index,
					..options
				},
			)
			.unwrap();
		let source_pointer = exporter.map(&source_allocation).unwrap().cast::<u8>();
		std::slice::from_raw_parts_mut(source_pointer, 4096).fill(0xA7);
		exporter.flush(&source_allocation, 0, 4096).unwrap();
		exporter.unmap(&source_allocation).unwrap();
		let descriptor = OwnedFd::from_raw_fd(
			exporter
				.get_memory_fd(&source_allocation, handle_type)
				.unwrap(),
		);
		let compatible_bits = if handle_type == vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_BIT_EXT {
			let mut properties = vk::MemoryFdPropertiesKHR::default();
			assert_eq!(
				device
					.get_memory_fd_properties_khr(handle_type, descriptor.as_raw_fd(), &mut properties)
					.unwrap(),
				vk::Result::SUCCESS
			);
			assert_ne!(properties.memory_type_bits, 0);
			properties.memory_type_bits
		} else {
			1 << source_allocation.memory_type_index()
		};
		let destination = device.create_buffer(&info, None).unwrap();
		let record = vk::ImportMemoryFdInfoKHR {
			handle_type,
			fd: descriptor.as_raw_fd(),
			..Default::default()
		};
		// OPAQUE_FD uses the exporting allocation's exact size/type, not the
		// vkGetMemoryFdPropertiesKHR query forbidden for this handle type.
		let chain = AllocationChain::new_from_record_with_import(
			&record,
			ImportMemoryInfo {
				allocation_size: source_allocation.reserved_size(),
				memory_type_bits: compatible_bits,
				resource: Some(ImportResource::Buffer(destination)),
			},
		)
		.unwrap();
		let mut importer =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let mut destination_allocation = importer
			.allocate_for_buffer(
				destination,
				&AllocationCreateInfo {
					memory_allocate_next: Some(chain),
					required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
					..Default::default()
				},
			)
			.unwrap();
		// This route publishes immediately after native import (no subsequent
		// mapping/binding work). Stop Rust ownership before any other fallible step.
		let _transferred = descriptor.into_raw_fd();
		assert_ne!(source_allocation.handle(), destination_allocation.handle());
		assert_eq!(
			importer.statistics().native_bytes,
			exporter.statistics().native_bytes
		);
		assert_eq!(
			importer.statistics().allocation_bytes,
			source_allocation.reserved_size()
		);
		importer
			.bind_buffer_memory(destination, &destination_allocation)
			.unwrap();
		let pointer = importer.map(&destination_allocation).unwrap().cast::<u8>();
		importer
			.invalidate(&destination_allocation, 0, 4096)
			.unwrap();
		assert!(
			std::slice::from_raw_parts(pointer, 4096)
				.iter()
				.all(|&byte| byte == 0xA7)
		);
		std::slice::from_raw_parts_mut(pointer, 4096).fill(0x5C);
		importer.flush(&destination_allocation, 0, 4096).unwrap();
		importer.unmap(&destination_allocation).unwrap();
		// Both buffers/objects share a payload but no host-byte accesses overlap.
		let pointer = exporter.map(&source_allocation).unwrap().cast::<u8>();
		exporter.invalidate(&source_allocation, 0, 4096).unwrap();
		assert!(
			std::slice::from_raw_parts(pointer, 4096)
				.iter()
				.all(|&byte| byte == 0x5C)
		);
		exporter.unmap(&source_allocation).unwrap();
		// Externally created destination uses the original None callbacks.
		device.destroy_buffer(destination, None).unwrap();
		importer.free(&mut destination_allocation).unwrap();
		importer.close().unwrap();
		exporter
			.destroy_buffer(source, &mut source_allocation)
			.unwrap();
		exporter.close().unwrap();
		assert_eq!(importer.statistics().native_bytes, 0);
		assert_eq!(exporter.statistics().native_bytes, 0);
		drop(importer);
		drop(exporter);
		device.destroy(None);
		instance.destroy(None);
		eprintln!(
			"PASS: real buffer {handle_type:?} export/import, dedicated bind, bidirectional 4096-byte payload, exact accounting and cleanup"
		);
	}
}

#[test]
fn opaque_fd_linear_image_export_import_pixels_and_accounting() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: exact queried external linear-image parameters, matching dedicated
	// imports, explicit GENERAL layout initialization and serialized host access. Only subresource rows
	// reported by the native layout are touched; no GPU work is submitted.
	unsafe {
		let Some((instance, physical, device)) =
			super::synchronized::contexts(&[c"VK_KHR_external_memory_fd"])
		else {
			return;
		};
		let handle_type = vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT;
		let format = vk::Format::R8G8B8A8_UNORM;
		let usage = vk::ImageUsageFlagBits::TRANSFER_SRC_BIT | vk::ImageUsageFlagBits::TRANSFER_DST_BIT;
		let external_query = vk::PhysicalDeviceExternalImageFormatInfo {
			handle_type,
			..Default::default()
		};
		let mut external = vk::ExternalImageFormatProperties::default();
		let mut output = vk::ImageFormatProperties2 {
			p_next: std::ptr::from_mut(&mut external).cast(),
			..Default::default()
		};
		let result = instance.get_physical_device_image_format_properties2(
			physical,
			&vk::PhysicalDeviceImageFormatInfo2 {
				p_next: std::ptr::from_ref(&external_query).cast(),
				format,
				r#type: vk::ImageType::_2D,
				flags: vk::ImageCreateFlagBits::ALIAS_BIT,
				tiling: vk::ImageTiling::LINEAR,
				usage,
				..Default::default()
			},
			&mut output,
		);
		if matches!(
			result,
			Err(vk::CallError::Vulkan(
				vk::Result::ERROR_FORMAT_NOT_SUPPORTED
			))
		) {
			eprintln!("SKIP: external linear RGBA image format unsupported");
			device.destroy(None);
			instance.destroy(None);
			return;
		}
		result.unwrap();
		let needed = (vk::ExternalMemoryFeatureFlagBits::IMPORTABLE_BIT
			| vk::ExternalMemoryFeatureFlagBits::EXPORTABLE_BIT)
			.as_raw();
		if external
			.external_memory_properties
			.external_memory_features
			.as_raw()
			& needed
			!= needed
		{
			eprintln!("SKIP: external linear RGBA image import/export unsupported");
			device.destroy(None);
			instance.destroy(None);
			return;
		}
		let external_info = vk::ExternalMemoryImageCreateInfo {
			handle_types: handle_type,
			..Default::default()
		};
		let info = vk::ImageCreateInfo {
			p_next: std::ptr::from_ref(&external_info).cast(),
			image_type: vk::ImageType::_2D,
			flags: vk::ImageCreateFlagBits::ALIAS_BIT,
			format,
			extent: vk::Extent3D {
				width: 32,
				height: 32,
				depth: 1,
			},
			mip_levels: 1,
			array_layers: 1,
			samples: vk::SampleCountFlagBits::_1_BIT,
			tiling: vk::ImageTiling::LINEAR,
			usage,
			initial_layout: vk::ImageLayout::UNDEFINED,
			..Default::default()
		};
		let mut exporter =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let options = AllocationCreateInfo {
			required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
			..Default::default()
		};
		let index = match exporter.find_memory_type_for_image_info(&info, &options) {
			Ok(index) => index,
			Err(vk::AllocError::NoCompatibleMemoryType) => {
				eprintln!("SKIP: external image has no host-visible type");
				exporter.close().unwrap();
				drop(exporter);
				device.destroy(None);
				instance.destroy(None);
				return;
			}
			Err(error) => panic!("external image type query: {error}"),
		};
		let mut types = vec![
			vk::ExternalMemoryHandleTypeFlags::empty();
			exporter.memory_properties().unwrap().memory_type_count as usize
		];
		types[index as usize] = handle_type;
		exporter.set_external_memory_types(&types).unwrap();
		let (source, mut source_allocation) = exporter
			.create_image_with_options(
				&info,
				&AllocationCreateInfo {
					memory_type_bits: 1 << index,
					..options
				},
			)
			.unwrap();
		let subresource = vk::ImageSubresource {
			aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
			mip_level: 0,
			array_layer: 0,
		};
		let mut source_layout = vk::SubresourceLayout::default();
		device
			.get_image_subresource_layout(source, &subresource, &mut source_layout)
			.unwrap();
		let descriptor = OwnedFd::from_raw_fd(
			exporter
				.get_memory_fd(&source_allocation, handle_type)
				.unwrap(),
		);
		let destination = device.create_image(&info, None).unwrap();
		let record = vk::ImportMemoryFdInfoKHR {
			handle_type,
			fd: descriptor.as_raw_fd(),
			..Default::default()
		};
		let chain = AllocationChain::new_from_record_with_import(
			&record,
			ImportMemoryInfo {
				allocation_size: source_allocation.reserved_size(),
				memory_type_bits: 1 << index,
				resource: Some(ImportResource::Image(destination)),
			},
		)
		.unwrap();
		let mut importer =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let mut allocation = importer
			.allocate_for_image(
				destination,
				&AllocationCreateInfo {
					memory_allocate_next: Some(chain),
					..Default::default()
				},
			)
			.unwrap();
		let _transferred = descriptor.into_raw_fd();
		assert_eq!(
			importer.statistics().native_bytes,
			exporter.statistics().native_bytes
		);
		importer
			.bind_image_memory(destination, &allocation)
			.unwrap();
		// External images must begin UNDEFINED. Initialize both aliases before
		// writing pixels; no later UNDEFINED transition can discard that payload.
		initialize_image_layouts(&instance, physical, &device, &[source, destination]);
		let source_pointer = exporter.map(&source_allocation).unwrap().cast::<u8>();
		pixel_rows(
			source_pointer,
			source_allocation.size(),
			&source_layout,
			Some(0xA7),
			None,
		);
		exporter
			.flush(&source_allocation, 0, vk::WHOLE_SIZE)
			.unwrap();
		exporter.unmap(&source_allocation).unwrap();

		let mut layout = vk::SubresourceLayout::default();
		device
			.get_image_subresource_layout(destination, &subresource, &mut layout)
			.unwrap();
		let pointer = importer.map(&allocation).unwrap().cast::<u8>();
		importer.invalidate(&allocation, 0, vk::WHOLE_SIZE).unwrap();
		pixel_rows(pointer, allocation.size(), &layout, Some(0x5C), Some(0xA7));
		importer.flush(&allocation, 0, vk::WHOLE_SIZE).unwrap();
		importer.unmap(&allocation).unwrap();
		let pointer = exporter.map(&source_allocation).unwrap().cast::<u8>();
		exporter
			.invalidate(&source_allocation, 0, vk::WHOLE_SIZE)
			.unwrap();
		pixel_rows(
			pointer,
			source_allocation.size(),
			&source_layout,
			None,
			Some(0x5C),
		);
		exporter.unmap(&source_allocation).unwrap();
		importer
			.destroy_image(destination, &mut allocation)
			.unwrap();
		importer.close().unwrap();
		exporter
			.destroy_image(source, &mut source_allocation)
			.unwrap();
		exporter.close().unwrap();
		drop(importer);
		drop(exporter);
		device.destroy(None);
		instance.destroy(None);
		eprintln!(
			"PASS: real opaque-FD linear RGBA image, 4096 pixel bytes each direction, native row pitches and exact accounting"
		);
	}
}

unsafe fn pixel_rows(
	pointer: *mut u8,
	size: u64,
	layout: &vk::SubresourceLayout,
	write: Option<u8>,
	expected: Option<u8>,
) {
	for row in 0..32_u64 {
		let offset = layout
			.offset
			.checked_add(row.checked_mul(layout.row_pitch).unwrap())
			.unwrap();
		assert!(layout.row_pitch >= 128 && offset.checked_add(128).unwrap() <= size);
		// SAFETY: explicit mapping covers the allocation; native linear-image layout
		// reports these in-bounds pixel rows and no other host access overlaps.
		let bytes =
			unsafe { std::slice::from_raw_parts_mut(pointer.add(usize::try_from(offset).unwrap()), 128) };
		if let Some(expected) = expected {
			assert!(bytes.iter().all(|&byte| byte == expected));
		}
		if let Some(pattern) = write {
			bytes.fill(pattern);
		}
	}
}

unsafe fn initialize_image_layouts(
	instance: &vk::Instance,
	physical: vk::PhysicalDevice,
	device: &vk::Device,
	images: &[vk::Image],
) {
	// SAFETY: helper owns this one command/fence; all images are live, bound,
	// identical linear aliases. Both layout transitions finish before host writes.
	unsafe {
		let family = instance
			.get_physical_device_queue_family_properties(physical)
			.unwrap()
			.iter()
			.position(|family| family.queue_count > 0)
			.unwrap() as u32;
		let queue = device.get_device_queue(family, 0).unwrap();
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
		for &image in images {
			device
				.cmd_pipeline_barrier(
					command,
					vk::PipelineStageFlagBits::TOP_OF_PIPE_BIT,
					vk::PipelineStageFlagBits::HOST_BIT,
					vk::DependencyFlags::empty(),
					&[],
					&[],
					&[vk::ImageMemoryBarrier {
						dst_access_mask: vk::AccessFlagBits::HOST_READ_BIT | vk::AccessFlagBits::HOST_WRITE_BIT,
						old_layout: vk::ImageLayout::UNDEFINED,
						new_layout: vk::ImageLayout::GENERAL,
						src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
						image,
						subresource_range: vk::ImageSubresourceRange {
							aspect_mask: vk::ImageAspectFlagBits::COLOR_BIT,
							level_count: 1,
							layer_count: 1,
							..Default::default()
						},
						..Default::default()
					}],
				)
				.unwrap();
		}
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
		device.destroy_fence(fence, None).unwrap();
		device.destroy_command_pool(pool, None).unwrap();
	}
}
