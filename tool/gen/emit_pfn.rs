//! Exact nullable command and callback function-pointer prototypes.
use crate::{
	model::{Registry, TypeRef},
	names,
};
use std::fmt::Write;

pub fn emit_pfn(reg: &Registry) -> String {
	let mut out = String::from(
		"// @generated — do not edit by hand.\n#![allow(non_camel_case_types)]\nuse super::{types::*, enums::*};\n\n",
	);
	for ty in &reg.types {
		if let crate::model::TypeKind::FuncPointer {
			return_type,
			parameters,
		} = &ty.kind
		{
			out.push_str(&crate::protect::cfg(&ty.guards));
			let params: Vec<_> = parameters
				.iter()
				.map(|p| format!("{}: {}", escape_name(&p.name), parameter_type(&p.ty)))
				.collect();
			writeln!(
				out,
				"pub type {} = Option<unsafe extern \"system\" fn({}) -> {}>;\n",
				ty.c_name,
				params.join(", "),
				type_str(return_type)
			)
			.unwrap();
		}
	}
	for cmd in &reg.commands {
		out.push_str(&crate::protect::cfg(&cmd.guards));
		let pfn = format!("PFN_{}", cmd.c_name);
		if let Some(target) = &cmd.alias_of {
			writeln!(out, "pub type {pfn} = PFN_{target};\n").unwrap();
			continue;
		}
		let params: Vec<_> = cmd
			.parameters
			.iter()
			.map(|p| format!("{}: {}", escape_name(&p.name), parameter_type(&p.ty)))
			.collect();
		writeln!(
			out,
			"pub type {pfn} = Option<unsafe extern \"system\" fn({}) -> {}>;\n",
			params.join(", "),
			type_str(&cmd.return_type)
		)
		.unwrap();
	}
	out
}

pub fn escape_name(name: &str) -> String {
	names::member_name(name)
}

pub fn base_type(name: &str) -> String {
	match name {
		"void" => "std::ffi::c_void".into(),
		"char" => "std::ffi::c_char".into(),
		"float" => "f32".into(),
		"double" => "f64".into(),
		"int8_t" => "i8".into(),
		"uint8_t" => "u8".into(),
		"int16_t" => "i16".into(),
		"uint16_t" => "u16".into(),
		"int32_t" | "int" => "i32".into(),
		"uint32_t" => "u32".into(),
		"int64_t" => "i64".into(),
		"uint64_t" => "u64".into(),
		"size_t" => "usize".into(),
		other => names::type_name(other),
	}
}

pub fn type_str(tr: &TypeRef) -> String {
	type_str_with(tr, false)
}

fn type_str_with(tr: &TypeRef, qualified: bool) -> String {
	if tr.name == "void" && tr.pointers.is_empty() && tr.arrays.is_empty() {
		return "()".into();
	}
	let mut s = base_type(&tr.name);
	if qualified
		&& !s.starts_with("std::")
		&& !matches!(
			s.as_str(),
			"u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | "f32" | "f64"
		) {
		s = format!("vk::{s}");
	}
	for constant in &tr.pointers {
		s = format!("*{} {s}", if *constant { "const" } else { "mut" });
	}
	for extent in tr.arrays.iter().rev() {
		let extent = names::const_name(extent);
		let extent = if extent.parse::<usize>().is_ok() {
			extent.to_string()
		} else {
			format!("{}{extent} as usize", if qualified { "vk::" } else { "" })
		};
		s = format!("[{s}; {extent}]");
	}
	s
}

pub fn qualified_type(tr: &TypeRef) -> String {
	type_str_with(tr, true)
}

pub fn qualified_parameter_type(tr: &TypeRef) -> String {
	parameter_type_with(tr, true)
}

pub fn parameter_type(tr: &TypeRef) -> String {
	parameter_type_with(tr, false)
}

fn parameter_type_with(tr: &TypeRef, qualified: bool) -> String {
	if tr.arrays.is_empty() {
		return type_str_with(tr, qualified);
	}
	let mut inner = tr.clone();
	inner.arrays.remove(0);
	// Array qualifiers belong to the array element in C parameter declarations.
	format!(
		"*{} {}",
		if tr.array_element_const {
			"const"
		} else {
			"mut"
		},
		type_str_with(&inner, qualified)
	)
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_emit_pfn.rs"]
mod tests;
