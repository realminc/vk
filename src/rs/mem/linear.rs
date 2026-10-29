//! Integer linear placement; no native calls or implicit retirement.
//!
//! Donor: VMA 3.4.0, 3aa921224c154a0d2c43912bc88e1c42ce1f7607,
//! `VmaBlockMetadata_Linear::{CreateAllocationRequest_LowerAddress,
//! CreateAllocationRequest_UpperAddress,Alloc,Free,CleanupAfterFree,Clear}`.
//! See LICENSE-VMA-MIT. Adaptation: checked arithmetic and generations, fallible
//! reserve before publication, live-neighbor granularity checks and atom padding.
//! Ring wrap is explicitly disabled for multi-block pools. Invalid ring/upper
//! combinations return errors rather than invoking the donor's debug assertion.

use super::{
	AllocError,
	granularity::{Granularity, ResourceClass},
	host::{HostMemory, HostVec},
	metadata::Placement,
	virtual_block::{VirtualAllocationInfo, VirtualBlockStats},
};

#[derive(Clone, Copy, PartialEq)]
enum Mode {
	Empty,
	Ring,
	DoubleStack,
}

#[derive(Clone, Copy)]
struct Node {
	offset: u64,
	size: u64,
	serial: u64,
	live: bool,
	class: ResourceClass,
	user_data: usize,
}

pub(super) struct Linear<'callbacks> {
	pub(super) granularity: Granularity,
	size: u64,
	vectors: [HostVec<'callbacks, Node>; 2],
	first: usize,
	mode: Mode,
	begin: usize,
	middle: usize,
	second_null: usize,
	allocations: usize,
	free_bytes: u64,
	next_serial: u64,
	allow_ring: bool,
}

impl<'callbacks> Linear<'callbacks> {
	pub(super) fn free_bytes(&self) -> u64 {
		self.free_bytes
	}

	pub(super) fn is_empty(&self) -> bool {
		self.allocations == 0
	}

	pub fn new(size: u64, allow_ring: bool) -> Result<Self, AllocError> {
		Self::new_with_host(size, allow_ring, HostMemory::default())
	}

	pub fn host_memory(&self) -> HostMemory<'callbacks> {
		self.vectors[0].host_memory()
	}

	pub fn new_with_host(
		size: u64,
		allow_ring: bool,
		host: HostMemory<'callbacks>,
	) -> Result<Self, AllocError> {
		if size == 0 {
			return Err(AllocError::InvalidRequest(
				"linear block size must be nonzero",
			));
		}
		Ok(Self {
			granularity: Granularity(Some(1)),
			size,
			vectors: [HostVec::new(host), HostVec::new(host)],
			first: 0,
			mode: Mode::Empty,
			begin: 0,
			middle: 0,
			second_null: 0,
			allocations: 0,
			free_bytes: size,
			next_serial: 1,
			allow_ring,
		})
	}

	pub fn allocate(
		&mut self,
		size: u64,
		alignment: u64,
		upper: bool,
	) -> Result<Placement, AllocError> {
		self.allocate_resource(size, alignment, upper, ResourceClass::Unassigned)
	}

	pub fn allocate_resource(
		&mut self,
		size: u64,
		alignment: u64,
		upper: bool,
		class: ResourceClass,
	) -> Result<Placement, AllocError> {
		if size == 0 || !alignment.is_power_of_two() {
			return Err(AllocError::InvalidRequest(
				"nonzero size and power-of-two alignment required",
			));
		}
		if self.next_serial == u64::MAX {
			return Err(AllocError::SerialExhausted);
		}
		if size > self.size {
			return Err(AllocError::OutOfSpace);
		}
		let (offset, second) = if upper {
			if self.mode == Mode::Ring {
				return Err(AllocError::InvalidRequest(
					"upper address during active ring placement",
				));
			}
			let end = self.vectors[self.first ^ 1]
				.last()
				.map_or(self.size, |node| node.offset);
			let mut offset = end.checked_sub(size).ok_or(AllocError::OutOfSpace)? & !(alignment - 1);
			if let Some(next) = self.vectors[self.first ^ 1].last() {
				offset = self
					.granularity
					.before(offset, size, alignment, class, next.offset, next.class)
					.ok_or(AllocError::OutOfSpace)?;
			}
			let lower_end = self.vectors[self.first]
				.last()
				.map_or(0, |node| node.offset + node.size);
			if lower_end > offset
				|| self.vectors[self.first].last().is_some_and(|previous| {
					self.granularity.conflict(
						previous.offset,
						previous.size,
						previous.class,
						offset,
						class,
					)
				}) {
				return Err(AllocError::OutOfSpace);
			}
			(offset, true)
		} else {
			self
				.choose_lower(size, alignment, class)
				.ok_or(AllocError::OutOfSpace)?
		};
		let vector = self.first ^ usize::from(second);
		// Allocation/identity failure must not advance cursors or change modes.
		self.vectors[vector]
			.try_reserve(1)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		let serial = self.next_serial;
		self.vectors[vector].push_reserved(Node {
			offset,
			size,
			serial,
			live: true,
			user_data: 0,
			class,
		});
		self.next_serial += 1;
		self.allocations += 1;
		self.free_bytes -= size;
		if second {
			self.mode = if upper { Mode::DoubleStack } else { Mode::Ring };
		}
		Ok(Placement::Linear { offset, serial })
	}

	fn choose_lower(&self, size: u64, alignment: u64, class: ResourceClass) -> Option<(u64, bool)> {
		let fits = |previous: Option<&Node>, next: Option<&Node>| {
			let base = previous.map_or(0, |node| node.offset + node.size);
			let end = next.map_or(self.size, |node| node.offset);
			let mut offset = base.checked_add(alignment - 1)? & !(alignment - 1);
			if let Some(previous) = previous {
				offset = self.granularity.after(
					offset,
					class,
					previous.offset,
					previous.size,
					previous.class,
				)?;
			}
			if offset > end || size > end - offset {
				return None;
			}
			if next.is_some_and(|next| {
				self
					.granularity
					.conflict(offset, size, class, next.offset, next.class)
			}) {
				return None;
			}
			Some(offset)
		};
		let first = &self.vectors[self.first];
		let second = &self.vectors[self.first ^ 1];
		if self.mode != Mode::Ring {
			let next = if self.mode == Mode::DoubleStack {
				second.last()
			} else {
				None
			};
			if let Some(offset) = fits(first.last(), next) {
				return Some((offset, false));
			}
		}
		if self.allow_ring && self.mode != Mode::DoubleStack {
			return fits(second.last(), first.get(self.begin)).map(|offset| (offset, true));
		}
		None
	}

	fn locate(&self, offset: u64, serial: u64) -> Result<(usize, usize), AllocError> {
		for side in 0..2 {
			let vector = self.first ^ side;
			let found = if side == 1 && self.mode == Mode::DoubleStack {
				self.vectors[vector].binary_search_by(|node| offset.cmp(&node.offset))
			} else {
				self.vectors[vector].binary_search_by_key(&offset, |node| node.offset)
			};
			if let Ok(index) = found {
				let node = &self.vectors[vector][index];
				if node.live && node.serial == serial {
					return Ok((vector, index));
				}
			}
		}
		Err(AllocError::UnknownAllocation)
	}

	pub fn info(&self, offset: u64, serial: u64) -> Result<VirtualAllocationInfo, AllocError> {
		let (vector, index) = self.locate(offset, serial)?;
		Ok(VirtualAllocationInfo {
			offset,
			size: self.vectors[vector][index].size,
			user_data: self.vectors[vector][index].user_data,
		})
	}

	pub fn free(&mut self, offset: u64, serial: u64) -> Result<(), AllocError> {
		let (vector, index) = self.locate(offset, serial)?;
		let node = &mut self.vectors[vector][index];
		node.live = false;
		self.free_bytes += node.size;
		self.allocations -= 1;
		if vector == self.first {
			self.middle += 1;
		} else {
			self.second_null += 1;
		}
		self.cleanup();
		Ok(())
	}

	fn cleanup(&mut self) {
		if self.allocations == 0 {
			self.clear();
			return;
		}
		let first = &mut self.vectors[self.first];
		while self.begin < first.len() && !first[self.begin].live {
			self.begin += 1;
			self.middle -= 1;
		}
		while self.middle != 0 && first.last().is_some_and(|node| !node.live) {
			first.pop();
			self.middle -= 1;
		}
		// Pinned donor compaction threshold; tokens use offset/generation, not vector indices.
		let null = self.begin + self.middle;
		if first.len() > 32
			&& null >= first.len() - null
			&& null - (first.len() - null) >= (first.len() - null).div_ceil(2)
		{
			first.retain(|node| node.live);
			self.begin = 0;
			self.middle = 0;
		}
		let second = &mut self.vectors[self.first ^ 1];
		while self.second_null != 0 && second.last().is_some_and(|node| !node.live) {
			second.pop();
			self.second_null -= 1;
		}
		let leading = second.iter().take_while(|node| !node.live).count();
		if leading != 0 {
			second.remove_prefix(leading);
			self.second_null -= leading;
		}
		if second.is_empty() {
			self.mode = Mode::Empty;
		}
		if self.vectors[self.first].len() == self.begin {
			self.vectors[self.first].clear();
			self.begin = 0;
			if self.mode == Mode::Ring {
				self.mode = Mode::Empty;
				self.first ^= 1;
				self.middle = self.second_null;
				self.second_null = 0;
			}
		}
	}

	pub fn clear(&mut self) {
		self.vectors[0].clear();
		self.vectors[1].clear();
		self.mode = Mode::Empty;
		self.begin = 0;
		self.middle = 0;
		self.second_null = 0;
		self.allocations = 0;
		self.free_bytes = self.size;
		// Never reset generations: old tokens must stay invalid after clear/reuse.
	}

	pub fn set_user_data(&mut self, offset: u64, serial: u64, data: usize) -> Result<(), AllocError> {
		let (vector, index) = self.locate(offset, serial)?;
		self.vectors[vector][index].user_data = data;
		Ok(())
	}

	pub fn for_each_allocation(&self, mut visit: impl FnMut(u64, u64, usize)) {
		for vector in &self.vectors {
			for node in vector {
				if node.live {
					visit(node.offset, node.size, node.user_data);
				}
			}
		}
	}

	pub fn for_each_free(&self, mut emit: impl FnMut(u64, u64)) {
		let mut cursor = 0;
		let mut visit = |node: &Node| {
			if node.live {
				if node.offset > cursor {
					emit(cursor, node.offset - cursor);
				}
				cursor = node.offset + node.size;
			}
		};
		if self.mode == Mode::Ring {
			for node in &self.vectors[self.first ^ 1] {
				visit(node);
			}
		}
		for node in &self.vectors[self.first] {
			visit(node);
		}
		if self.mode == Mode::DoubleStack {
			for node in self.vectors[self.first ^ 1].iter().rev() {
				visit(node);
			}
		}
		if cursor < self.size {
			emit(cursor, self.size - cursor);
		}
	}

	pub fn statistics(&self) -> VirtualBlockStats {
		let mut result = VirtualBlockStats {
			size: self.size,
			allocation_count: self.allocations,
			allocated_bytes: self.size - self.free_bytes,
			free_bytes: self.free_bytes,
			free_region_count: 0,
			largest_free_region: 0,
		};
		self.for_each_free(|_, size| {
			result.free_region_count += 1;
			result.largest_free_region = result.largest_free_region.max(size);
		});
		result
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_linear.rs"]
mod tests;
