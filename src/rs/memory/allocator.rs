//! Separate and pooled ordinary buffers with explicit native lifetime management.
//!
//! Donor: VMA 3.4.0, commit 3aa921224c154a0d2c43912bc88e1c42ce1f7607,
//! `GetBufferMemoryRequirements` and `AllocateDedicatedMemory`. See LICENSE-VMA-MIT.
//! Adaptation: owned capability admission, checked tokens, fallible host reservation,
//! cached PFNs and explicit Rust error-bearing cleanup; no implicit native Drop.

use super::{
	AllocError, AllocRequest, Allocation, AllocationCreateInfo, AllocationPolicy, AllocatorStats,
	MemoryTypeSelector,
	block::{Block, Mapping},
	tlsf::Tlsf,
};
use crate::{core as vk, core::Device, load::CallError};
use std::{collections::HashMap, sync::Arc};

struct Live {
	serial: u64,
	handle: vk::DeviceMemory,
	size: u64,
	reserved_size: u64,
	mapped: bool,
	persistent: bool,
	buffer: Option<vk::Buffer>,
	placement: Option<(usize, u64)>,
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
	device: &'device Device,
	selector: MemoryTypeSelector,
	owner: Arc<()>,
	next_serial: u64,
	live: HashMap<u64, Live>,
	blocks: Vec<Block>,
	native_count: usize,
	native_bytes: u64,
	heap_bytes: [u64; 16],
	allocation_limit: u32,
	allocation_size_limit: Option<u64>,
	closed: bool,
}

impl<'device> Allocator<'device> {
	/// Construct from caller-supplied physical-device metadata.
	///
	/// Prefer [`Self::new_from_physical_device`] for queried properties and the
	/// local native-allocation count limit. Managed buffers require the device's
	/// owned creation capabilities; raw imports without them support only the
	/// explicitly unsafe low-level allocation route.
	///
	/// # Safety
	/// Properties and atom size must belong to this device's physical device.
	/// Caller must enforce device allocation size/count limits and enabled features.
	pub unsafe fn new(
		device: &'device Device,
		properties: vk::PhysicalDeviceMemoryProperties,
		atom: u64,
	) -> Result<Self, AllocError> {
		let selector = MemoryTypeSelector::new(properties, atom)?;
		require(device.fp().allocate_memory.is_some(), "vkAllocateMemory")?;
		require(device.fp().free_memory.is_some(), "vkFreeMemory")?;
		Ok(Self {
			device,
			selector,
			owner: Arc::new(()),
			next_serial: 1,
			live: HashMap::new(),
			blocks: Vec::new(),
			native_count: 0,
			native_bytes: 0,
			heap_bytes: [0; 16],
			allocation_limit: u32::MAX,
			allocation_size_limit: None,
			closed: false,
		})
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
		let mut allocator =
			unsafe { Self::new(device, memory, properties.limits.non_coherent_atom_size) }?;
		allocator.allocation_limit = properties.limits.max_memory_allocation_count;
		allocator.allocation_size_limit = allocation_size_limit;
		// Select/validate the exact admitted requirements command before publication.
		allocator.buffer_route()?;
		Ok(allocator)
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
	/// No external, protected, device-address or required-dedication semantics are admitted.
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

	/// Map the complete allocation; repeated maps are rejected.
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
	/// Info must satisfy device limits/valid usage. Sparse, external, protected
	/// and device-address buffers and arbitrary pNext chains are not admitted.
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

	/// Observe a current explicit or persistent mapping without acquiring a hold.
	pub fn mapped_pointer(
		&self,
		allocation: &Allocation,
	) -> Result<Option<*mut std::ffi::c_void>, AllocError> {
		mapped_pointer(self, allocation)
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

	fn validate(&self, allocation: &Allocation) -> Result<&Live, AllocError> {
		self.ensure_open()?;
		if !Arc::ptr_eq(&self.owner, &allocation.owner) {
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
		let available = self.selector.properties.memory_heaps[heap].size - self.heap_bytes[heap];
		if size > available {
			return Err(AllocError::OutOfSpace);
		}
		self
			.native_bytes
			.checked_add(size)
			.ok_or(AllocError::InvalidRange)?;
		Ok(())
	}

	fn add_native(&mut self, size: u64, index: u32) {
		let heap = self.selector.properties.memory_types[index as usize].heap_index as usize;
		self.native_count += 1;
		self.native_bytes += size;
		self.heap_bytes[heap] += size;
	}

	fn remove_native(&mut self, size: u64, index: u32) {
		let heap = self.selector.properties.memory_types[index as usize].heap_index as usize;
		self.native_count -= 1;
		self.native_bytes -= size;
		self.heap_bytes[heap] -= size;
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
		let capabilities = self
			.device
			.capabilities()
			.ok_or(AllocError::UnsupportedCapability(
				"managed buffers require device creation metadata",
			))?;
		if capabilities.api_version() >= vk::API_VERSION_1_1 {
			require(
				self.device.fp().get_buffer_memory_requirements2.is_some(),
				"vkGetBufferMemoryRequirements2",
			)?;
			return Ok(RequirementsRoute::Core);
		}
		let enabled = |name| {
			capabilities
				.device_extensions()
				.is_some_and(|mut extensions| extensions.any(|extension| extension == name))
		};
		if enabled(c"VK_KHR_dedicated_allocation") {
			if !enabled(c"VK_KHR_get_memory_requirements2") {
				return Err(AllocError::UnsupportedCapability(
					"dedication requires VK_KHR_get_memory_requirements2",
				));
			}
			require(
				self
					.device
					.fp()
					.get_buffer_memory_requirements2_khr
					.is_some(),
				"vkGetBufferMemoryRequirements2KHR",
			)?;
			Ok(RequirementsRoute::Khr)
		} else {
			require(
				self.device.fp().get_buffer_memory_requirements.is_some(),
				"vkGetBufferMemoryRequirements",
			)?;
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
	if !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	allocator.closed = true;
	Ok(())
}

/// Allocate separate native storage with no advanced allocation chain.
/// # Safety
/// Requirements, size/count limits and enabled features must be valid. Required
/// dedication, protected, device-address and external allocation are excluded.
pub unsafe fn allocate(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
) -> Result<Allocation, AllocError> {
	// SAFETY: caller provides the requirements and ordinary allocation semantics.
	unsafe { allocate_storage(allocator, request, None) }
}

unsafe fn allocate_storage(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	dedicated: Option<&vk::MemoryDedicatedAllocateInfo>,
) -> Result<Allocation, AllocError> {
	allocator.ensure_open()?;
	allocator.validate_size(request.size)?;
	if request.required_flags.as_raw() & vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw() != 0 {
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
		let index = match allocator.selector.find_memory_type(&candidate) {
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
		let info = vk::MemoryAllocateInfo {
			allocation_size: request.size,
			memory_type_index: index,
			p_next: dedicated.map_or(std::ptr::null(), |info| std::ptr::from_ref(info).cast()),
			..Default::default()
		};
		// SAFETY: matching requirements, admitted chain and reserved bookkeeping.
		let handle = match unsafe { allocator.device.allocate_memory(&info, None) } {
			Ok(handle) => handle,
			Err(error) if recoverable(&error) => {
				last = Some(error.into());
				continue;
			}
			Err(error) => return Err(error.into()),
		};
		if handle.is_null() || allocator.contains_memory(handle) {
			return Err(
				CallError::InvalidOutput("allocator returned a null or already-live memory handle").into(),
			);
		}
		allocator.add_native(request.size, index);
		return Ok(publish(
			allocator,
			Region {
				handle,
				index,
				size: request.size,
				reserved_size: request.size,
				offset: 0,
				block_size: request.size,
				placement: None,
			},
		));
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
	block_size: u64,
	placement: Option<(usize, u64)>,
}

fn publish(allocator: &mut Allocator<'_>, region: Region) -> Allocation {
	let Region {
		handle,
		index,
		size,
		reserved_size,
		offset,
		block_size,
		placement,
	} = region;
	let serial = allocator.next_serial;
	allocator.next_serial += 1;
	allocator.live.insert(
		serial,
		Live {
			serial,
			handle,
			size,
			reserved_size,
			mapped: false,
			persistent: false,
			buffer: None,
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
		owner: allocator.owner.clone(),
		serial,
	}
}

/// Release a separate allocation or pooled placement and invalidate its token.
/// # Safety
/// Destroy all bound resources and complete every host/GPU access first. This
/// is also the low-level escape route after manually destroying a managed buffer.
pub unsafe fn free(
	allocator: &mut Allocator<'_>,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	let live = allocator.validate(allocation)?;
	if live.mapped {
		return Err(AllocError::AlreadyMapped);
	}
	let placement = live.placement;
	let persistent = live.persistent;
	if persistent {
		let device = allocator.device;
		// SAFETY: no explicit maps remain; caller ends every persistent pointer use.
		unsafe {
			allocator
				.mapping_mut(allocation)
				.release(device, allocation.handle)
		}?;
	}
	if let Some((index, generation)) = placement {
		let block_index = allocator
			.blocks
			.iter()
			.position(|b| b.handle == allocation.handle)
			.expect("live block");
		let block = &mut allocator.blocks[block_index];
		block
			.metadata
			.free(index, generation)
			.expect("validated private placement");
		if block.metadata.statistics().allocation_count == 0 {
			debug_assert_eq!(block.mapping.references, 0);
			// SAFETY: caller ended every resource/access; the final region released the block.
			unsafe { allocator.device.free_memory(allocation.handle, None) }?;
			let block = allocator.blocks.remove(block_index);
			allocator.remove_native(block.size, block.memory_type_index);
		}
	} else {
		// SAFETY: checked ownership; caller guarantees resource destruction/completion.
		unsafe { allocator.device.free_memory(allocation.handle, None) }?;
		allocator.remove_native(allocation.block_size, allocation.memory_type_index);
	}
	allocator.live.remove(&allocation.serial);
	allocation.handle = vk::DeviceMemory::NULL;
	Ok(())
}

/// Map one allocation. Distinct regions share a reference-counted block mapping.
/// Repeated explicit maps of the same region return `AlreadyMapped`.
/// # Safety
/// Synchronize host/GPU access. Pointers remain valid only while an explicit or
/// persistent hold for this allocation remains live. Do not alias Rust references.
pub unsafe fn map(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<*mut std::ffi::c_void, AllocError> {
	if allocator.validate(allocation)?.mapped {
		return Err(AllocError::AlreadyMapped);
	}
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
		.mapped = true;
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
	if !allocator.validate(allocation)?.mapped {
		return Err(AllocError::NotMapped);
	}
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
		.mapped = false;
	Ok(())
}

/// Return a region pointer only if that region has an explicit or persistent hold.
/// Does not map, acquire a hold, synchronize, or authorize pointer dereferences.
pub fn mapped_pointer(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
) -> Result<Option<*mut std::ffi::c_void>, AllocError> {
	let live = allocator.validate(allocation)?;
	if !live.mapped && !live.persistent {
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
			.filter(|b| b.mapping.references != 0)
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
	if !live.mapped && !live.persistent {
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
	if !live.mapped && !live.persistent {
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
/// valid usage. Arbitrary chains, sparse/protected/external/BDA are excluded.
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
/// before release. Sparse, external, protected, BDA and arbitrary chains are excluded.
pub unsafe fn create_buffer_with_options(
	allocator: &mut Allocator<'_>,
	info: &vk::BufferCreateInfo,
	options: &AllocationCreateInfo,
) -> Result<(vk::Buffer, Allocation), AllocError> {
	let mut required = options.required_flags;
	if options.persistent_map {
		required = required | vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT;
	}
	let preferred = options.preferred_flags;
	allocator.ensure_open()?;
	allocator.validate_size(info.size)?;
	if !info.p_next.is_null()
		|| info.flags.as_raw() != 0
		|| info.size == 0
		|| info.usage.as_raw() == 0
		|| info.usage.as_raw()
			& (vk::BufferUsageFlagBits::SHADER_DEVICE_ADDRESS_BIT.as_raw()
				| vk::BufferUsageFlagBits::TILE_MEMORY_BIT_QCOM.as_raw())
			!= 0
	{
		return Err(AllocError::InvalidRequest("ordinary nonempty buffers only"));
	}
	if required.as_raw() & vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw() != 0 {
		return Err(AllocError::UnsupportedCapability("protected allocation"));
	}
	if let AllocationPolicy::Pooled { block_size: 0 } = options.policy {
		return Err(AllocError::InvalidRequest(
			"pooled block size must be nonzero",
		));
	}
	if options.policy == AllocationPolicy::Separate
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
	let route = allocator.buffer_route()?;
	let fp = allocator.device.fp();
	require(fp.create_buffer.is_some(), "vkCreateBuffer")?;
	require(fp.destroy_buffer.is_some(), "vkDestroyBuffer")?;
	require(fp.bind_buffer_memory.is_some(), "vkBindBufferMemory")?;
	allocator.reserve()?;
	// SAFETY: caller guarantees ordinary buffer valid usage; cleanup/metadata preflighted.
	let buffer = unsafe { allocator.device.create_buffer(info, None) }?;
	if buffer.is_null() {
		return Err(CallError::InvalidOutput("vkCreateBuffer returned a null handle").into());
	}
	let mut dedication = vk::MemoryDedicatedRequirements::default();
	let mut requirements2 = vk::MemoryRequirements2 {
		p_next: std::ptr::from_mut(&mut dedication).cast(),
		..Default::default()
	};
	let query = vk::BufferMemoryRequirementsInfo2 {
		buffer,
		..Default::default()
	};
	// SAFETY: live unbound ordinary buffer; exact route admitted and PFN preflighted.
	let requirements = unsafe {
		match route {
			RequirementsRoute::Legacy => allocator.device.get_buffer_memory_requirements(buffer),
			RequirementsRoute::Core => allocator
				.device
				.get_buffer_memory_requirements2(&query, &mut requirements2)
				.map(|()| requirements2.memory_requirements),
			RequirementsRoute::Khr => allocator
				.device
				.get_buffer_memory_requirements2_khr(&query, &mut requirements2)
				.map(|()| requirements2.memory_requirements),
		}
	};
	let request = requirements.map(|requirements| AllocRequest {
		size: requirements.size,
		alignment: requirements.alignment,
		memory_type_bits: requirements.memory_type_bits & options.memory_type_bits,
		required_flags: required,
		preferred_flags: preferred,
	});
	let dedicated = vk::MemoryDedicatedAllocateInfo {
		buffer,
		..Default::default()
	};
	let dedicated = match route {
		RequirementsRoute::Legacy => None,
		_ => Some(&dedicated),
	};
	// SAFETY: queried compatible requirements and legal buffer-specific dedication.
	let result = match request {
		Ok(request) => {
			let force_separate = dedication.requires_dedicated_allocation != 0
				|| dedication.prefers_dedicated_allocation != 0;
			if let AllocationPolicy::Pooled { block_size } = options.policy
				&& !force_separate
				&& request.size <= block_size
			{
				// SAFETY: queried ordinary buffer requirements and explicitly selected policy.
				unsafe { allocate_pooled(allocator, &request, block_size, options.strategy) }
			} else {
				// SAFETY: queried matching requirements and admitted buffer dedication.
				unsafe { allocate_storage(allocator, &request, dedicated) }
			}
		}
		Err(error) => Err(error.into()),
	};
	let mut allocation = match result {
		Ok(allocation) => allocation,
		Err(error) => {
			// SAFETY: unbound new buffer, cached immutable destruction PFN preflighted.
			unsafe { allocator.device.destroy_buffer(buffer, None) }.expect("preflighted cleanup");
			return Err(error);
		}
	};
	if options.persistent_map {
		// SAFETY: storage is host visible; no pointers have been published.
		if let Err(error) = unsafe { map(allocator, &allocation) } {
			// SAFETY: newly created buffer/storage only; no external use exists.
			unsafe {
				allocator
					.device
					.destroy_buffer(buffer, None)
					.expect("preflighted cleanup");
				free(allocator, &mut allocation).expect("preflighted allocation rollback");
			}
			return Err(error);
		}
		let live = allocator
			.live
			.get_mut(&allocation.serial)
			.expect("new allocation");
		live.mapped = false;
		live.persistent = true;
	}
	// SAFETY: matching queried requirements, legal dedication or isolated pooled offset.
	if let Err(error) = unsafe {
		allocator
			.device
			.bind_buffer_memory(buffer, allocation.handle, allocation.offset)
	} {
		// SAFETY: only newly acquired objects are rolled back; no GPU use occurred.
		unsafe {
			allocator
				.device
				.destroy_buffer(buffer, None)
				.expect("preflighted cleanup");
			free(allocator, &mut allocation).expect("preflighted allocation rollback");
		}
		return Err(error.into());
	}
	allocator.live.get_mut(&allocation.serial).unwrap().buffer = Some(buffer);
	Ok((buffer, allocation))
}

// Donor: VMA BlockVector existing-block-first policy, explicit-size blocks,
// FindMemoryTypeIndex retry, GetMemoryTypeMinAlignment. Adaptations: homogeneous
// buffers using the verified virtual TLSF bucket geometry, insertion-order block
// search, atom-rounded reservation and immediate empty-block release (no hysteresis).
unsafe fn allocate_pooled(
	allocator: &mut Allocator<'_>,
	request: &AllocRequest,
	block_size: u64,
	strategy: super::PlacementStrategy,
) -> Result<Allocation, AllocError> {
	allocator.validate_size(block_size)?;
	let mut bits = request.memory_type_bits;
	let mut last = None;
	loop {
		let candidate = AllocRequest {
			memory_type_bits: bits,
			..*request
		};
		let index = match allocator.selector.find_memory_type(&candidate) {
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
		let alignment = request.alignment.max(atom);
		if !alignment.is_power_of_two() || !request.alignment.is_power_of_two() || request.size == 0 {
			return Err(AllocError::InvalidRequest(
				"invalid native buffer requirements",
			));
		}
		let reserved = request
			.size
			.checked_add(atom - 1)
			.map(|v| v & !(atom - 1))
			.ok_or(AllocError::InvalidRange)?;
		if reserved > block_size {
			// The nominal requirement fits, but its private atom extent does not.
			last = Some(AllocError::OutOfSpace);
			continue;
		}
		for block in &mut allocator.blocks {
			if block.memory_type_index != index || block.size != block_size {
				continue;
			}
			let placement = match block.metadata.allocate(reserved, alignment, strategy) {
				Ok(placement) => placement,
				Err(AllocError::OutOfSpace) => continue,
				Err(error) => return Err(error),
			};
			let offset = block
				.metadata
				.info(placement.0, placement.1)
				.expect("new placement")
				.offset;
			let handle = block.handle;
			return Ok(publish(
				allocator,
				Region {
					handle,
					index,
					size: request.size,
					reserved_size: reserved,
					offset,
					block_size,
					placement: Some(placement),
				},
			));
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
		let mut metadata = Tlsf::new(block_size)?;
		let placement = metadata.allocate(reserved, alignment, strategy)?;
		let info = vk::MemoryAllocateInfo {
			allocation_size: block_size,
			memory_type_index: index,
			..Default::default()
		};
		// SAFETY: admitted plain buffer storage, native limits and all metadata preflighted.
		let handle = match unsafe { allocator.device.allocate_memory(&info, None) } {
			Ok(handle) => handle,
			Err(error) if recoverable(&error) => {
				last = Some(error.into());
				continue;
			}
			Err(error) => return Err(error.into()),
		};
		if handle.is_null() || allocator.contains_memory(handle) {
			return Err(
				CallError::InvalidOutput("allocator returned a null or already-live memory handle").into(),
			);
		}
		allocator.blocks.push(Block {
			handle,
			size: block_size,
			memory_type_index: index,
			metadata,
			mapping: Mapping::default(),
		});
		allocator.add_native(block_size, index);
		return Ok(publish(
			allocator,
			Region {
				handle,
				index,
				size: request.size,
				reserved_size: reserved,
				offset: 0,
				block_size,
				placement: Some(placement),
			},
		));
	}
}

/// Destroy a checked managed resource pair in buffer-before-memory order.
/// # Safety
/// Complete all host/GPU uses. Neither object may have been externally destroyed.
pub unsafe fn destroy_buffer(
	allocator: &mut Allocator<'_>,
	buffer: vk::Buffer,
	allocation: &mut Allocation,
) -> Result<(), AllocError> {
	let live = allocator.validate(allocation)?;
	if live.buffer != Some(buffer) {
		return Err(AllocError::WrongResource);
	}
	if live.mapped {
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
		allocator.device.destroy_buffer(buffer, None)?;
		free(allocator, allocation)
	}
}

#[cfg(test)]
#[path = "../../../test/rs/memory/test_lifecycle.rs"]
mod tests;
