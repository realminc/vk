//! Inspectable extension and command provenance without loading a driver.
use crate::model::Registry;
use anyhow::{Result, anyhow, ensure};
use std::fmt::Write;

pub fn emit_metadata(reg: &Registry) -> Result<String> {
	let mut out = String::from(
		"// @generated — registry facts, not enabled-device capabilities.\n\n#[derive(Debug)]\npub struct ExtensionMetadata {pub name: &'static std::ffi::CStr, pub number:u32, pub scope:&'static str, pub platform:Option<&'static str>,pub provisional:bool,pub promoted_to:Option<&'static str>,pub depends:Option<&'static str>,pub standard_video:bool,pub commands:&'static [&'static str]}\n\npub const EXTENSIONS:&[ExtensionMetadata]=&[\n",
	);
	for ext in &reg.extensions {
		writeln!(out,"ExtensionMetadata {{ name:c{:?},number:{},scope:{:?},platform:{:?},provisional:{},promoted_to:{:?},depends:{:?},standard_video:{},commands:&{:?} }},",ext.name,ext.number,ext.kind,ext.platform,ext.provisional,ext.promoted_to,ext.depends,ext.from_video,ext.requires_commands).unwrap();
	}
	out.push_str("];\n\n/// A registry version or enabled-extension requirement; not feature-bit validation.\n#[derive(Debug, Clone, Copy)]\npub enum CapabilityRequirement {Core(u32), InstanceExtension(&'static std::ffi::CStr), DeviceExtension(&'static std::ffi::CStr)}\n\n#[derive(Debug)]\npub struct CommandMetadata {pub name:&'static str,pub alias_of:Option<&'static str>,pub dispatch:&'static str, /// Physical-device query, even though it uses instance dispatch.\npub physical_device:bool, /// Alternatives of conjunctions for this exact command spelling.\npub requirements:&'static [&'static [CapabilityRequirement]]}\npub const COMMANDS:&[CommandMetadata]=&[\n");
	for cmd in &reg.commands {
		let mut alternatives = Vec::new();
		for expression in reg
			.command_requirements
			.get(&cmd.c_name)
			.into_iter()
			.flatten()
		{
			for clause in crate::capability::clauses(expression)? {
				let mut atoms = Vec::new();
				for token in clause {
					if let Some(feature) = reg.features.iter().find(|f| f.name == token) {
						let (major, minor) = feature
							.number
							.split_once('.')
							.ok_or_else(|| anyhow!("invalid core version {}", feature.number))?;
						let major: u32 = major.parse()?;
						let minor: u32 = minor.parse()?;
						ensure!(major < 128 && minor < 1024, "core version out of range");
						atoms.push(format!(
							"CapabilityRequirement::Core({})",
							(major << 22) | (minor << 12)
						));
					} else {
						let ext = reg
							.extensions
							.iter()
							.find(|e| e.name == token)
							.ok_or_else(|| anyhow!("unknown capability token {token}"))?;
						let scope = match ext.kind.as_str() {
							"instance" => "InstanceExtension",
							"device" => "DeviceExtension",
							_ => return Err(anyhow!("invalid extension scope {}", ext.kind)),
						};
						atoms.push(format!("CapabilityRequirement::{scope}(c{token:?})"));
					}
				}
				alternatives.push(format!("&[{}]", atoms.join(",")));
			}
		}
		// Alias closure can retain a spelling with no provider of its own. Such a
		// spelling stays disabled; an alias never inherits its target's admission.
		writeln!(
			out,
			"CommandMetadata {{name:{:?},alias_of:{:?},dispatch:{:?},physical_device:{},requirements:&[{}]}},",
			cmd.c_name,
			cmd.alias_of,
			format!("{:?}", cmd.dispatch),
			cmd.parameters.first().is_some_and(|parameter| parameter.ty.name == "VkPhysicalDevice"),
			alternatives.join(",")
		)
		.unwrap();
	}
	out.push_str("];\n");
	Ok(out)
}
