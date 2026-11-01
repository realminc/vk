fn main() {
	let mut priorities = [1.0];
	let queues = [vk::DeviceQueueCreateInfo::default()
		.set_queue_priorities(&priorities)
		.unwrap()];
	let info = vk::DeviceCreateInfo::default()
		.set_queue_create_infos(&queues)
		.unwrap();
	priorities[0] = 0.5;
	std::hint::black_box(info);
}
