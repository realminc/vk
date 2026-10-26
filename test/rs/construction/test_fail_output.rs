fn main() {
	let mut extension = vk::PhysicalDeviceVulkan12Features::default();
	let root = vk::PhysicalDeviceFeatures2::default()
		.push_next(&mut extension)
		.unwrap();
	std::hint::black_box(&extension.timeline_semaphore);
	std::hint::black_box(root);
}
