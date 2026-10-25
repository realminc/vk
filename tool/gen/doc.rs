//! Native-name search and Khronos references shared by all binding emitters.
use std::fmt::Write;

const REFPAGES: &str = "https://docs.vulkan.org/refpages/latest/refpages/source";

/// Registry-only definitions do not necessarily have individual reference pages.
pub fn registry_url(video: bool) -> String {
	let revision = include_str!("../../registry/SOURCES.md")
		.lines()
		.find_map(|line| line.strip_prefix("| Commit | `")?.strip_suffix("` |"))
		.expect("registry/SOURCES.md must record the pinned Vulkan-Headers commit");
	assert!(revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()));
	let file = if video { "video.xml" } else { "vk.xml" };
	format!("https://github.com/KhronosGroup/Vulkan-Headers/blob/{revision}/registry/{file}")
}

pub fn alias(out: &mut String, native: &str, rust: &str) {
	if native != rust.trim_start_matches("r#") {
		writeln!(out, "#[doc(alias = {native:?})]").unwrap();
	}
}

/// `owner` is the documented enum for enumerants, or the command for command PFNs.
pub fn reference(out: &mut String, native: &str, owner: &str, video: bool) {
	let reference = if !video
		&& (owner.starts_with("Vk") || owner.starts_with("vk") || owner.starts_with("PFN_vk"))
	{
		format!("[Khronos reference: `{native}`]({REFPAGES}/{owner}.html)")
	} else {
		format!(
			"[Khronos registry source: `{native}`]({})",
			registry_url(video)
		)
	};
	writeln!(out, "#[doc = {:?}]", format!("\n\n{reference}")).unwrap();
}

pub fn item(out: &mut String, native: &str, rust: &str, kind: &str, video: bool) {
	writeln!(out, "#[doc = {:?}]", format!("{kind} `{native}`.")).unwrap();
	alias(out, native, rust);
	reference(out, native, native, video);
}

pub fn command(out: &mut String, native: &str, rust: &str) {
	alias(out, native, rust);
	reference(out, native, native, false);
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_doc.rs"]
mod tests;
