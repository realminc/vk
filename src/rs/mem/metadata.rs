//! Shared, private placement dispatch for native and virtual blocks.

use super::{
	AllocError,
	granularity::{Granularity, ResourceClass},
	host::HostMemory,
	linear::Linear,
	tlsf::Tlsf,
	virtual_block::{PlacementStrategy, VirtualAllocationInfo, VirtualBlockStats},
};

/// Placement metadata for custom pools and host-only virtual blocks.
///
/// Linear placement appends after the current tail; middle holes are not reused.
/// Releasing all regions restarts the range. Last-in-first-out release rewinds
/// a stack; a single-block pool also supports ring and double-stack lifetimes.
/// Strategy flags affect TLSF only. These are placement rules, not GPU lifetime
/// tracking or latency guarantees.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AllocationAlgorithm {
	#[default]
	Tlsf,
	Linear,
}

#[derive(Clone, Copy)]
pub(super) enum Placement {
	Tlsf(usize, u64),
	Linear { offset: u64, serial: u64 },
}

// Keep the existing TLSF block header inline and host acquisition fallible.
// Boxing it would add an unrelated, infallible allocation per native block.
#[expect(
	clippy::large_enum_variant,
	reason = "inline metadata preserves fallible block acquisition"
)]
pub(super) enum Metadata<'callbacks> {
	Tlsf(Tlsf<'callbacks>),
	Linear(Linear<'callbacks>),
}

impl<'callbacks> Metadata<'callbacks> {
	/// Total free extent in constant time, including currently unusable holes.
	pub fn free_bytes(&self) -> u64 {
		match self {
			Self::Tlsf(metadata) => metadata.free_bytes(),
			Self::Linear(metadata) => metadata.free_bytes(),
		}
	}

	pub fn is_empty(&self) -> bool {
		match self {
			Self::Tlsf(metadata) => metadata.is_empty(),
			Self::Linear(metadata) => metadata.is_empty(),
		}
	}

	pub fn new_native(
		size: u64,
		algorithm: AllocationAlgorithm,
		allow_ring: bool,
		granularity: Granularity,
		host: HostMemory<'callbacks>,
	) -> Result<Self, AllocError> {
		let mut metadata = Self::new_with_host(size, algorithm, allow_ring, host)?;
		match &mut metadata {
			Self::Tlsf(metadata) => metadata.granularity = granularity,
			Self::Linear(metadata) => metadata.granularity = granularity,
		}
		Ok(metadata)
	}

	pub fn new(
		size: u64,
		algorithm: AllocationAlgorithm,
		allow_ring: bool,
	) -> Result<Self, AllocError> {
		match algorithm {
			AllocationAlgorithm::Tlsf => Tlsf::new(size).map(Self::Tlsf),
			AllocationAlgorithm::Linear => Linear::new(size, allow_ring).map(Self::Linear),
		}
	}

	pub fn host_memory(&self) -> HostMemory<'callbacks> {
		match self {
			Self::Tlsf(metadata) => metadata.host_memory(),
			Self::Linear(metadata) => metadata.host_memory(),
		}
	}

	pub fn new_with_host(
		size: u64,
		algorithm: AllocationAlgorithm,
		allow_ring: bool,
		host: HostMemory<'callbacks>,
	) -> Result<Self, AllocError> {
		match algorithm {
			AllocationAlgorithm::Tlsf => Tlsf::new_with_host(size, host).map(Self::Tlsf),
			AllocationAlgorithm::Linear => {
				Linear::new_with_host(size, allow_ring, host).map(Self::Linear)
			}
		}
	}

	pub fn allocate(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
		upper: bool,
	) -> Result<Placement, AllocError> {
		match self {
			Self::Tlsf(metadata) => {
				if upper {
					return Err(AllocError::UnsupportedCapability(
						"upper address requires linear placement",
					));
				}
				metadata
					.allocate(size, alignment, strategy)
					.map(|(index, serial)| Placement::Tlsf(index, serial))
			}
			Self::Linear(metadata) => metadata.allocate(size, alignment, upper),
		}
	}

	pub fn allocate_resource(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
		upper: bool,
		class: ResourceClass,
	) -> Result<Placement, AllocError> {
		match self {
			Self::Tlsf(metadata) => {
				if upper {
					return Err(AllocError::UnsupportedCapability(
						"upper address requires linear placement",
					));
				}
				metadata
					.allocate_resource(size, alignment, strategy, class)
					.map(|(index, serial)| Placement::Tlsf(index, serial))
			}
			Self::Linear(metadata) => metadata.allocate_resource(size, alignment, upper, class),
		}
	}

	pub fn info(&self, placement: Placement) -> Result<VirtualAllocationInfo, AllocError> {
		match (self, placement) {
			(Self::Tlsf(metadata), Placement::Tlsf(index, serial)) => metadata.info(index, serial),
			(Self::Linear(metadata), Placement::Linear { offset, serial }) => {
				metadata.info(offset, serial)
			}
			_ => Err(AllocError::UnknownAllocation),
		}
	}

	pub fn free(&mut self, placement: Placement) -> Result<(), AllocError> {
		match (self, placement) {
			(Self::Tlsf(metadata), Placement::Tlsf(index, serial)) => metadata.free(index, serial),
			(Self::Linear(metadata), Placement::Linear { offset, serial }) => {
				metadata.free(offset, serial)
			}
			_ => Err(AllocError::UnknownAllocation),
		}
	}

	pub fn clear(&mut self) {
		match self {
			Self::Tlsf(metadata) => metadata.clear(),
			Self::Linear(metadata) => metadata.clear(),
		}
	}

	pub fn set_user_data(&mut self, placement: Placement, data: usize) -> Result<(), AllocError> {
		match (self, placement) {
			(Self::Tlsf(metadata), Placement::Tlsf(index, serial)) => {
				metadata.set_user_data(index, serial, data)
			}
			(Self::Linear(metadata), Placement::Linear { offset, serial }) => {
				metadata.set_user_data(offset, serial, data)
			}
			_ => Err(AllocError::UnknownAllocation),
		}
	}

	pub fn for_each_allocation(&self, visit: impl FnMut(u64, u64, usize)) {
		match self {
			Self::Tlsf(metadata) => metadata.for_each_allocation(visit),
			Self::Linear(metadata) => metadata.for_each_allocation(visit),
		}
	}

	pub fn for_each_free(&self, visit: impl FnMut(u64, u64)) {
		match self {
			Self::Tlsf(metadata) => metadata.for_each_free(visit),
			Self::Linear(metadata) => metadata.for_each_free(visit),
		}
	}

	pub fn statistics(&self) -> VirtualBlockStats {
		match self {
			Self::Tlsf(metadata) => metadata.statistics(),
			Self::Linear(metadata) => metadata.statistics(),
		}
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_granularity.rs"]
mod tests;
