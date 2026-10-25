//! Native resource-class block and shared mapping state.
//!
//! Donor: pinned VMA `VmaDeviceMemoryBlock::{Map,Unmap}` and
//! `GetMemoryTypeMinAlignment`. See LICENSE-VMA-MIT.
//! Adaptation: exclusive allocator access, checked counts and explicit release;
//! no mapping hysteresis or native Drop. Homogeneous resource classes need no incompatible neighbors.

use super::{AllocError, tlsf::Tlsf};
use crate::{self as vk, load::CallError};
use std::ffi::c_void;

pub(super) struct Mapping {
	pub pointer: *mut c_void,
	pub references: u32,
}

impl Default for Mapping {
	fn default() -> Self {
		Self {
			pointer: std::ptr::null_mut(),
			references: 0,
		}
	}
}

impl Mapping {
	pub unsafe fn acquire(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
		size: u64,
	) -> Result<*mut c_void, AllocError> {
		if size > isize::MAX as u64 {
			return Err(AllocError::InvalidRange);
		}
		let count = self
			.references
			.checked_add(1)
			.ok_or(AllocError::InvalidRange)?;
		if self.references == 0 {
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
		self.references = count;
		Ok(self.pointer)
	}

	pub unsafe fn release(
		&mut self,
		device: &vk::Device,
		memory: vk::DeviceMemory,
	) -> Result<(), AllocError> {
		let count = self
			.references
			.checked_sub(1)
			.ok_or(AllocError::NotMapped)?;
		if count == 0 {
			// SAFETY: one native map remains; caller ends the final pointer lifetime.
			unsafe { device.unmap_memory(memory) }?;
			self.pointer = std::ptr::null_mut();
		}
		self.references = count;
		Ok(())
	}
}

/// Separate block domains exclude buffer/image granularity conflicts by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ResourceClass {
	Buffer,
	LinearImage,
	OptimalImage,
}

pub(super) struct Block {
	pub class: ResourceClass,
	pub handle: vk::DeviceMemory,
	pub size: u64,
	pub memory_type_index: u32,
	pub metadata: Tlsf,
	pub mapping: Mapping,
}
