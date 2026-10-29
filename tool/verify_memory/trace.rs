//! Development-only public-API trace consumer; no algorithm implementation here.
use std::{
	collections::BTreeMap,
	io::{self, BufRead},
};
use vk::{
	VirtualAllocation, VirtualBlock,
	mem::{AllocationAlgorithm, PlacementStrategy},
};

fn main() {
	let mut block = None;
	let mut live: BTreeMap<u64, VirtualAllocation> = BTreeMap::new();
	for line in io::stdin().lock().lines() {
		let line = line.unwrap();
		let words: Vec<_> = line.split_whitespace().collect();
		let number = |i: usize| words[i].parse::<u64>().unwrap();
		let prefix = match words[0] {
			"b" => {
				assert!(block.is_none());
				block = Some(
					VirtualBlock::new_with_algorithm(
						number(1),
						if words.get(2) == Some(&"1") {
							AllocationAlgorithm::Linear
						} else {
							AllocationAlgorithm::Tlsf
						},
					)
					.unwrap(),
				);
				format!("b {}", number(1))
			}
			"a" => {
				let (id, size, alignment) = (number(1), number(2), number(3));
				let strategy = match number(4) {
					0 => PlacementStrategy::Default,
					1 => PlacementStrategy::MinMemory,
					2 => PlacementStrategy::MinTime,
					3 => PlacementStrategy::MinOffset,
					_ => panic!("invalid strategy"),
				};
				let result = if words.get(5) == Some(&"1") {
					block.as_mut().unwrap().allocate_upper(size, alignment)
				} else {
					block.as_mut().unwrap().allocate(size, alignment, strategy)
				};
				match result {
					Ok(allocation) => {
						let info = block
							.as_ref()
							.unwrap()
							.allocation_info(&allocation)
							.unwrap();
						live.insert(id, allocation);
						format!("a {id} 1 {} {size}", info.offset)
					}
					Err(vk::AllocError::OutOfSpace) => format!("a {id} 0 0 {size}"),
					Err(error) => panic!("unexpected trace failure: {error}"),
				}
			}
			"f" => {
				let id = number(1);
				if let Some(mut allocation) = live.remove(&id) {
					block.as_mut().unwrap().free(&mut allocation).unwrap();
				}
				format!("f {id}")
			}
			"c" => {
				block.as_mut().unwrap().clear();
				live.clear();
				"c".into()
			}
			_ => panic!("invalid operation"),
		};
		let s = block.as_ref().unwrap().statistics();
		println!(
			"{prefix} {} {} {} {} {}",
			s.allocation_count,
			s.allocated_bytes,
			s.free_bytes,
			s.free_region_count,
			s.largest_free_region
		);
	}
}
