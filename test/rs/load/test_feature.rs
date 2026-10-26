use super::*;
use std::ffi::c_void;

fn base_features() -> vk::PhysicalDeviceFeatures {
	// SAFETY: every field is an integer VkBool32; zero disables all base features.
	unsafe { std::mem::zeroed() }
}

#[test]
fn base_defaults_and_owned_legacy_values() {
	// SAFETY: valid local create-info graphs, captured before their storage expires.
	let defaults = unsafe { Features::capture(&vk::DeviceCreateInfo::default()) }.unwrap();
	assert_eq!(
		defaults.enabled(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2, "shaderInt64"),
		Some(false)
	);
	let captured = {
		let base = vk::PhysicalDeviceFeatures {
			shader_int64: vk::TRUE,
			..base_features()
		};
		// SAFETY: base remains readable through capture.
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_enabled_features: &base,
				..Default::default()
			})
		}
		.unwrap()
	};
	assert_eq!(
		captured.enabled(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2, "shaderInt64"),
		Some(true)
	);
	assert_eq!(
		captured.enabled(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2, "notAFeature"),
		None
	);
	assert!(
		captured
			.require(&[(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2, "shaderInt64")])
			.is_ok()
	);
	assert!(matches!(
		captured.require(&[(
			vk::StructureType::PHYSICAL_DEVICE_FEATURES_2,
			"shaderFloat64"
		)]),
		Err(CallError::FeatureNotEnabled {
			member: "shaderFloat64",
			..
		})
	));
}

#[test]
fn features2_and_multiple_records_preserve_order_and_disabled_bits() {
	let captured = {
		let sync = vk::PhysicalDeviceSynchronization2FeaturesKHR {
			synchronization2: vk::TRUE,
			..Default::default()
		};
		let mut timeline = vk::PhysicalDeviceTimelineSemaphoreFeatures {
			p_next: (&sync as *const vk::PhysicalDeviceSynchronization2FeaturesKHR)
				.cast_mut()
				.cast(),
			timeline_semaphore: vk::FALSE,
			..Default::default()
		};
		let head = vk::PhysicalDeviceFeatures2 {
			p_next: (&mut timeline as *mut vk::PhysicalDeviceTimelineSemaphoreFeatures).cast(),
			features: vk::PhysicalDeviceFeatures {
				shader_float64: vk::TRUE,
				..base_features()
			},
			..Default::default()
		};
		// SAFETY: every linked complete record is initialized, live and aligned.
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_next: (&head as *const vk::PhysicalDeviceFeatures2).cast(),
				..Default::default()
			})
		}
		.unwrap()
	};
	assert_eq!(
		captured.enabled(
			vk::StructureType::PHYSICAL_DEVICE_FEATURES_2,
			"shaderFloat64"
		),
		Some(true)
	);
	assert_eq!(
		captured.enabled(
			vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
			"timelineSemaphore"
		),
		Some(false)
	);
	// KHR/native alias share the exact sType, so both query the same captured bit.
	assert_eq!(
		captured.enabled(
			vk::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
			"synchronization2"
		),
		Some(true)
	);
	assert!(captured.unrecognized_structures().is_empty());
	let timeline = captured
		.values()
		.iter()
		.position(|v| v.member == "timelineSemaphore")
		.unwrap();
	let sync = captured
		.values()
		.iter()
		.position(|v| v.member == "synchronization2")
		.unwrap();
	assert!(timeline < sync);
}

#[test]
fn unified_records_are_explicit_and_do_not_infer_unrecorded_equivalence() {
	let unified = vk::PhysicalDeviceVulkan13Features {
		synchronization2: vk::TRUE,
		..Default::default()
	};
	// SAFETY: initialized complete feature record remains live through capture.
	let captured = unsafe {
		Features::capture(&vk::DeviceCreateInfo {
			p_next: (&unified as *const vk::PhysicalDeviceVulkan13Features).cast(),
			..Default::default()
		})
	}
	.unwrap();
	assert_eq!(
		captured.enabled(
			vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
			"synchronization2"
		),
		Some(true)
	);
	assert_eq!(
		captured.enabled(
			vk::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
			"synchronization2"
		),
		None
	);
	assert!(
		captured
			.require(&[(
				vk::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
				"synchronization2"
			)])
			.is_err()
	);
}

#[test]
fn features2_excludes_legacy_and_duplicate_records_are_rejected() {
	let base = base_features();
	let features2 = vk::PhysicalDeviceFeatures2::default();
	// SAFETY: valid initialized storage; only the mutually exclusive combination is invalid.
	assert!(matches!(
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_enabled_features: &base,
				p_next: (&features2 as *const vk::PhysicalDeviceFeatures2).cast(),
				..Default::default()
			})
		},
		Err(CallError::InvalidMetadata(_))
	));
	let second = vk::PhysicalDeviceSynchronization2Features::default();
	let first = vk::PhysicalDeviceSynchronization2Features {
		p_next: (&second as *const vk::PhysicalDeviceSynchronization2Features)
			.cast_mut()
			.cast(),
		..Default::default()
	};
	// SAFETY: both complete typed records are readable; duplicate sType is rejected.
	assert!(matches!(
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_next: (&first as *const vk::PhysicalDeviceSynchronization2Features).cast(),
				..Default::default()
			})
		},
		Err(CallError::InvalidMetadata(_))
	));
}

#[test]
fn unknown_headers_are_not_cast_to_known_payloads_and_cycles_terminate() {
	let mut unknown = vk::BaseInStructure {
		s_type: vk::StructureType::from_raw(i32::MAX),
		p_next: std::ptr::null(),
		..Default::default()
	};
	// SAFETY: unknown node needs only its complete readable header; no payload cast occurs.
	let captured = unsafe {
		Features::capture(&vk::DeviceCreateInfo {
			p_next: (&unknown as *const vk::BaseInStructure).cast(),
			..Default::default()
		})
	}
	.unwrap();
	assert_eq!(captured.unrecognized_structures(), &[unknown.s_type]);
	unknown.p_next = &unknown;
	// SAFETY: header is readable; cyclic next pointer is detected before repeat dereference.
	assert!(matches!(
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_next: (&unknown as *const vk::BaseInStructure).cast::<c_void>(),
				..Default::default()
			})
		},
		Err(CallError::InvalidMetadata(_))
	));
}

#[test]
fn oversized_chain_is_bounded_without_reading_beyond_its_nodes() {
	let mut nodes = vec![
		vk::BaseInStructure {
			s_type: vk::StructureType::from_raw(i32::MAX),
			p_next: std::ptr::null(),
			..Default::default()
		};
		1025
	];
	for index in 0..nodes.len() - 1 {
		nodes[index].p_next = &nodes[index + 1];
	}
	// SAFETY: storage is stable, initialized and readable throughout traversal.
	assert!(matches!(
		unsafe {
			Features::capture(&vk::DeviceCreateInfo {
				p_next: nodes.as_ptr().cast(),
				..Default::default()
			})
		},
		Err(CallError::InvalidMetadata(_))
	));
}
