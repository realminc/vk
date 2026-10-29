//! Buffer/image page separation, derived from pinned VMA 3.4.0
//! `VmaIsBufferImageGranularityConflict` and linear neighbor checks.
//! See LICENSE-VMA-MIT. Ordinary buffers and images are admitted.
//! Raw/unclassified storage and unknown image tiling conflict with every native class, including
//! themselves, and therefore occupy otherwise unshared pages. Known classes have
//! two compatibility groups. Within valid placements the nearest live
//! neighbor on each side still suffices: an incompatible farther neighbor on that
//! same page would already conflict with the nearest one.
//!
//! Adaptation: TLSF uses physical neighbors instead of VMA's per-page counters
//! and small-granularity rounding. Padding stays free. Upper placement aligns
//! the allocation's end below the next page before aligning its start, including
//! extents larger than a page. Unknown device limits conservatively treat the
//! whole block as one page. Virtual placement uses granularity one.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ResourceClass {
	Unassigned,
	Buffer,
	LinearImage,
	OptimalImage,
	UnknownImage,
	Unknown,
}

impl ResourceClass {
	fn conflicts(self, other: Self) -> bool {
		self != Self::Unassigned
			&& other != Self::Unassigned
			&& (self == Self::Unknown
				|| other == Self::Unknown
				|| self == Self::UnknownImage
				|| other == Self::UnknownImage
				|| ((self == Self::OptimalImage) != (other == Self::OptimalImage)))
	}
}

/// None is unknown; Some is a nonzero power of two validated at admission.
#[derive(Clone, Copy)]
pub(super) struct Granularity(pub Option<u64>);

impl Granularity {
	pub fn conflict(
		self,
		offset: u64,
		size: u64,
		class: ResourceClass,
		next_offset: u64,
		next_class: ResourceClass,
	) -> bool {
		class.conflicts(next_class)
			&& self
				.0
				.is_none_or(|page| (offset + size - 1) & !(page - 1) == next_offset & !(page - 1))
	}

	pub fn after(
		self,
		offset: u64,
		class: ResourceClass,
		previous_offset: u64,
		previous_size: u64,
		previous_class: ResourceClass,
	) -> Option<u64> {
		if self.conflict(
			previous_offset,
			previous_size,
			previous_class,
			offset,
			class,
		) {
			let page = self.0?;
			offset.checked_add(page - 1).map(|end| end & !(page - 1))
		} else {
			Some(offset)
		}
	}

	pub fn before(
		self,
		offset: u64,
		size: u64,
		alignment: u64,
		class: ResourceClass,
		next_offset: u64,
		next_class: ResourceClass,
	) -> Option<u64> {
		if self.conflict(offset, size, class, next_offset, next_class) {
			let page = self.0?;
			(next_offset & !(page - 1))
				.checked_sub(size)
				.map(|start| start & !(alignment - 1))
		} else {
			Some(offset)
		}
	}
}
