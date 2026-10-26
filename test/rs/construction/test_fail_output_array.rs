fn main() {
	let swaps = [vk::SwapchainKHR::NULL];
	let indices = [0_u32];
	let mut results = [vk::Result::NOT_READY];
	let info = vk::PresentInfoKHR::default().swapchains(&swaps, &indices, Some(&mut results)).unwrap();
	let _ = std::hint::black_box(&results);
	std::hint::black_box(info);
}
