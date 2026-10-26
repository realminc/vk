fn main() {
	let root = {
		let mut extension = vk::PhysicalDeviceVulkan12Features::default();
		vk::PhysicalDeviceFeatures2::default()
			.push_next(&mut extension)
			.unwrap()
	};
	std::hint::black_box(root);
}
