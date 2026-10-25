//! Checked memory selection, separate storage and explicit pooled buffers.
//!
//! Allocator operations do not submit or wait. GPU completion, resource
//! destruction and host/GPU access ordering remain caller responsibilities.

mod allocation;
mod allocator;
mod block;
mod policy;
mod resource;
mod selector;
mod tlsf;
pub mod virtual_block;

pub use allocation::Allocation;
pub use allocator::{
	Allocator, allocate, budget_snapshot, close, create_buffer, create_buffer_with_options,
	create_image, create_image_with_options, destroy_buffer, destroy_image, flush, free, invalidate,
	map, mapped_pointer, query_budget, set_heap_limit, statistics, unmap,
};
pub use policy::{HostAccess, MemoryUsage};
pub use selector::MemoryTypeSelector;
pub use virtual_block::{
	PlacementStrategy, VirtualAllocation, VirtualAllocationInfo, VirtualBlock, VirtualBlockStats,
};

use crate::{self as vk, load::CallError};

#[derive(Debug, PartialEq)]
pub enum AllocError {
	NoCompatibleMemoryType,
	InvalidProperties(&'static str),
	InvalidRequest(&'static str),
	InvalidRange,
	UnknownAllocation,
	WrongAllocator,
	AlreadyMapped,
	NotMapped,
	NotHostVisible,
	SerialExhausted,
	HostAllocationFailed,
	AllocationLimit,
	OutOfSpace,
	AllocationTooLarge { requested: u64, limit: u64 },
	UnsupportedCapability(&'static str),
	WrongResource,
	LiveAllocations,
	Closed,
	Call(CallError),
}

impl std::fmt::Display for AllocError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "allocation error: {self:?}")
	}
}

impl std::error::Error for AllocError {}

impl From<CallError> for AllocError {
	fn from(error: CallError) -> Self {
		Self::Call(error)
	}
}

pub struct AllocRequest {
	pub size: u64,
	pub alignment: u64,
	pub memory_type_bits: u32,
	pub required_flags: vk::MemoryPropertyFlags,
	pub preferred_flags: vk::MemoryPropertyFlags,
}

/// Managed resource storage policy. Required or preferred native dedication takes precedence.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AllocationPolicy {
	#[default]
	Separate,
	/// Reuse ordinary buffer blocks with this exact native size. No block shrinking.
	/// Requests larger than the block use separate storage.
	Pooled { block_size: u64 },
	/// Try bounded power-of-two block sizes, shrinking only on recoverable storage failure.
	/// Large regions and exhausted block attempts fall back to separate storage.
	Adaptive {
		block_size: u64,
		min_block_size: u64,
	},
}

/// Managed-resource policy; not a Vulkan ABI record or an arbitrary allocation chain.
#[derive(Clone, Copy, Debug)]
pub struct AllocationCreateInfo {
	pub usage: MemoryUsage,
	pub host_access: HostAccess,
	/// Permit automatic selection of non-host-visible memory; caller supplies staging.
	pub allow_transfer_instead: bool,
	/// Properties penalized during memory-type selection; never a hard exclusion.
	pub avoided_flags: vk::MemoryPropertyFlags,
	pub required_flags: vk::MemoryPropertyFlags,
	pub preferred_flags: vk::MemoryPropertyFlags,
	/// Intersected with queried native requirements; zero admits no memory types.
	pub memory_type_bits: u32,
	pub strategy: PlacementStrategy,
	pub policy: AllocationPolicy,
	/// Retain a mapping until explicit allocation release. Requires host-visible storage.
	pub persistent_map: bool,
}

impl Default for AllocationCreateInfo {
	fn default() -> Self {
		Self {
			usage: MemoryUsage::Explicit,
			host_access: HostAccess::None,
			allow_transfer_instead: false,
			avoided_flags: vk::MemoryPropertyFlags::empty(),
			required_flags: vk::MemoryPropertyFlags::empty(),
			preferred_flags: vk::MemoryPropertyFlags::empty(),
			memory_type_bits: u32::MAX,
			strategy: PlacementStrategy::Default,
			policy: AllocationPolicy::Separate,
			persistent_map: false,
		}
	}
}

/// Local accounting snapshot; excludes allocations made by other allocators/raw calls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AllocatorStats {
	pub allocation_count: usize,
	pub native_allocation_count: usize,
	pub pooled_block_count: usize,
	/// Sum of native requirements sizes of live regions.
	pub allocation_bytes: u64,
	/// Includes cache-atom padding reserved to live pooled regions.
	pub reserved_bytes: u64,
	pub native_bytes: u64,
	pub mapped_native_count: usize,
}

#[cfg(test)]
#[path = "../../test/rs/memory/test_host_allocator.rs"]
pub(crate) mod test_host_allocator;

/// Budget provenance. Heap-size/local accounting does not measure external pressure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BudgetSource {
	#[default]
	Local,
	Extension,
}

/// One heap at the explicit snapshot boundary. Budget/usage can change immediately.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HeapBudget {
	pub size: u64,
	pub limit: u64,
	pub budget: u64,
	pub usage: u64,
	pub native_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BudgetSnapshot {
	pub source: BudgetSource,
	pub heap_count: u32,
	pub heaps: [HeapBudget; 16],
}
