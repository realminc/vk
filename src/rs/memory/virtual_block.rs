//! Host-only region placement over an integer byte range.
//!
//! No Vulkan memory, buffers, mapping, synchronization or implicit retirement is
//! provided. Consumers own the validity/completion of any resource using these
//! offsets. Tokens are explicit: dropping one does not release its region.
//!
//! ```
//! use vk::memory::{PlacementStrategy, VirtualBlock};
//! let mut block = VirtualBlock::new(4096)?;
//! let mut region = block.allocate(64, 16, PlacementStrategy::Default)?;
//! assert_eq!(block.allocation_info(&region)?.offset, 0);
//! block.free(&mut region)?;
//! # Ok::<(), vk::AllocError>(())
//! ```

use super::{AllocError, tlsf::Tlsf};
use std::sync::Arc;

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
	owner: Arc<()>,
	index: usize,
	serial: u64,
}

/// Placement snapshot; valid only while the matching allocation remains live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualAllocationInfo {
	pub offset: u64,
	pub size: u64,
}

/// Host metadata statistics. Alignment holes remain free, not reserved to a token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualBlockStats {
	pub size: u64,
	pub allocation_count: usize,
	pub allocated_bytes: u64,
	pub free_bytes: u64,
	pub free_region_count: usize,
	pub largest_free_region: u64,
}

/// Virtual TLSF allocator; owns host metadata and no native objects.
///
/// Mutations require exclusive access. Drop releases Rust metadata only.
/// This is not a Vulkan memory block or a pool with GPU lifetime tracking.
pub struct VirtualBlock {
	owner: Arc<()>,
	metadata: Tlsf,
}

impl VirtualBlock {
	pub fn new(size: u64) -> Result<Self, AllocError> {
		Ok(Self {
			owner: Arc::new(()),
			metadata: Tlsf::new(size)?,
		})
	}

	/// Allocate an aligned region using the chosen donor search order.
	pub fn allocate(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
	) -> Result<VirtualAllocation, AllocError> {
		allocate(self, size, alignment, strategy)
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
	let (index, serial) = block.metadata.allocate(size, alignment, strategy)?;
	Ok(VirtualAllocation {
		owner: block.owner.clone(),
		index,
		serial,
	})
}

/// Reject foreign, freed or stale tokens before reading placement metadata.
pub fn allocation_info(
	block: &VirtualBlock,
	allocation: &VirtualAllocation,
) -> Result<VirtualAllocationInfo, AllocError> {
	if !Arc::ptr_eq(&block.owner, &allocation.owner) {
		return Err(AllocError::WrongAllocator);
	}
	block.metadata.info(allocation.index, allocation.serial)
}

/// Free a checked live region and invalidate its token; no native effects.
pub fn free(
	block: &mut VirtualBlock,
	allocation: &mut VirtualAllocation,
) -> Result<(), AllocError> {
	allocation_info(block, allocation)?;
	block.metadata.free(allocation.index, allocation.serial)?;
	allocation.index = usize::MAX;
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
