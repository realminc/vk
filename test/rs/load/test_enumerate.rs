use super::*;

#[test]
fn byte_enumeration_retries_growth_and_preserves_exact_payload() {
	let mut pass = 0;
	// SAFETY: every data call writes at most its supplied capacity, and the
	// successful call initializes precisely its returned byte count.
	let bytes = unsafe {
		enumerate_bytes(|size, data| {
			if data.is_null() {
				*size = if pass == 0 { 2 } else { 5 };
				return vk::Result::SUCCESS;
			}
			if pass == 0 {
				*data = 0x80;
				*size = 1;
				pass += 1;
				return vk::Result::INCOMPLETE;
			}
			std::ptr::copy_nonoverlapping([0, 255, 3, 128, 7].as_ptr(), data, 5);
			*size = 5;
			vk::Result::SUCCESS
		})
	}
	.unwrap();
	assert_eq!(bytes, [0, 255, 3, 128, 7]);
}

#[test]
fn byte_enumeration_bounds_errors_zero_shrink_and_allocation_failure() {
	// SAFETY: mocks never write beyond capacity; invalid counts are returned
	// without touching storage so the wrapper can reject them safely.
	unsafe {
		assert_eq!(
			enumerate_bytes(|size, _| {
				*size = 0;
				vk::Result::SUCCESS
			}),
			Ok(vec![])
		);
		assert_eq!(
			enumerate_bytes(|_, _| vk::Result::ERROR_DEVICE_LOST),
			Err(CallError::Vulkan(vk::Result::ERROR_DEVICE_LOST))
		);
		assert_eq!(
			enumerate_bytes(|size, _| {
				*size = usize::MAX;
				vk::Result::SUCCESS
			}),
			Err(CallError::HostAllocationFailed)
		);
		assert!(matches!(
			enumerate_bytes(|size, data| {
				*size = if data.is_null() { 1 } else { 2 };
				vk::Result::SUCCESS
			}),
			Err(CallError::InvalidOutput(_))
		));
		assert_eq!(
			enumerate_bytes(|size, data| {
				*size = if data.is_null() { 3 } else { 0 };
				vk::Result::SUCCESS
			}),
			Ok(vec![])
		);
		assert_eq!(
			enumerate_bytes(|size, data| {
				*size = 1;
				if data.is_null() {
					vk::Result::SUCCESS
				} else {
					vk::Result::ERROR_OUT_OF_HOST_MEMORY
				}
			}),
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		);
		let mut calls = 0;
		assert_eq!(
			enumerate_bytes(|size, _| {
				calls += 1;
				*size = 0;
				vk::Result::INCOMPLETE
			}),
			Err(CallError::EnumerationUnstable {
				attempts: MAX_ATTEMPTS
			})
		);
		assert_eq!(calls, MAX_ATTEMPTS);
	}
}

#[test]
fn retries_incomplete_without_returning_a_partial_list() {
	let mut pass = 0;
	// SAFETY: the closure follows the documented count/data protocol.
	let values = unsafe {
		enumerate(|count, data: *mut u32| {
			if data.is_null() {
				*count = if pass == 0 { 1 } else { 3 };
				return vk::Result::SUCCESS;
			}
			if pass == 0 {
				pass += 1;
				*data = 7;
				return vk::Result::INCOMPLETE;
			}
			for i in 0..3 {
				*data.add(i) = i as u32 + 7;
			}
			*count = 3;
			vk::Result::SUCCESS
		})
	}
	.unwrap();
	assert_eq!(values, [7, 8, 9]);
}

#[test]
fn invalid_reported_count_does_not_expose_uninitialized_values() {
	// SAFETY: the mock never writes beyond the supplied buffer.
	let result = unsafe {
		enumerate::<u32>(|count, data| {
			*count = if data.is_null() { 1 } else { 2 };
			vk::Result::SUCCESS
		})
	};
	assert!(matches!(result, Err(CallError::InvalidOutput(_))));
}

#[test]
fn unstable_enumeration_terminates_and_preserves_native_failures() {
	let mut calls = 0;
	// SAFETY: the mock reports a count, initializes one item and requests retry.
	let result = unsafe {
		enumerate::<u32>(|count, data| {
			calls += 1;
			*count = 1;
			if data.is_null() {
				vk::Result::SUCCESS
			} else {
				*data = 99;
				vk::Result::INCOMPLETE
			}
		})
	};
	assert_eq!(
		result,
		Err(CallError::EnumerationUnstable {
			attempts: MAX_ATTEMPTS
		})
	);
	assert_eq!(calls, MAX_ATTEMPTS * 2);
	for fail_on_data in [false, true] {
		// SAFETY: the mock writes only a count on its error path; no output escapes.
		let result = unsafe {
			enumerate::<u32>(|count, data| {
				*count = 1;
				if !data.is_null() || !fail_on_data {
					vk::Result::ERROR_OUT_OF_HOST_MEMORY
				} else {
					vk::Result::SUCCESS
				}
			})
		};
		assert_eq!(
			result,
			Err(CallError::Vulkan(vk::Result::ERROR_OUT_OF_HOST_MEMORY))
		);
	}
}

#[test]
fn incomplete_zero_count_retries_and_successful_shrink_exposes_only_written_values() {
	let mut count_calls = 0;
	// SAFETY: one zero/incomplete count pass, followed by a fully initialized result.
	let result = unsafe {
		enumerate::<u32>(|count, data| {
			if data.is_null() {
				count_calls += 1;
				*count = if count_calls == 1 { 0 } else { 3 };
				if count_calls == 1 {
					return vk::Result::INCOMPLETE;
				}
			} else {
				*data = 123;
				*count = 1;
			}
			vk::Result::SUCCESS
		})
	}
	.unwrap();
	assert_eq!(result, [123]);
	assert_eq!(count_calls, 2);
}

#[test]
fn void_enumeration_retries_growth_without_publishing_truncated_values() {
	let mut pass = 0;
	let mut calls = 0;
	// SAFETY: each fill initializes only the capacity supplied on entry.
	let values = unsafe {
		enumerate_void::<u32>(|count, data| {
			calls += 1;
			if data.is_null() {
				*count = if pass == 0 { 1 } else { 3 };
				return;
			}
			let capacity = *count;
			for index in 0..capacity {
				*data.add(index as usize) = index + 10;
			}
			pass += 1;
		})
	}
	.unwrap();
	assert_eq!(values, [10, 11, 12]);
	assert_eq!(calls, 6);
}

#[test]
fn void_enumeration_bounds_invalid_counts_and_continuous_growth() {
	// SAFETY: no writes beyond the allocation; the deliberately bad count is rejected.
	let result = unsafe {
		enumerate_void::<u32>(|count, data| {
			*count = if data.is_null() { 1 } else { 2 };
		})
	};
	assert!(matches!(result, Err(CallError::InvalidOutput(_))));
	let mut available = 1;
	// SAFETY: every fill initializes exactly the input count before growth.
	let result = unsafe {
		enumerate_void::<u32>(|count, data| {
			if data.is_null() {
				*count = available;
			} else {
				for index in 0..*count {
					*data.add(index as usize) = index;
				}
				available += 1;
			}
		})
	};
	assert_eq!(
		result,
		Err(CallError::EnumerationUnstable {
			attempts: MAX_ATTEMPTS
		})
	);
}
