use super::*;
use crate::model::TypeRef;

fn parameter(name: &str, ty: &str, pointers: &[bool], len: Option<&str>) -> Parameter {
	Parameter {
		name: name.into(),
		ty: TypeRef {
			name: ty.into(),
			pointers: pointers.into(),
			arrays: vec![],
			array_element_const: false,
		},
		len: len.map(str::to_owned),
		optional: false,
		extern_sync: false,
	}
}

fn fixture(parameters: Vec<Parameter>) -> CommandDecl {
	CommandDecl {
		c_name: "vkFixture".into(),
		rust_name: String::new(),
		return_type: TypeRef {
			name: "void".into(),
			pointers: vec![],
			arrays: vec![],
			array_element_const: false,
		},
		success_codes: vec![],
		parameters,
		dispatch: DispatchLevel::Device,
		alias_of: None,
		guards: vec![],
		feature: None,
	}
}

#[test]
fn unsupported_lengths_and_nested_pointers_fail_instead_of_guessing() {
	for (pointers, len) in [
		(&[true][..], Some("latexmath:[2N]")),
		(&[true, true][..], None),
	] {
		let command = fixture(vec![parameter("pInput", "VkFixture", pointers, len)]);
		assert!(operation(&Registry::default(), &command, Recipe::Forward).is_err());
	}
	let command = fixture(vec![
		parameter("count", "uint64_t", &[], None),
		parameter("pInputs", "VkFixture", &[true], Some("count")),
	]);
	assert!(operation(&Registry::default(), &command, Recipe::Forward).is_err());
}

#[test]
fn optional_length_records_are_rejected() {
	let mut input = parameter("pInfo", "VkFixture", &[true], None);
	input.optional = true;
	let command = fixture(vec![
		input,
		parameter("pOutput", "VkFixture", &[false], Some("pInfo->count")),
	]);
	assert!(operation(&Registry::default(), &command, Recipe::Forward).is_err());
}

#[test]
fn missing_result_policy_is_rejected() {
	let mut command = fixture(vec![]);
	command.return_type.name = "VkResult".into();
	assert!(operation(&Registry::default(), &command, Recipe::Forward).is_err());
	command.success_codes.push("VK_SUCCESS".into());
	assert!(
		!operation(&Registry::default(), &command, Recipe::Forward)
			.unwrap()
			.status
	);
	command
		.success_codes
		.push("VK_PIPELINE_COMPILE_REQUIRED_EXT".into());
	assert!(
		operation(&Registry::default(), &command, Recipe::Forward)
			.unwrap()
			.status
	);
}

#[test]
fn admission_policy_has_unique_native_and_rust_names() {
	let mut native = std::collections::HashSet::new();
	let mut rust = std::collections::HashSet::new();
	for (name, _) in selected() {
		assert!(native.insert(name.clone()));
		assert!(rust.insert(names::command_name(&name)));
	}
	assert!(emit_commands(&Registry::default()).is_err());
}

#[test]
fn fixed_arrays_and_byte_counts_preserve_the_native_shape() {
	let mut array = parameter("blendConstants", "float", &[], None);
	array.ty.arrays = vec!["4".into()];
	array.ty.array_element_const = true;
	let op = operation(
		&Registry::default(),
		&fixture(vec![array.clone()]),
		Recipe::Forward,
	)
	.unwrap();
	assert_eq!(op.arguments[0].ty, "&[f32; 4]");
	assert_eq!(op.native, ["blend_constants.as_ptr()"]);
	for extent in ["0", "VK_UNKNOWN", "-1"] {
		array.ty.arrays = vec![extent.into()];
		assert!(
			operation(
				&Registry::default(),
				&fixture(vec![array.clone()]),
				Recipe::Forward
			)
			.is_err()
		);
	}
	array.ty.arrays = vec!["4".into()];
	array.ty.array_element_const = false;
	assert!(operation(&Registry::default(), &fixture(vec![array]), Recipe::Forward).is_err());
	for (count_type, expected_type) in [
		("uint32_t", "u32"),
		("size_t", "usize"),
		("VkDeviceSize", "vk::DeviceSize"),
	] {
		let op = operation(
			&Registry::default(),
			&fixture(vec![
				parameter("dataSize", count_type, &[], None),
				parameter("pData", "void", &[true], Some("dataSize")),
			]),
			Recipe::Forward,
		)
		.unwrap();
		assert_eq!(op.arguments[0].ty, "&[u8]");
		if count_type == "size_t" {
			assert!(op.checks.contains("let data_size = data.len();"));
		} else {
			assert!(
				op.checks
					.contains(&format!("{expected_type}::try_from(data.len())"))
			);
		}
		assert_eq!(op.native, ["data_size", "data.as_ptr().cast()"]);
	}
}

#[test]
fn enumeration_recipes_reject_unknown_counts_statuses_and_chained_outputs() {
	use crate::model::{StructMember, TypeDecl};
	let mut reg = Registry::default();
	reg.types.push(TypeDecl {
		parent: None,
		c_name: "VkFixture".into(),
		kind: TypeKind::Struct {
			members: vec![],
			extends: vec![],
			returned_only: true,
		},
		guards: vec![],
		alias_of: None,
		feature: None,
		from_video: false,
	});
	let mut command = fixture(vec![
		parameter("device", "VkDevice", &[], None),
		parameter("pCount", "uint32_t", &[false], None),
		parameter("pValues", "VkFixture", &[false], Some("pCount")),
	]);
	assert!(operation(&reg, &command, Recipe::Enumerate).is_err());
	command.parameters[2].optional = true;
	assert!(operation(&reg, &command, Recipe::Enumerate).is_ok());
	command.parameters[1].ty.name = "uint64_t".into();
	assert!(operation(&reg, &command, Recipe::Enumerate).is_err());
	command.parameters[1].ty.name = "uint32_t".into();
	command.return_type.name = "VkResult".into();
	command.success_codes = vec!["VK_SUCCESS".into(), "VK_TIMEOUT".into()];
	assert!(operation(&reg, &command, Recipe::Enumerate).is_err());
	command.success_codes = vec!["VK_SUCCESS".into(), "VK_INCOMPLETE".into()];
	assert!(operation(&reg, &command, Recipe::Enumerate).is_ok());
	let TypeKind::Struct { members, .. } = &mut reg.types[0].kind else {
		unreachable!()
	};
	members.push(StructMember {
		name: "sType".into(),
		ty: parameter("sType", "VkStructureType", &[], None).ty,
		values: None,
		len: None,
		optional: false,
		selector: None,
		selection: None,
		no_auto_validity: false,
		bitfield: None,
	});
	assert!(operation(&reg, &command, Recipe::Enumerate).is_err());
}

#[test]
fn counted_outputs_require_typed_headers_and_exact_count_protocol() {
	let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/vk.xml");
	let mut reg = crate::parse::parse_registry(&path, false).unwrap();
	crate::resolve::resolve(&mut reg).unwrap();
	for name in [
		"vkEnumeratePhysicalDeviceGroups",
		"vkGetImageSparseMemoryRequirements2",
		"vkGetPhysicalDeviceQueueFamilyProperties2",
		"vkGetPhysicalDeviceSparseImageFormatProperties2",
	] {
		let command = reg.commands.iter().find(|c| c.c_name == name).unwrap();
		let op = operation(&reg, command, Recipe::Counted).unwrap();
		assert!(op.enumeration.is_none());
		assert_eq!(op.arguments[op.arguments.len() - 2].ty, "&mut u32");
		assert!(
			op.arguments
				.last()
				.unwrap()
				.ty
				.starts_with("Option<&mut [vk::")
		);
		assert!(operation(&reg, command, Recipe::Enumerate).is_err());
		let count = command.parameters.len() - 2;
		let data = count + 1;
		for case in 0..5 {
			let mut bad = command.clone();
			match case {
				0 => bad.parameters[count].ty.name = "size_t".into(),
				1 => bad.parameters[data].optional = false,
				2 => bad.parameters[data].len = Some("unknown".into()),
				3 => bad.parameters[data].ty.name = "VkLayerProperties".into(),
				_ => bad.parameters[count].ty.pointers = vec![true],
			}
			assert!(
				operation(&reg, &bad, Recipe::Counted).is_err(),
				"{name}, {case}"
			);
		}
	}
	let mut command = reg
		.commands
		.iter()
		.find(|c| c.c_name == "vkEnumeratePhysicalDeviceGroups")
		.unwrap()
		.clone();
	command.success_codes.push("VK_TIMEOUT".into());
	assert!(operation(&reg, &command, Recipe::Counted).is_err());
}

#[test]
fn template_payload_stays_opaque_and_other_void_pointers_stay_rejected() {
	let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/vk.xml");
	let mut reg = crate::parse::parse_registry(&path, false).unwrap();
	crate::resolve::resolve(&mut reg).unwrap();
	let command = reg
		.commands
		.iter()
		.find(|c| c.c_name == "vkUpdateDescriptorSetWithTemplate")
		.unwrap();
	let op = operation(&reg, command, Recipe::TemplateUpdate).unwrap();
	assert_eq!(op.arguments.last().unwrap().ty, "*const std::ffi::c_void");
	assert_eq!(op.native.last().unwrap(), "data");
	assert!(operation(&reg, command, Recipe::Forward).is_err());
	for case in 0..7 {
		let mut bad = command.clone();
		match case {
			0 => bad.parameters[3].ty.pointers = vec![true, true],
			1 => bad.parameters[3].len = Some("descriptorSet".into()),
			2 => bad.parameters[3].ty.pointers = vec![false],
			3 => bad.parameters[3].ty.pointers.clear(),
			4 => bad.parameters[3].ty.arrays.push("4".into()),
			5 => bad.parameters[3].optional = true,
			_ => bad.parameters[1].ty.name = "VkBuffer".into(),
		}
		assert!(operation(&reg, &bad, Recipe::TemplateUpdate).is_err());
	}
}

#[test]
fn scalar_queries_preserve_native_width_and_reject_unreviewed_returns() {
	let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/vk.xml");
	let mut reg = crate::parse::parse_registry(&path, false).unwrap();
	crate::resolve::resolve(&mut reg).unwrap();
	for name in [
		"vkGetBufferDeviceAddress",
		"vkGetBufferDeviceAddressKHR",
		"vkGetBufferDeviceAddressEXT",
		"vkGetBufferOpaqueCaptureAddress",
		"vkGetBufferOpaqueCaptureAddressKHR",
		"vkGetDeviceMemoryOpaqueCaptureAddress",
		"vkGetDeviceMemoryOpaqueCaptureAddressKHR",
	] {
		let command = reg.commands.iter().find(|c| c.c_name == name).unwrap();
		let op = operation(&reg, command, Recipe::Scalar).unwrap();
		assert_eq!(
			return_type(&op),
			if name.contains("Opaque") {
				"u64"
			} else {
				"vk::DeviceAddress"
			}
		);
		assert!(op.output.is_none() && !op.status && op.enumeration.is_none());
		assert_eq!(op.native, ["device.handle()", "info"]);
		assert!(operation(&reg, command, Recipe::Forward).is_err());
		for case in 0..13 {
			let mut invalid = command.clone();
			match case {
				0 => invalid.return_type.name = "uint32_t".into(),
				1 => invalid.return_type.name = "VkResult".into(),
				2 => invalid.return_type.name = "void".into(),
				3 => invalid.return_type.pointers = vec![true],
				4 => invalid.return_type.arrays = vec!["2".into()],
				5 => invalid.success_codes.push("VK_SUCCESS".into()),
				6 => invalid.parameters[1].optional = true,
				7 => invalid.parameters[1].ty.pointers = vec![false],
				8 => invalid.parameters[1].len = Some("size".into()),
				9 => invalid.parameters[1].ty.arrays = vec!["1".into()],
				10 => invalid.parameters[1].ty.name = "VkBuffer".into(),
				11 => invalid.parameters[0].ty.name = "VkPhysicalDevice".into(),
				12 => invalid.dispatch = DispatchLevel::Instance,
				_ => unreachable!(),
			}
			assert!(
				operation(&reg, &invalid, Recipe::Scalar).is_err(),
				"{name}: case {case}"
			);
		}
	}
}

#[test]
fn native_recipe_preserves_nested_optional_pointers_counts_and_arrays() {
	let mut inputs = vec![
		parameter("device", "VkDevice", &[], None),
		parameter("count", "uint32_t", &[], None),
		parameter("ppData", "void", &[true, true], Some("latexmath:[2N]")),
		parameter("pOutput", "void", &[false, false], None),
		parameter("function", "uint64_t", &[], None),
	];
	inputs[2].optional = true;
	let mut array = parameter("values", "float", &[], None);
	array.ty.arrays = vec!["4".into(), "3".into()];
	array.ty.array_element_const = true;
	inputs.push(array);
	let command = fixture(inputs);
	let op = operation(&Registry::default(), &command, Recipe::Native).unwrap();
	assert_eq!(
		op.native,
		[
			"device.handle()",
			"count",
			"pp_data",
			"p_output",
			"function",
			"values"
		]
	);
	assert_eq!(
		op.arguments
			.iter()
			.map(|a| a.ty.as_str())
			.collect::<Vec<_>>(),
		[
			"u32",
			"*const *const std::ffi::c_void",
			"*mut *mut std::ffi::c_void",
			"u64",
			"*const [f32; 3]"
		]
	);
	assert!(op.checks.is_empty());
	assert!(op.output.is_none() && op.enumeration.is_none() && !op.counted);
}

#[test]
fn native_returns_preserve_statuses_and_scalar_widths_without_output_ownership() {
	for (name, expected) in [
		("void", "()"),
		("VkResult", "vk::Result"),
		("VkBool32", "vk::Bool32"),
		("VkDeviceAddress", "vk::DeviceAddress"),
		("VkDeviceSize", "vk::DeviceSize"),
		("uint32_t", "u32"),
		("uint64_t", "u64"),
	] {
		let mut command = fixture(vec![]);
		command.return_type.name = name.into();
		if name == "VkResult" {
			command.success_codes.push("VK_SUCCESS".into());
		}
		let op = operation(&Registry::default(), &command, Recipe::Native).unwrap();
		assert_eq!(return_type(&op), expected);
		assert_eq!(op.status, name == "VkResult");
		assert!(op.output.is_none());
		command.return_type.pointers.push(false);
		assert!(operation(&Registry::default(), &command, Recipe::Native).is_err());
	}
	let mut command = fixture(vec![]);
	command.return_type.name = "VkResult".into();
	assert!(operation(&Registry::default(), &command, Recipe::Native).is_err());
	command.return_type.name = "VkUnknownRecord".into();
	assert!(operation(&Registry::default(), &command, Recipe::Native).is_err());
	let command = fixture(vec![
		parameter("pData", "void", &[true], None),
		parameter("p_data", "void", &[false], None),
	]);
	assert!(operation(&Registry::default(), &command, Recipe::Native).is_err());
}
