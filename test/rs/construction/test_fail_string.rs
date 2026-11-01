fn main() {
	let info = {
		let name = std::ffi::CString::new("temporary").unwrap();
		vk::ApplicationInfo::default().set_application_name(&name)
	};
	std::hint::black_box(info);
}
