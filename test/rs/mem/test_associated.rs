//! Raw-handle association index mirrors live resource and pending claims.
use super::*;

const PLANE_0: vk::ImageAspectFlagBits = vk::ImageAspectFlagBits::PLANE_0_BIT;
const PLANE_1: vk::ImageAspectFlagBits = vk::ImageAspectFlagBits::PLANE_1_BIT;
const PLANE_2: vk::ImageAspectFlagBits = vk::ImageAspectFlagBits::PLANE_2_BIT;

fn index() -> Associations<'static> {
	Associations::new(HostMemory::default())
}

fn record(
	index: &mut Associations<'_>,
	resource: Resource,
	plane: Option<vk::ImageAspectFlagBits>,
	serial: u64,
) {
	index.try_reserve_for(resource).unwrap();
	index.record(resource, plane, serial);
}

#[test]
fn whole_resource_claims_are_indexed_and_conflicting() {
	let mut index = index();
	let buffer = Resource::Buffer(vk::Buffer::from_raw(77));
	assert!(!index.associated(buffer));
	assert!(!index.conflicts(buffer, None));

	record(&mut index, buffer, None, 1);
	assert!(index.associated(buffer));
	assert!(index.conflicts(buffer, None));
	assert!(index.conflicts(buffer, Some(PLANE_0)));

	// A different raw handle stays unclaimed.
	let other = Resource::Buffer(vk::Buffer::from_raw(78));
	assert!(!index.associated(other));
	assert!(!index.conflicts(other, None));
}

#[test]
fn buffer_and_image_registries_reject_type_confusion() {
	let mut index = index();
	let buffer = Resource::Buffer(vk::Buffer::from_raw(77));
	let image = Resource::Image(vk::Image::from_raw(77));

	record(&mut index, buffer, None, 1);
	assert!(index.associated(buffer));
	assert!(!index.associated(image));
	assert!(!index.conflicts(image, None));
}

#[test]
fn disjoint_planes_claim_independent_slots_on_one_image() {
	let mut index = index();
	let image = Resource::Image(vk::Image::from_raw(77));

	record(&mut index, image, Some(PLANE_0), 1);
	record(&mut index, image, Some(PLANE_1), 2);

	// A whole-image claim and each owned plane conflict; the free plane does not.
	assert!(index.conflicts(image, None));
	assert!(index.conflicts(image, Some(PLANE_0)));
	assert!(index.conflicts(image, Some(PLANE_1)));
	assert!(!index.conflicts(image, Some(PLANE_2)));
}

#[test]
fn release_drops_only_the_owning_serial_and_removes_empty_keys() {
	let mut index = index();
	let image = Resource::Image(vk::Image::from_raw(77));

	record(&mut index, image, Some(PLANE_0), 1);
	record(&mut index, image, Some(PLANE_1), 2);

	// A foreign serial cannot steal the slot.
	index.release(image, Some(PLANE_0), 9);
	assert!(index.conflicts(image, Some(PLANE_0)));

	index.release(image, Some(PLANE_0), 1);
	assert!(!index.conflicts(image, Some(PLANE_0)));
	assert!(index.associated(image));

	index.release(image, Some(PLANE_1), 2);
	assert!(!index.associated(image));
	// Releasing an absent key is a no-op.
	index.release(image, None, 3);
}

#[test]
fn recycled_handles_keep_the_newer_association() {
	let mut index = index();
	let buffer = Resource::Buffer(vk::Buffer::from_raw(77));

	record(&mut index, buffer, None, 1);
	index.release(buffer, None, 1);
	record(&mut index, buffer, None, 2);

	index.release(buffer, None, 1);
	assert!(index.associated(buffer));
	assert!(index.conflicts(buffer, None));

	index.release(buffer, None, 2);
	assert!(!index.associated(buffer));
}
