//! Default-pool policy and multi-pass lifetime contracts against native effect traces.
use super::*;
use crate::mem::{AllocationChain, DefragmentationContext, DefragmentationInfo};

#[test]
fn automatic_defaults_reuse_heap_sized_blocks_and_close_releases_the_spare() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|state| state.pooled = true);
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Automatic,
		..Default::default()
	};
	let requirements = vk::MemoryRequirements {
		size: 257,
		alignment: 64,
		memory_type_bits: 1,
	};
	// SAFETY: synthetic raw requirements and static mock PFNs, no real native ownership.
	unsafe {
		let mut first = allocator
			.allocate_with_options(&requirements, &options)
			.unwrap();
		let mut second = allocator
			.allocate_with_options(
				&requirements,
				&AllocationCreateInfo {
					never_allocate: true,
					..options
				},
			)
			.unwrap();
		assert_eq!(first.handle(), second.handle());
		assert_ne!(first.offset(), second.offset());
		assert_eq!(
			STATE.with_borrow(|state| state.allocated.clone()),
			vec![(0, 16 * 1024 * 1024)]
		);
		allocator.free(&mut first).unwrap();
		allocator.free(&mut second).unwrap();
	}
	assert_eq!(allocator.statistics().native_allocation_count, 1);
	allocator.close().unwrap();
	assert_eq!(allocator.statistics().native_allocation_count, 0);
	assert_eq!(events().iter().filter(|&&event| event == "free").count(), 1);
}

#[test]
fn explicit_separate_override_bypasses_automatic_pooling_and_rejects_reuse_conflict() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	STATE.with_borrow_mut(|state| state.pooled = true);
	let requirements = vk::MemoryRequirements {
		size: 257,
		alignment: 64,
		memory_type_bits: 1,
	};
	let options = AllocationCreateInfo {
		policy: AllocationPolicy::Automatic,
		dedicated_memory: true,
		..Default::default()
	};
	// SAFETY: synthetic native context and valid raw requirements.
	unsafe {
		let mut allocation = allocator
			.allocate_with_options(&requirements, &options)
			.unwrap();
		assert!(allocator.allocation_info(&allocation).unwrap().is_separate);
		assert_eq!(
			STATE.with_borrow(|state| state.allocated.clone()),
			vec![(0, 257)]
		);
		allocator.free(&mut allocation).unwrap();
		clear();
		assert!(matches!(
			allocator.allocate_with_options(
				&requirements,
				&AllocationCreateInfo {
					never_allocate: true,
					..options
				}
			),
			Err(AllocError::InvalidRequest(_))
		));
	}
	assert!(events().is_empty());
}

#[test]
fn import_batches_reject_handle_reuse_before_any_native_effect() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	let import = vk::ImportMemoryFdInfoKHR {
		handle_type: vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD_BIT,
		fd: 7,
		..Default::default()
	};
	// SAFETY: inspection only; the batch must reject reuse before touching the synthetic fd.
	let chain = unsafe { AllocationChain::new_from_record(&import) }.unwrap();
	let options = AllocationCreateInfo {
		memory_allocate_next: Some(chain),
		..Default::default()
	};
	let requirements = vk::MemoryRequirements {
		size: 257,
		alignment: 64,
		memory_type_bits: 1,
	};
	// SAFETY: no actual import is admitted, synthetic native context only.
	assert!(matches!(
		unsafe { allocator.allocate_many(&requirements, &options, 2) },
		Err(AllocError::InvalidRequest(_))
	));
	assert!(events().is_empty());
}

#[test]
fn unfinished_context_pass_blocks_further_passes_and_context_completion() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	let mut context =
		DefragmentationContext::new(&mut allocator, &DefragmentationInfo::default()).unwrap();
	// SAFETY: empty synthetic allocator; no native sources or reservations exist.
	let pass = unsafe { context.begin_pass() }.unwrap();
	assert!(pass.is_empty());
	drop(pass);
	assert_eq!(context.finish(), Err(AllocError::LiveAllocations));
	// SAFETY: still no native sources; the context must reject the unfinished prior pass.
	assert!(matches!(
		unsafe { context.begin_pass() },
		Err(AllocError::LiveAllocations)
	));
}

#[test]
fn completed_empty_context_pass_is_repeatable_and_does_not_double_count() {
	let fixture = Fixture::new(vk::API_VERSION_1_1, &[], true, None);
	let mut allocator = fixture.allocator();
	let mut context =
		DefragmentationContext::new(&mut allocator, &DefragmentationInfo::default()).unwrap();
	for _ in 0..2 {
		// SAFETY: empty synthetic allocator, no native sources or copies.
		let mut pass = unsafe { context.begin_pass() }.unwrap();
		assert!(pass.is_empty());
		assert_eq!(pass.finish().unwrap(), Default::default());
		assert_eq!(pass.finish().unwrap(), Default::default());
	}
	assert_eq!(context.finish().unwrap(), Default::default());
}
