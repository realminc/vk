//! Vulkan API version encoding.

#[test]
fn version_variant_round_trip() {
	let v = vk::make_api_version(3, 7, 21, 1234);
	assert_eq!(vk::api_version_variant(v), 3);
	assert_eq!(vk::api_version_major(v), 7);
	assert_eq!(vk::api_version_minor(v), 21);
	assert_eq!(vk::api_version_patch(v), 1234);
}
