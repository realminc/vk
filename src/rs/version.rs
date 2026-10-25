//! Vulkan's packed API version, including the variant bits.

/// Pack a Vulkan API version using the Khronos bit allocation.
#[doc(alias = "VK_MAKE_API_VERSION")]
#[doc = "\n\n[Khronos reference: `VK_MAKE_API_VERSION`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_API_VERSION.html)"]
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
	(variant << 29) | (major << 22) | (minor << 12) | patch
}

/// Extract the API variant.
#[doc(alias = "VK_API_VERSION_VARIANT")]
#[doc = "\n\n[Khronos reference: `VK_API_VERSION_VARIANT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_VARIANT.html)"]
pub const fn api_version_variant(version: u32) -> u32 {
	version >> 29
}

/// Extract the major version, excluding the variant.
#[doc(alias = "VK_API_VERSION_MAJOR")]
#[doc = "\n\n[Khronos reference: `VK_API_VERSION_MAJOR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MAJOR.html)"]
pub const fn api_version_major(version: u32) -> u32 {
	(version >> 22) & 0x7f
}

/// Extract the minor version.
#[doc(alias = "VK_API_VERSION_MINOR")]
#[doc = "\n\n[Khronos reference: `VK_API_VERSION_MINOR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MINOR.html)"]
pub const fn api_version_minor(version: u32) -> u32 {
	(version >> 12) & 0x3ff
}

/// Extract the patch version.
#[doc(alias = "VK_API_VERSION_PATCH")]
#[doc = "\n\n[Khronos reference: `VK_API_VERSION_PATCH`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_PATCH.html)"]
pub const fn api_version_patch(version: u32) -> u32 {
	version & 0xfff
}
