//! Native block and shared mapping state.
//!
//! Donor: pinned VMA `VmaDeviceMemoryBlock::{Map,Unmap}` and
//! `GetMemoryTypeMinAlignment`. See LICENSE-VMA-MIT.
//! Adaptation: exclusive allocator access, checked counts and explicit release;
//! donor mapping hysteresis and no Vulkan destruction in Drop. Placement metadata owns page separation.

use super::{AllocError, metadata::Metadata};
use crate::{self as vk, load::CallError};
use std::ffi::c_void;

pub(super) struct Mapping {
	pub pointer: *mut c_void,
	pub references: u32,
	hysteresis: Hysteresis,
}

impl Default for Mapping {
	fn default() -> Self {
		Self {
			pointer: std::ptr::null_mut(),
			references: 0,
			hysteresis: Hysteresis::default(),
		}
	}
}

// Donor: VmaMappingHysteresis, same threshold and major/minor transitions.
// Saturating counters avoid wraparound on very long-lived native blocks.
#[derive(Default)]
struct Hysteresis {
	enabled: bool,
	cached: bool,
	major: u32,
	minor: u32,
}

impl Hysteresis {
	fn minor(&mut self) {
		if self.minor < self.major {
			self.minor += 1;
		} else if self.major != 0 {
			self.major -= 1;
			self.minor -= 1;
		}
	}

	fn post_map(&mut self) {
		if !self.enabled {
			return;
		}
		if self.cached {
			self.minor();
		} else {
			self.major = self.major.saturating_add(1);
			if self.major >= 7 {
				self.cached = true;
				self.major = 0;
				self.minor = 0;
			}
		}
	}

	fn post_unmap(&mut self) {
		if !self.enabled {
			return;
		}
		if self.cached {
			self.minor();
		} else {
			self.major = self.major.saturating_add(1);
		}
	}

	fn post_alloc(&mut self) {
		if !self.enabled {
			return;
		}
		if self.cached {
			self.major = self.major.saturating_add(1);
		} else {
			self.minor();
		}
	}

	fn post_free(&mut self) -> bool {
		if !self.enabled {
			return false;
		}
		if !self.cached {
			self.minor();
			return false;
		}
		self.major = self.major.saturating_add(1);
		if self.major >= 7 && self.major.saturating_sub(self.minor) > 1 {
			self.cached = false;
			self.major = 0;
			self.minor = 0;
			return true;
		}
		false
	}
}

impl Mapping {
	pub fn new_with_hysteresis(enabled: bool) -> Self {
		Self {
			hysteresis: Hysteresis {
				enabled,
				..Default::default()
			},
			..Default::default()
		}
	}

	pub fn post_alloc(&mut self) {
		self.hysteresis.post_alloc();
	}

	pub unsafe fn post_free(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
	) -> Result<(), AllocError> {
		if self.hysteresis.post_free() && self.references == 0 {
			// SAFETY: only the cache held this native mapping; no allocation pointer borrow remains.
			unsafe { self.clear_cache(device, memory) }?;
		}
		Ok(())
	}

	pub unsafe fn clear_cache(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
	) -> Result<(), AllocError> {
		if self.references != 0 {
			return Err(AllocError::AlreadyMapped);
		}
		if !self.pointer.is_null() {
			// SAFETY: no explicit/persistent holds remain; immutable inverse PFN was preflighted on acquire.
			unsafe { device.unmap_memory(memory) }?;
			self.pointer = std::ptr::null_mut();
		}
		self.hysteresis.cached = false;
		Ok(())
	}

	pub unsafe fn acquire(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
		size: u64,
	) -> Result<*mut c_void, AllocError> {
		// SAFETY: the same native lifetime/access contract applies to one hold.
		unsafe { self.acquire_many(device, memory, size, 1) }
	}

	// Transfer relocation holds as one checked native mapping event. Failed
	// acquisition leaves the old reference count and source mapping unchanged.
	pub unsafe fn acquire_many(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
		size: u64,
		holds: u32,
	) -> Result<*mut c_void, AllocError> {
		if size > isize::MAX as u64 {
			return Err(AllocError::InvalidRange);
		}
		if holds == 0 {
			return Err(AllocError::InvalidRange);
		}
		let count = self
			.references
			.checked_add(holds)
			.ok_or(AllocError::InvalidRange)?;
		if self.pointer.is_null() {
			// Preflight the inverse transition before acquiring native state.
			if device.fp().unmap_memory.is_none() {
				return Err(CallError::MissingCommand("vkUnmapMemory").into());
			}
			// SAFETY: caller owns live host-visible storage and synchronizes all access.
			let pointer = unsafe { device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty()) }?;
			if pointer.is_null() {
				// SAFETY: successful mapping must be released; inverse PFN preflighted.
				unsafe { device.unmap_memory(memory) }?;
				return Err(CallError::InvalidOutput("vkMapMemory returned a null pointer").into());
			}
			self.pointer = pointer;
		}
		self.hysteresis.post_map();
		self.references = count;
		Ok(self.pointer)
	}

	pub unsafe fn release(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
	) -> Result<(), AllocError> {
		// SAFETY: the caller ends the same pointer lifetime for one checked hold.
		unsafe { self.release_many(device, memory, 1) }
	}

	pub unsafe fn release_many(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
		holds: u32,
	) -> Result<(), AllocError> {
		if holds == 0 {
			return Err(AllocError::InvalidRange);
		}
		let count = self
			.references
			.checked_sub(holds)
			.ok_or(AllocError::NotMapped)?;
		if count == 0 && !self.hysteresis.cached {
			// SAFETY: one native map remains; caller ends the final pointer lifetime.
			unsafe { device.unmap_memory(memory) }?;
			self.pointer = std::ptr::null_mut();
		}
		self.references = count;
		self.hysteresis.post_unmap();
		Ok(())
	}
}

pub(super) struct Block<'callbacks> {
	pub defragmentation_hold: bool,
	pub automatic: bool,
	pub export_types: vk::ExternalMemoryHandleTypeFlags,
	pub priority: super::MemoryPriority,
	pub pool: Option<u64>,
	pub handle: vk::DeviceMemory,
	pub size: u64,
	pub memory_type_index: u32,
	pub metadata: Metadata<'callbacks>,
	pub mapping: Mapping,
}

/// One compatible native block vector inside the allocator's shared storage.
/// Explicit block sizes stay separate; automatic vectors contain varied sizes.
#[derive(Clone, Copy)]
pub(super) struct BlockDomain {
	pub memory_type_index: u32,
	pub pool: Option<u64>,
	pub automatic: bool,
	pub priority: super::MemoryPriority,
	pub exact_size: Option<u64>,
}

impl BlockDomain {
	pub fn new_from_block(block: &Block) -> Self {
		Self {
			memory_type_index: block.memory_type_index,
			pool: block.pool,
			automatic: block.automatic,
			priority: block.priority,
			exact_size: (!block.automatic).then_some(block.size),
		}
	}

	pub fn contains(self, block: &Block) -> bool {
		block.memory_type_index == self.memory_type_index
			&& block.pool == self.pool
			&& block.automatic == self.automatic
			&& block.priority == self.priority
			&& self.exact_size.is_none_or(|size| block.size == size)
	}

	// Donor: VmaBlockVector::IncrementallySortBlocks. Swap only the first
	// inverted adjacent pair in this vector, leaving other domains in place.
	// Linear vectors retain creation order; Extensive retains its own ordering.
	pub fn incrementally_sort(self, blocks: &mut [Block]) {
		if blocks.iter().any(|block| {
			self.contains(block)
				&& (block.defragmentation_hold || matches!(block.metadata, Metadata::Linear(_)))
		}) {
			return;
		}
		let mut previous = None;
		for index in 0..blocks.len() {
			if !self.contains(&blocks[index]) {
				continue;
			}
			if let Some((before, free_bytes)) = previous
				&& free_bytes > blocks[index].metadata.free_bytes()
			{
				blocks.swap(before, index);
				return;
			}
			previous = Some((index, blocks[index].metadata.free_bytes()));
		}
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_mapping_hysteresis.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../test/rs/mem/test_block.rs"]
mod order_tests;
