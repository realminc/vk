//! Vulkano's checked managed API; no `_unchecked` or raw-PFN substitutions.
//! Pool geometry matches the native consumers; ownership, FreeList placement,
//! validation and eager host-visible mapping remain Vulkano's own policies.
mod common;
use common::{Args, Result, Workload};
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::Arc, time::Instant};
use vulkano::{
	Version, VulkanLibrary, VulkanObject,
	buffer::{Buffer, BufferCreateInfo, BufferMemory, BufferUsage},
	device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags},
	instance::{Instance, InstanceCreateInfo},
	memory::{
		MemoryPropertyFlags,
		allocator::{
			AllocationCreateInfo, DeviceLayout, GenericMemoryAllocatorCreateInfo,
			MemoryAllocatePreference, MemoryTypeFilter, StandardMemoryAllocator,
		},
	},
};

struct Buffers {
	allocator: Arc<StandardMemoryAllocator>,
	device: Arc<Device>,
	memory_type: u32,
	pooled: bool,
}

impl Workload for Buffers {
	type Token = Arc<Buffer>;

	fn allocate(&mut self, size: u64) -> Result<Self::Token> {
		let buffer = Buffer::new(
			self.allocator.clone(),
			BufferCreateInfo {
				usage: BufferUsage::TRANSFER_SRC | BufferUsage::TRANSFER_DST,
				..Default::default()
			},
			AllocationCreateInfo {
				memory_type_bits: 1 << self.memory_type,
				memory_type_filter: MemoryTypeFilter {
					required_flags: MemoryPropertyFlags::HOST_VISIBLE,
					..Default::default()
				},
				allocate_preference: if self.pooled {
					MemoryAllocatePreference::NeverAllocate
				} else {
					MemoryAllocatePreference::AlwaysAllocate
				},
				..Default::default()
			},
			DeviceLayout::from_size_alignment(size, 1).ok_or("invalid buffer layout")?,
		)?;
		Ok(buffer)
	}

	fn free(&mut self, token: Self::Token) -> Result<()> {
		drop(token);
		Ok(())
	}

	fn inspect(&self, tokens: &[Self::Token]) -> Result<Value> {
		let mut native = BTreeMap::new();
		let mut ranges = Vec::with_capacity(tokens.len());
		let mut allocation_bytes = 0;
		for buffer in tokens {
			if Arc::strong_count(buffer) != 1 {
				return Err("buffer owner escaped benchmark".into());
			}
			let BufferMemory::Normal(region) = buffer.memory() else {
				return Err("unexpected buffer memory owner".into());
			};
			let memory = region.device_memory();
			if memory.memory_type_index() != self.memory_type {
				return Err("memory type changed".into());
			}
			let handle = memory.handle();
			let end = region
				.offset()
				.checked_add(region.size())
				.ok_or("region overflow")?;
			if region.size() < buffer.size() || end > memory.allocation_size() {
				return Err("invalid bound extent".into());
			}
			native.insert(handle, memory.allocation_size());
			ranges.push((handle, region.offset(), end));
			allocation_bytes += region.size();
		}
		ranges.sort_unstable();
		if ranges
			.windows(2)
			.any(|pair| pair[0].0 == pair[1].0 && pair[0].2 > pair[1].1)
		{
			return Err("overlapping native regions".into());
		}
		let pool_live: usize = self
			.allocator
			.pools()
			.iter()
			.map(|pool| {
				pool
					.blocks()
					.map(|block| block.allocation_count())
					.sum::<usize>()
			})
			.sum();
		if (self.pooled && pool_live != tokens.len())
			|| self.device.allocation_count() as usize != native.len()
		{
			return Err("native/live accounting mismatch".into());
		}
		Ok(
			json!({"live_count": tokens.len(), "allocation_bytes": allocation_bytes,
			"native_count": native.len(), "native_bytes": native.values().sum::<u64>()}),
		)
	}

	fn empty(&self) -> Result<()> {
		let expected = u32::from(self.pooled);
		if self.device.allocation_count() != expected {
			return Err("native block count changed".into());
		}
		for pool in self.allocator.pools() {
			if pool.blocks().any(|block| block.allocation_count() != 0) {
				return Err("live suballocation retained".into());
			}
		}
		Ok(())
	}
}

fn run() -> Result<()> {
	let args = Args::new()?;
	if !["probe", "startup", "buffer-pool", "buffer-separate"].contains(&args.case.as_str()) {
		return Err("Vulkano comparison supports startup and managed buffers; its cached getters are not fresh native queries".into());
	}
	let requested = Version::from(args.api);
	let start = Instant::now();
	let library = VulkanLibrary::new()?;
	let load_ns = start.elapsed().as_nanos();
	if library.api_version() < requested {
		return Err("loader API below request".into());
	}
	let create = InstanceCreateInfo {
		max_api_version: Some(requested),
		..Default::default()
	};
	let start = Instant::now();
	let instance = Instance::new(library, create)?;
	let instance_ns = start.elapsed().as_nanos();
	let devices: Vec<_> = instance.enumerate_physical_devices()?.collect();
	let physical = devices
		.get(args.device)
		.ok_or("device index absent")?
		.clone();
	if physical.api_version() < requested {
		return Err("physical API below request".into());
	}
	let properties = physical.properties();
	let family = physical
		.queue_family_properties()
		.iter()
		.position(|q| q.queue_count > 0 && q.queue_flags.intersects(QueueFlags::COMPUTE))
		.ok_or("compute queue absent")? as u32;
	let api_version =
		u32::try_from(properties.api_version).map_err(|_| "invalid native API version")?;
	let identity = json!({"device_uuid": properties.device_uuid.ok_or("device UUID unavailable")?,
		"driver_uuid": properties.driver_uuid.ok_or("driver UUID unavailable")?,
		"device_count": devices.len(), "vendor_id": properties.vendor_id, "device_id": properties.device_id,
		"device_type": properties.device_type as i32, "api_version": api_version,
		"driver_version": properties.driver_version, "device_name": properties.device_name,
		"queue_family": family});
	if args.case == "probe" {
		common::emit(
			"vulkano",
			&args,
			identity,
			json!({"api_policy": "checked, Arc-owned"}),
		);
		return Ok(());
	}
	let create = DeviceCreateInfo {
		queue_create_infos: vec![QueueCreateInfo {
			queue_family_index: family,
			queues: vec![1.0],
			..Default::default()
		}],
		..Default::default()
	};
	let start = Instant::now();
	let (device, queues) = Device::new(physical.clone(), create)?;
	let device_ns = start.elapsed().as_nanos();
	let queues: Vec<_> = queues.collect();
	let measurements = if args.case == "startup" {
		json!({"load_ns": load_ns, "instance_ns": instance_ns, "device_ns": device_ns})
	} else {
		let memory = physical.memory_properties();
		if args.memory_type as usize >= memory.memory_types.len()
			|| !memory.memory_types[args.memory_type as usize]
				.property_flags
				.intersects(MemoryPropertyFlags::HOST_VISIBLE)
		{
			return Err("explicit host-visible memory type required".into());
		}
		let block_sizes = vec![common::BLOCK_SIZE; memory.memory_types.len()];
		let allocator = Arc::new(StandardMemoryAllocator::new(
			device.clone(),
			GenericMemoryAllocatorCreateInfo {
				block_sizes: &block_sizes,
				memory_type_bits: 1 << args.memory_type,
				..Default::default()
			},
		));
		let mut workload = Buffers {
			allocator,
			device: device.clone(),
			memory_type: args.memory_type,
			pooled: args.case == "buffer-pool",
		};
		if workload.pooled {
			// Acquire the one backing block outside timing; measured allocations are NeverAllocate.
			let warm = Buffer::new(
				workload.allocator.clone(),
				BufferCreateInfo {
					usage: BufferUsage::TRANSFER_SRC | BufferUsage::TRANSFER_DST,
					..Default::default()
				},
				AllocationCreateInfo {
					memory_type_bits: 1 << args.memory_type,
					memory_type_filter: MemoryTypeFilter {
						required_flags: MemoryPropertyFlags::HOST_VISIBLE,
						..Default::default()
					},
					..Default::default()
				},
				DeviceLayout::from_size_alignment(64, 1).ok_or("invalid warmup layout")?,
			)?;
			let BufferMemory::Normal(region) = warm.memory() else {
				return Err("unexpected warmup memory".into());
			};
			if region.device_memory().allocation_size() != common::BLOCK_SIZE {
				return Err("pool block size fallback".into());
			}
			drop(warm);
			workload.empty()?;
		}
		let mut result = common::batches(&args, &mut workload)?;
		result["allocator_policy"] = json!({"placement": "Vulkano StandardMemoryAllocator/FreeList",
			"ownership": "checked Buffer API, Arc and Drop", "mapping": "host-visible backing automatically mapped",
			"pool_warmup": "one untimed buffer when pooled", "no_implicit_wait": "no GPU work submitted"});
		drop(workload);
		if device.allocation_count() != 0 {
			return Err("native backing leak after allocator drop".into());
		}
		result
	};
	drop(queues);
	drop(device);
	common::emit("vulkano", &args, identity, measurements);
	Ok(())
}

fn main() {
	if let Err(error) = run() {
		eprintln!("Vulkano SDK failed: {error}");
		std::process::exit(1);
	}
}
