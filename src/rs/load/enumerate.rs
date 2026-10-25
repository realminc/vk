//! Shared retry policy for Vulkan count/data enumeration commands.

use super::CallError;
use crate as vk;

const MAX_ATTEMPTS: u32 = 8;

/// Enumerate a size_t byte-count query such as pipeline cache serialization.
///
/// # Safety
/// On success, `query` initializes the reported number of bytes without
/// exceeding the input capacity. Count-only calls must write the byte count.
pub(crate) unsafe fn enumerate_bytes(
	mut query: impl FnMut(*mut usize, *mut u8) -> vk::Result,
) -> Result<Vec<u8>, CallError> {
	for _ in 0..MAX_ATTEMPTS {
		let mut size = 0;
		let status = query(&mut size, std::ptr::null_mut());
		if status != vk::Result::SUCCESS && status != vk::Result::INCOMPLETE {
			return Err(CallError::Vulkan(status));
		}
		if size == 0 {
			if status == vk::Result::SUCCESS {
				return Ok(Vec::new());
			}
			continue;
		}
		let mut bytes = Vec::new();
		bytes
			.try_reserve_exact(size)
			.map_err(|_| CallError::HostAllocationFailed)?;
		let capacity = size;
		let status = query(&mut size, bytes.as_mut_ptr());
		if status == vk::Result::INCOMPLETE {
			continue;
		}
		if status != vk::Result::SUCCESS {
			return Err(CallError::Vulkan(status));
		}
		if size > capacity {
			return Err(CallError::InvalidOutput(
				"enumeration byte count exceeds supplied capacity",
			));
		}
		// SAFETY: the query initializes exactly size bytes on success, within capacity.
		unsafe { bytes.set_len(size) };
		return Ok(bytes);
	}
	Err(CallError::EnumerationUnstable {
		attempts: MAX_ATTEMPTS,
	})
}

/// # Safety
/// On success, `query` must initialize the reported number of `T` objects,
/// never exceeding the supplied capacity, according to Vulkan enumeration rules.
pub(crate) unsafe fn enumerate<T: Copy>(
	mut query: impl FnMut(*mut u32, *mut T) -> vk::Result,
) -> Result<Vec<T>, CallError> {
	for _ in 0..MAX_ATTEMPTS {
		let mut count = 0;
		let status = query(&mut count, std::ptr::null_mut());
		if status != vk::Result::SUCCESS && status != vk::Result::INCOMPLETE {
			return Err(CallError::Vulkan(status));
		}
		if count == 0 {
			if status == vk::Result::SUCCESS {
				return Ok(Vec::new());
			}
			continue;
		}
		let mut values = Vec::new();
		values
			.try_reserve_exact(count as usize)
			.map_err(|_| CallError::HostAllocationFailed)?;
		let capacity = count;
		let status = query(&mut count, values.as_mut_ptr());
		if status == vk::Result::INCOMPLETE {
			continue;
		}
		if status != vk::Result::SUCCESS {
			return Err(CallError::Vulkan(status));
		}
		if count > capacity {
			return Err(CallError::InvalidOutput(
				"enumeration count exceeds supplied capacity",
			));
		}
		// SAFETY: the caller's query initializes exactly count values on success.
		unsafe { values.set_len(count as usize) };
		return Ok(values);
	}
	Err(CallError::EnumerationUnstable {
		attempts: MAX_ATTEMPTS,
	})
}

/// Enumerate commands with a void return and a count/data protocol. Requery
/// the available count after filling, because truncation has no status code.
///
/// # Safety
/// Each non-null data call must initialize the reported number of T objects
/// without exceeding the input capacity. Count-only calls must write the count.
pub(crate) unsafe fn enumerate_void<T: Copy>(
	mut query: impl FnMut(*mut u32, *mut T),
) -> Result<Vec<T>, CallError> {
	for _ in 0..MAX_ATTEMPTS {
		let mut count = 0;
		query(&mut count, std::ptr::null_mut());
		if count == 0 {
			return Ok(Vec::new());
		}
		let mut values = Vec::new();
		values
			.try_reserve_exact(count as usize)
			.map_err(|_| CallError::HostAllocationFailed)?;
		let capacity = count;
		query(&mut count, values.as_mut_ptr());
		if count > capacity {
			return Err(CallError::InvalidOutput(
				"enumeration count exceeds supplied capacity",
			));
		}
		let mut available = 0;
		query(&mut available, std::ptr::null_mut());
		if available > count {
			continue;
		}
		// SAFETY: the query contract initializes count items without exceeding capacity.
		unsafe { values.set_len(count as usize) };
		return Ok(values);
	}
	Err(CallError::EnumerationUnstable {
		attempts: MAX_ATTEMPTS,
	})
}

#[cfg(test)]
#[path = "../../../test/rs/load/test_enumerate.rs"]
mod tests;
