//! Resource-aware type queries and unclassified native requests, ported from
//! pinned VMA 3.4.0 `FindMemoryTypeIndexForBufferInfo`,
//! `FindMemoryTypeIndexForImageInfo` and `vmaAllocateMemory` (LICENSE-VMA-MIT).
//! Adaptation: cached exact PFNs, checked identities, conservative native pages
//! and the same fallible placement, mapping and accounting as managed resources.
use super::*;

pub(super) unsafe fn validate_buffer(
	allocator: &Allocator<'_>,
	info: &vk::BufferCreateInfo,
) -> Result<bool, AllocError> {
	allocator.ensure_open()?;
	// SAFETY: caller supplies a readable native create chain for this call.
	unsafe {
		super::super::resource::validate_chain(allocator.device.as_device(), info.p_next, false)
	}?;
	let protected = info.flags.as_raw() & vk::BufferCreateFlagBits::PROTECTED_BIT.as_raw() != 0;
	if protected && !allocator.protected_memory {
		return Err(AllocError::UnsupportedCapability(
			"protected buffer requires enabled feature metadata",
		));
	}
	if info.flags.as_raw() & !vk::BufferCreateFlagBits::PROTECTED_BIT.as_raw() != 0
		|| info.size == 0
		|| info.usage.as_raw() == 0
		|| info.usage.as_raw() & vk::BufferUsageFlagBits::TILE_MEMORY_BIT_QCOM.as_raw() != 0
	{
		return Err(AllocError::InvalidRequest("ordinary nonempty buffers only"));
	}
	let device_address =
		info.usage.as_raw() & vk::BufferUsageFlagBits::SHADER_DEVICE_ADDRESS_BIT.as_raw() != 0;
	if device_address && !allocator.buffer_device_address {
		return Err(AllocError::UnsupportedCapability(
			"buffer device address requires enabled core/KHR feature metadata",
		));
	}
	Ok(device_address)
}

pub(super) unsafe fn validate_image(
	allocator: &Allocator<'_>,
	info: &vk::ImageCreateInfo,
) -> Result<ResourceClass, AllocError> {
	allocator.ensure_open()?;
	use crate::ImageCreateFlagBits as Flag;
	let allowed = Flag::MUTABLE_FORMAT_BIT
		| Flag::CUBE_COMPATIBLE_BIT
		| Flag::_2D_ARRAY_COMPATIBLE_BIT
		| Flag::BLOCK_TEXEL_VIEW_COMPATIBLE_BIT
		| Flag::EXTENDED_USAGE_BIT
		| Flag::ALIAS_BIT
		| Flag::PROTECTED_BIT;
	if info.flags.as_raw() & Flag::PROTECTED_BIT.as_raw() != 0 && !allocator.protected_memory {
		return Err(AllocError::UnsupportedCapability(
			"protected image requires enabled feature metadata",
		));
	}
	if info.flags.as_raw() & !allowed.as_raw() != 0
		|| info.extent.width == 0
		|| info.extent.height == 0
		|| info.extent.depth == 0
		|| info.mip_levels == 0
		|| info.array_layers == 0
		|| info.usage.as_raw() == 0
	{
		return Err(AllocError::InvalidRequest(
			"ordinary nonempty non-disjoint images only",
		));
	}
	let class = match info.tiling {
		vk::ImageTiling::LINEAR => ResourceClass::LinearImage,
		vk::ImageTiling::OPTIMAL => ResourceClass::OptimalImage,
		_ => return Err(AllocError::UnsupportedCapability("image tiling")),
	};
	// SAFETY: caller supplies readable native create records and pointed-to storage.
	unsafe {
		super::super::resource::validate_chain(allocator.device.as_device(), info.p_next, true)
	}?;
	Ok(class)
}

pub(super) fn resource_properties(
	allocator: &Allocator<'_>,
	protected: bool,
	required: vk::MemoryPropertyFlags,
) -> Result<vk::MemoryPropertyFlags, AllocError> {
	let flag = vk::MemoryPropertyFlagBits::PROTECTED_BIT;
	if protected {
		if !allocator.protected_memory {
			return Err(AllocError::UnsupportedCapability(
				"protected memory feature is not enabled",
			));
		}
		Ok(required | flag)
	} else if required.as_raw() & flag.as_raw() != 0 {
		Err(AllocError::InvalidRequest(
			"non-protected resource cannot use protected memory",
		))
	} else {
		Ok(required)
	}
}

pub(super) fn validate_options(
	allocator: &Allocator<'_>,
	options: &AllocationCreateInfo,
	required: vk::MemoryPropertyFlags,
) -> Result<(), AllocError> {
	allocator.ensure_allocation()?;
	if matches!(allocator.partition, parallel::Domain::Pool(_)) && options.pool.is_none() {
		return Err(AllocError::InvalidRequest(
			"pool scope requires its selected pool",
		));
	}
	let forced = options.dedicated_memory || options.memory_allocate_next.is_some();
	if forced && (options.never_allocate || options.upper_address) {
		return Err(AllocError::InvalidRequest(
			"separate storage conflicts with reuse/upper placement",
		));
	}
	if let Some(pool) = options.pool {
		let state = pool::validate(allocator, pool)?;
		if forced && state.info.block_size != 0 {
			return Err(AllocError::UnsupportedCapability(
				"fixed pool cannot dedicate",
			));
		}
		if options.memory_allocate_next.is_some() && !state.allocation_chain.next.is_null() {
			return Err(AllocError::InvalidRequest(
				"allocation and pool chains cannot be combined",
			));
		}
	}
	if options.within_budget {
		budget::validate_profile(allocator)?;
	}
	let priority = if let Some(pool) = options.pool {
		pool::validate_options(allocator, pool, options)?
			.info
			.priority
	} else {
		options.priority
	};
	if priority != MemoryPriority::default() && !allocator.memory_priority {
		return Err(AllocError::UnsupportedCapability(
			"memory priority requires enabled EXT feature metadata",
		));
	}
	if !allocator.protected_memory
		&& required.as_raw() & vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw() != 0
	{
		return Err(AllocError::UnsupportedCapability("protected allocation"));
	}
	if let Some(pool) = options.pool {
		pool::validate_options(allocator, pool, options)?;
	} else {
		if options.upper_address {
			return Err(AllocError::UnsupportedCapability(
				"upper address requires a single-block linear pool",
			));
		}
		validate_policy(options.policy)?;
	}
	if options.min_allocation_alignment != 0 && !options.min_allocation_alignment.is_power_of_two() {
		return Err(AllocError::InvalidRequest(
			"minimum allocation alignment must be zero or a power of two",
		));
	}
	Ok(())
}

fn explicit(options: &AllocationCreateInfo) -> Result<(), AllocError> {
	if options.usage != super::super::MemoryUsage::Explicit {
		return Err(AllocError::UnsupportedCapability(
			"raw requirements have no creation usage; use explicit memory flags",
		));
	}
	Ok(())
}

fn select(
	allocator: &Allocator<'_>,
	bits: u32,
	options: &AllocationCreateInfo,
	device_access: bool,
	protected: Option<bool>,
) -> Result<u32, AllocError> {
	allocator.ensure_open()?;
	let (mut required, preferred, avoided) =
		super::super::policy::preferences(options, device_access, allocator.integrated)?;
	if let Some(protected) = protected {
		required = resource_properties(allocator, protected, required)?;
	}
	validate_options(allocator, options, required)?;
	let mut bits = bits & options.memory_type_bits;
	if let Some(pool) = options.pool {
		bits &= 1 << pool::validate(allocator, pool)?.info.memory_type_index;
	}
	// Selection is independent of allocation pressure and requested storage size.
	// Preserve the selector's required/preferred/avoided cost and native exclusions.
	allocator.selector.find_memory_type_with_preferences(
		&AllocRequest {
			size: 1,
			alignment: 1,
			memory_type_bits: bits,
			required_flags: required,
			preferred_flags: preferred,
		},
		avoided,
	)
}

/// Select from explicit native memory-type bits without allocating or querying a budget.
///
/// Pool identity/type, masks and mandatory properties remain checked. This does
/// not reserve storage or promise allocation success. Automatic usage requires
/// a creation-info query instead. Zero mask means no compatible type.
/// # Errors
/// Returns closed/pool identity, unsupported policy or incompatible-type errors.
pub fn find_memory_type(
	allocator: &Allocator<'_>,
	memory_type_bits: u32,
	options: &AllocationCreateInfo,
) -> Result<u32, AllocError> {
	allocator.ensure_open()?;
	explicit(options)?;
	select(allocator, memory_type_bits, options, false, None)
}

enum QueryRoute {
	Core,
	Khr,
}

fn direct_route(allocator: &Allocator<'_>) -> Result<Option<QueryRoute>, AllocError> {
	let capabilities =
		allocator
			.device
			.as_device()
			.capabilities()
			.ok_or(AllocError::UnsupportedCapability(
				"resource queries require device creation metadata",
			))?;
	Ok(if capabilities.api_version() >= vk::API_VERSION_1_3 {
		Some(QueryRoute::Core)
	} else if super::super::resource::extension(allocator.device.as_device(), c"VK_KHR_maintenance4")
	{
		Some(QueryRoute::Khr)
	} else {
		None
	})
}

fn valid_requirements(memory: vk::MemoryRequirements) -> Result<u32, AllocError> {
	if memory.size == 0 || !memory.alignment.is_power_of_two() {
		return Err(AllocError::InvalidRequest(
			"invalid native resource requirements",
		));
	}
	Ok(memory.memory_type_bits)
}

/// Select memory for an ordinary buffer using its actual creation usage and requirements.
///
/// Core 1.3 or enabled KHR maintenance4 queries without creating a resource.
/// Older versions create/query/destroy one temporary unbound buffer. No memory
/// is allocated, mapped or bound; allocator accounting and live tokens stay unchanged.
/// This is type selection, not an allocation/budget guarantee.
/// # Errors
/// Reports invalid/unsupported creation policy, missing exact PFNs, native create
/// failure or incompatible requirements/properties. Temporary resources are destroyed
/// after successful creation even if requirement validation or selection fails.
/// # Safety
/// Creation records/chains must satisfy enabled native valid usage and remain
/// readable for this call; device metadata, callbacks and host synchronization must be valid.
pub unsafe fn find_memory_type_for_buffer_info(
	allocator: &Allocator<'_>,
	info: &vk::BufferCreateInfo,
	options: &AllocationCreateInfo,
) -> Result<u32, AllocError> {
	// SAFETY: identical create-record lifetime/valid-usage contract.
	unsafe { validate_buffer(allocator, info) }?;
	// Validate policy and pool identity before any temporary native creation.
	let (mut required, _, _) = super::super::policy::preferences(
		options,
		info.usage.as_raw() & !3 != 0,
		allocator.integrated,
	)?;
	let protected = info.flags.as_raw() & vk::BufferCreateFlagBits::PROTECTED_BIT.as_raw() != 0;
	required = resource_properties(allocator, protected, required)?;
	validate_options(allocator, options, required)?;
	let mut output = vk::MemoryRequirements2::default();
	if let Some(route) = direct_route(allocator)? {
		let query = vk::DeviceBufferMemoryRequirements {
			p_create_info: info,
			..Default::default()
		};
		// SAFETY: admitted exact core/KHR command and live initialized records.
		unsafe {
			match route {
				QueryRoute::Core => allocator
					.device
					.as_device()
					.get_device_buffer_memory_requirements(&query, &mut output)?,
				QueryRoute::Khr => allocator
					.device
					.as_device()
					.get_device_buffer_memory_requirements_khr(&query, &mut output)?,
			}
		}
	} else {
		let fp = allocator.device.fp();
		require(fp.create_buffer.is_some(), "vkCreateBuffer")?;
		require(fp.destroy_buffer.is_some(), "vkDestroyBuffer")?;
		require(
			fp.get_buffer_memory_requirements.is_some(),
			"vkGetBufferMemoryRequirements",
		)?;
		// SAFETY: caller establishes native creation validity; inverse PFN preflighted.
		let buffer = unsafe { allocator.device.create_buffer(info, allocator.callbacks) }?;
		if buffer.is_null() || binding::associated(allocator, Resource::Buffer(buffer)) {
			return Err(CallError::InvalidOutput("temporary buffer is null or already live").into());
		}
		// SAFETY: new unbound temporary; no GPU work or allocation exists.
		let queried = unsafe { allocator.device.get_buffer_memory_requirements(buffer) };
		// SAFETY: new temporary uses the same retained callbacks; immutable PFN preflighted.
		unsafe { allocator.device.destroy_buffer(buffer, allocator.callbacks) }
			.expect("preflighted temporary cleanup");
		output.memory_requirements = queried?;
	}
	select(
		allocator,
		valid_requirements(output.memory_requirements)?,
		options,
		info.usage.as_raw() & !3 != 0,
		Some(protected),
	)
}

/// Select memory for an ordinary non-disjoint image from creation usage and native requirements.
///
/// Uses exact core/KHR maintenance4 when admitted, otherwise a temporary unbound
/// image with preflighted cleanup. No allocation/binding/mapping or budget query occurs.
/// # Errors
/// Same policy, identity, missing-command, native creation and compatibility errors
/// as `find_memory_type_for_buffer_info`; all created temporaries are cleaned up.
/// # Safety
/// Image/format/feature, chain, callback and host synchronization valid usage must
/// hold for the complete call. Sparse/disjoint/DRM/external paths are excluded; protected queries require captured enablement.
pub unsafe fn find_memory_type_for_image_info(
	allocator: &Allocator<'_>,
	info: &vk::ImageCreateInfo,
	options: &AllocationCreateInfo,
) -> Result<u32, AllocError> {
	// SAFETY: identical creation-record and enabled-feature contract.
	unsafe { validate_image(allocator, info) }?;
	let (mut required, _, _) = super::super::policy::preferences(
		options,
		info.usage.as_raw() & !3 != 0,
		allocator.integrated,
	)?;
	let protected = info.flags.as_raw() & vk::ImageCreateFlagBits::PROTECTED_BIT.as_raw() != 0;
	required = resource_properties(allocator, protected, required)?;
	validate_options(allocator, options, required)?;
	let mut output = vk::MemoryRequirements2::default();
	if let Some(route) = direct_route(allocator)? {
		let query = vk::DeviceImageMemoryRequirements {
			p_create_info: info,
			..Default::default()
		};
		// SAFETY: ordinary non-disjoint/non-DRM image; initialized output and admitted command.
		unsafe {
			match route {
				QueryRoute::Core => allocator
					.device
					.as_device()
					.get_device_image_memory_requirements(&query, &mut output)?,
				QueryRoute::Khr => allocator
					.device
					.as_device()
					.get_device_image_memory_requirements_khr(&query, &mut output)?,
			}
		}
	} else {
		let fp = allocator.device.fp();
		require(fp.create_image.is_some(), "vkCreateImage")?;
		require(fp.destroy_image.is_some(), "vkDestroyImage")?;
		require(
			fp.get_image_memory_requirements.is_some(),
			"vkGetImageMemoryRequirements",
		)?;
		// SAFETY: valid ordinary image and preflighted matching inverse.
		let image = unsafe { allocator.device.create_image(info, allocator.callbacks) }?;
		if image.is_null() || binding::associated(allocator, Resource::Image(image)) {
			return Err(CallError::InvalidOutput("temporary image is null or already live").into());
		}
		// SAFETY: new unbound temporary with initialized synchronous output.
		let queried = unsafe {
			allocator
				.device
				.as_device()
				.get_image_memory_requirements(image, &mut output.memory_requirements)
		};
		// SAFETY: same retained callbacks and no native uses; immutable PFN preflighted.
		unsafe { allocator.device.destroy_image(image, allocator.callbacks) }
			.expect("preflighted temporary cleanup");
		queried?;
	}
	select(
		allocator,
		valid_requirements(output.memory_requirements)?,
		options,
		info.usage.as_raw() & !3 != 0,
		Some(protected),
	)
}

/// Allocate unbound ordinary storage from native requirements with explicit policy.
///
/// Supports separate, adaptive and default/custom TLSF/linear pools. Unclassified
/// pooled regions conflict with every native class (including themselves) on a
/// granularity page. Actual offset satisfies native, minimum, pool and atom alignment.
/// Persistent mapping, BDA flags, limits and rollback share the managed path.
/// # Errors
/// Reports invalid requirements/policy, incompatible type, pool identity, limits,
/// placement, bookkeeping or native errors. Mapping failure releases only new storage.
/// # Safety
/// Requirements/physical metadata must be valid for this device. Imports use a
/// validated AllocationChain; size/type-overriding payloads additionally require
/// truthful ImportMemoryInfo. Its resource supplies any required dedication;
/// without it, required native resource dedication remains excluded. Sparse,
/// disjoint, capture/replay and tile semantics remain excluded. Protected memory requires captured enablement; aliases
/// require can_alias and explicit native lifetime management. Complete every access and destroy bound resources before
/// release. Bind once through checked helpers; raw pointer and GPU lifetimes remain caller-owned.
pub unsafe fn allocate_with_options(
	allocator: &mut Allocator<'_>,
	requirements: &vk::MemoryRequirements,
	options: &AllocationCreateInfo,
) -> Result<Allocation, AllocError> {
	allocator.ensure_open()?;
	explicit(options)?;
	valid_requirements(*requirements)?;
	let (required, preferred, avoided) =
		super::super::policy::preferences(options, false, allocator.integrated)?;
	validate_options(allocator, options, required)?;
	if options.persistent_map {
		require(allocator.device.fp().map_memory.is_some(), "vkMapMemory")?;
		require(
			allocator.device.fp().unmap_memory.is_some(),
			"vkUnmapMemory",
		)?;
	}
	let request = AllocRequest {
		size: requirements.size,
		alignment: requirements.alignment.max(options.min_allocation_alignment),
		memory_type_bits: requirements.memory_type_bits & options.memory_type_bits,
		required_flags: required,
		preferred_flags: preferred,
	};
	allocator.reserve()?;
	// SAFETY: validated ordinary requirements; no resource-specific dedication exists.
	let mut allocation = unsafe {
		allocate_managed(
			allocator,
			&request,
			StorageOptions {
				chain: Default::default(),
				dedicated: None,
				device_address: allocator.buffer_device_address,
				priority: options.pool.map_or(options.priority, |pool| {
					allocator.pools[&pool.serial].info.priority
				}),
				within_budget: options.within_budget,
			},
			Dedication::None,
			options,
			ResourceClass::Unknown,
			avoided,
		)
	}?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("new raw allocation")
		.can_alias = options.can_alias;
	// SAFETY: new unbound host-visible storage; inverse PFNs preflighted.
	if let Err(error) = unsafe { binding::persistent(allocator, &allocation, options) } {
		// SAFETY: only unpublished new storage is released, with no resource or GPU use.
		unsafe { rollback_allocation(allocator, &mut allocation) }
			.expect("preflighted allocation rollback");
		return Err(error);
	}
	Ok(allocation)
}

impl Allocator<'_> {
	/// Select from explicit requirements bits without allocating native storage.
	/// # Errors
	/// Same checked policy/type/identity errors as `mem::find_memory_type`.
	pub fn find_memory_type(
		&self,
		bits: u32,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		find_memory_type(self, bits, options)
	}

	/// Select using ordinary buffer creation usage and native requirements.
	/// # Safety
	/// Same native valid-usage/lifetime contract as `mem::find_memory_type_for_buffer_info`.
	pub unsafe fn find_memory_type_for_buffer_info(
		&self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		// SAFETY: caller supplies the canonical creation-record guarantees.
		unsafe { find_memory_type_for_buffer_info(self, info, options) }
	}

	/// Select using ordinary image creation usage and native requirements.
	/// # Safety
	/// Same native valid-usage/lifetime contract as `mem::find_memory_type_for_image_info`.
	pub unsafe fn find_memory_type_for_image_info(
		&self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo,
	) -> Result<u32, AllocError> {
		// SAFETY: caller supplies the canonical creation-record guarantees.
		unsafe { find_memory_type_for_image_info(self, info, options) }
	}

	/// Allocate raw separate/pooled storage using the canonical managed placement path.
	/// # Safety
	/// Same ordinary requirements, completion and release contract as `mem::allocate_with_options`.
	pub unsafe fn allocate_with_options(
		&mut self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical native requirements and lifetime contract.
		unsafe { allocate_with_options(self, requirements, options) }
	}
}

fn dedicated_options<'a>(
	options: &AllocationCreateInfo<'a>,
	chain: Option<super::super::AllocationChain<'a>>,
) -> Result<AllocationCreateInfo<'a>, AllocError> {
	if chain.is_some() && options.memory_allocate_next.is_some() {
		return Err(AllocError::InvalidRequest(
			"two per-allocation chains supplied",
		));
	}
	Ok(AllocationCreateInfo {
		dedicated_memory: true,
		memory_allocate_next: chain.or(options.memory_allocate_next),
		..*options
	})
}

/// Allocate separate native storage with an optional borrowed allocation chain.
/// Equivalent to `allocate_with_options` with `dedicated_memory` enabled.
/// # Errors
/// Preserves ordinary policy/type/native failures and rejects conflicting chains.
/// # Safety
/// Requirements/type/size and every chain payload must meet native valid usage.
/// Successful native imports transfer handles according to their Vulkan contract,
/// even if later setup fails; imported contents are not debug-filled or poisoned.
pub unsafe fn allocate_dedicated<'a>(
	allocator: &mut Allocator<'_>,
	requirements: &vk::MemoryRequirements,
	options: &AllocationCreateInfo<'a>,
	chain: Option<super::super::AllocationChain<'a>>,
) -> Result<Allocation, AllocError> {
	let options = dedicated_options(options, chain)?;
	// SAFETY: same native requirements/chain and imported-handle obligations.
	unsafe { allocate_with_options(allocator, requirements, &options) }
}

/// Create, separately allocate and bind a buffer, optionally importing/exporting memory.
/// # Errors
/// Same transaction/rollback errors as `create_buffer_with_options` and chain conflicts.
/// # Safety
/// All resource and allocation chain payloads, external compatibility, callback
/// and completion requirements must hold. Successful import handle transfer
/// survives later setup failure; resource/native storage rollback does not undo it.
pub unsafe fn create_dedicated_buffer<'a>(
	allocator: &mut Allocator<'_>,
	info: &vk::BufferCreateInfo,
	options: &AllocationCreateInfo<'a>,
	chain: Option<super::super::AllocationChain<'a>>,
) -> Result<(vk::Buffer, Allocation), AllocError> {
	let options = dedicated_options(options, chain)?;
	// SAFETY: same complete resource/allocation native transaction obligations.
	unsafe { create_buffer_with_options(allocator, info, &options) }
}

/// Create, separately allocate and bind an ordinary non-disjoint image with an optional chain.
/// # Errors
/// Same transaction/rollback errors as `create_image_with_options` and chain conflicts.
/// # Safety
/// Same native/external valid usage and successful-import transfer contract as
/// `create_dedicated_buffer`, applied to the image and its queried requirements.
pub unsafe fn create_dedicated_image<'a>(
	allocator: &mut Allocator<'_>,
	info: &vk::ImageCreateInfo,
	options: &AllocationCreateInfo<'a>,
	chain: Option<super::super::AllocationChain<'a>>,
) -> Result<(vk::Image, Allocation), AllocError> {
	let options = dedicated_options(options, chain)?;
	// SAFETY: same complete resource/allocation native transaction obligations.
	unsafe { create_image_with_options(allocator, info, &options) }
}

impl Allocator<'_> {
	/// Allocate separate native storage with a borrowed chain.
	/// # Errors
	/// Same checked/native/chain errors as `mem::allocate_dedicated`.
	/// # Safety
	/// Same requirements, chain, import ownership and completion obligations.
	pub unsafe fn allocate_dedicated<'a>(
		&mut self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo<'a>,
		chain: Option<super::super::AllocationChain<'a>>,
	) -> Result<Allocation, AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { allocate_dedicated(self, requirements, options, chain) }
	}

	/// Create a separately allocated/bound buffer with an optional native chain.
	/// # Errors
	/// Same checked/native/rollback errors as `mem::create_dedicated_buffer`.
	/// # Safety
	/// Same complete resource/allocation external-memory and completion obligations.
	pub unsafe fn create_dedicated_buffer<'a>(
		&mut self,
		info: &vk::BufferCreateInfo,
		options: &AllocationCreateInfo<'a>,
		chain: Option<super::super::AllocationChain<'a>>,
	) -> Result<(vk::Buffer, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { create_dedicated_buffer(self, info, options, chain) }
	}

	/// Create a separately allocated/bound image with an optional native chain.
	/// # Errors
	/// Same checked/native/rollback errors as `mem::create_dedicated_image`.
	/// # Safety
	/// Same complete resource/allocation external-memory and completion obligations.
	pub unsafe fn create_dedicated_image<'a>(
		&mut self,
		info: &vk::ImageCreateInfo,
		options: &AllocationCreateInfo<'a>,
		chain: Option<super::super::AllocationChain<'a>>,
	) -> Result<(vk::Image, Allocation), AllocError> {
		// SAFETY: identical canonical operation contract.
		unsafe { create_dedicated_image(self, info, options, chain) }
	}
}
