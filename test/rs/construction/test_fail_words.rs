fn main() {
	let info;
	{
		let words = [0_u32; 5];
		info = vk::ShaderModuleCreateInfo::default().code(&words);
	}
	std::hint::black_box(info);
}
