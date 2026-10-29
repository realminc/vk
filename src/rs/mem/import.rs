//! External accounting supplied at the unsafe native import boundary.
use crate as vk;

/// Resource-specific native dedication for an imported payload.
/// The resource remains caller-owned; this record never destroys it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportResource {
	Buffer(vk::Buffer),
	Image(vk::Image),
}

/// Actual external backing extent and compatible types for one import.
///
/// Attach through [`super::AllocationChain::new_from_record_with_import`].
/// This is Rust metadata, not a Vulkan ABI record or a support query.
/// All bytes are charged to the selected heap even when Vulkan ignores the size.
#[derive(Clone, Copy, Debug)]
pub struct ImportMemoryInfo {
	/// Exact backing size from native external properties or authoritative OS metadata.
	pub allocation_size: u64,
	/// Compatible native types for this exact device and payload; never inferred from PFNs.
	pub memory_type_bits: u32,
	/// Exact resource when dedication is required or pre-bind requirements cannot be queried.
	/// Its requirements must fit the supplied extent and type mask. Binding remains explicit.
	pub resource: Option<ImportResource>,
}
