//! Native external-memory services. Donor: pinned VMA 3.4.0
//! `pTypeExternalMemoryHandleTypes`, `VmaWin32Handle` and interop allocation chains.
//! Rust adaptation: copied export profiles, exact enabled-extension admission,
//! retained native handle caches and explicit ownership/error paths.
use super::*;
use crate as vk;
use crate::mem::{AllocationChain, chain::ChainData};

pub(super) fn validate_types(
	allocator: &Allocator<'_>,
	types: vk::ExternalMemoryHandleTypeFlags,
) -> Result<(), AllocError> {
	crate::mem::resource::validate_external_types(allocator.device.as_device(), types)
}

pub(super) fn allocation_exports(
	allocator: &Allocator<'_>,
	index: u32,
	chain: ChainData,
) -> Result<vk::ExternalMemoryHandleTypeFlags, AllocError> {
	validate_allocation_chain(allocator, chain)?;
	let configured = allocator.external_types[index as usize];
	if configured.as_raw() != 0 && chain.has_export {
		return Err(AllocError::InvalidRequest(
			"duplicate configured and chained export records",
		));
	}
	let types = configured | chain.exports;
	validate_types(allocator, types)?;
	Ok(types)
}

/// Configure copied per-memory-type external export flags before creating pools/storage.
/// Zero flags leave that type ordinary. Every native allocation of a configured
/// type receives `VkExportMemoryAllocateInfo`, including separate allocations.
/// # Errors
/// Rejects live domains, incorrect array length and unavailable handle extensions.
/// # Safety
/// Flags must be supported and mutually compatible for every resource subsequently
/// allocated on each type, including external dedication requirements. No support
/// query is implied. Matching external resource create chains remain required.
pub unsafe fn set_external_memory_types(
	allocator: &mut Allocator<'_>,
	types: &[vk::ExternalMemoryHandleTypeFlags],
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if allocator.native_count != 0 || !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	if !allocator.pools.is_empty() {
		return Err(AllocError::LivePools);
	}
	if types.len() != allocator.selector.properties.memory_type_count as usize {
		return Err(AllocError::InvalidRequest(
			"external profile must cover exactly the native memory types",
		));
	}
	for &types in types {
		validate_types(allocator, types)?;
	}
	allocator
		.external_types
		.fill(vk::ExternalMemoryHandleTypeFlags::empty());
	allocator.external_types[..types.len()].copy_from_slice(types);
	Ok(())
}

pub(super) fn exports(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
) -> Result<vk::ExternalMemoryHandleTypeFlags, AllocError> {
	let live = allocator.validate(allocation)?;
	if live.placement.is_some() {
		Ok(
			allocator
				.blocks
				.iter()
				.find(|block| block.handle == live.handle)
				.ok_or(AllocError::UnknownAllocation)?
				.export_types,
		)
	} else {
		Ok(live.export_types)
	}
}

/// Observe the export flags retained for this allocation's entire native block.
pub fn external_memory_types(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
) -> Result<vk::ExternalMemoryHandleTypeFlags, AllocError> {
	exports(allocator, allocation)
}

/// Export a new caller-owned file descriptor for the complete backing memory.
/// Allocation offsets still apply. Close the returned descriptor using the OS;
/// neither token nor allocator owns it. No GPU synchronization occurs.
/// # Errors
/// Rejects stale/non-exportable allocations, invalid types, missing enabled
/// extensions/PFNs and native failures. Native failure transfers no descriptor.
/// # Safety
/// Meet native external-memory compatibility, sharing/security and synchronization
/// contracts. The caller closes each successful descriptor exactly once.
#[cfg(unix)]
pub unsafe fn get_memory_fd(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
	handle_type: vk::ExternalMemoryHandleTypeFlagBits,
) -> Result<std::os::fd::RawFd, AllocError> {
	use vk::ExternalMemoryHandleTypeFlagBits as F;
	if handle_type != F::OPAQUE_FD_BIT && handle_type != F::DMA_BUF_BIT_EXT {
		return Err(AllocError::InvalidRequest(
			"file descriptor handle type required",
		));
	}
	validate_export(allocator, allocation, handle_type)?;
	require(
		allocator.device.fp().get_memory_fd_khr.is_some(),
		"vkGetMemoryFdKHR",
	)?;
	let info = vk::MemoryGetFdInfoKHR {
		memory: allocation.handle,
		handle_type,
		..Default::default()
	};
	let mut fd = -1;
	// SAFETY: checked native identity/export admission; output is initialized and caller meets sharing requirements.
	unsafe { allocator.device.get_memory_fd_khr(&info, &mut fd) }?;
	if fd < 0 {
		return Err(
			vk::load::CallError::InvalidOutput("vkGetMemoryFdKHR returned an invalid descriptor").into(),
		);
	}
	Ok(fd)
}

fn validate_export(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
	handle_type: vk::ExternalMemoryHandleTypeFlagBits,
) -> Result<(), AllocError> {
	let bits = handle_type.as_raw();
	if bits == 0 || !bits.is_power_of_two() || exports(allocator, allocation)?.as_raw() & bits == 0 {
		return Err(AllocError::InvalidRequest(
			"handle type was not enabled for this native allocation",
		));
	}
	validate_types(allocator, handle_type)
}

pub(super) unsafe fn required_resource_types(
	chain: *const std::ffi::c_void,
) -> vk::ExternalMemoryHandleTypeFlags {
	let mut types = vk::ExternalMemoryHandleTypeFlags::empty();
	let mut cursor = chain.cast::<vk::BaseInStructure>();
	// Callers first validate the bounded resource chain; never traverse a fresh unchecked pointer.
	while !cursor.is_null() {
		// SAFETY: prior resource-chain validation and unsafe operation contract cover all records.
		let node = unsafe { &*cursor };
		match node.s_type {
			vk::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => {
				// SAFETY: complete matching record from the validated chain.
				types = unsafe { (*cursor.cast::<vk::ExternalMemoryBufferCreateInfo>()).handle_types };
			}
			vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => {
				// SAFETY: complete matching record from the validated chain.
				types = unsafe { (*cursor.cast::<vk::ExternalMemoryImageCreateInfo>()).handle_types };
			}
			_ => {}
		}
		cursor = node.p_next;
	}
	types
}

pub(super) unsafe fn resource_options<'a>(
	allocator: &Allocator<'_>,
	options: &AllocationCreateInfo<'a>,
	chain: *const std::ffi::c_void,
) -> Result<AllocationCreateInfo<'a>, AllocError> {
	// SAFETY: resource validation already established a bounded readable chain.
	let required = unsafe { required_resource_types(chain) };
	if required.as_raw() == 0 {
		return Ok(*options);
	}
	let tail = options.memory_allocate_next.map_or_else(
		|| {
			options
				.pool
				.and_then(|pool| allocator.pools.get(&pool.serial))
				.map_or(Default::default(), |pool| pool.allocation_chain)
		},
		|chain| chain.data,
	);
	// Imported payloads need not be re-exportable; native compatibility is caller-owned.
	if tail.imports {
		return Ok(*options);
	}
	let mut bits = 0;
	for index in 0..allocator.selector.properties.memory_type_count {
		if options.memory_type_bits & (1 << index) == 0
			|| options
				.pool
				.is_some_and(|pool| allocator.pools[&pool.serial].info.memory_type_index != index)
		{
			continue;
		}
		// A chained export cannot be prepended to a configured export on the same
		// type. Exclude that type without rejecting another compatible profile.
		if tail.has_export && allocator.external_types[index as usize].as_raw() != 0 {
			continue;
		}
		let types = allocation_exports(allocator, index, tail)?;
		if types.as_raw() & required.as_raw() == required.as_raw() {
			bits |= 1 << index;
		}
	}
	if bits & options.memory_type_bits == 0 {
		return Err(AllocError::NoCompatibleMemoryType);
	}
	Ok(AllocationCreateInfo {
		memory_type_bits: options.memory_type_bits & bits,
		..*options
	})
}

impl<'device> Allocator<'device> {
	/// Copy an external export profile before any pool/native acquisition.
	/// # Errors
	/// Same validation errors as `mem::set_external_memory_types`.
	/// # Safety
	/// All future resources and native allocations must meet the configured export valid usage.
	pub unsafe fn set_external_memory_types(
		&mut self,
		types: &[vk::ExternalMemoryHandleTypeFlags],
	) -> Result<(), AllocError> {
		// SAFETY: same profile and future-resource obligations.
		unsafe { set_external_memory_types(self, types) }
	}

	pub fn external_memory_types(
		&self,
		allocation: &Allocation,
	) -> Result<vk::ExternalMemoryHandleTypeFlags, AllocError> {
		external_memory_types(self, allocation)
	}

	/// Export one caller-owned descriptor for the complete backing memory.
	/// # Errors
	/// Same checked identity/admission and native failures as `mem::get_memory_fd`.
	/// # Safety
	/// Caller meets sharing/synchronization requirements and closes the returned descriptor.
	#[cfg(unix)]
	pub unsafe fn get_memory_fd(
		&self,
		allocation: &Allocation,
		handle_type: vk::ExternalMemoryHandleTypeFlagBits,
	) -> Result<std::os::fd::RawFd, AllocError> {
		// SAFETY: identical external-memory and caller descriptor-ownership obligations.
		unsafe { get_memory_fd(self, allocation, handle_type) }
	}

	/// Create a custom pool retaining a borrowed chain for future native allocations.
	/// # Errors
	/// Same checked policy/chain/native errors as `mem::create_pool_with_chain`.
	/// # Safety
	/// Retain the complete immutable chain for the device borrow; every block must meet native valid usage.
	pub unsafe fn create_pool_with_chain(
		&mut self,
		info: &PoolCreateInfo,
		chain: AllocationChain<'device>,
	) -> Result<Pool, AllocError> {
		// SAFETY: identical native metadata and retained-chain contract.
		unsafe { create_pool_with_chain(self, info, chain) }
	}
}

#[cfg(all(windows, feature = "platform-win32"))]
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};

/// One root per (memory, type), required by Vulkan's NT-handle export rules.
/// `OwnedHandle` releases only an OS reference; it never frees Vulkan memory,
/// submits or waits. Explicit memory release removes these roots. Returned
/// duplicates belong to the caller/target process and are never retained here.
#[cfg(all(windows, feature = "platform-win32"))]
pub(super) struct CachedHandle {
	pub memory: vk::DeviceMemory,
	kind: vk::ExternalMemoryHandleTypeFlagBits,
	handle: *mut std::ffi::c_void,
	_root: Option<OwnedHandle>,
}

#[cfg(all(windows, feature = "platform-win32"))]
#[link(name = "kernel32")]
unsafe extern "system" {
	fn GetCurrentProcess() -> *mut std::ffi::c_void;
	fn DuplicateHandle(
		source_process: *mut std::ffi::c_void,
		source: *mut std::ffi::c_void,
		target_process: *mut std::ffi::c_void,
		target: *mut *mut std::ffi::c_void,
		access: u32,
		inherit: i32,
		options: u32,
	) -> i32;
}

/// Export one Win32 handle for the entire backing native allocation.
///
/// NT roots are fetched once per native memory/type and cached. Each call
/// duplicates the root into the requested process; close each returned NT handle
/// in that process. KMT handles are borrowed, never closed, and cannot be
/// duplicated into a target process. Native memory destruction invalidates them.
/// # Errors
/// Preserves native and OS duplication failures, and rejects stale allocations,
/// unavailable export types/PFNs and target-process requests for KMT handles.
/// # Safety
/// Meet native external-memory/synchronization/security valid usage. A non-null
/// `target_process` must be a live handle with PROCESS_DUP_HANDLE permission.
/// The caller/target process owns each returned NT duplicate and closes it.
#[cfg(all(windows, feature = "platform-win32"))]
pub unsafe fn get_memory_win32_handle(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	handle_type: vk::ExternalMemoryHandleTypeFlagBits,
	target_process: *mut std::ffi::c_void,
) -> Result<vk::HANDLE, AllocError> {
	use vk::ExternalMemoryHandleTypeFlagBits as F;
	let nt = match handle_type {
		F::OPAQUE_WIN32_BIT | F::D3D11_TEXTURE_BIT | F::D3D12_HEAP_BIT | F::D3D12_RESOURCE_BIT => true,
		F::OPAQUE_WIN32_KMT_BIT | F::D3D11_TEXTURE_KMT_BIT => false,
		_ => return Err(AllocError::InvalidRequest("Win32 handle type required")),
	};
	validate_export(allocator, allocation, handle_type)?;
	if !nt && !target_process.is_null() {
		return Err(AllocError::InvalidRequest(
			"KMT handles cannot be duplicated into a target process",
		));
	}
	require(
		allocator.device.fp().get_memory_win32_handle_khr.is_some(),
		"vkGetMemoryWin32HandleKHR",
	)?;
	let cached = allocator
		.external_handles
		.iter()
		.position(|entry| entry.memory == allocation.handle && entry.kind == handle_type);
	let index = if let Some(index) = cached {
		index
	} else {
		if let Some(shared) = allocator.parallel_state() {
			shared.reserve_external()?;
		}
		allocator
			.external_handles
			.try_reserve(1)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		let info = vk::MemoryGetWin32HandleInfoKHR {
			memory: allocation.handle,
			handle_type,
			..Default::default()
		};
		let mut handle = std::ptr::null_mut();
		// SAFETY: cached export admission/identity and initialized output; caller establishes sharing valid usage.
		unsafe {
			allocator
				.device
				.as_device()
				.get_memory_win32_handle_khr(&info, &mut handle)
		}?;
		if handle.is_null() || handle == (-1_isize) as *mut std::ffi::c_void {
			return Err(
				vk::load::CallError::InvalidOutput("vkGetMemoryWin32HandleKHR returned an invalid handle")
					.into(),
			);
		}
		let root = if nt {
			// SAFETY: successful export returned a new owned NT handle, never exposed or adopted elsewhere.
			Some(unsafe { OwnedHandle::from_raw_handle(handle) })
		} else {
			None
		};
		let index = allocator.external_handles.len();
		allocator.external_handles.push_reserved(CachedHandle {
			memory: allocation.handle,
			kind: handle_type,
			handle,
			_root: root,
		});
		if let Some(shared) = allocator.parallel_state() {
			shared.external_added();
		}
		index
	};
	let entry = &allocator.external_handles[index];
	if !nt {
		return Ok(entry.handle);
	}
	let root = entry
		._root
		.as_ref()
		.expect("NT export root owns its handle");
	// SAFETY: process pseudo-handle is valid in the current process, requires no close.
	let current = unsafe { GetCurrentProcess() };
	let target = if target_process.is_null() {
		current
	} else {
		target_process
	};
	let mut duplicate = std::ptr::null_mut();
	// SAFETY: live owned source, valid target process per caller, initialized output,
	// DUPLICATE_SAME_ACCESS preserves permissions without transferring our root.
	if unsafe {
		DuplicateHandle(
			current,
			root.as_raw_handle(),
			target,
			&mut duplicate,
			0,
			0,
			2,
		)
	} == 0
	{
		let error = std::io::Error::last_os_error();
		return Err(AllocError::Output {
			kind: error.kind(),
			code: error.raw_os_error(),
		});
	}
	Ok(duplicate)
}

#[cfg(all(windows, feature = "platform-win32"))]
impl Allocator<'_> {
	/// Export a caller-owned NT duplicate or a borrowed KMT handle.
	/// # Errors
	/// Same checked/native/OS errors as `mem::get_memory_win32_handle`.
	/// # Safety
	/// Same sharing, target-process permissions and handle-closing obligations.
	pub unsafe fn get_memory_win32_handle(
		&mut self,
		allocation: &Allocation,
		handle_type: vk::ExternalMemoryHandleTypeFlagBits,
		target_process: *mut std::ffi::c_void,
	) -> Result<vk::HANDLE, AllocError> {
		// SAFETY: identical native export and target-process ownership contract.
		unsafe { get_memory_win32_handle(self, allocation, handle_type, target_process) }
	}
}

fn validate_allocation_chain(
	allocator: &Allocator<'_>,
	chain: ChainData,
) -> Result<(), AllocError> {
	if let Some(name) = chain.import_extension
		&& !crate::mem::resource::extension(allocator.device.as_device(), name)
	{
		return Err(AllocError::UnsupportedCapability(
			"memory import extension is not enabled",
		));
	}
	if let Some(flags) = chain.flags {
		if !allocator
			.device
			.as_device()
			.capabilities()
			.is_some_and(|caps| caps.api_version() >= vk::API_VERSION_1_1)
			&& !crate::mem::resource::extension(allocator.device.as_device(), c"VK_KHR_device_group")
		{
			return Err(AllocError::UnsupportedCapability(
				"allocation flags require core 1.1 or enabled device_group",
			));
		}
		if flags.as_raw() & vk::MemoryAllocateFlagBits::DEVICE_ADDRESS_BIT.as_raw() != 0
			&& !allocator.buffer_device_address
		{
			return Err(AllocError::UnsupportedCapability(
				"allocation address flag requires enabled feature",
			));
		}
	}
	Ok(())
}
