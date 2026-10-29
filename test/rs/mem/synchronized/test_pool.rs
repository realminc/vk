//! Independent-pool overlap, in-flight budget reservations and lossless restoration.
use super::*;
use crate::mem::{Pool, PoolCreateInfo};
use std::{
	sync::{Arc, Condvar, PoisonError, atomic::AtomicBool},
	time::Duration,
};

struct GateState {
	entered: usize,
	expected: usize,
	released: bool,
	timed_out: bool,
}

struct Gate(Mutex<GateState>, Condvar);

static GATE: Mutex<Option<Arc<Gate>>> = Mutex::new(None);
static BIND_TARGET: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static BIND_GATE: Mutex<Option<Arc<Gate>>> = Mutex::new(None);
static FAIL_MAP: AtomicBool = AtomicBool::new(false);
static FAIL_BIND: AtomicBool = AtomicBool::new(false);
static MAP_GATE: Mutex<Option<Arc<Gate>>> = Mutex::new(None);
pub(super) static REQ_BITS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

static FAIL: AtomicBool = AtomicBool::new(false);

static RECYCLE: AtomicUsize = AtomicUsize::new(0);
static REUSE_GATE: Mutex<Option<(u64, Arc<Gate>)>> = Mutex::new(None);

pub(super) fn reset_native_hooks() {
	*GATE.lock().unwrap() = None;
	*MAP_GATE.lock().unwrap() = None;
	*BIND_GATE.lock().unwrap() = None;
	BIND_TARGET.store(0, Ordering::SeqCst);
	FAIL_MAP.store(false, Ordering::SeqCst);
	FAIL_BIND.store(false, Ordering::SeqCst);
	REQ_BITS.store(1, Ordering::SeqCst);
	*REUSE_GATE.lock().unwrap() = None;
	RECYCLE.store(0, Ordering::SeqCst);
	FAIL.store(false, Ordering::SeqCst);
}

pub(super) fn recycled_buffer() -> Option<u64> {
	let raw = RECYCLE.swap(0, Ordering::SeqCst);
	(raw != 0).then_some(raw as u64)
}

pub(super) fn after_buffer_destroyed(buffer: vk::Buffer) {
	let selected = REUSE_GATE.lock().unwrap().clone();
	let Some((target, gate)) = selected.filter(|(target, _)| *target == buffer.as_raw()) else {
		return;
	};
	RECYCLE.store(usize::try_from(target).unwrap(), Ordering::SeqCst);
	let mut state = gate.0.lock().unwrap();
	state.entered += 1;
	gate.1.notify_all();
	let (mut state, timeout) = gate
		.1
		.wait_timeout_while(state, Duration::from_secs(10), |state| !state.released)
		.unwrap();
	if timeout.timed_out() {
		state.timed_out = true;
		state.released = true;
		gate.1.notify_all();
	}
}

impl Gate {
	fn new(expected: usize) -> Arc<Self> {
		let gate = Arc::new(Self(
			Mutex::new(GateState {
				entered: 0,
				expected,
				released: false,
				timed_out: false,
			}),
			Condvar::new(),
		));
		*GATE.lock().unwrap() = Some(Arc::clone(&gate));
		gate
	}

	fn wait_for_entry(&self) {
		let state = self.0.lock().unwrap();
		let (state, timeout) = self
			.1
			.wait_timeout_while(state, Duration::from_secs(10), |state| state.entered == 0)
			.unwrap();
		assert!(
			!timeout.timed_out() && state.entered != 0,
			"native allocation did not start"
		);
	}

	fn release(&self) {
		self.0.lock().unwrap().released = true;
		self.1.notify_all();
	}
}

pub(super) fn allocation_gate() -> bool {
	if FAIL.swap(false, Ordering::SeqCst) {
		return false;
	}
	let gate = GATE.lock().unwrap().clone();
	let Some(gate) = gate else {
		return true;
	};
	wait_gate(&gate)
}

pub(super) fn mapping_gate() -> bool {
	if FAIL_MAP.swap(false, Ordering::SeqCst) {
		return false;
	}
	let gate = MAP_GATE.lock().unwrap().clone();
	gate.as_ref().is_none_or(|gate| wait_gate(gate))
}

pub(super) fn binding_gate(memory: u64) -> bool {
	let target = BIND_TARGET.load(Ordering::SeqCst);
	if target != 0 && target != memory {
		return true;
	}
	if FAIL_BIND.swap(false, Ordering::SeqCst) {
		return false;
	}
	let gate = BIND_GATE.lock().unwrap().clone();
	gate.as_ref().is_none_or(|gate| wait_gate(gate))
}

fn wait_gate(gate: &Gate) -> bool {
	let mut state = gate.0.lock().unwrap();
	state.entered += 1;
	if state.entered >= state.expected {
		state.released = true;
	}
	gate.1.notify_all();
	let (mut state, timeout) = gate
		.1
		.wait_timeout_while(state, Duration::from_secs(10), |state| !state.released)
		.unwrap();
	if timeout.timed_out() {
		state.timed_out = true;
		state.released = true;
		gate.1.notify_all();
	}
	!state.timed_out
}

fn properties_coherent() -> vk::PhysicalDeviceMemoryProperties {
	let mut result = properties();
	result.memory_types[0].property_flags =
		result.memory_types[0].property_flags | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	result
}

fn reset() -> std::sync::MutexGuard<'static, ()> {
	let guard = TEST_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
	*NATIVE.lock().unwrap() = Native {
		next: 1,
		..Default::default()
	};
	reset_native_hooks();
	OVERLAP.store(0, Ordering::SeqCst);
	guard
}

unsafe fn new_pool(allocator: &mut Allocator<'_>) -> Pool {
	// SAFETY: exact fixed ordinary fixture type, no GPU work or external chain.
	unsafe {
		allocator.create_pool(&PoolCreateInfo {
			block_size: 4096,
			..Default::default()
		})
	}
	.unwrap()
}

unsafe fn create(
	allocator: &mut Allocator<'_>,
	pool: &Pool,
) -> Result<(vk::Buffer, Allocation), AllocError> {
	// SAFETY: fixture-backed ordinary resource with independent explicit lifetime.
	unsafe {
		allocator.create_buffer_with_options(
			&vk::BufferCreateInfo {
				size: 64,
				usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
				..Default::default()
			},
			&AllocationCreateInfo {
				pool: Some(pool),
				..Default::default()
			},
		)
	}
}

fn empty_native() {
	let state = NATIVE.lock().unwrap();
	assert!(state.memory.is_empty() && state.buffers.is_empty() && state.types.is_empty());
	assert_eq!(state.allocations, state.frees);
	assert_eq!(state.maps, state.unmaps);
}

#[test]
fn independent_pools_overlap_and_restore_tokens_without_host_allocation() {
	let _test = reset();
	let (_instance, device) = contexts();
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	// SAFETY: fixed native backing and concurrent callback ledger outlive all workers.
	unsafe {
		let mut allocator =
			Allocator::new_with_callbacks(&device, properties_coherent(), 64, &callbacks).unwrap();
		let mut first = new_pool(&mut allocator);
		let mut second = new_pool(&mut allocator);
		let mut unselected = allocator
			.allocate(&AllocRequest {
				size: 64,
				..request()
			})
			.unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(2);
		let (mut results, attempts) = shared
			.with_pools(&[&first, &second], |children| {
				assert!(
					shared
						.try_with_allocator(|_| panic!("parent accessed during scope"))
						.unwrap()
						.is_none()
				);
				let results = std::thread::scope(|scope| {
					let workers: Vec<_> = children
						.iter()
						.zip([&first, &second])
						.enumerate()
						.map(|(worker, (child, pool))| {
							scope.spawn(move || {
								child
									.with_allocator(|allocator| {
										let mut values = Vec::new();
										for index in 0..80 {
											let (buffer, allocation) = create(allocator, pool).unwrap();
											allocator
												.set_allocation_name(&allocation, Some("parallel pool"))
												.unwrap();
											let pointer = allocator.map(&allocation).unwrap().cast::<u64>();
											pointer.write_unaligned(((worker as u64) << 32) | index);
											values.push((buffer, allocation));
										}
										values
									})
									.unwrap()
							})
						})
						.collect();
					workers
						.into_iter()
						.map(|worker| worker.join().unwrap())
						.collect::<Vec<_>>()
				});
				let attempts = tracker.snapshot().attempts;
				tracker.fail_at(attempts + 1);
				(results, attempts)
			})
			.unwrap();
		assert_eq!(
			tracker.snapshot().attempts,
			attempts,
			"restoration allocated CPU storage"
		);
		assert!(!gate.0.lock().unwrap().timed_out);
		assert!(
			OVERLAP.load(Ordering::SeqCst) > 0,
			"native calls never overlapped"
		);
		*GATE.lock().unwrap() = None;
		assert_eq!(shared.statistics().unwrap().allocation_count, 161);
		let mut allocator = shared.into_inner().unwrap();
		let mut serials = std::collections::BTreeSet::new();
		for (worker, values) in results.iter_mut().enumerate() {
			for (index, (buffer, allocation)) in values.iter_mut().enumerate() {
				assert!(serials.insert(allocation.serial));
				let pointer = allocator
					.mapped_pointer(allocation)
					.unwrap()
					.unwrap()
					.cast::<u64>();
				assert_eq!(
					pointer.read_unaligned(),
					((worker as u64) << 32) | index as u64
				);
				allocator.unmap(allocation).unwrap();
				allocator.destroy_buffer(*buffer, allocation).unwrap();
			}
		}
		allocator.free(&mut unselected).unwrap();
		allocator.destroy_pool(&mut first).unwrap();
		allocator.destroy_pool(&mut second).unwrap();
		allocator.close().unwrap();
	}
	assert_eq!(tracker.snapshot().live, 0);
	assert_eq!(tracker.snapshot().unexpected, 0);
	empty_native();
}

#[test]
fn in_flight_native_reservation_enforces_global_count_and_heap_limits() {
	let _test = reset();
	let (_instance, device) = contexts();
	for heap_limited in [false, true] {
		// SAFETY: exact synthetic device and joined worker ownership.
		unsafe {
			let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
			let mut first = new_pool(&mut allocator);
			let mut second = new_pool(&mut allocator);
			if heap_limited {
				allocator.set_heap_limit(0, 4096).unwrap();
			} else {
				allocator.allocation_limit = 1;
			}
			let shared = SynchronizedAllocator::new(allocator);
			let gate = Gate::new(usize::MAX);
			let (buffer, mut allocation) = shared
				.with_pools(&[&first, &second], |children| {
					std::thread::scope(|scope| {
						let worker = scope.spawn(|| {
							children[0]
								.with_allocator(|allocator| create(allocator, &first))
								.unwrap()
								.unwrap()
						});
						gate.wait_for_entry();
						let other = children[1]
							.with_allocator(|allocator| create(allocator, &second))
							.unwrap();
						gate.release();
						let result = worker.join().unwrap();
						assert!(
							matches!(other, Err(AllocError::OutOfSpace))
								|| (!heap_limited && matches!(other, Err(AllocError::AllocationLimit)))
						);
						assert_eq!(
							gate.0.lock().unwrap().entered,
							1,
							"limit bypassed by another domain"
						);
						result
					})
				})
				.unwrap();
			*GATE.lock().unwrap() = None;
			let mut allocator = shared.into_inner().unwrap();
			assert_eq!(allocator.statistics().native_bytes, 4096);
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.destroy_pool(&mut first).unwrap();
			allocator.destroy_pool(&mut second).unwrap();
			allocator.close().unwrap();
		}
		empty_native();
	}
}

#[test]
fn failed_native_allocation_refunds_shared_capacity_and_configuration_is_frozen() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: independent synchronous native fixture, no GPU use.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut first = new_pool(&mut allocator);
		let mut second = new_pool(&mut allocator);
		allocator.allocation_limit = 1;
		let shared = SynchronizedAllocator::new(allocator);
		let (buffer, mut allocation) = shared
			.with_pools(&[&first, &second], |children| {
				FAIL.store(true, Ordering::SeqCst);
				assert!(
					children[0]
						.with_allocator(|allocator| create(allocator, &first))
						.unwrap()
						.is_err()
				);
				children[1]
					.with_allocator(|allocator| {
						assert!(matches!(
							allocator.set_heap_limit(0, 1),
							Err(AllocError::InvalidRequest(_))
						));
						assert!(matches!(
							allocator.create_pool(&PoolCreateInfo::default()),
							Err(AllocError::InvalidRequest(_))
						));
						assert!(matches!(
							allocator.allocate(&request()),
							Err(AllocError::InvalidRequest(_))
						));
						assert!(matches!(
							allocator.close(),
							Err(AllocError::InvalidRequest(_))
						));
						assert!(matches!(
							allocator.create_buffer_with_options(
								&vk::BufferCreateInfo {
									size: 64,
									usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
									..Default::default()
								},
								&AllocationCreateInfo::default()
							),
							Err(AllocError::InvalidRequest(_))
						));
						assert!(matches!(
							allocator.begin_defragmentation(&crate::mem::DefragmentationInfo::default()),
							Err(AllocError::InvalidRequest(_))
						));
						create(allocator, &second).unwrap()
					})
					.unwrap()
			})
			.unwrap();
		let mut allocator = shared.into_inner().unwrap();
		assert_eq!(allocator.statistics().native_allocation_count, 1);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut first).unwrap();
		allocator.destroy_pool(&mut second).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn scope_rejects_duplicate_pools_and_propagates_caught_worker_poison() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: panic is injected only after a complete transactional native operation.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut pool = new_pool(&mut allocator);
		let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		assert!(matches!(
			shared.with_pools::<()>(&[&pool, &pool], |_| panic!("duplicate admitted")),
			Err(AllocError::InvalidRequest(_))
		));
		assert!(
			shared
				.with_pools(&[], |children| children.is_empty())
				.unwrap()
		);
		let result = shared.with_pools(&[&pool], |children| {
			std::thread::scope(|scope| {
				let worker =
					scope.spawn(|| children[0].with_allocator::<()>(|_| panic!("caught worker failure")));
				assert!(worker.join().is_err());
			});
		});
		assert_eq!(result, Err(AllocError::SynchronizationPoisoned));
		assert_eq!(
			shared.statistics(),
			Err(AllocError::SynchronizationPoisoned)
		);
		let mut allocator = shared
			.into_inner()
			.err()
			.expect("parent must retain poison")
			.into_inner();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn callback_oom_sweep_preserves_preexisting_mappings_and_pool_owners() {
	let _test = reset();
	let (_instance, device) = contexts();
	for fail_after in 1..=24 {
		let tracker = Tracker::new();
		let callbacks = tracker.callbacks();
		// SAFETY: fixture-backed live mapping survives each failed/successful scope;
		// all callback storage and native objects are explicitly retired afterward.
		unsafe {
			let mut allocator =
				Allocator::new_with_callbacks(&device, properties_coherent(), 64, &callbacks).unwrap();
			let mut pool = new_pool(&mut allocator);
			let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
			let pointer = allocator.map(&allocation).unwrap().cast::<u64>();
			pointer.write_unaligned(0x1234_5678);
			tracker.fail_at(tracker.snapshot().attempts + fail_after);
			let shared = SynchronizedAllocator::new(allocator);
			let result = shared.with_pools(&[&pool], |children| {
				children[0]
					.with_allocator(|allocator| create(allocator, &pool))
					.unwrap()
			});
			let mut allocator = shared.into_inner().unwrap();
			assert_eq!(pointer.read_unaligned(), 0x1234_5678);
			assert_eq!(allocator.allocation_info(&allocation).unwrap().map_count, 1);
			match result {
				Ok(Ok((buffer, mut allocation))) => {
					allocator.destroy_buffer(buffer, &mut allocation).unwrap()
				}
				Err(AllocError::HostAllocationFailed) | Ok(Err(AllocError::HostAllocationFailed)) => {}
				_ => panic!("unexpected scope outcome at OOM position {fail_after}"),
			}
			allocator.unmap(&allocation).unwrap();
			allocator.destroy_buffer(buffer, &mut allocation).unwrap();
			allocator.destroy_pool(&mut pool).unwrap();
			allocator.close().unwrap();
		}
		assert_eq!(tracker.snapshot().live, 0);
		assert_eq!(tracker.snapshot().unexpected, 0);
		empty_native();
	}
}

#[test]
fn outer_panic_restores_native_state_and_linear_pool_block_order() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: panic occurs before native work, with one stable explicitly held map.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut pool = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 64,
				min_block_count: 2,
				algorithm: crate::mem::AllocationAlgorithm::Linear,
				..Default::default()
			})
			.unwrap();
		let order: Vec<_> = allocator.blocks.iter().map(|block| block.handle).collect();
		let (buffer, mut allocation) = create(&mut allocator, &pool).unwrap();
		assert_eq!(allocation.handle(), *order.last().unwrap());
		let shared = SynchronizedAllocator::new(allocator);
		let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
			shared.with_pools::<()>(&[&pool], |children| {
				children[0]
					.with_allocator(|allocator| {
						assert_eq!(
							allocator
								.blocks
								.iter()
								.map(|block| block.handle)
								.collect::<Vec<_>>(),
							order
						);
					})
					.unwrap();
				panic!("outer pool-scope unwind");
			})
		}));
		assert!(panic.is_err());
		let mut allocator = shared
			.into_inner()
			.err()
			.expect("outer panic poisons parent")
			.into_inner();
		assert_eq!(
			allocator
				.blocks
				.iter()
				.map(|block| block.handle)
				.collect::<Vec<_>>(),
			order
		);
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn resource_claims_exclude_other_pools_during_native_allocation_and_raw_binding() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: caller-created buffer remains unbound until its single admitted
	// allocation is bound. The other pool must fail before another native import/
	// allocation/binding. No host bytes or GPU operations use this resource.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut first = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 64,
				..Default::default()
			})
			.unwrap();
		let mut second = allocator
			.create_pool(&PoolCreateInfo {
				block_size: 64,
				..Default::default()
			})
			.unwrap();
		let buffer = device
			.create_buffer(
				&vk::BufferCreateInfo {
					size: 64,
					usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let shared = SynchronizedAllocator::new(allocator);
		let gate = Gate::new(usize::MAX);
		let mut allocation = shared
			.with_pools(&[&first, &second], |children| {
				std::thread::scope(|scope| {
					let worker = scope.spawn(|| {
						children[0]
							.with_allocator(|allocator| {
								allocator.allocate_for_buffer(
									buffer,
									&AllocationCreateInfo {
										pool: Some(&first),
										..Default::default()
									},
								)
							})
							.unwrap()
							.unwrap()
					});
					gate.wait_for_entry();
					let duplicate = children[1]
						.with_allocator(|allocator| {
							allocator.allocate_for_buffer(
								buffer,
								&AllocationCreateInfo {
									pool: Some(&second),
									..Default::default()
								},
							)
						})
						.unwrap();
					gate.release();
					let allocation = worker.join().unwrap();
					assert!(matches!(
						duplicate,
						Err(AllocError::InvalidRequest(_)) | Err(AllocError::WrongResource)
					));
					assert_eq!(gate.0.lock().unwrap().entered, 1);
					*GATE.lock().unwrap() = None;
					children[1]
						.with_allocator(|allocator| {
							let mut raw = allocator
								.allocate_with_options(
									&vk::MemoryRequirements {
										size: 64,
										alignment: 64,
										memory_type_bits: 1,
									},
									&AllocationCreateInfo {
										pool: Some(&second),
										..Default::default()
									},
								)
								.unwrap();
							assert_eq!(
								allocator.bind_buffer_memory(buffer, &raw),
								Err(AllocError::WrongResource)
							);
							allocator.free(&mut raw).unwrap();
						})
						.unwrap();
					// A failed native request must also release its resource claim,
					// so another pool can allocate for that same still-unbound buffer.
					let retry_buffer = device
						.create_buffer(
							&vk::BufferCreateInfo {
								size: 64,
								usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
								..Default::default()
							},
							None,
						)
						.unwrap();
					FAIL.store(true, Ordering::SeqCst);
					assert!(
						children[0]
							.with_allocator(|allocator| allocator.allocate_for_buffer(
								retry_buffer,
								&AllocationCreateInfo {
									pool: Some(&first),
									..Default::default()
								}
							))
							.unwrap()
							.is_err()
					);
					children[1]
						.with_allocator(|allocator| {
							let mut retry = allocator
								.allocate_for_buffer(
									retry_buffer,
									&AllocationCreateInfo {
										pool: Some(&second),
										..Default::default()
									},
								)
								.unwrap();
							allocator.free(&mut retry).unwrap();
						})
						.unwrap();
					device.destroy_buffer(retry_buffer, None).unwrap();
					children[0]
						.with_allocator(|allocator| allocator.bind_buffer_memory(buffer, &allocation))
						.unwrap()
						.unwrap();
					allocation
				})
			})
			.unwrap();
		let mut allocator = shared.into_inner().unwrap();
		device.destroy_buffer(buffer, None).unwrap();
		allocator.free(&mut allocation).unwrap();
		allocator.destroy_pool(&mut first).unwrap();
		allocator.destroy_pool(&mut second).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[test]
fn native_resource_handle_reuse_during_another_pools_destroy_keeps_new_generation() {
	let _test = reset();
	let (_instance, device) = contexts();
	// SAFETY: destroy completes all uses; the native fixture recycles a destroyed
	// buffer handle in another pool before the old void PFN returns. Only checked
	// negative admission sees the new bound handle, before any native query/effect.
	unsafe {
		let mut allocator = Allocator::new(&device, properties_coherent(), 64).unwrap();
		let mut first = new_pool(&mut allocator);
		let mut second = new_pool(&mut allocator);
		let (old_buffer, mut old_allocation) = create(&mut allocator, &first).unwrap();
		let gate = Arc::new(Gate(
			Mutex::new(GateState {
				entered: 0,
				expected: usize::MAX,
				released: false,
				timed_out: false,
			}),
			Condvar::new(),
		));
		*REUSE_GATE.lock().unwrap() = Some((old_buffer.as_raw(), Arc::clone(&gate)));
		let shared = SynchronizedAllocator::new(allocator);
		let (buffer, mut allocation) = shared
			.with_pools(&[&first, &second], |children| {
				std::thread::scope(|scope| {
					let worker = scope.spawn(|| {
						children[0]
							.with_allocator(|allocator| allocator.destroy_buffer(old_buffer, &mut old_allocation))
							.unwrap()
							.unwrap()
					});
					gate.wait_for_entry();
					let result = children[1]
						.with_allocator(|allocator| create(allocator, &second))
						.unwrap();
					gate.release();
					worker.join().unwrap();
					let (buffer, allocation) = result.unwrap();
					assert_eq!(buffer, old_buffer);
					assert!(!gate.0.lock().unwrap().timed_out);
					*REUSE_GATE.lock().unwrap() = None;
					assert!(matches!(
						children[0]
							.with_allocator(|allocator| allocator.allocate_for_buffer(
								buffer,
								&AllocationCreateInfo {
									pool: Some(&first),
									..Default::default()
								}
							))
							.unwrap(),
						Err(AllocError::InvalidRequest(_))
					));
					(buffer, allocation)
				})
			})
			.unwrap();
		let mut allocator = shared.into_inner().unwrap();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		allocator.destroy_pool(&mut first).unwrap();
		allocator.destroy_pool(&mut second).unwrap();
		allocator.close().unwrap();
	}
	empty_native();
}

#[path = "test_scope.rs"]
mod scope;

#[path = "test_access.rs"]
mod access;

#[path = "test_native.rs"]
mod native;
