//! Rust-native, VMA-derived allocation, managed buffers/images and virtual placement.
//!
//! Free functions and [`Allocator`] methods share one implementation. For example,
//! [`create_buffer_with_options`] and [`Allocator::create_buffer_with_options`]
//! create, allocate and bind a managed buffer. The root [`crate::create_buffer`]
//! is the raw Vulkan operation and creates an unbound buffer.
//!
//! Use `use vk::mem as vma;` for a caller-local alias. This adds no wrapper,
//! runtime cost or second allocator type; [`crate::Allocator`] is [`Allocator`].
//! Full VMA equivalence, broad concurrent stress, multi-driver execution and
//! additional platforms remain
//! unqualified. See the [qualification report](https://github.com/realminc/vk/blob/main/docs/external/qualification.md).
//!
//! Every public field on a plain record has a consuming `set_*()` method alongside
//! direct field access. Default-backed records support `new().set_*()`;
//! [`AllocRequest`], [`ImportMemoryInfo`] and [`CacheRange`] take required inputs
//! in `new`. Setters retain borrows and leave validation to the using operation.
//! Editing a diagnostic snapshot changes only that record, never live allocator
//! state. Resource identities and owners retain their checked operation APIs.
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
pub use synchronized::{SynchronizedAllocator, SynchronizedVirtualBlock};
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

impl DebugMemorySettings {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`margin`](Self::margin) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_margin(mut self, value: u64) -> Self {
		self.margin = value;
		self
	}

	/// Set [`initialize_allocations`](Self::initialize_allocations) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_initialize_allocations(mut self, value: bool) -> Self {
		self.initialize_allocations = value;
		self
	}

	/// Set [`poison_frees`](Self::poison_frees) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_poison_frees(mut self, value: bool) -> Self {
		self.poison_frees = value;
		self
	}
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

impl DeviceMemoryCallbacks {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`allocated`](Self::allocated) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_allocated(
		mut self,
		value: Option<
			unsafe extern "system" fn(
				vk::DeviceHandle,
				vk::DeviceMemory,
				u64,
				u32,
				*mut std::ffi::c_void,
			),
		>,
	) -> Self {
		self.allocated = value;
		self
	}

	/// Set [`freed`](Self::freed) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_freed(
		mut self,
		value: Option<
			unsafe extern "system" fn(
				vk::DeviceHandle,
				vk::DeviceMemory,
				u64,
				u32,
				*mut std::ffi::c_void,
			),
		>,
	) -> Self {
		self.freed = value;
		self
	}

	/// Set [`p_user_data`](Self::p_user_data) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_p_user_data(mut self, value: *mut std::ffi::c_void) -> Self {
		self.p_user_data = value;
		self
	}
}

/// Captured native identities. Manual metadata constructors do not invent parent identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocatorInfo {
	pub device: vk::DeviceHandle,
	pub instance: Option<vk::InstanceHandle>,
	pub physical_device: Option<vk::PhysicalDevice>,
	pub api_version: Option<u32>,
}

impl AllocatorInfo {
	/// Set [`device`](Self::device) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_device(mut self, value: vk::DeviceHandle) -> Self {
		self.device = value;
		self
	}

	/// Set [`instance`](Self::instance) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_instance(mut self, value: Option<vk::InstanceHandle>) -> Self {
		self.instance = value;
		self
	}

	/// Set [`physical_device`](Self::physical_device) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_physical_device(mut self, value: Option<vk::PhysicalDevice>) -> Self {
		self.physical_device = value;
		self
	}

	/// Set [`api_version`](Self::api_version) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_api_version(mut self, value: Option<u32>) -> Self {
		self.api_version = value;
		self
	}
}

pub struct AllocRequest {
	pub size: u64,
	pub alignment: u64,
	pub memory_type_bits: u32,
	pub required_flags: vk::MemoryPropertyFlags,
	pub preferred_flags: vk::MemoryPropertyFlags,
}

impl AllocRequest {
	/// Construct explicit requirements with no required or preferred properties.
	/// The selector/allocation operation validates size, alignment and type compatibility.
	#[inline]
	#[must_use]
	pub fn new(size: u64, alignment: u64, memory_type_bits: u32) -> Self {
		Self {
			size,
			alignment,
			memory_type_bits,
			required_flags: vk::MemoryPropertyFlags::empty(),
			preferred_flags: vk::MemoryPropertyFlags::empty(),
		}
	}

	/// Set [`size`](Self::size) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_size(mut self, value: u64) -> Self {
		self.size = value;
		self
	}

	/// Set [`alignment`](Self::alignment) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_alignment(mut self, value: u64) -> Self {
		self.alignment = value;
		self
	}

	/// Set [`memory_type_bits`](Self::memory_type_bits) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_memory_type_bits(mut self, value: u32) -> Self {
		self.memory_type_bits = value;
		self
	}

	/// Set [`required_flags`](Self::required_flags) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_required_flags(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.required_flags = value;
		self
	}

	/// Set [`preferred_flags`](Self::preferred_flags) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_preferred_flags(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.preferred_flags = value;
		self
	}
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
	/// let options = vk::mem::AllocationCreateInfo::new().set_pool(Some(pool));
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

impl<'pool> AllocationCreateInfo<'pool> {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`within_budget`](Self::within_budget) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_within_budget(mut self, value: bool) -> Self {
		self.within_budget = value;
		self
	}

	/// Set [`dedicated_memory`](Self::dedicated_memory) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_dedicated_memory(mut self, value: bool) -> Self {
		self.dedicated_memory = value;
		self
	}

	/// Set [`memory_allocate_next`](Self::memory_allocate_next) and return the updated record.
	///
	/// The descriptor retains its chain storage borrow:
	/// ```compile_fail,E0597
	/// # fn example() {
	/// let options;
	/// {
	///     let record = vk::ExportMemoryAllocateInfo::new();
	///     // SAFETY: this correctly tagged record has no linked or nested storage.
	///     let chain = unsafe { vk::mem::AllocationChain::new_from_record(&record) }.unwrap();
	///     options = vk::mem::AllocationCreateInfo::new()
	///         .set_memory_allocate_next(Some(chain))
	///         .set_dedicated_memory(true);
	/// }
	/// assert!(options.memory_allocate_next.is_some());
	/// # }
	/// ```
	#[inline]
	#[must_use]
	pub fn set_memory_allocate_next(mut self, value: Option<AllocationChain<'pool>>) -> Self {
		self.memory_allocate_next = value;
		self
	}

	/// Set [`never_allocate`](Self::never_allocate) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_never_allocate(mut self, value: bool) -> Self {
		self.never_allocate = value;
		self
	}

	/// Set [`can_alias`](Self::can_alias) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_can_alias(mut self, value: bool) -> Self {
		self.can_alias = value;
		self
	}

	/// Set [`priority`](Self::priority) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_priority(mut self, value: MemoryPriority) -> Self {
		self.priority = value;
		self
	}

	/// Set [`min_allocation_alignment`](Self::min_allocation_alignment) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_min_allocation_alignment(mut self, value: u64) -> Self {
		self.min_allocation_alignment = value;
		self
	}

	/// Set [`pool`](Self::pool) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_pool(mut self, value: Option<&'pool Pool>) -> Self {
		self.pool = value;
		self
	}

	/// Set [`usage`](Self::usage) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_usage(mut self, value: MemoryUsage) -> Self {
		self.usage = value;
		self
	}

	/// Set [`host_access`](Self::host_access) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_host_access(mut self, value: HostAccess) -> Self {
		self.host_access = value;
		self
	}

	/// Set [`allow_transfer_instead`](Self::allow_transfer_instead) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_allow_transfer_instead(mut self, value: bool) -> Self {
		self.allow_transfer_instead = value;
		self
	}

	/// Set [`avoided_flags`](Self::avoided_flags) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_avoided_flags(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.avoided_flags = value;
		self
	}

	/// Set [`required_flags`](Self::required_flags) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_required_flags(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.required_flags = value;
		self
	}

	/// Set [`preferred_flags`](Self::preferred_flags) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_preferred_flags(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.preferred_flags = value;
		self
	}

	/// Set [`memory_type_bits`](Self::memory_type_bits) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_memory_type_bits(mut self, value: u32) -> Self {
		self.memory_type_bits = value;
		self
	}

	/// Set [`strategy`](Self::strategy) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_strategy(mut self, value: PlacementStrategy) -> Self {
		self.strategy = value;
		self
	}

	/// Set [`upper_address`](Self::upper_address) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_upper_address(mut self, value: bool) -> Self {
		self.upper_address = value;
		self
	}

	/// Set [`policy`](Self::policy) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_policy(mut self, value: AllocationPolicy) -> Self {
		self.policy = value;
		self
	}

	/// Set [`persistent_map`](Self::persistent_map) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_persistent_map(mut self, value: bool) -> Self {
		self.persistent_map = value;
		self
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

impl AllocationInfo {
	/// Set [`device_memory`](Self::device_memory) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_device_memory(mut self, value: vk::DeviceMemory) -> Self {
		self.device_memory = value;
		self
	}

	/// Set [`offset`](Self::offset) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_offset(mut self, value: u64) -> Self {
		self.offset = value;
		self
	}

	/// Set [`size`](Self::size) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_size(mut self, value: u64) -> Self {
		self.size = value;
		self
	}

	/// Set [`reserved_size`](Self::reserved_size) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_reserved_size(mut self, value: u64) -> Self {
		self.reserved_size = value;
		self
	}

	/// Set [`memory_type_index`](Self::memory_type_index) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_memory_type_index(mut self, value: u32) -> Self {
		self.memory_type_index = value;
		self
	}

	/// Set [`heap_index`](Self::heap_index) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_heap_index(mut self, value: u32) -> Self {
		self.heap_index = value;
		self
	}

	/// Set [`memory_properties`](Self::memory_properties) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_memory_properties(mut self, value: vk::MemoryPropertyFlags) -> Self {
		self.memory_properties = value;
		self
	}

	/// Set [`mapped_pointer`](Self::mapped_pointer) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_mapped_pointer(mut self, value: Option<*mut std::ffi::c_void>) -> Self {
		self.mapped_pointer = value;
		self
	}

	/// Set [`map_count`](Self::map_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_map_count(mut self, value: u32) -> Self {
		self.map_count = value;
		self
	}

	/// Set [`native_size`](Self::native_size) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_native_size(mut self, value: u64) -> Self {
		self.native_size = value;
		self
	}

	/// Set [`is_separate`](Self::is_separate) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_is_separate(mut self, value: bool) -> Self {
		self.is_separate = value;
		self
	}

	/// Set [`user_data`](Self::user_data) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_user_data(mut self, value: usize) -> Self {
		self.user_data = value;
		self
	}
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

impl AllocatorStats {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`allocation_count`](Self::allocation_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_allocation_count(mut self, value: usize) -> Self {
		self.allocation_count = value;
		self
	}

	/// Set [`native_allocation_count`](Self::native_allocation_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_native_allocation_count(mut self, value: usize) -> Self {
		self.native_allocation_count = value;
		self
	}

	/// Set [`pooled_block_count`](Self::pooled_block_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_pooled_block_count(mut self, value: usize) -> Self {
		self.pooled_block_count = value;
		self
	}

	/// Set [`allocation_bytes`](Self::allocation_bytes) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_allocation_bytes(mut self, value: u64) -> Self {
		self.allocation_bytes = value;
		self
	}

	/// Set [`reserved_bytes`](Self::reserved_bytes) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_reserved_bytes(mut self, value: u64) -> Self {
		self.reserved_bytes = value;
		self
	}

	/// Set [`native_bytes`](Self::native_bytes) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_native_bytes(mut self, value: u64) -> Self {
		self.native_bytes = value;
		self
	}

	/// Set [`mapped_native_count`](Self::mapped_native_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_mapped_native_count(mut self, value: usize) -> Self {
		self.mapped_native_count = value;
		self
	}
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

impl HeapBudget {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`size`](Self::size) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_size(mut self, value: u64) -> Self {
		self.size = value;
		self
	}

	/// Set [`limit`](Self::limit) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_limit(mut self, value: u64) -> Self {
		self.limit = value;
		self
	}

	/// Set [`budget`](Self::budget) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_budget(mut self, value: u64) -> Self {
		self.budget = value;
		self
	}

	/// Set [`usage`](Self::usage) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_usage(mut self, value: u64) -> Self {
		self.usage = value;
		self
	}

	/// Set [`native_bytes`](Self::native_bytes) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_native_bytes(mut self, value: u64) -> Self {
		self.native_bytes = value;
		self
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BudgetSnapshot {
	pub source: BudgetSource,
	pub heap_count: u32,
	pub heaps: [HeapBudget; 16],
}

impl BudgetSnapshot {
	/// Construct the record with its default state.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set [`source`](Self::source) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_source(mut self, value: BudgetSource) -> Self {
		self.source = value;
		self
	}

	/// Set [`heap_count`](Self::heap_count) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_heap_count(mut self, value: u32) -> Self {
		self.heap_count = value;
		self
	}

	/// Set [`heaps`](Self::heaps) and return the updated record.
	#[inline]
	#[must_use]
	pub fn set_heaps(mut self, value: [HeapBudget; 16]) -> Self {
		self.heaps = value;
		self
	}
}

#[cfg(test)]
#[path = "../../test/rs/mem/test_host_allocator.rs"]
pub(crate) mod test_host_allocator;
