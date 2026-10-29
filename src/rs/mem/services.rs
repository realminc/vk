//! Pinned VMA 3.4.0 page allocation/free, batched cache maintenance, host-copy
//! helpers and allocation/pool metadata (LICENSE-VMA-MIT).
//! Adaptation: fallible Rust containers, checked tokens/ranges, explicit mapping
//! holds and cleanup; no implicit synchronization, native Drop or raw-data ownership.
use super::*;

/// One allocation-relative cache range. Each allocation must own a mapping hold.
#[derive(Clone, Copy)]
pub struct CacheRange<'a> {
	pub allocation: &'a Allocation,
	pub offset: u64,
	pub size: u64,
}

/// Allocate identical ordinary requirement pages; failure releases new pages only.
///
/// Host result storage is reserved before native effects. Generations may advance
/// after rollback, and normal pool caching policy may retain empty native blocks.
/// # Errors
/// Preserves allocation failure; rejects impossible result extents/serial overflow first.
/// # Safety
/// Same raw requirements/feature/lifetime contract as `allocate_with_options`.
pub unsafe fn allocate_many(
	allocator: &mut Allocator<'_>,
	requirements: &vk::MemoryRequirements,
	options: &AllocationCreateInfo,
	count: usize,
) -> Result<Vec<Allocation>, AllocError> {
	if count > 1
		&& options
			.memory_allocate_next
			.is_some_and(|chain| chain.data.imports)
	{
		return Err(AllocError::InvalidRequest(
			"one imported handle cannot back multiple allocations",
		));
	}
	allocator.ensure_open()?;
	allocator
		.next_serial
		.checked_add(u64::try_from(count).map_err(|_| AllocError::SerialExhausted)?)
		.ok_or(AllocError::SerialExhausted)?;
	let mut allocations = Vec::new();
	allocations
		.try_reserve_exact(count)
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for _ in 0..count {
		// SAFETY: each page is unpublished, unbound ordinary storage with the same native requirements.
		match unsafe { allocate_with_options(allocator, requirements, options) } {
			Ok(allocation) => allocations.push(allocation),
			Err(error) => {
				for allocation in allocations.iter_mut().rev() {
					// SAFETY: no caller received any token or mapping pointer; no GPU/native resource use exists.
					unsafe { rollback_allocation(allocator, allocation) }.expect("unpublished page rollback");
				}
				return Err(error);
			}
		}
	}
	Ok(allocations)
}

/// Validate an entire release batch before freeing any allocation.
///
/// Successful release invalidates every token. Ordinary native cleanup uses
/// immutable preflighted void PFNs; no host allocation is needed for this batch.
/// # Errors
/// Rejects closed, foreign, stale, duplicate or explicitly mapped tokens before effects.
/// # Safety
/// Every bound resource must already be destroyed and every CPU/GPU use complete.
pub unsafe fn free_many(
	allocator: &mut Allocator<'_>,
	allocations: &mut [Allocation],
) -> Result<(), AllocError> {
	allocator.ensure_open()?;
	for (index, allocation) in allocations.iter().enumerate() {
		if allocator.validate(allocation)?.map_count != 0 {
			return Err(AllocError::AlreadyMapped);
		}
		if allocations[..index]
			.iter()
			.any(|previous| previous.serial == allocation.serial)
		{
			return Err(AllocError::InvalidRequest(
				"duplicate allocation in release batch",
			));
		}
	}
	for allocation in allocations {
		// SAFETY: all identities/holds were checked; caller has completed uses and destroyed resources.
		unsafe { free(allocator, allocation) }?;
	}
	Ok(())
}

fn cache_ranges<'device>(
	allocator: &Allocator<'device>,
	ranges: &[CacheRange<'_>],
) -> Result<HostVec<'device, vk::MappedMemoryRange<'static>>, AllocError> {
	allocator.ensure_open()?;
	let mut native = HostVec::new(allocator.host);
	native
		.try_reserve_exact(ranges.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for range in ranges {
		let live = allocator.validate(range.allocation)?;
		if live.map_count == 0 && !live.persistent {
			return Err(AllocError::NotMapped);
		}
		if let Some(range) = range.allocation.mapped_range(range.offset, range.size)? {
			native.push_reserved(range);
		}
	}
	u32::try_from(native.len()).map_err(|_| AllocError::InvalidRange)?;
	Ok(native)
}

/// Flush checked ranges in one native call; coherent/empty ranges remain checked no-ops.
/// # Errors
/// Invalid identity/map/range or host reservation fails before the native call.
/// # Safety
/// Match Vulkan host/GPU synchronization and host-write visibility requirements.
pub unsafe fn flush_many(
	allocator: &Allocator<'_>,
	ranges: &[CacheRange<'_>],
) -> Result<(), AllocError> {
	let native = cache_ranges(allocator, ranges)?;
	if !native.is_empty() {
		// SAFETY: every range belongs to a currently held mapping and satisfies native atom bounds.
		unsafe { allocator.device.flush_mapped_memory_ranges(&native) }?;
	}
	Ok(())
}

/// Invalidate checked ranges in one native call; never waits for device writes.
/// # Errors
/// Same checked identity/map/range and native errors as `flush_many`.
/// # Safety
/// Device writes must have completed; synchronize every host/GPU access first.
pub unsafe fn invalidate_many(
	allocator: &Allocator<'_>,
	ranges: &[CacheRange<'_>],
) -> Result<(), AllocError> {
	let native = cache_ranges(allocator, ranges)?;
	if !native.is_empty() {
		// SAFETY: initialized native ranges are bounded within held native mappings.
		unsafe { allocator.device.invalidate_mapped_memory_ranges(&native) }?;
	}
	Ok(())
}

fn copy_range(
	allocator: &Allocator<'_>,
	allocation: &Allocation,
	offset: u64,
	length: usize,
	write: bool,
) -> Result<u64, AllocError> {
	allocator.validate(allocation)?;
	let size = u64::try_from(length).map_err(|_| AllocError::InvalidRange)?;
	if offset
		.checked_add(size)
		.is_none_or(|end| end > allocation.size)
	{
		return Err(AllocError::InvalidRange);
	}
	if size != 0 && allocation.mapped_range(offset, size)?.is_some() {
		if write {
			require(
				allocator.device.fp().flush_mapped_memory_ranges.is_some(),
				"vkFlushMappedMemoryRanges",
			)?;
		} else {
			require(
				allocator
					.device
					.fp()
					.invalidate_mapped_memory_ranges
					.is_some(),
				"vkInvalidateMappedMemoryRanges",
			)?;
		}
	}
	Ok(size)
}

/// Copy host bytes to an allocation, flush them and release only this helper's map hold.
///
/// Existing explicit/persistent maps remain held. Zero bytes still validate
/// identity and bounds. Does not submit transfers, wait or infer GPU completion.
/// # Errors
/// Reports checked identity/range/map/cache errors; a cache failure still releases this map hold.
/// # Safety
/// Caller synchronizes all accesses and provides initialized readable source
/// storage. Destination native bytes must be writable; no live Rust reference may
/// be invalidated by this write. Resource layout (especially images) remains caller-owned.
pub unsafe fn copy_to_allocation(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	offset: u64,
	source: &[u8],
) -> Result<(), AllocError> {
	let size = copy_range(allocator, allocation, offset, source.len(), true)?;
	if size == 0 {
		return Ok(());
	}
	let pointer_offset = usize::try_from(offset).map_err(|_| AllocError::InvalidRange)?;
	// SAFETY: caller establishes host access and initialization; checked extent is within native storage.
	let pointer = unsafe { map(allocator, allocation) }?;
	// SAFETY: Mapping validates native host extent; checked copy range and caller's alias/access contract apply.
	unsafe {
		std::ptr::copy(
			source.as_ptr(),
			pointer.cast::<u8>().add(pointer_offset),
			source.len(),
		)
	};
	// SAFETY: this helper's live map covers the checked allocation-relative range.
	let result = unsafe { flush(allocator, allocation, offset, size) };
	// SAFETY: helper no longer uses the pointer; preserve other explicit/persistent holds.
	let cleanup = unsafe { unmap(allocator, allocation) };
	result.and(cleanup)
}

/// Invalidate and copy allocation bytes to initialized writable host storage.
/// # Errors
/// Checked identity/range/map/cache errors; invalidation failure performs no host copy.
/// # Safety
/// Complete device writes and synchronize all host/GPU accesses. Source bytes
/// must be initialized; destination exclusivity and native image layout remain caller obligations.
pub unsafe fn copy_from_allocation(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	offset: u64,
	destination: &mut [u8],
) -> Result<(), AllocError> {
	let size = copy_range(allocator, allocation, offset, destination.len(), false)?;
	if size == 0 {
		return Ok(());
	}
	let pointer_offset = usize::try_from(offset).map_err(|_| AllocError::InvalidRange)?;
	// SAFETY: caller supplies completed writes and synchronized visible storage.
	let pointer = unsafe { map(allocator, allocation) }?;
	// SAFETY: checked allocation-relative range lies within this held mapping.
	let result = unsafe { invalidate(allocator, allocation, offset, size) };
	if result.is_ok() {
		// SAFETY: caller guarantees initialized native bytes and exclusive writable destination.
		unsafe {
			std::ptr::copy(
				pointer.cast::<u8>().add(pointer_offset),
				destination.as_mut_ptr(),
				destination.len(),
			)
		};
	}
	// SAFETY: no helper pointer uses remain; release exactly the acquired hold.
	let cleanup = unsafe { unmap(allocator, allocation) };
	result.and(cleanup)
}

fn name_copy<'callbacks>(
	host: HostMemory<'callbacks>,
	name: Option<&str>,
) -> Result<Option<HostString<'callbacks>>, AllocError> {
	name
		.map(|name| HostString::new_from_str(host, name))
		.transpose()
}

/// Borrow an allocation's diagnostic name; no native lifetime is acquired.
pub fn allocation_name<'a>(
	allocator: &'a Allocator<'_>,
	allocation: &Allocation,
) -> Result<Option<&'a str>, AllocError> {
	Ok(allocator.validate(allocation)?.name.as_deref())
}

/// Replace a copied UTF-8 diagnostic name transactionally; None clears it.
/// # Errors
/// Identity/state or host reservation failure leaves the previous name intact.
pub fn set_allocation_name(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	name: Option<&str>,
) -> Result<(), AllocError> {
	allocator.validate(allocation)?;
	let name = name_copy(allocator.host, name)?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("validated allocation")
		.name = name;
	Ok(())
}

/// Set uninterpreted consumer metadata. It is never dereferenced, freed or owned.
pub fn set_allocation_user_data(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	data: usize,
) -> Result<(), AllocError> {
	allocator.validate(allocation)?;
	allocator
		.live
		.get_mut(&allocation.serial)
		.expect("validated allocation")
		.user_data = data;
	Ok(())
}

/// Borrow a custom pool's copied diagnostic name without native effects.
pub fn pool_name<'a>(
	allocator: &'a Allocator<'_>,
	pool: &Pool,
) -> Result<Option<&'a str>, AllocError> {
	Ok(pool::validate(allocator, pool)?.name.as_deref())
}

/// Replace a pool diagnostic name; metadata allocation failure preserves its old name.
pub fn set_pool_name(
	allocator: &mut Allocator<'_>,
	pool: &Pool,
	name: Option<&str>,
) -> Result<(), AllocError> {
	pool::validate(allocator, pool)?;
	let name = name_copy(allocator.host, name)?;
	allocator
		.pools
		.get_mut(&pool.serial)
		.expect("validated pool")
		.name = name;
	Ok(())
}

/// Detailed local byte/count statistics. Empty populations have None extrema.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DetailedStatistics {
	pub native_count: usize,
	pub native_bytes: u64,
	pub allocation_count: usize,
	pub allocation_bytes: u64,
	pub reserved_bytes: u64,
	pub free_range_count: usize,
	pub free_bytes: u64,
	pub allocation_size_min: Option<u64>,
	pub allocation_size_max: Option<u64>,
	pub free_range_size_min: Option<u64>,
	pub free_range_size_max: Option<u64>,
}

/// Immutable explicit diagnostic snapshot, indexed by native memory type and heap.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryStatistics {
	pub total: DetailedStatistics,
	pub memory_types: [DetailedStatistics; 32],
	pub heaps: [DetailedStatistics; 16],
}

impl DetailedStatistics {
	fn include(&mut self, other: Self) {
		self.native_count += other.native_count;
		self.native_bytes += other.native_bytes;
		self.allocation_count += other.allocation_count;
		self.allocation_bytes += other.allocation_bytes;
		self.reserved_bytes += other.reserved_bytes;
		self.free_range_count += other.free_range_count;
		self.free_bytes += other.free_bytes;
		for (target, value, minimum) in [
			(
				&mut self.allocation_size_min,
				other.allocation_size_min,
				true,
			),
			(
				&mut self.allocation_size_max,
				other.allocation_size_max,
				false,
			),
			(
				&mut self.free_range_size_min,
				other.free_range_size_min,
				true,
			),
			(
				&mut self.free_range_size_max,
				other.free_range_size_max,
				false,
			),
		] {
			if let Some(value) = value {
				*target = Some(target.map_or(value, |old| {
					if minimum {
						old.min(value)
					} else {
						old.max(value)
					}
				}));
			}
		}
	}
}

/// Walk actual native blocks and live regions to build detailed local statistics.
///
/// Linear free holes are physical space, not a promise of immediate reuse.
/// Native block bytes, requested bytes and reserved atom padding stay distinct.
/// No driver query, map, native allocation or GPU synchronization occurs.
/// # Errors
/// Closed allocators are rejected.
pub fn detailed_statistics(allocator: &Allocator<'_>) -> Result<MemoryStatistics, AllocError> {
	collect_statistics(allocator, None)
}

/// Walk actual blocks/live regions belonging to one checked custom pool.
pub fn detailed_pool_statistics(
	allocator: &Allocator<'_>,
	pool: &Pool,
) -> Result<DetailedStatistics, AllocError> {
	pool::validate(allocator, pool)?;
	Ok(collect_statistics(allocator, Some(pool.serial))?.total)
}

fn collect_statistics(
	allocator: &Allocator<'_>,
	pool_filter: Option<u64>,
) -> Result<MemoryStatistics, AllocError> {
	allocator.ensure_open()?;
	let mut result = MemoryStatistics::default();
	for block in &allocator.blocks {
		if pool_filter.is_some() && block.pool != pool_filter {
			continue;
		}
		let stats = &mut result.memory_types[block.memory_type_index as usize];
		stats.native_count += 1;
		stats.native_bytes += block.size;
		block.metadata.for_each_free(|_, size| {
			stats.free_range_count += 1;
			stats.free_bytes += size;
			stats.free_range_size_min = Some(stats.free_range_size_min.map_or(size, |old| old.min(size)));
			stats.free_range_size_max = Some(stats.free_range_size_max.map_or(size, |old| old.max(size)));
		});
	}
	for live in allocator.live.values() {
		if pool_filter.is_some() && live.pool != pool_filter {
			continue;
		}
		let stats = &mut result.memory_types[live.memory_type_index as usize];
		if live.placement.is_none() {
			stats.native_count += 1;
			stats.native_bytes += live.size;
		}
		stats.allocation_count += 1;
		stats.allocation_bytes += live.size;
		stats.reserved_bytes += live.reserved_size;
		stats.allocation_size_min = Some(
			stats
				.allocation_size_min
				.map_or(live.size, |old| old.min(live.size)),
		);
		stats.allocation_size_max = Some(
			stats
				.allocation_size_max
				.map_or(live.size, |old| old.max(live.size)),
		);
	}
	for index in 0..allocator.selector.properties.memory_type_count as usize {
		let stats = result.memory_types[index];
		result.total.include(stats);
		result.heaps[allocator.selector.properties.memory_types[index].heap_index as usize]
			.include(stats);
	}
	Ok(result)
}

/// Write ordered local allocator diagnostics as `vk.memory.statistics.v1` JSON.
///
/// Detailed mode includes live allocation identifiers, offsets, names, user data
/// and per-block free spans. No driver budget query, map or synchronization is hidden.
/// Output can be partial on writer failure; names are copied/escaped as UTF-8 JSON.
/// # Errors
/// Returns closed/host-reservation errors before output, or writer kind/OS code on failure.
pub fn write_statistics_json(
	allocator: &Allocator<'_>,
	output: &mut impl std::io::Write,
	detailed: bool,
) -> Result<(), AllocError> {
	let stats = detailed_statistics(allocator)?;
	let mut allocations = HostVec::new(allocator.host);
	if detailed {
		allocations
			.try_reserve_exact(allocator.live.len())
			.map_err(|_| AllocError::HostAllocationFailed)?;
		for entry in allocator.live.iter() {
			allocations.push_reserved(entry);
		}
		allocations.sort_unstable_by_key(|(serial, _)| **serial);
	}
	let mut pools = HostVec::new(allocator.host);
	pools
		.try_reserve_exact(allocator.pools.len())
		.map_err(|_| AllocError::HostAllocationFailed)?;
	for entry in allocator.pools.iter() {
		pools.push_reserved(entry);
	}
	pools.sort_unstable_by_key(|(serial, _)| **serial);
	fn quote(out: &mut impl std::io::Write, text: &str) -> std::io::Result<()> {
		out.write_all(b"\"")?;
		for value in text.chars() {
			match value {
				'"' => out.write_all(b"\\\"")?,
				'\\' => out.write_all(b"\\\\")?,
				value if value <= '\u{1f}' => write!(out, "\\u{:04x}", u32::from(value))?,
				value => {
					let mut bytes = [0; 4];
					out.write_all(value.encode_utf8(&mut bytes).as_bytes())?;
				}
			}
		}
		out.write_all(b"\"")
	}
	fn number(out: &mut impl std::io::Write, value: Option<u64>) -> std::io::Result<()> {
		if let Some(value) = value {
			write!(out, "{value}")
		} else {
			out.write_all(b"null")
		}
	}
	fn statistics(out: &mut impl std::io::Write, stats: &DetailedStatistics) -> std::io::Result<()> {
		write!(
			out,
			"{{\"native_count\":{},\"native_bytes\":{},\"allocation_count\":{},\"allocation_bytes\":{},\"reserved_bytes\":{},\"free_range_count\":{},\"free_bytes\":{},\"allocation_size_min\":",
			stats.native_count,
			stats.native_bytes,
			stats.allocation_count,
			stats.allocation_bytes,
			stats.reserved_bytes,
			stats.free_range_count,
			stats.free_bytes
		)?;
		number(out, stats.allocation_size_min)?;
		out.write_all(b",\"allocation_size_max\":")?;
		number(out, stats.allocation_size_max)?;
		out.write_all(b",\"free_range_size_min\":")?;
		number(out, stats.free_range_size_min)?;
		out.write_all(b",\"free_range_size_max\":")?;
		number(out, stats.free_range_size_max)?;
		out.write_all(b"}")
	}
	let written = (|| -> std::io::Result<()> {
		output.write_all(b"{\"schema\":\"vk.memory.statistics.v1\",\"total\":")?;
		statistics(output, &stats.total)?;
		output.write_all(b",\"memory_types\":[")?;
		for index in 0..allocator.selector.properties.memory_type_count as usize {
			if index != 0 {
				output.write_all(b",")?;
			}
			statistics(output, &stats.memory_types[index])?;
		}
		output.write_all(b"],\"heaps\":[")?;
		for index in 0..allocator.selector.properties.memory_heap_count as usize {
			if index != 0 {
				output.write_all(b",")?;
			}
			statistics(output, &stats.heaps[index])?;
		}
		output.write_all(b"],\"pools\":[")?;
		for (index, (serial, pool)) in pools.iter().enumerate() {
			if index != 0 {
				output.write_all(b",")?;
			}
			write!(
				output,
				"{{\"id\":{serial},\"memory_type\":{},\"name\":",
				pool.info.memory_type_index
			)?;
			if let Some(name) = &pool.name {
				quote(output, name)?;
			} else {
				output.write_all(b"null")?;
			}
			output.write_all(b"}")?;
		}
		output.write_all(b"]")?;
		if detailed {
			output.write_all(b",\"allocations\":[")?;
			for (index, (serial, live)) in allocations.iter().enumerate() {
				if index != 0 {
					output.write_all(b",")?;
				}
				write!(
					output,
					"{{\"id\":{serial},\"offset\":{},\"size\":{},\"reserved_size\":{},\"memory_type\":{},\"user_data\":{},\"map_count\":{},\"persistent\":{},\"pinned\":{},\"name\":",
					live.offset,
					live.size,
					live.reserved_size,
					live.memory_type_index,
					live.user_data,
					live.map_count,
					live.persistent,
					live.pinned
				)?;
				if let Some(name) = &live.name {
					quote(output, name)?;
				} else {
					output.write_all(b"null")?;
				}
				output.write_all(b"}")?;
			}
			output.write_all(b"],\"blocks\":[")?;
			for (index, block) in allocator.blocks.iter().enumerate() {
				if index != 0 {
					output.write_all(b",")?;
				}
				write!(
					output,
					"{{\"memory_type\":{},\"size\":{},\"free_ranges\":[",
					block.memory_type_index, block.size
				)?;
				let mut first = true;
				let mut error = None;
				block.metadata.for_each_free(|offset, size| {
					if error.is_some() {
						return;
					}
					let result = (|| {
						if !first {
							output.write_all(b",")?;
						}
						first = false;
						write!(output, "[{offset},{size}]")
					})();
					if let Err(failure) = result {
						error = Some(failure);
					}
				});
				if let Some(error) = error {
					return Err(error);
				}
				output.write_all(b"]}")?;
			}
			output.write_all(b"]")?;
		}
		output.write_all(b"}\n")
	})();
	written.map_err(|error| AllocError::Output {
		kind: error.kind(),
		code: error.raw_os_error(),
	})
}

impl Allocator<'_> {
	/// Calculate detailed checked custom-pool native/live/free-span diagnostics.
	pub fn detailed_pool_statistics(&self, pool: &Pool) -> Result<DetailedStatistics, AllocError> {
		detailed_pool_statistics(self, pool)
	}

	/// Stream explicit local JSON diagnostics; detailed mode includes allocations/free spans.
	pub fn write_statistics_json(
		&self,
		output: &mut impl std::io::Write,
		detailed: bool,
	) -> Result<(), AllocError> {
		write_statistics_json(self, output, detailed)
	}

	/// Calculate explicit per-type/per-heap byte, count, free-span and extent diagnostics.
	pub fn detailed_statistics(&self) -> Result<MemoryStatistics, AllocError> {
		detailed_statistics(self)
	}

	/// Allocate a transactional batch of ordinary raw requirement pages.
	/// # Safety
	/// Same requirements/feature/release contract as `mem::allocate_many`.
	pub unsafe fn allocate_many(
		&mut self,
		requirements: &vk::MemoryRequirements,
		options: &AllocationCreateInfo,
		count: usize,
	) -> Result<Vec<Allocation>, AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { allocate_many(self, requirements, options, count) }
	}

	/// Release a fully prevalidated batch after explicit native resource destruction.
	/// # Safety
	/// Same completion/ownership contract as `mem::free_many`.
	pub unsafe fn free_many(&mut self, allocations: &mut [Allocation]) -> Result<(), AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { free_many(self, allocations) }
	}

	/// Flush a batch of allocation-relative mapped ranges in one native call.
	/// # Safety
	/// Same host/GPU visibility contract as `mem::flush_many`.
	pub unsafe fn flush_many(&self, ranges: &[CacheRange<'_>]) -> Result<(), AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { flush_many(self, ranges) }
	}

	/// Invalidate a batch after completion of device writes.
	/// # Safety
	/// Same host/GPU visibility contract as `mem::invalidate_many`.
	pub unsafe fn invalidate_many(&self, ranges: &[CacheRange<'_>]) -> Result<(), AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { invalidate_many(self, ranges) }
	}

	/// Copy host bytes, flush and balance the helper's explicit map hold.
	/// # Safety
	/// Same pointer/access/layout contract as `mem::copy_to_allocation`.
	pub unsafe fn copy_to_allocation(
		&mut self,
		allocation: &Allocation,
		offset: u64,
		source: &[u8],
	) -> Result<(), AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { copy_to_allocation(self, allocation, offset, source) }
	}

	/// Invalidate and copy bytes to exclusive writable host storage.
	/// # Safety
	/// Same completion/initialization contract as `mem::copy_from_allocation`.
	pub unsafe fn copy_from_allocation(
		&mut self,
		allocation: &Allocation,
		offset: u64,
		destination: &mut [u8],
	) -> Result<(), AllocError> {
		// SAFETY: same caller guarantees as the canonical operation.
		unsafe { copy_from_allocation(self, allocation, offset, destination) }
	}

	/// Borrow an allocation's checked diagnostic name.
	pub fn allocation_name<'a>(
		&'a self,
		allocation: &Allocation,
	) -> Result<Option<&'a str>, AllocError> {
		allocation_name(self, allocation)
	}

	/// Replace a copied name; None clears it.
	pub fn set_allocation_name(
		&mut self,
		allocation: &Allocation,
		name: Option<&str>,
	) -> Result<(), AllocError> {
		set_allocation_name(self, allocation, name)
	}

	/// Set opaque consumer metadata, with no ownership or dereference.
	pub fn set_allocation_user_data(
		&mut self,
		allocation: &Allocation,
		data: usize,
	) -> Result<(), AllocError> {
		set_allocation_user_data(self, allocation, data)
	}

	/// Borrow a checked pool's diagnostic name.
	pub fn pool_name<'a>(&'a self, pool: &Pool) -> Result<Option<&'a str>, AllocError> {
		pool_name(self, pool)
	}

	/// Replace a copied pool name; None clears it.
	pub fn set_pool_name(&mut self, pool: &Pool, name: Option<&str>) -> Result<(), AllocError> {
		set_pool_name(self, pool, name)
	}
}

/// Create an ordinary alias buffer at a checked allocation-local offset.
///
/// Backing storage must have `can_alias`; no new memory or allocation token is
/// created. Aliases are caller-owned: destroy every alias before freeing storage.
/// Original resource association and binding state remain unchanged.
/// # Errors
/// Rejects non-aliasable/foreign/stale storage, invalid requirements/offset/type,
/// required native dedication, unavailable PFNs or native creation/binding failure.
/// # Safety
/// Follow Vulkan aliasing, barrier, native resource/feature and callback rules.
/// Do not access overlapping aliases concurrently. Retain all native lifetimes.
/// Use `allocation_callbacks()` for matching explicit native alias destruction.
pub unsafe fn create_aliasing_buffer(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	local_offset: u64,
	info: &vk::BufferCreateInfo,
) -> Result<vk::Buffer, AllocError> {
	if !allocator.validate(allocation)?.can_alias {
		return Err(AllocError::UnsupportedCapability(
			"allocation was not created for aliasing",
		));
	}
	// SAFETY: caller supplies valid ordinary native creation records/chains.
	unsafe { request::validate_buffer(allocator, info) }?;
	// SAFETY: the preceding validation established bounded readable resource chains.
	let required_exports = unsafe { external::required_resource_types(info.p_next) };
	if !allocator.validate(allocation)?.imported
		&& external::exports(allocator, allocation)?.as_raw() & required_exports.as_raw()
			!= required_exports.as_raw()
	{
		return Err(AllocError::InvalidRequest(
			"alias resource export types do not match backing memory",
		));
	}
	require(
		allocator.device.fp().create_buffer.is_some(),
		"vkCreateBuffer",
	)?;
	require(
		allocator.device.fp().destroy_buffer.is_some(),
		"vkDestroyBuffer",
	)?;
	// SAFETY: validated ordinary create semantics and retained callback lifetime; inverse preflighted.
	let buffer = unsafe { allocator.device.create_buffer(info, allocator.callbacks) }?;
	if buffer.is_null() || binding::associated(allocator, Resource::Buffer(buffer)) {
		return Err(CallError::InvalidOutput("alias buffer is null or already live").into());
	}
	// SAFETY: new unbound caller-owned alias and explicitly conservative aliasable storage.
	if let Err(error) = unsafe {
		binding::bind_alias(
			allocator,
			Resource::Buffer(buffer),
			allocation,
			local_offset,
		)
	} {
		// SAFETY: only the new unpublished alias exists; no GPU use or memory ownership is affected.
		unsafe { allocator.device.destroy_buffer(buffer, allocator.callbacks) }
			.expect("preflighted alias cleanup");
		return Err(error);
	}
	Ok(buffer)
}

/// Create a caller-owned ordinary alias image without allocating new native memory.
/// # Errors
/// Same identity/requirements/binding errors as `create_aliasing_buffer`;
/// requires explicit ALIAS_BIT image creation semantics.
/// # Safety
/// Native image alias/layout/barrier and enabled-feature valid usage apply.
/// Destroy all aliases with matching callbacks before releasing backing storage.
pub unsafe fn create_aliasing_image(
	allocator: &mut Allocator<'_>,
	allocation: &Allocation,
	local_offset: u64,
	info: &vk::ImageCreateInfo,
) -> Result<vk::Image, AllocError> {
	if !allocator.validate(allocation)?.can_alias {
		return Err(AllocError::UnsupportedCapability(
			"allocation was not created for aliasing",
		));
	}
	if info.flags.as_raw() & vk::ImageCreateFlagBits::ALIAS_BIT.as_raw() == 0 {
		return Err(AllocError::InvalidRequest("alias images require ALIAS_BIT"));
	}
	// SAFETY: caller supplies valid ordinary alias image records/chains.
	unsafe { request::validate_image(allocator, info) }?;
	// SAFETY: the preceding validation established bounded readable resource chains.
	let required_exports = unsafe { external::required_resource_types(info.p_next) };
	if !allocator.validate(allocation)?.imported
		&& external::exports(allocator, allocation)?.as_raw() & required_exports.as_raw()
			!= required_exports.as_raw()
	{
		return Err(AllocError::InvalidRequest(
			"alias resource export types do not match backing memory",
		));
	}
	require(
		allocator.device.fp().create_image.is_some(),
		"vkCreateImage",
	)?;
	require(
		allocator.device.fp().destroy_image.is_some(),
		"vkDestroyImage",
	)?;
	// SAFETY: validated ordinary image alias and retained callbacks, inverse preflighted.
	let image = unsafe { allocator.device.create_image(info, allocator.callbacks) }?;
	if image.is_null() || binding::associated(allocator, Resource::Image(image)) {
		return Err(CallError::InvalidOutput("alias image is null or already live").into());
	}
	// SAFETY: same device, conservative aliasable storage; no new native memory ownership.
	if let Err(error) =
		unsafe { binding::bind_alias(allocator, Resource::Image(image), allocation, local_offset) }
	{
		// SAFETY: new unpublished alias only; inverse and callback lifetime preflighted.
		unsafe { allocator.device.destroy_image(image, allocator.callbacks) }
			.expect("preflighted alias cleanup");
		return Err(error);
	}
	Ok(image)
}

impl Allocator<'_> {
	/// Borrow this allocator's retained native callbacks for matching explicit resource cleanup.
	pub fn allocation_callbacks(&self) -> Option<&vk::AllocationCallbacks> {
		self.callbacks
	}

	/// Create and bind a caller-owned buffer alias without allocating memory.
	/// # Safety
	/// Same alias/barrier/feature/lifetime contract as `mem::create_aliasing_buffer`.
	pub unsafe fn create_aliasing_buffer(
		&mut self,
		allocation: &Allocation,
		local_offset: u64,
		info: &vk::BufferCreateInfo,
	) -> Result<vk::Buffer, AllocError> {
		// SAFETY: caller provides the canonical alias and resource valid-usage guarantees.
		unsafe { create_aliasing_buffer(self, allocation, local_offset, info) }
	}

	/// Create and bind a caller-owned image alias without allocating memory.
	/// # Safety
	/// Same native alias/layout/barrier contract as `mem::create_aliasing_image`.
	pub unsafe fn create_aliasing_image(
		&mut self,
		allocation: &Allocation,
		local_offset: u64,
		info: &vk::ImageCreateInfo,
	) -> Result<vk::Image, AllocError> {
		// SAFETY: caller provides the canonical image alias valid-usage guarantees.
		unsafe { create_aliasing_image(self, allocation, local_offset, info) }
	}
}

/// Observe captured native identities without inferring manually omitted parent metadata.
pub fn allocator_info(
	allocator: &Allocator<'_>,
) -> Result<super::super::AllocatorInfo, AllocError> {
	allocator.ensure_open()?;
	Ok(super::super::AllocatorInfo {
		device: allocator.device.handle(),
		instance: allocator.instance_handle,
		physical_device: allocator.physical_device,
		api_version: allocator
			.device
			.capabilities()
			.map(|caps| caps.api_version()),
	})
}

/// Borrow validated native memory metadata used by this allocator's selection policy.
pub fn memory_properties<'a>(
	allocator: &'a Allocator<'_>,
) -> Result<&'a vk::PhysicalDeviceMemoryProperties, AllocError> {
	allocator.ensure_open()?;
	Ok(&allocator.selector.properties)
}

/// Borrow queried physical-device properties, or None after manual metadata construction.
pub fn physical_device_properties<'a>(
	allocator: &'a Allocator<'_>,
) -> Result<Option<&'a vk::PhysicalDeviceProperties>, AllocError> {
	allocator.ensure_open()?;
	Ok(allocator.physical_properties.as_ref())
}

/// Observe one checked native memory type's actual property flags.
pub fn memory_type_properties(
	allocator: &Allocator<'_>,
	index: u32,
) -> Result<vk::MemoryPropertyFlags, AllocError> {
	allocator.ensure_open()?;
	if index >= allocator.selector.properties.memory_type_count {
		return Err(AllocError::InvalidRequest("absent memory type"));
	}
	Ok(allocator.selector.properties.memory_types[index as usize].property_flags)
}

impl<'device> Allocator<'device> {
	/// Register borrowed informative callbacks before acquiring native memory/pools.
	/// # Errors
	/// Rejects closed allocators or any existing native allocation/pool domain.
	/// # Safety
	/// User data must remain live for the callback registration. Functions must not
	/// unwind, reenter this allocator or violate their host thread/synchronization contract.
	pub unsafe fn set_device_memory_callbacks(
		&mut self,
		callbacks: Option<&'device super::super::DeviceMemoryCallbacks>,
	) -> Result<(), AllocError> {
		self.ensure_open()?;
		self.ensure_global()?;
		if self.native_count != 0 || !self.live.is_empty() {
			return Err(AllocError::LiveAllocations);
		}
		if !self.pools.is_empty() {
			return Err(AllocError::LivePools);
		}
		self.device_memory_callbacks = callbacks;
		Ok(())
	}

	/// Query retained native identity metadata, with explicit None for unknown manual parents.
	pub fn info(&self) -> Result<super::super::AllocatorInfo, AllocError> {
		allocator_info(self)
	}

	/// Borrow this allocator's validated physical memory properties.
	pub fn memory_properties(&self) -> Result<&vk::PhysicalDeviceMemoryProperties, AllocError> {
		memory_properties(self)
	}

	/// Borrow queried physical-device properties if the construction route captured them.
	pub fn physical_device_properties(
		&self,
	) -> Result<Option<&vk::PhysicalDeviceProperties>, AllocError> {
		physical_device_properties(self)
	}

	/// Query one validated native memory type's property flags.
	pub fn memory_type_properties(&self, index: u32) -> Result<vk::MemoryPropertyFlags, AllocError> {
		memory_type_properties(self, index)
	}
}
