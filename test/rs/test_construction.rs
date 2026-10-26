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
