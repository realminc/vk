use super::*;

unsafe fn instance(version: u32, extensions: &[&CStr]) -> Capabilities {
	let app = vk::ApplicationInfo {
		api_version: version,
		..Default::default()
	};
	let names: Vec<_> = extensions.iter().map(|name| name.as_ptr()).collect();
	// SAFETY: borrowed application info and names remain valid through capture.
	unsafe {
		Capabilities::instance(&vk::InstanceCreateInfo {
			p_application_info: &app,
			enabled_extension_count: names.len() as u32,
			pp_enabled_extension_names: names.as_ptr(),
			..Default::default()
		})
	}
	.unwrap()
}

#[test]
fn version_defaults_patch_and_alias_spelling_are_exact() {
	// SAFETY: local, valid create-info metadata; no native calls.
	unsafe {
		let baseline = Capabilities::instance(&vk::InstanceCreateInfo::default()).unwrap();
		assert_eq!(baseline.api_version(), vk::API_VERSION_1_0);
		assert_eq!(
			baseline.command_support(c"vkGetPhysicalDeviceFeatures2"),
			Some(CommandSupport::Disabled)
		);
		let promoted = instance(vk::API_VERSION_1_1 | 999, &[]);
		assert_eq!(promoted.api_version(), vk::API_VERSION_1_1);
		assert_eq!(
			promoted.command_support(c"vkGetPhysicalDeviceFeatures2"),
			Some(CommandSupport::RequiresPhysicalDevice)
		);
		assert_eq!(
			promoted.command_support(c"vkGetPhysicalDeviceFeatures2KHR"),
			Some(CommandSupport::Disabled)
		);
		let extension = instance(0, &[c"VK_KHR_get_physical_device_properties2"]);
		assert_eq!(
			extension.command_support(c"vkGetPhysicalDeviceFeatures2KHR"),
			Some(CommandSupport::Enabled)
		);
		assert_eq!(
			extension.command_support(c"vkGetPhysicalDeviceFeatures2"),
			Some(CommandSupport::Disabled)
		);
		assert_eq!(extension.command_support(c"vkUnknown"), None);
	}
}

#[test]
fn instance_and_physical_api_ceilings_are_distinct() {
	// SAFETY: valid metadata-only create info; no native effects.
	let mut parent = unsafe { instance(vk::API_VERSION_1_3, &[]) };
	parent.limit_instance_version(vk::API_VERSION_1_0).unwrap();
	assert_eq!(parent.api_version(), vk::API_VERSION_1_3);
	assert_eq!(parent.instance_api_version(), vk::API_VERSION_1_0);
	assert_eq!(
		parent.command_support(c"vkEnumeratePhysicalDeviceGroups"),
		Some(CommandSupport::Disabled)
	);
	assert_eq!(
		parent.command_support(c"vkGetPhysicalDeviceFeatures2"),
		Some(CommandSupport::RequiresPhysicalDevice)
	);
	// SAFETY: valid local device create-info graph and physical-device version.
	let device =
		unsafe { parent.device(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_2) }.unwrap();
	assert_eq!(device.api_version(), vk::API_VERSION_1_2);
	assert_eq!(device.instance_api_version(), vk::API_VERSION_1_0);
	assert_eq!(
		device.command_support(c"vkGetBufferMemoryRequirements2"),
		Some(CommandSupport::Enabled)
	);
}

#[test]
fn device_capabilities_cap_version_and_own_extension_storage() {
	// SAFETY: all inputs are valid borrowed C strings/create-info graphs.
	unsafe {
		let parent = instance(vk::API_VERSION_1_3, &[c"VK_KHR_surface"]);
		let device = {
			let extension = std::ffi::CString::new("VK_KHR_get_memory_requirements2").unwrap();
			let names = [extension.as_ptr()];
			parent
				.device(
					&vk::DeviceCreateInfo {
						enabled_extension_count: 1,
						pp_enabled_extension_names: names.as_ptr(),
						..Default::default()
					},
					vk::API_VERSION_1_0,
				)
				.unwrap()
		};
		assert_eq!(device.api_version(), vk::API_VERSION_1_0);
		assert_eq!(
			device.device_extensions().unwrap().collect::<Vec<_>>(),
			[c"VK_KHR_get_memory_requirements2"]
		);
		assert_eq!(
			device.instance_extensions().collect::<Vec<_>>(),
			[c"VK_KHR_surface"]
		);
		assert_eq!(
			device.command_support(c"vkGetBufferMemoryRequirements2"),
			Some(CommandSupport::Disabled)
		);
		assert_eq!(
			device.command_support(c"vkGetBufferMemoryRequirements2KHR"),
			Some(CommandSupport::Enabled)
		);
		assert_eq!(
			device.command_support(c"vkBindBufferMemory2KHR"),
			Some(CommandSupport::Disabled)
		);
		assert_eq!(
			parent.command_support(c"vkGetPhysicalDeviceVideoCapabilitiesKHR"),
			Some(CommandSupport::RequiresPhysicalDevice)
		);
	}
}

#[test]
fn malformed_metadata_fails_before_any_native_object_exists() {
	// SAFETY: only deliberately null pointers are invalid; capture rejects them
	// before dereferencing. No out-of-bounds/dangling pointers are constructed.
	unsafe {
		assert!(matches!(
			Capabilities::instance(&vk::InstanceCreateInfo {
				enabled_extension_count: 1,
				..Default::default()
			}),
			Err(CallError::InvalidOutput(_))
		));
		let names = [std::ptr::null()];
		assert!(matches!(
			Capabilities::instance(&vk::InstanceCreateInfo {
				enabled_extension_count: 1,
				pp_enabled_extension_names: names.as_ptr(),
				..Default::default()
			}),
			Err(CallError::InvalidOutput(_))
		));
	}
	assert!(normalize_version(vk::make_api_version(1, 1, 0, 0)).is_err());
	assert!(normalize_version(1).is_err());
}

#[test]
fn conditional_extension_dependencies_and_unknown_physical_support_do_not_become_enabled() {
	use CapabilityRequirement::*;
	let command = |requirements| CommandMetadata {
		name: "vkFixture",
		alias_of: None,
		dispatch: "Instance",
		physical_device: false,
		requirements,
	};
	// SAFETY: these owned metadata snapshots use fully valid local create infos.
	unsafe {
		let parent = instance(vk::API_VERSION_1_0, &[]);
		assert_eq!(
			parent.support(&command(&[&[
				DeviceExtension(c"VK_EXT_unknown"),
				InstanceExtension(c"VK_EXT_missing")
			]])),
			CommandSupport::Disabled
		);
		assert_eq!(
			parent.support(&command(&[&[
				DeviceExtension(c"VK_EXT_unknown"),
				Core(vk::API_VERSION_1_0)
			]])),
			CommandSupport::RequiresPhysicalDevice
		);
		assert_eq!(
			parent.support(&command(&[
				&[DeviceExtension(c"VK_EXT_unknown")],
				&[Core(vk::API_VERSION_1_0)]
			])),
			CommandSupport::Enabled
		);
		let names = [c"VK_EXT_host_image_copy".as_ptr()];
		let info = vk::DeviceCreateInfo {
			enabled_extension_count: 1,
			pp_enabled_extension_names: names.as_ptr(),
			..Default::default()
		};
		let device12 = instance(vk::API_VERSION_1_2, &[])
			.device(&info, vk::API_VERSION_1_2)
			.unwrap();
		assert_eq!(
			device12.command_support(c"vkCopyMemoryToImageEXT"),
			Some(CommandSupport::Disabled)
		);
		let device13 = instance(vk::API_VERSION_1_3, &[])
			.device(&info, vk::API_VERSION_1_3)
			.unwrap();
		assert_eq!(
			device13.command_support(c"vkCopyMemoryToImageEXT"),
			Some(CommandSupport::Enabled)
		);
		assert_eq!(
			device13.command_support(c"vkCopyMemoryToImage"),
			Some(CommandSupport::Disabled)
		);
	}
}

#[test]
fn external_metadata_constructors_preserve_scope_and_features() {
	// SAFETY: local initialized graphs and explicitly supplied API versions.
	unsafe {
		let parent = Capabilities::new_from_instance_create_info(
			&vk::InstanceCreateInfo::default(),
			vk::API_VERSION_1_1,
		)
		.unwrap();
		assert!(parent.features().is_none());
		let device = parent
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_1)
			.unwrap();
		assert_eq!(
			device
				.features()
				.unwrap()
				.enabled(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2, "shaderInt64"),
			Some(false)
		);
		assert!(matches!(
			device.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_1),
			Err(CallError::InvalidMetadata(_))
		));
	}
}

#[test]
fn import_parent_matching_compares_extension_sets_in_both_directions() {
	// SAFETY: local C-string arrays are only captured as metadata, never passed
	// to a driver; duplicate/order variants exercise effective-set comparison.
	unsafe {
		let parent = instance(
			0,
			&[c"VK_KHR_surface", c"VK_KHR_get_physical_device_properties2"],
		);
		let wrong = instance(0, &[c"VK_KHR_surface", c"VK_KHR_surface"])
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_0)
			.unwrap();
		assert!(!wrong.matches_instance(&parent));
		let reordered = instance(
			0,
			&[
				c"VK_KHR_get_physical_device_properties2",
				c"VK_KHR_surface",
				c"VK_KHR_surface",
			],
		)
		.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_0)
		.unwrap();
		assert!(reordered.matches_instance(&parent));
	}
}
