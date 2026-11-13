//! Ash public-API reference. Deliberately uses the same native requests as vk.rs.
#[allow(dead_code)]
mod common;
use ash::{Entry, vk};
use common::{Args, Result};
use serde_json::json;
use std::{ffi::CStr, hint::black_box, time::Instant};

fn run() -> Result<()> {
	let args = Args::new()?;
	if !["probe", "startup", "query"].contains(&args.case.as_str()) {
		return Err("Ash baseline covers binding startup/query; use VMA for allocator cases".into());
	}
	// SAFETY: this process owns all contexts, queries exact output records and
	// destroys the device before its instance. No work is submitted.
	unsafe {
		let start = Instant::now();
		let entry = Entry::load()?;
		let load_ns = start.elapsed().as_nanos();
		if entry
			.try_enumerate_instance_version()?
			.unwrap_or(vk::API_VERSION_1_0)
			< args.api
		{
			return Err("loader API below request".into());
		}
		let app = vk::ApplicationInfo::default().api_version(args.api);
		let info = vk::InstanceCreateInfo::default().application_info(&app);
		let start = Instant::now();
		let instance = entry.create_instance(&info, None)?;
		let instance_ns = start.elapsed().as_nanos();
		let devices = instance.enumerate_physical_devices()?;
		let physical = *devices.get(args.device).ok_or("device index absent")?;
		let properties = instance.get_physical_device_properties(physical);
		if properties.api_version < args.api {
			return Err("physical API below request".into());
		}
		let mut id = vk::PhysicalDeviceIDProperties::default();
		let mut properties2 = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
		instance.get_physical_device_properties2(physical, &mut properties2);
		let family = instance
			.get_physical_device_queue_family_properties(physical)
			.iter()
			.position(|q| q.queue_count > 0 && q.queue_flags.contains(vk::QueueFlags::COMPUTE))
			.ok_or("compute queue absent")? as u32;
		let identity = json!({"device_uuid": id.device_uuid, "driver_uuid": id.driver_uuid,
			"vendor_id": properties.vendor_id, "device_id": properties.device_id,
			"device_type": properties.device_type.as_raw(), "api_version": properties.api_version,
			"driver_version": properties.driver_version,
			"device_name": CStr::from_ptr(properties.device_name.as_ptr()).to_string_lossy(),
			"device_count": devices.len(), "queue_family": family});
		if args.case == "probe" {
			instance.destroy_instance(None);
			common::emit("ash", &args, identity, json!({}));
			return Ok(());
		}
		let priorities = [1.0];
		let queues = [vk::DeviceQueueCreateInfo::default()
			.queue_family_index(family)
			.queue_priorities(&priorities)];
		let create = vk::DeviceCreateInfo::default().queue_create_infos(&queues);
		let start = Instant::now();
		let device = instance.create_device(physical, &create, None)?;
		let device_ns = start.elapsed().as_nanos();
		let measurements = if args.case == "startup" {
			json!({"load_ns": load_ns, "instance_ns": instance_ns, "device_ns": device_ns})
		} else {
			let operations = args.batches * args.batch_size;
			let start = Instant::now();
			for _ in 0..operations {
				let _ = black_box(instance.get_physical_device_memory_properties(black_box(physical)));
			}
			json!({"query_ns": start.elapsed().as_nanos(), "queries": operations})
		};
		device.destroy_device(None);
		instance.destroy_instance(None);
		common::emit("ash", &args, identity, measurements);
	}
	Ok(())
}

fn main() {
	if let Err(error) = run() {
		eprintln!("Ash SDK failed: {error}");
		std::process::exit(1);
	}
}
