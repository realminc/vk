//! Host-only region placement over an integer byte range.
//!
//! No Vulkan memory, buffers, mapping, synchronization or implicit retirement is
//! provided. Consumers own the validity/completion of any resource using these
//! offsets. Tokens are explicit: dropping one does not release its region.
//!
//! ```
//! use vk::mem::{PlacementStrategy, VirtualBlock};
//! let mut block = VirtualBlock::new(4096)?;
//! let mut region = block.allocate(64, 16, PlacementStrategy::Default)?;
//! assert_eq!(block.allocation_info(&region)?.offset, 0);
//! block.free(&mut region)?;
//! # Ok::<(), vk::AllocError>(())
//! ```

use super::{
	AllocError, AllocationAlgorithm,
	host::{HostMemory, HostVec, Identity},
	metadata::{Metadata, Placement},
};
use crate as vk;

/// VMA virtual-TLSF search order; these policies are not latency guarantees.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlacementStrategy {
	#[default]
	Default,
	MinMemory,
	MinTime,
	MinOffset,
}

/// Generation-checked identity in exactly one virtual block; no implicit free.
pub struct VirtualAllocation {
	owner: Identity,
	placement: Option<Placement>,
}

/// Placement snapshot; valid only while the matching allocation remains live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualAllocationInfo {
	/// Uninterpreted consumer metadata; no ownership or dereference is inferred.
	pub user_data: usize,
	pub offset: u64,
	pub size: u64,
}

/// Host metadata statistics. Alignment holes remain free, not reserved to a token.
/// Linear free spans can include holes that placement cannot immediately reuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualBlockStats {
	pub size: u64,
	pub allocation_count: usize,
	pub allocated_bytes: u64,
	pub free_bytes: u64,
	pub free_region_count: usize,
	pub largest_free_region: u64,
}

/// Virtual TLSF or linear allocator; owns host metadata and no native objects.
///
/// Mutations require exclusive access. Drop releases host metadata only.
/// Callback construction borrows its allocation/free record through Drop; raw
/// callback user data remains the unsafe constructor caller's responsibility.
/// This is not a Vulkan memory block or a pool with GPU lifetime tracking.
pub struct VirtualBlock<'callbacks> {
	owner: Identity,
	metadata: Metadata<'callbacks>,
}

/// Explicit virtual placement diagnostics; physical holes may not be immediately reusable by linear placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualBlockDetails {
	pub statistics: VirtualBlockStats,
	pub allocation_size_min: Option<u64>,
	pub allocation_size_max: Option<u64>,
	pub free_range_size_min: Option<u64>,
	pub free_range_size_max: Option<u64>,
}

/// Set checked per-region opaque metadata without native effects or ownership transfer.
pub fn set_user_data(
	block: &mut VirtualBlock,
	allocation: &VirtualAllocation,
	data: usize,
) -> Result<(), AllocError> {
	allocation_info(block, allocation)?;
	block.metadata.set_user_data(
		allocation.placement.ok_or(AllocError::UnknownAllocation)?,
		data,
	)
}

/// Observe whether every virtual allocation has been explicitly released or cleared.
pub fn is_empty(block: &VirtualBlock) -> bool {
	block.metadata.is_empty()
}

/// Walk virtual placements/free spans for explicit extent statistics.
pub fn detailed_statistics(block: &VirtualBlock) -> VirtualBlockDetails {
	let mut result = VirtualBlockDetails {
		statistics: statistics(block),
		allocation_size_min: None,
		allocation_size_max: None,
		free_range_size_min: None,
		free_range_size_max: None,
	};
	block.metadata.for_each_allocation(|_, size, _| {
		result.allocation_size_min = Some(result.allocation_size_min.map_or(size, |old| old.min(size)));
		result.allocation_size_max = Some(result.allocation_size_max.map_or(size, |old| old.max(size)));
	});
	block.metadata.for_each_free(|_, size| {
		result.free_range_size_min = Some(result.free_range_size_min.map_or(size, |old| old.min(size)));
		result.free_range_size_max = Some(result.free_range_size_max.map_or(size, |old| old.max(size)));
	});
	result
}

/// Write explicit virtual metadata as ordered `vk.memory.virtual.v1` JSON.
/// # Errors
/// Host reservation fails before output; writer errors retain their kind/OS code.
pub fn write_statistics_json(
	block: &VirtualBlock,
	output: &mut impl std::io::Write,
	detailed: bool,
) -> Result<(), AllocError> {
	let stats = statistics(block);
	let mut allocations = HostVec::new(block.metadata.host_memory());
	if detailed {
		allocations
			.try_reserve_exact(stats.allocation_count)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		block
			.metadata
			.for_each_allocation(|offset, size, user_data| {
				allocations.push_reserved((offset, size, user_data))
			});
		allocations.sort_unstable_by_key(|value| value.0);
	}
	let result = (|| -> std::io::Result<()> {
		write!(
			output,
			"{{\"schema\":\"vk.memory.virtual.v1\",\"size\":{},\"allocation_count\":{},\"allocation_bytes\":{},\"free_bytes\":{},\"free_range_count\":{},\"largest_free_range\":{}",
			stats.size,
			stats.allocation_count,
			stats.allocated_bytes,
			stats.free_bytes,
			stats.free_region_count,
			stats.largest_free_region
		)?;
		if detailed {
			output.write_all(b",\"allocations\":[")?;
			for (index, (offset, size, data)) in allocations.iter().enumerate() {
				if index != 0 {
					output.write_all(b",")?;
				}
				write!(
					output,
					"{{\"offset\":{offset},\"size\":{size},\"user_data\":{data}}}"
				)?;
			}
			output.write_all(b"],\"free_ranges\":[")?;
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
			output.write_all(b"]")?;
		}
		output.write_all(b"}\n")
	})();
	result.map_err(|error| AllocError::Output {
		kind: error.kind(),
		code: error.raw_os_error(),
	})
}

impl<'callbacks> VirtualBlock<'callbacks> {
	/// Observe checked virtual emptiness in constant time.
	pub fn is_empty(&self) -> bool {
		is_empty(self)
	}

	/// Set opaque per-allocation metadata, with no native pointer dereference or ownership.
	pub fn set_user_data(
		&mut self,
		allocation: &VirtualAllocation,
		data: usize,
	) -> Result<(), AllocError> {
		set_user_data(self, allocation, data)
	}

	/// Calculate explicit virtual allocation/free-span extrema.
	pub fn detailed_statistics(&self) -> VirtualBlockDetails {
		detailed_statistics(self)
	}

	/// Stream virtual JSON diagnostics; no native memory is accessed.
	pub fn write_statistics_json(
		&self,
		output: &mut impl std::io::Write,
		detailed: bool,
	) -> Result<(), AllocError> {
		write_statistics_json(self, output, detailed)
	}

	pub fn new(size: u64) -> Result<Self, AllocError> {
		Self::new_with_algorithm(size, AllocationAlgorithm::Tlsf)
	}

	/// Create TLSF or linear host-only placement. No native storage is acquired.
	///
	/// ```
	/// use vk::{VirtualBlock, mem::{AllocationAlgorithm, PlacementStrategy}};
	/// let mut block = VirtualBlock::new_with_algorithm(4096, AllocationAlgorithm::Linear)?;
	/// let mut lower = block.allocate(64, 16, PlacementStrategy::Default)?;
	/// let mut upper = block.allocate_upper(64, 16)?;
	/// assert_eq!(block.allocation_info(&lower)?.offset, 0);
	/// assert_eq!(block.allocation_info(&upper)?.offset, 4032);
	/// block.free(&mut upper)?;
	/// block.free(&mut lower)?;
	/// # Ok::<(), vk::AllocError>(())
	/// ```
	/// # Errors
	/// Rejects a zero size or failed host metadata reservation.
	pub fn new_with_algorithm(size: u64, algorithm: AllocationAlgorithm) -> Result<Self, AllocError> {
		Ok(Self {
			owner: Identity::new()?,
			metadata: Metadata::new(size, algorithm, true)?,
		})
	}

	/// Create host-only placement whose metadata uses borrowed Vulkan allocation callbacks.
	///
	/// TLSF/linear buffers, growth and detailed-JSON scratch storage use only this
	/// allocation/free pair, with `SystemAllocationScope::OBJECT`. The block value
	/// stays inline; tokens need no reference-count allocation. Reallocation and
	/// driver internal-allocation notifications are not invoked. Drop releases
	/// host metadata only; no Vulkan device or native memory is involved.
	/// The callback record cannot expire while the block still owns storage:
	/// ```compile_fail,E0597
	/// # use vk::{VirtualBlock, mem::AllocationAlgorithm};
	/// let mut block;
	/// {
	///     let callbacks = vk::AllocationCallbacks::default();
	///     // SAFETY: demonstration of record lifetime; null functions are rejected at runtime.
	///     block = unsafe {
	///         VirtualBlock::new_with_callbacks(1024, AllocationAlgorithm::Tlsf, &callbacks)
	///     }.unwrap();
	/// }
	/// block.clear();
	/// ```
	/// # Errors
	/// Rejects zero size, a missing allocation/free function or failed host storage.
	/// # Safety
	/// Callbacks must return uniquely owned memory of the requested size/alignment,
	/// or null on failure, and free only their own allocations. They must not unwind
	/// or reenter this block. Their code and user data must remain valid through
	/// block Drop and support calls from any thread, including concurrent independent
	/// blocks. Borrowing retains the callback record, not ownership of its raw user data.
	pub unsafe fn new_with_callbacks(
		size: u64,
		algorithm: AllocationAlgorithm,
		callbacks: &'callbacks vk::AllocationCallbacks,
	) -> Result<Self, AllocError> {
		// SAFETY: caller establishes callback ownership/lifetime/thread contracts;
		// this block's metadata retains the borrow through every buffer's Drop.
		let host = unsafe { HostMemory::new_from_callbacks(callbacks) }?;
		Ok(Self {
			owner: Identity::new()?,
			metadata: Metadata::new_with_host(size, algorithm, true, host)?,
		})
	}

	/// Allocate an aligned region. Linear placement ignores the TLSF search strategy.
	pub fn allocate(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
	) -> Result<VirtualAllocation, AllocError> {
		allocate(self, size, alignment, strategy)
	}

	/// Allocate downward in a linear block. Active ring mode rejects this request.
	/// # Errors
	/// Rejects TLSF, invalid extents/alignment, exhausted space or host metadata.
	pub fn allocate_upper(
		&mut self,
		size: u64,
		alignment: u64,
	) -> Result<VirtualAllocation, AllocError> {
		allocate_upper(self, size, alignment)
	}

	/// Release a matching live identity. No native resource is destroyed or waited on.
	pub fn free(&mut self, allocation: &mut VirtualAllocation) -> Result<(), AllocError> {
		free(self, allocation)
	}

	/// Validate identity/generation and return the current offset/size snapshot.
	pub fn allocation_info(
		&self,
		allocation: &VirtualAllocation,
	) -> Result<VirtualAllocationInfo, AllocError> {
		allocation_info(self, allocation)
	}

	/// Invalidate all tokens and restore the full range, retaining host capacity.
	/// Users of previously returned offsets must stop using them before reuse.
	pub fn clear(&mut self) {
		clear(self)
	}

	/// Calculate detailed free-span statistics; this is not a constant-time hot path.
	pub fn statistics(&self) -> VirtualBlockStats {
		statistics(self)
	}
}

/// Allocate a nonzero region with a nonzero power-of-two alignment.
///
/// Failure leaves placements and donor free-list ordering unchanged. Only host
/// metadata is allocated; no Vulkan calls or memory mapping occur.
pub fn allocate(
	block: &mut VirtualBlock,
	size: u64,
	alignment: u64,
	strategy: PlacementStrategy,
) -> Result<VirtualAllocation, AllocError> {
	let placement = block.metadata.allocate(size, alignment, strategy, false)?;
	Ok(VirtualAllocation {
		owner: block.owner,
		placement: Some(placement),
	})
}

/// Allocate downward from the upper end of a linear block.
/// # Errors
/// Rejects TLSF, active ring mode, invalid extents/alignment and exhausted space/metadata.
pub fn allocate_upper(
	block: &mut VirtualBlock,
	size: u64,
	alignment: u64,
) -> Result<VirtualAllocation, AllocError> {
	let placement = block
		.metadata
		.allocate(size, alignment, PlacementStrategy::Default, true)?;
	Ok(VirtualAllocation {
		owner: block.owner,
		placement: Some(placement),
	})
}

/// Reject foreign, freed or stale tokens before reading placement metadata.
pub fn allocation_info(
	block: &VirtualBlock,
	allocation: &VirtualAllocation,
) -> Result<VirtualAllocationInfo, AllocError> {
	if block.owner != allocation.owner {
		return Err(AllocError::WrongAllocator);
	}
	block
		.metadata
		.info(allocation.placement.ok_or(AllocError::UnknownAllocation)?)
}

/// Free a checked live region and invalidate its token; no native effects.
pub fn free(
	block: &mut VirtualBlock,
	allocation: &mut VirtualAllocation,
) -> Result<(), AllocError> {
	allocation_info(block, allocation)?;
	block
		.metadata
		.free(allocation.placement.ok_or(AllocError::UnknownAllocation)?)?;
	allocation.placement = None;
	Ok(())
}

/// Clear all regions, invalidating every prior generation; never submits or waits.
pub fn clear(block: &mut VirtualBlock) {
	block.metadata.clear()
}

/// Calculate allocation bytes, free bytes, free-region count and largest span.
pub fn statistics(block: &VirtualBlock) -> VirtualBlockStats {
	block.metadata.statistics()
}
