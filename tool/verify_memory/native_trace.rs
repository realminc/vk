//! Development-only public allocator consumer with an independently checked synthetic driver.
//! No allocator algorithm is implemented here; the same protocol drives pinned native VMA.
use std::{
	cell::RefCell,
	collections::BTreeMap,
	ffi::{CStr, c_char, c_void},
	io::{self, BufRead},
};
use vk::{
	AllocError, Allocation, Allocator,
	load::{CallError, Capabilities, Entry},
	memory::{AllocationCreateInfo, AllocationPolicy, PlacementStrategy},
};

struct Native {
	bytes: Box<[u8]>,
	index: u32,
	mapped: bool,
}

#[derive(Default)]
struct Driver {
	coherent: bool,
	size: u64,
	alignment: u64,
	bits: u32,
	failure: (i32, u32, i32),
	next: u64,
	memory: BTreeMap<u64, Native>,
	buffers: BTreeMap<u64, u64>,
}

thread_local! { static DRIVER: RefCell<Driver> = RefCell::new(Driver { failure: (-1, 0, 0), next: 100, ..Default::default() }); }

fn status(site: i32, index: u32) -> vk::Result {
	DRIVER.with_borrow(|d| {
		if d.failure.0 == site && (site != 0 || d.failure.1 == index) {
			vk::Result::from_raw(d.failure.2)
		} else {
			vk::Result::SUCCESS
		}
	})
}

fn memory_properties() -> vk::PhysicalDeviceMemoryProperties {
	// SAFETY: output-only integer ABI record accepts zero initialization.
	let mut p: vk::PhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
	p.memory_heap_count = 1;
	p.memory_heaps[0].size = 1 << 30;
	p.memory_heaps[0].flags = vk::MemoryHeapFlagBits::DEVICE_LOCAL_BIT;
	p.memory_type_count = 3;
	let coherent = DRIVER.with_borrow(|d| if d.coherent { 4 } else { 0 });
	p.memory_types[0].property_flags = vk::MemoryPropertyFlags::from_raw(2 | coherent);
	p.memory_types[1].property_flags = vk::MemoryPropertyFlags::from_raw(2 | 8 | coherent);
	p.memory_types[2].property_flags = vk::MemoryPropertyFlags::from_raw(1);
	p
}

macro_rules! pointer {
	($ty:ty, $function:ident) => {{
		// SAFETY: each arm erases the exact named command's signature.
		unsafe { std::mem::transmute::<$ty, vk::PFN_vkVoidFunction>(Some($function)) }
	}};
}

unsafe extern "system" fn resolve(
	_: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: table initialization supplies live NUL-terminated names.
	match unsafe { CStr::from_ptr(name) }.to_bytes() {
		b"vkCreateInstance" => pointer!(vk::PFN_vkCreateInstance, create_instance),
		b"vkEnumerateInstanceExtensionProperties" => {
			pointer!(vk::PFN_vkEnumerateInstanceExtensionProperties, extensions)
		}
		b"vkEnumerateInstanceLayerProperties" => {
			pointer!(vk::PFN_vkEnumerateInstanceLayerProperties, layers)
		}
		b"vkDestroyInstance" => pointer!(vk::PFN_vkDestroyInstance, destroy_instance),
		b"vkGetDeviceProcAddr" => pointer!(vk::PFN_vkGetDeviceProcAddr, resolve_device),
		_ => None,
	}
}

unsafe extern "system" fn resolve_device(
	_: vk::DeviceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: table initialization supplies live NUL-terminated names.
	match unsafe { CStr::from_ptr(name) }.to_bytes() {
		b"vkDestroyDevice" => pointer!(vk::PFN_vkDestroyDevice, destroy_device),
		b"vkAllocateMemory" => pointer!(vk::PFN_vkAllocateMemory, allocate_memory),
		b"vkFreeMemory" => pointer!(vk::PFN_vkFreeMemory, free_memory),
		b"vkCreateBuffer" => pointer!(vk::PFN_vkCreateBuffer, create_buffer),
		b"vkDestroyBuffer" => pointer!(vk::PFN_vkDestroyBuffer, destroy_buffer),
		b"vkGetBufferMemoryRequirements" => {
			pointer!(vk::PFN_vkGetBufferMemoryRequirements, requirements)
		}
		b"vkBindBufferMemory" => pointer!(vk::PFN_vkBindBufferMemory, bind),
		b"vkMapMemory" => pointer!(vk::PFN_vkMapMemory, map),
		b"vkUnmapMemory" => pointer!(vk::PFN_vkUnmapMemory, unmap),
		b"vkFlushMappedMemoryRanges" => pointer!(vk::PFN_vkFlushMappedMemoryRanges, flush),
		b"vkInvalidateMappedMemoryRanges" => {
			pointer!(vk::PFN_vkInvalidateMappedMemoryRanges, invalidate)
		}
		_ => None,
	}
}

unsafe extern "system" fn create_instance(
	_: *const vk::InstanceCreateInfo,
	_: *const vk::AllocationCallbacks,
	_: *mut vk::InstanceHandle,
) -> vk::Result {
	unreachable!()
}

unsafe extern "system" fn extensions(
	_: *const c_char,
	_: *mut u32,
	_: *mut vk::ExtensionProperties,
) -> vk::Result {
	unreachable!()
}

unsafe extern "system" fn layers(_: *mut u32, _: *mut vk::LayerProperties) -> vk::Result {
	unreachable!()
}

unsafe extern "system" fn destroy_instance(
	_: vk::InstanceHandle,
	_: *const vk::AllocationCallbacks,
) {
}

unsafe extern "system" fn destroy_device(_: vk::DeviceHandle, _: *const vk::AllocationCallbacks) {}

unsafe extern "system" fn allocate_memory(
	_: vk::DeviceHandle,
	info: *const vk::MemoryAllocateInfo,
	_: *const vk::AllocationCallbacks,
	output: *mut vk::DeviceMemory,
) -> vk::Result {
	// SAFETY: allocator supplies live synchronous input/output ABI records.
	let info = unsafe { &*info };
	assert!(info.p_next.is_null());
	let result = status(0, info.memory_type_index);
	let handle = DRIVER.with_borrow_mut(|d| {
		if result.is_err() {
			return 0;
		}
		let handle = d.next;
		d.next += 1;
		d.memory.insert(
			handle,
			Native {
				bytes: vec![0; info.allocation_size as usize].into_boxed_slice(),
				index: info.memory_type_index,
				mapped: false,
			},
		);
		handle
	});
	println!(
		"e allocate {} {} {} {handle}",
		info.memory_type_index,
		info.allocation_size,
		result.as_raw()
	);
	if !result.is_err() {
		// SAFETY: one writable output supplied by allocator.
		unsafe { *output = vk::DeviceMemory::from_raw(handle) };
	}
	result
}

unsafe extern "system" fn free_memory(
	_: vk::DeviceHandle,
	memory: vk::DeviceMemory,
	_: *const vk::AllocationCallbacks,
) {
	DRIVER.with_borrow_mut(|d| {
		assert!(!d.memory.remove(&memory.as_raw()).unwrap().mapped);
	});
	println!("e free {}", memory.as_raw());
}

unsafe extern "system" fn create_buffer(
	_: vk::DeviceHandle,
	info: *const vk::BufferCreateInfo,
	_: *const vk::AllocationCallbacks,
	output: *mut vk::Buffer,
) -> vk::Result {
	let result = status(3, 0);
	let handle = DRIVER.with_borrow_mut(|d| {
		if result.is_err() {
			return 0;
		}
		let h = d.next;
		d.next += 1;
		// SAFETY: live synchronous input record.
		d.buffers.insert(h, unsafe { (*info).size });
		h
	});
	println!("e create {handle} {}", result.as_raw());
	if !result.is_err() {
		// SAFETY: one writable output supplied by allocator.
		unsafe { *output = vk::Buffer::from_raw(handle) };
	}
	result
}

unsafe extern "system" fn destroy_buffer(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	_: *const vk::AllocationCallbacks,
) {
	DRIVER.with_borrow_mut(|d| {
		d.buffers.remove(&buffer.as_raw()).unwrap();
	});
	println!("e destroy {}", buffer.as_raw());
}

unsafe extern "system" fn requirements(
	_: vk::DeviceHandle,
	_: vk::Buffer,
	output: *mut vk::MemoryRequirements,
) {
	DRIVER.with_borrow(|d| {
		// SAFETY: one writable output supplied by allocator.
		unsafe {
			*output = vk::MemoryRequirements {
				size: d.size,
				alignment: d.alignment,
				memory_type_bits: d.bits,
			}
		};
	});
}

unsafe extern "system" fn bind(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	memory: vk::DeviceMemory,
	offset: u64,
) -> vk::Result {
	let result = status(2, 0);
	DRIVER.with_borrow(|d| {
		let native = &d.memory[&memory.as_raw()];
		assert!(d.buffers.contains_key(&buffer.as_raw()));
		assert!(offset.is_multiple_of(d.alignment));
		assert!(offset + d.size <= native.bytes.len() as u64);
		assert_ne!(d.bits & (1 << native.index), 0);
	});
	println!(
		"e bind {} {} {offset} {}",
		buffer.as_raw(),
		memory.as_raw(),
		result.as_raw()
	);
	result
}

unsafe extern "system" fn map(
	_: vk::DeviceHandle,
	memory: vk::DeviceMemory,
	offset: u64,
	size: u64,
	_: vk::MemoryMapFlags,
	output: *mut *mut c_void,
) -> vk::Result {
	let result = status(1, 0);
	DRIVER.with_borrow_mut(|d| {
		let native = d.memory.get_mut(&memory.as_raw()).unwrap();
		assert!(!native.mapped);
		assert_eq!(offset, 0);
		assert_eq!(size, native.bytes.len() as u64);
		if !result.is_err() {
			native.mapped = true;
			// SAFETY: real backing storage outlives its explicit native map.
			unsafe { *output = native.bytes.as_mut_ptr().cast() };
		}
	});
	println!(
		"e map {} {offset} {size} {}",
		memory.as_raw(),
		result.as_raw()
	);
	result
}

unsafe extern "system" fn unmap(_: vk::DeviceHandle, memory: vk::DeviceMemory) {
	DRIVER.with_borrow_mut(|d| {
		let n = d.memory.get_mut(&memory.as_raw()).unwrap();
		assert!(n.mapped);
		n.mapped = false;
	});
	println!("e unmap {}", memory.as_raw());
}

unsafe fn ranges(op: &str, count: u32, ranges: *const vk::MappedMemoryRange) -> vk::Result {
	assert_eq!(count, 1);
	// SAFETY: caller supplies count live ABI records.
	let range = unsafe { &*ranges };
	println!(
		"e {op} {} {} {}",
		range.memory.as_raw(),
		range.offset,
		range.size
	);
	vk::Result::SUCCESS
}

unsafe extern "system" fn flush(
	_: vk::DeviceHandle,
	count: u32,
	data: *const vk::MappedMemoryRange,
) -> vk::Result {
	// SAFETY: forwarded exact native arguments.
	unsafe { ranges("flush", count, data) }
}

unsafe extern "system" fn invalidate(
	_: vk::DeviceHandle,
	count: u32,
	data: *const vk::MappedMemoryRange,
) -> vk::Result {
	// SAFETY: forwarded exact native arguments.
	unsafe { ranges("invalidate", count, data) }
}

fn check_pointer(allocation: &Allocation, pointer: *mut c_void) {
	DRIVER.with_borrow(|d| {
		let n = &d.memory[&allocation.handle().as_raw()];
		assert!(n.mapped);
		assert_eq!(
			pointer as usize,
			n.bytes.as_ptr() as usize + allocation.offset() as usize
		);
	});
}

fn check_failure(error: AllocError) {
	let AllocError::Call(CallError::Vulkan(result)) = error else {
		panic!("unexpected allocator failure: {error}");
	};
	DRIVER.with_borrow(|d| {
		assert!(d.failure.0 >= 0);
		assert_eq!(result.as_raw(), d.failure.2);
	});
}

fn main() {
	let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
	let config: Vec<_> = lines[0].split_whitespace().collect();
	assert_eq!(config[0], "b");
	let atom = config[1].parse::<u64>().unwrap();
	let coherent = config[2] == "1";
	let separate = config[3] == "1";
	DRIVER.with_borrow_mut(|d| {
		d.coherent = coherent;
	});
	let app = vk::ApplicationInfo {
		api_version: vk::API_VERSION_1_0,
		..Default::default()
	};
	let info = vk::InstanceCreateInfo {
		p_application_info: &app,
		..Default::default()
	};
	// SAFETY: synthetic handles, exact static PFNs and live creation records only.
	let (instance, device) = unsafe {
		let parent = Capabilities::new_from_instance_create_info(&info, vk::API_VERSION_1_0).unwrap();
		let caps = parent
			.new_from_device_create_info(&vk::DeviceCreateInfo::default(), vk::API_VERSION_1_0)
			.unwrap();
		let entry = Entry::new_from_resolver(resolve).unwrap();
		let instance = vk::Instance::new_from_handle_with_capabilities(
			&entry,
			vk::InstanceHandle(std::ptr::without_provenance_mut(0x1000)),
			parent,
		)
		.unwrap();
		let device = vk::Device::new_from_handle_with_capabilities(
			&instance,
			vk::DeviceHandle(std::ptr::without_provenance_mut(0x2000)),
			caps,
		)
		.unwrap();
		(instance, device)
	};
	// SAFETY: properties exactly match the synthetic driver and all its admitted limits.
	let mut allocator = unsafe { Allocator::new(&device, memory_properties(), atom) }.unwrap();
	let mut live: BTreeMap<u64, (vk::Buffer, Allocation)> = BTreeMap::new();
	for line in lines.iter().skip(1) {
		let words: Vec<_> = line.split_whitespace().collect();
		let number = |i: usize| words[i].parse::<u64>().unwrap();
		let op = words[0];
		let id = if op == "c" || op == "x" { 0 } else { number(1) };
		let (mut ok, mut output) = (true, [0u64; 5]);
		// SAFETY: each request is a valid synthetic resource; maps are matched and no GPU work exists.
		unsafe {
			match op {
				"a" => {
					let size = number(2);
					let alignment = number(3);
					let bits = number(4) as u32;
					DRIVER.with_borrow_mut(|d| {
						d.size = size;
						d.alignment = alignment;
						d.bits = bits;
					});
					let info = vk::BufferCreateInfo {
						size,
						usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
						..Default::default()
					};
					let strategy = match number(8) {
						0 => PlacementStrategy::Default,
						1 => PlacementStrategy::MinMemory,
						2 => PlacementStrategy::MinTime,
						3 => PlacementStrategy::MinOffset,
						_ => panic!("bad strategy"),
					};
					let options = AllocationCreateInfo {
						required_flags: vk::MemoryPropertyFlags::from_raw(number(5) as u32),
						preferred_flags: vk::MemoryPropertyFlags::from_raw(number(6) as u32),
						memory_type_bits: bits,
						persistent_map: number(7) != 0,
						strategy,
						policy: if separate {
							AllocationPolicy::Separate
						} else {
							AllocationPolicy::Pooled { block_size: 65536 }
						},
						..Default::default()
					};
					match allocator.create_buffer_with_options(&info, &options) {
						Ok((buffer, allocation)) => {
							output = [
								buffer.as_raw(),
								allocation.handle().as_raw(),
								allocation.offset(),
								allocation.size(),
								allocation.memory_type_index() as u64,
							];
							if options.persistent_map {
								check_pointer(
									&allocation,
									allocator.mapped_pointer(&allocation).unwrap().unwrap(),
								);
							}
							assert!(live.insert(id, (buffer, allocation)).is_none());
						}
						Err(error) => {
							check_failure(error);
							ok = false;
						}
					}
				}
				"m" => {
					let a = &live[&id].1;
					match allocator.map(a) {
						Ok(p) => check_pointer(a, p),
						Err(error) => {
							check_failure(error);
							ok = false;
						}
					}
				}
				"u" => {
					allocator.unmap(&live[&id].1).unwrap();
				}
				"h" => {
					allocator.flush(&live[&id].1, number(2), number(3)).unwrap();
				}
				"i" => {
					allocator
						.invalidate(&live[&id].1, number(2), number(3))
						.unwrap();
				}
				"f" => {
					let (buffer, mut allocation) = live.remove(&id).unwrap();
					allocator.destroy_buffer(buffer, &mut allocation).unwrap();
				}
				"x" => {
					DRIVER.with_borrow_mut(|d| {
						d.failure = (
							words[1].parse().unwrap(),
							number(2) as u32,
							words[3].parse().unwrap(),
						)
					});
				}
				"c" => {
					assert!(live.is_empty());
					allocator.close().unwrap();
				}
				_ => panic!("invalid operation"),
			}
		}
		let s = allocator.statistics();
		println!(
			"r {op} {id} {} {} {} {} {} {} {} {} {} {} {}",
			u8::from(ok),
			output[0],
			output[1],
			output[2],
			output[3],
			output[4],
			s.allocation_count,
			s.allocation_bytes,
			s.native_allocation_count,
			s.native_bytes,
			s.mapped_native_count
		);
	}
	assert!(live.is_empty());
	DRIVER.with_borrow(|d| {
		assert!(d.memory.is_empty() && d.buffers.is_empty());
	});
	// SAFETY: synthetic children/storage are explicitly closed first.
	unsafe {
		device.destroy(None);
		instance.destroy(None);
	}
}
