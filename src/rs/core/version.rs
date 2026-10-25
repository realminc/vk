//! Vulkan's packed API version, including the variant bits.

/// Pack a Vulkan API version using the Khronos bit allocation.
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
	(variant << 29) | (major << 22) | (minor << 12) | patch
}

/// Extract the API variant.
pub const fn api_version_variant(version: u32) -> u32 {
	version >> 29
}

/// Extract the major version, excluding the variant.
pub const fn api_version_major(version: u32) -> u32 {
	(version >> 22) & 0x7f
}

/// Extract the minor version.
pub const fn api_version_minor(version: u32) -> u32 {
	(version >> 12) & 0x3ff
}

/// Extract the patch version.
pub const fn api_version_patch(version: u32) -> u32 {
	version & 0xfff
}
