//! Multi-thread native-PFN ledger, real mapped host backing and explicit cleanup.
use super::*;
use crate::mem::{SynchronizedAllocator, host::test_callbacks::Tracker};
use std::{
	collections::BTreeMap,
	sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
};

#[derive(Default)]
struct Native {
	next: u64,
	// Never resize a live backing. Vec pointer/length access touches metadata
	// without borrowing a slice over bytes currently mapped by other workers.
	memory: BTreeMap<u64, (Vec<u8>, bool)>,
	types: BTreeMap<u64, u32>,
	buffers: BTreeMap<u64, Option<(u64, u64)>>,
	maps: usize,
	unmaps: usize,
	allocations: usize,
	frees: usize,
}

static NATIVE: Mutex<Native> = Mutex::new(Native {
	next: 1,
	memory: BTreeMap::new(),
	types: BTreeMap::new(),
	buffers: BTreeMap::new(),
	maps: 0,
	unmaps: 0,
	allocations: 0,
	frees: 0,
});
static TEST_LOCK: Mutex<()> = Mutex::new(());

static ACTIVE: AtomicUsize = AtomicUsize::new(0);
static OVERLAP: AtomicUsize = AtomicUsize::new(0);

struct Effect;

impl Effect {
	fn new() -> Self {
		if ACTIVE.fetch_add(1, Ordering::SeqCst) != 0 {
			OVERLAP.fetch_add(1, Ordering::SeqCst);
		}
		std::thread::yield_now();
		Self
	}
}

impl Drop for Effect {
	fn drop(&mut self) {
		ACTIVE.fetch_sub(1, Ordering::SeqCst);
	}
}

static BACKING_USERS: Mutex<std::collections::BTreeSet<u64>> =
	Mutex::new(std::collections::BTreeSet::new());

struct BackingUse(u64);

impl BackingUse {
	fn new(memory: vk::DeviceMemory) -> Self {
		assert!(
			BACKING_USERS.lock().unwrap().insert(memory.as_raw()),
			"overlapping native use of one backing"
		);
		Self(memory.as_raw())
	}
}

impl Drop for BackingUse {
	fn drop(&mut self) {
		assert!(BACKING_USERS.lock().unwrap().remove(&self.0));
	}
}

unsafe extern "system" fn resolve_instance(
	handle: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: loader supplies a NUL-terminated native command name.
	if unsafe { CStr::from_ptr(name) } == c"vkGetDeviceProcAddr" {
		pointer!(vk::PFN_vkGetDeviceProcAddr, resolve_device)
	} else {
		// SAFETY: ordinary parent fixture resolves only exact static PFNs.
		unsafe { super::gipa(handle, name) }
	}
}

unsafe extern "system" fn resolve_device(
	_: vk::DeviceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: loader supplies a NUL-terminated command name; every arm has the exact ABI.
	match unsafe { CStr::from_ptr(name) }.to_bytes() {
		b"vkDestroyDevice" => pointer!(vk::PFN_vkDestroyDevice, device_destroy),
		b"vkAllocateMemory" => pointer!(vk::PFN_vkAllocateMemory, memory_allocate),
		b"vkFreeMemory" => pointer!(vk::PFN_vkFreeMemory, memory_free),
		b"vkMapMemory" => pointer!(vk::PFN_vkMapMemory, memory_map),
		b"vkUnmapMemory" => pointer!(vk::PFN_vkUnmapMemory, memory_unmap),
		b"vkCreateBuffer" => pointer!(vk::PFN_vkCreateBuffer, buffer_create),
		b"vkDestroyBuffer" => pointer!(vk::PFN_vkDestroyBuffer, buffer_destroy),
		b"vkGetBufferMemoryRequirements" => {
			pointer!(vk::PFN_vkGetBufferMemoryRequirements, buffer_requirements)
		}
		b"vkBindBufferMemory" => pointer!(vk::PFN_vkBindBufferMemory, buffer_bind),
		_ => None,
	}
}

unsafe extern "system" fn device_destroy(_: vk::DeviceHandle, _: *const vk::AllocationCallbacks) {}

unsafe extern "system" fn memory_allocate(
	_: vk::DeviceHandle,
	info: *const vk::MemoryAllocateInfo,
	_: *const vk::AllocationCallbacks,
	output: *mut vk::DeviceMemory,
) -> vk::Result {
	let _effect = Effect::new();
	if !pools::allocation_gate() {
		return vk::Result::ERROR_OUT_OF_DEVICE_MEMORY;
	}
	let mut state = NATIVE.lock().unwrap();
	let serial = state.next;
	state.next += 1;
	// SAFETY: canonical allocator supplies initialized input and writable output.
	unsafe {
		let size = usize::try_from((*info).allocation_size).unwrap();
		state.memory.insert(serial, (vec![0; size], false));
		state.types.insert(serial, (*info).memory_type_index);
		*output = vk::DeviceMemory::from_raw(serial);
	}
	state.allocations += 1;
	vk::Result::SUCCESS
}

unsafe extern "system" fn memory_free(
	_: vk::DeviceHandle,
	memory: vk::DeviceMemory,
	_: *const vk::AllocationCallbacks,
) {
	let _effect = Effect::new();
	assert!(!BACKING_USERS.lock().unwrap().contains(&memory.as_raw()));
	let mut state = NATIVE.lock().unwrap();
	assert!(
		!state
			.buffers
			.values()
			.flatten()
			.any(|&(handle, _)| handle == memory.as_raw())
	);
	let (_, mapped) = state.memory.remove(&memory.as_raw()).unwrap();
	assert!(state.types.remove(&memory.as_raw()).is_some());
	assert!(!mapped);
	state.frees += 1;
}

unsafe extern "system" fn memory_map(
	_: vk::DeviceHandle,
	memory: vk::DeviceMemory,
	offset: u64,
	size: u64,
	_: vk::MemoryMapFlags,
	output: *mut *mut c_void,
) -> vk::Result {
	let _effect = Effect::new();
	let _backing = BackingUse::new(memory);
	if !pools::mapping_gate() {
		return vk::Result::ERROR_MEMORY_MAP_FAILED;
	}
	let mut state = NATIVE.lock().unwrap();
	let (backing, mapped) = state.memory.get_mut(&memory.as_raw()).unwrap();
	assert!(!*mapped);
	assert_eq!(offset, 0);
	assert_eq!(size, backing.len() as u64);
	*mapped = true;
	// SAFETY: canonical map supplies one writable output; boxed backing is stable
	// until explicit free, which this ledger rejects while a native map is live.
	unsafe {
		*output = backing.as_mut_ptr().cast();
	}
	state.maps += 1;
	vk::Result::SUCCESS
}

unsafe extern "system" fn memory_unmap(_: vk::DeviceHandle, memory: vk::DeviceMemory) {
	let _effect = Effect::new();
	let _backing = BackingUse::new(memory);
	let mut state = NATIVE.lock().unwrap();
	let (_, mapped) = state.memory.get_mut(&memory.as_raw()).unwrap();
	assert!(*mapped);
	*mapped = false;
	state.unmaps += 1;
}

unsafe extern "system" fn buffer_create(
	_: vk::DeviceHandle,
	_: *const vk::BufferCreateInfo,
	_: *const vk::AllocationCallbacks,
	output: *mut vk::Buffer,
) -> vk::Result {
	let _effect = Effect::new();
	let mut state = NATIVE.lock().unwrap();
	let serial = pools::recycled_buffer().unwrap_or_else(|| {
		let serial = state.next;
		state.next += 1;
		serial
	});
	state.buffers.insert(serial, None);
	// SAFETY: one writable output supplied by canonical creation.
	unsafe {
		*output = vk::Buffer::from_raw(serial);
	}
	vk::Result::SUCCESS
}

unsafe extern "system" fn buffer_destroy(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	_: *const vk::AllocationCallbacks,
) {
	let _effect = Effect::new();
	assert!(
		NATIVE
			.lock()
			.unwrap()
			.buffers
			.remove(&buffer.as_raw())
			.is_some()
	);
	pools::after_buffer_destroyed(buffer);
}

unsafe extern "system" fn buffer_requirements(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	output: *mut vk::MemoryRequirements,
) {
	let _effect = Effect::new();
	assert!(
		NATIVE
			.lock()
			.unwrap()
			.buffers
			.get(&buffer.as_raw())
			.unwrap()
			.is_none()
	);
	// SAFETY: one writable native output; fixed exact mock buffer requirements.
	unsafe {
		*output = vk::MemoryRequirements {
			size: 64,
			alignment: 64,
			memory_type_bits: pools::REQ_BITS.load(Ordering::SeqCst),
		};
	}
}

unsafe extern "system" fn buffer_bind(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	memory: vk::DeviceMemory,
	offset: u64,
) -> vk::Result {
	let _effect = Effect::new();
	let _backing = BackingUse::new(memory);
	if !pools::binding_gate(memory.as_raw()) {
		return vk::Result::ERROR_OUT_OF_DEVICE_MEMORY;
	}
	let mut state = NATIVE.lock().unwrap();
	assert!(offset + 64 <= state.memory[&memory.as_raw()].0.len() as u64);
	let binding = state.buffers.get_mut(&buffer.as_raw()).unwrap();
	assert!(binding.is_none());
	*binding = Some((memory.as_raw(), offset));
	vk::Result::SUCCESS
}

fn contexts() -> (vk::Instance, Device) {
	// SAFETY: static resolver with exact PFNs; only synthetic identities are imported.
	unsafe {
		let entry = Entry::new_from_resolver(resolve_instance).unwrap();
		let parent = Capabilities::new_from_instance_create_info(
			&vk::InstanceCreateInfo::default(),
			vk::API_VERSION_1_0,
		)
		.unwrap();
		let capabilities = parent
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_0)
			.unwrap();
		let instance = vk::Instance::new_from_handle_with_capabilities(
			&entry,
			vk::InstanceHandle(std::ptr::without_provenance_mut(0x1000)),
			parent,
		)
		.unwrap();
		let device = Device::new_from_handle_with_capabilities(
			&instance,
			vk::DeviceHandle(std::ptr::without_provenance_mut(0x2000)),
			capabilities,
		)
		.unwrap();
		(instance, device)
	}
}

#[test]
fn serialized_native_ownership_mapping_handoff_poison_and_host_drop() {
	let _test = TEST_LOCK
		.lock()
		.unwrap_or_else(std::sync::PoisonError::into_inner);
	*NATIVE.lock().unwrap() = Native {
		next: 1,
		..Default::default()
	};
	OVERLAP.store(0, Ordering::SeqCst);
	pools::reset_native_hooks();
	fn send_sync<T: Send + Sync>() {}
	send_sync::<SynchronizedAllocator<'_>>();
	fn send<T: Send>() {}
	send::<Allocation>();
	let tracker = Tracker::new();
	let callbacks = tracker.callbacks();
	let (instance, device) = contexts();
	let mut properties = properties();
	properties.memory_types[0].property_flags =
		properties.memory_types[0].property_flags | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	// SAFETY: exact fixture metadata, thread-safe fixed callback tracker and native
	// ledger; callbacks and contexts outlive all workers and final metadata cleanup.
	let shared = unsafe {
		SynchronizedAllocator::new(
			Allocator::new_with_callbacks(&device, properties, 64, &callbacks).unwrap(),
		)
	};

	std::thread::scope(|scope| {
		for worker in 0..4_u64 {
			let shared = &shared;
			scope.spawn(move || {
				let options = AllocationCreateInfo {
					policy: AllocationPolicy::Pooled { block_size: 4096 },
					..Default::default()
				};
				for iteration in 0..64_u64 {
					// SAFETY: the native ledger owns real stable backing. All accesses
					// occur while this exclusive operation holds; no GPU work exists.
					unsafe {
						shared
							.with_allocator(|allocator| {
								let (buffer, mut allocation) = allocator
									.create_buffer_with_options(
										&vk::BufferCreateInfo {
											size: 64,
											usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
											..Default::default()
										},
										&options,
									)
									.unwrap();
								allocator
									.set_allocation_name(&allocation, Some("threaded λ"))
									.unwrap();
								let pointer = allocator.map(&allocation).unwrap().cast::<u64>();
								let value = (worker << 32) | iteration;
								pointer.write_unaligned(value);
								assert_eq!(pointer.read_unaligned(), value);
								allocator.unmap(&allocation).unwrap();
								allocator.destroy_buffer(buffer, &mut allocation).unwrap();
							})
							.unwrap();
					}
					let _ = shared.statistics().unwrap();
				}
			});
		}
	});
	// A mapped token is transferred between workers; a free request must observe
	// its outstanding map, then succeed only after the receiving worker unmaps.
	let (send, receive) = std::sync::mpsc::channel();
	let receive = Mutex::new(receive);
	std::thread::scope(|scope| {
		scope.spawn(|| {
			// SAFETY: real host backing; token moves through the channel, pointer never escapes.
			let token = unsafe {
				shared
					.with_allocator(|allocator| {
						let allocation = allocator
							.allocate(&AllocRequest {
								size: 64,
								..request()
							})
							.unwrap();
						allocator.map(&allocation).unwrap();
						allocation
					})
					.unwrap()
			};
			send.send(token).unwrap();
		});
		scope.spawn(|| {
			let mut allocation = receive.lock().unwrap().recv().unwrap();
			// SAFETY: transferred map has no outstanding byte uses; explicit unmap
			// ends the hold before free, and device/callback lifetimes are retained.
			unsafe {
				shared
					.with_allocator(|allocator| {
						assert_eq!(
							allocator.free(&mut allocation),
							Err(AllocError::AlreadyMapped)
						);
						allocator.unmap(&allocation).unwrap();
						allocator.free(&mut allocation).unwrap();
					})
					.unwrap();
			}
		});
	});
	// Busy does not execute the nested nonblocking operation. Ordinary errors do
	// not poison the owner; panic does and recovery retains the original allocator.
	// SAFETY: no thread-affine replacement/registration or native work in this closure.
	unsafe {
		shared
			.with_allocator(|_| {
				assert!(
					shared
						.try_with_allocator(|_| panic!("busy operation ran"))
						.unwrap()
						.is_none()
				);
			})
			.unwrap();
	}
	assert!(shared.statistics().is_ok());
	std::thread::scope(|scope| {
		let worker = scope.spawn(|| {
			// SAFETY: deliberate panic occurs before touching native state.
			let _ = unsafe { shared.with_allocator::<()>(|_| panic!("injected closure failure")) };
		});
		assert!(worker.join().is_err());
	});
	assert_eq!(
		shared.statistics(),
		Err(AllocError::SynchronizationPoisoned)
	);
	// SAFETY: poison refusal must prevent any closure access.
	assert_eq!(
		unsafe { shared.with_allocator(|_| ()) },
		Err(AllocError::SynchronizationPoisoned)
	);
	// SAFETY: nonblocking poison refusal must also prevent access.
	assert_eq!(
		unsafe { shared.try_with_allocator(|_| ()) },
		Err(AllocError::SynchronizationPoisoned)
	);
	let mut allocator = match shared.into_inner() {
		Err(poison) => poison.into_inner(),
		Ok(_) => panic!("poison lost"),
	};
	allocator.close().unwrap();
	drop(allocator);
	let snapshot = tracker.snapshot();
	assert_eq!(
		(snapshot.live, snapshot.bytes, snapshot.unexpected),
		(0, 0, 0)
	);
	assert_eq!(snapshot.successes, snapshot.frees);
	let state = NATIVE.lock().unwrap();
	assert!(state.memory.is_empty() && state.buffers.is_empty());
	assert_eq!(state.maps, state.unmaps);
	assert_eq!(state.allocations, state.frees);
	assert!(state.maps > 0 && state.allocations > 0);
	assert_eq!(OVERLAP.load(Ordering::SeqCst), 0);
	assert_eq!(ACTIVE.load(Ordering::SeqCst), 0);
	drop(state);
	// A final CPU-only Drop with deliberately unfreed native storage must not
	// make a native call; clear only the independent mock backing after observing it.
	// SAFETY: exact same retained any-thread fixture, no GPU or external byte uses.
	unsafe {
		let owner = SynchronizedAllocator::new(
			Allocator::new_with_callbacks(&device, properties, 64, &callbacks).unwrap(),
		);
		owner
			.with_allocator(|allocator| {
				allocator
					.allocate(&AllocRequest {
						size: 64,
						..request()
					})
					.unwrap()
			})
			.unwrap();
		std::thread::scope(|scope| {
			scope.spawn(move || drop(owner)).join().unwrap();
		});
	}
	assert_eq!(tracker.snapshot().live, 0);
	let mut state = NATIVE.lock().unwrap();
	assert_eq!(state.memory.len(), 1);
	assert_eq!(state.allocations, state.frees + 1);
	state.memory.clear();
	state.types.clear();
	drop(state);
	drop(instance);
}

#[path = "synchronized/test_pool.rs"]
mod pools;
