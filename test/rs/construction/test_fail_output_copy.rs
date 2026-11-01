fn main() {
	let mut bytes = [0_u8; 8];
	let output = vk::PipelineBinaryDataKHR::default().set_data(&mut bytes).unwrap();
	let moved = output;
	std::hint::black_box(output);
	std::hint::black_box(moved);
}
