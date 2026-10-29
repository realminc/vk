//! Independent resource ownership claims, including native identity reuse and planes.
use super::*;

#[test]
fn pending_claims_planes_and_recycled_handles_keep_exact_generation_ownership() {
	let mut resources = Resources {
		buffers: HostMap::new(HostMemory::default()),
		images: HostMap::new(HostMemory::default()),
	};
	let buffer = Resource::Buffer(vk::Buffer::from_raw(77));
	let image = Resource::Image(vk::Image::from_raw(77));
	assert_eq!(resources.claim(buffer, None, 1, None), Ok(true));
	assert!(!resources.associated(buffer, 1));
	assert!(resources.associated(buffer, 2));
	assert_eq!(
		resources.claim(buffer, None, 2, None),
		Err(AllocError::WrongResource)
	);
	assert_eq!(resources.claim(buffer, None, 1, None), Ok(false));
	resources.commit(buffer, None, 10);
	resources.release(buffer, None, Owner::Pending(1));
	assert!(resources.associated(buffer, 1));
	assert_eq!(resources.claim(buffer, None, 1, Some(10)), Ok(false));
	assert_eq!(
		resources.claim(buffer, None, 2, Some(11)),
		Err(AllocError::WrongResource)
	);
	// Retire before native destruction; a different pool may receive the same
	// raw native handle before the old allocation's memory retirement completes.
	resources.release(buffer, None, Owner::Allocation(10));
	assert_eq!(resources.claim(buffer, None, 2, None), Ok(true));
	resources.commit(buffer, None, 11);
	resources.release(buffer, None, Owner::Allocation(10));
	assert!(resources.associated(buffer, 1));
	let plane0 = Some(vk::ImageAspectFlagBits::PLANE_0_BIT);
	let plane1 = Some(vk::ImageAspectFlagBits::PLANE_1_BIT);
	assert_eq!(resources.claim(image, plane0, 1, None), Ok(true));
	assert_eq!(resources.claim(image, plane1, 2, None), Ok(true));
	assert_eq!(
		resources.claim(image, None, 3, None),
		Err(AllocError::WrongResource)
	);
	resources.commit(image, plane0, 20);
	resources.commit(image, plane1, 21);
	assert_eq!(
		resources.claim(image, plane0, 3, None),
		Err(AllocError::WrongResource)
	);
	resources.release(image, plane0, Owner::Allocation(20));
	assert_eq!(resources.claim(image, plane0, 3, None), Ok(true));
	resources.release(image, plane0, Owner::Pending(3));
	assert_eq!(
		resources.claim(image, None, 3, None),
		Err(AllocError::WrongResource)
	);
	resources.release(image, plane1, Owner::Allocation(21));
	assert_eq!(resources.claim(image, None, 3, None), Ok(true));
	assert_eq!(
		resources.claim(image, plane0, 1, None),
		Err(AllocError::WrongResource)
	);
	resources.release(image, None, Owner::Pending(3));
	resources.release(buffer, None, Owner::Allocation(11));
	assert!(resources.buffers.is_empty() && resources.images.is_empty());
}
