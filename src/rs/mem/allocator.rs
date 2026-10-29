//! Separate and pooled ordinary buffers with explicit native lifetime management.
//!
//! Donor: VMA 3.4.0, commit 3aa921224c154a0d2c43912bc88e1c42ce1f7607,
//! `GetBufferMemoryRequirements` and `AllocateDedicatedMemory`. See LICENSE-VMA-MIT.
//! Adaptation: owned capability admission, checked tokens, fallible host reservation,
//! cached PFNs and explicit Rust error-bearing cleanup; no implicit native Drop.

use super::{
	AllocError, AllocRequest, Allocation, AllocationCreateInfo, AllocationInfo, AllocationPolicy,
	AllocatorStats, MemoryPriority, MemoryTypeSelector,
	block::{Block, BlockDomain, Mapping},
	granularity::{Granularity, ResourceClass},
	host::{HostMap, HostMemory, HostString, HostVec, Identity},
	metadata::{AllocationAlgorithm, Metadata, Placement},
};
use crate::{self as vk, Device, load::CallError};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Resource {
	Buffer(vk::Buffer),
	Image(vk::Image),
}

impl Resource {
	fn is_null(self) -> bool {
		match self {
			Self::Buffer(buffer) => buffer.is_null(),
			Self::Image(image) => image.is_null(),
		}
	}
}

fn import_resource(resource: Option<super::ImportResource>) -> Option<Resource> {
	resource.map(|resource| match resource {
		super::ImportResource::Buffer(buffer) => Resource::Buffer(buffer),
		super::ImportResource::Image(image) => Resource::Image(image),
	})
}

struct Live<'callbacks> {
	serial: u64,
	pool: Option<u64>,
	handle: vk::DeviceMemory,
	size: u64,
	reserved_size: u64,
	memory_type_index: u32,
	offset: u64,
	alignment: u64,
	map_count: u32,
	persistent: bool,
	resource: Option<Resource>,
	bound: bool,
	name: Option<HostString<'callbacks>>,
	user_data: usize,
	pinned: bool,
	can_alias: bool,
	imported: bool,
	plane: Option<vk::ImageAspectFlagBits>,
	export_types: vk::ExternalMemoryHandleTypeFlags,
	guard_size: u64,
	guard_offset: u64,
	debug_ready: bool,
	class: ResourceClass,
	placement: Option<Placement>,
	mapping: Mapping,
}

#[derive(Clone, Copy)]
enum RequirementsRoute {
	Legacy,
	Core,
	Khr,
}

/// Borrows one device; native memory is released only by explicit operations.
///
/// Each allocation has an allocator identity and generation. Dropping either
/// the allocator or a token can leak native objects; it never calls Vulkan.
pub struct Allocator<'device> {
	host: HostMemory<'device>,
	device: &'device Device,
	callbacks: Option<&'device vk::AllocationCallbacks>,
	device_memory_callbacks: Option<&'device super::DeviceMemoryCallbacks>,
	physical_device: Option<vk::PhysicalDevice>,
	instance_handle: Option<vk::InstanceHandle>,
	physical_properties: Option<vk::PhysicalDeviceProperties>,
	selector: MemoryTypeSelector,
	owner: Identity,
	next_serial: u64,
	live: HostMap<'device, Live<'device>>,
	pools: HostMap<'device, pool::PoolState<'device>>,
	blocks: HostVec<'device, Block<'device>>,
	native_count: usize,
	native_bytes: u64,
	heap_bytes: [u64; 16],
	heap_limits: [Option<u64>; 16],
	allocation_limit: u32,
	allocation_size_limit: Option<u64>,
	integrated: Option<bool>,
	buffer_device_address: bool,
	memory_priority: bool,
	protected_memory: bool,
	budget_query: Option<budget::BudgetQuery>,
	budget_cache: Option<super::BudgetSnapshot>,
	budget_operations: u32,
	current_frame_index: u32,
	debug_settings: super::DebugMemorySettings,
	external_types: [vk::ExternalMemoryHandleTypeFlags; 32],
	preferred_large_heap_block_size: u64,
	mapping_hysteresis: bool,
	#[cfg(all(windows, feature = "platform-win32"))]
	external_handles: HostVec<'device, external::CachedHandle>,
	granularity: Granularity,
	closed: bool,
	parallel: Option<std::ptr::NonNull<std::ffi::c_void>>,
	partition: parallel::Domain,
	partition_id: u64,
}

#[path = "synchronized/scope.rs"]
mod parallel;
pub(super) use parallel::{Selection, with_domains};

#[path = "external.rs"]
mod external;
#[cfg(unix)]
pub use external::get_memory_fd;
#[cfg(all(windows, feature = "platform-win32"))]
pub use external::get_memory_win32_handle;
pub use external::{external_memory_types, set_external_memory_types};

#[path = "guard.rs"]
mod guard;
pub use guard::{check_corruption, check_pool_corruption, set_debug_memory_settings};

#[path = "defragmentation.rs"]
mod defragmentation;
pub use defragmentation::{
	DefragmentationAlgorithm, DefragmentationContext, DefragmentationInfo, DefragmentationMove,
	DefragmentationPass, DefragmentationResource, DefragmentationStats, begin_defragmentation,
	begin_defragmentation_with_break, set_allocation_pinned,
};

#[path = "services.rs"]
mod services;
pub use services::{
	CacheRange, DetailedStatistics, MemoryStatistics, allocate_many, allocation_name, allocator_info,
	copy_from_allocation, copy_to_allocation, create_aliasing_buffer, create_aliasing_image,
	detailed_pool_statistics, detailed_statistics, flush_many, free_many, invalidate_many,
	memory_properties, memory_type_properties, physical_device_properties, pool_name,
	set_allocation_name, set_allocation_user_data, set_pool_name, write_statistics_json,
};

#[path = "request.rs"]
mod request;
pub use request::{
	allocate_dedicated, allocate_with_options, create_dedicated_buffer, create_dedicated_image,
	find_memory_type, find_memory_type_for_buffer_info, find_memory_type_for_image_info,
};

#[path = "binding.rs"]
mod binding;
pub use binding::{
	allocate_for_buffer, allocate_for_image, bind_buffer_memory, bind_buffer_memory2,
	bind_image_memory, bind_image_memory2,
};

#[path = "budget.rs"]
mod budget;
pub use budget::{
	TrimStats, budget_snapshot, heap_budgets, query_budget, refresh_budget,
	refresh_budget_with_context, set_current_frame_index, set_heap_limit, trim, trim_with_budget,
};

#[path = "pool.rs"]
mod pool;
pub use pool::{
	Pool, PoolCreateInfo, PoolStats, create_pool, create_pool_with_chain, destroy_pool,
	pool_statistics, set_mapping_hysteresis, set_preferred_block_size,
};

impl<'device> Allocator<'device> {
	/// Construct from caller-supplied physical-device metadata.
	///
	/// Prefer [`Self::new_from_physical_device`] for queried properties and the
	/// local native-allocation count limit. Managed buffers require the device's
	/// owned creation capabilities; raw imports without them support only the
	/// explicitly unsafe low-level allocation route.
	/// Without a buffer/image granularity limit, blocks conservatively exclude
	/// conflicting image tilings. Use [`Self::new_with_granularity`] to supply it.
	///
	/// # Safety
	/// Properties and atom size must belong to this device's physical device.
	/// Caller must enforce device allocation size/count limits and enabled features.
	pub unsafe fn new(
		device: &'device Device,
		properties: vk::PhysicalDeviceMemoryProperties,
		atom: u64,
	) -> Result<Self, AllocError> {
		let mut selector = MemoryTypeSelector::new(properties, atom)?;
		selector.device_coherent = super::resource::device_coherent_memory(device);
		require(device.fp().allocate_memory.is_some(), "vkAllocateMemory")?;
		require(device.fp().free_memory.is_some(), "vkFreeMemory")?;
		Ok(Self {
			host: HostMemory::default(),
			device,
			callbacks: None,
			device_memory_callbacks: None,
			physical_device: None,
			instance_handle: None,
			physical_properties: None,
			selector,
			owner: Identity::new()?,
			next_serial: 1,
			live: HostMap::new(HostMemory::default()),
			pools: HostMap::new(HostMemory::default()),
			blocks: HostVec::new(HostMemory::default()),
			native_count: 0,
			native_bytes: 0,
			heap_bytes: [0; 16],
			heap_limits: [None; 16],
			allocation_limit: u32::MAX,
			allocation_size_limit: None,
			integrated: None,
			buffer_device_address: super::resource::buffer_device_address(device),
			memory_priority: super::resource::memory_priority(device),
			protected_memory: super::resource::protected_memory(device),
			budget_query: None,
			budget_cache: None,
			budget_operations: 0,
			current_frame_index: 0,
			debug_settings: super::DebugMemorySettings::default(),
			external_types: [vk::ExternalMemoryHandleTypeFlags::empty(); 32],
			preferred_large_heap_block_size: 256 * 1024 * 1024,
			mapping_hysteresis: true,
			#[cfg(all(windows, feature = "platform-win32"))]
			external_handles: HostVec::new(HostMemory::default()),
			granularity: Granularity(None),
			closed: false,
			parallel: None,
			partition: parallel::Domain::Global,
			partition_id: 0,
		})
	}

	/// Construct from memory properties and both physical-device alignment limits.
	///
	/// Enables mixed buffer/linear/optimal image blocks with checked granularity
	/// page separation. Otherwise follows [`Self::new`].
	/// # Errors
	/// Rejects invalid memory properties, non-power-of-two limits or missing PFNs.
	/// # Safety
	/// Properties, `atom` (`nonCoherentAtomSize`) and `granularity`
	/// (`bufferImageGranularity`) must describe this device's physical device.
	/// The caller enforces native size/count limits, enabled features and lifetimes.
	pub unsafe fn new_with_granularity(
		device: &'device Device,
		properties: vk::PhysicalDeviceMemoryProperties,
		atom: u64,
		granularity: u64,
	) -> Result<Self, AllocError> {
		if !granularity.is_power_of_two() {
			return Err(AllocError::InvalidProperties(
				"buffer/image granularity must be a nonzero power of two",
			));
		}
		// SAFETY: identical supplied metadata and native lifetime contract.
		let mut allocator = unsafe { Self::new(device, properties, atom) }?;
		allocator.granularity = Granularity(Some(granularity));
		Ok(allocator)
	}

	/// Query properties and limits for the selected physical device.
	///
	/// Requires owned instance/device creation metadata and a compatible parent
	/// snapshot. It does not discover a logical device's physical-device identity.
	/// Queries the maximum allocation size when core 1.1 or enabled maintenance3
	/// provides it. Legacy 1.0 leaves this limit unknown; see [`Self::max_allocation_size`].
	/// The local count limit excludes allocations made by other allocators/raw calls.
	///
	/// # Safety
	/// `physical_device` must belong to `instance` and be the physical device used
	/// to create `device`. All contexts must remain live; native ownership and
	/// host synchronization remain the caller's responsibility. Unqueried size
	/// limits and resource valid usage must still be observed by the caller.
	pub unsafe fn new_from_physical_device(
		instance: &vk::Instance,
		physical_device: vk::PhysicalDevice,
		device: &'device Device,
	) -> Result<Self, AllocError> {
		let parent = instance
			.capabilities()
			.ok_or(AllocError::UnsupportedCapability(
				"instance creation metadata is required",
			))?;
		let capabilities = device
			.capabilities()
			.ok_or(AllocError::UnsupportedCapability(
				"device creation metadata is required",
			))?;
		if !capabilities.matches_instance(parent) {
			return Err(AllocError::InvalidProperties(
				"device/instance metadata disagree",
			));
		}
		if physical_device.is_null() {
			return Err(AllocError::InvalidProperties("null physical device"));
		}
		// SAFETY: caller guarantees the selected physical device and parent lifetime.
		let properties = unsafe { instance.get_physical_device_properties(physical_device) }?;
		// These are property-query availability gates, not feature enablement.
		// Use conservative owned provenance; never infer command legality from a PFN.
		let maintenance3 = capabilities
			.device_extensions()
			.is_some_and(|mut extensions| extensions.any(|name| name == c"VK_KHR_maintenance3"));
		let allocation_size_limit = if capabilities.api_version() >= vk::API_VERSION_1_1 || maintenance3
		{
			let mut limits = vk::PhysicalDeviceMaintenance3Properties::default();
			let mut output = vk::PhysicalDeviceProperties2 {
				p_next: std::ptr::from_mut(&mut limits).cast(),
				..Default::default()
			};
			if parent.instance_api_version() >= vk::API_VERSION_1_1 {
				// SAFETY: admitted core query, matching physical device and initialized chain.
				unsafe { instance.get_physical_device_properties2(physical_device, &mut output) }?;
			} else if parent
				.instance_extensions()
				.any(|name| name == c"VK_KHR_get_physical_device_properties2")
			{
				// SAFETY: exact KHR query is admitted by the enabled instance extension;
				// the device enabled maintenance3, which supplies this property record.
				unsafe { instance.get_physical_device_properties2_khr(physical_device, &mut output) }?;
			} else {
				return Err(AllocError::UnsupportedCapability(
					"maintenance3 requires properties2",
				));
			}
			if limits.max_memory_allocation_size == 0 {
				return Err(AllocError::InvalidProperties(
					"zero native allocation-size limit",
				));
			}
			Some(limits.max_memory_allocation_size)
		} else {
			None
		};
		// SAFETY: same live physical device.
		let memory = unsafe { instance.get_physical_device_memory_properties(physical_device) }?;
		if properties.limits.max_memory_allocation_count == 0 {
			return Err(AllocError::InvalidProperties(
				"zero native allocation limit",
			));
		}
		// SAFETY: metadata was queried from the caller's matching physical device.
		let mut allocator = unsafe {
			Self::new_with_granularity(
				device,
				memory,
				properties.limits.non_coherent_atom_size,
				properties.limits.buffer_image_granularity,
			)
		}?;
		allocator.integrated = Some(properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU);
		allocator.physical_device = Some(physical_device);
		allocator.budget_query = budget::capture_query(instance, physical_device);
		allocator.instance_handle = Some(instance.handle());
		allocator.physical_properties = Some(properties);
		allocator.allocation_limit = properties.limits.max_memory_allocation_count;
		allocator.allocation_size_limit = allocation_size_limit;
		// Select/validate the exact admitted requirements command before publication.
		allocator.buffer_route()?;
		Ok(allocator)
	}

	/// Construct with borrowed callbacks for native effects and CPU metadata.
	/// Callback storage is borrowed for the allocator's complete borrow lifetime.
	/// # Errors
	/// Same metadata/PFN errors as `new`; rejects missing host allocation/free functions.
	/// # Safety
	/// All `new` requirements apply. Callbacks/user data must remain valid and
	/// satisfy Vulkan alignment, thread, reentrancy and non-unwinding contracts.
	/// They are used consistently for native allocation, creation and rollback;
	/// CPU metadata uses the allocation/free pair with OBJECT scope. The inline
	/// allocator and non-owning tokens need no heap identity. Returned Vec output
	/// collections and caller-owned writers retain their Rust allocation policy.
	/// Callback free may run while host metadata is dropped; it never frees
	/// native Vulkan objects implicitly.
	pub unsafe fn new_with_callbacks(
		device: &'device Device,
		properties: vk::PhysicalDeviceMemoryProperties,
		atom: u64,
		callbacks: &'device vk::AllocationCallbacks,
	) -> Result<Self, AllocError> {
		// SAFETY: caller supplies the canonical native metadata contract.
		let mut allocator = unsafe { Self::new(device, properties, atom) }?;
		// SAFETY: fresh empty allocator; caller retains complete callback validity.
		unsafe { allocator.initialize_callbacks(callbacks) }?;
		Ok(allocator)
	}

	/// Query limits and retain matching caller-owned native allocation callbacks.
	/// # Errors
	/// Same provenance/property/PFN errors as `new_from_physical_device`.
	/// # Safety
	/// Match the physical-device constructor's identity/lifetime contract and the
	/// callback lifetime/alignment/thread/reentrancy contract of `new_with_callbacks`.
	pub unsafe fn new_from_physical_device_with_callbacks(
		instance: &vk::Instance,
		physical_device: vk::PhysicalDevice,
		device: &'device Device,
		callbacks: &'device vk::AllocationCallbacks,
	) -> Result<Self, AllocError> {
		// SAFETY: same native identity and queried-property contract; no storage is allocated yet.
		let mut allocator =
			unsafe { Self::new_from_physical_device(instance, physical_device, device) }?;
		// SAFETY: fresh queried allocator; caller retains complete callback validity.
		unsafe { allocator.initialize_callbacks(callbacks) }?;
		Ok(allocator)
	}

	unsafe fn initialize_callbacks(
		&mut self,
		callbacks: &'device vk::AllocationCallbacks,
	) -> Result<(), AllocError> {
		debug_assert!(self.live.is_empty() && self.pools.is_empty() && self.blocks.is_empty());
		// SAFETY: same borrowed caller callback/thread contract, now used for
		// every allocator-owned host buffer as well as native effects.
		let host = unsafe { HostMemory::new_from_callbacks(callbacks) }?;
		self.host = host;
		self.live = HostMap::new(host);
		self.pools = HostMap::new(host);
		self.blocks = HostVec::new(host);
		#[cfg(all(windows, feature = "platform-win32"))]
		{
			self.external_handles = HostVec::new(host);
		}
		self.callbacks = Some(callbacks);
		Ok(())
	}

	pub fn live_count(&self) -> usize {
		self.live.len()
	}

	pub fn is_closed(&self) -> bool {
		self.closed
	}

	/// Queried native per-allocation size ceiling, or unknown on the caller-supplied
	/// metadata route and legacy 1.0 without admitted maintenance3 properties.
	/// A known ceiling is not a promise of available heap space or allocation success.
	pub fn max_allocation_size(&self) -> Option<u64> {
		self.allocation_size_limit
	}

	/// Close only when all allocations have been explicitly released; repeat close succeeds.
	pub fn close(&mut self) -> Result<(), AllocError> {
		close(self)
	}

	/// Allocate separate native storage without a resource-specific allocation chain.
	/// # Safety
	/// Request must satisfy resource requirements and device size/count limits.
	/// External and required-dedication semantics are excluded. Protected storage
	/// requires captured enabled feature metadata and explicit required flags. Owned
	/// core/KHR device-address enablement automatically flags raw storage.
	pub unsafe fn allocate(&mut self, request: &AllocRequest) -> Result<Allocation, AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { allocate(self, request) }
	}

	/// # Safety
	/// Destroy every bound resource and complete all GPU/host accesses first.
	pub unsafe fn free(&mut self, allocation: &mut Allocation) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { free(self, allocation) }
	}

	/// Acquire one explicit mapping hold; every successful call needs one unmap.
	/// # Safety
	/// Synchronize host/GPU access; returned pointers cannot outlive the mapping.
	pub unsafe fn map(
		&mut self,
		allocation: &Allocation,
	) -> Result<*mut std::ffi::c_void, AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { map(self, allocation) }
	}

	/// # Safety
	/// No pointer into this mapping may be used after unmapping.
	pub unsafe fn unmap(&mut self, allocation: &Allocation) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { unmap(self, allocation) }
	}

	/// # Safety
	/// Mapping and accesses must satisfy Vulkan host/device synchronization rules.
	pub unsafe fn flush(
		&self,
		allocation: &Allocation,
		offset: u64,
		size: u64,
	) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { flush(self, allocation, offset, size) }
	}

	/// # Safety
	/// Complete device writes before invalidating and do not race host accesses.
	pub unsafe fn invalidate(
		&self,
		allocation: &Allocation,
		offset: u64,
		size: u64,
	) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { invalidate(self, allocation, offset, size) }
	}

	/// Create, allocate and bind an ordinary buffer transactionally.
	/// # Safety
	/// Info must satisfy device limits/valid usage. Sparse, external
	/// and arbitrary pNext chains are not admitted. Protected resources need captured
	/// enabled feature metadata and matching creation flags. Device-address usage requires
	/// captured enabled core/KHR feature metadata.
	pub unsafe fn create_buffer(
		&mut self,
		info: &vk::BufferCreateInfo,
		required: vk::MemoryPropertyFlags,
		preferred: vk::MemoryPropertyFlags,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { create_buffer(self, info, required, preferred) }
	}

	/// Destroy a matching managed buffer before freeing its allocation.
	/// # Safety
	/// Complete every CPU/GPU use; do not separately destroy this buffer first.
	/// The buffer must use this allocator's callbacks; otherwise destroy it directly then free.
	pub unsafe fn destroy_buffer(
		&mut self,
		buffer: vk::Buffer,
		allocation: &mut Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical operation's native guarantees.
		unsafe { destroy_buffer(self, buffer, allocation) }
	}

	/// Create and bind an ordinary buffer using explicit separate/pooled policy.
	/// # Safety
	/// Same native valid-usage, completion and synchronization obligations as `create_buffer`.
	pub unsafe fn create_buffer_with_options(
		&mut self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: caller supplies the canonical operation's guarantees.
		unsafe { create_buffer_with_options(self, info, options) }
	}

	/// Create and bind an image using separate storage and explicit memory properties.
	/// # Safety
	/// All image/create-chain valid usage and host/GPU lifetimes remain caller obligations.
	pub unsafe fn create_image(
		&mut self,
		info: &vk::ImageCreateInfo,
		required: vk::MemoryPropertyFlags,
		preferred: vk::MemoryPropertyFlags,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical contract to the canonical operation.
		unsafe { create_image(self, info, required, preferred) }
	}

	/// Create, allocate and bind an image using explicit or automatic memory policy.
	/// # Safety
	/// Match native image, feature, pointer-chain and synchronization valid usage.
	pub unsafe fn create_image_with_options(
		&mut self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical contract to the canonical operation.
		unsafe { create_image_with_options(self, info, options) }
	}

	/// Destroy a managed image and release its allocation after exact completion.
	/// # Safety
	/// Destroy views first; complete all host/GPU uses and end explicit maps.
	/// The image must use this allocator's callbacks; otherwise destroy it directly then free.
	pub unsafe fn destroy_image(
		&mut self,
		image: vk::Image,
		allocation: &mut Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical contract to the canonical operation.
		unsafe { destroy_image(self, image, allocation) }
	}

	/// Observe a current explicit or persistent mapping without acquiring a hold.
	pub fn mapped_pointer(
		&self,
		allocation: &Allocation,
	) -> Result<Option<*mut std::ffi::c_void>, AllocError> {
		mapped_pointer(self, allocation)
	}

	/// Query checked binding metadata and this region's current mapping without native calls.
	/// # Errors
	/// Returns `Closed`, `WrongAllocator` or `UnknownAllocation` for invalid identity/state.
	pub fn allocation_info(&self, allocation: &Allocation) -> Result<AllocationInfo, AllocError> {
		allocation_info(self, allocation)
	}

	pub fn statistics(&self) -> AllocatorStats {
		statistics(self)
	}

	fn ensure_open(&self) -> Result<(), AllocError> {
		if self.closed {
			Err(AllocError::Closed)
		} else {
			Ok(())
		}
	}

	fn parallel_state(&self) -> Option<&parallel::Shared<'device>> {
		// SAFETY: only with_pools installs this pointer in a privately owned child.
		// The stable callback-owned context outlives every child, operation and
		// unwind restoration; children cannot escape the borrowed scope.
		self
			.parallel
			.map(|pointer| unsafe { pointer.cast::<parallel::Shared<'device>>().as_ref() })
	}

	fn total_native_count(&self) -> usize {
		self
			.parallel_state()
			.map_or(self.native_count, parallel::Shared::native_count)
	}

	fn release_resource_claim(&self, allocation: &Allocation) {
		if let Some(shared) = self.parallel_state() {
			let live = &self.live[&allocation.serial];
			shared.release_resource(live.resource, live.plane, allocation.serial);
		}
	}

	fn ensure_allocation(&self) -> Result<(), AllocError> {
		if matches!(self.partition, parallel::Domain::Block(_)) {
			Err(AllocError::InvalidRequest(
				"block scope cannot allocate, free or trim storage",
			))
		} else {
			Ok(())
		}
	}

	fn ensure_default(&self) -> Result<(), AllocError> {
		self.ensure_allocation()?;
		if matches!(self.partition, parallel::Domain::Pool(_)) {
			Err(AllocError::InvalidRequest(
				"pool scope requires its selected pool",
			))
		} else {
			Ok(())
		}
	}

	fn ensure_global(&self) -> Result<(), AllocError> {
		if self.parallel.is_some() {
			Err(AllocError::InvalidRequest(
				"allocator-wide operation inside pool scope",
			))
		} else {
			Ok(())
		}
	}

	fn token(&self, serial: u64) -> Result<Allocation, AllocError> {
		let live = self
			.live
			.get(&serial)
			.ok_or(AllocError::UnknownAllocation)?;
		let memory = self.selector.properties.memory_types[live.memory_type_index as usize];
		let block_size = if live.placement.is_some() {
			self
				.blocks
				.iter()
				.find(|block| block.handle == live.handle)
				.ok_or(AllocError::UnknownAllocation)?
				.size
		} else {
			live.size
		};
		Ok(Allocation {
			handle: live.handle,
			size: live.size,
			reserved_size: live.reserved_size,
			offset: live.offset,
			block_size,
			memory_type_index: live.memory_type_index,
			heap_index: memory.heap_index,
			flags: memory.property_flags,
			atom: self.selector.atom,
			owner: self.owner,
			serial,
		})
	}

	fn validate(&self, allocation: &Allocation) -> Result<&Live<'device>, AllocError> {
		self.ensure_open()?;
		if self.owner != allocation.owner {
			return Err(AllocError::WrongAllocator);
		}
		self
			.live
			.get(&allocation.serial)
			.filter(|live| live.serial == allocation.serial && live.handle == allocation.handle)
			.ok_or(AllocError::UnknownAllocation)
	}

	fn reserve(&mut self) -> Result<(), AllocError> {
		self.ensure_open()?;
		self.ensure_allocation()?;
		if let Some(shared) = self.parallel_state() {
			self.next_serial = shared.reserve_publication()?;
		}
		self
			.next_serial
			.checked_add(1)
			.ok_or(AllocError::SerialExhausted)?;
		self
			.live
			.try_reserve(1)
			.map_err(|_| AllocError::HostAllocationFailed)
	}

	fn validate_size(&self, size: u64) -> Result<(), AllocError> {
		if let Some(limit) = self.allocation_size_limit
			&& size > limit
		{
			return Err(AllocError::AllocationTooLarge {
				requested: size,
				limit,
			});
		}
		Ok(())
	}

	fn check_native(&self, size: u64, index: u32) -> Result<(), AllocError> {
		self.validate_size(size)?;
		if self.native_count >= self.allocation_limit as usize {
			return Err(AllocError::AllocationLimit);
		}
		let heap = self.selector.properties.memory_types[index as usize].heap_index as usize;
		let available = self.heap_limits[heap]
			.unwrap_or(self.selector.properties.memory_heaps[heap].size)
			- self.heap_bytes[heap];
		if size > available {
			return Err(AllocError::OutOfSpace);
		}
		self
			.native_bytes
			.checked_add(size)
			.ok_or(AllocError::InvalidRange)?;
		Ok(())
	}

	fn add_native(&mut self, size: u64, index: u32, memory: vk::DeviceMemory) {
		let heap = self.selector.properties.memory_types[index as usize].heap_index as usize;
		self.native_count += 1;
		self.native_bytes += size;
		self.heap_bytes[heap] += size;
		if let Some(callbacks) = self.device_memory_callbacks
			&& let Some(callback) = callbacks.allocated
		{
			// SAFETY: callback registration establishes live user data, no unwinding/reentrancy and permitted thread access.
			unsafe {
				callback(
					self.device.handle(),
					memory,
					size,
					index,
					callbacks.p_user_data,
				)
			};
		}
	}

	fn remove_native(&mut self, size: u64, index: u32, memory: vk::DeviceMemory) {
		#[cfg(all(windows, feature = "platform-win32"))]
		{
			let before = self.external_handles.len();
			self.external_handles.retain(|entry| entry.memory != memory);
			if let Some(shared) = self.parallel_state() {
				shared.external_removed(before - self.external_handles.len());
			}
		}
		let heap = self.selector.properties.memory_types[index as usize].heap_index as usize;
		if let Some(shared) = self.parallel_state() {
			shared.release_native(size, heap);
		}
		self.native_count -= 1;
		self.native_bytes -= size;
		self.heap_bytes[heap] -= size;
		if let Some(callbacks) = self.device_memory_callbacks
			&& let Some(callback) = callbacks.freed
		{
			// SAFETY: retained registration remains valid through this actual native free notification.
			unsafe {
				callback(
					self.device.handle(),
					memory,
					size,
					index,
					callbacks.p_user_data,
				)
			};
		}
	}

	fn mapping(&self, allocation: &Allocation) -> &Mapping {
		if self.live[&allocation.serial].placement.is_some() {
			&self
				.blocks
				.iter()
				.find(|b| b.handle == allocation.handle)
				.expect("live block")
				.mapping
		} else {
			&self.live[&allocation.serial].mapping
		}
	}

	fn mapping_mut(&mut self, allocation: &Allocation) -> &mut Mapping {
		if self.live[&allocation.serial].placement.is_some() {
			&mut self
				.blocks
				.iter_mut()
				.find(|b| b.handle == allocation.handle)
				.expect("live block")
				.mapping
		} else {
			&mut self
				.live
				.get_mut(&allocation.serial)
				.expect("validated live region")
				.mapping
		}
	}

	fn contains_memory(&self, memory: vk::DeviceMemory) -> bool {
		self.blocks.iter().any(|b| b.handle == memory)
			|| self.live.values().any(|live| live.handle == memory)
	}

	fn buffer_route(&self) -> Result<RequirementsRoute, AllocError> {
		self.requirements_route(false)
	}

	fn requirements_route(&self, image: bool) -> Result<RequirementsRoute, AllocError> {
		let capabilities = self
			.device
			.capabilities()
			.ok_or(AllocError::UnsupportedCapability(
				"managed resources require device creation metadata",
			))?;
		let fp = self.device.fp();
		let (core, khr, legacy) = if image {
			(
				(
					fp.get_image_memory_requirements2.is_some(),
					"vkGetImageMemoryRequirements2",
				),
				(
					fp.get_image_memory_requirements2_khr.is_some(),
					"vkGetImageMemoryRequirements2KHR",
				),
				(
					fp.get_image_memory_requirements.is_some(),
					"vkGetImageMemoryRequirements",
				),
			)
		} else {
			(
				(
					fp.get_buffer_memory_requirements2.is_some(),
					"vkGetBufferMemoryRequirements2",
				),
				(
					fp.get_buffer_memory_requirements2_khr.is_some(),
					"vkGetBufferMemoryRequirements2KHR",
				),
				(
					fp.get_buffer_memory_requirements.is_some(),
					"vkGetBufferMemoryRequirements",
				),
			)
		};
		if capabilities.api_version() >= vk::API_VERSION_1_1 {
			require(core.0, core.1)?;
			return Ok(RequirementsRoute::Core);
		}
		if super::resource::extension(self.device, c"VK_KHR_dedicated_allocation") {
			if !super::resource::extension(self.device, c"VK_KHR_get_memory_requirements2") {
				return Err(AllocError::UnsupportedCapability(
					"dedication requires VK_KHR_get_memory_requirements2",
				));
			}
			require(khr.0, khr.1)?;
			Ok(RequirementsRoute::Khr)
		} else {
			require(legacy.0, legacy.1)?;
			Ok(RequirementsRoute::Legacy)
		}
	}
}

fn require(present: bool, name: &'static str) -> Result<(), AllocError> {
	if present {
		Ok(())
	} else {
		Err(CallError::MissingCommand(name).into())
	}
}

/// Checked shutdown after all regions and their blocks have been explicitly released.
pub fn close(allocator: &mut Allocator<'_>) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	if !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	if !allocator.pools.is_empty() {
		return Err(AllocError::LivePools);
	}
	let mut snapshot = budget::budget_snapshot(allocator);
	pool::trim_blocks(allocator, &mut snapshot, true);
	allocator.closed = true;
	Ok(())
}

/// Allocate separate native storage with automatically admitted allocation flags.
/// # Safety
/// Requirements, size/count limits and enabled features must be valid. Required
/// dedication and per-request import chains are excluded. Configured exports
/// apply to the selected type. Protected memory requires
/// captured enabled feature metadata. When owned device
/// metadata enables core/KHR buffer device address, raw storage carries its flag.
pub unsafe fn allocate(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
) -> Result<Allocation, AllocError> {
	allocator.ensure_default()?;
	// SAFETY: caller provides the requirements and ordinary allocation semantics.
	let mut allocation = unsafe {
		allocate_storage(
			allocator,
			request,
			StorageOptions {
				chain: Default::default(),
				dedicated: None,
				device_address: allocator.buffer_device_address,
				priority: MemoryPriority::default(),
				within_budget: false,
			},
			vk::MemoryPropertyFlags::empty(),
		)
	}?;
	// SAFETY: new unbound raw storage; this helper owns unpublished initialization/rollback.
	if let Err(error) = unsafe { guard::initialize(allocator, &allocation) } {
		// SAFETY: initialization failed before publication; no native resource uses exist.
		unsafe { rollback_allocation(allocator, &mut allocation) }
			.expect("unpublished debug initialization rollback");
		return Err(error);
	}
	Ok(allocation)
}

#[derive(Clone, Copy)]
struct StorageOptions<'a> {
	chain: super::chain::ChainData,
	dedicated: Option<&'a vk::MemoryDedicatedAllocateInfo<'a>>,
	device_address: bool,
	priority: MemoryPriority,
	within_budget: bool,
}

unsafe fn allocate_native(
	allocator: &mut Allocator<'_>,
	size: u64,
	index: u32,
	storage: StorageOptions<'_>,
) -> Result<vk::DeviceMemory, AllocError> {
	if storage.within_budget {
		budget::admit_native(allocator, index, size)?;
	}
	external::allocation_exports(allocator, index, storage.chain)?;
	let export = vk::ExportMemoryAllocateInfo {
		handle_types: allocator.external_types[index as usize],
		p_next: storage.chain.next,
		..Default::default()
	};
	let tail = if export.handle_types.as_raw() != 0 {
		std::ptr::from_ref(&export).cast()
	} else {
		storage.chain.next
	};
	let mut dedicated = vk::MemoryDedicatedAllocateInfo {
		buffer: storage
			.dedicated
			.map_or(vk::Buffer::NULL, |info| info.buffer),
		image: storage.dedicated.map_or(vk::Image::NULL, |info| info.image),
		..Default::default()
	};
	dedicated.p_next = tail;
	let tail = if storage.dedicated.is_some() {
		std::ptr::from_ref(&dedicated).cast()
	} else {
		tail
	};
	if let Some(flags) = storage.chain.flags
		&& storage.device_address
		&& flags.as_raw() & vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_BIT.as_raw() == 0
	{
		return Err(AllocError::InvalidRequest(
			"explicit allocation flags omit required device address",
		));
	}
	let flags = vk::MemoryAllocateFlagsInfo {
		flags: vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_BIT,
		p_next: tail,
		..Default::default()
	};
	let next = if storage.device_address && storage.chain.flags.is_none() {
		std::ptr::from_ref(&flags).cast()
	} else {
		tail
	};
	let priority = vk::MemoryPriorityAllocateInfoEXT {
		priority: storage.priority.get(),
		p_next: next,
		..Default::default()
	};
	let info = vk::MemoryAllocateInfo {
		allocation_size: if storage.chain.import_kind == super::chain::ImportKind::MetalTexture {
			0
		} else {
			size
		},
		memory_type_index: index,
		p_next: if allocator.memory_priority {
			std::ptr::from_ref(&priority).cast()
		} else {
			next
		},
		..Default::default()
	};
	let reservation = allocator
		.parallel_state()
		.map(|shared| {
			let heap = allocator.selector.properties.memory_types[index as usize].heap_index as usize;
			shared.reserve_native(size, heap, storage.within_budget)
		})
		.transpose()?;
	// SAFETY: caller preflights allocation semantics; stack chains and retained callbacks live through this synchronous call.
	let handle = unsafe { allocator.device.allocate_memory(&info, allocator.callbacks) }?;
	if let Some(reservation) = reservation
		&& !handle.is_null()
	{
		reservation.commit();
	}
	Ok(handle)
}

unsafe fn allocate_storage(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	storage: StorageOptions<'_>,
	avoided: vk::MemoryPropertyFlags,
) -> Result<Allocation, AllocError> {
	allocator.ensure_open()?;
	let imported = storage.chain.import_info;
	if let Some(info) = imported {
		if request.size > info.allocation_size {
			return Err(AllocError::InvalidRequest(
				"resource exceeds imported payload",
			));
		}
		if storage.chain.import_kind == super::chain::ImportKind::Collection
			&& request.size != info.allocation_size
		{
			return Err(AllocError::InvalidRequest(
				"collection requires exact resource size",
			));
		}
		if let Some(resource) = import_resource(info.resource)
			&& binding::associated(allocator, resource)
		{
			return Err(AllocError::InvalidRequest(
				"imported resource already has an allocation",
			));
		}
	}
	let claim = imported
		.and_then(|info| import_resource(info.resource))
		.map(|resource| parallel::claim(allocator, resource, None, None))
		.transpose()?
		.flatten();
	let request = &AllocRequest {
		size: imported.map_or(request.size, |info| info.allocation_size),
		memory_type_bits: request.memory_type_bits
			& imported.map_or(u32::MAX, |info| info.memory_type_bits),
		..*request
	};
	allocator.validate_size(request.size)?;
	if !allocator.protected_memory
		&& request.required_flags.as_raw() & vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw() != 0
	{
		return Err(AllocError::UnsupportedCapability("protected allocation"));
	}
	if allocator.native_count >= allocator.allocation_limit as usize {
		return Err(AllocError::AllocationLimit);
	}
	allocator.reserve()?;
	let mut bits = request.memory_type_bits;
	let mut last = None;
	loop {
		let candidate = AllocRequest {
			memory_type_bits: bits,
			..*request
		};
		let index = match allocator
			.selector
			.find_memory_type_with_preferences(&candidate, avoided)
		{
			Ok(index) => index,
			Err(error) => return Err(last.unwrap_or(error)),
		};
		bits &= !(1 << index);
		if let Err(error) = allocator.check_native(request.size, index) {
			if error == AllocError::OutOfSpace {
				last = Some(error);
				continue;
			}
			return Err(error);
		}
		let exports = external::allocation_exports(allocator, index, storage.chain)?;
		// SAFETY: matching requirements, admitted chains and reserved bookkeeping.
		let handle = match unsafe { allocate_native(allocator, request.size, index, storage) } {
			Ok(handle) => handle,
			Err(error)
				if !storage.chain.imports
					&& (matches!(&error, AllocError::Call(native) if recoverable(native))
						|| error == AllocError::OutOfSpace) =>
			{
				last = Some(error);
				continue;
			}
			Err(error) => return Err(error),
		};
		if handle.is_null() || allocator.contains_memory(handle) {
			return Err(
				CallError::InvalidOutput("allocator returned a null or already-live memory handle").into(),
			);
		}
		allocator.add_native(request.size, index, handle);
		let allocation = publish(
			allocator,
			Region {
				handle,
				index,
				size: request.size,
				reserved_size: request.size,
				offset: 0,
				alignment: request.alignment,
				block_size: request.size,
				placement: None,
			},
		);
		let live = allocator
			.live
			.get_mut(&allocation.serial)
			.expect("new separate allocation");
		live.imported = storage.chain.imports;
		live.resource = imported.and_then(|info| import_resource(info.resource));
		live.export_types = exports;
		if let Some(claim) = claim {
			claim.commit(allocation.serial);
		}
		return Ok(allocation);
	}
}

fn recoverable(error: &CallError) -> bool {
	matches!(error, CallError::Vulkan(result) if *result == vk::Result::ERROR_OUT_OF_DEVICE_MEMORY || *result == vk::Result::ERROR_OUT_OF_HOST_MEMORY)
}

// Publication is infallible after tracking reservation and native/placement success.
struct Region {
	handle: vk::DeviceMemory,
	index: u32,
	size: u64,
	reserved_size: u64,
	offset: u64,
	alignment: u64,
	block_size: u64,
	placement: Option<Placement>,
}

fn memory_flags(allocator: &Allocator<'_>, index: u32) -> vk::MemoryPropertyFlags {
	allocator.selector.properties.memory_types[index as usize].property_flags
}

fn publish(allocator: &mut Allocator<'_>, region: Region) -> Allocation {
	let Region {
		handle,
		index,
		size,
		reserved_size,
		offset,
		alignment,
		block_size,
		placement,
	} = region;
	if placement.is_some() {
		allocator
			.blocks
			.iter_mut()
			.find(|block| block.handle == handle)
			.expect("published pooled block")
			.mapping
			.post_alloc();
	}
	let serial = allocator.next_serial;
	allocator.next_serial += 1;
	if let Some(shared) = allocator.parallel_state() {
		shared.publish();
	}
	allocator.budget_operations = allocator.budget_operations.saturating_add(1);
	allocator.live.insert_reserved(
		serial,
		Live {
			serial,
			pool: None,
			handle,
			size,
			reserved_size,
			memory_type_index: index,
			offset,
			alignment,
			map_count: 0,
			persistent: false,
			resource: None,
			bound: false,
			name: None,
			user_data: 0,
			pinned: false,
			can_alias: false,
			imported: false,
			plane: None,
			export_types: vk::ExternalMemoryHandleTypeFlags::empty(),
			guard_size: if allocator.debug_settings.margin != 0
				&& placement.is_some()
				&& guard::eligible(memory_flags(allocator, index))
			{
				allocator.debug_settings.margin
			} else {
				0
			},
			guard_offset: if allocator.debug_settings.margin != 0
				&& placement.is_some()
				&& guard::eligible(memory_flags(allocator, index))
			{
				size.checked_add(3).expect("preflighted guard extent") & !3
			} else {
				0
			},
			debug_ready: false,
			class: ResourceClass::Unknown,
			placement,
			mapping: Mapping::default(),
		},
	);
	let memory = allocator.selector.properties.memory_types[index as usize];
	Allocation {
		handle,
		size,
		reserved_size,
		offset,
		block_size,
		memory_type_index: index,
		heap_index: memory.heap_index,
		flags: memory.property_flags,
		atom: allocator.selector.atom,
		owner: allocator.owner,
		serial,
	}
}

unsafe fn rollback_allocation(
	allocator: &mut Allocator<'_>,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	allocator.validate(allocation)?;
	// Unpublished storage has no consumer data to diagnose or poison; debug setup
	// may itself have failed. Never repeat that failed operation during rollback.
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("unpublished allocation")
		.debug_ready = false;
	// SAFETY: callers own only new unpublished storage and already destroyed any new resource.
	unsafe { free_region(allocator, allocation, true) }
}

/// Release a separate allocation or pooled placement and invalidate its token.
/// # Safety
/// Destroy all bound resources and complete every host/GPU access first. This
/// is also the low-level escape route after manually destroying a managed buffer.
pub unsafe fn free(
	allocator: &mut Allocator<'_>,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	// SAFETY: the public release contract requires completed resource/host/GPU uses.
	unsafe { free_region(allocator, allocation, false) }
}

// Relocation alone may retire outstanding counted holds: its unsafe commit/
// ignore/destroy contract explicitly ends every old pointer use. Ordinary public
// free still rejects explicit maps. Keep guard failure before any hold release.
unsafe fn free_region(
	allocator: &mut Allocator<'_>,
	allocation: &mut Allocation,
	release_explicit: bool,
) -> Result<(), AllocError> {
	allocator.ensure_allocation()?;
	let live = allocator.validate(allocation)?;
	if live.map_count != 0 && !release_explicit {
		return Err(AllocError::AlreadyMapped);
	}
	// SAFETY: caller completed all uses and ended the applicable pointer lifetimes.
	unsafe { guard::before_free(allocator, allocation) }?;
	let live = allocator.validate(allocation)?;
	let resource = live.resource;
	let plane = live.plane;
	let placement = live.placement;
	let holds = live
		.map_count
		.checked_add(u32::from(live.persistent))
		.ok_or(AllocError::InvalidRange)?;
	if holds != 0 {
		let device = allocator.device;
		// SAFETY: ordinary free rejected explicit maps; relocation ends all counted
		// pointer uses. The canonical shared owner releases exactly these holds.
		unsafe {
			allocator
				.mapping_mut(allocation)
				.release_many(device, allocation.handle, holds)
		}?;
	}
	if let Some(placement) = placement {
		let block_index = allocator
			.blocks
			.iter()
			.position(|b| b.handle == allocation.handle)
			.expect("live block");
		let domain = BlockDomain::new_from_block(&allocator.blocks[block_index]);
		let block = &mut allocator.blocks[block_index];
		// SAFETY: every hold belonging to this region was released above.
		unsafe { block.mapping.post_free(allocator.device, block.handle) }?;
		block
			.metadata
			.free(placement)
			.expect("validated private placement");
		if block.defragmentation_hold {
			// The exclusive Extensive context will reuse or explicitly release this block.
		} else if block.automatic || block.pool.is_some() {
			pool::release_block(allocator, block_index);
		} else if block.metadata.is_empty() {
			debug_assert_eq!(block.mapping.references, 0);
			// SAFETY: no region holds remain; release the optional cache before native free.
			unsafe {
				allocator.blocks[block_index]
					.mapping
					.clear_cache(allocator.device, allocation.handle)
			}?;
			// SAFETY: all regions and maps ended; caller completed native resources.
			unsafe {
				allocator
					.device
					.free_memory(allocation.handle, allocator.callbacks)
			}?;
			let block = allocator.blocks.remove(block_index);
			allocator.remove_native(block.size, block.memory_type_index, block.handle);
		}
		domain.incrementally_sort(&mut allocator.blocks);
	} else {
		// SAFETY: checked ownership; caller guarantees resource destruction/completion.
		unsafe {
			allocator
				.device
				.free_memory(allocation.handle, allocator.callbacks)
		}?;
		allocator.remove_native(
			allocation.block_size,
			allocation.memory_type_index,
			allocation.handle,
		);
	}
	allocator.live.remove(&allocation.serial);
	if let Some(shared) = allocator.parallel_state() {
		shared.retire(resource, plane, allocation.serial);
	}
	allocator.budget_operations = allocator.budget_operations.saturating_add(1);
	allocation.handle = vk::DeviceMemory::NULL;
	Ok(())
}

/// Map one allocation. Distinct regions share a reference-counted block mapping.
/// Repeated explicit maps acquire checked holds on the same native mapping.
/// # Safety
/// Synchronize host/GPU access. Pointers remain valid only while an explicit or
/// persistent hold for this allocation remains live. Do not alias Rust references.
pub unsafe fn map(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<*mut std::ffi::c_void, AllocError> {
	let count = allocator
		.validate(allocation)?
		.map_count
		.checked_add(1)
		.ok_or(AllocError::InvalidRange)?;
	if !allocation.is_host_visible() {
		return Err(AllocError::NotHostVisible);
	}
	let device = allocator.device;
	// SAFETY: live visible block, checked host extent in Mapping; caller synchronizes.
	let base = unsafe {
		allocator
			.mapping_mut(allocation)
			.acquire(device, allocation.handle, allocation.block_size)
	}?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("validated live region")
		.map_count = count;
	// wrapping_add preserves pointer provenance without claiming a Rust object
	// bounds proof for an FFI mapping. Checked native bounds/host extent precede it.
	Ok(
		base
			.cast::<u8>()
			.wrapping_add(allocation.offset as usize)
			.cast(),
	)
}

/// Release one explicit map; persistent holds and other regions remain mapped.
/// # Safety
/// Stop using pointers acquired by this explicit hold. This never flushes or waits.
pub unsafe fn unmap(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	let count = allocator
		.validate(allocation)?
		.map_count
		.checked_sub(1)
		.ok_or(AllocError::NotMapped)?;
	let device = allocator.device;
	// SAFETY: a checked explicit hold exists and the caller ends its pointer uses.
	unsafe {
		allocator
			.mapping_mut(allocation)
			.release(device, allocation.handle)
	}?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("validated live region")
		.map_count = count;
	Ok(())
}

/// Return a region pointer only if that region has an explicit or persistent hold.
/// Does not map, acquire a hold, synchronize, or authorize pointer dereferences.
pub fn mapped_pointer(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
) -> Result<Option<*mut std::ffi::c_void>, AllocError> {
	let live = allocator.validate(allocation)?;
	if live.map_count == 0 && !live.persistent {
		return Ok(None);
	}
	Ok(Some(
		allocator
			.mapping(allocation)
			.pointer
			.cast::<u8>()
			.wrapping_add(allocation.offset as usize)
			.cast(),
	))
}

/// Snapshot one live allocation's binding, actual properties and current mapping.
///
/// Donor: pinned VMA `GetAllocationInfo` and `vmaGetAllocationMemoryProperties`.
/// Adaptation: checked owner/generation, one typed snapshot and per-region map holds.
/// Performs no native query, mapping, allocation or synchronization. A pooled
/// neighbor's mapping does not expose a pointer for an unmapped region.
/// # Errors
/// Returns `Closed`, `WrongAllocator` or `UnknownAllocation` before observing stale state.
pub fn allocation_info(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
) -> Result<AllocationInfo, AllocError> {
	let mapped_pointer = mapped_pointer(allocator, allocation)?;
	Ok(AllocationInfo {
		device_memory: allocation.handle,
		offset: allocation.offset,
		size: allocation.size,
		reserved_size: allocation.reserved_size,
		memory_type_index: allocation.memory_type_index,
		heap_index: allocation.heap_index,
		memory_properties: allocation.flags,
		mapped_pointer,
		map_count: allocator.validate(allocation)?.map_count,
		native_size: allocation.block_size,
		is_separate: allocator.validate(allocation)?.placement.is_none(),
		user_data: allocator.validate(allocation)?.user_data,
	})
}

/// Calculate local live-region/native-block accounting without querying the driver.
pub fn statistics(allocator: &Allocator<'_>) -> AllocatorStats {
	AllocatorStats {
		allocation_count: allocator.live.len(),
		native_allocation_count: allocator.native_count,
		pooled_block_count: allocator.blocks.len(),
		allocation_bytes: allocator.live.values().map(|v| v.size).sum(),
		reserved_bytes: allocator.live.values().map(|v| v.reserved_size).sum(),
		native_bytes: allocator.native_bytes,
		mapped_native_count: allocator
			.blocks
			.iter()
			.filter(|b| !b.mapping.pointer.is_null())
			.count()
			+ allocator
				.live
				.values()
				.filter(|v| v.placement.is_none() && v.mapping.references != 0)
				.count(),
	}
}

/// Flush a bounded allocation-relative mapped range, observing atom/end rules.
/// # Safety
/// Follow Vulkan's host/device synchronization and mapping requirements.
pub unsafe fn flush(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
	offset: u64,
	size: u64,
) -> Result<(), AllocError> {
	let live = allocator.validate(allocation)?;
	if live.map_count == 0 && !live.persistent {
		return Err(AllocError::NotMapped);
	}
	if let Some(range) = allocation.mapped_range(offset, size)? {
		// SAFETY: whole allocation mapped; range is bounded and atom aligned.
		unsafe { allocator.device.flush_mapped_memory_ranges(&[range]) }?;
	}
	Ok(())
}

/// Invalidate a bounded allocation-relative mapped range; never waits for the GPU.
/// # Safety
/// Complete device writes first and do not race host accesses.
pub unsafe fn invalidate(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
	offset: u64,
	size: u64,
) -> Result<(), AllocError> {
	let live = allocator.validate(allocation)?;
	if live.map_count == 0 && !live.persistent {
		return Err(AllocError::NotMapped);
	}
	if let Some(range) = allocation.mapped_range(offset, size)? {
		// SAFETY: whole allocation mapped; range is bounded and atom aligned.
		unsafe { allocator.device.invalidate_mapped_memory_ranges(&[range]) }?;
	}
	Ok(())
}

/// Transactionally create, query, allocate and bind an ordinary buffer.
///
/// Core 1.1 or enabled KHR dedication uses requirements2 with a dedicated
/// requirements output chain and dedicates this separate allocation to the
/// buffer, even when dedication is only preferred or unnecessary. Core 1.0
/// without dedication uses legacy requirements. Exact PFN spellings are retained.
///
/// # Safety
/// Info/limits/queue-family data and ordinary buffer usage must satisfy Vulkan
/// valid usage. Arbitrary chains and sparse/external memory are excluded. Protected memory
/// requires captured enabled feature metadata.
/// Device-address usage requires captured enabled core/KHR feature metadata.
pub unsafe fn create_buffer(
	allocator: &mut Allocator<'_>,
	info: &vk::BufferCreateInfo,
	required: vk::MemoryPropertyFlags,
	preferred: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, Allocation), AllocError> {
	let options = AllocationCreateInfo {
		required_flags: required,
		preferred_flags: preferred,
		..Default::default()
	};
	// SAFETY: same native guarantees with the existing separate-storage policy.
	unsafe { create_buffer_with_options(allocator, info, &options) }
}

/// Create, allocate, optionally map and bind an ordinary buffer transactionally.
///
/// Only recoverable native allocation OOM retries another compatible memory type.
/// Required/preferred dedication and regions larger than the explicit block use
/// separate storage. Empty blocks are released immediately, without caching.
/// # Errors
/// Returns checked policy/identity/limit errors or the preserved native failure.
/// # Safety
/// Info and queue-family pointers must satisfy native valid usage. Complete uses
/// before release. Sparse and capture/replay resources are excluded. Ordinary
/// external create/allocation profiles are admitted with caller-validated native
/// compatibility. Protected resources and BDA require captured enabled metadata.
pub unsafe fn create_buffer_with_options(
	allocator: &mut Allocator<'_>,
	info: &vk::BufferCreateInfo,
	options: &AllocationCreateInfo,
) -> Result<(vk::Buffer, Allocation), AllocError> {
	// SAFETY: same caller-owned native create records as the canonical operation.
	let device_address = unsafe { request::validate_buffer(allocator, info) }?;
	allocator.validate_size(info.size)?;
	let (mut required, preferred, avoided) =
		super::policy::preferences(options, info.usage.as_raw() & !3 != 0, allocator.integrated)?;
	required = request::resource_properties(
		allocator,
		info.flags.as_raw() & vk::BufferCreateFlagBits::PROTECTED_BIT.as_raw() != 0,
		required,
	)?;
	request::validate_options(allocator, options, required)?;
	// SAFETY: preceding resource validation checked the complete bounded native chain.
	let filtered = unsafe { external::resource_options(allocator, options, info.p_next) }?;
	let options = &filtered;
	if options.pool.is_none()
		&& options.policy == AllocationPolicy::Separate
		&& allocator.native_count >= allocator.allocation_limit as usize
	{
		return Err(AllocError::AllocationLimit);
	}
	if options.persistent_map {
		require(allocator.device.fp().map_memory.is_some(), "vkMapMemory")?;
		require(
			allocator.device.fp().unmap_memory.is_some(),
			"vkUnmapMemory",
		)?;
	}
	allocator.buffer_route()?;
	let fp = allocator.device.fp();
	require(fp.create_buffer.is_some(), "vkCreateBuffer")?;
	require(fp.destroy_buffer.is_some(), "vkDestroyBuffer")?;
	require(fp.bind_buffer_memory.is_some(), "vkBindBufferMemory")?;
	allocator.reserve()?;
	// SAFETY: caller guarantees ordinary buffer valid usage; cleanup/metadata preflighted.
	let buffer = unsafe { allocator.device.create_buffer(info, allocator.callbacks) }?;
	if buffer.is_null() {
		return Err(CallError::InvalidOutput("vkCreateBuffer returned a null handle").into());
	}
	if binding::associated(allocator, Resource::Buffer(buffer)) {
		return Err(CallError::InvalidOutput("vkCreateBuffer returned an already-live handle").into());
	}
	// SAFETY: newly created ordinary resource, exact native requirements and preflighted inverses.
	let result = unsafe {
		binding::allocate_resource(
			allocator,
			Resource::Buffer(buffer),
			options,
			(required, preferred, avoided),
			ResourceClass::Buffer,
			device_address,
		)
	};

	let mut allocation = match result {
		Ok(allocation) => allocation,
		Err(error) => {
			// SAFETY: unbound new buffer, cached immutable destruction PFN preflighted.
			unsafe { allocator.device.destroy_buffer(buffer, allocator.callbacks) }
				.expect("preflighted cleanup");
			return Err(error);
		}
	};
	// SAFETY: new host-visible storage and exact unbound resource/allocation pair.
	let result = unsafe { binding::persistent(allocator, &allocation, options) }
		.and_then(|()| unsafe { bind_buffer_memory(allocator, buffer, &allocation) });
	if let Err(error) = result {
		// SAFETY: new unpublished objects only; inverse operations preflighted.
		unsafe {
			allocator.release_resource_claim(&allocation);
			allocator
				.device
				.destroy_buffer(buffer, allocator.callbacks)
				.expect("preflighted cleanup");
			rollback_allocation(allocator, &mut allocation).expect("preflighted allocation rollback");
		}
		return Err(error);
	}

	Ok((buffer, allocation))
}

fn validate_policy(policy: AllocationPolicy) -> Result<(), AllocError> {
	match policy {
		AllocationPolicy::Pooled { block_size: 0 } => Err(AllocError::InvalidRequest(
			"pooled block size must be nonzero",
		)),
		AllocationPolicy::Adaptive {
			block_size,
			min_block_size,
		} if !block_size.is_power_of_two()
			|| !min_block_size.is_power_of_two()
			|| min_block_size > block_size =>
		{
			Err(AllocError::InvalidRequest(
				"adaptive block bounds must be ordered powers of two",
			))
		}
		_ => Ok(()),
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dedication {
	None,
	Preferred,
	Required,
}

impl Dedication {
	fn new(requirements: &vk::MemoryDedicatedRequirements) -> Self {
		if requirements.requires_dedicated_allocation != 0 {
			Self::Required
		} else if requirements.prefers_dedicated_allocation != 0 {
			Self::Preferred
		} else {
			Self::None
		}
	}
}

#[derive(Clone, Copy)]
struct BlockOptions {
	automatic: bool,
	block_size: u64,
	strategy: super::PlacementStrategy,
	class: ResourceClass,
	grow: bool,
	pool: Option<u64>,
	upper_address: bool,
	priority: MemoryPriority,
	within_budget: bool,
	prefer_mapped: bool,
}

// Donor: VMA BlockVector existing-block-first and smaller-block OOM retries.
// Adaptation: caller-visible finite bounds, no global growth history/hysteresis.
unsafe fn allocate_managed(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	storage: StorageOptions<'_>,
	dedication_policy: Dedication,
	options: &AllocationCreateInfo,
	class: ResourceClass,
	avoided: vk::MemoryPropertyFlags,
) -> Result<Allocation, AllocError> {
	let mut storage = storage;
	let pool_chain = options.pool.map_or(Default::default(), |pool| {
		allocator.pools[&pool.serial].allocation_chain
	});
	if let Some(chain) = options.memory_allocate_next {
		if !pool_chain.next.is_null() {
			return Err(AllocError::InvalidRequest(
				"allocation and pool chains cannot be combined",
			));
		}
		storage.chain = chain.data;
	} else {
		storage.chain = pool_chain;
	}
	let mut imported_dedicated = vk::MemoryDedicatedAllocateInfo::default();
	if let Some(resource) = storage
		.chain
		.import_info
		.and_then(|info| import_resource(info.resource))
	{
		if options.can_alias {
			return Err(AllocError::InvalidRequest(
				"resource-specific imports cannot alias",
			));
		}
		if !allocator
			.device
			.capabilities()
			.is_some_and(|caps| caps.api_version() >= vk::API_VERSION_1_1)
			&& !super::resource::extension(allocator.device, c"VK_KHR_dedicated_allocation")
		{
			return Err(AllocError::UnsupportedCapability(
				"import dedication requires core 1.1 or KHR_dedicated_allocation",
			));
		}
		if let Some(existing) = storage.dedicated {
			let matches = match resource {
				Resource::Buffer(buffer) => existing.buffer == buffer && existing.image.is_null(),
				Resource::Image(image) => existing.image == image && existing.buffer.is_null(),
			};
			if !matches {
				return Err(AllocError::WrongResource);
			}
		}
		match resource {
			Resource::Buffer(buffer) => imported_dedicated.buffer = buffer,
			Resource::Image(image) => imported_dedicated.image = image,
		}
		storage.dedicated = Some(&imported_dedicated);
	}
	if storage.chain.import_kind == super::chain::ImportKind::Host {
		if dedication_policy == Dedication::Required {
			return Err(AllocError::InvalidRequest(
				"host imports forbid required resource dedication",
			));
		}
		storage.dedicated = None;
	}
	let class = if options.can_alias {
		ResourceClass::Unknown
	} else {
		class
	};
	let dedication_policy = if options.can_alias {
		if dedication_policy == Dedication::Required {
			return Err(AllocError::UnsupportedCapability(
				"required resource dedication cannot alias",
			));
		}
		storage.dedicated = None;
		storage.device_address = allocator.buffer_device_address;
		Dedication::None
	} else {
		dedication_policy
	};
	let dedication_policy = if options.never_allocate && dedication_policy == Dedication::Preferred {
		Dedication::None
	} else {
		dedication_policy
	};
	if options.never_allocate && dedication_policy == Dedication::Required {
		return Err(AllocError::OutOfSpace);
	}

	let forced_separate = options.dedicated_memory || options.memory_allocate_next.is_some();
	if forced_separate {
		if options.never_allocate || options.upper_address {
			return Err(AllocError::InvalidRequest(
				"separate allocation conflicts with reuse/upper placement",
			));
		}
		if let Some(pool) = options.pool {
			if allocator.pools[&pool.serial].info.block_size != 0 {
				return Err(AllocError::UnsupportedCapability(
					"fixed pool cannot allocate separate storage",
				));
			}
			let index = allocator.pools[&pool.serial].info.memory_type_index;
			let request = AllocRequest {
				memory_type_bits: request.memory_type_bits & (1 << index),
				..*request
			};
			// SAFETY: exact compatible pool type and caller's synchronous chain contract.
			let allocation = unsafe { allocate_storage(allocator, &request, storage, avoided) }?;
			allocator
				.live
				.get_mut(&allocation.serial)
				.expect("new separate allocation")
				.pool = Some(pool.serial);
			return Ok(allocation);
		}
		// SAFETY: explicit separate request, admitted tail and original requirements.
		return unsafe { allocate_storage(allocator, request, storage, avoided) };
	}

	if options.pool.is_some() {
		// SAFETY: identical native requirements, chain and resource lifetime contract.
		return unsafe {
			pool::allocate(
				allocator,
				request,
				storage,
				dedication_policy,
				options,
				class,
				avoided,
			)
		};
	}
	if options.policy == AllocationPolicy::Automatic {
		// SAFETY: original native requirements/chain; shared donor automatic policy fixes each candidate type.
		return unsafe {
			pool::allocate_default(
				allocator,
				request,
				storage,
				dedication_policy,
				options,
				class,
				avoided,
			)
		};
	}

	if dedication_policy == Dedication::None {
		match options.policy {
			AllocationPolicy::Pooled { block_size } if request.size <= block_size => {
				// SAFETY: matching native requirements and checked resource page separation.
				return unsafe {
					allocate_pooled(
						allocator,
						request,
						avoided,
						BlockOptions {
							automatic: false,
							block_size,
							strategy: options.strategy,
							class,
							grow: !options.never_allocate,
							pool: None,
							upper_address: false,
							priority: options.priority,
							within_budget: options.within_budget,
							prefer_mapped: super::policy::prefers_mapped(options),
						},
					)
				};
			}
			AllocationPolicy::Adaptive {
				block_size,
				min_block_size,
			} => {
				for grow in [false, true] {
					if grow && options.never_allocate {
						break;
					}
					let mut size = block_size;
					while size >= request.size && size >= min_block_size {
						// SAFETY: matching native requirements; failed attempts publish no placement.
						match unsafe {
							allocate_pooled(
								allocator,
								request,
								avoided,
								BlockOptions {
									automatic: false,
									block_size: size,
									strategy: options.strategy,
									class,
									grow,
									pool: None,
									upper_address: false,
									priority: options.priority,
									within_budget: options.within_budget,
									prefer_mapped: super::policy::prefers_mapped(options),
								},
							)
						} {
							Ok(allocation) => return Ok(allocation),
							Err(
								AllocError::OutOfSpace
								| AllocError::AllocationLimit
								| AllocError::AllocationTooLarge { .. },
							) => {}
							Err(AllocError::Call(error)) if recoverable(&error) => {}
							Err(error) => return Err(error),
						}
						if size == min_block_size {
							break;
						}
						size /= 2;
					}
				}
			}
			_ => {}
		}
	}
	// SAFETY: unchanged required flags and admitted buffer/image dedication chain.
	if options.never_allocate {
		return Err(AllocError::OutOfSpace);
	}
	unsafe { allocate_storage(allocator, request, storage, avoided) }
}

// Donor: VMA BlockVector existing-block-first policy, explicit-size blocks,
// FindMemoryTypeIndex retry, GetMemoryTypeMinAlignment. Adaptations: verified
// virtual TLSF bucket geometry, compatible block-vector search, physical-neighbor
// granularity and atom-rounded reservation. Default blocks are released when empty;
// custom pools retain their minimum and one empty spare through the pool owner.
unsafe fn allocate_pooled(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	avoided: vk::MemoryPropertyFlags,
	options: BlockOptions,
) -> Result<Allocation, AllocError> {
	let BlockOptions {
		automatic,
		block_size,
		strategy,
		class,
		grow,
		pool,
		upper_address,
		priority,
		within_budget,
		prefer_mapped,
	} = options;
	let algorithm = pool.map_or(AllocationAlgorithm::Tlsf, |serial| {
		allocator.pools[&serial].info.algorithm
	});
	let allow_ring = pool.is_some_and(|serial| allocator.pools[&serial].info.max_block_count == 1);
	let exact_size = !automatic;
	if exact_size {
		allocator.validate_size(block_size)?;
	}
	let mut bits = request.memory_type_bits;
	let mut last = None;
	loop {
		let candidate = AllocRequest {
			memory_type_bits: bits,
			..*request
		};
		let index = match allocator
			.selector
			.find_memory_type_with_preferences(&candidate, avoided)
		{
			Ok(index) => index,
			Err(error) => return Err(last.unwrap_or(error)),
		};
		bits &= !(1 << index);
		let flags = allocator.selector.properties.memory_types[index as usize].property_flags;
		let noncoherent = flags.as_raw()
			& (vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw()
				| vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw())
			== vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw();
		let atom = if noncoherent {
			allocator.selector.atom
		} else {
			1
		};
		let guard_size = if guard::eligible(flags) {
			allocator.debug_settings.margin
		} else {
			0
		};
		let data_extent = if guard_size != 0 {
			request
				.size
				.checked_add(3)
				.map(|size| size & !3)
				.ok_or(AllocError::InvalidRange)?
		} else {
			request.size
		};
		let alignment = request
			.alignment
			.max(atom)
			.max(if guard_size != 0 { 4 } else { 1 });
		if !alignment.is_power_of_two() || !request.alignment.is_power_of_two() || request.size == 0 {
			return Err(AllocError::InvalidRequest(
				"invalid native buffer requirements",
			));
		}
		let reserved = data_extent
			.checked_add(guard_size)
			.and_then(|size| size.checked_add(atom - 1))
			.map(|v| v & !(atom - 1))
			.ok_or(AllocError::InvalidRange)?;
		if reserved > block_size {
			// The nominal requirement fits, but its private atom extent does not.
			last = Some(AllocError::OutOfSpace);
			continue;
		}
		let domain = BlockDomain {
			memory_type_index: index,
			pool,
			automatic,
			priority,
			exact_size: exact_size.then_some(block_size),
		};
		// Linear vectors use only their newest block. TLSF MIN_TIME scans
		// backward without mapping preference; every other strategy scans
		// forward, first in the preferred mapping group, then the other.
		let last_block = if algorithm == AllocationAlgorithm::Linear {
			allocator
				.blocks
				.iter()
				.rposition(|block| domain.contains(block))
		} else {
			None
		};
		let reverse = strategy == super::PlacementStrategy::MinTime;
		let mapping_groups = algorithm == AllocationAlgorithm::Tlsf
			&& !reverse
			&& flags.as_raw() & vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw() != 0;
		for group in 0..if mapping_groups { 2 } else { 1 } {
			for step in 0..allocator.blocks.len() {
				let block_index = if reverse {
					allocator.blocks.len() - 1 - step
				} else {
					step
				};
				let block = &mut allocator.blocks[block_index];
				let is_mapped = !block.mapping.pointer.is_null();
				if !domain.contains(block)
					|| (algorithm == AllocationAlgorithm::Linear && Some(block_index) != last_block)
					|| (mapping_groups && ((group == 0) != (prefer_mapped == is_mapped)))
				{
					continue;
				}
				let placement = match block.metadata.allocate_resource(
					reserved,
					alignment,
					strategy,
					upper_address,
					class,
				) {
					Ok(placement) => placement,
					Err(AllocError::OutOfSpace) => continue,
					Err(error) => return Err(error),
				};
				let offset = block
					.metadata
					.info(placement)
					.expect("new placement")
					.offset;
				let handle = block.handle;
				let actual_size = block.size;
				let allocation = publish(
					allocator,
					Region {
						handle,
						index,
						size: request.size,
						reserved_size: reserved,
						offset,
						alignment,
						block_size: actual_size,
						placement: Some(placement),
					},
				);
				domain.incrementally_sort(&mut allocator.blocks);
				return Ok(allocation);
			}
		}
		if let Some(pool) = pool {
			let max = allocator.pools[&pool].info.max_block_count;
			if max != 0
				&& allocator
					.blocks
					.iter()
					.filter(|block| block.pool == Some(pool))
					.count()
					>= max
			{
				return Err(AllocError::OutOfSpace);
			}
		}
		if !grow {
			last = Some(AllocError::OutOfSpace);
			continue;
		}
		if let Err(error) = allocator.check_native(block_size, index) {
			// A count ceiling prevents growth, not reuse of another compatible type.
			if error == AllocError::OutOfSpace || error == AllocError::AllocationLimit {
				last = Some(error);
				continue;
			}
			return Err(error);
		}
		allocator
			.blocks
			.try_reserve(1)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		let mut metadata = Metadata::new_native(
			block_size,
			algorithm,
			allow_ring,
			allocator.granularity,
			allocator.host,
		)?;
		let placement =
			metadata.allocate_resource(reserved, alignment, strategy, upper_address, class)?;
		let offset = metadata.info(placement)?.offset;
		let chain = pool.map_or(Default::default(), |serial| {
			allocator.pools[&serial].allocation_chain
		});
		let export_types = external::allocation_exports(allocator, index, chain)?;
		// SAFETY: exact ordinary block requirements and admitted native capability metadata.
		let handle = match unsafe {
			allocate_native(
				allocator,
				block_size,
				index,
				StorageOptions {
					chain: pool.map_or(Default::default(), |serial| {
						allocator.pools[&serial].allocation_chain
					}),
					dedicated: None,
					device_address: allocator.buffer_device_address,
					priority,
					within_budget,
				},
			)
		} {
			Ok(handle) => handle,
			Err(error)
				if matches!(&error, AllocError::Call(native) if recoverable(native))
					|| error == AllocError::OutOfSpace =>
			{
				last = Some(error);
				continue;
			}
			Err(error) => return Err(error),
		};
		if handle.is_null() || allocator.contains_memory(handle) {
			return Err(
				CallError::InvalidOutput("allocator returned a null or already-live memory handle").into(),
			);
		}
		allocator.blocks.push_reserved(Block {
			defragmentation_hold: false,
			automatic,
			export_types,
			priority,
			pool,
			handle,
			size: block_size,
			memory_type_index: index,
			metadata,
			mapping: Mapping::new_with_hysteresis(allocator.mapping_hysteresis),
		});
		allocator.add_native(block_size, index, handle);
		let allocation = publish(
			allocator,
			Region {
				handle,
				index,
				size: request.size,
				reserved_size: reserved,
				offset,
				alignment,
				block_size,
				placement: Some(placement),
			},
		);
		domain.incrementally_sort(&mut allocator.blocks);
		return Ok(allocation);
	}
}

/// Destroy a matching resource pair in buffer-before-memory order with matching retained callbacks.
/// # Safety
/// Complete all host/GPU uses. Neither object may have been externally destroyed.
/// The buffer must use this allocator's callbacks; otherwise destroy it directly then free.
pub unsafe fn destroy_buffer(
	allocator: &mut Allocator<'_>,
	buffer: vk::Buffer,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	allocator.ensure_allocation()?;
	let live = allocator.validate(allocation)?;
	if live.resource != Some(Resource::Buffer(buffer)) {
		return Err(AllocError::WrongResource);
	}
	if live.map_count != 0 {
		return Err(AllocError::AlreadyMapped);
	}
	// Both transitions must be possible before any native destruction occurs.
	require(
		allocator.device.fp().destroy_buffer.is_some(),
		"vkDestroyBuffer",
	)?;
	require(allocator.device.fp().free_memory.is_some(), "vkFreeMemory")?;
	// SAFETY: association/state validated; caller guarantees exact completion.
	unsafe {
		guard::before_free(allocator, allocation)?;
		allocator.release_resource_claim(allocation);
		allocator
			.device
			.destroy_buffer(buffer, allocator.callbacks)?;
		allocator
			.live
			.get_mut(&allocation.serial)
			.expect("validated resource allocation")
			.debug_ready = false;
		free(allocator, allocation)
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_lifecycle.rs"]
mod tests;

/// Create an ordinary image with separate storage and explicit memory properties.
/// # Safety
/// Caller establishes all native image, create-chain and synchronization valid usage.
pub unsafe fn create_image(
	allocator: &mut Allocator<'_>,
	info: &vk::ImageCreateInfo,
	required: vk::MemoryPropertyFlags,
	preferred: vk::MemoryPropertyFlags,
) -> Result<(vk::Image, Allocation), AllocError> {
	let options = AllocationCreateInfo {
		required_flags: required,
		preferred_flags: preferred,
		..Default::default()
	};
	// SAFETY: same native guarantees and separate-storage policy.
	unsafe { create_image_with_options(allocator, info, &options) }
}

/// Create, query, allocate, optionally map and bind a non-disjoint image transactionally.
///
/// Donor: pinned VMA `CreateImage`, `GetImageMemoryRequirements`, `AllocateDedicatedMemory`.
/// Adaptation: checked neighbor metadata separates incompatible buffer/image pages
/// while compatible resources share native blocks. Unknown granularity stays conservative.
/// Video-profile and image-format-list chains are forwarded unchanged after admission.
/// Sparse, disjoint, external and unknown allocation chains are excluded.
/// Aliases require explicit can_alias storage; protected images require enabled metadata.
/// # Safety
/// Meet image/format/feature and pointer-chain valid usage. A mapped optimal image is not a
/// linear pixel array. Callers use native subresource layouts for host access to linear images.
pub unsafe fn create_image_with_options(
	allocator: &mut Allocator<'_>,
	info: &vk::ImageCreateInfo,
	options: &AllocationCreateInfo,
) -> Result<(vk::Image, Allocation), AllocError> {
	// SAFETY: same caller-owned native create records as the canonical operation.
	let class = unsafe { request::validate_image(allocator, info) }?;
	let (mut required, preferred, avoided) =
		super::policy::preferences(options, info.usage.as_raw() & !3 != 0, allocator.integrated)?;
	required = request::resource_properties(
		allocator,
		info.flags.as_raw() & vk::ImageCreateFlagBits::PROTECTED_BIT.as_raw() != 0,
		required,
	)?;
	request::validate_options(allocator, options, required)?;
	// SAFETY: preceding resource validation checked the complete bounded native chain.
	let filtered = unsafe { external::resource_options(allocator, options, info.p_next) }?;
	let options = &filtered;
	allocator.requirements_route(true)?;
	let fp = allocator.device.fp();
	require(fp.create_image.is_some(), "vkCreateImage")?;
	require(fp.destroy_image.is_some(), "vkDestroyImage")?;
	require(fp.bind_image_memory.is_some(), "vkBindImageMemory")?;
	if options.persistent_map {
		require(fp.map_memory.is_some(), "vkMapMemory")?;
		require(fp.unmap_memory.is_some(), "vkUnmapMemory")?;
	}
	allocator.reserve()?;
	// SAFETY: native info valid by caller contract; inverse PFNs and metadata preflighted.
	let image = unsafe { allocator.device.create_image(info, allocator.callbacks) }?;
	if image.is_null() {
		return Err(CallError::InvalidOutput("vkCreateImage returned a null handle").into());
	}
	if binding::associated(allocator, Resource::Image(image)) {
		return Err(CallError::InvalidOutput("vkCreateImage returned an already-live handle").into());
	}
	// SAFETY: newly created ordinary image with validated creation metadata.
	let result = unsafe {
		binding::allocate_resource(
			allocator,
			Resource::Image(image),
			options,
			(required, preferred, avoided),
			class,
			false,
		)
	};

	let mut allocation = match result {
		Ok(allocation) => allocation,
		Err(error) => {
			// SAFETY: only the unpublished unbound image exists; inverse PFN preflighted.
			unsafe { allocator.device.destroy_image(image, allocator.callbacks) }
				.expect("preflighted cleanup");
			return Err(error);
		}
	};
	// SAFETY: new host-visible storage and matching unbound image/allocation pair.
	let result = unsafe { binding::persistent(allocator, &allocation, options) }
		.and_then(|()| unsafe { bind_image_memory(allocator, image, &allocation) });

	if let Err(error) = result {
		// SAFETY: unpublished objects only; inverse transitions preflighted.
		unsafe {
			allocator.release_resource_claim(&allocation);
			allocator
				.device
				.destroy_image(image, allocator.callbacks)
				.expect("preflighted cleanup");
			rollback_allocation(allocator, &mut allocation).expect("preflighted allocation rollback");
		}
		return Err(error);
	}
	Ok((image, allocation))
}

/// Destroy the exact image/token pair with matching retained callbacks before freeing memory.
/// # Safety
/// Destroy views and complete all native uses first; do not externally destroy this image.
/// The image must use this allocator's callbacks; otherwise destroy it directly then free.
pub unsafe fn destroy_image(
	allocator: &mut Allocator<'_>,
	image: vk::Image,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	allocator.ensure_allocation()?;
	let live = allocator.validate(allocation)?;
	if live.plane.is_some() {
		return Err(AllocError::InvalidRequest(
			"destroy a disjoint image once, then release every plane allocation",
		));
	}
	if live.resource != Some(Resource::Image(image)) {
		return Err(AllocError::WrongResource);
	}
	if live.map_count != 0 {
		return Err(AllocError::AlreadyMapped);
	}
	require(
		allocator.device.fp().destroy_image.is_some(),
		"vkDestroyImage",
	)?;
	require(allocator.device.fp().free_memory.is_some(), "vkFreeMemory")?;
	// SAFETY: checked association/state; caller supplies exact completion.
	unsafe {
		guard::before_free(allocator, allocation)?;
		allocator.release_resource_claim(allocation);
		allocator.device.destroy_image(image, allocator.callbacks)?;
		allocator
			.live
			.get_mut(&allocation.serial)
			.expect("validated resource allocation")
			.debug_ready = false;
		free(allocator, allocation)
	}
}
