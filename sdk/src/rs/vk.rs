//! Native vkrust public-API benchmark consumer. Cleanup is explicit.
mod common;
use common::{Args, Result, Workload};
use serde_json::{Value, json};
use std::{ffi::CStr, hint::black_box, time::Instant};

struct Buffers<'a, 'device> {
	allocator: &'a mut vk::Allocator<'device>,
	device: &'device vk::Device,
	options: vk::mem::AllocationCreateInfo<'a>,
	image: bool,
}

impl Workload for Buffers<'_, '_> {
	type Token = (vk::Buffer, vk::Image, vk::Allocation);

	fn allocate(&mut self, size: u64) -> Result<Self::Token> {
		if self.image {
			let info = vk::ImageCreateInfo::new()
				.set_image_type(vk::ImageType::_2D)
				.set_format(vk::Format::R8_UNORM)
				.set_extent(
					vk::Extent3D::new()
						.set_width(64)
						.set_height((size / 64) as u32)
						.set_depth(1),
				)
				.set_mip_levels(1)
				.set_array_layers(1)
				.set_samples(vk::SampleCountFlagBits::_1_BIT)
				.set_tiling(vk::ImageTiling::OPTIMAL)
				.set_usage(vk::ImageUsageFlags::TRANSFER_SRC_BIT | vk::ImageUsageFlags::TRANSFER_DST_BIT)
				.set_sharing_mode(vk::SharingMode::EXCLUSIVE);
			// SAFETY: checked fixed dimensions and no submitted image accesses.
			let (image, allocation) = unsafe {
				self
					.allocator
					.create_image_with_options(&info, &self.options)
			}?;
			return Ok((vk::Buffer::NULL, image, allocation));
		}
		let info = vk::BufferCreateInfo::new()
			.set_size(size)
			.set_usage(vk::BufferUsageFlags::TRANSFER_SRC_BIT | vk::BufferUsageFlags::TRANSFER_DST_BIT)
			.set_sharing_mode(vk::SharingMode::EXCLUSIVE);
		// SAFETY: this process owns the device and never submits these buffers.
		let (buffer, allocation) = unsafe {
			self
				.allocator
				.create_buffer_with_options(&info, &self.options)
		}?;
		Ok((buffer, vk::Image::NULL, allocation))
	}

	fn free(&mut self, (buffer, image, mut allocation): Self::Token) -> Result<()> {
		// SAFETY: no submitted GPU work or outstanding maps refer to this buffer.
		unsafe {
			if self.image {
				self.allocator.destroy_image(image, &mut allocation)?;
			} else {
				self.allocator.destroy_buffer(buffer, &mut allocation)?;
			}
		}
		Ok(())
	}

	fn inspect(&self, tokens: &[Self::Token]) -> Result<Value> {
		let stats = self.allocator.statistics();
		if stats.allocation_count != tokens.len() {
			return Err("allocator live count mismatch".into());
		}
		for (buffer, image, allocation) in tokens {
			let info = self.allocator.allocation_info(allocation)?;
			// SAFETY: live unpublished resources belong to this process and device.
			let requirements = unsafe {
				if self.image {
					let mut requirements = vk::MemoryRequirements::new();
					self
						.device
						.get_image_memory_requirements(*image, &mut requirements)?;
					requirements
				} else {
					self.device.get_buffer_memory_requirements(*buffer)?
				}
			};
			if info.size < requirements.size
				|| !info.offset.is_multiple_of(requirements.alignment)
				|| requirements.memory_type_bits & (1 << info.memory_type_index) == 0
				|| info
					.offset
					.checked_add(info.reserved_size)
					.is_none_or(|end| end > info.native_size)
			{
				return Err("native requirements/range mismatch".into());
			}
		}
		let mut ranges: Vec<_> = tokens
			.iter()
			.map(|(_, _, allocation)| {
				(
					allocation.handle().as_raw(),
					allocation.offset(),
					allocation.reserved_size(),
				)
			})
			.collect();
		ranges.sort_unstable();
		if ranges.iter().any(|(_, _, size)| *size == 0)
			|| ranges
				.windows(2)
				.any(|pair| pair[0].0 == pair[1].0 && pair[0].1 + pair[0].2 > pair[1].1)
		{
			return Err("native allocation overlap/extent violation".into());
		}
		Ok(
			json!({"live_count": stats.allocation_count, "native_count": stats.native_allocation_count,
			"native_bytes": stats.native_bytes, "allocation_bytes": stats.allocation_bytes}),
		)
	}

	fn empty(&self) -> Result<()> {
		if self.allocator.statistics().allocation_count != 0 {
			return Err("allocator leaked a region".into());
		}
		Ok(())
	}

	fn map_write(&mut self, (_, _, allocation): &Self::Token, pattern: u64) -> Result<()> {
		// SAFETY: all benchmark sizes exceed eight bytes; no GPU work or aliased references.
		unsafe {
			let pointer = self.allocator.map(allocation)?;
			pointer.cast::<u64>().write_unaligned(black_box(pattern));
			self.allocator.unmap(allocation)?;
		}
		Ok(())
	}

	fn map_check(&mut self, (_, _, allocation): &Self::Token, pattern: u64) -> Result<()> {
		// SAFETY: host-only payload remains within the live allocation; no GPU access exists.
		unsafe {
			let pointer = self.allocator.map(allocation)?;
			let observed = pointer.cast::<u64>().read_unaligned();
			self.allocator.unmap(allocation)?;
			if observed != pattern {
				return Err("mapped payload mismatch".into());
			}
		}
		Ok(())
	}
}

struct Virtual<'a>(vk::VirtualBlock<'a>, vk::mem::PlacementStrategy);

impl Workload for Virtual<'_> {
	type Token = vk::VirtualAllocation;

	fn allocate(&mut self, size: u64) -> Result<Self::Token> {
		Ok(self.0.allocate(size, 64, self.1)?)
	}

	fn free(&mut self, mut token: Self::Token) -> Result<()> {
		Ok(self.0.free(&mut token)?)
	}

	fn inspect(&self, tokens: &[Self::Token]) -> Result<Value> {
		let mut ranges = Vec::with_capacity(tokens.len());
		for token in tokens {
			let info = self.0.allocation_info(token)?;
			ranges.push((info.offset, info.size));
		}
		ranges.sort_unstable();
		if ranges
			.iter()
			.any(|(offset, size)| offset % 64 != 0 || *size == 0 || offset + size > common::BLOCK_SIZE)
			|| ranges
				.windows(2)
				.any(|pair| pair[0].0 + pair[0].1 > pair[1].0)
		{
			return Err("virtual alignment/extent/overlap violation".into());
		}
		let stats = self.0.statistics();
		if stats.allocation_count != tokens.len() {
			return Err("virtual count mismatch".into());
		}
		Ok(
			json!({"live_count": stats.allocation_count, "allocation_bytes": stats.allocated_bytes,
			"free_bytes": stats.free_bytes, "largest_free_region": stats.largest_free_region}),
		)
	}

	fn empty(&self) -> Result<()> {
		if self.0.statistics().allocation_count != 0 {
			return Err("virtual leak".into());
		}
		Ok(())
	}
}

fn run() -> Result<()> {
	let args = Args::new()?;
	let algorithm = if args.algorithm == "linear" {
		vk::mem::AllocationAlgorithm::Linear
	} else {
		vk::mem::AllocationAlgorithm::Tlsf
	};
	let strategy = match args.strategy.as_str() {
		"min-memory" => vk::mem::PlacementStrategy::MinMemory,
		"min-time" => vk::mem::PlacementStrategy::MinTime,
		"min-offset" => vk::mem::PlacementStrategy::MinOffset,
		_ => vk::mem::PlacementStrategy::Default,
	};
	if args.case == "virtual" {
		let measurements = common::batches(
			&args,
			&mut Virtual(
				vk::VirtualBlock::new_with_algorithm(common::BLOCK_SIZE, algorithm)?,
				strategy,
			),
		)?;
		common::emit("vkrust", &args, Value::Null, measurements);
		return Ok(());
	}
	// SAFETY: SDK owns the whole process and every native object, uses queried metadata,
	// and destroys children before parents. Errors terminate the process (not a reusable owner).
	unsafe {
		let start = Instant::now();
		let entry = vk::Entry::new()?;
		let load_ns = start.elapsed().as_nanos();
		if entry
			.enumerate_instance_version()
			.map_err(vk::CallError::from)?
			< args.api
		{
			return Err("loader API below request".into());
		}
		let app = vk::ApplicationInfo::new().set_api_version(args.api);
		let info = vk::InstanceCreateInfo::new().set_application_info(&app);
		let start = Instant::now();
		let instance = vk::Instance::new(&entry, &info, None)?;
		let instance_ns = start.elapsed().as_nanos();
		let devices = instance.enumerate_physical_devices()?;
		let physical = *devices.get(args.device).ok_or("device index absent")?;
		let properties = instance.get_physical_device_properties(physical)?;
		if properties.api_version < args.api {
			return Err("physical device API below request".into());
		}
		let mut id = vk::PhysicalDeviceIDProperties::new();
		{
			let mut properties2 = vk::PhysicalDeviceProperties2::new().push_next(&mut id)?;
			instance.get_physical_device_properties2(physical, &mut properties2)?;
		}
		let memory = instance.get_physical_device_memory_properties(physical)?;
		let families = instance.get_physical_device_queue_family_properties(physical)?;
		let family = families
			.iter()
			.position(|q| {
				q.queue_count > 0 && q.queue_flags.as_raw() & vk::QueueFlagBits::COMPUTE_BIT.as_raw() != 0
			})
			.ok_or("compute queue absent")? as u32;
		let extensions = instance.enumerate_device_extension_properties(physical, None)?;
		let layers = entry.enumerate_instance_layer_properties()?;
		let mut f12 = vk::PhysicalDeviceVulkan12Features::new();
		let mut f13 = vk::PhysicalDeviceVulkan13Features::new();
		let mut f14 = vk::PhysicalDeviceVulkan14Features::new();
		{
			let mut query = vk::PhysicalDeviceFeatures2::new();
			if args.api >= vk::API_VERSION_1_2 {
				query = query.push_next(&mut f12)?;
			}
			if args.api >= vk::API_VERSION_1_3 {
				query = query.push_next(&mut f13)?;
			}
			if args.api >= vk::API_VERSION_1_4 {
				query = query.push_next(&mut f14)?;
			}
			instance.get_physical_device_features2(physical, &mut query)?;
		}
		let identity = json!({"device_count": devices.len(), "device_uuid": id.device_uuid,
			"driver_uuid": id.driver_uuid, "vendor_id": properties.vendor_id,
			"device_id": properties.device_id, "device_type": properties.device_type.as_raw(),
			"device_name": CStr::from_ptr(properties.device_name.as_ptr()).to_string_lossy(),
			"api_version": properties.api_version, "driver_version": properties.driver_version,
			"queue_family": family, "non_coherent_atom_size": properties.limits.non_coherent_atom_size,
			"buffer_image_granularity": properties.limits.buffer_image_granularity,
			"features": {"timeline_semaphore": f12.timeline_semaphore != 0,
				"buffer_device_address": f12.buffer_device_address != 0,
				"dynamic_rendering": f13.dynamic_rendering != 0, "synchronization2": f13.synchronization2 != 0,
				"maintenance5": f14.maintenance5 != 0, "maintenance6": f14.maintenance6 != 0,
				"host_image_copy": f14.host_image_copy != 0},
			"memory_types": memory.memory_types[..memory.memory_type_count as usize].iter().map(|m| json!({"flags": m.property_flags.as_raw(), "heap": m.heap_index})).collect::<Vec<_>>(),
			"extensions": extensions.iter().map(|e| CStr::from_ptr(e.extension_name.as_ptr()).to_string_lossy().into_owned()).collect::<Vec<_>>(),
			"layers": layers.iter().map(|l| CStr::from_ptr(l.layer_name.as_ptr()).to_string_lossy().into_owned()).collect::<Vec<_>>()});
		if args.case == "probe" {
			common::emit("vkrust", &args, identity, json!({}));
			instance.destroy(None);
			return Ok(());
		}
		let priorities = [1.0];
		let queues = [vk::DeviceQueueCreateInfo::new()
			.set_queue_family_index(family)
			.set_queue_priorities(&priorities)?];
		let create = vk::DeviceCreateInfo::new().set_queue_create_infos(&queues)?;
		let start = Instant::now();
		let device = vk::Device::new(&instance, physical, &create, None)?;
		let device_ns = start.elapsed().as_nanos();
		let measurements = if args.case == "startup" {
			json!({"load_ns": load_ns, "instance_ns": instance_ns, "device_ns": device_ns})
		} else if args.case == "query" {
			let operations = args.batches * args.batch_size;
			let start = Instant::now();
			for _ in 0..operations {
				let _ = black_box(instance.get_physical_device_memory_properties(black_box(physical))?);
			}
			json!({"query_ns": start.elapsed().as_nanos(), "queries": operations})
		} else {
			if args.memory_type >= memory.memory_type_count
				|| memory.memory_types[args.memory_type as usize]
					.property_flags
					.as_raw()
					& vk::MemoryPropertyFlags::HOST_VISIBLE_BIT.as_raw()
					== 0
			{
				return Err("benchmark requires a host-visible explicit memory type".into());
			}
			let mut allocator = vk::Allocator::new_from_physical_device(&instance, physical, &device)?;
			let mut pool = if args.case.ends_with("pool") {
				Some(
					allocator.create_pool(
						&vk::mem::PoolCreateInfo::new()
							.set_memory_type_index(args.memory_type)
							.set_block_size(common::BLOCK_SIZE)
							.set_algorithm(algorithm)
							.set_min_block_count(1)
							.set_max_block_count(1),
					)?,
				)
			} else {
				None
			};
			let options = vk::mem::AllocationCreateInfo::new()
				.set_memory_type_bits(1 << args.memory_type)
				.set_required_flags(vk::MemoryPropertyFlags::HOST_VISIBLE_BIT)
				.set_strategy(strategy)
				.set_policy(if pool.is_some() {
					vk::mem::AllocationPolicy::Automatic
				} else {
					vk::mem::AllocationPolicy::Separate
				})
				.set_pool(pool.as_ref());
			let measurements = common::batches(
				&args,
				&mut Buffers {
					allocator: &mut allocator,
					device: &device,
					options,
					image: args.case.starts_with("image-"),
				},
			)?;
			if let Some(pool) = &mut pool {
				allocator.destroy_pool(pool)?;
			}
			allocator.close()?;
			if allocator.statistics().native_allocation_count != 0 {
				return Err("native allocation leak".into());
			}
			drop(allocator);
			measurements
		};
		device.destroy(None);
		instance.destroy(None);
		common::emit("vkrust", &args, identity, measurements);
	}
	Ok(())
}

fn main() {
	if let Err(error) = run() {
		eprintln!("vk SDK failed: {error}");
		std::process::exit(1);
	}
}
