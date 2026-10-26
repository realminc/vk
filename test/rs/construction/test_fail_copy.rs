fn main() {
	let mut extension = vk::PhysicalDeviceVulkan12Features::default();
	let root = vk::PhysicalDeviceFeatures2::default()
		.push_next(&mut extension)
		.unwrap();
	let copied = root;
	std::hint::black_box(root);
	std::hint::black_box(copied);
}
