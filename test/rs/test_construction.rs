//! Independent host construction behavior and external borrow-checker contracts.
use vk::{ConstructionError, StructureType};

#[test]
fn strings_nested_records_and_arrays_keep_exact_native_fields() -> Result<(), ConstructionError> {
	let name = std::ffi::CString::new("construction").unwrap();
	let application = vk::ApplicationInfo::default()
		.application_name(&name)
		.api_version(vk::API_VERSION_1_0);
	let info = vk::InstanceCreateInfo::default().application_info(&application);
	assert_eq!(application.p_application_name, name.as_ptr());
	assert_eq!(info.p_application_info, std::ptr::from_ref(&application));
	assert_eq!(info.s_type, StructureType::INSTANCE_CREATE_INFO);
	assert!(info.p_next.is_null());
	let priorities = [1.0, 0.25];
	let queues = [vk::DeviceQueueCreateInfo::default()
		.queue_family_index(7)
		.queue_priorities(&priorities)?];
	let device = vk::DeviceCreateInfo::default().queue_create_infos(&queues)?;
	assert_eq!(device.queue_create_info_count, 1);
	assert_eq!(device.p_queue_create_infos, queues.as_ptr());
	assert_eq!(queues[0].queue_count, 2);
	assert_eq!(queues[0].p_queue_priorities, priorities.as_ptr());
	let empty = vk::DeviceQueueCreateInfo::default().queue_priorities(&[])?;
	assert_eq!(empty.queue_count, 0);
	Ok(())
}

#[test]
fn pointer_arrays_borrow_array_without_claiming_string_ownership() -> Result<(), ConstructionError>
{
	let names = [c"VK_EXAMPLE_one".as_ptr(), c"VK_EXAMPLE_two".as_ptr()];
	let info = vk::InstanceCreateInfo::default().enabled_extension_names(&names)?;
	assert_eq!(info.enabled_extension_count, 2);
	assert_eq!(info.pp_enabled_extension_names, names.as_ptr());
	Ok(())
}

#[test]
fn chain_insertion_prepends_without_traversing_raw_links() -> Result<(), ConstructionError> {
	let sentinel = std::ptr::without_provenance::<std::ffi::c_void>(0xBAD);
	let mut first = vk::PhysicalDeviceVulkan11Features::default();
	let mut second = vk::PhysicalDeviceVulkan12Features::default();
	let first_address = std::ptr::from_mut(&mut first).cast::<std::ffi::c_void>();
	let second_address = std::ptr::from_mut(&mut second).cast::<std::ffi::c_void>();
	{
		let root = vk::PhysicalDeviceFeatures2 {
			p_next: sentinel.cast_mut(),
			..Default::default()
		}
		.push_next(&mut first)?
		.push_next(&mut second)?;
		assert_eq!(root.p_next, second_address);
	}
	assert_eq!(second.p_next, first_address);
	assert_eq!(first.p_next, sentinel.cast_mut());
	Ok(())
}

#[test]
fn rejected_chain_nodes_are_unchanged() {
	let sentinel = std::ptr::without_provenance_mut::<std::ffi::c_void>(0xBAD);
	let mut linked = vk::PhysicalDeviceVulkan12Features {
		p_next: sentinel,
		..Default::default()
	};
	let result = vk::PhysicalDeviceFeatures2::default().push_next(&mut linked);
	assert!(matches!(result, Err(ConstructionError::AlreadyLinked)));
	assert_eq!(linked.p_next, sentinel);
	let mut wrong_tag = vk::PhysicalDeviceVulkan12Features {
		s_type: StructureType::APPLICATION_INFO,
		..Default::default()
	};
	let result = vk::PhysicalDeviceFeatures2::default().push_next(&mut wrong_tag);
	assert!(matches!(
		result,
		Err(ConstructionError::UnexpectedStructureType { .. })
	));
	assert!(wrong_tag.p_next.is_null());
	let mut valid = vk::PhysicalDeviceVulkan12Features::default();
	let root = vk::PhysicalDeviceFeatures2 {
		s_type: StructureType::APPLICATION_INFO,
		..Default::default()
	};
	assert!(matches!(
		root.push_next(&mut valid),
		Err(ConstructionError::UnexpectedStructureType { .. })
	));
	assert!(valid.p_next.is_null());
}

#[test]
fn output_chain_writes_are_visible_after_the_root_last_use() -> Result<(), ConstructionError> {
	unsafe extern "system" fn query(
		_: vk::PhysicalDevice,
		root: *mut vk::PhysicalDeviceFeatures2<'_>,
	) {
		// SAFETY: the test supplies this exact writable, initialized typed chain.
		unsafe {
			let extension = (*root)
				.p_next
				.cast::<vk::PhysicalDeviceVulkan12Features<'_>>();
			(*extension).timeline_semaphore = vk::TRUE;
		}
	}
	let mut extension = vk::PhysicalDeviceVulkan12Features::default();
	{
		let mut root = vk::PhysicalDeviceFeatures2::default().push_next(&mut extension)?;
		let function: vk::PFN_vkGetPhysicalDeviceFeatures2 = std::hint::black_box(Some(query));
		// SAFETY: an exact-prototype host mock writes the borrowed extension only.
		unsafe { function.unwrap()(vk::PhysicalDevice::NULL, &mut root) };
	}
	assert_eq!(extension.timeline_semaphore, vk::TRUE);
	Ok(())
}

#[test]
fn external_consumers_enforce_borrowing_contracts() {
	use std::{path::Path, process::Command};
	let dependencies = std::env::current_exe()
		.unwrap()
		.parent()
		.unwrap()
		.to_owned();
	let mut libraries: Vec<_> = std::fs::read_dir(&dependencies)
		.unwrap()
		.map(|p| p.unwrap().path())
		.filter(|p| {
			p.file_name()
				.unwrap()
				.to_string_lossy()
				.starts_with("libvk-")
				&& p.extension().is_some_and(|e| e == "rlib")
		})
		.collect();
	libraries.sort_by_key(|p| std::cmp::Reverse(p.metadata().unwrap().modified().unwrap()));
	let directory = std::env::temp_dir().join(format!("vk-construction-{}", std::process::id()));
	std::fs::create_dir_all(&directory).unwrap();
	struct Cleanup(std::path::PathBuf);
	impl Drop for Cleanup {
		fn drop(&mut self) {
			let _ = std::fs::remove_dir_all(&self.0);
		}
	}
	let _cleanup = Cleanup(directory.clone());
	let compile = |library: &Path, source: &str| {
		let input = directory.join("fixture.rs");
		std::fs::write(&input, source).unwrap();
		Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
			.args([
				"--edition=2024",
				"--crate-name=construction_fixture",
				"--emit=metadata",
				"--error-format=json",
			])
			.arg("--extern")
			.arg(format!("vk={}", library.display()))
			.arg("-L")
			.arg(format!("dependency={}", dependencies.display()))
			.arg(&input)
			.arg("-o")
			.arg(directory.join("fixture.rmeta"))
			.output()
			.unwrap()
	};
	let positive = include_str!("construction/test_pass.rs");
	let mut failures = Vec::new();
	let library = libraries
		.iter()
		.find(|p| {
			let output = compile(p, positive);
			if !output.status.success() {
				failures.push(String::from_utf8_lossy(&output.stderr).into_owned());
			}
			output.status.success()
		})
		.unwrap_or_else(|| {
			panic!(
				"no matching vk rlib compiled the positive construction consumer: {libraries:?}\n{failures:?}"
			)
		});
	for (name, source, diagnostic) in [
		(
			"dangling_shader_words",
			include_str!("construction/test_fail_words.rs"),
			"E0597",
		),
		(
			"access_borrowed_output_array",
			include_str!("construction/test_fail_output_array.rs"),
			"E0502",
		),
		(
			"duplicate_writable_record",
			include_str!("construction/test_fail_output_copy.rs"),
			"E0382",
		),
		(
			"implicit_opaque_pointer",
			include_str!("construction/test_fail_raw.rs"),
			"E0133",
		),
		(
			"dangling_string",
			include_str!("construction/test_fail_string.rs"),
			"E0597",
		),
		(
			"dangling_nested_record",
			include_str!("construction/test_fail_nested.rs"),
			"E0597",
		),
		(
			"mutated_backing_array",
			include_str!("construction/test_fail_slice.rs"),
			"E0506",
		),
		(
			"wrong_chain_root",
			include_str!("construction/test_fail_chain_type.rs"),
			"E0277",
		),
		(
			"access_borrowed_output",
			include_str!("construction/test_fail_output.rs"),
			"E0502",
		),
		(
			"duplicate_exclusive_root",
			include_str!("construction/test_fail_copy.rs"),
			"E0382",
		),
		(
			"escape_output",
			include_str!("construction/test_fail_escape.rs"),
			"E0597",
		),
	] {
		let output = compile(library, source);
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(!output.status.success(), "{name} unexpectedly compiled");
		assert!(
			stderr.contains(diagnostic),
			"{name} failed for an unexpected reason: {stderr}"
		);
	}
}

#[test]
fn shared_counts_require_matching_parallel_arrays() -> Result<(), ConstructionError> {
	let semaphores = [vk::Semaphore::NULL; 2];
	let stages = [vk::PipelineStageFlags::from_raw(1); 2];
	let values = [3_u64, 7];
	let submit = vk::SubmitInfo::default().wait_semaphores(&semaphores, &stages)?;
	assert_eq!(submit.wait_semaphore_count, 2);
	assert_eq!(submit.p_wait_semaphores, semaphores.as_ptr());
	assert_eq!(submit.p_wait_dst_stage_mask, stages.as_ptr());
	let wait = vk::SemaphoreWaitInfo::default().semaphores(&semaphores, &values)?;
	assert_eq!(wait.semaphore_count, 2);
	assert_eq!(wait.p_values, values.as_ptr());
	assert!(matches!(
		vk::SubmitInfo::default().wait_semaphores(&semaphores, &stages[..1]),
		Err(ConstructionError::LengthMismatch {
			expected: 2,
			actual: 1,
			..
		})
	));
	assert!(matches!(
		vk::SemaphoreWaitInfo::default().semaphores(&semaphores, &values[..1]),
		Err(ConstructionError::LengthMismatch {
			expected: 2,
			actual: 1,
			..
		})
	));
	Ok(())
}

#[test]
fn descriptor_payload_selection_clears_conflicting_native_arrays() -> Result<(), ConstructionError>
{
	let images = [vk::DescriptorImageInfo::default()];
	let buffers = [vk::DescriptorBufferInfo::default(); 2];
	let views = [vk::BufferView::NULL; 3];
	let write = vk::WriteDescriptorSet::default().image_info(&images)?;
	assert_eq!(write.descriptor_count, 1);
	assert_eq!(write.p_image_info, images.as_ptr());
	let write = write.buffer_info(&buffers)?;
	assert_eq!(write.descriptor_count, 2);
	assert!(write.p_image_info.is_null());
	assert_eq!(write.p_buffer_info, buffers.as_ptr());
	let write = write.texel_buffer_view(&views)?;
	assert_eq!(write.descriptor_count, 3);
	assert!(write.p_image_info.is_null());
	assert!(write.p_buffer_info.is_null());
	assert_eq!(write.p_texel_buffer_view, views.as_ptr());
	Ok(())
}

#[test]
fn word_and_byte_payloads_use_native_byte_extents() -> Result<(), ConstructionError> {
	let words = [0x0723_0203_u32, 0, 1, 2];
	let shader = vk::ShaderModuleCreateInfo::default().code(&words);
	assert_eq!(shader.code_size, 16);
	assert_eq!(shader.p_code, words.as_ptr());
	let bytes = [0_u8, 5, 255, 7, 9];
	let specialization = vk::SpecializationInfo::default().data(&bytes)?;
	assert_eq!(specialization.data_size, 5);
	assert_eq!(specialization.p_data, bytes.as_ptr().cast());
	let cache = vk::PipelineCacheCreateInfo::default().initial_data(&bytes)?;
	assert_eq!(cache.initial_data_size, 5);
	assert_eq!(cache.p_initial_data, bytes.as_ptr().cast());
	let inline = vk::WriteDescriptorSetInlineUniformBlock::default().data(&bytes)?;
	assert_eq!(inline.data_size, 5);
	let empty = vk::ShaderModuleCreateInfo::default().code(&[]);
	assert_eq!(empty.code_size, 0);
	let empty = vk::PipelineCacheCreateInfo::default().initial_data(&[])?;
	assert_eq!(empty.initial_data_size, 0);
	Ok(())
}

#[test]
fn optional_parallel_arrays_and_writable_results_share_one_count() -> Result<(), ConstructionError>
{
	let swaps = [vk::SwapchainKHR::NULL; 2];
	let indices = [3, 7];
	let mut results = [vk::Result::NOT_READY; 2];
	let results_address = results.as_mut_ptr();
	{
		let present = vk::PresentInfoKHR::default().swapchains(&swaps, &indices, Some(&mut results))?;
		assert_eq!(present.swapchain_count, 2);
		assert_eq!(present.p_swapchains, swaps.as_ptr());
		assert_eq!(present.p_image_indices, indices.as_ptr());
		assert_eq!(present.p_results, results_address);
		// SAFETY: the helper borrows two initialized output slots exclusively.
		unsafe { present.p_results.add(1).write(vk::Result::SUCCESS) };
	}
	assert_eq!(results, [vk::Result::NOT_READY, vk::Result::SUCCESS]);
	let no_results = vk::PresentInfoKHR::default().swapchains(&swaps, &indices, None)?;
	assert!(no_results.p_results.is_null());
	assert!(matches!(
		vk::PresentInfoKHR::default().swapchains(&swaps, &indices[..1], None),
		Err(ConstructionError::LengthMismatch {
			field: "pImageIndices",
			..
		})
	));
	let mut too_short = [vk::Result::NOT_READY];
	assert!(matches!(
		vk::PresentInfoKHR::default().swapchains(&swaps, &indices, Some(&mut too_short)),
		Err(ConstructionError::LengthMismatch {
			field: "pResults",
			..
		})
	));
	assert_eq!(too_short[0], vk::Result::NOT_READY);
	let attachments = [vk::AttachmentReference::default(); 2];
	let subpass = vk::SubpassDescription::default().color_attachments(&attachments, None)?;
	assert_eq!(subpass.color_attachment_count, 2);
	assert!(subpass.p_resolve_attachments.is_null());
	assert!(
		vk::SubpassDescription::default()
			.color_attachments(&attachments, Some(&attachments[..1]))
			.is_err()
	);
	Ok(())
}

#[test]
fn fully_optional_groups_preserve_explicit_query_counts() -> Result<(), ConstructionError> {
	let query = vk::SwapchainTimeDomainPropertiesEXT::default().time_domains(0, None, None)?;
	assert_eq!(query.time_domain_count, 0);
	assert!(query.p_time_domains.is_null());
	assert!(query.p_time_domain_ids.is_null());
	let mut domains = [vk::TimeDomainKHR::default(); 2];
	let mut ids = [0_u64; 2];
	let info = vk::SwapchainTimeDomainPropertiesEXT::default().time_domains(
		2,
		Some(&mut domains),
		Some(&mut ids),
	)?;
	assert_eq!(info.time_domain_count, 2);
	assert!(
		vk::SwapchainTimeDomainPropertiesEXT::default()
			.time_domains(3, None, Some(&mut ids))
			.is_err()
	);
	let extent = [5_u64, 9];
	let tensor = vk::TensorCopyARM::default().src_offset(2, None, None, Some(&extent))?;
	assert_eq!(tensor.dimension_count, 2);
	assert_eq!(tensor.p_extent, extent.as_ptr());
	assert!(tensor.p_src_offset.is_null());
	Ok(())
}

#[test]
fn indirect_alternatives_clear_other_representations() -> Result<(), ConstructionError> {
	let geometries: [_; 2] = std::array::from_fn(|_| vk::AccelerationStructureGeometryKHR::default());
	let addresses = geometries.each_ref().map(std::ptr::from_ref);
	let direct = vk::AccelerationStructureBuildGeometryInfoKHR::default().geometries(&geometries)?;
	assert_eq!(direct.geometry_count, 2);
	assert_eq!(direct.p_geometries, geometries.as_ptr());
	assert!(direct.pp_geometries.is_null());
	let indirect = direct.pp_geometries(&addresses[..1])?;
	assert_eq!(indirect.geometry_count, 1);
	assert!(indirect.p_geometries.is_null());
	assert_eq!(indirect.pp_geometries, addresses.as_ptr());
	let direct = indirect.geometries(&geometries)?;
	assert!(direct.pp_geometries.is_null());
	let usages = [vk::MicromapUsageEXT::default(); 2];
	let pointers = usages.each_ref().map(std::ptr::from_ref);
	let info = vk::MicromapBuildInfoEXT::default()
		.usage_counts(&usages)?
		.pp_usage_counts(&pointers)?;
	assert_eq!(info.usage_counts_count, 2);
	assert!(info.p_usage_counts.is_null());
	assert_eq!(info.pp_usage_counts, pointers.as_ptr());
	Ok(())
}

#[test]
fn writable_byte_and_scalar_storage_remains_initialized_and_borrowed()
-> Result<(), ConstructionError> {
	let mut bytes = [0_u8; 5];
	let address = bytes.as_mut_ptr();
	{
		let mut output = vk::PipelineExecutableInternalRepresentationKHR::default().data(&mut bytes)?;
		assert_eq!(output.data_size, 5);
		assert_eq!(output.p_data, address.cast());
		// SAFETY: exactly five initialized bytes are exclusively borrowed.
		unsafe { output.p_data.cast::<u8>().add(4).write(255) };
		// A native query may report a larger required size. No safe accessor may
		// manufacture a slice from this public, driver-controlled field.
		output.data_size = usize::MAX;
		std::hint::black_box(&output);
	}
	assert_eq!(bytes, [0, 0, 0, 0, 255]);
	let mut result = vk::Result::NOT_READY;
	{
		let status = vk::BindMemoryStatus::default().result(&mut result);
		// SAFETY: the constructor borrows one initialized result exclusively.
		unsafe { status.p_result.write(vk::Result::SUCCESS) };
	}
	assert_eq!(result, vk::Result::SUCCESS);
	Ok(())
}

#[test]
fn fixed_formula_and_parent_sized_video_storage_have_exact_extents() -> Result<(), ConstructionError>
{
	let mask = [u32::MAX; 2];
	let info = vk::PipelineMultisampleStateCreateInfo::default()
		.rasterization_samples(vk::SampleCountFlagBits::_64_BIT)
		.sample_mask(&mask)?;
	assert_eq!(info.p_sample_mask, mask.as_ptr());
	assert!(
		vk::PipelineMultisampleStateCreateInfo::default()
			.rasterization_samples(vk::SampleCountFlagBits::_64_BIT)
			.sample_mask(&mask[..1])
			.is_err()
	);
	let version = [0_u8; 2 * vk::UUID_SIZE as usize];
	let info = vk::AccelerationStructureVersionInfoKHR::default().version_data(&version);
	assert_eq!(info.p_version_data, version.as_ptr());
	let offsets = [1_i32; 255];
	let sps = vk::StdVideoH264SequenceParameterSet::default().offset_for_ref_frame(&offsets)?;
	assert_eq!(sps.num_ref_frames_in_pic_order_cnt_cycle, 255);
	let too_many = [1_i32; 256];
	assert!(matches!(
		vk::StdVideoH264SequenceParameterSet::default().offset_for_ref_frame(&too_many),
		Err(ConstructionError::CountOverflow { count: 256, .. })
	));
	let layers = [vk::StdVideoH265SubLayerHrdParameters::default(); 3];
	let hrd = vk::StdVideoH265HrdParameters::default().sub_layer_hrd_parameters_nal(2, &layers)?;
	assert_eq!(hrd.p_sub_layer_hrd_parameters_nal, layers.as_ptr());
	assert!(
		vk::StdVideoH265HrdParameters::default()
			.sub_layer_hrd_parameters_nal(1, &layers)
			.is_err()
	);
	Ok(())
}

#[test]
fn layer_values_count_elements_and_select_native_storage_type() -> Result<(), ConstructionError> {
	let values = [2_i64, 9];
	let setting = vk::LayerSettingEXT::default().values_int64(&values)?;
	assert_eq!(setting.r#type, vk::LayerSettingTypeEXT::INT64_EXT);
	assert_eq!(setting.value_count, 2);
	assert_eq!(setting.p_values, values.as_ptr().cast());
	let names = [c"one".as_ptr(), c"two".as_ptr()];
	let setting = setting.values_strings(&names)?;
	assert_eq!(setting.r#type, vk::LayerSettingTypeEXT::STRING_EXT);
	assert_eq!(setting.value_count, 2);
	assert_eq!(setting.p_values, names.as_ptr().cast());
	Ok(())
}

#[test]
fn fault_buffers_populate_the_separate_native_capacity_record() -> Result<(), ConstructionError> {
	let mut counts = vk::DeviceFaultCountsEXT::default();
	let mut addresses = [vk::DeviceFaultAddressInfoKHR::default(); 2];
	let mut vendors = [vk::DeviceFaultVendorInfoKHR::default()];
	let mut binary = [0_u8; 13];
	let pointers = (
		addresses.as_mut_ptr(),
		vendors.as_mut_ptr(),
		binary.as_mut_ptr(),
	);
	let info = vk::DeviceFaultInfoEXT::default().buffers(
		&mut counts,
		&mut addresses,
		&mut vendors,
		&mut binary,
	)?;
	assert_eq!(counts.address_info_count, 2);
	assert_eq!(counts.vendor_info_count, 1);
	assert_eq!(counts.vendor_binary_size, 13);
	assert_eq!(info.p_address_infos, pointers.0);
	assert_eq!(info.p_vendor_infos, pointers.1);
	assert_eq!(info.p_vendor_binary_data, pointers.2.cast());
	Ok(())
}

#[test]
fn union_construction_and_opaque_native_setters_preserve_exact_values() {
	let sampler = vk::Sampler::NULL;
	let data = vk::DescriptorDataEXT::default().sampler(&sampler);
	// SAFETY: this is the exact union member most recently initialized.
	assert_eq!(unsafe { data.p_sampler }, std::ptr::from_ref(&sampler));
	let clear = vk::ClearColorValue::default().uint32([1, 2, 3, 4]);
	// SAFETY: this is the exact union member most recently initialized.
	assert_eq!(unsafe { clear.uint32 }, [1, 2, 3, 4]);
	let mut user = 7_u32;
	let address = std::ptr::from_mut(&mut user).cast();
	// SAFETY: the test only compares the pointer, with no callbacks or native use.
	let callbacks = unsafe { vk::AllocationCallbacks::default().user_data(address) };
	assert_eq!(callbacks.p_user_data, address);
}

#[test]
fn direct_native_counts_preserve_null_pointer_and_extension_payload_paths()
-> Result<(), ConstructionError> {
	let dynamic = vk::PipelineViewportStateCreateInfo::default()
		.viewport_count(2)
		.scissor_count(2);
	assert_eq!(dynamic.viewport_count, 2);
	assert_eq!(dynamic.scissor_count, 2);
	assert!(dynamic.p_viewports.is_null());
	assert!(dynamic.p_scissors.is_null());
	let bytes = [0_u8; 4];
	let mut inline = vk::WriteDescriptorSetInlineUniformBlock::default().data(&bytes)?;
	let write = vk::WriteDescriptorSet::default()
		.descriptor_type(vk::DescriptorType::INLINE_UNIFORM_BLOCK)
		.descriptor_count(4)
		.push_next(&mut inline)?;
	assert_eq!(write.descriptor_count, 4);
	assert!(write.p_image_info.is_null());
	assert!(write.p_buffer_info.is_null());
	assert!(write.p_texel_buffer_view.is_null());
	Ok(())
}
