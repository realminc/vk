use super::*;

#[test]
fn members_preserve_snake_case_and_split_acronyms() {
	for (c, rust) in [
		("sType", "s_type"),
		("pNext", "p_next"),
		("ppEnabledExtensionNames", "pp_enabled_extension_names"),
		("deviceUUID", "device_uuid"),
		("textureCompressionASTC_LDR", "texture_compression_astc_ldr"),
		("pStdPictureInfo", "p_std_picture_info"),
		("error_resilient_mode", "error_resilient_mode"),
		("reserved1", "reserved1"),
		("type", "r#type"),
		("gen", "r#gen"),
	] {
		assert_eq!(member_name(c), rust);
	}
}

#[test]
fn test_type_name() {
	assert_eq!(type_name("VkInstance"), "InstanceHandle");
	assert_eq!(
		type_name("VkPhysicalDeviceVulkan13Features"),
		"PhysicalDeviceVulkan13Features"
	);
	assert_eq!(
		type_name("VkVideoDecodeVP9PictureInfoKHR"),
		"VideoDecodeVP9PictureInfoKHR"
	);
}

#[test]
fn test_command_name() {
	assert_eq!(command_name("vkCreateInstance"), "create_instance");
	assert_eq!(
		command_name("vkGetPhysicalDeviceProperties2"),
		"get_physical_device_properties2"
	);
	assert_eq!(command_name("vkCmdDecodeVideoKHR"), "cmd_decode_video_khr");
	assert_eq!(command_name("vkQueueSubmit2"), "queue_submit2");
	for (native, rust) in [
		(
			"vkCmdSetDispatchParametersARM",
			"cmd_set_dispatch_parameters_arm",
		),
		("vkCmdDrawIndirectCountAMD", "cmd_draw_indirect_count_amd"),
		("vkCmdDispatchGraphAMDX", "cmd_dispatch_graph_amdx"),
		("vkCreateCuModuleNVX", "create_cu_module_nvx"),
		(
			"vkGetPastPresentationTimingGOOGLE",
			"get_past_presentation_timing_google",
		),
		(
			"vkGetMemoryAndroidHardwareBufferANDROID",
			"get_memory_android_hardware_buffer_android",
		),
		("vkCreateViSurfaceNN", "create_vi_surface_nn"),
		("vkCreateSurfaceOHOS", "create_surface_ohos"),
		("vkCreateIOSSurfaceMVK", "create_ios_surface_mvk"),
		("vkCreateDirectFBSurfaceEXT", "create_direct_fb_surface_ext"),
	] {
		assert_eq!(command_name(native), rust);
	}
}

#[test]
fn every_registered_vendor_tag_remains_one_command_suffix() {
	use quick_xml::{Reader, events::Event};
	let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/vk.xml");
	let xml = std::fs::read_to_string(path).unwrap();
	let mut reader = Reader::from_str(&xml);
	let mut tags = 0;
	loop {
		match reader.read_event().unwrap() {
			Event::Empty(node) if node.name().as_ref() == b"tag" => {
				let tag = node
					.attributes()
					.map(|a| a.unwrap())
					.find(|a| a.key.as_ref() == b"name")
					.unwrap();
				let tag = tag.decode_and_unescape_value(reader.decoder()).unwrap();
				for stem in ["CmdExecute", "CmdExecute2"] {
					assert_eq!(
						command_name(&format!("vk{stem}{tag}")),
						format!("{}_{}", member_name(stem), tag.to_ascii_lowercase())
					);
				}
				tags += 1;
			}
			Event::Eof => break,
			_ => {}
		}
	}
	assert!(
		tags > 40,
		"registered vendor tag inventory unexpectedly small"
	);
	// Previously unknown tags follow the same rule; no list needs updating.
	assert_eq!(
		command_name("vkCmdExecuteNEWVENDOR"),
		"cmd_execute_newvendor"
	);
}

#[test]
fn test_const_name() {
	assert_eq!(const_name("VK_MAX_MEMORY_TYPES"), "MAX_MEMORY_TYPES");
	assert_eq!(const_name("VK_TRUE"), "TRUE");
}

#[test]
fn test_screaming_to_pascal() {
	assert_eq!(screaming_to_pascal("R8G8B8A8_UNORM"), "R8g8b8a8Unorm");
	assert_eq!(screaming_to_pascal("DECODE_VP9_BIT_KHR"), "DecodeVp9BitKhr");
}
