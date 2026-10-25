//! Core ABI and public-consumer checks, independent of driver availability.
use std::{
	ffi::c_void,
	mem::{align_of, offset_of, size_of},
};

#[test]
fn scalar_handles_and_unknown_enum_values() {
	assert_eq!(size_of::<vk::InstanceHandle>(), size_of::<*mut c_void>());
	assert_eq!(align_of::<vk::InstanceHandle>(), align_of::<*mut c_void>());
	assert_eq!(size_of::<vk::Result>(), 4);
	assert_eq!(size_of::<vk::Flags64>(), 8);
	assert_eq!(vk::Result::SUCCESS.as_raw(), 0);
	assert_eq!(vk::Result::ERROR_OUT_OF_HOST_MEMORY.as_raw(), -1);
	assert!(vk::Result::from_raw(7).is_success());
	assert!(vk::Result::from_raw(-12345).is_err());
	assert_eq!(vk::Format::from_raw(i32::MAX).as_raw(), i32::MAX);
	assert_eq!(vk::QUEUE_FAMILY_IGNORED, u32::MAX);
	assert_eq!(vk::WHOLE_SIZE, u64::MAX);
	let _: u32 = vk::REMAINING_MIP_LEVELS;
	assert_eq!(
		vk::AccessFlags3KHR::from_raw(1u64 << 40).as_raw(),
		1u64 << 40
	);
}

#[test]
fn real_m1_structs_and_defaults() {
	let app = vk::ApplicationInfo::default();
	let info = vk::InstanceCreateInfo::default();
	assert_eq!(app.s_type, vk::StructureType::APPLICATION_INFO);
	assert_eq!(info.s_type, vk::StructureType::INSTANCE_CREATE_INFO);
	assert!(info.p_next.is_null());
	assert!(info.p_application_info.is_null());
	assert_eq!(offset_of!(vk::InstanceCreateInfo, s_type), 0);
	if cfg!(target_pointer_width = "64") {
		assert_eq!(size_of::<vk::ApplicationInfo>(), 48);
		assert_eq!(size_of::<vk::InstanceCreateInfo>(), 64);
		assert_eq!(size_of::<vk::AllocationCallbacks>(), 48);
		assert_eq!(offset_of!(vk::InstanceCreateInfo, p_application_info), 24);
	}
}

#[test]
fn prototypes_have_correct_parameters() {
	unsafe extern "system" fn blend(_: vk::CommandBuffer, _: *const f32) {}
	unsafe extern "system" fn alloc(
		_: *mut c_void,
		_: usize,
		_: usize,
		_: vk::SystemAllocationScope,
	) -> *mut c_void {
		std::ptr::null_mut()
	}
	unsafe extern "system" fn create(
		_: *const vk::InstanceCreateInfo,
		_: *const vk::AllocationCallbacks,
		_: *mut vk::InstanceHandle,
	) -> vk::Result {
		vk::Result::ERROR_INITIALIZATION_FAILED
	}
	let _: vk::PFN_vkCmdSetBlendConstants = Some(blend);
	let _: vk::PFN_vkAllocationFunction = Some(alloc);
	let _: vk::PFN_vkCreateInstance = Some(create);
	let _: vk::PFN_vkCmdDrawIndirectCountKHR = None;
	assert_eq!(vk::ImageLayout::READ_ONLY_OPTIMAL.as_raw(), 1000314000);
	assert_eq!(
		vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES.as_raw(),
		49
	);
}
