//! Registry-owned borrowed record construction. No runtime registry or pointer walking.
use crate::{
	emit_pfn::type_str_for,
	model::{Registry, StructMember, TypeDecl, TypeKind},
	names, protect,
};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, fmt::Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Setter {
	Value,
	String,
	Reference,
	Slice { count: String },
	DescriptorPayload,
}

fn members(ty: &TypeDecl) -> &[StructMember] {
	match &ty.kind {
		TypeKind::Struct { members, .. } | TypeKind::Union { members } => members,
		_ => &[],
	}
}

pub fn tagged(ty: &TypeDecl) -> bool {
	let fields = members(ty);
	matches!(ty.kind, TypeKind::Struct { .. })
		&& fields
			.first()
			.is_some_and(|m| m.name == "sType" && m.values.is_some())
		&& fields
			.get(1)
			.is_some_and(|m| m.name == "pNext" && m.ty.name == "void" && m.ty.pointers.len() == 1)
}

fn count_name(member: &StructMember) -> Option<&str> {
	let len = member.len.as_deref()?;
	let count = len.strip_suffix(",null-terminated").unwrap_or(len);
	(!count.is_empty() && count.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
		.then_some(count)
}

fn record(reg: &Registry, name: &str) -> bool {
	let mut current = name;
	for _ in 0..reg.types.len() {
		let Some(ty) = reg.types.iter().find(|ty| ty.c_name == current) else {
			return false;
		};
		match &ty.kind {
			TypeKind::Struct { .. } | TypeKind::Union { .. } => return true,
			TypeKind::Alias { target } => current = target,
			_ => return false,
		}
	}
	false
}

pub fn setter(reg: &Registry, ty: &TypeDecl, member: &StructMember) -> Option<Setter> {
	if !matches!(ty.kind, TypeKind::Struct { .. })
		|| member.bitfield.is_some()
		|| matches!(member.name.as_str(), "sType" | "pNext")
	{
		return None;
	}
	if member.ty.pointers.is_empty() {
		// Native count fields stay raw; a borrowed slice sets its own count.
		return (!members(ty)
			.iter()
			.any(|m| count_name(m) == Some(member.name.as_str())))
		.then_some(Setter::Value);
	}
	if member.ty.name == "char"
		&& member.ty.pointers == [true]
		&& member.len.as_deref() == Some("null-terminated")
		&& member.ty.arrays.is_empty()
	{
		return Some(Setter::String);
	}
	if !member.ty.arrays.is_empty() || member.ty.name == "void" {
		return None;
	}
	if let Some(count) = count_name(member) {
		let count_field = members(ty).iter().find(|m| m.name == count)?;
		if !count_field.ty.pointers.is_empty()
			|| !count_field.ty.arrays.is_empty()
			|| !matches!(count_field.ty.name.as_str(), "uint32_t" | "size_t")
		{
			return None;
		}
		// Shared counts need a reviewed grouped/alternative recipe, never guessed setters.
		if members(ty)
			.iter()
			.filter(|m| count_name(m) == Some(count))
			.count()
			!= 1
		{
			if ty.c_name == "VkWriteDescriptorSet"
				&& count == "descriptorCount"
				&& matches!(
					member.name.as_str(),
					"pImageInfo" | "pBufferInfo" | "pTexelBufferView"
				) && member.ty.pointers == [true]
			{
				return Some(Setter::DescriptorPayload);
			}
			return None;
		}
		if member.ty.pointers == [true]
			|| (member.ty.name == "char" && member.ty.pointers == [true, true])
		{
			return Some(Setter::Slice {
				count: count.into(),
			});
		}
	}
	if member.len.is_none() && member.ty.pointers == [true] && record(reg, &member.ty.name) {
		return Some(Setter::Reference);
	}
	None
}

/// Fixed-point propagation keeps aliases and every containing/pointer record honest.
pub fn resolve(reg: &mut Registry) -> Result<()> {
	reg.borrowed.clear();
	reg.exclusive.clear();
	for ty in &reg.types {
		if tagged(ty) {
			reg.borrowed.insert(ty.c_name.clone());
			reg.exclusive.insert(ty.c_name.clone());
		}
		if members(ty).iter().any(|m| {
			matches!(
				setter(reg, ty, m),
				Some(Setter::String | Setter::Reference | Setter::Slice { .. } | Setter::DescriptorPayload)
			)
		}) {
			reg.borrowed.insert(ty.c_name.clone());
		}
	}
	loop {
		let previous = (reg.borrowed.len(), reg.exclusive.len());
		for ty in &reg.types {
			let alias = match &ty.kind {
				TypeKind::Alias { target } => Some(target),
				_ => None,
			};
			if alias.is_some_and(|t| reg.borrowed.contains(t))
				|| members(ty)
					.iter()
					.any(|m| reg.borrowed.contains(&m.ty.name))
			{
				reg.borrowed.insert(ty.c_name.clone());
			}
			if alias.is_some_and(|t| reg.exclusive.contains(t))
				|| members(ty)
					.iter()
					.any(|m| m.ty.pointers.is_empty() && reg.exclusive.contains(&m.ty.name))
			{
				reg.exclusive.insert(ty.c_name.clone());
			}
		}
		if previous == (reg.borrowed.len(), reg.exclusive.len()) {
			break;
		}
	}
	for ty in &reg.types {
		if let TypeKind::Struct { extends, .. } = &ty.kind {
			for base in extends {
				ensure!(
					reg.types.iter().any(|t| t.c_name == *base && tagged(t)),
					"unknown chain root {base} for {}",
					ty.c_name
				);
			}
		}
	}
	Ok(())
}

pub fn declaration(reg: &Registry, ty: &TypeDecl, lifetime: &str) -> String {
	let name = names::type_name(&ty.c_name);
	if reg.borrowed.contains(&ty.c_name) {
		format!("{name}<{lifetime}>")
	} else {
		name
	}
}

pub fn emit_setters(out: &mut String, reg: &Registry, ty: &TypeDecl) -> Result<()> {
	if !matches!(ty.kind, TypeKind::Struct { .. }) {
		return Ok(());
	}
	let fields = members(ty);
	let guard = protect::cfg(&ty.guards);
	let generic = if reg.borrowed.contains(&ty.c_name) {
		"<'a>"
	} else {
		""
	};
	writeln!(
		out,
		"{guard}impl{generic} {} {{",
		declaration(reg, ty, "'a")
	)?;
	for member in fields {
		if emit_group(out, reg, ty, member)? {
			continue;
		}
		let Some(kind) = setter(reg, ty, member) else {
			continue;
		};
		let field = names::member_name(&member.name);
		let method = if member.ty.pointers.is_empty() {
			field.clone()
		} else {
			field
				.strip_prefix("pp_")
				.or_else(|| field.strip_prefix("p_"))
				.unwrap_or(&field)
				.to_owned()
		};
		// Scalars may themselves contain non-Copy, borrowed union members.
		let native = type_str_for(&member.ty, reg, "'a", false);
		let (argument, body, fallible) = match kind {
			Setter::Value => (native, format!("self.{field} = value;"), false),
			Setter::String => (
				"&'a std::ffi::CStr".into(),
				format!("self.{field} = value.as_ptr();"),
				false,
			),
			Setter::Reference => {
				let mut target = member.ty.clone();
				target.pointers.clear();
				(
					format!("&'a {}", type_str_for(&target, reg, "'a", false)),
					format!("self.{field} = value;"),
					false,
				)
			}
			kind @ (Setter::Slice { .. } | Setter::DescriptorPayload) => {
				let descriptor = kind == Setter::DescriptorPayload;
				let count = if descriptor {
					"descriptorCount".to_owned()
				} else {
					let Setter::Slice { count } = kind else {
						unreachable!()
					};
					count
				};
				let mut target = member.ty.clone();
				target.pointers.pop();
				let target = type_str_for(&target, reg, "'a", false);
				let counter = fields.iter().find(|m| m.name == count).unwrap();
				let conversion = if counter.ty.name == "size_t" {
					"value.len()".into()
				} else {
					format!(
						"crate::construction::count({:?}, value.len())?",
						member.name
					)
				};
				writeln!(
					out,
					"/// Set `{}` and its checked native count `{count}`.\n///\n/// # Errors\n/// Returns [`crate::ConstructionError::CountOverflow`] if the length cannot fit.\n/// The borrow covers the array; any nested raw pointers remain caller obligations.",
					member.name
				)?;
				let clear = if descriptor {
					"self.p_image_info = std::ptr::null(); self.p_buffer_info = std::ptr::null(); self.p_texel_buffer_view = std::ptr::null();"
				} else {
					""
				};
				(
					format!("&'a [{target}]"),
					format!(
						"let count = {conversion}; {clear} self.{} = count; self.{field} = value.as_ptr();",
						names::member_name(&count)
					),
					true,
				)
			}
		};
		if !fallible {
			writeln!(
				out,
				"/// Set native field `{}` without native effects.",
				member.name
			)?;
		}
		crate::doc::alias(out, &member.name, &method);
		let result = if fallible {
			"std::result::Result<Self, crate::ConstructionError>"
		} else {
			"Self"
		};
		let ret = if fallible { "Ok(self)" } else { "self" };
		let must_use = if fallible { "" } else { "#[must_use]\n" };
		writeln!(
			out,
			"#[inline]\n{must_use}pub fn {method}(mut self, value: {argument}) -> {result} {{ {body} {ret} }}"
		)?;
	}
	if tagged(ty) {
		writeln!(
			out,
			"/// Prepend one registry-compatible node, retaining its exclusive borrow.\n///\n/// The node must have a null `p_next` and both headers must have canonical tags.\n/// Existing links are never traversed and remain written after the borrow ends.\n/// Type compatibility does not establish enabled extensions or Vulkan valid usage.\n///\n/// # Errors\n/// Returns a construction error for a changed structure tag or already-linked node.\n#[inline]\npub fn push_next<T: crate::Extends<Self>>(mut self, next: &'a mut T) -> std::result::Result<Self, crate::ConstructionError> {{\ncrate::construction::prepend(&mut self, next)?; Ok(self)\n}}"
		)?;
	}
	writeln!(out, "}}")?;
	Ok(())
}

/// Reviewed native parallel arrays. Optional/alternative groups remain raw.
fn emit_group(
	out: &mut String,
	reg: &Registry,
	ty: &TypeDecl,
	member: &StructMember,
) -> Result<bool> {
	let recipe = match (ty.c_name.as_str(), member.name.as_str()) {
		("VkSubmitInfo", "pWaitSemaphores") => Some((
			"waitSemaphoreCount",
			["pWaitSemaphores", "pWaitDstStageMask"],
			["VkSemaphore", "VkPipelineStageFlags"],
		)),
		("VkSemaphoreWaitInfo", "pSemaphores") => Some((
			"semaphoreCount",
			["pSemaphores", "pValues"],
			["VkSemaphore", "uint64_t"],
		)),
		_ => None,
	};
	let Some((count, names, types)) = recipe else {
		return Ok(false);
	};
	let mut parameters = Vec::new();
	let mut assignments = String::new();
	let mut arguments = Vec::new();
	for (name, expected) in names.into_iter().zip(types) {
		let field = members(ty)
			.iter()
			.find(|m| m.name == name)
			.ok_or_else(|| anyhow::anyhow!("missing grouped field {}.{name}", ty.c_name))?;
		ensure!(
			field.ty.name == expected
				&& field.ty.pointers == [true]
				&& field.ty.arrays.is_empty()
				&& field.len.as_deref() == Some(count),
			"changed grouped field {}.{name}",
			ty.c_name
		);
		let native = crate::names::member_name(name);
		let argument = native.strip_prefix("p_").unwrap();
		let mut target = field.ty.clone();
		target.pointers.clear();
		parameters.push(format!(
			"{argument}: &'a [{}]",
			type_str_for(&target, reg, "'a", false)
		));
		writeln!(assignments, "self.{native} = {argument}.as_ptr();")?;
		arguments.push(argument.to_owned());
	}
	let count_field = members(ty).iter().find(|m| m.name == count).unwrap();
	ensure!(
		count_field.ty.name == "uint32_t"
			&& count_field.ty.pointers.is_empty()
			&& count_field.ty.arrays.is_empty(),
		"changed grouped count {}.{count}",
		ty.c_name
	);
	let first = &arguments[0];
	let second = &arguments[1];
	writeln!(
		out,
		"/// Borrow parallel native arrays and set their shared checked count.\n///\n/// # Errors\n/// Returns a construction error for unequal lengths or native count overflow.\n#[inline]\npub fn {first}(mut self, {}) -> std::result::Result<Self, crate::ConstructionError> {{\nif {first}.len() != {second}.len() {{ return Err(crate::ConstructionError::LengthMismatch {{ field: {:?}, expected: {first}.len(), actual: {second}.len() }}); }}\nlet count = crate::construction::count({:?}, {first}.len())?;\n{assignments}self.{} = count; Ok(self)\n}}",
		parameters.join(", "),
		names[1],
		names[0],
		crate::names::member_name(count)
	)?;
	Ok(true)
}

pub fn emit_chains(reg: &Registry) -> Result<String> {
	let mut out = String::from(
		"// @generated by vk-gen — sealed native chain relationships.\nuse crate as vk;\n\n",
	);
	for ty in reg.types.iter().filter(|t| tagged(t)) {
		let guard = protect::cfg(&ty.guards);
		let name = declaration(reg, ty, "'a");
		let fields = members(ty);
		let stype = names::enum_variant(fields[0].values.as_deref().unwrap(), "VkStructureType");
		let read = if fields[1].ty.pointers == [true] {
			"self.p_next"
		} else {
			"self.p_next.cast_const()"
		};
		let pointer = if fields[1].ty.pointers == [true] {
			"p_next.cast_const()"
		} else {
			"p_next"
		};
		writeln!(
			out,
			"{guard}impl<'a> crate::construction::sealed::ChainNode for vk::{name} {{\nconst S_TYPE: vk::StructureType = vk::StructureType::{stype};\n\nfn s_type(&self) -> vk::StructureType {{ self.s_type }}\nfn p_next(&self) -> *const std::ffi::c_void {{ {read} }}\nfn set_p_next(&mut self, p_next: *mut std::ffi::c_void) {{ self.p_next = {pointer}; }}\nfn address(&mut self) -> *mut std::ffi::c_void {{ std::ptr::from_mut(self).cast() }}\n}}"
		)?;
		if let TypeKind::Struct { extends, .. } = &ty.kind {
			for base in extends {
				let base_ty = reg.types.iter().find(|t| t.c_name == *base).unwrap();
				let guards = if ty.guards == base_ty.guards {
					guard.clone()
				} else {
					format!("{guard}{}", protect::cfg(&base_ty.guards))
				};
				writeln!(
					out,
					"{guards}impl<'a, 'root> crate::Extends<vk::{}> for vk::{name} {{}}",
					declaration(reg, base_ty, "'root")
				)?;
			}
		}
	}
	Ok(out)
}

/// Public signature/default contracts; independent borrow/ABI proofs live elsewhere.
pub fn emit_tests(reg: &Registry) -> Result<String> {
	let mut out = String::from(
		"// @generated by vk-gen — public construction signatures and native headers.\n// Independent compile-fail, behavior and C ABI tests remain separate.\nuse ::vk;\n\n",
	);
	for ty in reg
		.types
		.iter()
		.filter(|ty| matches!(ty.kind, TypeKind::Struct { .. }))
	{
		let guard = protect::cfg(&ty.guards);
		let name = declaration(reg, ty, "'static");
		let test = names::member_name(&ty.c_name);
		writeln!(out, "{guard}#[test]\nfn {test}_construction_contract() {{")?;
		if tagged(ty) {
			let tag = names::enum_variant(members(ty)[0].values.as_deref().unwrap(), "VkStructureType");
			writeln!(
				out,
				"let value = vk::{}::default();\nassert_eq!(value.s_type, vk::StructureType::{tag});\nassert!(value.p_next.is_null());",
				names::type_name(&ty.c_name)
			)?;
		}
		for field in members(ty) {
			let Some(setter) = setter(reg, ty, field) else {
				continue;
			};
			let native = names::member_name(&field.name);
			let method = if field.ty.pointers.is_empty() {
				&native
			} else {
				native
					.strip_prefix("pp_")
					.or_else(|| native.strip_prefix("p_"))
					.unwrap_or(&native)
			};
			let mut target = field.ty.clone();
			let (argument, fallible) = match setter {
				Setter::Value => (type_str_for(&target, reg, "'static", true), false),
				Setter::String => ("&'static std::ffi::CStr".into(), false),
				Setter::Reference => {
					target.pointers.clear();
					(
						format!("&'static {}", type_str_for(&target, reg, "'static", true)),
						false,
					)
				}
				Setter::Slice { .. } | Setter::DescriptorPayload => {
					target.pointers.pop();
					(
						format!("&'static [{}]", type_str_for(&target, reg, "'static", true)),
						true,
					)
				}
			};
			let result = if fallible {
				format!("std::result::Result<vk::{name}, vk::ConstructionError>")
			} else {
				format!("vk::{name}")
			};
			writeln!(
				out,
				"let _: fn(vk::{name}, {argument}) -> {result} = vk::{}::{method};",
				names::type_name(&ty.c_name)
			)?;
		}
		writeln!(out, "}}")?;
	}
	Ok(out)
}

/// Diagnostic field-setter counts keyed by native type name.
/// Emission uses registry vectors directly to preserve declaration order.
pub fn inventory(reg: &Registry) -> BTreeMap<String, usize> {
	reg
		.types
		.iter()
		.filter(|ty| matches!(ty.kind, TypeKind::Struct { .. }))
		.map(|ty| {
			(
				ty.c_name.clone(),
				members(ty)
					.iter()
					.filter(|m| setter(reg, ty, m).is_some())
					.count(),
			)
		})
		.collect()
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_construction.rs"]
mod tests;
