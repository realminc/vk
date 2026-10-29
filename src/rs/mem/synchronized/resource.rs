//! Preserve allocator-wide primary-resource admission across pool partitions.
//! Claims cover pending native operations without holding the shared lock through
//! a PFN. Distinct disjoint image planes retain the canonical binding contract.
use super::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Owner {
	#[default]
	Empty,
	Pending(u64),
	Allocation(u64),
}

#[derive(Clone, Copy, Default)]
pub(super) struct Owners([Owner; 4]);

pub(super) struct Resources<'device> {
	buffers: HostMap<'device, Owners>,
	images: HostMap<'device, Owners>,
}

fn slot(plane: Option<vk::ImageAspectFlagBits>) -> usize {
	match plane {
		Some(vk::ImageAspectFlagBits::PLANE_0_BIT) => 1,
		Some(vk::ImageAspectFlagBits::PLANE_1_BIT) => 2,
		Some(vk::ImageAspectFlagBits::PLANE_2_BIT) => 3,
		None => 0,
		_ => unreachable!("canonical binding validates plane aspects"),
	}
}

impl<'device> Resources<'device> {
	pub fn new(root: &Allocator<'device>) -> Result<Self, AllocError> {
		let mut resources = Self {
			buffers: HostMap::new(root.host),
			images: HostMap::new(root.host),
		};
		for live in root.live.values() {
			if let Some(resource) = live.resource {
				let (registry, key) = resources.registry(resource);
				if registry.get(&key).is_none() {
					registry.try_reserve(1)?;
					registry.insert_reserved(key, Owners::default());
				}
				registry.get_mut(&key).expect("reserved resource").0[slot(live.plane)] =
					Owner::Allocation(live.serial);
			}
		}
		Ok(resources)
	}

	fn registry(&mut self, resource: Resource) -> (&mut HostMap<'device, Owners>, u64) {
		match resource {
			Resource::Buffer(buffer) => (&mut self.buffers, buffer.as_raw()),
			Resource::Image(image) => (&mut self.images, image.as_raw()),
		}
	}

	pub fn associated(&self, resource: Resource, domain: u64) -> bool {
		let owners = match resource {
			Resource::Buffer(buffer) => self.buffers.get(&buffer.as_raw()),
			Resource::Image(image) => self.images.get(&image.as_raw()),
		};
		owners.is_some_and(|owners| {
			owners
				.0
				.iter()
				.any(|owner| !matches!(owner, Owner::Empty) && *owner != Owner::Pending(domain))
		})
	}

	fn claim(
		&mut self,
		resource: Resource,
		plane: Option<vk::ImageAspectFlagBits>,
		domain: u64,
		existing: Option<u64>,
	) -> Result<bool, AllocError> {
		let (registry, key) = self.registry(resource);
		let selected = slot(plane);
		if let Some(owners) = registry.get_mut(&key) {
			for (index, owner) in owners.0.iter().enumerate() {
				if *owner == Owner::Empty || (selected != 0 && index != 0 && index != selected) {
					continue;
				}
				if index == selected
					&& (*owner == Owner::Pending(domain)
						|| existing.is_some_and(|serial| *owner == Owner::Allocation(serial)))
				{
					return Ok(false);
				}
				return Err(AllocError::WrongResource);
			}
			owners.0[selected] = Owner::Pending(domain);
		} else {
			registry.try_reserve(1)?;
			let mut owners = Owners::default();
			owners.0[selected] = Owner::Pending(domain);
			registry.insert_reserved(key, owners);
		}
		Ok(true)
	}

	fn commit(&mut self, resource: Resource, plane: Option<vk::ImageAspectFlagBits>, serial: u64) {
		let (registry, key) = self.registry(resource);
		registry.get_mut(&key).expect("claimed resource").0[slot(plane)] = Owner::Allocation(serial);
	}

	fn release(&mut self, resource: Resource, plane: Option<vk::ImageAspectFlagBits>, owner: Owner) {
		let (registry, key) = self.registry(resource);
		if let Some(owners) = registry.get_mut(&key) {
			let selected = slot(plane);
			if owners.0[selected] == owner {
				owners.0[selected] = Owner::Empty;
			}
			if owners.0.iter().all(|owner| *owner == Owner::Empty) {
				registry.remove(&key);
			}
		}
	}
}

pub(in crate::mem::allocator) struct Claim<'device> {
	shared: std::ptr::NonNull<Shared<'device>>,
	resource: Resource,
	plane: Option<vk::ImageAspectFlagBits>,
	domain: u64,
	acquired: bool,
}

impl<'device> Claim<'device> {
	fn shared(&self) -> &Shared<'device> {
		// SAFETY: claim is private, retained only within a canonical operation;
		// the enclosing pool scope outlives every operation and unwind guard.
		unsafe { self.shared.as_ref() }
	}

	pub fn commit(mut self, serial: u64) {
		self
			.shared()
			.lock()
			.resources
			.commit(self.resource, self.plane, serial);
		self.acquired = false;
	}
}

impl Drop for Claim<'_> {
	fn drop(&mut self) {
		if self.acquired {
			self.shared().lock().resources.release(
				self.resource,
				self.plane,
				Owner::Pending(self.domain),
			);
		}
	}
}

// This guard borrows the stable shared context, rather than the child allocator;
// canonical operations can retain it while exclusively mutating their own domain.
pub(in crate::mem::allocator) fn claim<'device>(
	allocator: &Allocator<'device>,
	resource: Resource,
	plane: Option<vk::ImageAspectFlagBits>,
	existing: Option<u64>,
) -> Result<Option<Claim<'device>>, AllocError> {
	let Some(pointer) = allocator.parallel else {
		return Ok(None);
	};
	// SAFETY: scoped child construction keeps this callback-owned context live
	// until every child/operation/guard finishes; no reference escapes an operation.
	let pointer = pointer.cast::<Shared<'device>>();
	let shared = unsafe { pointer.as_ref() };
	let domain = allocator.partition_id;
	let acquired = shared
		.lock()
		.resources
		.claim(resource, plane, domain, existing)?;
	Ok(Some(Claim {
		shared: pointer,
		resource,
		plane,
		domain,
		acquired,
	}))
}

pub(super) fn retire(
	state: &mut State<'_>,
	resource: Option<Resource>,
	plane: Option<vk::ImageAspectFlagBits>,
	serial: u64,
) {
	if let Some(resource) = resource {
		state
			.resources
			.release(resource, plane, Owner::Allocation(serial));
	}
}

#[cfg(test)]
#[path = "../../../../test/rs/mem/synchronized/test_resource.rs"]
mod tests;
