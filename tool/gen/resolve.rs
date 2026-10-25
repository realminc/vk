//! Resolve aliases with cycle detection, then assign dispatch domains.
use crate::model::{CommandDecl, DispatchLevel, Registry, TypeKind};
use anyhow::{Result, anyhow, ensure};
use std::collections::{HashMap, HashSet};

pub fn resolve(reg: &mut Registry) -> Result<()> {
	let map: HashMap<_, _> = reg
		.commands
		.iter()
		.map(|c| (c.c_name.clone(), c.clone()))
		.collect();
	for cmd in &mut reg.commands {
		let mut target = cmd.clone();
		let mut seen = HashSet::new();
		while let Some(alias) = &target.alias_of {
			ensure!(
				seen.insert(target.c_name.clone()),
				"command alias cycle at {}",
				cmd.c_name
			);
			target = map
				.get(alias)
				.ok_or_else(|| anyhow!("missing command alias target {alias}"))?
				.clone();
		}
		cmd.parameters = target.parameters;
		cmd.return_type = target.return_type;
		cmd.success_codes = target.success_codes;
		cmd.dispatch = dispatch_for(cmd, &reg.types)?;
	}
	select_vulkan(reg);
	crate::protect::resolve(reg)?;
	for e in &mut reg.enums {
		let mut seen = HashSet::new();
		e.values.retain(|v| seen.insert(v.name.clone()));
	}
	Ok(())
}

fn dispatch_for(cmd: &CommandDecl, types: &[crate::model::TypeDecl]) -> Result<DispatchLevel> {
	if matches!(
		cmd.c_name.as_str(),
		"vkCreateInstance"
			| "vkEnumerateInstanceExtensionProperties"
			| "vkEnumerateInstanceLayerProperties"
			| "vkEnumerateInstanceVersion"
			| "vkGetInstanceProcAddr"
	) {
		return Ok(DispatchLevel::Global);
	}
	// Obtained through the instance resolver, then retained directly by Device.
	if cmd.c_name == "vkGetDeviceProcAddr" {
		return Ok(DispatchLevel::Instance);
	}
	let mut name = cmd
		.parameters
		.first()
		.ok_or_else(|| anyhow!("non-global command {} has no dispatch handle", cmd.c_name))?
		.ty
		.name
		.as_str();
	let mut seen = HashSet::new();
	loop {
		ensure!(seen.insert(name), "handle ancestry cycle at {name}");
		match name {
			"VkInstance" | "VkPhysicalDevice" => return Ok(DispatchLevel::Instance),
			"VkDevice" => return Ok(DispatchLevel::Device),
			_ => {}
		}
		let ty = types
			.iter()
			.find(|t| t.c_name == name)
			.ok_or_else(|| anyhow!("unknown dispatch handle {name} for {}", cmd.c_name))?;
		name = ty
			.alias_of
			.as_deref()
			.or(ty.parent.as_deref())
			.ok_or_else(|| anyhow!("no dispatch ancestry for {name}"))?;
	}
}

/// Keep the Vulkan requirement closure, excluding SC-only and disabled declarations.
fn select_vulkan(reg: &mut Registry) {
	let mut commands: HashSet<String> = reg
		.features
		.iter()
		.flat_map(|f| f.requires_commands.iter())
		.chain(
			reg
				.extensions
				.iter()
				.flat_map(|e| e.requires_commands.iter()),
		)
		.cloned()
		.collect();
	loop {
		let previous = commands.len();
		for c in &reg.commands {
			if commands.contains(&c.c_name)
				&& let Some(alias) = &c.alias_of
			{
				commands.insert(alias.clone());
			}
		}
		if commands.len() == previous {
			break;
		}
	}
	reg.commands.retain(|c| commands.contains(&c.c_name));
	let mut types: HashSet<String> = reg
		.features
		.iter()
		.flat_map(|f| f.requires_types.iter())
		.chain(reg.extensions.iter().flat_map(|e| e.requires_types.iter()))
		.cloned()
		.collect();
	types.extend(
		reg
			.types
			.iter()
			.filter(|t| t.from_video)
			.map(|t| t.c_name.clone()),
	);
	for c in &reg.commands {
		types.insert(c.return_type.name.clone());
		types.extend(c.parameters.iter().map(|p| p.ty.name.clone()));
	}
	loop {
		let previous = types.len();
		for t in &reg.types {
			if !types.contains(&t.c_name) {
				continue;
			}
			match &t.kind {
				TypeKind::Alias { target } => {
					types.insert(target.clone());
				}
				TypeKind::Bitmask { base, requires } => {
					types.insert(base.clone());
					types.extend(requires.iter().cloned());
				}
				TypeKind::BaseType { ty } => {
					types.insert(ty.name.clone());
				}
				TypeKind::Struct { members, .. } | TypeKind::Union { members } => {
					types.extend(members.iter().map(|m| m.ty.name.clone()))
				}
				TypeKind::FuncPointer {
					return_type,
					parameters,
				} => {
					types.insert(return_type.name.clone());
					types.extend(parameters.iter().map(|p| p.ty.name.clone()));
				}
				_ => {}
			}
		}
		if types.len() == previous {
			break;
		}
	}
	reg.types.retain(|t| types.contains(&t.c_name));
	reg.enums.retain(|e| types.contains(&e.c_name));
}
