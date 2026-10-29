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
	mem::{
		AllocationAlgorithm, AllocationCreateInfo, AllocationPolicy, PlacementStrategy, PoolCreateInfo,
	},
};

struct Native {
	bytes: Box<[u8]>,
	index: u32,
	mapped: bool,
}

#[derive(Default)]
struct Driver {
	coherent: bool,
	address: bool,
	automatic_pool: bool,
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
	p.memory_heaps[0].size = DRIVER.with_borrow(|d| {
		if d.automatic_pool {
			512 * 1024
		} else {
			1 << 30
		}
	});
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
		b"vkCreateImage" => pointer!(vk::PFN_vkCreateImage, create_image),
		b"vkDestroyImage" => pointer!(vk::PFN_vkDestroyImage, destroy_image),
		b"vkGetImageMemoryRequirements" => {
			pointer!(vk::PFN_vkGetImageMemoryRequirements, image_requirements)
		}
		b"vkBindImageMemory" => pointer!(vk::PFN_vkBindImageMemory, bind_image),
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
	let flags = if info.p_next.is_null() {
		0
	} else {
		// SAFETY: allocator supplies a complete allocation-flags record for this mode.
		let flags = unsafe { &*info.p_next.cast::<vk::MemoryAllocateFlagsInfo>() };
		assert_eq!(flags.s_type, vk::StructureType::MEMORY_ALLOCATE_FLAGS_INFO);
		assert!(flags.p_next.is_null());
		assert_eq!(flags.device_mask, 0);
		flags.flags.as_raw()
	};
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
	print!(
		"e allocate {} {} {} {handle}",
		info.memory_type_index,
		info.allocation_size,
		result.as_raw()
	);
	if DRIVER.with_borrow(|d| d.address) {
		print!(" {flags}");
	} else {
		assert_eq!(flags, 0);
	}
	println!();
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

// The synthetic driver uses one integer resource namespace. Image entry points
// retain their exact PFN ABI and share only its bookkeeping/event helpers.
unsafe extern "system" fn create_image(
	device: vk::DeviceHandle,
	_: *const vk::ImageCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::Image,
) -> vk::Result {
	let info = vk::BufferCreateInfo {
		size: DRIVER.with_borrow(|d| d.size),
		..Default::default()
	};
	let mut buffer = vk::Buffer::NULL;
	// SAFETY: exact synchronous synthetic inputs/outputs, no native Vulkan call.
	let result = unsafe { create_buffer(device, &info, callbacks, &mut buffer) };
	if !result.is_err() {
		// SAFETY: exact writable image output supplied by allocator.
		unsafe { *output = vk::Image::from_raw(buffer.as_raw()) };
	}
	result
}

unsafe extern "system" fn destroy_image(
	device: vk::DeviceHandle,
	image: vk::Image,
	callbacks: *const vk::AllocationCallbacks,
) {
	// SAFETY: synthetic resource identity belongs to the same driver namespace.
	unsafe { destroy_buffer(device, vk::Buffer::from_raw(image.as_raw()), callbacks) };
}

unsafe extern "system" fn image_requirements(
	device: vk::DeviceHandle,
	image: vk::Image,
	output: *mut vk::MemoryRequirements,
) {
	// SAFETY: exact writable requirements output; common synthetic requirement source.
	unsafe { requirements(device, vk::Buffer::from_raw(image.as_raw()), output) };
}

unsafe extern "system" fn bind_image(
	device: vk::DeviceHandle,
	image: vk::Image,
	memory: vk::DeviceMemory,
	offset: u64,
) -> vk::Result {
	// SAFETY: common synthetic resource identity and checked native storage range.
	unsafe { bind(device, vk::Buffer::from_raw(image.as_raw()), memory, offset) }
}

#[derive(Clone, Copy)]
enum Resource {
	Buffer(vk::Buffer),
	Image(vk::Image),
}

impl Resource {
	fn as_raw(self) -> u64 {
		match self {
			Self::Buffer(value) => value.as_raw(),
			Self::Image(value) => value.as_raw(),
		}
	}
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

unsafe fn existing_resource(
	allocator: &mut Allocator<'_>,
	device: &vk::Device,
	resource: Resource,
	options: &AllocationCreateInfo,
	raw: bool,
) -> Result<(Resource, Allocation), AllocError> {
	// SAFETY: fresh ordinary unbound resource belongs to this synthetic device.
	let result = (|| unsafe {
		if raw {
			let mut requirements = vk::MemoryRequirements::default();
			match resource {
				Resource::Buffer(buffer) => requirements = device.get_buffer_memory_requirements(buffer)?,
				Resource::Image(image) => device.get_image_memory_requirements(image, &mut requirements)?,
			}
			allocator.allocate_with_options(&requirements, options)
		} else {
			match resource {
				Resource::Buffer(buffer) => allocator.allocate_for_buffer(buffer, options),
				Resource::Image(image) => allocator.allocate_for_image(image, options),
			}
		}
	})();
	let destroy = || {
		assert!(DRIVER.with_borrow(|d| d.buffers.contains_key(&resource.as_raw())));
		// SAFETY: this consumer owns the unbound/unused resource and supplies matching callbacks.
		unsafe {
			match resource {
				Resource::Buffer(buffer) => device.destroy_buffer(buffer, None).unwrap(),
				Resource::Image(image) => device.destroy_image(image, None).unwrap(),
			}
		}
	};
	let mut allocation = match result {
		Ok(allocation) => allocation,
		Err(error) => {
			destroy();
			return Err(error);
		}
	};
	// SAFETY: queried storage and the same unbound resource; no GPU work exists.
	let binding = unsafe {
		match resource {
			Resource::Buffer(buffer) => allocator.bind_buffer_memory(buffer, &allocation),
			Resource::Image(image) => allocator.bind_image_memory(image, &allocation),
		}
	};
	if let Err(error) = binding {
		destroy();
		// SAFETY: failed binding published no resource use; release only this allocation.
		unsafe { allocator.free(&mut allocation) }.unwrap();
		return Err(error);
	}
	Ok((resource, allocation))
}

fn main() {
	let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
	let config: Vec<_> = lines[0].split_whitespace().collect();
	assert_eq!(config[0], "b");
	let atom = config[1].parse::<u64>().unwrap();
	let coherent = config[2] == "1";
	let heap_limit = config
		.get(7)
		.map_or(0, |value| value.parse::<u64>().unwrap());
	let minimum = config
		.get(8)
		.map_or(0, |value| value.parse::<usize>().unwrap());
	let granularity = config
		.get(6)
		.map_or(1, |value| value.parse::<u64>().unwrap());
	let separate = config[3] == "1";
	let pool_mode = config.get(4).copied().unwrap_or("0");
	let custom_pool = pool_mode != "0";
	let automatic_pool = pool_mode == "2";
	let linear_pool = pool_mode == "3";
	let maximum = config
		.get(9)
		.map_or(if automatic_pool { 2 } else { 1 }, |value| {
			value.parse::<usize>().unwrap()
		});
	let address = config.get(5).is_some_and(|value| *value == "1");
	DRIVER.with_borrow_mut(|d| {
		d.coherent = coherent;
		d.address = address;
		d.automatic_pool = automatic_pool;
	});
	let app = vk::ApplicationInfo {
		api_version: vk::API_VERSION_1_0,
		..Default::default()
	};
	let instance_extensions = [c"VK_KHR_get_physical_device_properties2".as_ptr()];
	let info = vk::InstanceCreateInfo {
		p_application_info: &app,
		enabled_extension_count: u32::from(address),
		pp_enabled_extension_names: instance_extensions.as_ptr(),
		..Default::default()
	};
	// SAFETY: synthetic handles, exact static PFNs and live creation records only.
	let (instance, device) = unsafe {
		let parent = Capabilities::new_from_instance_create_info(&info, vk::API_VERSION_1_0).unwrap();
		let feature = vk::PhysicalDeviceBufferDeviceAddressFeatures {
			buffer_device_address: vk::TRUE,
			..Default::default()
		};
		let extensions = [
			c"VK_KHR_device_group".as_ptr(),
			c"VK_KHR_buffer_device_address".as_ptr(),
		];
		let device_info = vk::DeviceCreateInfo {
			p_next: if address {
				std::ptr::from_ref(&feature).cast()
			} else {
				std::ptr::null()
			},
			enabled_extension_count: if address { 2 } else { 0 },
			pp_enabled_extension_names: extensions.as_ptr(),
			..Default::default()
		};
		let caps = parent
			.new_from_device_create_info(&device_info, vk::API_VERSION_1_0)
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
	let mut allocator =
		unsafe { Allocator::new_with_granularity(&device, memory_properties(), atom, granularity) }
			.unwrap();
	// Match native_oracle.cpp's VMA_MAPPING_HYSTERESIS_ENABLED=0 profile;
	// the independent model requires native maps to equal explicit/persistent holds.
	allocator.set_mapping_hysteresis(false).unwrap();
	if heap_limit != 0 {
		allocator.set_heap_limit(0, heap_limit).unwrap();
	}
	// SAFETY: exact synthetic memory type and plain fixed-size native blocks.
	let mut pool = custom_pool.then(|| unsafe {
		allocator
			.create_pool(&PoolCreateInfo {
				memory_type_index: 0,
				algorithm: if linear_pool {
					AllocationAlgorithm::Linear
				} else {
					AllocationAlgorithm::Tlsf
				},
				block_size: if automatic_pool { 0 } else { 65536 },
				min_block_count: minimum,
				max_block_count: maximum,
				min_allocation_alignment: 256,
				..Default::default()
			})
			.unwrap()
	});
	let mut live: BTreeMap<u64, (Resource, Allocation)> = BTreeMap::new();
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
						usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT
							| if address && id % 2 == 0 {
								vk::BufferUsageFlagBits::SHADER_DEVICE_ADDRESS_BIT
							} else {
								vk::BufferUsageFlags::empty()
							},
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
						pool: pool.as_ref(),
						required_flags: vk::MemoryPropertyFlags::from_raw(number(5) as u32),
						preferred_flags: vk::MemoryPropertyFlags::from_raw(number(6) as u32),
						memory_type_bits: bits,
						persistent_map: number(7) != 0,
						upper_address: words.get(9) == Some(&"1"),
						strategy,
						policy: if separate {
							AllocationPolicy::Separate
						} else {
							AllocationPolicy::Pooled { block_size: 65536 }
						},
						..Default::default()
					};
					let class = words
						.get(10)
						.map_or(0, |value| value.parse::<u8>().unwrap());
					let raw = words.get(11) == Some(&"2");
					let existing = raw || words.get(11) == Some(&"1");
					let result = if class == 0 && existing {
						device
							.create_buffer(&info, None)
							.map_err(AllocError::from)
							.and_then(|buffer| {
								existing_resource(
									&mut allocator,
									&device,
									Resource::Buffer(buffer),
									&options,
									raw,
								)
							})
					} else if class == 0 {
						allocator
							.create_buffer_with_options(&info, &options)
							.map(|(buffer, allocation)| (Resource::Buffer(buffer), allocation))
					} else {
						assert!(class == 1 || class == 2);
						let info = vk::ImageCreateInfo {
							image_type: vk::ImageType::_2D,
							format: vk::Format::R8G8B8A8_UNORM,
							extent: vk::Extent3D {
								width: 7,
								height: 9,
								depth: 1,
							},
							mip_levels: 1,
							array_layers: 1,
							samples: vk::SampleCountFlagBits::_1_BIT,
							tiling: if class == 1 {
								vk::ImageTiling::LINEAR
							} else {
								vk::ImageTiling::OPTIMAL
							},
							usage: vk::ImageUsageFlagBits::TRANSFER_DST_BIT,
							..Default::default()
						};
						if existing {
							device
								.create_image(&info, None)
								.map_err(AllocError::from)
								.and_then(|image| {
									existing_resource(
										&mut allocator,
										&device,
										Resource::Image(image),
										&options,
										raw,
									)
								})
						} else {
							allocator
								.create_image_with_options(&info, &options)
								.map(|(image, allocation)| (Resource::Image(image), allocation))
						}
					};
					match result {
						Ok((buffer, allocation)) => {
							let snapshot = allocator.allocation_info(&allocation).unwrap();
							output = [
								buffer.as_raw(),
								snapshot.device_memory.as_raw(),
								snapshot.offset,
								snapshot.size,
								u64::from(snapshot.memory_type_index),
							];
							if options.persistent_map {
								check_pointer(&allocation, snapshot.mapped_pointer.unwrap());
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
					match buffer {
						Resource::Buffer(buffer) => allocator.destroy_buffer(buffer, &mut allocation).unwrap(),
						Resource::Image(image) => allocator.destroy_image(image, &mut allocation).unwrap(),
					}
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
					if let Some(pool) = pool.as_mut() {
						allocator.destroy_pool(pool).unwrap();
					}
					allocator.close().unwrap();
				}
				_ => panic!("invalid operation"),
			}
		}
		for (_, allocation) in live.values() {
			let info = allocator.allocation_info(allocation).unwrap();
			DRIVER.with_borrow(|d| {
				let native = &d.memory[&info.device_memory.as_raw()];
				assert_eq!(info.memory_type_index, native.index);
				assert!(info.offset + info.reserved_size <= native.bytes.len() as u64);
			});
			assert_eq!(info.heap_index, 0);
			assert_eq!(
				info.memory_properties,
				memory_properties().memory_types[info.memory_type_index as usize].property_flags
			);
			if let Some(pointer) = info.mapped_pointer {
				check_pointer(allocation, pointer);
			}
		}
		let s = allocator.statistics();
		if let Some(pool) = pool.as_ref().filter(|pool| !pool.is_destroyed()) {
			let p = allocator.pool_statistics(pool).unwrap();
			assert_eq!(p.allocation_count, s.allocation_count);
			assert_eq!(p.allocation_bytes, s.allocation_bytes);
			assert_eq!(p.block_count, s.native_allocation_count);
			assert_eq!(p.block_bytes, s.native_bytes);
			assert_eq!(p.free_bytes + p.reserved_bytes, p.block_bytes);
			assert!(p.largest_free_region <= p.free_bytes);
		}
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
	// Release callback-owned CPU metadata and its device borrow after explicit
	// native close, before destroying the borrowed device.
	drop(allocator);
	// SAFETY: synthetic children/storage are explicitly closed first.
	unsafe {
		device.destroy(None);
		instance.destroy(None);
	}
}
