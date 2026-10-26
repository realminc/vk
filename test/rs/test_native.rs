//! Independent output-storage contracts for commands beyond the native GPU profile.
use super::command::Fixture;
use std::{
	cell::{Cell, RefCell},
	ffi::c_void,
};
use vk::{self, CallError};

thread_local! {
	static TRACE: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
	static STATUS: Cell<vk::Result> = const { Cell::new(vk::Result::SUCCESS) };
}

unsafe extern "system" fn bind_vertices(
	command: vk::CommandBuffer,
	first: u32,
	count: u32,
	buffers: *const vk::Buffer,
	offsets: *const u64,
	sizes: *const u64,
	strides: *const u64,
) {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			command.as_raw() as usize,
			first as usize,
			count as usize,
			buffers as usize,
			offsets as usize,
			sizes as usize,
			strides as usize,
		])
	});
}

#[test]
fn vertex_buffers2_preserves_independent_optional_arrays_and_missing_alias() {
	// SAFETY: the resolver supplies this exact prototype; the mock only records pointers.
	let fixture = unsafe {
		Fixture::new_with_command(
			c"vkCmdBindVertexBuffers2",
			std::mem::transmute::<vk::PFN_vkCmdBindVertexBuffers2, vk::PFN_vkVoidFunction>(Some(
				bind_vertices,
			)),
		)
	};
	TRACE.with_borrow_mut(Vec::clear);
	let buffers = [vk::Buffer::from_raw(17), vk::Buffer::from_raw(19)];
	let offsets = [u64::MAX, 1 << 40];
	let sizes = [u64::MAX, 0];
	let strides = [0, u64::MAX];
	let command = vk::CommandBuffer::from_raw(0x8000);
	let resolutions = Fixture::query_count();
	// SAFETY: only the pointer-recording mock is called, never a native driver.
	unsafe {
		vk::cmd_bind_vertex_buffers2(
			&fixture.device,
			command,
			3,
			2,
			buffers.as_ptr(),
			offsets.as_ptr(),
			std::ptr::null(),
			strides.as_ptr(),
		)
		.unwrap();
		fixture
			.device
			.cmd_bind_vertex_buffers2(
				command,
				5,
				2,
				buffers.as_ptr(),
				offsets.as_ptr(),
				sizes.as_ptr(),
				std::ptr::null(),
			)
			.unwrap();
		assert_eq!(
			fixture.device.cmd_bind_vertex_buffers2_ext(
				command,
				0,
				0,
				std::ptr::null(),
				std::ptr::null(),
				std::ptr::null(),
				std::ptr::null()
			),
			Err(CallError::MissingCommand("vkCmdBindVertexBuffers2EXT"))
		);
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace.as_slice(),
			[
				0x8000,
				3,
				2,
				buffers.as_ptr() as usize,
				offsets.as_ptr() as usize,
				0,
				strides.as_ptr() as usize,
				0x8000,
				5,
				2,
				buffers.as_ptr() as usize,
				offsets.as_ptr() as usize,
				sizes.as_ptr() as usize,
				0
			]
		)
	});
	assert_eq!(Fixture::query_count(), resolutions);
}

unsafe extern "system" fn sparse_requirements(
	device: vk::DeviceHandle,
	info: *const vk::DeviceImageMemoryRequirements,
	count: *mut u32,
	outputs: *mut vk::SparseImageMemoryRequirements2,
) {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			device.as_raw() as usize,
			info as usize,
			count as usize,
			outputs as usize,
		])
	});
	// SAFETY: this test supplies a live count and either NULL or two initialized records.
	unsafe {
		if !outputs.is_null() {
			(*outputs).memory_requirements.image_mip_tail_size = u64::MAX;
			(*outputs.add(1)).memory_requirements.image_mip_tail_size = 1 << 40;
		}
		*count = 2;
	}
}

#[test]
fn sparse_queries_keep_caller_initialized_headers_chains_and_explicit_count_protocol() {
	// SAFETY: the exact-ABI resolver supplies a mock with test-owned writable storage.
	let fixture = unsafe {
		Fixture::new_with_command(
			c"vkGetDeviceImageSparseMemoryRequirements",
			std::mem::transmute::<vk::PFN_vkGetDeviceImageSparseMemoryRequirements, vk::PFN_vkVoidFunction>(
				Some(sparse_requirements),
			),
		)
	};
	TRACE.with_borrow_mut(Vec::clear);
	let info = vk::DeviceImageMemoryRequirements::default();
	let mut chains = [7_u64, 11];
	let mut values =
		std::array::from_fn::<_, 2, _>(|_| vk::SparseImageMemoryRequirements2::default());
	for (value, chain) in values.iter_mut().zip(&mut chains) {
		value.p_next = std::ptr::from_mut(chain).cast();
	}
	let pointers = [values[0].p_next, values[1].p_next];
	let mut count = 0;
	// SAFETY: the mock writes only the live count and the two initialized records.
	unsafe {
		fixture
			.device
			.get_device_image_sparse_memory_requirements(&info, &mut count, std::ptr::null_mut())
			.unwrap();
		assert_eq!(count, 2);
		fixture
			.device
			.get_device_image_sparse_memory_requirements(&info, &mut count, values.as_mut_ptr())
			.unwrap();
	}
	assert_eq!(count, 2);
	assert_eq!(values[0].memory_requirements.image_mip_tail_size, u64::MAX);
	assert_eq!(values[1].memory_requirements.image_mip_tail_size, 1 << 40);
	assert_eq!([values[0].p_next, values[1].p_next], pointers);
	assert!(
		values
			.iter()
			.all(|v| v.s_type == vk::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2)
	);
	assert_eq!(chains, [7, 11]);
	TRACE.with_borrow(|trace| {
		assert_eq!(trace.len(), 8);
		assert_eq!(trace[3], 0);
		assert_eq!(trace[7], values.as_ptr() as usize);
	});
}

unsafe extern "system" fn map_memory(
	device: vk::DeviceHandle,
	info: *const vk::MemoryMapInfo,
	output: *mut *mut c_void,
) -> vk::Result {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([device.as_raw() as usize, info as usize, output as usize])
	});
	// SAFETY: output is the test's live pointer slot. The synthetic result is never dereferenced.
	unsafe {
		*output = std::ptr::without_provenance_mut::<c_void>(0x9000);
	}
	STATUS.get()
}

#[test]
fn map_memory2_retains_pointer_to_pointer_output_and_native_error() {
	// SAFETY: the exact-ABI mock only writes a caller-provided pointer slot.
	let fixture = unsafe {
		Fixture::new_with_command(
			c"vkMapMemory2",
			std::mem::transmute::<vk::PFN_vkMapMemory2, vk::PFN_vkVoidFunction>(Some(map_memory)),
		)
	};
	TRACE.with_borrow_mut(Vec::clear);
	let info = vk::MemoryMapInfo {
		offset: 1 << 40,
		size: u64::MAX,
		..Default::default()
	};
	let mut output = std::ptr::null_mut();
	for status in [vk::Result::SUCCESS, vk::Result::ERROR_MEMORY_MAP_FAILED] {
		STATUS.set(status);
		// SAFETY: the mock reads no input pointees and writes only this live output slot.
		let actual = unsafe { fixture.device.map_memory2(&info, &mut output) };
		assert_eq!(
			actual,
			if status.is_err() {
				Err(CallError::Vulkan(status))
			} else {
				Ok(status)
			}
		);
		assert_eq!(output as usize, 0x9000);
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(
			trace.as_slice(),
			[
				0x2000,
				std::ptr::from_ref(&info) as usize,
				std::ptr::from_ref(&output) as usize,
				0x2000,
				std::ptr::from_ref(&info) as usize,
				std::ptr::from_ref(&output) as usize
			]
		)
	});
}

unsafe extern "system" fn ray_pipelines(
	device: vk::DeviceHandle,
	deferred: vk::DeferredOperationKHR,
	cache: vk::PipelineCache,
	count: u32,
	info: *const vk::RayTracingPipelineCreateInfoKHR,
	allocator: *const vk::AllocationCallbacks,
	outputs: *mut vk::Pipeline,
) -> vk::Result {
	TRACE.with_borrow_mut(|trace| {
		trace.extend([
			device.as_raw() as usize,
			deferred.as_raw() as usize,
			cache.as_raw() as usize,
			count as usize,
			info as usize,
			allocator as usize,
			outputs as usize,
		])
	});
	// SAFETY: each test call passes two live writable pipeline slots. These are mock identities.
	unsafe {
		*outputs = vk::Pipeline::from_raw(0x7000);
		*outputs.add(1) = vk::Pipeline::NULL;
	}
	STATUS.get()
}

#[test]
fn deferred_and_failed_pipeline_creation_leaves_partial_outputs_with_caller() {
	// SAFETY: exact-ABI mock writes two test-owned output slots; no native resources exist.
	let fixture = unsafe {
		Fixture::new_with_command(
			c"vkCreateRayTracingPipelinesKHR",
			std::mem::transmute::<vk::PFN_vkCreateRayTracingPipelinesKHR, vk::PFN_vkVoidFunction>(Some(
				ray_pipelines,
			)),
		)
	};
	TRACE.with_borrow_mut(Vec::clear);
	let infos = std::array::from_fn::<_, 2, _>(|_| vk::RayTracingPipelineCreateInfoKHR::default());
	let mut outputs = [vk::Pipeline::NULL; 2];
	// SAFETY: allocation callbacks are never invoked; all nullable members may be zero.
	let allocator: vk::AllocationCallbacks = unsafe { std::mem::zeroed() };
	for status in [
		vk::Result::OPERATION_DEFERRED_KHR,
		vk::Result::PIPELINE_COMPILE_REQUIRED,
		vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
	] {
		STATUS.set(status);
		// SAFETY: the mock sees initialized input records and two live output slots.
		let result = unsafe {
			vk::create_ray_tracing_pipelines_khr(
				&fixture.device,
				vk::DeferredOperationKHR::from_raw(23),
				vk::PipelineCache::from_raw(29),
				2,
				infos.as_ptr(),
				&allocator,
				outputs.as_mut_ptr(),
			)
		};
		assert_eq!(
			result,
			if status.is_err() {
				Err(CallError::Vulkan(status))
			} else {
				Ok(status)
			}
		);
		assert_eq!(
			outputs,
			[vk::Pipeline::from_raw(0x7000), vk::Pipeline::NULL]
		);
	}
	TRACE.with_borrow(|trace| {
		assert_eq!(trace.len(), 21);
		for values in trace.as_chunks::<7>().0 {
			assert_eq!(
				*values,
				[
					0x2000,
					23,
					29,
					2,
					infos.as_ptr() as usize,
					std::ptr::from_ref(&allocator) as usize,
					outputs.as_ptr() as usize
				]
			);
		}
	});
}

unsafe extern "system" fn binary_data(
	_: vk::DeviceHandle,
	info: *const vk::PipelineBinaryDataInfoKHR,
	key: *mut vk::PipelineBinaryKeyKHR,
	size: *mut usize,
	data: *mut c_void,
) -> vk::Result {
	// SAFETY: the test supplies initialized key/size and either NULL or a live four-byte buffer.
	unsafe {
		TRACE
			.with_borrow_mut(|trace| trace.extend([info as usize, key as usize, *size, data as usize]));
		(*key).key_size = 1;
		(*key).key[0] = 0xa7;
		if !data.is_null() {
			*data.cast::<u8>() = 0x5a;
		}
		*size = if data.is_null() { 16 } else { 4 };
	}
	STATUS.get()
}

#[test]
fn binary_size_queries_preserve_inout_storage_on_not_enough_space() {
	// SAFETY: exact-ABI mock operates only on live caller-owned storage.
	let fixture = unsafe {
		Fixture::new_with_command(
			c"vkGetPipelineBinaryDataKHR",
			std::mem::transmute::<vk::PFN_vkGetPipelineBinaryDataKHR, vk::PFN_vkVoidFunction>(Some(
				binary_data,
			)),
		)
	};
	TRACE.with_borrow_mut(Vec::clear);
	let info = vk::PipelineBinaryDataInfoKHR::default();
	let mut chain = 37_u64;
	let mut key = vk::PipelineBinaryKeyKHR {
		p_next: std::ptr::from_mut(&mut chain).cast(),
		..Default::default()
	};
	let original_next = key.p_next;
	let mut size = 0;
	STATUS.set(vk::Result::SUCCESS);
	// SAFETY: live initialized key/size; NULL requests only the size from the mock.
	unsafe {
		fixture
			.device
			.get_pipeline_binary_data_khr(&info, &mut key, &mut size, std::ptr::null_mut())
			.unwrap();
	}
	assert_eq!(size, 16);
	let mut bytes = [0xcc_u8; 4];
	size = bytes.len();
	STATUS.set(vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR);
	// SAFETY: the mock writes only the first byte, live key and size.
	let result = unsafe {
		fixture.device.get_pipeline_binary_data_khr(
			&info,
			&mut key,
			&mut size,
			bytes.as_mut_ptr().cast(),
		)
	};
	assert_eq!(
		result,
		Err(CallError::Vulkan(vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR))
	);
	assert_eq!(size, 4);
	assert_eq!(bytes, [0x5a, 0xcc, 0xcc, 0xcc]);
	assert_eq!(key.s_type, vk::StructureType::PIPELINE_BINARY_KEY_KHR);
	assert_eq!(key.p_next, original_next);
	assert_eq!(key.key_size, 1);
	assert_eq!(key.key[0], 0xa7);
	assert_eq!(chain, 37);
	TRACE.with_borrow(|trace| {
		assert_eq!(trace.len(), 8);
		assert_eq!(trace[2], 0);
		assert_eq!(trace[3], 0);
		assert_eq!(trace[6], 4);
		assert_eq!(trace[7], bytes.as_ptr() as usize);
	});
}
