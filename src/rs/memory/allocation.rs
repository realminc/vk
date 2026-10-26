//! Checked separate or pooled allocation identity and bounded cache ranges.

use super::AllocError;
use crate as vk;
use std::sync::Arc;

/// One live native region. No implicit free, unmap, submission or wait in Drop.
pub struct Allocation {
	pub(crate) handle: vk::DeviceMemory,
	pub(crate) size: u64,
	pub(crate) offset: u64,
	pub(crate) block_size: u64,
	pub(crate) reserved_size: u64,
	pub(crate) memory_type_index: u32,
	pub(crate) heap_index: u32,
	pub(crate) flags: vk::MemoryPropertyFlags,
	pub(crate) atom: u64,
	pub(crate) owner: Arc<()>,
	pub(crate) serial: u64,
}

impl Allocation {
	pub fn handle(&self) -> vk::DeviceMemory {
		self.handle
	}

	pub fn offset(&self) -> u64 {
		self.offset
	}

	pub fn size(&self) -> u64 {
		self.size
	}

	/// Native extent reserved to this region, including non-coherent atom padding.
	pub fn reserved_size(&self) -> u64 {
		self.reserved_size
	}

	pub fn memory_type_index(&self) -> u32 {
		self.memory_type_index
	}

	pub fn heap_index(&self) -> u32 {
		self.heap_index
	}

	pub fn is_host_coherent(&self) -> bool {
		self.flags.as_raw() & vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT.as_raw() != 0
	}

	pub fn is_host_visible(&self) -> bool {
		self.flags.as_raw() & vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT.as_raw() != 0
	}

	pub fn is_freed(&self) -> bool {
		self.handle.is_null()
	}

	/// Translate an allocation-relative range to a native block range.
	/// This does not validate allocator liveness or establish a mapping.
	///
	/// Donor: VMA `VmaAllocator_T::GetFlushOrInvalidateRange`.
	/// Adaptation: checked arithmetic, no-op ranges, allocation-end exception.
	pub fn mapped_range(
		&self,
		offset: u64,
		size: u64,
	) -> Result<Option<vk::MappedMemoryRange<'static>>, AllocError> {
		if self.is_freed() {
			return Err(AllocError::UnknownAllocation);
		}
		if !self.is_host_visible() {
			return Err(AllocError::NotHostVisible);
		}
		let end = if size == vk::WHOLE_SIZE {
			self.size
		} else {
			offset.checked_add(size).ok_or(AllocError::InvalidRange)?
		};
		if offset > self.size || end > self.size || end < offset {
			return Err(AllocError::InvalidRange);
		}
		if offset == end || self.is_host_coherent() {
			return Ok(None);
		}
		let absolute_end = self
			.offset
			.checked_add(end)
			.ok_or(AllocError::InvalidRange)?;
		let start = self
			.offset
			.checked_add(offset)
			.ok_or(AllocError::InvalidRange)?
			& !(self.atom - 1);
		let rounded = absolute_end
			.checked_add(self.atom - 1)
			.map(|v| v & !(self.atom - 1))
			.unwrap_or(self.block_size)
			.min(self.block_size);
		Ok(Some(vk::MappedMemoryRange {
			memory: self.handle,
			offset: start,
			size: rounded - start,
			..Default::default()
		}))
	}
}

#[cfg(test)]
#[path = "../../../test/rs/memory/test_allocation.rs"]
mod tests;
