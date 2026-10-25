//! Typed opaque handles. Non-dispatchable identities retain the Ash-style u64 ABI.
use crate::{
	model::{TypeDecl, TypeKind},
	names,
	protect::cfg,
};
use std::fmt::Write;

pub fn emit_handle(out: &mut String, ty: &TypeDecl) {
	let name = names::type_name(&ty.c_name);
	let guard = cfg(&ty.guards);
	let (backing, null, from_raw, as_raw) = if matches!(ty.kind, TypeKind::DispatchableHandle) {
		(
			"*mut c_void",
			"std::ptr::null_mut()",
			"value as *mut c_void",
			"self.0 as u64",
		)
	} else {
		("u64", "0", "value", "self.0")
	};
	writeln!(out,"{guard}/// Raw Vulkan handle `{}`. Calls using it must satisfy Vulkan lifetime and synchronization rules.\n#[repr(transparent)]\n#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]\npub struct {name}(pub {backing});",ty.c_name).unwrap();
	writeln!(out,"{guard}impl {name} {{\npub const NULL:Self=Self({null});\npub fn is_null(self)->bool{{self.0==Self::NULL.0}}\npub fn from_raw(value:u64)->Self{{Self({from_raw})}}\npub fn as_raw(self)->u64{{{as_raw}}}\n}}\n{guard}impl Default for {name} {{fn default()->Self{{Self::NULL}}}}\n{guard}// SAFETY: an opaque raw identity grants no safe dereference or synchronized access.\nunsafe impl Send for {name} {{}}\n{guard}// SAFETY: all Vulkan calls remain unsafe and retain external synchronization contracts.\nunsafe impl Sync for {name} {{}}").unwrap();
}
