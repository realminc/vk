fn main() {
	let info = {
		let name = std::ffi::CString::new("temporary").unwrap();
		vk::ApplicationInfo::default().application_name(&name)
	};
	std::hint::black_box(info);
}
