fn main() {
	let _ = vk::AllocationCallbacks::default().set_user_data(std::ptr::null_mut());
}
