fn main() {
	let info = {
		let application = vk::ApplicationInfo::default();
		vk::InstanceCreateInfo::default().set_application_info(&application)
	};
	std::hint::black_box(info);
}
