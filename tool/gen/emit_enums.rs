//! Extensible Vulkan integer values. Every bit pattern is valid at the FFI boundary.
use crate::{
	model::{EnumDecl, Registry},
	names,
};
use anyhow::{Result, anyhow, ensure};
use std::{collections::HashMap, fmt::Write};

pub fn emit_enums(reg: &Registry) -> Result<String> {
	let mut out = String::from(
		"// @generated — do not edit by hand.\n#![allow(non_camel_case_types, non_upper_case_globals)]\n",
	);
	for decl in &reg.enums {
		emit_enum(&mut out, decl)?;
	}
	if reg.enums.iter().any(|e| e.c_name == "VkResult") {
		out.push_str("impl Result { pub const fn is_success(self) -> bool { self.0 >= 0 } pub const fn is_err(self) -> bool { self.0 < 0 } }\n");
	}
	Ok(out)
}
pub(crate) fn emit_enum(out: &mut String, decl: &EnumDecl) -> Result<()> {
	let name = names::type_name(&decl.c_name);
	let backing = &decl.backing;
	let guard = crate::protect::cfg(&decl.guards);
	out.push_str(&guard);
	writeln!(out,"#[repr(transparent)]\n#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]\npub struct {name}(pub {backing});\n{guard}impl {name} {{\npub const fn from_raw(value: {backing}) -> Self {{ Self(value) }}\npub const fn as_raw(self) -> {backing} {{ self.0 }}

pub const fn empty() -> Self {{ Self(0) }}")?;
	let values: HashMap<_, _> = decl.values.iter().map(|v| (v.name.as_str(), v)).collect();
	let mut seen = std::collections::HashSet::new();
	for value in &decl.values {
		if !seen.insert(&value.name) {
			continue;
		}
		let mut v = value;
		let mut visited = std::collections::HashSet::new();
		while let Some(target) = &v.alias_of {
			ensure!(visited.insert(&v.name), "enum alias cycle at {}", v.name);
			v = values
				.get(target.as_str())
				.ok_or_else(|| anyhow!("missing enum alias target {target} for {}", value.name))?;
		}
		let number = parse_integer(&v.value)?;
		let valid = match backing.as_str() {
			"i32" => i32::try_from(number).is_ok(),
			"u32" => u32::try_from(number).is_ok(),
			"u64" => u64::try_from(number).is_ok(),
			_ => false,
		};
		ensure!(
			valid,
			"{} value {number} does not fit {backing}",
			value.name
		);
		let constant = names::enum_variant(&value.name, &decl.c_name);
		out.push_str(&crate::protect::cfg(&value.guards));
		writeln!(out, "pub const {constant}: Self = Self({number});")?;
	}
	writeln!(out, "}}")?;
	if backing.starts_with('u') {
		for (trait_name, method, op) in [
			("BitOr", "bitor", "|"),
			("BitAnd", "bitand", "&"),
			("BitXor", "bitxor", "^"),
		] {
			writeln!(
				out,
				"{guard}impl std::ops::{trait_name} for {name} {{ type Output=Self; fn {method}(self,rhs:Self)->Self {{Self(self.0 {op} rhs.0)}} }}"
			)?;
		}
		writeln!(
			out,
			"{guard}impl std::ops::Not for {name} {{ type Output=Self; fn not(self)->Self {{Self(!self.0)}} }}"
		)?;
	}
	Ok(())
}

pub fn parse_integer(s: &str) -> Result<i128> {
	let s = s.trim().trim_end_matches(['U', 'L', 'u', 'l']);
	if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
		Ok(i128::from_str_radix(h, 16)?)
	} else {
		Ok(s.parse()?)
	}
}
