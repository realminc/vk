#![allow(dead_code)]
//! Pass C: Vulkan C name → idiomatic Rust name conversion.
//!
//! Handles Vulkan acronym conventions (VP9, AV1, H264, H265, KHR, EXT,
//! vendor tags, digits) without naïve camel-case splitting.

// ── Type names ────────────────────────────────────────────────────────────────

/// Strip the `Vk` prefix from a C type name.
///
/// ```
/// # use vkrust_gen::names::type_name;
/// assert_eq!(type_name("VkInstance"), "InstanceHandle");
/// assert_eq!(type_name("VkVideoDecodeVP9PictureInfoKHR"), "VideoDecodeVP9PictureInfoKHR");
/// ```
pub fn type_name(c: &str) -> String {
	match c {
		"VkInstance" => "InstanceHandle".into(),
		"VkDevice" => "DeviceHandle".into(),
		_ => c.strip_prefix("Vk").unwrap_or(c).to_string(),
	}
}

/// Strip `StdVideo` prefix for video.xml types, leave the rest intact.
pub fn video_type_name(c: &str) -> String {
	c.strip_prefix("StdVideo").unwrap_or(c).to_string()
}

// ── Constant names ────────────────────────────────────────────────────────────

/// Strip the `VK_` prefix; SCREAMING_SNAKE_CASE is preserved.
///
/// ```
/// # use vkrust_gen::names::const_name;
/// assert_eq!(const_name("VK_MAX_MEMORY_TYPES"), "MAX_MEMORY_TYPES");
/// ```
pub fn const_name(c: &str) -> &str {
	c.strip_prefix("VK_").unwrap_or(c)
}

// ── Enum variant names ────────────────────────────────────────────────────────

/// Convert a Vulkan enum value name into a Rust variant name.
///
/// Strips the common type prefix then converts from SCREAMING_SNAKE_CASE to
/// PascalCase, applying Vulkan acronym rules.
///
/// ```
/// # use vkrust_gen::names::enum_variant;
/// assert_eq!(enum_variant("VK_FORMAT_R8G8B8A8_UNORM", "VkFormat"), "R8G8B8A8_UNORM");
/// assert_eq!(enum_variant("VK_VIDEO_CODEC_OPERATION_DECODE_VP9_BIT_KHR", "VkVideoCodecOperationFlagBitsKHR"), "DECODE_VP9_BIT_KHR");
/// ```
pub fn enum_variant(c_value: &str, enum_c_name: &str) -> String {
	let stripped = c_value
		.strip_prefix("VK_")
		.or_else(|| c_value.strip_prefix("STD_VIDEO_"))
		.unwrap_or(c_value);
	if enum_c_name == "VkResult" {
		return stripped.to_owned();
	}
	let type_name = enum_c_name
		.strip_prefix("Vk")
		.or_else(|| enum_c_name.strip_prefix("StdVideo"))
		.unwrap_or(enum_c_name);
	// Derive the registry's common word prefix, preserving vendor suffixes on values.
	let without_tag = [
		"KHR", "EXT", "NV", "AMD", "ARM", "QCOM", "INTEL", "HUAWEI", "FUCHSIA", "VALVE",
	]
	.iter()
	.find_map(|tag| type_name.strip_suffix(tag))
	.unwrap_or(type_name);
	let prefix = camel_to_snake(without_tag)
		.replace("flag_bits", "")
		.trim_end_matches('_')
		.to_uppercase();
	let prefix = if let Some(rest) = prefix.strip_suffix('2') {
		format!("{}_2", rest.trim_end_matches('_'))
	} else {
		prefix
	};
	let name = stripped.strip_prefix(&(prefix + "_")).unwrap_or(stripped);
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		format!("_{name}")
	} else {
		name.to_owned()
	}
}

// ── Command names ─────────────────────────────────────────────────────────────

/// Convert a Vulkan command name to Rust snake_case, stripping the `vk` prefix.
///
/// ```
/// # use vkrust_gen::names::command_name;
/// assert_eq!(command_name("vkCreateInstance"), "create_instance");
/// assert_eq!(command_name("vkCmdDecodeVideoKHR"), "cmd_decode_video_khr");
/// assert_eq!(command_name("vkGetPhysicalDeviceProperties2"), "get_physical_device_properties2");
/// ```
pub fn command_name(c: &str) -> String {
	let stripped = c.strip_prefix("vk").unwrap_or(c);
	// Preserve uppercase runs without a finite vendor allowlist. This keeps new
	// tags atomic too, including extensions of existing tags (NVX and AMDX).
	member_name(stripped)
}

/// Rust field/parameter spelling. Existing snake_case and numeric suffixes are
/// retained; acronym boundaries follow ordinary Rust conventions.
pub fn member_name(c: &str) -> String {
	let chars: Vec<_> = c.chars().collect();
	let mut out = String::new();
	for (i, &ch) in chars.iter().enumerate() {
		if ch.is_ascii_uppercase() && i > 0 {
			let previous = chars[i - 1];
			let next_lower = chars.get(i + 1).is_some_and(char::is_ascii_lowercase);
			if previous.is_ascii_lowercase()
				|| previous.is_ascii_digit()
				|| (previous.is_ascii_uppercase() && next_lower)
			{
				out.push('_');
			}
		}
		out.push(ch.to_ascii_lowercase());
	}
	match out.as_str() {
		"self" | "super" | "crate" => format!("{out}_"),
		"as" | "async" | "await" | "break" | "const" | "continue" | "dyn" | "else" | "enum"
		| "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match"
		| "mod" | "move" | "mut" | "pub" | "ref" | "return" | "static" | "struct" | "trait"
		| "true" | "type" | "unsafe" | "use" | "where" | "while" | "abstract" | "become" | "box"
		| "do" | "final" | "gen" | "macro" | "override" | "priv" | "try" | "typeof" | "unsized"
		| "virtual" | "yield" => {
			format!("r#{out}")
		}
		_ => out,
	}
}

// ── Core conversions ──────────────────────────────────────────────────────────

/// SCREAMING_SNAKE_CASE → PascalCase with Vulkan acronym awareness.
///
/// Rules (applied per word token after splitting on `_`):
/// - Known Vulkan acronyms remain fully upper: `KHR`, `EXT`, `ARB`, `NV`, `AMD`, etc.
/// - Codec/numeric tokens remain fully upper: `VP9`, `AV1`, `H264`, `H265`, `HEVC`.
/// - Digit-only tokens stay as-is.
/// - Everything else: first letter uppercase, rest lowercase.
pub fn screaming_to_pascal(s: &str) -> String {
	s.split('_')
		.filter(|w| !w.is_empty())
		.map(word_to_pascal_token)
		.collect()
}

/// PascalCase/camelCase → snake_case with Vulkan acronym awareness.
pub fn camel_to_snake(s: &str) -> String {
	let tokens = split_camel_vulkan(s);
	tokens
		.iter()
		.map(|t| t.to_lowercase())
		.collect::<Vec<_>>()
		.join("_")
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn word_to_pascal_token(word: &str) -> String {
	// Known all-caps Vulkan acronyms/suffixes that must keep their casing as
	// a single unit (or map to a conventional Rust spelling).
	const KEEP_UPPER: &[&str] = &[
		"KHR", "EXT", "ARB", "NV", "AMD", "INTEL", "IMG", "QCOM", "HUAWEI", "VALVE", "MVK", "GGP",
		"QNX", "FUCHSIA", "SEC", "LUNARG", "LAYERED",
	];
	const CODEC_UPPER: &[&str] = &["VP9", "AV1", "H264", "H265", "HEVC", "AVC", "VP8"];

	if KEEP_UPPER.contains(&word) || CODEC_UPPER.contains(&word) {
		// Use title-case equivalent for readability in variant names:
		// KHR → Khr, VP9 → Vp9, AV1 → Av1
		let mut chars = word.chars();
		return match chars.next() {
			Some(c) => c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
			None => String::new(),
		};
	}
	// Digit-only or all-digit-prefixed: return as-is.
	if word.chars().all(|c| c.is_ascii_digit()) {
		return word.to_string();
	}
	// Normal word: first char upper, rest lower.
	let mut chars = word.chars();
	match chars.next() {
		Some(c) => c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
		None => String::new(),
	}
}

/// Split a Vulkan PascalCase/camelCase string into word tokens, treating known
/// Vulkan acronyms and codec names as atomic tokens rather than splitting them.
pub fn split_camel_vulkan(s: &str) -> Vec<String> {
	// Known all-uppercase atomic tokens (longest first to avoid prefix matching).
	const ATOMS: &[&str] = &[
		"INTEL", "VALVE", "QCOM", "HUAWEI", "FUCHSIA", "LAYERED", "LUNARG", "KHR", "EXT", "ARB", "NV",
		"AMD", "IMG", "MVK", "GGP", "QNX", "SEC", "VP9", "AV1", "H264", "H265", "HEVC", "AVC", "VP8",
		"2D", "3D",
	];

	let mut tokens: Vec<String> = Vec::new();
	let chars: Vec<char> = s.chars().collect();
	let mut i = 0;

	while i < chars.len() {
		// Try to match a known atomic token at position i.
		let remaining: String = chars[i..].iter().collect();
		let remaining_upper = remaining.to_uppercase();
		let atom_match = ATOMS.iter().find(|&&atom| {
			remaining_upper.starts_with(atom)
				&& remaining
					.chars()
					.nth(atom.len())
					.map(|c| c.is_uppercase() || c.is_ascii_digit() || !c.is_alphabetic())
					.unwrap_or(true)
		});

		if let Some(atom) = atom_match {
			tokens.push(atom.to_string());
			i += atom.len();
			continue;
		}

		// Collect the next word: an uppercase letter optionally followed by
		// lowercase letters and digits.
		if chars[i].is_uppercase() {
			let mut word = chars[i].to_string();
			i += 1;
			while i < chars.len() && (chars[i].is_lowercase() || chars[i].is_ascii_digit()) {
				word.push(chars[i]);
				i += 1;
			}
			tokens.push(word);
		} else {
			// Lowercase run (beginning of string).
			let mut word = String::new();
			while i < chars.len() && chars[i].is_lowercase() {
				word.push(chars[i]);
				i += 1;
			}
			if !word.is_empty() {
				tokens.push(word);
			}
		}
	}
	tokens
}

#[cfg(test)]
#[path = "../../test/rs/tool/gen/test_names.rs"]
mod tests;
