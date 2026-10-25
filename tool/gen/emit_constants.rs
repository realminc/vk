//! Typed C constant expressions. Unsupported expressions stop generation.
use crate::{emit_enums::parse_integer, model::Registry, names};
use anyhow::{Result, anyhow, bail, ensure};
use std::{collections::HashMap, fmt::Write};

pub fn emit_constants(reg: &Registry) -> Result<String> {
	let mut out = String::from("// @generated — do not edit by hand.\n");
	let constants: HashMap<_, _> = reg.constants.iter().map(|c| (c.name.as_str(), c)).collect();
	let mut seen = std::collections::HashSet::new();
	for c in &reg.constants {
		if !seen.insert(&c.name) {
			continue;
		}
		let mut target = c;
		let mut visited = std::collections::HashSet::new();
		while let Some(alias) = target.alias_of.as_deref().or_else(|| {
			constants
				.contains_key(target.value.as_str())
				.then_some(target.value.as_str())
		}) {
			ensure!(
				visited.insert(&target.name),
				"constant alias cycle at {}",
				c.name
			);
			target = constants
				.get(alias)
				.ok_or_else(|| anyhow!("constant {} missing alias target {alias}", c.name))?;
		}
		let (ty, value) = constant_value(&target.value, target.ty.as_deref())?;
		out.push_str(&crate::protect::cfg(&c.guards));
		writeln!(
			out,
			"pub const {}: {ty} = {value};",
			names::const_name(&c.name)
		)?;
	}
	Ok(out)
}

fn constant_value(value: &str, declared: Option<&str>) -> Result<(&'static str, String)> {
	let v = value.trim();
	if v.starts_with('"') && v.ends_with('"') {
		// Vulkan extension strings are ASCII; CStr keeps their trailing NUL available.
		ensure!(
			!v[1..v.len() - 1].contains(['\\', '\0']),
			"unsupported string escape: {v}"
		);
		return Ok(("&std::ffi::CStr", format!("c{v}")));
	}
	if let Some(inner) = v.strip_prefix("(~").and_then(|s| s.strip_suffix(')')) {
		let number = parse_integer(inner)?;
		if declared == Some("uint64_t") || inner.to_uppercase().ends_with("ULL") {
			return Ok(("u64", (!u64::try_from(number)?).to_string()));
		}
		return Ok(("u32", (!u32::try_from(number)?).to_string()));
	}
	if v.starts_with("VK_MAKE_API_VERSION(") || v.starts_with("VK_MAKE_VIDEO_STD_VERSION(") {
		let args = v
			.split_once('(')
			.unwrap()
			.1
			.strip_suffix(')')
			.ok_or_else(|| anyhow!("invalid version expression {v}"))?;
		let args: Vec<_> = args.split(',').map(str::trim).collect();
		ensure!(
			args.len() == 3 || args.len() == 4,
			"invalid version expression {v}"
		);
		let mut values = vec![];
		for arg in args {
			if let Ok(number) = arg.parse::<u32>() {
				values.push(number.to_string());
			} else {
				ensure!(
					arg == "VK_HEADER_VERSION",
					"unsupported version argument {arg}"
				);
				values.push("HEADER_VERSION".into());
			}
		}
		if values.len() == 3 {
			values.insert(0, "0".into());
		}
		return Ok((
			"u32",
			format!("crate::core::make_api_version({})", values.join(", ")),
		));
	}
	if declared == Some("float") || v.ends_with(['f', 'F']) && !v.starts_with("0x") {
		let literal = v.trim_end_matches(['f', 'F']);
		let _: f32 = literal.parse()?;
		return Ok(("f32", format!("{literal}_f32")));
	}
	let number = parse_integer(v).map_err(|e| anyhow!("unsupported constant {v}: {e}"))?;
	let ty = match declared {
		Some("uint64_t") => "u64",
		Some("uint32_t") => "u32",
		Some("int32_t") => "i32",
		Some("uint8_t") => "u8",
		Some("uint16_t") => "u16",
		None if v.to_uppercase().ends_with("ULL") => "u64",
		None if number < 0 => "i32",
		None => "u32",
		other => bail!("unsupported constant type {other:?}"),
	};
	let valid = match ty {
		"u8" => u8::try_from(number).is_ok(),
		"u16" => u16::try_from(number).is_ok(),
		"u64" => u64::try_from(number).is_ok(),
		"u32" => u32::try_from(number).is_ok(),
		"i32" => i32::try_from(number).is_ok(),
		_ => false,
	};
	ensure!(valid, "constant {v} out of range for {ty}");
	Ok((ty, number.to_string()))
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_emit_constants.rs"]
mod tests;
