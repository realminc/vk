//! Existing-resource allocation and checked binding, following pinned VMA 3.4.0
//! `vmaAllocateMemoryForBuffer`, `vmaAllocateMemoryForImage`, and ordinary binds.
//! See LICENSE-VMA-MIT. Raw handles reveal no creation usage/tiling; automatic
//! usage policies are rejected and unknown image placement is conservative.
use super::*;

struct Requirements {
	memory: vk::MemoryRequirements,
	dedication: Dedication,
	dedicated_supported: bool,
}

unsafe fn requirements(
	allocator: &Allocator<'_>,
	resource: Resource,
) -> Result<Requirements, AllocError> {
	// SAFETY: caller supplies a live ordinary resource; no disjoint plane is requested.
	unsafe { requirements_with_plane(allocator, resource, None) }
}

unsafe fn requirements_with_plane(
	allocator: &Allocator<'_>,
	resource: Resource,
	plane: Option<vk::ImageAspectFlagBits>,
) -> Result<Requirements, AllocError> {
	let route = allocator.requirements_route(matches!(resource, Resource::Image(_)))?;
	// SAFETY: same live native resource/plane query contract.
	unsafe { query_requirements(allocator.device.as_device(), route, resource, plane) }
}

unsafe fn query_requirements(
	device: &vk::Device,
	route: RequirementsRoute,
	resource: Resource,
	plane: Option<vk::ImageAspectFlagBits>,
) -> Result<Requirements, AllocError> {
	if plane.is_some() && matches!(route, RequirementsRoute::Legacy) {
		return Err(AllocError::UnsupportedCapability(
			"disjoint planes require memory requirements2",
		));
	}
	let plane_info = vk::ImagePlaneMemoryRequirementsInfo {
		plane_aspect: plane.unwrap_or_default(),
		..Default::default()
	};
	let mut dedication = vk::MemoryDedicatedRequirements::default();
	let mut output = vk::MemoryRequirements2 {
		p_next: std::ptr::from_mut(&mut dedication).cast(),
		..Default::default()
	};
	// SAFETY: caller supplies a live ordinary resource from this device; exact
	// route preflights its PFN and both writable output records remain in scope.
	unsafe {
		match resource {
			Resource::Buffer(buffer) => {
				let query = vk::BufferMemoryRequirementsInfo2 {
					buffer,
					..Default::default()
				};
				match route {
					RequirementsRoute::Legacy => {
						output.memory_requirements = device.get_buffer_memory_requirements(buffer)?
					}
					RequirementsRoute::Core => device.get_buffer_memory_requirements2(&query, &mut output)?,
					RequirementsRoute::Khr => {
						device.get_buffer_memory_requirements2_khr(&query, &mut output)?
					}
				}
			}
			Resource::Image(image) => {
				let query = vk::ImageMemoryRequirementsInfo2 {
					p_next: if plane.is_some() {
						std::ptr::from_ref(&plane_info).cast()
					} else {
						std::ptr::null()
					},
					image,
					..Default::default()
				};
				match route {
					RequirementsRoute::Legacy => {
						device.get_image_memory_requirements(image, &mut output.memory_requirements)?
					}
					RequirementsRoute::Core => device.get_image_memory_requirements2(&query, &mut output)?,
					RequirementsRoute::Khr => {
						device.get_image_memory_requirements2_khr(&query, &mut output)?
					}
				}
			}
		}
	}
	if output.memory_requirements.size == 0 || !output.memory_requirements.alignment.is_power_of_two()
	{
		return Err(AllocError::InvalidRequest(
			"invalid native resource requirements",
		));
	}
	Ok(Requirements {
		memory: output.memory_requirements,
		dedication: Dedication::new(&dedication),
		dedicated_supported: !matches!(route, RequirementsRoute::Legacy),
	})
}

pub(super) fn associated(allocator: &Allocator<'_>, resource: Resource) -> bool {
	if let Some(shared) = allocator.parallel_state() {
		let domain = allocator.partition_id;
		if shared.associated(resource, domain) {
			return true;
		}
	}
	allocator.live.values().any(|live| {
		live.resource == Some(resource)
			|| live
				.pending_binding
				.is_some_and(|(pending, _)| pending == resource)
	})
}

pub(super) unsafe fn allocate_resource(
	allocator: &mut Allocator<'_>,
	resource: Resource,
	options: &AllocationCreateInfo,
	preferences: (
		vk::MemoryPropertyFlags,
		vk::MemoryPropertyFlags,
		vk::MemoryPropertyFlags,
	),
	class: ResourceClass,
	device_address: bool,
) -> Result<Allocation, AllocError> {
	allocator.ensure_open()?;
	if resource.is_null() {
		return Err(AllocError::InvalidRequest("null resource"));
	}
	let (required, preferred, avoided) = preferences;
	request::validate_options(allocator, options, required)?;
	if options.persistent_map {
		require(allocator.device.fp().map_memory.is_some(), "vkMapMemory")?;
		require(
			allocator.device.fp().unmap_memory.is_some(),
			"vkUnmapMemory",
		)?;
	}
	allocator.reserve()?;
	let claim = parallel::claim(allocator, resource, None, None)?;
	let imported = options
		.memory_allocate_next
		.and_then(|chain| chain.data.import_info);
	let requirements = if let Some(info) = imported.filter(|info| info.resource.is_some()) {
		if import_resource(info.resource) != Some(resource) {
			return Err(AllocError::WrongResource);
		}
		// The unsafe import constructor admits exact external/resource metadata.
		// In particular, Android image requirements cannot be queried before binding.
		Requirements {
			memory: vk::MemoryRequirements {
				size: info.allocation_size,
				alignment: 1,
				memory_type_bits: info.memory_type_bits,
			},
			dedication: Dedication::Required,
			dedicated_supported: true,
		}
	} else {
		if options
			.memory_allocate_next
			.is_some_and(|chain| chain.data.import_kind.requires_info())
		{
			return Err(AllocError::InvalidRequest(
				"platform resource imports require exact resource metadata",
			));
		}
		// SAFETY: exact live ordinary resource supplied by caller; no platform pre-bind query.
		unsafe { requirements(allocator, resource) }?
	};
	let request = AllocRequest {
		size: requirements.memory.size,
		alignment: requirements
			.memory
			.alignment
			.max(options.min_allocation_alignment),
		memory_type_bits: requirements.memory.memory_type_bits & options.memory_type_bits,
		required_flags: required,
		preferred_flags: preferred,
	};
	let mut dedicated = vk::MemoryDedicatedAllocateInfo::default();
	match resource {
		Resource::Buffer(buffer) => dedicated.buffer = buffer,
		Resource::Image(image) => dedicated.image = image,
	}
	// SAFETY: matching queried requirements, exact supported dedication, and admitted class.
	let allocation = unsafe {
		allocate_managed(
			allocator,
			&request,
			StorageOptions {
				chain: Default::default(),
				dedicated: requirements.dedicated_supported.then_some(&dedicated),
				device_address,
				priority: options.pool.map_or(options.priority, |pool| {
					allocator.pools[&pool.serial].info.priority
				}),
				within_budget: options.within_budget,
			},
			requirements.dedication,
			options,
			class,
			avoided,
		)
	}?;
	let live = allocator
		.live
		.get_mut(&allocation.serial)
		.expect("new allocation");
	live.resource = Some(resource);
	live.class = if options.can_alias {
		ResourceClass::Unknown
	} else {
		class
	};
	live.can_alias = options.can_alias;
	if let Some(claim) = claim {
		claim.commit(allocation.serial);
	}
	Ok(allocation)
}

pub(super) unsafe fn persistent(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	options: &AllocationCreateInfo,
) -> Result<(), AllocError> {
	// SAFETY: unpublished unbound storage; debug mapping/checks retain no user references.
	unsafe { guard::initialize(allocator, allocation) }?;
	if options.persistent_map {
		// SAFETY: caller supplies new host-visible storage with preflighted inverse PFN.
		unsafe { map(allocator, allocation) }?;
		let live = allocator
			.live
			.get_mut(&allocation.serial)
			.expect("new allocation");
		live.map_count = 0;
		live.persistent = true;
	}
	Ok(())
}

unsafe fn allocate_existing(
	allocator: &mut Allocator<'_>,
	resource: Resource,
	options: &AllocationCreateInfo,
) -> Result<Allocation, AllocError> {
	allocator.ensure_open()?;
	if associated(allocator, resource) {
		return Err(AllocError::InvalidRequest(
			"resource already has an allocation",
		));
	}

	if options.usage != super::super::MemoryUsage::Explicit {
		return Err(AllocError::UnsupportedCapability(
			"existing resource usage is unknown; use explicit memory flags",
		));
	}
	let preferences = super::super::policy::preferences(options, false, allocator.integrated)?;
	let class = match resource {
		Resource::Buffer(_) => ResourceClass::Buffer,
		Resource::Image(_) => ResourceClass::UnknownImage,
	};
	let address = matches!(resource, Resource::Buffer(_)) && allocator.buffer_device_address;
	// SAFETY: caller guarantees ordinary resource semantics; unknown usage never enables a feature.
	let mut allocation =
		unsafe { allocate_resource(allocator, resource, options, preferences, class, address) }?;
	// SAFETY: unpublished new allocation; no binding or external access exists.
	if let Err(error) = unsafe { persistent(allocator, &allocation, options) } {
		// SAFETY: unbound storage only; the caller's resource remains untouched.
		unsafe { rollback_allocation(allocator, &mut allocation) }
			.expect("preflighted allocation rollback");
		return Err(error);
	}
	Ok(allocation)
}

/// Allocate for a caller-created unbound ordinary buffer without creating, binding or destroying it.
///
/// Queries exact legacy/core/KHR requirements and dedication. Imports carrying
/// resource-specific ImportMemoryInfo instead use those admitted external
/// requirements without an otherwise illegal pre-bind native query. Policy must use
/// `MemoryUsage::Explicit`; creation usage is unavailable from the opaque handle.
/// BDA-enabled allocators flag separate buffer storage conservatively, following VMA.
/// Allocation/map failure retains the caller's resource. Release unbound storage
/// with `free`; bind it explicitly with `bind_buffer_memory` when ready.
/// # Errors
/// Rejects null/already-associated resources, unsupported policy, identity/type/
/// limit failures or preserves the native allocation/map/query error.
/// # Safety
/// Buffer must be live, unbound, from this device and remain so until binding or
/// allocation release. Supply ordinary non-sparse,
/// non-capture/replay resource semantics; admitted import chains retain their
/// exact external metadata/handle obligations. Address usage requires enabled core/KHR
/// BDA metadata. Protected storage requires enabled features and explicit flags.
/// The caller owns resource lifetime, callbacks and synchronization.
pub unsafe fn allocate_for_buffer(
	allocator: &mut Allocator<'_>,
	buffer: vk::Buffer,
	options: &AllocationCreateInfo,
) -> Result<Allocation, AllocError> {
	// SAFETY: identical native resource/ownership contract.
	unsafe { allocate_existing(allocator, Resource::Buffer(buffer), options) }
}

/// Allocate for a caller-created unbound ordinary image; no creation, binding or destruction.
///
/// Unknown tiling conflicts with every native class, including another unknown
/// image, on a granularity page. Explicit memory flags are required. Native
/// requirements/dedication, pool limits, persistent mapping and rollback share
/// the managed-resource implementation. Resource-specific import metadata
/// replaces pre-bind queries where native external-memory rules forbid them. Failure retains the caller's handle.
/// # Errors
/// Same checked policy, identity/type/limit and native errors as `allocate_for_buffer`.
/// # Safety
/// Image must be live, unbound and from this device. Sparse, disjoint, DRM-modifier,
/// implicit aliasing and unsupported allocation capabilities are excluded. Ordinary
/// external allocation profiles retain caller-owned native compatibility obligations.
/// Protected storage requires captured enablement and explicit required flags.
/// The caller owns resource lifetime, callbacks, valid usage and synchronization.
pub unsafe fn allocate_for_image(
	allocator: &mut Allocator<'_>,
	image: vk::Image,
	options: &AllocationCreateInfo,
) -> Result<Allocation, AllocError> {
	// SAFETY: identical native resource/ownership contract.
	unsafe { allocate_existing(allocator, Resource::Image(image), options) }
}

struct BindChain {
	status: Option<*mut vk::Result>,
	plane: Option<vk::ImageAspectFlagBits>,
}

unsafe fn binding_status(mut next: *const std::ffi::c_void) -> Result<BindChain, AllocError> {
	let mut seen = [std::ptr::null(); 64];
	let mut count = 0;
	let mut status = None;
	let mut plane = None;
	let mut tags = [vk::StructureType::from_raw(0); 64];
	while !next.is_null() {
		if count == seen.len() || seen[..count].contains(&next) {
			return Err(AllocError::InvalidRequest("cyclic/excessive bind chain"));
		}
		seen[count] = next;
		// SAFETY: caller supplies readable native headers and correctly tagged complete records.
		let node = unsafe { &*next.cast::<vk::BaseInStructure>() };
		if tags[..count].contains(&node.s_type) {
			return Err(AllocError::InvalidRequest("duplicate bind chain record"));
		}
		tags[count] = node.s_type;
		count += 1;
		if node.s_type == vk::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO {
			// SAFETY: caller supplies a complete correctly tagged native record.
			let aspect = unsafe { (*next.cast::<vk::BindImagePlaneMemoryInfo>()).plane_aspect };
			use crate::ImageAspectFlagBits as A;
			if !matches!(aspect, A::PLANE_0_BIT | A::PLANE_1_BIT | A::PLANE_2_BIT) {
				return Err(AllocError::UnsupportedCapability(
					"ordinary disjoint image plane aspect required",
				));
			}
			plane = Some(aspect);
		}
		if node.s_type == vk::StructureType::BIND_MEMORY_STATUS {
			// SAFETY: caller supplies the complete native status record matching the tag.
			let pointer = unsafe { (*next.cast::<vk::BindMemoryStatus>()).p_result };
			if status.is_some() || pointer.is_null() {
				return Err(AllocError::InvalidRequest(
					"duplicate/null bind status output",
				));
			}
			status = Some(pointer);
		}
		next = node.p_next.cast();
	}
	Ok(BindChain { status, plane })
}

/// Bind one ordinary buffer with a checked allocation-local offset and borrowed native chain.
///
/// Uses exact core 1.1 or enabled KHR bind_memory2. Associated resource allocations
/// require offset zero; raw allocations re-query type/size/alignment. Any supplied
/// per-bind status must report success before bound state is published.
/// # Errors
/// Reports state/range/capability, malformed chain, per-item status or native errors.
/// # Safety
/// Buffer must be live, ordinary and unbound on this device. Chain records and
/// outputs must be valid and enabled for this exact native call; caller owns their
/// extents/lifetimes and native error/device-loss recovery. No aliasing is admitted.
pub unsafe fn bind_buffer_memory2(
	allocator: &mut Allocator<'_>,
	buffer: vk::Buffer,
	allocation: &Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
) -> Result<(), AllocError> {
	// SAFETY: caller establishes canonical native binding/chain guarantees.
	unsafe {
		bind_resource(
			allocator,
			Resource::Buffer(buffer),
			allocation,
			local_offset,
			next,
			true,
			false,
		)
	}
}

/// Bind an ordinary image or one disjoint plane with an allocation-local offset and native chain.
/// A plane uses unassociated raw storage allocated from caller-queried plane
/// requirements. The allocator re-queries that exact plane before binding and
/// tracks each plane independently; DRM modifier plane aspects remain excluded.
/// # Errors
/// Same checked state/range/chain, per-item and native errors as `bind_buffer_memory2`.
/// # Safety
/// Same native chain/lifetime/completion contract as `bind_buffer_memory2`; image
/// semantics must meet native valid usage for the selected plane. Destroy a
/// disjoint image once, then free all its plane allocations; do not use the
/// ordinary paired `destroy_image` helper for a plane allocation.
pub unsafe fn bind_image_memory2(
	allocator: &mut Allocator<'_>,
	image: vk::Image,
	allocation: &Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
) -> Result<(), AllocError> {
	// SAFETY: caller establishes canonical native image binding/chain guarantees.
	unsafe {
		bind_resource(
			allocator,
			Resource::Image(image),
			allocation,
			local_offset,
			next,
			true,
			false,
		)
	}
}

pub(super) unsafe fn bind_alias(
	allocator: &mut Allocator<'_>,
	resource: Resource,
	allocation: &Allocation,
	local_offset: u64,
) -> Result<(), AllocError> {
	// SAFETY: caller establishes native alias/resource valid usage; backing storage is explicitly aliasable.
	unsafe {
		bind_resource(
			allocator,
			resource,
			allocation,
			local_offset,
			std::ptr::null(),
			false,
			true,
		)
	}
}

unsafe fn bind_resource(
	allocator: &mut Allocator<'_>,
	resource: Resource,
	allocation: &Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
	extended: bool,
	alias: bool,
) -> Result<(), AllocError> {
	// SAFETY: canonical operation checks and borrowed native-chain contract.
	let operation = unsafe {
		prepare_bind(
			allocator,
			resource,
			allocation,
			local_offset,
			next,
			extended,
			alias,
		)
	}?;
	// The exclusive ordinary owner and synchronized backing leases share this
	// exact execution/publication route. No registry guard is needed by execute.
	let backing = allocator.mapping(allocation);
	// SAFETY: caller owns this native resource and meets bind synchronization.
	unsafe { operation.execute() }?;
	drop(backing);
	operation.commit(allocator)
}

pub(super) struct BindOperation<'device> {
	device: DeviceRetention<'device>,
	route: RequirementsRoute,
	requirements_route: Option<RequirementsRoute>,
	resource: Resource,
	allocation: Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
	status: Option<*mut vk::Result>,
	pub(super) plane: Option<vk::ImageAspectFlagBits>,
	alias: bool,
	claim: Option<parallel::resource::Claim<'device>>,
}

pub(super) unsafe fn prepare_bind<'device>(
	allocator: &Allocator<'device>,
	resource: Resource,
	allocation: &Allocation,
	local_offset: u64,
	next: *const std::ffi::c_void,
	extended: bool,
	alias: bool,
) -> Result<BindOperation<'device>, AllocError> {
	let route = if !extended {
		RequirementsRoute::Legacy
	} else {
		let capabilities =
			allocator
				.device
				.as_device()
				.capabilities()
				.ok_or(AllocError::UnsupportedCapability(
					"extended binding requires creation metadata",
				))?;
		if capabilities.api_version() >= vk::API_VERSION_1_1 {
			RequirementsRoute::Core
		} else if super::super::resource::extension(
			allocator.device.as_device(),
			c"VK_KHR_bind_memory2",
		) {
			RequirementsRoute::Khr
		} else {
			return Err(AllocError::UnsupportedCapability(
				"bind_memory2 is not enabled",
			));
		}
	};
	// SAFETY: caller supplies valid readable/writable native bind-chain records for this call.
	let BindChain { status, plane } = unsafe { binding_status(next) }?;
	let live = allocator.validate(allocation)?;
	if live.native_users != 0 {
		return Err(AllocError::InvalidRequest("native operation in progress"));
	}
	if plane.is_some() {
		if !extended || !matches!(resource, Resource::Image(_)) || live.resource.is_some() || alias {
			return Err(AllocError::InvalidRequest(
				"plane binding requires unassociated raw storage and image bind2",
			));
		}
		if !allocator
			.device
			.as_device()
			.capabilities()
			.is_some_and(|caps| caps.api_version() >= vk::API_VERSION_1_1)
			&& !super::super::resource::extension(
				allocator.device.as_device(),
				c"VK_KHR_sampler_ycbcr_conversion",
			) {
			return Err(AllocError::UnsupportedCapability(
				"disjoint plane binding is not enabled",
			));
		}
	}
	if resource.is_null() {
		return Err(AllocError::InvalidRequest("null resource"));
	}
	if alias && !live.can_alias {
		return Err(AllocError::UnsupportedCapability(
			"allocation was not created for aliasing",
		));
	}
	if !alias && live.bound {
		return Err(AllocError::InvalidRequest("allocation already bound"));
	}
	if !alias && live.resource.is_some_and(|target| target != resource) {
		return Err(AllocError::WrongResource);
	}
	let raw = alias || live.resource.is_none();
	if raw
		&& allocator.live.values().any(|other| {
			(other.resource == Some(resource)
				&& (plane.is_none() || other.plane.is_none() || other.plane == plane))
				|| other
					.pending_binding
					.is_some_and(|(pending, pending_plane)| {
						pending == resource
							&& (plane.is_none() || pending_plane.is_none() || pending_plane == plane)
					})
		}) {
		return Err(AllocError::WrongResource);
	}
	let claim = parallel::claim(
		allocator,
		resource,
		plane,
		(!raw).then_some(allocation.serial),
	)?;
	let fp = allocator.device.fp();
	match (resource, route) {
		(Resource::Buffer(_), RequirementsRoute::Legacy) => {
			require(fp.bind_buffer_memory.is_some(), "vkBindBufferMemory")?
		}
		(Resource::Image(_), RequirementsRoute::Legacy) => {
			require(fp.bind_image_memory.is_some(), "vkBindImageMemory")?
		}
		(Resource::Buffer(_), RequirementsRoute::Core) => {
			require(fp.bind_buffer_memory2.is_some(), "vkBindBufferMemory2")?
		}
		(Resource::Image(_), RequirementsRoute::Core) => {
			require(fp.bind_image_memory2.is_some(), "vkBindImageMemory2")?
		}
		(Resource::Buffer(_), RequirementsRoute::Khr) => require(
			fp.bind_buffer_memory2_khr.is_some(),
			"vkBindBufferMemory2KHR",
		)?,
		(Resource::Image(_), RequirementsRoute::Khr) => {
			require(fp.bind_image_memory2_khr.is_some(), "vkBindImageMemory2KHR")?
		}
	}
	let _native_offset = allocation
		.offset
		.checked_add(local_offset)
		.ok_or(AllocError::InvalidRange)?;
	if local_offset > allocation.size || (!raw && local_offset != 0) {
		return Err(AllocError::InvalidRange);
	}
	let requirements_route = if raw {
		Some(allocator.requirements_route(matches!(resource, Resource::Image(_)))?)
	} else {
		None
	};
	Ok(BindOperation {
		device: allocator.device.clone(),
		route,
		requirements_route,
		resource,
		allocation: allocator.token(allocation.serial)?,
		local_offset,
		next,
		status,
		plane,
		alias,
		claim,
	})
}

impl BindOperation<'_> {
	pub(super) unsafe fn execute(&self) -> Result<(), AllocError> {
		let (route, resource, allocation, local_offset, next, status, plane) = (
			self.route,
			self.resource,
			&self.allocation,
			self.local_offset,
			self.next,
			self.status,
			self.plane,
		);
		let native_offset = allocation.offset + local_offset;

		if let Some(requirements_route) = self.requirements_route {
			// SAFETY: caller guarantees the live ordinary resource and native identity.
			let requirements = unsafe {
				query_requirements(self.device.as_device(), requirements_route, resource, plane)
			}?;
			if requirements.dedication == Dedication::Required {
				return Err(AllocError::UnsupportedCapability(
					"raw allocation cannot satisfy required resource dedication",
				));
			}
			if requirements.memory.memory_type_bits & (1 << allocation.memory_type_index) == 0 {
				return Err(AllocError::NoCompatibleMemoryType);
			}
			if requirements.memory.size > allocation.size - local_offset
				|| !native_offset.is_multiple_of(requirements.memory.alignment)
			{
				return Err(AllocError::InvalidRange);
			}
		}
		if let Some(status) = status {
			// SAFETY: validated writable output pointer from caller-owned live bind chain.
			unsafe {
				*status = vk::Result::NOT_READY;
			}
		}
		// SAFETY: one matching ordinary resource, checked storage offset/extent, admitted exact PFN and live caller chain.
		unsafe {
			match resource {
				Resource::Buffer(buffer) => {
					let info = vk::BindBufferMemoryInfo {
						buffer,
						memory: allocation.handle,
						memory_offset: native_offset,
						p_next: next,
						..Default::default()
					};
					match route {
						RequirementsRoute::Legacy => self.device.as_device().bind_buffer_memory(
							buffer,
							allocation.handle,
							native_offset,
						)?,
						RequirementsRoute::Core => self.device.bind_buffer_memory2(&[info])?,
						RequirementsRoute::Khr => self.device.bind_buffer_memory2_khr(&[info])?,
					}
				}
				Resource::Image(image) => {
					let info = vk::BindImageMemoryInfo {
						image,
						memory: allocation.handle,
						memory_offset: native_offset,
						p_next: next,
						..Default::default()
					};
					match route {
						RequirementsRoute::Legacy => {
							self
								.device
								.as_device()
								.bind_image_memory(image, allocation.handle, native_offset)?
						}
						RequirementsRoute::Core => self.device.bind_image_memory2(&[info])?,
						RequirementsRoute::Khr => self.device.bind_image_memory2_khr(&[info])?,
					}
				}
			}
		}
		if let Some(status) = status {
			// SAFETY: caller-owned status storage remains live after the synchronous native call.
			let result = unsafe { *status };
			if result != vk::Result::SUCCESS {
				return Err(
					if result.as_raw() < 0 {
						CallError::Vulkan(result)
					} else {
						CallError::InvalidOutput("native binding did not publish success status")
					}
					.into(),
				);
			}
		}
		Ok(())
	}

	pub(super) fn commit(self, allocator: &mut Allocator<'_>) -> Result<(), AllocError> {
		if self.alias {
			return Ok(());
		}
		let live = allocator
			.live
			.get_mut(&self.allocation.serial)
			.expect("validated allocation");
		live.resource = Some(self.resource);
		live.plane = self.plane;
		live.bound = true;
		if let Some(claim) = self.claim {
			claim.commit(self.allocation.serial);
		}
		Ok(())
	}
}

/// Bind at the allocation's actual native offset. Failed binds retain allocation and resource for retry.
///
/// Allocations for a resource accept only that target. Raw allocations
/// query size/type/alignment and reject required dedication. Repeated library
/// binding and aliasing are rejected. No pointer lookup or resource destruction occurs.
/// # Errors
/// Returns identity/state/association, requirement/type/range or native bind errors.
/// # Safety
/// Buffer must be live, unbound, ordinary and from this device. Meet allocation/
/// feature/lifetime and external host synchronization valid usage. Native calls
/// performed outside this allocator remain caller-owned state; do not rebind them.
pub unsafe fn bind_buffer_memory(
	allocator: &mut Allocator<'_>,
	buffer: vk::Buffer,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	// SAFETY: identical native binding contract.
	unsafe {
		bind_resource(
			allocator,
			Resource::Buffer(buffer),
			allocation,
			0,
			std::ptr::null(),
			false,
			false,
		)
	}
}

/// Bind a live ordinary image at its allocation's native offset; failure retains both objects.
/// # Errors
/// Same checked identity/association/requirement/type/range and native errors as buffer binding.
/// # Safety
/// Image must be live, unbound, non-disjoint/non-sparse/non-external,
/// non-aliased and from this device. Meet native feature and synchronization valid
/// usage. Never infer linear pixel layout from mapping optimal/unknown image storage.
pub unsafe fn bind_image_memory(
	allocator: &mut Allocator<'_>,
	image: vk::Image,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	// SAFETY: identical native binding contract.
	unsafe {
		bind_resource(
			allocator,
			Resource::Image(image),
			allocation,
			0,
			std::ptr::null(),
			false,
			false,
		)
	}
}

impl Allocator<'_> {
	/// Bind an ordinary buffer with explicit local offset and borrowed native bind chain.
	/// # Safety
	/// Same native chain/resource contract as `mem::bind_buffer_memory2`.
	pub unsafe fn bind_buffer_memory2(
		&mut self,
		buffer: vk::Buffer,
		allocation: &Allocation,
		local_offset: u64,
		next: *const std::ffi::c_void,
	) -> Result<(), AllocError> {
		// SAFETY: caller supplies canonical binding/chain guarantees.
		unsafe { bind_buffer_memory2(self, buffer, allocation, local_offset, next) }
	}

	/// Bind an ordinary image with explicit local offset and borrowed native bind chain.
	/// # Safety
	/// Same native chain/resource contract as `mem::bind_image_memory2`.
	pub unsafe fn bind_image_memory2(
		&mut self,
		image: vk::Image,
		allocation: &Allocation,
		local_offset: u64,
		next: *const std::ffi::c_void,
	) -> Result<(), AllocError> {
		// SAFETY: caller supplies canonical image binding/chain guarantees.
		unsafe { bind_image_memory2(self, image, allocation, local_offset, next) }
	}
	/// Allocate for an existing unbound ordinary buffer; retains caller ownership on error.
	/// # Safety
	/// Same resource/feature/callback/lifetime contract as `mem::allocate_for_buffer`.
	pub unsafe fn allocate_for_buffer(
		&mut self,
		buffer: vk::Buffer,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical native contract to the canonical operation.
		unsafe { allocate_for_buffer(self, buffer, options) }
	}

	/// Allocate for an existing ordinary image using conservative unknown-tiling placement.
	/// # Safety
	/// Same resource/feature/callback/lifetime contract as `mem::allocate_for_image`.
	pub unsafe fn allocate_for_image(
		&mut self,
		image: vk::Image,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical native contract to the canonical operation.
		unsafe { allocate_for_image(self, image, options) }
	}

	/// Bind a matching buffer/allocation pair; failure retains both for explicit retry.
	/// # Safety
	/// Same native unbound-resource and synchronization contract as `mem::bind_buffer_memory`.
	pub unsafe fn bind_buffer_memory(
		&mut self,
		buffer: vk::Buffer,
		allocation: &Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical native contract to the canonical operation.
		unsafe { bind_buffer_memory(self, buffer, allocation) }
	}

	/// Bind a matching ordinary image/allocation pair; no implicit destruction on failure.
	/// # Safety
	/// Same native image/lifetime/synchronization contract as `mem::bind_image_memory`.
	pub unsafe fn bind_image_memory(
		&mut self,
		image: vk::Image,
		allocation: &Allocation,
	) -> Result<(), AllocError> {
		// SAFETY: identical native contract to the canonical operation.
		unsafe { bind_image_memory(self, image, allocation) }
	}
}
