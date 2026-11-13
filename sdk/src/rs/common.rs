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
			"virtual",
		]
		.contains(&args.case.as_str())
		{
			return Err("unknown workload".into());
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
}

/// Time only API operations and pre-reserved token-container overhead.
/// Inspections, JSON, pattern generation and correctness assertions are untimed.
pub fn batches<W: Workload>(args: &Args, workload: &mut W) -> Result<Value> {
	let mut tokens = Vec::with_capacity(args.batch_size);
	let mut allocate_ns = 0u128;
	let mut free_ns = 0u128;
	let mut peak = Value::Null;
	let mut checksum = 0u64;
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
		let start = Instant::now();
		while let Some(token) = tokens.pop() {
			workload.free(black_box(token))?;
		}
		free_ns += start.elapsed().as_nanos();
		workload.empty()?;
	}
	Ok(
		json!({"allocations": args.batches * args.batch_size, "allocate_ns": allocate_ns,
		"free_ns": free_ns, "requested_byte_checksum": checksum, "peak": peak}),
	)
}

pub fn emit(backend: &str, args: &Args, identity: Value, measurements: Value) {
	println!(
		"{}",
		json!({"schema": "vk.sdk.sample.v1", "backend": backend, "case": args.case,
		"comparison_scope": if backend == "vulkano" { "checked-managed-api" } else { "native-public-api" },
		"api_requested": args.api, "device_index": args.device, "batches": args.batches,
		"batch_size": args.batch_size, "memory_type": args.memory_type,
		"block_size": BLOCK_SIZE, "sizes": SIZES, "identity": identity, "measurements": measurements})
	);
}
