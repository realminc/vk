//! Independent exact-PFN traces for allocation admission, rollback and ownership.
use super::*;
use crate::load::{Capabilities, Entry};
use crate::memory::test_host_allocator::FAIL_NEXT_HOST_ALLOCATION;
use std::{
	cell::RefCell,
	ffi::{CStr, c_char, c_void},
};

#[derive(Default)]
struct State {
	events: Vec<&'static str>,
	blocked: Option<&'static [u8]>,
	fail: Option<&'static str>,
	next_memory: u64,
	next_buffer: u64,
	dedicated: vk::Buffer,
	required: bool,
	preferred: bool,
	bad_requirements: bool,
	limit: u32,
	size_limit: u64,
	queries: usize,
	pooled: bool,
	allocated: Vec<(u32, u64)>,
	bindings: Vec<(u64, u64)>,
	ranges: Vec<(u64, u64)>,
	fail_type: Option<u32>,
	requirements_size: Option<u64>,
	requirements_bits: Option<u32>,
}

thread_local! {
	static STATE: RefCell<State> = RefCell::new(State { next_memory: 100, next_buffer: 10, limit: 4, ..Default::default() });
}

fn event(name: &'static str) -> vk::Result {
	STATE.with_borrow_mut(|s| {
		s.events.push(name);
		if s.fail == Some(name) {
			vk::Result::ERROR_DEVICE_LOST
		} else {
			vk::Result::SUCCESS
		}
	})
}

fn events() -> Vec<&'static str> {
	STATE.with_borrow(|s| s.events.clone())
}

fn clear() {
	STATE.with_borrow_mut(|s| s.events.clear());
}

fn properties() -> vk::PhysicalDeviceMemoryProperties {
	// SAFETY: this output-only ABI record contains integer/flag fields and arrays.
	let mut p: vk::PhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
	p.memory_type_count = 1;
	p.memory_heap_count = 1;
	p.memory_types[0].property_flags = vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT;
	p.memory_heaps[0].size = 1 << 30;
	p
}

fn request() -> AllocRequest {
	AllocRequest {
		size: 257,
		alignment: 64,
		memory_type_bits: 1,
		required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
		preferred_flags: vk::MemoryPropertyFlags::empty(),
	}
}

fn buffer_info() -> vk::BufferCreateInfo {
	vk::BufferCreateInfo {
		size: 257,
		usage: vk::BufferUsageFlagBits::STORAGE_BUFFER_BIT,
		..Default::default()
	}
}

struct Fixture {
	instance: vk::Instance,
	device: Device,
}

impl Fixture {
	fn new(
		version: u32,
		extensions: &[&CStr],
		metadata: bool,
		blocked: Option<&'static [u8]>,
	) -> Self {
		Self::new_with_instance_extensions(version, extensions, &[], metadata, blocked)
	}

	fn new_with_instance_extensions(
		version: u32,
		extensions: &[&CStr],
		instance_extensions: &[&CStr],
		metadata: bool,
		blocked: Option<&'static [u8]>,
	) -> Self {
		STATE.with_borrow_mut(|s| {
			*s = State {
				next_memory: 100,
				next_buffer: 10,
				limit: 4,
				size_limit: 1 << 29,
				blocked,
				..Default::default()
			}
		});
		let app = vk::ApplicationInfo {
			api_version: version,
			..Default::default()
		};
		let instance_names: Vec<_> = instance_extensions
			.iter()
			.map(|name| name.as_ptr())
			.collect();
		let info = vk::InstanceCreateInfo {
			p_application_info: &app,
			enabled_extension_count: instance_names.len() as u32,
			pp_enabled_extension_names: instance_names.as_ptr(),
			..Default::default()
		};
		let names: Vec<_> = extensions.iter().map(|name| name.as_ptr()).collect();
		let dev_info = vk::DeviceCreateInfo {
			enabled_extension_count: names.len() as u32,
			pp_enabled_extension_names: names.as_ptr(),
			..Default::default()
		};
		// SAFETY: exact static mock PFNs and synthetic opaque handles only; all
		// reachable metadata is live for capture. No real native objects exist.
		unsafe {
			let parent = Capabilities::new_from_instance_create_info(&info, version).unwrap();
			let capabilities = parent
				.new_from_device_create_info(&dev_info, version)
				.unwrap();
			let entry = Entry::new_from_resolver(gipa).unwrap();
			let instance = vk::Instance::new_from_handle_with_capabilities(
				&entry,
				vk::InstanceHandle(std::ptr::without_provenance_mut(0x1000)),
				parent,
			)
			.unwrap();
			let handle = vk::DeviceHandle(std::ptr::without_provenance_mut(0x2000));
			let device = if metadata {
				Device::new_from_handle_with_capabilities(&instance, handle, capabilities).unwrap()
			} else {
				Device::new_from_handle(&instance, handle).unwrap()
			};
			Self { instance, device }
		}
	}

	fn allocator(&self) -> Allocator<'_> {
		// SAFETY: matching synthetic properties; no real driver calls.
		unsafe { Allocator::new(&self.device, properties(), 64) }.unwrap()
	}
}

macro_rules! pointer {
	($ty:ty, $function:ident) => {{
		// SAFETY: each match arm erases exactly the named command's native PFN.
		unsafe { std::mem::transmute::<$ty, vk::PFN_vkVoidFunction>(Some($function)) }
	}};
}

unsafe extern "system" fn gipa(
	_: vk::InstanceHandle,
	name: *const c_char,
) -> vk::PFN_vkVoidFunction {
	// SAFETY: table loaders supply valid NUL-terminated names.
	let name = unsafe { CStr::from_ptr(name) }.to_bytes();
	if STATE.with_borrow(|s| s.blocked == Some(name)) {
		return None;
	}
	match name {
		b"vkCreateInstance" => pointer!(vk::PFN_vkCreateInstance, create_instance),
		b"vkEnumerateInstanceExtensionProperties" => {
			pointer!(vk::PFN_vkEnumerateInstanceExtensionProperties, extensions)
		}
		b"vkEnumerateInstanceLayerProperties" => {
			pointer!(vk::PFN_vkEnumerateInstanceLayerProperties, layers)
		}
		b"vkDestroyInstance" => pointer!(vk::PFN_vkDestroyInstance, destroy_instance),
		b"vkGetDeviceProcAddr" => pointer!(vk::PFN_vkGetDeviceProcAddr, gdpa),
		b"vkGetPhysicalDeviceProperties2" => pointer!(
			vk::PFN_vkGetPhysicalDeviceProperties2,
			physical_properties2_core
		),
		b"vkGetPhysicalDeviceProperties2KHR" => pointer!(
			vk::PFN_vkGetPhysicalDeviceProperties2KHR,
			physical_properties2_khr
		),
		b"vkGetPhysicalDeviceProperties" => {
			pointer!(vk::PFN_vkGetPhysicalDeviceProperties, physical_properties)
		}
		b"vkGetPhysicalDeviceMemoryProperties" => pointer!(
			vk::PFN_vkGetPhysicalDeviceMemoryProperties,
			memory_properties
		),
		_ => None,
	}
}

unsafe extern "system" fn gdpa(_: vk::DeviceHandle, name: *const c_char) -> vk::PFN_vkVoidFunction {
	// SAFETY: table loaders supply valid NUL-terminated names.
	let name = unsafe { CStr::from_ptr(name) }.to_bytes();
	if STATE.with_borrow_mut(|s| {
		s.queries += 1;
		s.blocked == Some(name)
	}) {
		return None;
	}
	match name {
		b"vkDestroyDevice" => pointer!(vk::PFN_vkDestroyDevice, destroy_device),
		b"vkAllocateMemory" => pointer!(vk::PFN_vkAllocateMemory, allocate_memory),
		b"vkFreeMemory" => pointer!(vk::PFN_vkFreeMemory, free_memory),
		b"vkCreateBuffer" => pointer!(vk::PFN_vkCreateBuffer, create_native_buffer),
		b"vkDestroyBuffer" => pointer!(vk::PFN_vkDestroyBuffer, destroy_native_buffer),
		b"vkGetBufferMemoryRequirements" => {
			pointer!(vk::PFN_vkGetBufferMemoryRequirements, requirements)
		}
		b"vkGetBufferMemoryRequirements2" => {
			pointer!(vk::PFN_vkGetBufferMemoryRequirements2, requirements_core)
		}
		b"vkGetBufferMemoryRequirements2KHR" => {
			pointer!(vk::PFN_vkGetBufferMemoryRequirements2KHR, requirements_khr)
		}
		b"vkBindBufferMemory" => pointer!(vk::PFN_vkBindBufferMemory, bind),
		b"vkMapMemory" => pointer!(vk::PFN_vkMapMemory, map_memory),
		b"vkUnmapMemory" => pointer!(vk::PFN_vkUnmapMemory, unmap_memory),
		b"vkFlushMappedMemoryRanges" => pointer!(vk::PFN_vkFlushMappedMemoryRanges, flush_ranges),
		b"vkInvalidateMappedMemoryRanges" => {
			pointer!(vk::PFN_vkInvalidateMappedMemoryRanges, invalidate_ranges)
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
	event("destroy_instance");
}
unsafe extern "system" fn destroy_device(_: vk::DeviceHandle, _: *const vk::AllocationCallbacks) {
	event("destroy_device");
}

unsafe extern "system" fn physical_properties(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceProperties,
) {
	event("physical_properties");
	// SAFETY: the wrapper supplies one writable output record.
	unsafe {
		// The output-only ABI record has no Rust-invalid zero bit patterns.
		*output = std::mem::zeroed();
		(*output).limits.non_coherent_atom_size = 64;
		(*output).limits.max_memory_allocation_count = STATE.with_borrow(|s| s.limit);
	}
}

unsafe extern "system" fn memory_properties(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceMemoryProperties,
) {
	event("memory_properties");
	// SAFETY: one writable output supplied by the wrapper.
	unsafe {
		*output = properties();
	}
}

unsafe extern "system" fn allocate_memory(
	_: vk::DeviceHandle,
	info: *const vk::MemoryAllocateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::DeviceMemory,
) -> vk::Result {
	assert!(callbacks.is_null());
	let mut status = event("allocate");
	// SAFETY: the wrapper provides one initialized allocation record.
	unsafe {
		STATE.with_borrow_mut(|s| {
			if s.pooled {
				s.allocated
					.push(((*info).memory_type_index, (*info).allocation_size));
			}
			if s.fail_type == Some((*info).memory_type_index) {
				status = vk::Result::ERROR_OUT_OF_DEVICE_MEMORY;
			}
		});
	}
	// SAFETY: live synchronous input/output records, optionally one initialized dedicated record.
	unsafe {
		assert_eq!((*info).s_type, vk::StructureType::MEMORY_ALLOCATE_INFO);
		if !STATE.with_borrow(|s| s.pooled) {
			assert_eq!((*info).allocation_size, 257);
			assert_eq!((*info).memory_type_index, 0);
		}
		STATE.with_borrow_mut(|s| {
			s.dedicated = if (*info).p_next.is_null() {
				vk::Buffer::NULL
			} else {
				let dedication = &*(*info).p_next.cast::<vk::MemoryDedicatedAllocateInfo>();
				assert_eq!(
					dedication.s_type,
					vk::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO
				);
				assert!(dedication.image.is_null());
				assert!(dedication.p_next.is_null());
				dedication.buffer
			};
			if status == vk::Result::SUCCESS {
				*output = if s.fail == Some("null_memory") {
					vk::DeviceMemory::NULL
				} else {
					vk::DeviceMemory::from_raw(s.next_memory)
				};
				s.next_memory += 1;
			}
		});
	}
	status
}

unsafe extern "system" fn free_memory(
	_: vk::DeviceHandle,
	_: vk::DeviceMemory,
	callbacks: *const vk::AllocationCallbacks,
) {
	assert!(callbacks.is_null());
	event("free");
}

unsafe extern "system" fn create_native_buffer(
	_: vk::DeviceHandle,
	_: *const vk::BufferCreateInfo,
	callbacks: *const vk::AllocationCallbacks,
	output: *mut vk::Buffer,
) -> vk::Result {
	assert!(callbacks.is_null());
	let status = event("create");
	if status == vk::Result::SUCCESS {
		// SAFETY: wrapper supplies one writable output handle.
		unsafe {
			STATE.with_borrow_mut(|s| {
				*output = if s.fail == Some("null_buffer") {
					vk::Buffer::NULL
				} else {
					vk::Buffer::from_raw(s.next_buffer)
				};
				s.next_buffer += 1;
			});
		}
	}
	status
}
unsafe extern "system" fn destroy_native_buffer(
	_: vk::DeviceHandle,
	_: vk::Buffer,
	callbacks: *const vk::AllocationCallbacks,
) {
	assert!(callbacks.is_null());
	event("destroy_buffer");
}

fn native_requirements() -> vk::MemoryRequirements {
	vk::MemoryRequirements {
		size: if STATE.with_borrow(|s| s.bad_requirements) {
			0
		} else {
			STATE.with_borrow(|s| s.requirements_size.unwrap_or(257))
		},
		alignment: 64,
		memory_type_bits: STATE.with_borrow(|s| s.requirements_bits.unwrap_or(1)),
	}
}
unsafe extern "system" fn requirements(
	_: vk::DeviceHandle,
	_: vk::Buffer,
	output: *mut vk::MemoryRequirements,
) {
	event("requirements_legacy");
	// SAFETY: wrapper provides one writable output record.
	unsafe {
		*output = native_requirements();
	}
}
unsafe extern "system" fn requirements_core(
	_: vk::DeviceHandle,
	info: *const vk::BufferMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	event("requirements_core");
	// SAFETY: forwards the exact same live records to the common mock body.
	unsafe {
		requirements2(info, output);
	}
}
unsafe extern "system" fn requirements_khr(
	_: vk::DeviceHandle,
	info: *const vk::BufferMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	event("requirements_khr");
	// SAFETY: forwards the exact same live records to the common mock body.
	unsafe {
		requirements2(info, output);
	}
}
unsafe fn requirements2(
	info: *const vk::BufferMemoryRequirementsInfo2,
	output: *mut vk::MemoryRequirements2,
) {
	// SAFETY: allocator initializes the full output chain before querying the mock.
	unsafe {
		assert_eq!(
			(*info).s_type,
			vk::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2
		);
		assert!(!(*info).buffer.is_null());
		assert_eq!((*output).s_type, vk::StructureType::MEMORY_REQUIREMENTS_2);
		let dedicated = &mut *(*output).p_next.cast::<vk::MemoryDedicatedRequirements>();
		assert_eq!(
			dedicated.s_type,
			vk::StructureType::MEMORY_DEDICATED_REQUIREMENTS
		);
		assert!(dedicated.p_next.is_null());
		dedicated.requires_dedicated_allocation = STATE.with_borrow(|s| u32::from(s.required));
		dedicated.prefers_dedicated_allocation = STATE.with_borrow(|s| u32::from(s.preferred));
		(*output).memory_requirements = native_requirements();
	}
	if STATE.with_borrow(|s| s.fail == Some("host_after_requirements")) {
		FAIL_NEXT_HOST_ALLOCATION.set(true);
	}
}
unsafe extern "system" fn bind(
	_: vk::DeviceHandle,
	buffer: vk::Buffer,
	_: vk::DeviceMemory,
	offset: u64,
) -> vk::Result {
	assert!(!buffer.is_null());
	if STATE.with_borrow(|s| s.pooled) {
		STATE.with_borrow_mut(|s| s.bindings.push((buffer.as_raw(), offset)));
	} else {
		assert_eq!(offset, 0);
	}
	event("bind")
}
unsafe extern "system" fn map_memory(
	_: vk::DeviceHandle,
	_: vk::DeviceMemory,
	offset: u64,
	size: u64,
	flags: vk::MemoryMapFlags,
	output: *mut *mut c_void,
) -> vk::Result {
	assert_eq!(offset, 0);
	assert_eq!(flags.as_raw(), 0);
	if !STATE.with_borrow(|s| s.pooled) {
		assert_eq!(size, 257);
	}
	let status = event("map");
	if status == vk::Result::SUCCESS {
		// SAFETY: one output pointer; synthetic pointer is never dereferenced.
		unsafe {
			*output = if STATE.with_borrow(|s| s.fail == Some("null_map")) {
				std::ptr::null_mut()
			} else {
				std::ptr::without_provenance_mut(0x4000)
			};
		}
	}
	status
}
unsafe extern "system" fn unmap_memory(_: vk::DeviceHandle, _: vk::DeviceMemory) {
	event("unmap");
}
unsafe extern "system" fn flush_ranges(
	_: vk::DeviceHandle,
	count: u32,
	ranges: *const vk::MappedMemoryRange,
) -> vk::Result {
	assert_eq!(count, 1);
	// SAFETY: one initialized record provided by allocator.
	unsafe {
		if STATE.with_borrow(|s| s.pooled) {
			STATE.with_borrow_mut(|s| s.ranges.push(((*ranges).offset, (*ranges).size)));
		} else {
			assert_eq!(((*ranges).offset, (*ranges).size), (64, 64));
		}
	}
	event("flush")
}
unsafe extern "system" fn invalidate_ranges(
	_: vk::DeviceHandle,
	count: u32,
	ranges: *const vk::MappedMemoryRange,
) -> vk::Result {
	assert_eq!(count, 1);
	// SAFETY: one initialized record provided by allocator.
	unsafe {
		if STATE.with_borrow(|s| s.pooled) {
			STATE.with_borrow_mut(|s| s.ranges.push(((*ranges).offset, (*ranges).size)));
		} else {
			assert_eq!(((*ranges).offset, (*ranges).size), (64, 64));
		}
	}
	event("invalidate")
}

#[test]
fn buffer_routes_honor_required_preferred_and_optional_dedication() {
	for (version, extensions, expected) in [
		(vk::API_VERSION_1_0, vec![], "requirements_legacy"),
		(vk::API_VERSION_1_1, vec![], "requirements_core"),
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_get_memory_requirements2",
				c"VK_KHR_dedicated_allocation",
			],
			"requirements_khr",
		),
	] {
		for (required, preferred) in [(true, false), (false, true), (false, false)] {
			let fixture = Fixture::new(version, &extensions, true, None);
			STATE.with_borrow_mut(|s| {
				s.required = required;
				s.preferred = preferred;
			});
			let mut allocator = fixture.allocator();
			let queries = STATE.with_borrow(|s| s.queries);
			// SAFETY: synthetic resources never reach a driver; exact PFN mocks inspect chains.
			unsafe {
				let (buffer, mut allocation) = create_buffer(
					&mut allocator,
					&buffer_info(),
					request().required_flags,
					request().preferred_flags,
				)
				.unwrap();
				assert_eq!(events(), ["create", expected, "allocate", "bind"]);
				assert_eq!(
					STATE.with_borrow(|s| s.dedicated),
					if expected == "requirements_legacy" {
						vk::Buffer::NULL
					} else {
						buffer
					}
				);
				assert_eq!(close(&mut allocator), Err(AllocError::LiveAllocations));
				allocator.destroy_buffer(buffer, &mut allocation).unwrap();
				assert!(allocation.is_freed());
				assert_eq!(&events()[4..], ["destroy_buffer", "free"]);
				allocator.close().unwrap();
				allocator.close().unwrap();
				assert!(allocator.is_closed());
				assert!(matches!(
					allocator.allocate(&request()),
					Err(AllocError::Closed)
				));
				assert!(matches!(
					allocator.create_buffer(
						&buffer_info(),
						request().required_flags,
						request().preferred_flags
					),
					Err(AllocError::Closed)
				));
			}
			assert_eq!(STATE.with_borrow(|s| s.queries), queries);
		}
	}
}

#[test]
fn managed_buffer_failures_rollback_only_new_objects_and_preserve_initiating_error() {
	for (fail, expected) in [
		("create", vec!["create"]),
		(
			"allocate",
			vec!["create", "requirements_core", "allocate", "destroy_buffer"],
		),
		(
			"bind",
			vec![
				"create",
				"requirements_core",
				"allocate",
				"bind",
				"destroy_buffer",
				"free",
			],
		),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		// SAFETY: mock-only handles, used only by exact test PFNs.
		unsafe {
			let mut existing = allocator.allocate(&request()).unwrap();
			clear();
			STATE.with_borrow_mut(|s| s.fail = Some(fail));
			assert!(matches!(
				allocator.create_buffer(
					&buffer_info(),
					request().required_flags,
					request().preferred_flags
				),
				Err(AllocError::Call(CallError::Vulkan(
					vk::Result::ERROR_DEVICE_LOST
				)))
			));
			assert_eq!(events(), expected);
			assert_eq!(allocator.live_count(), 1);
			STATE.with_borrow_mut(|s| s.fail = None);
			allocator.free(&mut existing).unwrap();
		}
	}
}

#[test]
fn invalid_requirements_and_no_compatible_type_destroy_the_new_buffer() {
	for bad_size in [true, false] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		STATE.with_borrow_mut(|s| s.bad_requirements = bad_size);
		// SAFETY: synthetic ordinary buffer and property flags only.
		let result = unsafe {
			allocator.create_buffer(
				&buffer_info(),
				if bad_size {
					request().required_flags
				} else {
					vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT
				},
				request().preferred_flags,
			)
		};
		if bad_size {
			assert!(matches!(result, Err(AllocError::InvalidRequest(_))));
		} else {
			assert!(matches!(result, Err(AllocError::NoCompatibleMemoryType)));
		}
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(allocator.live_count(), 0);
	}
}

#[test]
fn unavailable_native_transitions_are_rejected_before_creation_or_mapping() {
	for (blocked, expected) in [
		(b"vkCreateBuffer".as_slice(), "vkCreateBuffer"),
		(b"vkDestroyBuffer".as_slice(), "vkDestroyBuffer"),
		(b"vkBindBufferMemory".as_slice(), "vkBindBufferMemory"),
		(
			b"vkGetBufferMemoryRequirements2".as_slice(),
			"vkGetBufferMemoryRequirements2",
		),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, Some(blocked));
		let mut allocator = fixture.allocator();
		// SAFETY: synthetic buffer parameters only; missing PFN prevents effects.
		assert!(
			matches!(unsafe { allocator.create_buffer(&buffer_info(), request().required_flags, request().preferred_flags) }, Err(AllocError::Call(CallError::MissingCommand(name))) if name == expected)
		);
		assert!(events().is_empty());
	}
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, Some(b"vkUnmapMemory"));
	let mut allocator = fixture.allocator();
	// SAFETY: mock-only allocation; unavailable inverse transition prevents native map.
	unsafe {
		let mut allocation = allocator.allocate(&request()).unwrap();
		clear();
		assert!(matches!(
			allocator.map(&allocation),
			Err(AllocError::Call(CallError::MissingCommand("vkUnmapMemory")))
		));
		assert!(events().is_empty());
		allocator.free(&mut allocation).unwrap();
	}
}

#[test]
fn unknown_creation_provenance_cannot_silently_use_legacy_requirements() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], false, None);
	let mut allocator = fixture.allocator();
	// SAFETY: mock-only ordinary buffer; metadata rejection occurs before effects.
	assert!(matches!(
		unsafe {
			allocator.create_buffer(
				&buffer_info(),
				request().required_flags,
				request().preferred_flags,
			)
		},
		Err(AllocError::UnsupportedCapability(_))
	));
	assert!(events().is_empty());
}

#[test]
fn mapping_failures_and_invalid_states_preserve_liveness() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: mocked pointers are never dereferenced; only ownership/state transitions tested.
	unsafe {
		let mut allocation = allocate(&mut allocator, &request()).unwrap();
		assert_eq!(
			unmap(&mut allocator, &allocation),
			Err(AllocError::NotMapped)
		);
		STATE.with_borrow_mut(|s| s.fail = Some("map"));
		clear();
		assert!(matches!(
			map(&mut allocator, &allocation),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_DEVICE_LOST
			)))
		));
		assert_eq!(events(), ["map"]);
		assert_eq!(allocator.unmap(&allocation), Err(AllocError::NotMapped));
		STATE.with_borrow_mut(|s| s.fail = Some("null_map"));
		clear();
		assert!(matches!(
			allocator.map(&allocation),
			Err(AllocError::Call(CallError::InvalidOutput(_)))
		));
		assert_eq!(events(), ["map", "unmap"]);
		STATE.with_borrow_mut(|s| s.fail = None);
		clear();
		map(&mut allocator, &allocation).unwrap();
		assert!(matches!(
			allocator.map(&allocation),
			Err(AllocError::AlreadyMapped)
		));
		assert_eq!(
			free(&mut allocator, &mut allocation),
			Err(AllocError::AlreadyMapped)
		);
		flush(&allocator, &allocation, 65, 2).unwrap();
		allocator.invalidate(&allocation, 65, 2).unwrap();
		assert_eq!(events(), ["map", "flush", "invalidate"]);
		unmap(&mut allocator, &allocation).unwrap();
		free(&mut allocator, &mut allocation).unwrap();
		clear();
		assert_eq!(
			allocator.free(&mut allocation),
			Err(AllocError::UnknownAllocation)
		);
		assert!(matches!(
			allocator.map(&allocation),
			Err(AllocError::UnknownAllocation)
		));
		assert_eq!(
			allocator.flush(&allocation, 0, 0),
			Err(AllocError::UnknownAllocation)
		);
		assert!(events().is_empty());
	}
}

#[test]
fn wrong_allocator_stale_generation_and_resource_pair_are_rejected_without_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	let mut other = fixture.allocator();
	// SAFETY: synthetic identities and mock PFNs; no native memory exists.
	unsafe {
		let (buffer, mut allocation) = allocator
			.create_buffer(
				&buffer_info(),
				request().required_flags,
				request().preferred_flags,
			)
			.unwrap();
		clear();
		assert_eq!(other.free(&mut allocation), Err(AllocError::WrongAllocator));
		assert_eq!(
			allocator.destroy_buffer(vk::Buffer::from_raw(buffer.as_raw() + 1), &mut allocation),
			Err(AllocError::WrongResource)
		);
		let serial = allocation.serial;
		allocation.serial += 1;
		assert_eq!(
			allocator.free(&mut allocation),
			Err(AllocError::UnknownAllocation)
		);
		allocation.serial = serial;
		assert!(events().is_empty());
		allocator.map(&allocation).unwrap();
		clear();
		assert_eq!(
			allocator.destroy_buffer(buffer, &mut allocation),
			Err(AllocError::AlreadyMapped)
		);
		assert!(events().is_empty());
		allocator.unmap(&allocation).unwrap();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		// Reuse a native handle but retain generation separation from the old token.
		STATE.with_borrow_mut(|s| s.next_memory = 100);
		let mut current = allocator.allocate(&request()).unwrap();
		allocation.handle = current.handle;
		clear();
		assert_eq!(
			allocator.free(&mut allocation),
			Err(AllocError::UnknownAllocation)
		);
		assert!(events().is_empty());
		allocator.free(&mut current).unwrap();
	}
}

#[test]
fn queried_constructor_enforces_local_count_limit_and_explicit_close() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.limit = 1);
	// SAFETY: fixture mock properties correspond to these synthetic contexts.
	unsafe {
		let physical = vk::PhysicalDevice::from_raw(0x3000);
		let mut allocator =
			Allocator::new_from_physical_device(&fixture.instance, physical, &fixture.device).unwrap();
		assert_eq!(
			events(),
			[
				"physical_properties",
				"properties2_core",
				"memory_properties"
			]
		);
		let mut allocation = allocator.allocate(&request()).unwrap();
		clear();
		assert!(matches!(
			allocator.allocate(&request()),
			Err(AllocError::AllocationLimit)
		));
		assert!(matches!(
			allocator.create_buffer(
				&buffer_info(),
				request().required_flags,
				request().preferred_flags
			),
			Err(AllocError::AllocationLimit)
		));
		assert!(events().is_empty());
		allocator.free(&mut allocation).unwrap();
		allocator.close().unwrap();
		assert!(allocator.is_closed());
	}
}

#[test]
fn serial_exhaustion_and_unsupported_requests_precede_native_effects() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: only test mocks exist; all failures must precede native effects.
	unsafe {
		allocator.next_serial = u64::MAX;
		assert!(matches!(
			allocator.allocate(&request()),
			Err(AllocError::SerialExhausted)
		));
		assert!(matches!(
			allocator.create_buffer(
				&buffer_info(),
				request().required_flags,
				request().preferred_flags
			),
			Err(AllocError::SerialExhausted)
		));
		allocator.next_serial = 1;
		let mut info = buffer_info();
		info.flags = vk::BufferCreateFlagBits::SPARSE_BINDING_BIT;
		assert!(matches!(
			allocator.create_buffer(&info, request().required_flags, request().preferred_flags),
			Err(AllocError::InvalidRequest(_))
		));
		info = buffer_info();
		info.usage = vk::BufferUsageFlagBits::SHADER_DEVICE_ADDRESS_BIT;
		assert!(matches!(
			allocator.create_buffer(&info, request().required_flags, request().preferred_flags),
			Err(AllocError::InvalidRequest(_))
		));
		let mut protected = request();
		protected.required_flags = vk::MemoryPropertyFlagBits::PROTECTED_BIT;
		assert!(matches!(
			allocator.allocate(&protected),
			Err(AllocError::UnsupportedCapability(_))
		));
		assert!(events().is_empty());
	}
}

#[test]
fn drop_performs_no_native_destruction_or_unmapping() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: deliberately abandoned synthetic objects have no native resources.
	let allocation = unsafe { allocator.allocate(&request()) }.unwrap();
	clear();
	drop(allocation);
	drop(allocator);
	drop(fixture);
	assert!(events().is_empty());
}

#[test]
fn missing_constructor_functions_and_exact_khr_query_fail_before_effects() {
	for (blocked, expected) in [
		(b"vkAllocateMemory".as_slice(), "vkAllocateMemory"),
		(b"vkFreeMemory".as_slice(), "vkFreeMemory"),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, Some(blocked));
		// SAFETY: synthetic physical metadata; constructor rejects the missing PFN.
		assert!(
			matches!(unsafe { Allocator::new(&fixture.device, properties(), 64) }, Err(AllocError::Call(CallError::MissingCommand(name))) if name == expected)
		);
		assert!(events().is_empty());
	}
	for (version, extensions, blocked, expected) in [
		(
			vk::API_VERSION_1_0,
			vec![
				c"VK_KHR_get_memory_requirements2",
				c"VK_KHR_dedicated_allocation",
			],
			b"vkGetBufferMemoryRequirements2KHR".as_slice(),
			"vkGetBufferMemoryRequirements2KHR",
		),
		(
			vk::API_VERSION_1_0,
			vec![],
			b"vkGetBufferMemoryRequirements".as_slice(),
			"vkGetBufferMemoryRequirements",
		),
	] {
		let fixture = Fixture::new(version, &extensions, true, Some(blocked));
		let mut allocator = fixture.allocator();
		// SAFETY: only mock PFNs can be reached; other aliases are deliberately present.
		assert!(
			matches!(unsafe { allocator.create_buffer(&buffer_info(), request().required_flags, request().preferred_flags) }, Err(AllocError::Call(CallError::MissingCommand(name))) if name == expected)
		);
		assert!(events().is_empty());
	}
}

#[test]
fn invalid_native_success_outputs_are_not_published() {
	for (fail, expected) in [
		("null_buffer", vec!["create"]),
		(
			"null_memory",
			vec!["create", "requirements_core", "allocate", "destroy_buffer"],
		),
	] {
		let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
		let mut allocator = fixture.allocator();
		STATE.with_borrow_mut(|s| s.fail = Some(fail));
		// SAFETY: mock-only outputs; NULL successful handles are intentionally invalid.
		assert!(matches!(
			unsafe {
				allocator.create_buffer(
					&buffer_info(),
					request().required_flags,
					request().preferred_flags,
				)
			},
			Err(AllocError::Call(CallError::InvalidOutput(_)))
		));
		assert_eq!(allocator.live_count(), 0);
		assert_eq!(events(), expected);
	}
}

#[test]
fn flush_and_invalidate_failures_keep_the_mapping_live() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	// SAFETY: synthetic mapping is never dereferenced; exact mock native PFNs only.
	unsafe {
		let mut allocation = allocator.allocate(&request()).unwrap();
		allocator.map(&allocation).unwrap();
		STATE.with_borrow_mut(|s| s.fail = Some("flush"));
		assert_eq!(
			allocator.flush(&allocation, 65, 2),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_DEVICE_LOST
			)))
		);
		STATE.with_borrow_mut(|s| s.fail = Some("invalidate"));
		assert_eq!(
			allocator.invalidate(&allocation, 65, 2),
			Err(AllocError::Call(CallError::Vulkan(
				vk::Result::ERROR_DEVICE_LOST
			)))
		);
		assert_eq!(
			allocator.free(&mut allocation),
			Err(AllocError::AlreadyMapped)
		);
		STATE.with_borrow_mut(|s| s.fail = None);
		allocator.unmap(&allocation).unwrap();
		allocator.free(&mut allocation).unwrap();
	}
}

#[test]
fn unknown_imports_retain_low_level_allocation_but_not_queried_construction() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], false, None);
	// SAFETY: synthetic contexts; provenance rejection must precede property queries.
	assert!(matches!(
		unsafe {
			Allocator::new_from_physical_device(
				&fixture.instance,
				vk::PhysicalDevice::from_raw(0x3000),
				&fixture.device,
			)
		},
		Err(AllocError::UnsupportedCapability(_))
	));
	assert!(events().is_empty());
	let mut allocator = fixture.allocator();
	// SAFETY: explicitly supplied ordinary low-level requirements and mock metadata.
	unsafe {
		let mut allocation = allocator.allocate(&request()).unwrap();
		assert!(STATE.with_borrow(|s| s.dedicated.is_null()));
		allocator.free(&mut allocation).unwrap();
	}
}

unsafe extern "system" fn physical_properties2_core(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceProperties2,
) {
	event("properties2_core");
	// SAFETY: forwards the live initialized property output chain.
	unsafe { fill_properties2(output) };
}

unsafe extern "system" fn physical_properties2_khr(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceProperties2,
) {
	event("properties2_khr");
	// SAFETY: forwards the live initialized property output chain.
	unsafe { fill_properties2(output) };
}

unsafe fn fill_properties2(output: *mut vk::PhysicalDeviceProperties2) {
	// SAFETY: the allocator provides both exact typed initialized records.
	unsafe {
		assert_eq!(
			(*output).s_type,
			vk::StructureType::PHYSICAL_DEVICE_PROPERTIES_2
		);
		let limits = &mut *(*output)
			.p_next
			.cast::<vk::PhysicalDeviceMaintenance3Properties>();
		assert_eq!(
			limits.s_type,
			vk::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES
		);
		assert!(limits.p_next.is_null());
		limits.max_memory_allocation_size = STATE.with_borrow(|s| s.size_limit);
	}
}

#[test]
fn actual_host_bookkeeping_oom_precedes_native_effects_and_preserves_existing_allocations() {
	for managed in [false, true] {
		for existing in [false, true] {
			let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
			let mut allocator = fixture.allocator();
			let mut allocations = Vec::new();
			// SAFETY: mock-only native storage and buffer handles; no real GPU resources.
			unsafe {
				if existing {
					allocations.push(allocator.allocate(&request()).unwrap());
					// Fill the current table so that exactly one more token requires
					// a real reservation. Do not depend on a particular growth factor.
					while allocator.live.len() < allocator.live.capacity() {
						allocations.push(allocator.allocate(&request()).unwrap());
					}
				}
				clear();
				// A regression that calls a native mock first must produce a trace,
				// rather than accidentally triggering Vec's infallible allocation.
				STATE.with_borrow_mut(|s| s.events.reserve(32));
				let serial = allocator.next_serial;
				let live = allocator.live_count();
				FAIL_NEXT_HOST_ALLOCATION.set(true);
				let failed = if managed {
					allocator
						.create_buffer(
							&buffer_info(),
							request().required_flags,
							request().preferred_flags,
						)
						.map(|_| ())
				} else {
					allocator.allocate(&request()).map(|_| ())
				};
				let injected = !FAIL_NEXT_HOST_ALLOCATION.replace(false);
				assert!(
					injected,
					"the native path must actually attempt host reservation"
				);
				assert_eq!(failed, Err(AllocError::HostAllocationFailed));
				assert!(events().is_empty());
				assert_eq!(allocator.next_serial, serial);
				assert_eq!(allocator.live_count(), live);
				// Failure does not poison the allocator: retry succeeds, then all
				// retained identities remain usable for ordinary explicit cleanup.
				if managed {
					let (buffer, mut allocation) = allocator
						.create_buffer(
							&buffer_info(),
							request().required_flags,
							request().preferred_flags,
						)
						.unwrap();
					allocator.destroy_buffer(buffer, &mut allocation).unwrap();
				} else {
					let mut allocation = allocator.allocate(&request()).unwrap();
					allocator.free(&mut allocation).unwrap();
				}
				for allocation in &mut allocations {
					allocator.free(allocation).unwrap();
				}
				allocator.close().unwrap();
			}
		}
	}
}

#[test]
fn allocation_size_query_uses_exact_core_khr_or_explicitly_unknown_legacy_route() {
	for (version, device_extensions, instance_extensions, expected) in [
		(
			vk::API_VERSION_1_1,
			vec![],
			vec![],
			Some("properties2_core"),
		),
		(
			vk::API_VERSION_1_0,
			vec![c"VK_KHR_maintenance3"],
			vec![c"VK_KHR_get_physical_device_properties2"],
			Some("properties2_khr"),
		),
		(
			vk::API_VERSION_1_0,
			vec![],
			vec![c"VK_KHR_get_physical_device_properties2"],
			None,
		),
		(vk::API_VERSION_1_0, vec![], vec![], None),
	] {
		let fixture = Fixture::new_with_instance_extensions(
			version,
			&device_extensions,
			&instance_extensions,
			true,
			None,
		);
		STATE.with_borrow_mut(|s| s.size_limit = (1 << 40) + 257);
		// SAFETY: all properties belong to synthetic contexts; mock validates exact output chain.
		let mut allocator = unsafe {
			Allocator::new_from_physical_device(
				&fixture.instance,
				vk::PhysicalDevice::from_raw(0x3000),
				&fixture.device,
			)
		}
		.unwrap();
		assert_eq!(
			allocator.max_allocation_size(),
			expected.map(|_| (1 << 40) + 257)
		);
		let trace = events();
		assert_eq!(
			trace,
			match expected {
				Some(name) => vec!["physical_properties", name, "memory_properties"],
				None => vec!["physical_properties", "memory_properties"],
			}
		);
		if let Some(limit) = allocator.max_allocation_size() {
			let mut too_large = request();
			too_large.size = limit + 1;
			clear();
			// SAFETY: size validation rejects this request before any native call.
			assert!(
				matches!(unsafe { allocator.allocate(&too_large) }, Err(AllocError::AllocationTooLarge { requested, limit: ceiling }) if requested == limit + 1 && ceiling == limit)
			);
			too_large.size = u64::MAX;
			// SAFETY: even the largest representable request fails before native effects.
			assert!(
				matches!(unsafe { allocator.allocate(&too_large) }, Err(AllocError::AllocationTooLarge { requested: u64::MAX, limit: ceiling }) if ceiling == limit)
			);
			assert!(events().is_empty());
		}
		assert_eq!(fixture.allocator().max_allocation_size(), None);
	}
}

#[test]
fn allocation_size_limits_cover_exact_boundary_preflight_and_requirement_growth_rollback() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	// SAFETY: mock-only properties and allocations; caller does not dereference handles.
	unsafe {
		STATE.with_borrow_mut(|s| s.size_limit = 257);
		let mut allocator = Allocator::new_from_physical_device(
			&fixture.instance,
			vk::PhysicalDevice::from_raw(0x3000),
			&fixture.device,
		)
		.unwrap();
		let mut allocation = allocator.allocate(&request()).unwrap();
		allocator.free(&mut allocation).unwrap();
		let (buffer, mut allocation) = allocator
			.create_buffer(
				&buffer_info(),
				request().required_flags,
				request().preferred_flags,
			)
			.unwrap();
		allocator.destroy_buffer(buffer, &mut allocation).unwrap();
		let mut too_large = request();
		too_large.size = 258;
		clear();
		assert!(matches!(
			allocator.allocate(&too_large),
			Err(AllocError::AllocationTooLarge {
				requested: 258,
				limit: 257
			})
		));
		let mut info = buffer_info();
		info.size = 258;
		assert!(matches!(
			allocator.create_buffer(&info, request().required_flags, request().preferred_flags),
			Err(AllocError::AllocationTooLarge {
				requested: 258,
				limit: 257
			})
		));
		assert!(events().is_empty());
		// Native requirements can exceed the requested logical resource size.
		allocator.allocation_size_limit = Some(256);
		info.size = 128;
		assert!(matches!(
			allocator.create_buffer(&info, request().required_flags, request().preferred_flags),
			Err(AllocError::AllocationTooLarge {
				requested: 257,
				limit: 256
			})
		));
		assert_eq!(events(), ["create", "requirements_core", "destroy_buffer"]);
		assert_eq!(allocator.live_count(), 0);
		allocator.close().unwrap();
	}
}

#[test]
fn size_query_missing_pfn_invalid_metadata_and_zero_limit_cannot_publish_an_allocator() {
	for (version, extensions, instance_extensions, blocked, expected) in [
		(
			vk::API_VERSION_1_1,
			vec![],
			vec![c"VK_KHR_get_physical_device_properties2"],
			b"vkGetPhysicalDeviceProperties2".as_slice(),
			"vkGetPhysicalDeviceProperties2",
		),
		(
			vk::API_VERSION_1_0,
			vec![c"VK_KHR_maintenance3"],
			vec![c"VK_KHR_get_physical_device_properties2"],
			b"vkGetPhysicalDeviceProperties2KHR".as_slice(),
			"vkGetPhysicalDeviceProperties2KHR",
		),
	] {
		let fixture = Fixture::new_with_instance_extensions(
			version,
			&extensions,
			&instance_extensions,
			true,
			Some(blocked),
		);
		// SAFETY: exact synthetic physical metadata; the opposite alias remains resolvable.
		assert!(
			matches!(unsafe { Allocator::new_from_physical_device(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000), &fixture.device) }, Err(AllocError::Call(CallError::MissingCommand(name))) if name == expected)
		);
		assert_eq!(events(), ["physical_properties"]);
	}
	let fixture = Fixture::new(vk::API_VERSION_1_0, &[c"VK_KHR_maintenance3"], true, None);
	// SAFETY: intentionally inconsistent enabled-extension metadata must fail closed.
	assert!(matches!(
		unsafe {
			Allocator::new_from_physical_device(
				&fixture.instance,
				vk::PhysicalDevice::from_raw(0x3000),
				&fixture.device,
			)
		},
		Err(AllocError::UnsupportedCapability(_))
	));
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	STATE.with_borrow_mut(|s| s.size_limit = 0);
	// SAFETY: the mock intentionally supplies an invalid zero native limit.
	assert!(matches!(
		unsafe {
			Allocator::new_from_physical_device(
				&fixture.instance,
				vk::PhysicalDevice::from_raw(0x3000),
				&fixture.device,
			)
		},
		Err(AllocError::InvalidProperties(_))
	));
}

#[path = "test_pooled.rs"]
mod pooled;
