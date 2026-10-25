//! Audit pass: verify completeness of the parsed registry against known
//! Vulkan invariants and report any unexpected gaps.

use crate::model::{DispatchLevel, Registry};

/// Run all audit checks.  Returns a list of error strings; an empty vec means
/// the registry passed all checks.
pub fn audit(reg: &Registry) -> Vec<String> {
	let mut errors: Vec<String> = Vec::new();

	check_required_commands(reg, &mut errors);
	check_dispatch_levels(reg, &mut errors);
	check_no_duplicate_types(reg, &mut errors);

	errors
}

fn check_required_commands(reg: &Registry, errors: &mut Vec<String>) {
	const REQUIRED: &[&str] = &[
		"vkCreateInstance",
		"vkDestroyInstance",
		"vkEnumeratePhysicalDevices",
		"vkGetDeviceProcAddr",
		"vkGetInstanceProcAddr",
		"vkCreateDevice",
		"vkDestroyDevice",
		"vkEnumerateInstanceExtensionProperties",
		"vkEnumerateInstanceLayerProperties",
		"vkEnumerateInstanceVersion",
	];
	let names: std::collections::HashSet<&str> =
		reg.commands.iter().map(|c| c.c_name.as_str()).collect();
	for r in REQUIRED {
		if !names.contains(r) {
			errors.push(format!("missing required command: {r}"));
		}
	}
}

fn check_dispatch_levels(reg: &Registry, errors: &mut Vec<String>) {
	// These commands must be Global.
	const MUST_BE_GLOBAL: &[&str] = &[
		"vkCreateInstance",
		"vkEnumerateInstanceExtensionProperties",
		"vkEnumerateInstanceLayerProperties",
		"vkEnumerateInstanceVersion",
	];
	for cmd in &reg.commands {
		if MUST_BE_GLOBAL.contains(&cmd.c_name.as_str()) && cmd.dispatch != DispatchLevel::Global {
			errors.push(format!(
				"command {} should be Global dispatch, got {:?}",
				cmd.c_name, cmd.dispatch
			));
		}
	}
}

fn check_no_duplicate_types(reg: &Registry, errors: &mut Vec<String>) {
	// Skip well-known macro-define entries that appear as both type and constant
	// in the registry (e.g. VK_HEADER_VERSION, VK_DEFINE_HANDLE).
	const KNOWN_MACRO_DUPS: &[&str] = &[
		"VK_HEADER_VERSION",
		"VK_HEADER_VERSION_COMPLETE",
		"VK_DEFINE_HANDLE",
		"VK_DEFINE_NON_DISPATCHABLE_HANDLE",
		// These bitmask types are defined once as a typedef and again as an
		// alias promoted by an extension; both entries are correct.
		"VkPipelineDepthStencilStateCreateFlags",
		"VkPipelineColorBlendStateCreateFlags",
	];
	let mut seen = std::collections::HashSet::new();
	for ty in &reg.types {
		if KNOWN_MACRO_DUPS.contains(&ty.c_name.as_str()) {
			continue;
		}
		if !seen.insert(ty.c_name.clone()) {
			errors.push(format!("duplicate type definition: {}", ty.c_name));
		}
	}
}
