//! Independent operation-order oracle for the benchmark scheduler itself.
use super::*;
use std::collections::BTreeMap;

#[derive(Default)]
struct Ledger {
	next: usize,
	live: BTreeMap<usize, (u64, Option<u64>)>,
	events: Vec<(char, usize)>,
}

impl Workload for Ledger {
	type Token = usize;
	fn allocate(&mut self, size: u64) -> Result<usize> {
		let id = self.next;
		self.next += 1;
		assert!(self.live.insert(id, (size, None)).is_none());
		self.events.push(('a', id));
		Ok(id)
	}
	fn free(&mut self, id: usize) -> Result<()> {
		assert!(self.live.remove(&id).is_some());
		self.events.push(('f', id));
		Ok(())
	}
	fn inspect(&self, tokens: &[usize]) -> Result<Value> {
		assert_eq!(tokens.len(), self.live.len());
		assert!(tokens.iter().all(|id| self.live.contains_key(id)));
		Ok(
			json!({"live_count": tokens.len(), "allocation_bytes": self.live.values().map(|(size, _)| size).sum::<u64>()}),
		)
	}
	fn empty(&self) -> Result<()> {
		assert!(self.live.is_empty());
		Ok(())
	}
	fn map_write(&mut self, id: &usize, pattern: u64) -> Result<()> {
		self.live.get_mut(id).expect("live mapping").1 = Some(pattern);
		self.events.push(('m', *id));
		Ok(())
	}
	fn map_check(&mut self, id: &usize, pattern: u64) -> Result<()> {
		assert_eq!(self.live[id].1, Some(pattern));
		Ok(())
	}
}

fn args(trace: &str) -> Args {
	Args {
		case: "virtual".into(),
		device: 0,
		api: vk::API_VERSION_1_4,
		batches: 1,
		batch_size: 8,
		memory_type: 0,
		trace: trace.into(),
		algorithm: "tlsf".into(),
		strategy: "default".into(),
		mapping: "none".into(),
	}
}

#[test]
fn fifo_and_reverse_have_exact_observable_retirement_order() {
	for (trace, expected) in [
		("reverse", vec![7, 6, 5, 4, 3, 2, 1, 0]),
		("fifo", vec![0, 1, 2, 3, 4, 5, 6, 7]),
	] {
		let mut ledger = Ledger::default();
		let result = batches(&args(trace), &mut ledger).unwrap();
		let freed: Vec<_> = ledger
			.events
			.iter()
			.filter_map(|&(kind, id)| (kind == 'f').then_some(id))
			.collect();
		assert_eq!(freed, expected);
		assert_eq!(result["allocations"], 8);
		assert_eq!(result["requested_byte_checksum"], 90752);
	}
}

#[test]
fn churn_preserves_even_survivors_replaces_odd_holes_and_maps_only_live_regions() {
	let mut ledger = Ledger::default();
	let mut args = args("churn");
	args.mapping = "cycle".into();
	let result = batches(&args, &mut ledger).unwrap();
	let freed: Vec<_> = ledger
		.events
		.iter()
		.filter_map(|&(kind, id)| (kind == 'f').then_some(id))
		.collect();
	assert_eq!(freed, [7, 5, 3, 1, 11, 10, 9, 8, 6, 4, 2, 0]);
	assert_eq!(
		&ledger.events[8..16],
		[
			('f', 7),
			('f', 5),
			('f', 3),
			('f', 1),
			('a', 8),
			('a', 9),
			('a', 10),
			('a', 11)
		]
	);
	let mapped: Vec<_> = ledger
		.events
		.iter()
		.filter_map(|&(kind, id)| (kind == 'm').then_some(id))
		.collect();
	assert_eq!(mapped, [0, 2, 4, 6, 8, 9, 10, 11]);
	assert_eq!(result["allocations"], 12);
	assert_eq!(result["map_cycles"], 8);
	assert_eq!(result["requested_byte_checksum"], 111552);
	assert_eq!(result["peak"], result["fragmented_peak"]);
}

#[test]
fn shuffled_retirement_is_a_deterministic_permutation() {
	let run = || {
		let mut ledger = Ledger::default();
		batches(&args("shuffle"), &mut ledger).unwrap();
		ledger
			.events
			.into_iter()
			.filter_map(|(kind, id)| (kind == 'f').then_some(id))
			.collect::<Vec<_>>()
	};
	let first = run();
	assert_eq!(first, run());
	assert_ne!(first, (0..8).collect::<Vec<_>>());
	let mut sorted = first;
	sorted.sort_unstable();
	assert_eq!(sorted, (0..8).collect::<Vec<_>>());
}
