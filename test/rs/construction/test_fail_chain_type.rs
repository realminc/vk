fn main() {
	let mut extension = vk::PhysicalDeviceVulkan12Features::default();
	let info = vk::InstanceCreateInfo::default()
		.push_next(&mut extension)
		.unwrap();
	std::hint::black_box(info);
}
