//! Rust-native, VMA-derived allocation, managed buffers/images and virtual placement.
//!
//! Free functions and [`Allocator`] methods share one implementation. For example,
//! [`create_buffer_with_options`] and [`Allocator::create_buffer_with_options`]
//! create, allocate and bind a managed buffer. The root [`crate::create_buffer`]
//! is the raw Vulkan operation and creates an unbound buffer.
//!
//! Use `use vk::mem as vma;` for a caller-local alias. This adds no wrapper,
//! runtime cost or second allocator type; [`crate::Allocator`] is [`Allocator`].
//! The allocator is Experimental. Full VMA equivalence, unrestricted mixed-phase
//! concurrency, broad platform execution and comparative performance remain
//! unqualified. See the [qualification report](https://github.com/realminc/vk/blob/main/docs/external/qualification.md).
//!
//! Allocator operations do not submit or wait. GPU completion, resource
//! destruction and host/GPU access ordering remain caller responsibilities.

mod allocation;
mod allocator;
mod block;
mod chain;
mod granularity;
mod host;
mod linear;
mod metadata;
mod policy;
mod resource;
mod selector;
mod synchronized;
pub use synchronized::SynchronizedAllocator;
mod tlsf;
pub mod virtual_block;

pub use allocation::Allocation;
#[cfg(unix)]
pub use allocator::get_memory_fd;
#[cfg(all(windows, feature = "platform-win32"))]
pub use allocator::get_memory_win32_handle;
pub use allocator::{
	Allocator, TrimStats, allocate, allocate_dedicated, allocate_for_buffer, allocate_for_image,
	allocate_with_options, allocation_info, bind_buffer_memory, bind_image_memory, budget_snapshot,
	close, create_buffer, create_buffer_with_options, create_dedicated_buffer,
	create_dedicated_image, create_image, create_image_with_options, destroy_buffer, destroy_image,
	find_memory_type, find_memory_type_for_buffer_info, find_memory_type_for_image_info, flush, free,
	invalidate, map, mapped_pointer, query_budget, set_heap_limit, statistics, trim,
	trim_with_budget, unmap,
};
pub use allocator::{
	CacheRange, DetailedStatistics, MemoryStatistics, allocate_many, allocation_name, allocator_info,
	copy_from_allocation, copy_to_allocation, create_aliasing_buffer, create_aliasing_image,
	detailed_pool_statistics, detailed_statistics, flush_many, free_many, invalidate_many,
	memory_properties, memory_type_properties, physical_device_properties, pool_name,
	set_allocation_name, set_allocation_user_data, set_pool_name, write_statistics_json,
};
pub use allocator::{
	DefragmentationAlgorithm, DefragmentationContext, DefragmentationInfo, DefragmentationMove,
	DefragmentationPass, DefragmentationResource, DefragmentationStats, begin_defragmentation,
	begin_defragmentation_with_break, set_allocation_pinned,
};
pub use allocator::{
	Pool, PoolCreateInfo, PoolStats, create_pool, create_pool_with_chain, destroy_pool,
	pool_statistics, set_mapping_hysteresis, set_preferred_block_size,
};
pub use allocator::{bind_buffer_memory2, bind_image_memory2};
pub use allocator::{check_corruption, check_pool_corruption, set_debug_memory_settings};
pub use allocator::{external_memory_types, set_external_memory_types};
pub use allocator::{
	heap_budgets, refresh_budget, refresh_budget_with_context, set_current_frame_index,
};
pub use chain::AllocationChain;
mod import;
pub use import::{ImportMemoryInfo, ImportResource};
pub use metadata::AllocationAlgorithm;
pub use policy::{HostAccess, MemoryUsage};
pub use selector::MemoryTypeSelector;
pub use virtual_block::{
	PlacementStrategy, VirtualAllocation, VirtualAllocationInfo, VirtualBlock, VirtualBlockDetails,
	VirtualBlockStats,
};

use crate::{self as vk, load::CallError};

#[derive(Debug, PartialEq)]
pub enum AllocError {
	NoCompatibleMemoryType,
	InvalidProperties(&'static str),
	InvalidRequest(&'static str),
	InvalidRange,
	UnknownAllocation,
	UnknownPool,
	CorruptionDetected {
		allocation: u64,
		offset: u64,
	},
	WrongAllocator,
	AlreadyMapped,
	NotMapped,
	NotHostVisible,
	SerialExhausted,
	SynchronizationPoisoned,
	HostAllocationFailed,
	AllocationLimit,
	OutOfSpace,
	AllocationTooLarge {
		requested: u64,
		limit: u64,
	},
	UnsupportedCapability(&'static str),
	WrongResource,
	LiveAllocations,
	LivePools,
	Closed,
	Call(CallError),
	Output {
		kind: std::io::ErrorKind,
		code: Option<i32>,
	},
}

impl std::fmt::Display for AllocError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "allocation error: {self:?}")
	}
}

impl std::error::Error for AllocError {}

impl From<crate::ConstructionError> for AllocError {
	fn from(error: crate::ConstructionError) -> Self {
		Self::from(CallError::from(error))
	}
}

impl From<CallError> for AllocError {
	fn from(error: CallError) -> Self {
		Self::Call(error)
	}
}

/// Finite native memory priority in the inclusive range 0..=1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryPriority(u32);

impl MemoryPriority {
	/// Validate a native allocation priority.
	/// # Errors
	/// Returns InvalidRequest for NaN, infinities or values outside 0..=1.
	pub fn new(value: f32) -> Result<Self, AllocError> {
		if !value.is_finite() || !(0.0..=1.0).contains(&value) {
			return Err(AllocError::InvalidRequest(
				"memory priority must be finite and in 0..=1",
			));
		}
		Ok(Self(if value == 0.0 {
			0.0_f32.to_bits()
		} else {
			value.to_bits()
		}))
	}

	pub fn get(self) -> f32 {
		f32::from_bits(self.0)
	}
}

impl Default for MemoryPriority {
	fn default() -> Self {
		Self(0.5_f32.to_bits())
	}
}

/// Opt-in host-visible debug instrumentation, configured before native acquisition.
/// Guard margins follow VMA's coherent pooled-memory policy. Zero disables guards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DebugMemorySettings {
	/// Trailing guard bytes, a multiple of four. Separate native allocations have no guard tail.
	pub margin: u64,
	/// Initialize admitted host-visible allocation bytes with VMA's created pattern (0xDC).
	pub initialize_allocations: bool,
	/// Overwrite admitted host-visible bytes with VMA's destroyed pattern (0xEF) on release.
	pub poison_frees: bool,
}

/// Informative notifications for actual native memory allocation/free effects.
/// Callbacks do not allocate Rust metadata and do not own the native memory.
#[derive(Clone, Copy, Default)]
pub struct DeviceMemoryCallbacks {
	pub allocated: Option<
		unsafe extern "system" fn(vk::DeviceHandle, vk::DeviceMemory, u64, u32, *mut std::ffi::c_void),
	>,
	pub freed: Option<
		unsafe extern "system" fn(vk::DeviceHandle, vk::DeviceMemory, u64, u32, *mut std::ffi::c_void),
	>,
	pub p_user_data: *mut std::ffi::c_void,
}

/// Captured native identities. Manual metadata constructors do not invent parent identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocatorInfo {
	pub device: vk::DeviceHandle,
	pub instance: Option<vk::InstanceHandle>,
	pub physical_device: Option<vk::PhysicalDevice>,
	pub api_version: Option<u32>,
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
	/// Heap-sized default pools with pinned VMA growth, reuse and separate fallback.
	Automatic,
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

/// Managed-resource policy, including optional borrowed native allocation storage;
/// this Rust record is not part of the Vulkan ABI.
#[derive(Clone, Copy, Debug)]
pub struct AllocationCreateInfo<'pool> {
	/// Gate new native memory against owned cached/refreshed budgets; reuse needs no new budget.
	pub within_budget: bool,
	/// Force separate native storage, including in automatic custom pools.
	pub dedicated_memory: bool,
	/// Borrowed native allocation chain. Implies separate storage; never combined
	/// with a pool chain. Imports are attempted once and never initialized/poisoned.
	pub memory_allocate_next: Option<AllocationChain<'pool>>,
	/// Reuse existing pool storage only; never grow or fall back to separate native storage.
	pub never_allocate: bool,
	/// Reserve conservatively for alias resources and omit resource-specific dedication.
	pub can_alias: bool,
	/// Non-default priority requires captured enabled EXT_memory_priority.
	pub priority: MemoryPriority,
	/// Additional offset alignment; zero means only native/pool/atom constraints.
	/// Otherwise a power of two, combined with every mandatory alignment.
	pub min_allocation_alignment: u64,
	/// Fixed-type custom pool. Overrides `policy`; native type compatibility and
	/// required properties remain mandatory. No fallback outside this pool.
	///
	/// Pool policy storage cannot outlive or be used after destroying its pool:
	/// ```compile_fail,E0502
	/// # fn example(allocator: &mut vk::Allocator<'_>, pool: &mut vk::Pool) {
	/// let options = vk::mem::AllocationCreateInfo {
	///     pool: Some(pool),
	///     ..Default::default()
	/// };
	/// allocator.destroy_pool(pool).unwrap();
	/// assert!(!options.pool.unwrap().is_destroyed());
	/// # }
	/// ```
	pub pool: Option<&'pool Pool>,
	pub usage: MemoryUsage,
	/// Automatic host intent also prefers already mapped compatible blocks.
	/// Explicit usage retains mapping intent; absence of automatic intent prefers unmapped blocks.
	pub host_access: HostAccess,
	/// Permit automatic selection of non-host-visible memory; caller supplies staging.
	pub allow_transfer_instead: bool,
	/// Properties penalized during memory-type selection; never a hard exclusion.
	pub avoided_flags: vk::MemoryPropertyFlags,
	pub required_flags: vk::MemoryPropertyFlags,
	pub preferred_flags: vk::MemoryPropertyFlags,
	/// Intersected with queried native requirements; zero admits no memory types.
	pub memory_type_bits: u32,
	/// TLSF block search is forward except MinTime, which searches in reverse.
	/// Other strategies group host-visible blocks by mapping preference first.
	pub strategy: PlacementStrategy,
	/// Allocate downward in a single-block linear custom pool. Other routes reject it.
	pub upper_address: bool,
	pub policy: AllocationPolicy,
	/// Retain a mapping until explicit allocation release. Requires host-visible storage.
	pub persistent_map: bool,
}

impl Default for AllocationCreateInfo<'_> {
	fn default() -> Self {
		Self {
			within_budget: false,
			dedicated_memory: false,
			memory_allocate_next: None,
			never_allocate: false,
			can_alias: false,
			priority: MemoryPriority::default(),
			min_allocation_alignment: 0,
			pool: None,
			usage: MemoryUsage::Explicit,
			host_access: HostAccess::None,
			allow_transfer_instead: false,
			avoided_flags: vk::MemoryPropertyFlags::empty(),
			required_flags: vk::MemoryPropertyFlags::empty(),
			preferred_flags: vk::MemoryPropertyFlags::empty(),
			memory_type_bits: u32::MAX,
			strategy: PlacementStrategy::Default,
			upper_address: false,
			policy: AllocationPolicy::Separate,
			persistent_map: false,
		}
	}
}

/// Checked binding and mapping snapshot of one live allocation.
///
/// Offsets and sizes are native bytes. `size` is the queried memory requirement,
/// not necessarily the logical buffer size or a linear image's pixel extent.
/// Copying a snapshot does not retain its allocation, mapping or native objects.
/// Query again after mapping changes; never use a saved pointer after unmap/free.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocationInfo {
	pub device_memory: vk::DeviceMemory,
	pub offset: u64,
	pub size: u64,
	/// Includes non-coherent atom padding reserved to a pooled region.
	pub reserved_size: u64,
	pub memory_type_index: u32,
	pub heap_index: u32,
	/// Actual selected memory properties, rather than requested preferences.
	pub memory_properties: vk::MemoryPropertyFlags,
	/// Allocation-relative pointer, present only for this region's mapping hold.
	/// Observing this pointer does not acquire a hold or authorize dereferencing it.
	pub mapped_pointer: Option<*mut std::ffi::c_void>,
	/// Explicit mapping holds; persistent mapping is independent.
	pub map_count: u32,
	/// Size of the backing native memory object, including other pooled regions.
	pub native_size: u64,
	/// Separate native storage rather than a pooled placement (not proof of native dedication).
	pub is_separate: bool,
	/// Opaque consumer metadata. No dereference or ownership is inferred.
	pub user_data: usize,
}

/// Local accounting snapshot; excludes allocations made by other allocators/raw calls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AllocatorStats {
	pub allocation_count: usize,
	pub native_allocation_count: usize,
	pub pooled_block_count: usize,
	/// Sum of native requirements sizes, or full accounted external import extents.
	pub allocation_bytes: u64,
	/// Includes cache-atom padding reserved to live pooled regions.
	pub reserved_bytes: u64,
	pub native_bytes: u64,
	pub mapped_native_count: usize,
}

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

#[cfg(test)]
#[path = "../../test/rs/mem/test_host_allocator.rs"]
pub(crate) mod test_host_allocator;
