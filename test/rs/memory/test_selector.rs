//! Checked memory-type selection.
#![cfg(feature = "allocator")]

use ::vk::{
	self,
	memory::{AllocError, AllocRequest, MemoryTypeSelector},
};

fn properties() -> vk::PhysicalDeviceMemoryProperties {
	let mut p: vk::PhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
	p.memory_type_count = 3;
	p.memory_heap_count = 1;
	p.memory_heaps[0].size = 1 << 30;
	p.memory_types[0].property_flags = vk::MemoryPropertyFlagBits::DEVICE_LOCAL_BIT;
	p.memory_types[1].property_flags =
		vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT | vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	p.memory_types[2].property_flags = vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT;
	p
}

fn request() -> AllocRequest {
	AllocRequest {
		size: 1024,
		alignment: 64,
		memory_type_bits: 7,
		required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
		preferred_flags: vk::MemoryPropertyFlags::empty(),
	}
}

#[test]
fn avoided_properties_are_penalties_and_never_relax_required_properties() {
	let mut properties = properties();
	properties.memory_types[1].property_flags =
		vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT | vk::MemoryPropertyFlagBits::HOST_CACHED_BIT;
	let selector = MemoryTypeSelector::new(properties, 64).unwrap();
	let mut request = request();
	assert_eq!(
		selector
			.find_memory_type_with_preferences(&request, vk::MemoryPropertyFlagBits::HOST_CACHED_BIT),
		Ok(2)
	);
	request.required_flags = request.required_flags | vk::MemoryPropertyFlagBits::HOST_CACHED_BIT;
	assert_eq!(
		selector
			.find_memory_type_with_preferences(&request, vk::MemoryPropertyFlagBits::HOST_CACHED_BIT),
		Ok(1)
	);
}

#[test]
fn selection_obeys_compatibility_required_and_preferred_flags() {
	let selector = MemoryTypeSelector::new(properties(), 64).unwrap();
	let mut req = request();
	assert_eq!(selector.find_memory_type(&req), Ok(1));
	req.memory_type_bits = 4;
	assert_eq!(selector.find_memory_type(&req), Ok(2));
	req.memory_type_bits = 1;
	assert_eq!(
		selector.find_memory_type(&req),
		Err(AllocError::NoCompatibleMemoryType)
	);
	req.memory_type_bits = 7;
	req.preferred_flags = vk::MemoryPropertyFlagBits::HOST_COHERENT_BIT;
	assert_eq!(selector.find_memory_type(&req), Ok(1));
}

#[test]
fn malformed_memory_metadata_and_requests_fail_without_panics() {
	for atom in [0, 3, 63] {
		assert!(MemoryTypeSelector::new(properties(), atom).is_err());
	}
	let mut p = properties();
	p.memory_type_count = 33;
	assert!(MemoryTypeSelector::new(p, 64).is_err());
	let mut p = properties();
	p.memory_types[0].heap_index = 1;
	assert!(MemoryTypeSelector::new(p, 64).is_err());
	let selector = MemoryTypeSelector::new(properties(), 64).unwrap();
	let mut req = request();
	req.alignment = 0;
	assert!(matches!(
		selector.find_memory_type(&req),
		Err(AllocError::InvalidRequest(_))
	));
	req.alignment = 64;
	req.size = 0;
	assert!(matches!(
		selector.find_memory_type(&req),
		Err(AllocError::InvalidRequest(_))
	));
}

#[test]
fn gated_memory_types_are_not_implicitly_selected() {
	let mut p = properties();
	p.memory_types[1].property_flags =
		p.memory_types[1].property_flags | vk::MemoryPropertyFlagBits::DEVICE_COHERENT_BIT_AMD;
	let selector = MemoryTypeSelector::new(p, 64).unwrap();
	assert_eq!(selector.find_memory_type(&request()), Ok(2));
}
