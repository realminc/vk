//! Offscreen Vulkan 1.3 triangle, native allocator and independently checked pixels.
//! No window system, implicit cleanup, timing claim or hardware fallback.
use std::{error::Error, fs, path::Path};
use vk::mem::{AllocationCreateInfo, HostAccess, MemoryUsage};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const EDGE: u32 = 64;
const BYTES: u64 = (EDGE * EDGE * 4) as u64;

fn words(path: &Path) -> Result<Vec<u32>> {
	let bytes = fs::read(path)?;
	if bytes.len() % 4 != 0 {
		return Err("SPIR-V is not word-sized".into());
	}
	let words: Vec<_> = bytes
		.as_chunks::<4>()
		.0
		.iter()
		.map(|bytes| u32::from_le_bytes(*bytes))
		.collect();
	if words.first() != Some(&0x07230203) {
		return Err("invalid SPIR-V magic".into());
	}
	Ok(words)
}

fn run() -> Result<()> {
	let args: Vec<_> = std::env::args_os().skip(1).collect();
	if args.len() != 3 {
		return Err(
			"usage: vk-triangle vertex.spv fragment.spv output.ppm; VK_SDK_DEVICE selects index".into(),
		);
	}
	let vertex = words(Path::new(&args[0]))?;
	let fragment = words(Path::new(&args[1]))?;
	let index: usize = std::env::var("VK_SDK_DEVICE")
		.unwrap_or_else(|_| "0".into())
		.parse()?;
	// SAFETY: this executable owns all native objects and waits for the exact fence
	// before readback/destruction. Any error exits the process; this is not a reusable
	// cleanup wrapper. Shader bytes are compiled by the documented SDK build step.
	unsafe {
		let entry = vk::Entry::new()?;
		if entry
			.enumerate_instance_version()
			.map_err(vk::CallError::from)?
			< vk::API_VERSION_1_3
		{
			return Err("Vulkan 1.3 loader required".into());
		}
		let app = vk::ApplicationInfo::new()
			.set_application_name(c"vk SDK triangle")
			.set_api_version(vk::API_VERSION_1_3);
		let instance = vk::Instance::new(
			&entry,
			&vk::InstanceCreateInfo::new().set_application_info(&app),
			None,
		)?;
		let physical = *instance
			.enumerate_physical_devices()?
			.get(index)
			.ok_or("device index absent")?;
		let properties = instance.get_physical_device_properties(physical)?;
		let mut ids = vk::PhysicalDeviceIDProperties::new();
		{
			let mut query = vk::PhysicalDeviceProperties2::new().push_next(&mut ids)?;
			instance.get_physical_device_properties2(physical, &mut query)?;
		}
		let identity = serde_json::json!({"device_uuid": ids.device_uuid,
			"driver_uuid": ids.driver_uuid, "api_version": properties.api_version,
			"driver_version": properties.driver_version, "device_index": index,
			"device_name": std::ffi::CStr::from_ptr(properties.device_name.as_ptr()).to_string_lossy()});
		if properties.api_version < vk::API_VERSION_1_3
			|| properties.device_type == vk::PhysicalDeviceType::CPU
		{
			return Err("physical Vulkan 1.3 GPU required".into());
		}
		let mut supported = vk::PhysicalDeviceVulkan13Features::new();
		{
			let mut query = vk::PhysicalDeviceFeatures2::new().push_next(&mut supported)?;
			instance.get_physical_device_features2(physical, &mut query)?;
		}
		if supported.dynamic_rendering == 0 || supported.synchronization2 == 0 {
			return Err("dynamicRendering/synchronization2 absent".into());
		}
		let family = instance
			.get_physical_device_queue_family_properties(physical)?
			.iter()
			.position(|q| {
				q.queue_count > 0 && q.queue_flags.as_raw() & vk::QueueFlags::GRAPHICS_BIT.as_raw() != 0
			})
			.ok_or("graphics queue absent")? as u32;
		let priorities = [1.0];
		let queues = [vk::DeviceQueueCreateInfo::new()
			.set_queue_family_index(family)
			.set_queue_priorities(&priorities)?];
		let mut enabled = vk::PhysicalDeviceVulkan13Features::new()
			.set_dynamic_rendering(1)
			.set_synchronization2(1);
		let create = vk::DeviceCreateInfo::new()
			.set_queue_create_infos(&queues)?
			.push_next(&mut enabled)?;
		let device = vk::Device::new(&instance, physical, &create, None)?;
		let queue = device.get_device_queue(family, 0)?;
		let mut allocator = vk::Allocator::new_from_physical_device(&instance, physical, &device)?;
		let image_info = vk::ImageCreateInfo::new()
			.set_image_type(vk::ImageType::_2D)
			.set_format(vk::Format::R8G8B8A8_UNORM)
			.set_extent(vk::Extent3D {
				width: EDGE,
				height: EDGE,
				depth: 1,
			})
			.set_mip_levels(1)
			.set_array_layers(1)
			.set_samples(vk::SampleCountFlagBits::_1_BIT)
			.set_tiling(vk::ImageTiling::OPTIMAL)
			.set_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT_BIT | vk::ImageUsageFlags::TRANSFER_SRC_BIT)
			.set_sharing_mode(vk::SharingMode::EXCLUSIVE)
			.set_initial_layout(vk::ImageLayout::UNDEFINED);
		let (image, mut image_memory) = allocator.create_image_with_options(
			&image_info,
			&AllocationCreateInfo::new().set_usage(MemoryUsage::PreferDevice),
		)?;
		let (buffer, mut buffer_memory) = allocator.create_buffer_with_options(
			&vk::BufferCreateInfo::new()
				.set_size(BYTES)
				.set_usage(vk::BufferUsageFlags::TRANSFER_DST_BIT),
			&AllocationCreateInfo::new()
				.set_usage(MemoryUsage::PreferHost)
				.set_host_access(HostAccess::Random)
				.set_required_flags(vk::MemoryPropertyFlags::HOST_VISIBLE_BIT),
		)?;
		let range = vk::ImageSubresourceRange::new()
			.set_aspect_mask(vk::ImageAspectFlags::COLOR_BIT)
			.set_level_count(1)
			.set_layer_count(1);
		let view = device.create_image_view(
			&vk::ImageViewCreateInfo::new()
				.set_image(image)
				.set_view_type(vk::ImageViewType::_2D)
				.set_format(vk::Format::R8G8B8A8_UNORM)
				.set_subresource_range(range),
			None,
		)?;
		let vert =
			device.create_shader_module(&vk::ShaderModuleCreateInfo::new().set_code(&vertex), None)?;
		let frag =
			device.create_shader_module(&vk::ShaderModuleCreateInfo::new().set_code(&fragment), None)?;
		let layout = device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::new(), None)?;
		let stages = [
			vk::PipelineShaderStageCreateInfo::new()
				.set_stage(vk::ShaderStageFlagBits::VERTEX_BIT)
				.set_module(vert)
				.set_name(c"main"),
			vk::PipelineShaderStageCreateInfo::new()
				.set_stage(vk::ShaderStageFlagBits::FRAGMENT_BIT)
				.set_module(frag)
				.set_name(c"main"),
		];
		let vertex_input = vk::PipelineVertexInputStateCreateInfo::new();
		let assembly = vk::PipelineInputAssemblyStateCreateInfo::new()
			.set_topology(vk::PrimitiveTopology::TRIANGLE_LIST);
		let viewports = [vk::Viewport::new()
			.set_width(EDGE as f32)
			.set_height(EDGE as f32)
			.set_min_depth(0.0)
			.set_max_depth(1.0)];
		let rect = vk::Rect2D::new().set_extent(vk::Extent2D {
			width: EDGE,
			height: EDGE,
		});
		let scissors = [rect];
		let viewport = vk::PipelineViewportStateCreateInfo::new()
			.set_viewports(&viewports)?
			.set_scissors(&scissors)?;
		let raster = vk::PipelineRasterizationStateCreateInfo::new()
			.set_polygon_mode(vk::PolygonMode::FILL)
			.set_cull_mode(vk::CullModeFlags::empty())
			.set_line_width(1.0);
		let multisample = vk::PipelineMultisampleStateCreateInfo::new()
			.set_rasterization_samples(vk::SampleCountFlagBits::_1_BIT);
		let attachments = [
			vk::PipelineColorBlendAttachmentState::new().set_color_write_mask(
				vk::ColorComponentFlags::R_BIT
					| vk::ColorComponentFlags::G_BIT
					| vk::ColorComponentFlags::B_BIT
					| vk::ColorComponentFlags::A_BIT,
			),
		];
		let blend = vk::PipelineColorBlendStateCreateInfo::new().set_attachments(&attachments)?;
		let formats = [vk::Format::R8G8B8A8_UNORM];
		let mut rendering =
			vk::PipelineRenderingCreateInfo::new().set_color_attachment_formats(&formats)?;
		let pipeline_info = vk::GraphicsPipelineCreateInfo::new()
			.set_stages(&stages)?
			.set_vertex_input_state(&vertex_input)
			.set_input_assembly_state(&assembly)
			.set_viewport_state(&viewport)
			.set_rasterization_state(&raster)
			.set_multisample_state(&multisample)
			.set_color_blend_state(&blend)
			.set_layout(layout)
			.push_next(&mut rendering)?;
		let mut pipelines = [vk::Pipeline::NULL];
		if device.create_graphics_pipelines(
			vk::PipelineCache::NULL,
			&[pipeline_info],
			None,
			&mut pipelines,
		)? != vk::Result::SUCCESS
		{
			return Err("graphics pipeline did not return SUCCESS".into());
		}
		let pool = device.create_command_pool(
			&vk::CommandPoolCreateInfo::new().set_queue_family_index(family),
			None,
		)?;
		let commands = device.allocate_command_buffers(
			&vk::CommandBufferAllocateInfo::new()
				.set_command_pool(pool)
				.set_level(vk::CommandBufferLevel::PRIMARY)
				.set_command_buffer_count(1),
		)?;
		let command = commands[0];
		device.begin_command_buffer(command, &vk::CommandBufferBeginInfo::new())?;
		let initial = [vk::ImageMemoryBarrier2::new()
			.set_src_stage_mask(vk::PipelineStageFlags2::NONE)
			.set_dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_BIT)
			.set_dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE_BIT)
			.set_old_layout(vk::ImageLayout::UNDEFINED)
			.set_new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
			.set_src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
			.set_dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
			.set_image(image)
			.set_subresource_range(range)];
		device.cmd_pipeline_barrier2(
			command,
			&vk::DependencyInfo::new().set_image_memory_barriers(&initial)?,
		)?;
		let color = [vk::RenderingAttachmentInfo::new()
			.set_image_view(view)
			.set_image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
			.set_load_op(vk::AttachmentLoadOp::CLEAR)
			.set_store_op(vk::AttachmentStoreOp::STORE)
			.set_clear_value(vk::ClearValue {
				color: vk::ClearColorValue {
					float32: [0.0, 0.0, 0.0, 1.0],
				},
			})];
		device.cmd_begin_rendering(
			command,
			&vk::RenderingInfo::new()
				.set_render_area(rect)
				.set_layer_count(1)
				.set_color_attachments(&color)?,
		)?;
		device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, pipelines[0])?;
		device.cmd_draw(command, 3, 1, 0, 0)?;
		device.cmd_end_rendering(command)?;
		let transfer = [vk::ImageMemoryBarrier2::new()
			.set_src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_BIT)
			.set_src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE_BIT)
			.set_dst_stage_mask(vk::PipelineStageFlags2::COPY_BIT)
			.set_dst_access_mask(vk::AccessFlags2::TRANSFER_READ_BIT)
			.set_old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
			.set_new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
			.set_src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
			.set_dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
			.set_image(image)
			.set_subresource_range(range)];
		device.cmd_pipeline_barrier2(
			command,
			&vk::DependencyInfo::new().set_image_memory_barriers(&transfer)?,
		)?;
		let regions = [vk::BufferImageCopy2::new()
			.set_image_subresource(
				vk::ImageSubresourceLayers::new()
					.set_aspect_mask(vk::ImageAspectFlags::COLOR_BIT)
					.set_layer_count(1),
			)
			.set_image_extent(vk::Extent3D {
				width: EDGE,
				height: EDGE,
				depth: 1,
			})];
		device.cmd_copy_image_to_buffer2(
			command,
			&vk::CopyImageToBufferInfo2::new()
				.set_src_image(image)
				.set_src_image_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
				.set_dst_buffer(buffer)
				.set_regions(&regions)?,
		)?;
		let host = [vk::MemoryBarrier2::new()
			.set_src_stage_mask(vk::PipelineStageFlags2::COPY_BIT)
			.set_src_access_mask(vk::AccessFlags2::TRANSFER_WRITE_BIT)
			.set_dst_stage_mask(vk::PipelineStageFlags2::HOST_BIT)
			.set_dst_access_mask(vk::AccessFlags2::HOST_READ_BIT)];
		device.cmd_pipeline_barrier2(
			command,
			&vk::DependencyInfo::new().set_memory_barriers(&host)?,
		)?;
		device.end_command_buffer(command)?;
		let fence = device.create_fence(&vk::FenceCreateInfo::new(), None)?;
		let command_infos = [vk::CommandBufferSubmitInfo::new().set_command_buffer(command)];
		device.queue_submit2(
			queue,
			&[vk::SubmitInfo2::new().set_command_buffer_infos(&command_infos)?],
			fence,
		)?;
		if !device.wait_for_fences(&[fence], true, 10_000_000_000)? {
			return Err("triangle fence timeout".into());
		}
		let pointer = allocator.map(&buffer_memory)?.cast::<u8>();
		allocator.invalidate(&buffer_memory, 0, vk::WHOLE_SIZE)?;
		let bytes = std::slice::from_raw_parts(pointer, BYTES as usize);
		let center = ((EDGE / 2 * EDGE + EDGE / 2) * 4) as usize;
		if bytes[center..center + 4] != [255, 0, 0, 255] || bytes[..4] != [0, 0, 0, 255] {
			return Err("triangle center/corner oracle failed".into());
		}
		let mut red = 0;
		let mut ppm = format!("P6\n{EDGE} {EDGE}\n255\n").into_bytes();
		for pixel in bytes.as_chunks::<4>().0 {
			if *pixel == [255, 0, 0, 255] {
				red += 1;
			} else if *pixel != [0, 0, 0, 255] {
				return Err("unexpected triangle pixel".into());
			}
			ppm.extend_from_slice(&pixel[..3]);
		}
		if !(500..2000).contains(&red) {
			return Err("triangle area oracle failed".into());
		}
		fs::write(&args[2], ppm)?;
		allocator.unmap(&buffer_memory)?;
		device.destroy_fence(fence, None)?;
		device.destroy_command_pool(pool, None)?;
		device.destroy_pipeline(pipelines[0], None)?;
		device.destroy_pipeline_layout(layout, None)?;
		device.destroy_shader_module(vert, None)?;
		device.destroy_shader_module(frag, None)?;
		device.destroy_image_view(view, None)?;
		allocator.destroy_image(image, &mut image_memory)?;
		allocator.destroy_buffer(buffer, &mut buffer_memory)?;
		allocator.close()?;
		if allocator.statistics().native_allocation_count != 0 {
			return Err("triangle native allocation leak".into());
		}
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
		println!(
			"{}",
			serde_json::json!({"schema": "vk.sdk.triangle.v1", "red_pixels": red,
				"width": EDGE, "height": EDGE, "identity": identity,
				"center_and_corner": "passed", "cleanup": "passed"})
		);
	}
	Ok(())
}

fn main() {
	if let Err(error) = run() {
		eprintln!("vk triangle failed: {error}");
		std::process::exit(1);
	}
}
