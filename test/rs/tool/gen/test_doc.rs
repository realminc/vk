use super::*;

#[test]
fn native_handle_names_are_search_aliases_and_refpage_targets() {
	let mut out = String::new();
	item(
		&mut out,
		"VkDevice",
		"DeviceHandle",
		"Raw Vulkan handle",
		false,
	);
	assert!(out.contains("#[doc(alias = \"VkDevice\")]"));
	assert!(out.contains("/VkDevice.html)"));
	assert!(!out.contains("/DeviceHandle.html"));
}

#[test]
fn command_pfns_link_to_the_command_and_values_to_their_enum() {
	let mut out = String::new();
	command(&mut out, "vkCreateDevice", "PFN_vkCreateDevice");
	assert!(out.contains("/vkCreateDevice.html)"));
	assert!(!out.contains("/PFN_vkCreateDevice.html"));
	out.clear();
	reference(&mut out, "VK_SUCCESS", "VkResult", false);
	assert!(out.contains("/VkResult.html)"));
	assert!(!out.contains("/VK_SUCCESS.html"));
}

#[test]
fn video_and_external_symbols_use_pinned_registry_sources() {
	for (native, video, file) in [
		("StdVideoDecodeVP9PictureInfo", true, "video.xml"),
		("STD_VIDEO_VP9_MAX_SEGMENTS", true, "video.xml"),
		("xcb_connection_t", false, "vk.xml"),
		("VK_WHOLE_SIZE", false, "vk.xml"),
	] {
		let mut out = String::new();
		reference(&mut out, native, native, video);
		assert!(out.contains(&format!("/registry/{file})")));
		assert!(!out.contains("refpages"));
		assert!(!out.contains("/main/"));
	}
}

#[test]
fn identical_and_raw_identifier_names_do_not_create_invalid_aliases() {
	let mut out = String::new();
	alias(&mut out, "type", "r#type");
	alias(&mut out, "StdVideoVP9Profile", "StdVideoVP9Profile");
	assert!(out.is_empty());
	alias(&mut out, "sType", "s_type");
	assert_eq!(out, "#[doc(alias = \"sType\")]\n");
}
