//! Explicit extension/local budget provenance and native heap-limit enforcement.
use super::*;
use crate::memory::BudgetSource;

pub(super) unsafe extern "system" fn properties_core(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceMemoryProperties2,
) {
	event("budget_core");
	// SAFETY: initialized synchronous output chain supplied by canonical query.
	unsafe {
		properties(output);
	}
}

pub(super) unsafe extern "system" fn properties_khr(
	_: vk::PhysicalDevice,
	output: *mut vk::PhysicalDeviceMemoryProperties2,
) {
	event("budget_khr");
	// SAFETY: initialized synchronous output chain supplied by canonical query.
	unsafe {
		properties(output);
	}
}

unsafe fn properties(output: *mut vk::PhysicalDeviceMemoryProperties2) {
	// SAFETY: canonical budget query owns both writable records through this call.
	unsafe {
		assert_eq!(
			(*output).s_type,
			vk::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2
		);
		(*output).memory_properties = super::properties();
		let budget = &mut *(*output)
			.p_next
			.cast::<vk::PhysicalDeviceMemoryBudgetPropertiesEXT>();
		assert_eq!(
			budget.s_type,
			vk::StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT
		);
		assert!(budget.p_next.is_null());
		budget.heap_budget[0] = STATE.with_borrow(|s| s.budget);
		budget.heap_usage[0] = STATE.with_borrow(|s| s.usage);
	}
}

#[test]
fn local_limits_enforce_native_bytes_without_pretending_to_measure_external_pressure() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	assert_eq!(allocator.budget_snapshot().source, BudgetSource::Local);
	allocator.set_heap_limit(0, 300).unwrap();
	// SAFETY: synthetic native memory only; exact explicit cleanup.
	unsafe {
		let mut a = allocator.allocate(&request()).unwrap();
		let snapshot = allocator.budget_snapshot();
		assert_eq!(
			(
				snapshot.heap_count,
				snapshot.heaps[0].native_bytes,
				snapshot.heaps[0].limit
			),
			(1, 257, 300)
		);
		assert!(allocator.set_heap_limit(0, 256).is_err());
		clear();
		assert!(allocator.allocate(&request()).is_err());
		assert!(events().is_empty());
		assert_eq!(allocator.budget_snapshot(), snapshot);
		allocator.free(&mut a).unwrap();
		allocator.set_heap_limit(0, 0).unwrap();
		assert!(allocator.allocate(&request()).is_err());
		assert!(allocator.set_heap_limit(1, 0).is_err());
		assert!(allocator.set_heap_limit(0, (1 << 30) + 1).is_err());
		allocator.close().unwrap();
		assert_eq!(allocator.set_heap_limit(0, 0), Err(AllocError::Closed));
	}
}

#[test]
fn budget_queries_preserve_exact_core_or_khr_routes_and_explicit_provenance() {
	for khr in [false, true] {
		let fixture = Fixture::new_with_instance_extensions(
			if khr {
				vk::API_VERSION_1_0
			} else {
				vk::API_VERSION_1_1
			},
			&[c"VK_EXT_memory_budget"],
			if khr {
				&[c"VK_KHR_get_physical_device_properties2"]
			} else {
				&[]
			},
			true,
			None,
		);
		let allocator = fixture.allocator();
		STATE.with_borrow_mut(|s| {
			s.budget = 4096;
			s.usage = 8192;
		});
		// SAFETY: matching synthetic contexts and initialized native query chain.
		unsafe {
			let snapshot = allocator
				.query_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
				.unwrap();
			assert_eq!(snapshot.source, BudgetSource::Extension);
			// Native usage may exceed budget. Neither value is a reservation or allocation guarantee.
			assert_eq!(
				(snapshot.heaps[0].budget, snapshot.heaps[0].usage),
				(4096, 8192)
			);
			assert_eq!(events(), [if khr { "budget_khr" } else { "budget_core" }]);
			assert_eq!(allocator.budget_snapshot().source, BudgetSource::Local);
			STATE.with_borrow_mut(|s| s.budget = 0);
			assert!(
				allocator
					.query_budget(&fixture.instance, vk::PhysicalDevice::from_raw(0x3000))
					.is_err()
			);
		}
	}
}
