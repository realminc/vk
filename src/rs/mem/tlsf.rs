//! Integer TLSF placement metadata; no Vulkan resources or native calls.
//!
//! Donor: VMA 3.4.0, 3aa921224c154a0d2c43912bc88e1c42ce1f7607,
//! `VmaBlockMetadata_TLSF::{CreateAllocationRequest,Alloc,Free,SizeToMemoryClass,
//! SizeToSecondIndex,FindFreeBlock,CheckBlock}`. See LICENSE-VMA-MIT.
//! Adaptation: stable arena indices, checked arithmetic/generations, fallible
//! reservation before mutation and a 64-bit first-level map for the full u64 domain.
//! Bucket geometry follows donor virtual mode. Native resource blocks
//! reuse it with atom isolation and physical-neighbor granularity checks.

use super::{
	AllocError,
	granularity::{Granularity, ResourceClass},
	host::{HostMemory, HostVec},
	virtual_block::{PlacementStrategy, VirtualAllocationInfo, VirtualBlockStats},
};

const NONE: usize = usize::MAX;
const CLASSES: usize = 57;

#[derive(Clone, Copy, PartialEq)]
enum State {
	Unused,
	Taken,
	Free,
}

struct Node {
	offset: u64,
	size: u64,
	previous: usize,
	next: usize,
	previous_free: usize,
	next_free: usize,
	state: State,
	serial: u64,
	class: ResourceClass,
	user_data: usize,
}

impl Node {
	fn new(offset: u64, size: u64, previous: usize, next: usize) -> Self {
		Self {
			offset,
			size,
			previous,
			next,
			previous_free: NONE,
			next_free: NONE,
			state: State::Taken,
			serial: 0,
			class: ResourceClass::Unassigned,
			user_data: 0,
		}
	}
}

pub(super) struct Tlsf<'callbacks> {
	pub(super) granularity: Granularity,
	size: u64,
	nodes: HostVec<'callbacks, Node>,
	head: usize,
	tail: usize,
	recycled: usize,
	recycled_count: usize,
	heads: HostVec<'callbacks, usize>,
	first_map: u64,
	second_maps: [u32; CLASSES],
	free_count: usize,
	free_bytes: u64,
	allocations: usize,
	next_serial: u64,
}

fn class(size: u64) -> (usize, usize) {
	debug_assert!(size != 0);
	if size <= 256 {
		(0, ((size - 1) / 8) as usize)
	} else {
		let first = (63 - size.leading_zeros()) as usize - 7;
		(first, ((size >> (first + 2)) ^ 32) as usize)
	}
}

fn list(size: u64) -> usize {
	let (first, second) = class(size);
	first * 32 + second
}

impl<'callbacks> Tlsf<'callbacks> {
	pub(super) fn is_empty(&self) -> bool {
		self.allocations == 0
	}

	pub(super) fn new(size: u64) -> Result<Self, AllocError> {
		Self::new_with_host(size, HostMemory::default())
	}

	pub(super) fn host_memory(&self) -> HostMemory<'callbacks> {
		self.nodes.host_memory()
	}

	pub(super) fn new_with_host(size: u64, host: HostMemory<'callbacks>) -> Result<Self, AllocError> {
		if size == 0 {
			return Err(AllocError::InvalidRequest(
				"virtual block size must be nonzero",
			));
		}
		let mut heads = HostVec::new(host);
		heads
			.try_reserve_exact(list(size) + 1)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		heads.resize_reserved(list(size) + 1, NONE);
		let mut nodes = HostVec::new(host);
		nodes
			.try_reserve_exact(1)
			.map_err(|_| AllocError::HostAllocationFailed)?;
		let mut tail = Node::new(0, size, NONE, NONE);
		tail.state = State::Free;
		nodes.push_reserved(tail);
		Ok(Self {
			granularity: Granularity(Some(1)),
			size,
			nodes,
			head: 0,
			tail: 0,
			recycled: NONE,
			recycled_count: 0,
			heads,
			first_map: 0,
			second_maps: [0; CLASSES],
			free_count: 0,
			free_bytes: 0,
			allocations: 0,
			next_serial: 1,
		})
	}

	fn find_free(&self, size: u64) -> Option<(usize, usize)> {
		let (mut first, second) = class(size);
		let mut inner = self.second_maps[first] & (u32::MAX << second);
		if inner == 0 {
			let higher = self.first_map & (u64::MAX << (first + 1));
			if higher == 0 {
				return None;
			}
			first = higher.trailing_zeros() as usize;
			inner = self.second_maps[first];
		}
		let index = first * 32 + inner.trailing_zeros() as usize;
		Some((self.heads[index], index))
	}

	fn fits(&self, index: usize, size: u64, alignment: u64, class: ResourceClass) -> Option<u64> {
		let node = &self.nodes[index];
		let end = node.offset + node.size;
		let mut offset = node.offset.checked_add(alignment - 1)? & !(alignment - 1);
		if offset > end || size > end - offset {
			return None;
		}
		if node.previous != NONE {
			let previous = &self.nodes[node.previous];
			debug_assert!(previous.state == State::Taken);
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
		if node.next != NONE {
			let next = &self.nodes[node.next];
			debug_assert!(next.state == State::Taken);
			if self
				.granularity
				.conflict(offset, size, class, next.offset, next.class)
			{
				return None;
			}
		}
		Some(offset)
	}

	fn search_list(
		&self,
		node: Option<(usize, usize)>,
		size: u64,
		alignment: u64,
		class: ResourceClass,
	) -> Option<(usize, u64)> {
		let (mut index, _) = node?;
		while index != NONE {
			if let Some(offset) = self.fits(index, size, alignment, class) {
				return Some((index, offset));
			}
			index = self.nodes[index].next_free;
		}
		None
	}

	fn choose(
		&self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
		class: ResourceClass,
	) -> Option<(usize, u64)> {
		let tail = || {
			self
				.fits(self.tail, size, alignment, class)
				.map(|offset| (self.tail, offset))
		};
		if self.free_count == 0 {
			return tail();
		}
		if strategy == PlacementStrategy::MinOffset {
			// Donor walks backwards then materializes ascending candidates. Walking
			// the same physical sequence forwards preserves order without a temporary Vec.
			let mut index = self.head;
			while index != self.tail {
				if self.nodes[index].state == State::Free
					&& let Some(offset) = self.fits(index, size, alignment, class)
				{
					return Some((index, offset));
				}
				index = self.nodes[index].next;
			}
			return tail();
		}
		let next_size = if size > 256 {
			size.checked_add(1 << (63 - size.leading_zeros() - 5))
		} else if size > 248 {
			Some(257)
		} else {
			Some(size + 8)
		};
		let next = next_size.and_then(|size| self.find_free(size));
		let previous = self.find_free(size);
		let chosen = match strategy {
			PlacementStrategy::MinTime => {
				let head = next.and_then(|(index, _)| {
					self
						.fits(index, size, alignment, class)
						.map(|offset| (index, offset))
				});
				head
					.or_else(tail)
					.or_else(|| self.search_list(next, size, alignment, class))
					.or_else(|| self.search_list(previous, size, alignment, class))
			}
			PlacementStrategy::MinMemory => self
				.search_list(previous, size, alignment, class)
				.or_else(tail)
				.or_else(|| self.search_list(next, size, alignment, class)),
			PlacementStrategy::Default => self
				.search_list(next, size, alignment, class)
				.or_else(tail)
				.or_else(|| self.search_list(previous, size, alignment, class)),
			PlacementStrategy::MinOffset => unreachable!(),
		};
		if chosen.is_some() {
			return chosen;
		}
		// Preserve donor fallback order, including its bounded bucket-list scan.
		let start = next.map_or(self.heads.len(), |(_, index)| index + 1);
		for bucket in start..self.heads.len() {
			if let Some(result) =
				self.search_list(Some((self.heads[bucket], bucket)), size, alignment, class)
			{
				return Some(result);
			}
		}
		None
	}

	pub(super) fn allocate(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
	) -> Result<(usize, u64), AllocError> {
		self.allocate_resource(size, alignment, strategy, ResourceClass::Unassigned)
	}

	pub(super) fn allocate_resource(
		&mut self,
		size: u64,
		alignment: u64,
		strategy: PlacementStrategy,
		class: ResourceClass,
	) -> Result<(usize, u64), AllocError> {
		if size == 0 || !alignment.is_power_of_two() {
			return Err(AllocError::InvalidRequest(
				"size must be nonzero and alignment a power of two",
			));
		}
		let next_serial = self
			.next_serial
			.checked_add(1)
			.ok_or(AllocError::SerialExhausted)?;
		if size > self.free_bytes + self.nodes[self.tail].size {
			return Err(AllocError::OutOfSpace);
		}
		let (current, offset) = self
			.choose(size, alignment, strategy, class)
			.ok_or(AllocError::OutOfSpace)?;
		// At most two new nodes: alignment prefix plus suffix (including empty tail).
		// Search has no mutations; OOM leaves placement and free-list tie order intact.
		self
			.nodes
			.try_reserve(2usize.saturating_sub(self.recycled_count))
			.map_err(|_| AllocError::HostAllocationFailed)?;
		if current != self.tail {
			self.remove_free(current);
		}
		let padding = offset - self.nodes[current].offset;
		if padding != 0 {
			let previous = self.nodes[current].previous;
			debug_assert!(previous != NONE);
			if self.nodes[previous].state == State::Free {
				let old_list = list(self.nodes[previous].size);
				let new_size = self.nodes[previous].size + padding;
				if old_list != list(new_size) {
					self.remove_free(previous);
					self.nodes[previous].size = new_size;
					self.insert_free(previous);
				} else {
					self.nodes[previous].size = new_size;
					self.free_bytes += padding;
				}
			} else {
				let prefix = self.node(Node::new(
					self.nodes[current].offset,
					padding,
					previous,
					current,
				));
				self.nodes[previous].next = prefix;
				self.nodes[current].previous = prefix;
				self.insert_free(prefix);
			}
			self.nodes[current].offset = offset;
			self.nodes[current].size -= padding;
		}
		let remaining = self.nodes[current].size - size;
		if remaining != 0 || current == self.tail {
			let next = self.nodes[current].next;
			let suffix = self.node(Node::new(offset + size, remaining, current, next));
			self.nodes[current].next = suffix;
			self.nodes[current].size = size;
			if current == self.tail {
				self.tail = suffix;
				self.nodes[suffix].state = State::Free;
			} else {
				self.nodes[next].previous = suffix;
				self.insert_free(suffix);
			}
		}
		self.nodes[current].state = State::Taken;
		self.nodes[current].serial = self.next_serial;
		self.nodes[current].user_data = 0;
		self.nodes[current].class = class;
		let serial = self.next_serial;
		self.next_serial = next_serial;
		self.allocations += 1;
		Ok((current, serial))
	}

	fn node(&mut self, node: Node) -> usize {
		if self.recycled == NONE {
			let index = self.nodes.len();
			self.nodes.push_reserved(node);
			index
		} else {
			let index = self.recycled;
			self.recycled = self.nodes[index].next_free;
			self.recycled_count -= 1;
			self.nodes[index] = node;
			index
		}
	}

	fn recycle(&mut self, index: usize) {
		self.nodes[index].state = State::Unused;
		self.nodes[index].next_free = self.recycled;
		self.recycled = index;
		self.recycled_count += 1;
	}

	fn insert_free(&mut self, index: usize) {
		let node = &self.nodes[index];
		let (first, second) = class(node.size);
		let bucket = first * 32 + second;
		let head = self.heads[bucket];
		self.nodes[index].previous_free = NONE;
		self.nodes[index].next_free = head;
		self.nodes[index].state = State::Free;
		if head != NONE {
			self.nodes[head].previous_free = index;
		}
		self.heads[bucket] = index;
		self.first_map |= 1u64 << first;
		self.second_maps[first] |= 1u32 << second;
		self.free_count += 1;
		self.free_bytes += self.nodes[index].size;
	}

	fn remove_free(&mut self, index: usize) {
		let node = &self.nodes[index];
		let (previous, next, size) = (node.previous_free, node.next_free, node.size);
		if next != NONE {
			self.nodes[next].previous_free = previous;
		}
		if previous != NONE {
			self.nodes[previous].next_free = next;
		} else {
			let (first, second) = class(size);
			self.heads[first * 32 + second] = next;
			if next == NONE {
				self.second_maps[first] &= !(1u32 << second);
				if self.second_maps[first] == 0 {
					self.first_map &= !(1u64 << first);
				}
			}
		}
		self.nodes[index].state = State::Taken;
		self.nodes[index].previous_free = NONE;
		self.nodes[index].next_free = NONE;
		self.free_count -= 1;
		self.free_bytes -= size;
	}

	fn merge(&mut self, current: usize, previous: usize) {
		let node = &self.nodes[previous];
		let (offset, size, before) = (node.offset, node.size, node.previous);
		self.nodes[current].offset = offset;
		self.nodes[current].size += size;
		self.nodes[current].previous = before;
		if before == NONE {
			self.head = current;
		} else {
			self.nodes[before].next = current;
		}
		self.recycle(previous);
	}

	pub(super) fn info(
		&self,
		index: usize,
		serial: u64,
	) -> Result<VirtualAllocationInfo, AllocError> {
		let node = self
			.nodes
			.get(index)
			.filter(|node| node.state == State::Taken && node.serial == serial)
			.ok_or(AllocError::UnknownAllocation)?;
		Ok(VirtualAllocationInfo {
			offset: node.offset,
			size: node.size,
			user_data: node.user_data,
		})
	}

	pub(super) fn free(&mut self, index: usize, serial: u64) -> Result<(), AllocError> {
		self.info(index, serial)?;
		let previous = self.nodes[index].previous;
		let next = self.nodes[index].next;
		self.allocations -= 1;
		if previous != NONE && self.nodes[previous].state == State::Free {
			self.remove_free(previous);
			self.merge(index, previous);
		}
		if self.nodes[next].state != State::Free {
			self.insert_free(index);
		} else if next == self.tail {
			self.merge(next, index);
		} else {
			self.remove_free(next);
			self.merge(next, index);
			self.insert_free(next);
		}
		Ok(())
	}

	pub(super) fn clear(&mut self) {
		self.nodes.clear();
		let mut tail = Node::new(0, self.size, NONE, NONE);
		tail.state = State::Free;
		self.nodes.push_reserved(tail);
		self.head = 0;
		self.tail = 0;
		self.recycled = NONE;
		self.recycled_count = 0;
		self.heads.fill(NONE);
		self.first_map = 0;
		self.second_maps.fill(0);
		self.free_count = 0;
		self.free_bytes = 0;
		self.allocations = 0;
		// Preserve the monotonic generation sequence across clear.
	}

	pub(super) fn set_user_data(
		&mut self,
		index: usize,
		serial: u64,
		data: usize,
	) -> Result<(), AllocError> {
		self.info(index, serial)?;
		self.nodes[index].user_data = data;
		Ok(())
	}

	pub(super) fn for_each_allocation(&self, mut visit: impl FnMut(u64, u64, usize)) {
		for node in &self.nodes {
			if node.state == State::Taken && node.serial != 0 {
				visit(node.offset, node.size, node.user_data);
			}
		}
	}

	pub(super) fn for_each_free(&self, mut visit: impl FnMut(u64, u64)) {
		for node in &self.nodes {
			if node.state == State::Free && node.size != 0 {
				visit(node.offset, node.size);
			}
		}
		let tail = &self.nodes[self.tail];
		if tail.size != 0 {
			visit(tail.offset, tail.size);
		}
	}

	pub(super) fn free_bytes(&self) -> u64 {
		self.free_bytes + self.nodes[self.tail].size
	}

	pub(super) fn statistics(&self) -> VirtualBlockStats {
		let mut largest = self.nodes[self.tail].size;
		for node in &self.nodes {
			if node.state == State::Free {
				largest = largest.max(node.size);
			}
		}
		let free_bytes = self.free_bytes();
		VirtualBlockStats {
			size: self.size,
			allocation_count: self.allocations,
			allocated_bytes: self.size - free_bytes,
			free_bytes,
			free_region_count: self.free_count + usize::from(self.nodes[self.tail].size != 0),
			largest_free_region: largest,
		}
	}
}

#[cfg(test)]
#[path = "../../../test/rs/mem/test_tlsf.rs"]
mod tests;
