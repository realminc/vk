//! Checked, driver-independent memory-type selection.

use super::{AllocError, AllocRequest};
use crate as vk;

/// Validated device memory properties and non-coherent atom size.
pub struct MemoryTypeSelector {
	pub(crate) properties: vk::PhysicalDeviceMemoryProperties,
	pub(crate) atom: u64,
	pub(super) device_coherent: bool,
	pub(super) allowed_bits: u32,
}

impl MemoryTypeSelector {
	pub fn new(
		properties: vk::PhysicalDeviceMemoryProperties,
		atom: u64,
	) -> Result<Self, AllocError> {
		if properties.memory_type_count == 0
			|| properties.memory_type_count > 32
			|| properties.memory_heap_count == 0
			|| properties.memory_heap_count > 16
		{
			return Err(AllocError::InvalidProperties(
				"invalid memory type or heap count",
			));
		}
		if !atom.is_power_of_two() {
			return Err(AllocError::InvalidProperties(
				"atom size must be a nonzero power of two",
			));
		}
		for memory in &properties.memory_types[..properties.memory_type_count as usize] {
			if memory.heap_index >= properties.memory_heap_count {
				return Err(AllocError::InvalidProperties(
					"memory type references an absent heap",
				));
			}
		}
		Ok(Self {
			properties,
			atom,
			device_coherent: false,
			allowed_bits: u32::MAX,
		})
	}

	/// Select a compatible type; preferred-bit ties retain the lower index.
	///
	/// Donor: VMA `VmaAllocator_T::FindMemoryTypeIndex`, required-bit test and
	/// missing-preferred-bit cost. Adaptation: checked metadata, no hidden usage policy.
	pub fn find_memory_type(&self, request: &AllocRequest) -> Result<u32, AllocError> {
		self.find_memory_type_with_preferences(request, vk::MemoryPropertyFlags::empty())
	}

	/// Required flags are mandatory; cost counts missing preferred and present avoided bits.
	pub fn find_memory_type_with_preferences(
		&self,
		request: &AllocRequest,
		avoided: vk::MemoryPropertyFlags,
	) -> Result<u32, AllocError> {
		if request.size == 0 || !request.alignment.is_power_of_two() {
			return Err(AllocError::InvalidRequest(
				"size must be nonzero and alignment a power of two",
			));
		}
		let mut best = None;
		let mut cost = u32::MAX;
		let required = request.required_flags.as_raw();
		let preferred = request.preferred_flags.as_raw();
		let avoided = avoided.as_raw();
		let protected = vk::MemoryPropertyFlagBits::PROTECTED_BIT.as_raw();
		let coherent = vk::MemoryPropertyFlagBits::DEVICE_COHERENT_BIT_AMD.as_raw();
		let tile = vk::MemoryHeapFlagBits::TILE_MEMORY_BIT_QCOM.as_raw();
		// Visit only admitted bits, still in increasing index order for ties.
		let mut bits = request.memory_type_bits
			& self.allowed_bits
			& (u32::MAX >> (32 - self.properties.memory_type_count));
		while bits != 0 {
			let i = bits.trailing_zeros();
			bits &= bits - 1;
			let memory = self.properties.memory_types[i as usize];
			let heap = self.properties.memory_heaps[memory.heap_index as usize];
			let flags = memory.property_flags.as_raw();
			if flags & required != required {
				continue;
			}
			// Tile heaps have a separate binding/size contract; ordinary storage
			// must not acquire them merely because their property flags match.
			if heap.flags.as_raw() & tile != 0 {
				continue;
			}
			// AMD device-coherent types require captured enabled feature admission.
			// A standalone selector is conservative. Protected types require explicit flags.
			if !self.device_coherent && flags & coherent != 0 {
				continue;
			}
			if flags & protected != 0 && required & protected == 0 {
				continue;
			}
			if request.size > heap.size {
				continue;
			}
			// The last admitted candidate wins outright when nothing was chosen,
			// so its cost (a software bit count on baseline x86-64) is not needed.
			if best.is_none() && bits == 0 {
				best = Some(i);
				break;
			}
			let candidate = (preferred & !flags).count_ones() + (avoided & flags).count_ones();
			if candidate < cost {
				best = Some(i);
				cost = candidate;
				// Costs are nonnegative; the first perfect match also wins every tie.
				if cost == 0 {
					break;
				}
			}
		}
		best.ok_or(AllocError::NoCompatibleMemoryType)
	}
}
