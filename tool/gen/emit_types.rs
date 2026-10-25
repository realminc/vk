//! Struct/union emission from complete declarations. No guessed integer widths.
use crate::{
	emit_pfn::{escape_name, type_str},
	model::{Registry, StructMember, TypeDecl, TypeKind},
	names,
};
use anyhow::{Result, ensure};
use std::{collections::HashSet, fmt::Write};

pub fn emit_types(reg: &Registry) -> Result<String> {
	let mut out = String::from(
		"// @generated — do not edit by hand.\n#![allow(non_camel_case_types, unused_imports)]\nuse super::{enums::*, constants::*, pfn::*};\nuse std::ffi::c_void;\n",
	);
	let mut seen: HashSet<String> = reg.enums.iter().map(|e| e.c_name.clone()).collect();
	for ty in &reg.types {
		if !seen.insert(ty.c_name.clone()) {
			continue;
		}
		emit_type(&mut out, ty)?;
	}
	Ok(out)
}

fn emit_type(out: &mut String, ty: &TypeDecl) -> Result<()> {
	let name = names::type_name(&ty.c_name);
	let guard = crate::protect::cfg(&ty.guards);
	if matches!(&ty.kind, TypeKind::ExternalType { requires: Some(requires) } if matches!(requires.as_str(), "vk_platform" | "stdint"))
	{
		return Ok(());
	}
	if !matches!(
		ty.kind,
		TypeKind::Struct { .. }
			| TypeKind::Union { .. }
			| TypeKind::Enum
			| TypeKind::DispatchableHandle
			| TypeKind::NonDispatchableHandle
			| TypeKind::FuncPointer { .. }
			| TypeKind::Bitmask { requires: None, .. }
	) {
		out.push_str(&guard);
		crate::doc::item(out, &ty.c_name, &name, "Native type", ty.from_video);
	}
	match &ty.kind {
		TypeKind::Alias { target } => writeln!(out, "pub type {name} = {};", names::type_name(target))?,
		TypeKind::Bitmask { base, requires } => {
			if let Some(bits) = requires {
				writeln!(out, "pub type {name} = {};", names::type_name(bits))?;
			} else {
				let backing = match base.as_str() {
					"VkFlags" => "u32",
					"VkFlags64" => "u64",
					_ => anyhow::bail!("unsupported flag storage {base}"),
				};
				crate::emit_enums::emit_enum(
					out,
					&crate::model::EnumDecl {
						c_name: ty.c_name.clone(),
						backing: backing.into(),
						values: vec![],
						guards: ty.guards.clone(),
					},
				)?;
			}
		}
		TypeKind::BaseType { ty } => writeln!(out, "pub type {name} = {};", type_str(ty))?,
		TypeKind::DispatchableHandle | TypeKind::NonDispatchableHandle => {
			crate::emit_handles::emit_handle(out, ty);
		}
		TypeKind::Enum | TypeKind::FuncPointer { .. } => {}
		TypeKind::ExternalType { requires } => emit_external(out, &name, requires.as_deref())?,
		TypeKind::Struct { members, .. } => emit_record(out, ty, members, false, &guard)?,
		TypeKind::Union { members } => emit_record(out, ty, members, true, &guard)?,
	}
	Ok(())
}

fn emit_external(out: &mut String, name: &str, requires: Option<&str>) -> Result<()> {
	if requires.is_some_and(|r| matches!(r, "vk_platform" | "stdint")) {
		return Ok(());
	}
	let alias = match name {
		"VisualID" | "Window" | "RROutput" => Some("std::ffi::c_ulong"),
		"DWORD" | "xcb_visualid_t" | "xcb_window_t" | "zx_handle_t" => Some("u32"),
		"GgpStreamDescriptor" => Some("u32"),
		"GgpFrameToken" => Some("u64"),
		"HINSTANCE" | "HWND" | "HMONITOR" | "HANDLE" | "NvSciSyncAttrList" | "NvSciSyncObj"
		| "NvSciBufAttrList" | "NvSciBufObj" => Some("*mut std::ffi::c_void"),
		"LPCWSTR" => Some("*const u16"),
		_ => None,
	};
	if let Some(alias) = alias {
		writeln!(out, "pub type {name} = {alias};")?;
	} else {
		// These are forward declarations, never substitutes for defined C records.
		// Their private zero-sized body cannot be constructed by consumers.
		writeln!(
			out,
			"#[repr(C)]\npub struct {name} {{ _private: [u8;0], _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)> }}"
		)?;
	}
	Ok(())
}

fn emit_record(
	out: &mut String,
	ty: &TypeDecl,
	members: &[StructMember],
	union: bool,
	guard: &str,
) -> Result<()> {
	let name = names::type_name(&ty.c_name);
	ensure!(!members.is_empty(), "defined record {name} has no members");
	out.push_str(guard);
	crate::doc::item(
		out,
		&ty.c_name,
		&name,
		if union {
			"Native union"
		} else {
			"Native structure"
		},
		ty.from_video,
	);
	writeln!(
		out,
		"#[repr(C)]\n#[derive(Copy, Clone)]\npub {} {name} {{",
		if union { "union" } else { "struct" }
	)?;
	let mut bitfield_group = 0;
	let mut bits = 0u32;
	let mut accessors = String::new();
	let mut seen = HashSet::new();
	for member in members {
		ensure!(
			seen.insert(names::member_name(&member.name)),
			"duplicate member {name}.{}",
			member.name
		);
		let field = escape_name(&member.name);
		if let Some(width) = member.bitfield {
			ensure!(
				!union && width > 0 && width <= 32,
				"unsupported bitfield {name}.{field}"
			);
			ensure!(
				member.ty.name == "uint32_t"
					|| member.ty.name.starts_with("Vk")
					|| member.ty.name.starts_with("StdVideo"),
				"unsupported bitfield base {}",
				member.ty.name
			);
			if bits == 0 || bits + width > 32 {
				bitfield_group += 1;
				bits = 0;
				writeln!(out, "pub _bitfield_{bitfield_group}: u32,")?;
			}
			let mask = if width == 32 {
				u32::MAX
			} else {
				(1u32 << width) - 1
			};
			let read = if bits == 0 {
				format!("self._bitfield_{bitfield_group}")
			} else {
				format!("(self._bitfield_{bitfield_group} >> {bits})")
			};
			let write = if bits == 0 {
				format!("(value & {mask})")
			} else {
				format!("((value & {mask}) << {bits})")
			};
			let clear = if bits == 0 {
				format!("{mask}u32")
			} else {
				format!("({mask}u32 << {bits})")
			};
			crate::doc::alias(&mut accessors, &member.name, &field);
			writeln!(
				accessors,
				"/// Read native bitfield `{}`.\npub const fn {field}(&self) -> u32 {{ {read} & {mask} }}\n/// Set native bitfield `{}`, retaining its low {width} bits.\npub fn set_{field}(&mut self, value: u32) {{ self._bitfield_{bitfield_group} = (self._bitfield_{bitfield_group} & !{clear}) | {write}; }}",
				member.name, member.name
			)?;
			bits += width;
			if bits == 32 {
				bits = 0;
			}
		} else {
			bits = 0;
			writeln!(out, "/// Native field `{}`.", member.name)?;
			crate::doc::alias(out, &member.name, &field);
			writeln!(out, "pub {field}: {},", type_str(&member.ty))?;
		}
	}
	writeln!(out, "}}")?;
	if !accessors.is_empty() {
		writeln!(out, "{guard}impl {name} {{ {accessors} }}")?;
	}
	if let Some(stype) = members
		.iter()
		.find(|m| m.name == "sType")
		.and_then(|m| m.values.as_deref())
	{
		let variant = names::enum_variant(stype, "VkStructureType");
		writeln!(
			out,
			"{guard}impl Default for {name} {{ fn default()->Self {{\n// SAFETY: generated fields are integers, floats, raw pointers, nullable\n// callbacks, integer newtypes or recursively composed records of these.\nlet mut value:Self=unsafe {{ std::mem::zeroed() }};\nvalue.s_type=StructureType::{variant}; value }} }}"
		)?;
	}
	Ok(())
}
