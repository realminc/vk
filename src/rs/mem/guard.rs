//! Opt-in VMA 3.4.0 debug fill and corruption guards (LICENSE-VMA-MIT).
//! Donor: `IsCorruptionDetectionEnabled`, `WriteMagicValueAfterAllocation`,
//! `ValidateMagicValueAfterAllocation`, `CheckCorruption` and `FillAllocation`.
//! Adaptation: explicit settings/errors, balanced map holds, no assertion abort
//! on corruption and no hidden GPU completion. Only coherent pooled storage has guards.
use super::*;
use crate::mem::DebugMemorySettings;

const MAGIC: u32 = 0x7F84_E666;

pub(super) fn eligible(flags: vk::MemoryPropertyFlags) -> bool {
	let required =
		vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	flags.as_raw() & required.as_raw() == required.as_raw()
}

/// Configure debug instrumentation before native allocations or pools exist.
/// # Errors
/// Rejects live domains, non-four-byte guard margins or missing required map PFNs.
pub fn set_debug_memory_settings(
	allocator: &mut Allocator<'_>,
	settings: DebugMemorySettings,
) -> Result<(), AllocError> {
	allocator.ensure_global()?;
	allocator.ensure_open()?;
	if allocator.native_count != 0 || !allocator.live.is_empty() {
		return Err(AllocError::LiveAllocations);
	}
	if !allocator.pools.is_empty() {
		return Err(AllocError::LivePools);
	}
	if !settings.margin.is_multiple_of(4) {
		return Err(AllocError::InvalidRequest(
			"debug guard margin must be a multiple of four",
		));
	}
	if settings.margin != 0 || settings.initialize_allocations || settings.poison_frees {
		require(allocator.device.fp().map_memory.is_some(), "vkMapMemory")?;
		require(
			allocator.device.fp().unmap_memory.is_some(),
			"vkUnmapMemory",
		)?;
		if settings.initialize_allocations || settings.poison_frees {
			require(
				allocator.device.fp().flush_mapped_memory_ranges.is_some(),
				"vkFlushMappedMemoryRanges",
			)?;
		}
	}
	allocator.debug_settings = settings;
	Ok(())
}

unsafe fn fill(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	byte: u8,
) -> Result<(), AllocError> {
	let length = usize::try_from(allocation.size).map_err(|_| AllocError::InvalidRange)?;
	// SAFETY: caller supplies synchronized host-visible bytes and no conflicting Rust reference.
	let pointer = unsafe { map(allocator, allocation) }?;
	// SAFETY: map validates complete native host extent and requested bytes are within the live allocation.
	unsafe { pointer.cast::<u8>().write_bytes(byte, length) };
	// SAFETY: the checked hold covers every initialized allocation byte.
	let flushed = unsafe { flush(allocator, allocation, 0, vk::WHOLE_SIZE) };
	// SAFETY: no helper pointer uses remain; preserve persistent and other explicit holds.
	let cleanup = unsafe { unmap(allocator, allocation) };
	flushed.and(cleanup)
}

pub(super) unsafe fn initialize(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	let settings = allocator.debug_settings;
	if settings.margin == 0 && !settings.initialize_allocations && !settings.poison_frees {
		return Ok(());
	}
	let live = allocator.validate(allocation)?;
	if live.imported {
		return Ok(());
	}
	let guard_size = live.guard_size;
	let guard_offset = usize::try_from(live.guard_offset).map_err(|_| AllocError::InvalidRange)?;
	if allocator.debug_settings.initialize_allocations && allocation.is_host_visible() {
		// SAFETY: new unpublished unbound storage; no consumer or resource can access it yet.
		unsafe { fill(allocator, allocation, 0xDC) }?;
	}
	if guard_size != 0 {
		let length = usize::try_from(guard_size).map_err(|_| AllocError::InvalidRange)?;
		// SAFETY: guard reservation was checked in native placement; new storage has no conflicting uses.
		let pointer = unsafe { map(allocator, allocation) }?;
		for offset in (0..length).step_by(4) {
			// SAFETY: every four-byte word fits the separately reserved coherent guard tail.
			unsafe {
				pointer
					.cast::<u8>()
					.add(guard_offset + offset)
					.cast::<u32>()
					.write_unaligned(MAGIC)
			};
		}
		// SAFETY: coherent guard writes need no cache flush; helper pointer uses are complete.
		unsafe { unmap(allocator, allocation) }?;
	}
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("live initialized allocation")
		.debug_ready = true;
	Ok(())
}

pub(super) unsafe fn check_one(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	let live = allocator.validate(allocation)?;
	if !live.debug_ready || live.guard_size == 0 {
		return Ok(());
	}
	let guard_offset = usize::try_from(live.guard_offset).map_err(|_| AllocError::InvalidRange)?;
	let length = usize::try_from(live.guard_size).map_err(|_| AllocError::InvalidRange)?;
	// SAFETY: caller establishes completion and coherent host visibility; guard bytes were initialized before publication.
	let pointer = unsafe { map(allocator, allocation) }?;
	let mut failure = None;
	for offset in (0..length).step_by(4) {
		// SAFETY: initialized four-byte words remain inside the live reserved native guard tail.
		let value = unsafe {
			pointer
				.cast::<u8>()
				.add(guard_offset + offset)
				.cast::<u32>()
				.read_unaligned()
		};
		if value != MAGIC {
			failure = Some(AllocError::CorruptionDetected {
				allocation: allocation.serial,
				offset: u64::try_from(guard_offset + offset).expect("mapped native host extent"),
			});
			break;
		}
	}
	// SAFETY: every helper read is complete; release only this helper's explicit hold.
	let cleanup = unsafe { unmap(allocator, allocation) };
	if let Some(error) = failure {
		Err(error)
	} else {
		cleanup
	}
}

pub(super) unsafe fn before_free(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
) -> Result<(), AllocError> {
	if allocator.debug_settings.margin == 0 && !allocator.debug_settings.poison_frees {
		return Ok(());
	}
	if !allocator.validate(allocation)?.debug_ready {
		return Ok(());
	}
	// SAFETY: the release contract ends all resource/host/GPU uses before debug reads/writes.
	unsafe { check_one(allocator, allocation) }?;
	if allocator.debug_settings.poison_frees && allocation.is_host_visible() {
		// SAFETY: caller ended all accesses and destroyed resources; no live Rust borrow may conflict.
		unsafe { fill(allocator, allocation, 0xEF) }?;
	}
	Ok(())
}

unsafe fn check(
	allocator: &mut Allocator<'_>,
	bits: u32,
	pool: Option<u64>,
) -> Result<(), AllocError> {
	allocator.ensure_open()?;
	if allocator.debug_settings.margin == 0
		|| !(0..allocator.selector.properties.memory_type_count)
			.any(|index| bits & (1 << index) != 0 && eligible(memory_flags(allocator, index)))
	{
		return Err(AllocError::UnsupportedCapability(
			"debug corruption detection is not enabled for these memory types",
		));
	}
	let mut selected = HostVec::new(allocator.host);
	selected
		.try_reserve_exact(allocator.live.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for (&serial, live) in &allocator.live {
		if bits & (1 << live.memory_type_index) != 0
			&& live.guard_size != 0
			&& (pool.is_none() || live.pool == pool)
		{
			selected.push_reserved(serial);
		}
	}
	selected.sort_unstable();
	for &serial in &selected {
		let allocation = allocator.token(serial)?;
		// SAFETY: caller supplies completion/synchronization for every selected live guard.
		unsafe { check_one(allocator, &allocation) }?;
	}
	Ok(())
}

/// Check enabled coherent pooled guard tails without waiting for GPU work.
/// # Errors
/// Returns unsupported debug configuration, identity/map or corruption diagnostics.
/// A corrupt region remains allocated; no memory/resource is destroyed on failure.
/// # Safety
/// Complete and synchronize every selected region's GPU/host writes before checking.
pub unsafe fn check_corruption(
	allocator: &mut Allocator<'_>,
	memory_type_bits: u32,
) -> Result<(), AllocError> {
	// SAFETY: caller supplies the all-selected-region completion/access contract.
	unsafe { check(allocator, memory_type_bits, None) }
}

/// Check coherent guard tails belonging to one checked custom pool.
/// # Safety
/// Same completion/coherent-access requirements as `check_corruption`.
pub unsafe fn check_pool_corruption(
	allocator: &mut Allocator<'_>,
	pool: &Pool,
) -> Result<(), AllocError> {
	let index = pool::validate(allocator, pool)?.info.memory_type_index;
	// SAFETY: caller supplies the selected pool's completion/access contract.
	unsafe { check(allocator, 1 << index, Some(pool.serial)) }
}

impl Allocator<'_> {
	/// Configure opt-in guard/fill instrumentation before acquiring memory/pools.
	pub fn set_debug_memory_settings(
		&mut self,
		settings: DebugMemorySettings,
	) -> Result<(), AllocError> {
		set_debug_memory_settings(self, settings)
	}

	/// Explicitly inspect enabled coherent pooled debug guards after completion.
	/// # Safety
	/// Same all-selected-region access contract as `mem::check_corruption`.
	pub unsafe fn check_corruption(&mut self, bits: u32) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical completion/visibility contract.
		unsafe { check_corruption(self, bits) }
	}

	/// Inspect one custom pool's enabled coherent debug guards.
	/// # Safety
	/// Same pool completion/access contract as `mem::check_pool_corruption`.
	pub unsafe fn check_pool_corruption(&mut self, pool: &Pool) -> Result<(), AllocError> {
		// SAFETY: caller supplies the canonical completion/visibility contract.
		unsafe { check_pool_corruption(self, pool) }
	}
}
