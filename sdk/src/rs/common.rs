//! Shared workload and timing boundaries; no native implementation lives here.
use serde_json::{Value, json};
use std::{error::Error, hint::black_box, time::Instant};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub const BLOCK_SIZE: u64 = 64 * 1024 * 1024;
pub const SIZES: [u64; 5] = [64, 256, 4096, 16384, 65536];

pub struct Args {
	pub case: String,
	pub device: usize,
	pub api: u32,
	pub batches: usize,
	pub batch_size: usize,
	pub memory_type: u32,
	pub trace: String,
	pub algorithm: String,
	pub strategy: String,
	pub mapping: String,
}

impl Args {
	pub fn new() -> Result<Self> {
		let mut args = Self {
			case: "probe".into(),
			device: 0,
			api: vk::API_VERSION_1_2,
			batches: 32,
			batch_size: 128,
			memory_type: 0,
			trace: "reverse".into(),
			algorithm: "tlsf".into(),
			strategy: "default".into(),
			mapping: "none".into(),
		};
		let mut input = std::env::args().skip(1);
		while let Some(key) = input.next() {
			let value = input.next().ok_or("missing argument value")?;
			match key.as_str() {
				"--case" => args.case = value,
				"--device" => args.device = value.parse()?,
				"--api" => {
					let minor: u32 = value
						.strip_prefix("1.")
						.ok_or("API must be 1.1..1.4")?
						.parse()?;
					if !(1..=4).contains(&minor) {
						return Err("API must be 1.1..1.4".into());
					}
					args.api = vk::make_api_version(0, 1, minor, 0);
				}
				"--batches" => args.batches = value.parse()?,
				"--batch-size" => args.batch_size = value.parse()?,
				"--memory-type" => args.memory_type = value.parse()?,
				"--trace" => args.trace = value,
				"--algorithm" => args.algorithm = value,
				"--strategy" => args.strategy = value,
				"--mapping" => args.mapping = value,
				_ => return Err(format!("unknown argument {key}").into()),
			}
		}
		if !(1..=4096).contains(&args.batches)
			|| !(1..=256).contains(&args.batch_size)
			|| args.memory_type >= 32
		{
			return Err("batches=1..4096, batch-size=1..256, memory-type=0..31 required".into());
		}
		if ![
			"probe",
			"startup",
			"query",
			"buffer-pool",
			"buffer-separate",
			"image-pool",
			"image-separate",
			"virtual",
		]
		.contains(&args.case.as_str())
		{
			return Err("unknown workload".into());
		}
		if !["reverse", "fifo", "shuffle", "churn"].contains(&args.trace.as_str())
			|| !["tlsf", "linear"].contains(&args.algorithm.as_str())
			|| !["default", "min-memory", "min-time", "min-offset"].contains(&args.strategy.as_str())
			|| !["none", "cycle"].contains(&args.mapping.as_str())
		{
			return Err("invalid allocation trace/algorithm/strategy/mapping policy".into());
		}
		let allocation_case =
			args.case == "virtual" || args.case.starts_with("buffer-") || args.case.starts_with("image-");
		if (!allocation_case
			&& (args.trace != "reverse"
				|| args.algorithm != "tlsf"
				|| args.strategy != "default"
				|| args.mapping != "none"))
			|| (args.case.ends_with("separate")
				&& (args.algorithm != "tlsf" || args.strategy != "default"))
			|| (args.case == "virtual" && args.mapping != "none")
			|| (args.trace == "churn" && args.batch_size < 2)
		{
			return Err("allocation policy does not apply to this workload".into());
		}
		Ok(args)
	}
}

pub trait Workload {
	type Token;
	fn allocate(&mut self, size: u64) -> Result<Self::Token>;
	fn free(&mut self, token: Self::Token) -> Result<()>;
	fn inspect(&self, tokens: &[Self::Token]) -> Result<Value>;
	fn empty(&self) -> Result<()>;
	fn map_write(&mut self, _token: &Self::Token, _pattern: u64) -> Result<()> {
		Err("mapping workload is unsupported".into())
	}
	fn map_check(&mut self, _token: &Self::Token, _pattern: u64) -> Result<()> {
		Err("mapping workload is unsupported".into())
	}
}

// Build the eviction permutation outside timing. The C++ reference uses the
// same xorshift64 schedule; the Python runner independently checks its checksum.
pub fn trace_swaps(count: usize, trace: &str) -> (Vec<(usize, usize)>, u64) {
	let mut order: Vec<_> = (0..count).collect();
	let mut swaps = Vec::new();
	if trace == "fifo" {
		for left in 0..count / 2 {
			let right = count - 1 - left;
			swaps.push((left, right));
			order.swap(left, right);
		}
	} else if trace == "shuffle" {
		let mut random = 0x6a09_e667_f3bc_c909u64;
		for left in (1..count).rev() {
			random ^= random << 13;
			random ^= random >> 7;
			random ^= random << 17;
			let right = (random % (left as u64 + 1)) as usize;
			swaps.push((left, right));
			order.swap(left, right);
		}
	} else if trace == "churn" {
		let desired: Vec<_> = (0..count).step_by(2).chain((1..count).step_by(2)).collect();
		for (left, source) in desired.into_iter().enumerate() {
			let right = order
				.iter()
				.position(|&index| index == source)
				.expect("permutation");
			swaps.push((left, right));
			order.swap(left, right);
		}
	}
	let checksum = order
		.iter()
		.rev()
		.fold(0xcbf2_9ce4_8422_2325u64, |hash, &index| {
			(hash ^ index as u64).wrapping_mul(0x100_0000_01b3)
		});
	(swaps, checksum)
}

/// Time only API operations and pre-reserved token-container overhead.
/// Inspections, JSON, pattern generation and correctness assertions are untimed.
pub fn batches<W: Workload>(args: &Args, workload: &mut W) -> Result<Value> {
	let mut tokens = Vec::with_capacity(args.batch_size);
	let mut allocate_ns = 0u128;
	let mut free_ns = 0u128;
	let mut peak = Value::Null;
	let mut checksum = 0u64;
	let mut map_ns = 0u128;
	let (swaps, trace_checksum) = trace_swaps(args.batch_size, &args.trace);
	let replacements = if args.trace == "churn" {
		args.batch_size / 2
	} else {
		0
	};
	let mut fragmented_peak = Value::Null;
	for batch in 0..args.batches {
		let start = Instant::now();
		for index in 0..args.batch_size {
			tokens.push(workload.allocate(black_box(SIZES[index % SIZES.len()]))?);
		}
		allocate_ns += start.elapsed().as_nanos();
		if batch == 0 {
			peak = workload.inspect(&tokens)?;
		}
		checksum = checksum.wrapping_add(
			(0..args.batch_size)
				.map(|i| SIZES[i % SIZES.len()])
				.sum::<u64>(),
		);
		for &(left, right) in &swaps {
			tokens.swap(left, right);
		}
		if replacements != 0 {
			let start = Instant::now();
			for _ in 0..replacements {
				workload.free(black_box(tokens.pop().expect("selected live region")))?;
			}
			free_ns += start.elapsed().as_nanos();
			let start = Instant::now();
			for slot in (0..replacements).rev() {
				tokens.push(workload.allocate(black_box(SIZES[(slot * 2 + 1) % SIZES.len()]))?);
			}
			allocate_ns += start.elapsed().as_nanos();
			checksum += (0..replacements)
				.map(|slot| SIZES[(slot * 2 + 1) % SIZES.len()])
				.sum::<u64>();
			if batch == 0 {
				fragmented_peak = workload.inspect(&tokens)?;
			}
		}
		if args.mapping == "cycle" {
			let start = Instant::now();
			for (index, token) in tokens.iter().enumerate() {
				workload.map_write(token, index as u64 ^ 0x9e37_79b9_7f4a_7c15)?;
			}
			map_ns += start.elapsed().as_nanos();
			// Independent remapping and payload verification are outside timing.
			for (index, token) in tokens.iter().enumerate() {
				workload.map_check(token, index as u64 ^ 0x9e37_79b9_7f4a_7c15)?;
			}
		}
		let start = Instant::now();
		while let Some(token) = tokens.pop() {
			workload.free(black_box(token))?;
		}
		free_ns += start.elapsed().as_nanos();
		workload.empty()?;
	}
	Ok(
		json!({"allocations": args.batches * (args.batch_size + replacements), "allocate_ns": allocate_ns,
		"free_ns": free_ns, "requested_byte_checksum": checksum, "peak": peak,
		"fragmented_peak": fragmented_peak, "trace_checksum": trace_checksum,
		"map_ns": map_ns, "map_cycles": if args.mapping == "cycle" { args.batches * args.batch_size } else { 0 }}),
	)
}

pub fn emit(backend: &str, args: &Args, identity: Value, measurements: Value) {
	println!(
		"{}",
		json!({"schema": "vk.sdk.sample.v2", "backend": backend, "case": args.case,
		"comparison_scope": if backend == "vulkano" { "checked-managed-api" } else { "native-public-api" },
		"api_requested": args.api, "device_index": args.device, "batches": args.batches,
		"batch_size": args.batch_size, "memory_type": args.memory_type,
		"block_size": BLOCK_SIZE, "sizes": SIZES, "identity": identity, "measurements": measurements,
		"trace": args.trace, "algorithm": if backend == "vulkano" { "free-list" } else { &args.algorithm },
		"strategy": args.strategy, "mapping": args.mapping})
	);
}

#[cfg(test)]
#[path = "../../test/rs/test_trace.rs"]
mod tests;
