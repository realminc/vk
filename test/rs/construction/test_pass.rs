use vk::ConstructionError;

struct Setup<'a> {
	info: vk::InstanceCreateInfo<'a>,
}

fn reborrow<'short, 'long: 'short>(
	application: &'short vk::ApplicationInfo<'long>,
) -> vk::InstanceCreateInfo<'short> {
	vk::InstanceCreateInfo::default().application_info(application)
}

fn main() -> Result<(), ConstructionError> {
	let name = std::ffi::CString::new("live").unwrap();
	let application = vk::ApplicationInfo::default().application_name(&name);
	let setup = Setup {
		info: reborrow(&application),
	};
	assert_eq!(
		setup.info.p_application_info,
		std::ptr::from_ref(&application)
	);
	let mut priorities = [1.0];
	let queues = [vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities)?];
	let info = vk::DeviceCreateInfo::default().queue_create_infos(&queues)?;
	std::hint::black_box(&info);
	priorities[0] = 0.5;
	let mut extension = vk::PhysicalDeviceVulkan12Features::default();
	{
		let root = vk::PhysicalDeviceFeatures2::default().push_next(&mut extension)?;
		std::hint::black_box(&root);
	}
	extension.timeline_semaphore = vk::TRUE;
	let raw = vk::BufferCreateInfo {
		size: 7,
		..Default::default()
	};
	assert_eq!(raw.size, 7);
	let words = [0_u32; 5];
	let shader = vk::ShaderModuleCreateInfo::default().code(&words);
	std::hint::black_box(&shader);
	let swaps = [vk::SwapchainKHR::NULL];
	let indices = [0_u32];
	let mut results = [vk::Result::NOT_READY];
	let present = vk::PresentInfoKHR::default().swapchains(&swaps, &indices, Some(&mut results))?;
	std::hint::black_box(&present);
	results[0] = vk::Result::SUCCESS;
	let mut bytes = [0_u8; 8];
	let output = vk::PipelineBinaryDataKHR::default().data(&mut bytes)?;
	std::hint::black_box(&output);
	bytes[0] = 1;
	Ok(())
}
