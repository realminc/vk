//! Per-context Vulkan dispatch tables and proc-address loading.
use crate::{
	model::{DispatchLevel, Registry},
	names,
};
use std::fmt::Write;

pub fn emit_dispatch(reg: &Registry) -> String {
	let mut out =
		String::from("// @generated — do not edit by hand.\nuse super::{pfn::*, types::*};\n");
	for (name, level) in [
		("FnEntry", DispatchLevel::Global),
		("FnInstance", DispatchLevel::Instance),
		("FnDevice", DispatchLevel::Device),
	] {
		writeln!(out, "#[derive(Clone, Default)]\npub struct {name} {{").unwrap();
		for cmd in reg.commands.iter().filter(|c| c.dispatch == level) {
			out.push_str(&crate::protect::cfg(&cmd.guards));
			writeln!(
				out,
				"pub {}: PFN_{},",
				names::command_name(&cmd.c_name),
				cmd.c_name
			)
			.unwrap();
		}
		out.push_str("}\n");
	}
	// impl Fn* { pub unsafe fn new_from_resolver(...) -> Self }
	emit_dispatch_impls(reg, &mut out);
	out
}

/// Emit `new_from_resolver()` constructors for each dispatch table.
///
/// - `FnEntry::new_from_resolver(gipa)` — resolves global commands against a null instance.
/// - `FnInstance::new_from_resolver(gipa, instance)` — resolves instance-level commands.
/// - `FnDevice::new_from_resolver(gdpa, device)` — resolves device-level commands.
///
/// Every field is attempted; missing optional functions remain `None`.
fn emit_dispatch_impls(reg: &Registry, out: &mut String) {
	// (struct name, dispatch level, resolver type alias, first param type, first param name)
	let tables = [
		(
			"FnEntry",
			DispatchLevel::Global,
			// gipa called with NULL instance for globals
			"unsafe extern \"system\" fn(InstanceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction",
			"InstanceHandle",
			"InstanceHandle::NULL",
		),
		(
			"FnInstance",
			DispatchLevel::Instance,
			"unsafe extern \"system\" fn(InstanceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction",
			"InstanceHandle",
			"instance",
		),
		(
			"FnDevice",
			DispatchLevel::Device,
			"unsafe extern \"system\" fn(DeviceHandle, *const std::ffi::c_char) -> PFN_vkVoidFunction",
			"DeviceHandle",
			"device",
		),
	];
	for (struct_name, level, resolver_sig, handle_type, handle_expr) in tables {
		let cmds: Vec<_> = reg
			.commands
			.iter()
			.filter(|c| c.dispatch == level)
			.collect();
		let param = if handle_expr == "InstanceHandle::NULL" {
			// FnEntry::new_from_resolver takes only the resolver; instance handle is always null
			format!("load: {resolver_sig}")
		} else {
			format!("load: {resolver_sig}, {handle_expr}: {handle_type}")
		};
		writeln!(
			out,
			"impl {struct_name} {{\n/// Load all commands for this dispatch level.\n///\n/// # Safety\n/// `load` must implement the Vulkan proc-addr contract and remain callable\n/// for the lifetime of this table and all derived raw function pointers.\npub unsafe fn new_from_resolver({param}) -> Self {{"
		).unwrap();
		writeln!(out, "macro_rules! resolve {{").unwrap();
		writeln!(out, "($name:literal, $ty:ty) => {{").unwrap();
		writeln!(
			out,
			"unsafe {{ std::mem::transmute::<PFN_vkVoidFunction, $ty>(load({handle_expr}, $name.as_ptr())) }}"
		).unwrap();
		writeln!(out, "}};").unwrap();
		writeln!(out, "}}").unwrap();
		writeln!(out, "Self {{").unwrap();
		for cmd in &cmds {
			out.push_str(&crate::protect::cfg(&cmd.guards));
			let field = names::command_name(&cmd.c_name);
			let pfn = format!("PFN_{}", cmd.c_name);
			let c_name_lit = &cmd.c_name;
			writeln!(out, "{field}: resolve!(c\"{c_name_lit}\", {pfn}),").unwrap();
		}
		writeln!(out, "}}").unwrap(); // close Self {
		writeln!(out, "}}").unwrap(); // close fn new_from_resolver
		if level != DispatchLevel::Global {
			out.push_str("#[cfg(feature = \"loader\")]\npub(crate) fn retain_enabled(&mut self, mut enabled: impl FnMut(&super::extensions::CommandMetadata) -> bool) {\n");
			for (index, cmd) in reg
				.commands
				.iter()
				.enumerate()
				.filter(|(_, c)| c.dispatch == level)
			{
				out.push_str(&crate::protect::cfg(&cmd.guards));
				writeln!(
					out,
					"if !enabled(&super::extensions::COMMANDS[{index}]) {{ self.{} = None; }}",
					names::command_name(&cmd.c_name)
				)
				.unwrap();
			}
			out.push_str("}\n");
		}
		writeln!(out, "}}").unwrap(); // close impl
	}
}
