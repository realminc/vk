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
	NativeCount,
	String,
	Reference,
	Slice { count: String },
	DescriptorPayload,
	Bytes { count: String },
	Words,
	SampleMask,
	VersionData,
	ExternalSlice,
	Raw,
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
	let count = member.len.as_deref()?.split(',').next()?;
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

fn sized(reg: &Registry, name: &str) -> bool {
	record(reg, name)
		|| matches!(
			name,
			"char"
				| "uint8_t"
				| "uint16_t"
				| "uint32_t"
				| "uint64_t"
				| "int8_t"
				| "int16_t"
				| "int32_t"
				| "int64_t"
				| "size_t"
				| "float"
				| "double"
		) || reg.types.iter().any(|ty| {
		ty.c_name == name
			&& matches!(
				ty.kind,
				TypeKind::Enum
					| TypeKind::Bitmask { .. }
					| TypeKind::DispatchableHandle
					| TypeKind::NonDispatchableHandle
					| TypeKind::BaseType { .. }
			)
	})
}

fn count_field<'a>(ty: &'a TypeDecl, count: &str) -> Option<&'a StructMember> {
	members(ty).iter().find(|m| {
		m.name == count
			&& m.ty.pointers.is_empty()
			&& m.ty.arrays.is_empty()
			&& matches!(
				m.ty.name.as_str(),
				"uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" | "size_t" | "VkDeviceSize"
			)
	})
}

fn count_expr(reg: &Registry, ty: &TypeDecl, count: &str, field: &str, length: &str) -> String {
	let counter = count_field(ty, count).unwrap();
	if counter.ty.name == "size_t" {
		return length.into();
	}
	if counter.ty.name == "uint32_t" {
		return format!("crate::construction::count({field:?}, {length})?");
	}
	format!(
		"crate::construction::count_as::<{}>({field:?}, {length})?",
		type_str_for(&counter.ty, reg, "'a", false)
	)
}

fn indirect_alternative(ty: &TypeDecl, member: &StructMember) -> bool {
	matches!(
		ty.c_name.as_str(),
		"VkAccelerationStructureBuildGeometryInfoKHR"
			| "VkMicromapBuildInfoEXT"
			| "VkAccelerationStructureGeometryMicromapDataKHR"
			| "VkAccelerationStructureTrianglesOpacityMicromapEXT"
			| "VkAccelerationStructureTrianglesDisplacementMicromapNV"
	) && count_name(member).is_some_and(|count| matches!(count, "geometryCount" | "usageCountsCount"))
}

/// Each native parallel group is reviewed explicitly. XML still owns its fields,
/// qualifiers, optionality and declaration order; newly shared counts fail closed.
fn grouped(ty: &TypeDecl, count: &str) -> bool {
	matches!(
		(ty.c_name.as_str(), count),
		("VkPipelineBinaryKeysAndDataKHR", "binaryCount")
			| (
				"VkSubpassDescription" | "VkSubpassDescription2",
				"colorAttachmentCount"
			) | ("VkSubmitInfo", "waitSemaphoreCount")
			| ("VkPresentInfoKHR", "swapchainCount")
			| (
				"VkWin32KeyedMutexAcquireReleaseInfoNV" | "VkWin32KeyedMutexAcquireReleaseInfoKHR",
				"acquireCount" | "releaseCount"
			) | ("VkIndirectCommandsLayoutTokenNV", "indexTypeCount")
			| ("VkSwapchainTimeDomainPropertiesEXT", "timeDomainCount")
			| ("VkSemaphoreWaitInfo", "semaphoreCount")
			| ("VkIndirectExecutionSetShaderInfoEXT", "shaderCount")
			| ("VkVideoDecodeAV1PictureInfoKHR", "tileCount")
			| ("VkSetDescriptorBufferOffsetsInfoEXT", "setCount")
			| (
				"VkTensorDescriptionARM" | "VkTensorCopyARM",
				"dimensionCount"
			) | ("StdVideoAV1TileInfo", "TileCols" | "TileRows")
	)
}

pub fn setter(reg: &Registry, ty: &TypeDecl, member: &StructMember) -> Option<Setter> {
	if member.bitfield.is_some() || (tagged(ty) && matches!(member.name.as_str(), "sType" | "pNext"))
	{
		return None;
	}
	if member.ty.pointers.is_empty() {
		return Some(
			if members(ty)
				.iter()
				.any(|m| count_name(m) == Some(member.name.as_str()))
			{
				Setter::NativeCount
			} else {
				Setter::Value
			},
		);
	}
	if member.ty.arrays.is_empty() {
		if ty.c_name == "StdVideoH265HrdParameters"
			&& member.len.as_deref() == Some("*_max_sub_layers_minus1 + 1")
		{
			return Some(Setter::ExternalSlice);
		}
		if ty.c_name == "VkCommandBufferInheritanceViewportScissorInfoNV"
			&& member.name == "pViewportDepths"
		{
			return Some(Setter::Slice {
				count: "viewportDepthCount".into(),
			});
		}
		if member.ty.name == "char"
			&& member.ty.pointers == [true]
			&& member.len.as_deref() == Some("null-terminated")
		{
			return Some(Setter::String);
		}
		match (
			ty.c_name.as_str(),
			member.name.as_str(),
			member.alt_len.as_deref(),
		) {
			("VkShaderModuleCreateInfo", "pCode", Some("codeSize / 4")) => return Some(Setter::Words),
			(
				"VkPipelineMultisampleStateCreateInfo",
				"pSampleMask",
				Some("(rasterizationSamples + 31) / 32"),
			) => return Some(Setter::SampleMask),
			(
				"VkAccelerationStructureVersionInfoKHR" | "VkMicromapVersionInfoEXT",
				"pVersionData",
				Some("2*VK_UUID_SIZE"),
			) => return Some(Setter::VersionData),
			_ => {}
		}
		if let Some(count) = count_name(member).filter(|count| count_field(ty, count).is_some()) {
			let peers = members(ty)
				.iter()
				.filter(|m| count_name(m) == Some(count))
				.count();
			if peers > 1 && !indirect_alternative(ty, member) {
				if ty.c_name == "VkWriteDescriptorSet"
					&& peers == 3
					&& member.ty.pointers == [true]
					&& matches!(
						member.name.as_str(),
						"pImageInfo" | "pBufferInfo" | "pTexelBufferView"
					) {
					return Some(Setter::DescriptorPayload);
				}
				return None;
			}
			if member.ty.name == "void" && member.ty.pointers.len() == 1 {
				return Some(
					if !matches!(
						count,
						"dataSize"
							| "initialDataSize"
							| "tagSize"
							| "dynamicMetadataSize"
							| "vendorBinarySize"
							| "codeSize"
							| "size"
							| "messageDataSize"
					) {
						Setter::Raw
					} else {
						Setter::Bytes {
							count: count.into(),
						}
					},
				);
			}
			return Some(Setter::Slice {
				count: count.into(),
			});
		}
		// These output arrays are sized by a separate VkDeviceFaultCountsEXT.
		// A one-element borrow would misrepresent their extent.
		if member.len.is_none()
			&& member.ty.pointers.len() == 1
			&& sized(reg, &member.ty.name)
			&& ty.c_name != "VkDeviceFaultInfoEXT"
			&& !(ty.c_name == "VkCommandBufferInheritanceViewportScissorInfoNV"
				&& member.name == "pViewportDepths")
		{
			return Some(Setter::Reference);
		}
	}
	Some(Setter::Raw)
}

fn mutable(member: &StructMember) -> bool {
	member.ty.pointers.last() == Some(&false)
}

fn method(member: &StructMember) -> String {
	let name = names::member_name(&member.name);
	if matches!(member.name.as_str(), "sType" | "pNext") {
		return name;
	}
	let base = name
		.strip_prefix("pp_")
		.or_else(|| name.strip_prefix("p_"))
		.unwrap_or(&name);
	if member.ty.pointers.len() > 1 && member.len.as_deref().is_some_and(|s| s.ends_with(",1")) {
		name
	} else {
		base.to_owned()
	}
}

fn retains_borrow(kind: &Setter) -> bool {
	!matches!(kind, Setter::Value | Setter::NativeCount | Setter::Raw)
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
		for m in members(ty) {
			let kind = setter(reg, ty, m);
			let group = count_name(m).is_some_and(|c| grouped(ty, c));
			if kind.as_ref().is_some_and(retains_borrow) || group {
				reg.borrowed.insert(ty.c_name.clone());
				if mutable(m) {
					reg.exclusive.insert(ty.c_name.clone());
				}
			}
		}
	}
	loop {
		let previous = (reg.borrowed.len(), reg.exclusive.len());
		for ty in &reg.types {
			let alias = if let TypeKind::Alias { target } = &ty.kind {
				Some(target)
			} else {
				None
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
		for m in members(ty) {
			let kind = setter(reg, ty, m);
			let expected = match kind {
				Some(Setter::Words) => Some("uint32_t"),
				Some(Setter::SampleMask) => Some("VkSampleMask"),
				Some(Setter::VersionData) => Some("uint8_t"),
				Some(Setter::ExternalSlice) => Some("StdVideoH265SubLayerHrdParameters"),
				_ => None,
			};
			if let Some(expected) = expected {
				ensure!(
					m.ty.name == expected && m.ty.pointers == [true] && m.ty.arrays.is_empty(),
					"changed construction recipe {}.{}",
					ty.c_name,
					m.name
				);
			}
			if kind == Some(Setter::Words) {
				ensure!(
					count_field(ty, "codeSize").is_some_and(|field| field.ty.name == "size_t"),
					"changed SPIR-V byte count {}",
					ty.c_name
				);
			}
			if indirect_alternative(ty, m) {
				let peers: Vec<_> = members(ty)
					.iter()
					.filter(|p| count_name(p) == count_name(m))
					.collect();
				ensure!(
					peers.len() == 2
						&& peers[0].ty.pointers == [true]
						&& peers[1].ty.pointers == [true, true]
						&& peers[0].ty.name == peers[1].ty.name,
					"changed indirect alternative {}.{}",
					ty.c_name,
					m.name
				);
			}
			if setter(reg, ty, m) == Some(Setter::Raw) && m.len.is_some() {
				ensure!(
					ty.c_name == "VkLayerSettingEXT" && m.name == "pValues",
					"unreviewed pointer extent {}.{}: {:?}",
					ty.c_name,
					m.name,
					m.len
				);
			}
			if let Some(count) = count_name(m).filter(|c| count_field(ty, c).is_some()) {
				ensure!(
					setter(reg, ty, m).is_some() || grouped(ty, count),
					"unreviewed shared count {}.{count}",
					ty.c_name
				);
			}
		}
		if matches!(ty.kind, TypeKind::Struct { .. } | TypeKind::Union { .. }) {
			let _ = recipes(reg, ty, "'a", false)?;
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

struct Recipe {
	method: String,
	parameters: Vec<(String, String)>,
	body: String,
	doc: String,
	fallible: bool,
	raw: bool,
}

fn pointee(reg: &Registry, member: &StructMember, lifetime: &str, qualified: bool) -> String {
	let mut target = member.ty.clone();
	target.pointers.pop();
	type_str_for(&target, reg, lifetime, qualified)
}

fn recipe(
	reg: &Registry,
	ty: &TypeDecl,
	member: &StructMember,
	kind: Setter,
	lifetime: &str,
	qualified: bool,
) -> Recipe {
	let field = names::member_name(&member.name);
	let mut recipe = Recipe {
		method: method(member),
		parameters: vec![],
		body: String::new(),
		doc: format!("Set native field `{}` without native effects.", member.name),
		fallible: false,
		raw: false,
	};
	let native = type_str_for(&member.ty, reg, lifetime, qualified);
	let exclusive = if mutable(member) { "mut " } else { "" };
	let pointer = if mutable(member) {
		"as_mut_ptr"
	} else {
		"as_ptr"
	};
	let argument = match kind {
		kind @ (Setter::Value | Setter::NativeCount) => {
			if kind == Setter::NativeCount {
				recipe.doc = format!(
					"Set native count `{}` directly without checking associated pointer extents.\n/// Use the borrowed array helpers to derive matching storage and counts together.\n/// Direct count edits support null-query/dynamic-state paths and remain a\n/// native-call validity obligation, including after earlier borrowed setters.",
					member.name
				);
			}
			let value = if matches!(ty.kind, TypeKind::Union { .. })
				&& member.ty.pointers.is_empty()
				&& reg.exclusive.contains(&member.ty.name)
			{
				"std::mem::ManuallyDrop::new(value)"
			} else {
				"value"
			};
			write!(recipe.body, "self.{field} = {value};").unwrap();
			native
		}
		Setter::Raw => {
			recipe.raw = true;
			recipe.doc = format!(
				"Set native pointer `{}` explicitly. No borrow or extent is inferred.\n///\n/// # Safety\n/// The caller must establish pointee validity, extent, alignment, aliasing and\n/// the full native retention lifetime before using this record in a native call.\n/// Callback, external-object and nested-pointer contracts remain caller obligations.",
				member.name
			);
			write!(recipe.body, "self.{field} = value;").unwrap();
			native
		}
		Setter::String => {
			write!(recipe.body, "self.{field} = value.as_ptr();").unwrap();
			format!("&{lifetime} std::ffi::CStr")
		}
		Setter::Reference => {
			write!(recipe.body, "self.{field} = value;").unwrap();
			format!(
				"&{lifetime} {exclusive}{}",
				pointee(reg, member, lifetime, qualified)
			)
		}
		Setter::Words => {
			recipe.doc = "Borrow SPIR-V words and set native `codeSize` in bytes.\n/// Shader validity and required capabilities remain native-call obligations.".into();
			recipe.body =
				"self.code_size = std::mem::size_of_val(value); self.p_code = value.as_ptr();".into();
			format!("&{lifetime} [u32]")
		}
		Setter::VersionData => {
			recipe.doc = "Borrow the two UUIDs required by the native version-data extent.".into();
			write!(recipe.body, "self.{field} = value.as_ptr();").unwrap();
			format!(
				"&{lifetime} [u8; 2 * {}UUID_SIZE as usize]",
				if qualified { "vk::" } else { "" }
			)
		}
		Setter::ExternalSlice => {
			recipe.fallible = true;
			recipe
				.parameters
				.push(("max_sub_layers_minus1".into(), "u8".into()));
			recipe.doc = "Borrow H.265 HRD storage using its enclosing SPS/VPS sub-layer extent.\n/// The caller supplies the matching enclosing `*_max_sub_layers_minus1`;\n/// presence flags and the enclosing record remain native-call obligations.\n///\n/// # Errors\n/// Returns a length mismatch unless the slice has max_sub_layers_minus1 + 1 entries.".into();
			write!(recipe.body, "let expected = usize::from(max_sub_layers_minus1) + 1; if value.len() != expected {{ return Err(crate::ConstructionError::LengthMismatch {{ field: {:?}, expected, actual: value.len() }}); }} self.{field} = value.as_ptr();", member.name).unwrap();
			format!(
				"&{lifetime} [{}]",
				pointee(reg, member, lifetime, qualified)
			)
		}
		Setter::SampleMask => {
			recipe.fallible = true;
			recipe.doc = "Borrow sample-mask words, checking the current `rasterizationSamples`.\n/// Later raw/scalar edits and Vulkan valid usage remain caller obligations.\n///\n/// # Errors\n/// Returns a length mismatch unless there are ceil(samples / 32) words.".into();
			recipe.body = "let expected = (self.rasterization_samples.as_raw() as usize).div_ceil(32); if value.len() != expected { return Err(crate::ConstructionError::LengthMismatch { field: \"pSampleMask\", expected, actual: value.len() }); } self.p_sample_mask = value.as_ptr();".into();
			format!(
				"&{lifetime} [{}]",
				pointee(reg, member, lifetime, qualified)
			)
		}
		kind @ (Setter::Slice { .. } | Setter::Bytes { .. } | Setter::DescriptorPayload) => {
			let bytes = matches!(kind, Setter::Bytes { .. });
			let descriptor = kind == Setter::DescriptorPayload;
			let count = match kind {
				Setter::Slice { count } | Setter::Bytes { count } => count,
				_ => "descriptorCount".into(),
			};
			recipe.fallible = true;
			recipe.doc = format!(
				"Borrow `{}` and set checked native {} `{count}`.\n/// The borrow covers this storage only; nested raw pointers, format, alignment\n/// and asynchronous retention remain native-call obligations.\n/// Output storage must be initialized; returned sizes do not enlarge its capacity.\n///\n/// # Errors\n/// Returns a construction error if the length exceeds the native count width.",
				member.name,
				if bytes { "byte size" } else { "count" }
			);
			let conversion = count_expr(reg, ty, &count, &member.name, "value.len()");
			write!(recipe.body, "let count = {conversion};").unwrap();
			if descriptor || indirect_alternative(ty, member) {
				for peer in members(ty).iter().filter(|m| count_name(m) == Some(&count)) {
					write!(
						recipe.body,
						"self.{} = std::ptr::null();",
						names::member_name(&peer.name)
					)
					.unwrap();
				}
			}
			write!(
				recipe.body,
				"self.{} = count; self.{field} = value.{pointer}(){};",
				names::member_name(&count),
				if bytes { ".cast()" } else { "" }
			)
			.unwrap();
			format!(
				"&{lifetime} {exclusive}[{}]",
				if bytes {
					"u8".into()
				} else {
					pointee(reg, member, lifetime, qualified)
				}
			)
		}
	};
	recipe.parameters.push(("value".into(), argument));
	recipe
}

fn group_recipe(
	reg: &Registry,
	ty: &TypeDecl,
	count: &str,
	lifetime: &str,
	qualified: bool,
) -> Result<Recipe> {
	let fields: Vec<_> = members(ty)
		.iter()
		.filter(|m| count_name(m) == Some(count))
		.collect();
	ensure!(
		fields.len() > 1
			&& fields
				.iter()
				.all(|m| m.ty.pointers.len() == 1 && m.ty.arrays.is_empty()),
		"changed parallel array group {}.{count}",
		ty.c_name
	);
	let mut recipe = Recipe {
		method: method(fields[0]),
		parameters: vec![],
		body: String::new(),
		doc: format!(
			"Borrow parallel arrays sharing native `{count}`. Optional arrays may be null.\n/// Mutable arrays retain exclusive borrows of initialized output storage.\n/// This checks lengths only; native valid usage and later raw edits remain caller obligations.\n///\n/// # Errors\n/// Returns a construction error for unequal lengths or native count overflow."
		),
		fallible: true,
		raw: false,
	};
	let required = fields.iter().find(|m| !m.optional);
	if required.is_none() {
		ensure!(
			count_field(ty, count)
				.is_some_and(|field| matches!(field.ty.name.as_str(), "uint8_t" | "uint16_t" | "uint32_t")),
			"unreviewed explicit parallel count {}.{count}",
			ty.c_name
		);
	}
	let expected = if let Some(first) = required {
		format!("{}.len()", method(first))
	} else {
		let counter = count_field(ty, count).unwrap();
		recipe.parameters.push((
			names::member_name(count),
			type_str_for(&counter.ty, reg, lifetime, qualified),
		));
		format!("{} as usize", names::member_name(count))
	};
	write!(recipe.body, "let expected = {expected};").unwrap();
	for m in &fields {
		let argument = method(m);
		let exclusive = if mutable(m) { "mut " } else { "" };
		let mut parameter = format!(
			"&{lifetime} {exclusive}[{}]",
			pointee(reg, m, lifetime, qualified)
		);
		if m.optional {
			parameter = format!("Option<{parameter}>");
		}
		recipe.parameters.push((argument.clone(), parameter));
		if m.optional {
			write!(
				recipe.body,
				"if let Some(value) = &{argument} && value.len() != expected {{ return Err(crate::ConstructionError::LengthMismatch {{ field: {:?}, expected, actual: value.len() }}); }}",
				m.name
			)?;
		} else {
			write!(
				recipe.body,
				"if {argument}.len() != expected {{ return Err(crate::ConstructionError::LengthMismatch {{ field: {:?}, expected, actual: {argument}.len() }}); }}",
				m.name
			)?;
		}
	}
	let converted = count_expr(reg, ty, count, &fields[0].name, "expected");
	write!(recipe.body, "let count = {converted};")?;
	for m in fields {
		let argument = method(m);
		let pointer = if mutable(m) { "as_mut_ptr" } else { "as_ptr" };
		let null = if mutable(m) { "null_mut" } else { "null" };
		let value = if m.optional {
			format!("{argument}.map_or(std::ptr::{null}(), |value| value.{pointer}())")
		} else {
			format!("{argument}.{pointer}()")
		};
		write!(
			recipe.body,
			"self.{} = {value};",
			names::member_name(&m.name)
		)?;
	}
	write!(recipe.body, "self.{} = count;", names::member_name(count))?;
	Ok(recipe)
}

/// Element-counted opaque layer values are not bytes. Select the registry type
/// and native element extent together; string pointers borrow their table only.
fn layer_recipes(reg: &Registry, lifetime: &str, qualified: bool) -> Result<Vec<Recipe>> {
	let mut result = Vec::new();
	for (suffix, element, native) in [
		("bool32", "u32", "VK_LAYER_SETTING_TYPE_BOOL32_EXT"),
		("int32", "i32", "VK_LAYER_SETTING_TYPE_INT32_EXT"),
		("int64", "i64", "VK_LAYER_SETTING_TYPE_INT64_EXT"),
		("uint32", "u32", "VK_LAYER_SETTING_TYPE_UINT32_EXT"),
		("uint64", "u64", "VK_LAYER_SETTING_TYPE_UINT64_EXT"),
		("float32", "f32", "VK_LAYER_SETTING_TYPE_FLOAT32_EXT"),
		("float64", "f64", "VK_LAYER_SETTING_TYPE_FLOAT64_EXT"),
		(
			"strings",
			"*const std::ffi::c_char",
			"VK_LAYER_SETTING_TYPE_STRING_EXT",
		),
	] {
		ensure!(
			reg
				.enums
				.iter()
				.any(|e| e.c_name == "VkLayerSettingTypeEXT" && e.values.iter().any(|v| v.name == native)),
			"missing layer setting kind {native}"
		);
		let variant = names::enum_variant(native, "VkLayerSettingTypeEXT");
		result.push(Recipe {
   method: format!("values_{suffix}"),
   parameters: vec![("value".into(), format!("&{lifetime} [{element}]"))],
   body: format!("let count = crate::construction::count(\"pValues\", value.len())?; self.r#type = {}LayerSettingTypeEXT::{variant}; self.value_count = count; self.p_values = value.as_ptr().cast();", if qualified { "vk::" } else { "" }),
   doc: "Borrow typed layer values and set their native type and element count.\n/// For strings the borrow covers the pointer table, not the strings it names.\n///\n/// # Errors\n/// Returns a construction error if the element count exceeds u32.".into(),
   fallible: true, raw: false,
  });
	}
	Ok(result)
}

/// The fault API deliberately keeps capacities in a separate native record.
/// Populate that exact record without retaining a borrow that would prevent
/// passing both native arguments. Later edits remain an unsafe-call obligation.
fn fault_recipe(lifetime: &str, qualified: bool) -> Recipe {
	let prefix = if qualified { "vk::" } else { "" };
	Recipe {
  method: "buffers".into(),
  parameters: vec![
   ("counts".into(), format!("&mut {prefix}DeviceFaultCountsEXT<'_>")),
   ("address_infos".into(), format!("&{lifetime} mut [{prefix}DeviceFaultAddressInfoKHR]")),
   ("vendor_infos".into(), format!("&{lifetime} mut [{prefix}DeviceFaultVendorInfoKHR]")),
   ("vendor_binary_data".into(), format!("&{lifetime} mut [u8]")),
  ],
  body: "let address_count = crate::construction::count(\"pAddressInfos\", address_infos.len())?; let vendor_count = crate::construction::count(\"pVendorInfos\", vendor_infos.len())?; let binary_size = crate::construction::count_as::<u64>(\"pVendorBinaryData\", vendor_binary_data.len())?; counts.address_info_count = address_count; counts.vendor_info_count = vendor_count; counts.vendor_binary_size = binary_size; self.p_address_infos = address_infos.as_mut_ptr(); self.p_vendor_infos = vendor_infos.as_mut_ptr(); self.p_vendor_binary_data = vendor_binary_data.as_mut_ptr().cast();".into(),
  doc: "Borrow initialized fault buffers and set capacities in the accompanying counts record.\n/// Pass that same counts record to the native query, without increasing capacities.\n/// Only the buffers are retained by this record's lifetime; the native query may\n/// change reported counts, which must be bounded before reading returned data.\n///\n/// # Errors\n/// Returns count overflow before changing the counts record or borrowed storage.".into(),
  fallible: true, raw: false,
 }
}

fn recipes(reg: &Registry, ty: &TypeDecl, lifetime: &str, qualified: bool) -> Result<Vec<Recipe>> {
	let mut result = Vec::new();
	for member in members(ty) {
		if ty.c_name == "VkDeviceFaultInfoEXT"
			&& matches!(
				member.name.as_str(),
				"pAddressInfos" | "pVendorInfos" | "pVendorBinaryData"
			) {
			if member.name == "pAddressInfos" {
				result.push(fault_recipe(lifetime, qualified));
			}
			continue;
		}
		if let Some(count) = count_name(member).filter(|c| grouped(ty, c)) {
			if members(ty)
				.iter()
				.find(|m| count_name(m) == Some(count))
				.is_some_and(|m| m.name == member.name)
			{
				result.push(group_recipe(reg, ty, count, lifetime, qualified)?);
			}
			continue;
		}
		if let Some(kind) = setter(reg, ty, member) {
			result.push(recipe(reg, ty, member, kind, lifetime, qualified));
		}
		if ty.c_name == "VkLayerSettingEXT" && member.name == "pValues" {
			result.extend(layer_recipes(reg, lifetime, qualified)?);
		}
	}
	let mut seen = std::collections::BTreeSet::new();
	for r in &result {
		ensure!(
			seen.insert(&r.method),
			"duplicate construction method {}.{}",
			ty.c_name,
			r.method
		);
	}
	Ok(result)
}

pub fn emit_setters(out: &mut String, reg: &Registry, ty: &TypeDecl) -> Result<()> {
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
	for recipe in recipes(reg, ty, "'a", false)? {
		writeln!(out, "/// {}", recipe.doc)?;
		for member in members(ty)
			.iter()
			.filter(|member| method(member) == recipe.method)
		{
			crate::doc::alias(out, &member.name, &recipe.method);
		}
		let result = if recipe.fallible {
			"std::result::Result<Self, crate::ConstructionError>"
		} else {
			"Self"
		};
		let ret = if recipe.fallible { "Ok(self)" } else { "self" };
		let must_use = if recipe.fallible { "" } else { "#[must_use]\n" };
		let safety = if recipe.raw { "unsafe " } else { "" };
		let params = recipe
			.parameters
			.iter()
			.map(|(n, t)| format!("{n}: {t}"))
			.collect::<Vec<_>>()
			.join(", ");
		writeln!(
			out,
			"#[inline]\n{must_use}pub {safety}fn {}(mut self, {params}) -> {result} {{ {} {ret} }}",
			recipe.method, recipe.body
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
		.filter(|ty| matches!(ty.kind, TypeKind::Struct { .. } | TypeKind::Union { .. }))
	{
		let guard = protect::cfg(&ty.guards);
		let name = declaration(reg, ty, "'static");
		writeln!(
			out,
			"{guard}#[test]\nfn {}_construction_contract() {{",
			names::member_name(&ty.c_name)
		)?;
		if tagged(ty) {
			let tag = names::enum_variant(members(ty)[0].values.as_deref().unwrap(), "VkStructureType");
			writeln!(
				out,
				"let value = vk::{}::default();\nassert_eq!(value.s_type, vk::StructureType::{tag});\nassert!(value.p_next.is_null());",
				names::type_name(&ty.c_name)
			)?;
		}
		for (index, recipe) in recipes(reg, ty, "'static", true)?.into_iter().enumerate() {
			let result = if recipe.fallible {
				format!("std::result::Result<vk::{name}, vk::ConstructionError>")
			} else {
				format!("vk::{name}")
			};
			let arguments = recipe
				.parameters
				.iter()
				.map(|(_, ty)| ty.as_str())
				.collect::<Vec<_>>()
				.join(", ");
			let signature = format!(
				"{}fn(vk::{name}, {arguments}) -> {result}",
				if recipe.raw { "unsafe " } else { "" }
			);
			let signature = if recipe.parameters.len() > 1 {
				writeln!(out, "type Signature{index} = {signature};")?;
				format!("Signature{index}")
			} else {
				signature
			};
			writeln!(
				out,
				"let _: {signature} = vk::{}::{};",
				names::type_name(&ty.c_name),
				recipe.method
			)?;
		}
		writeln!(out, "}}")?;
	}
	Ok(out)
}

/// Exhaustive native field classification, including deliberate unsafe boundaries.
pub fn emit_inventory(reg: &Registry) -> String {
	let mut out =
		String::from("# @generated by vk-gen: native record\tfield\tconstruction\tlen\taltlen\n");
	for ty in &reg.types {
		for member in members(ty) {
			let kind = if member.bitfield.is_some() {
				"bitfield".into()
			} else if tagged(ty) && matches!(member.name.as_str(), "sType" | "pNext") {
				"typed-header".into()
			} else if count_name(member).is_some_and(|count| grouped(ty, count)) {
				"parallel-array".into()
			} else if ty.c_name == "VkDeviceFaultInfoEXT"
				&& matches!(
					member.name.as_str(),
					"pAddressInfos" | "pVendorInfos" | "pVendorBinaryData"
				) {
				"external-capacity-buffers".into()
			} else if ty.c_name == "VkLayerSettingEXT" && member.name == "pValues" {
				"typed-layer-values".into()
			} else if setter(reg, ty, member) == Some(Setter::Raw) {
				if member.ty.name == "void" {
					"unsafe-opaque-address".into()
				} else {
					"unsafe-external-object".into()
				}
			} else if let Some(kind) = setter(reg, ty, member) {
				format!("{kind:?}")
			} else {
				"derived-count".into()
			};
			writeln!(
				out,
				"{}\t{}\t{kind}\t{}\t{}",
				ty.c_name,
				member.name,
				member.len.as_deref().unwrap_or("-"),
				member.alt_len.as_deref().unwrap_or("-")
			)
			.unwrap();
		}
	}
	out
}

/// Diagnostic method counts; emitted declarations retain registry order.
pub fn inventory(reg: &Registry) -> BTreeMap<String, usize> {
	reg
		.types
		.iter()
		.filter(|ty| matches!(ty.kind, TypeKind::Struct { .. } | TypeKind::Union { .. }))
		.map(|ty| {
			(
				ty.c_name.clone(),
				recipes(reg, ty, "'a", false)
					.expect("validated record recipes")
					.len(),
			)
		})
		.collect()
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_construction.rs"]
mod tests;
