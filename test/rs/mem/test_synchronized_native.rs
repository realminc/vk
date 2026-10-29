//! Explicit real-driver multi-thread host allocation/mapping qualification.
use vk::{
	self,
	mem::{AllocationCreateInfo, AllocationPolicy},
};

pub(super) unsafe fn contexts(
	extensions: &[&std::ffi::CStr],
) -> Option<(vk::Instance, vk::PhysicalDevice, vk::Device)> {
	// SAFETY: this fixture owns native parents; no borrowed create data escapes
	// synchronous Vulkan calls. Each caller explicitly destroys returned parents.
	unsafe {
		let entry = vk::Entry::new().unwrap();
		let app = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let instance = entry
			.create_instance(
				&vk::InstanceCreateInfo {
					p_application_info: &app,
					..Default::default()
				},
				None,
			)
			.unwrap();
		let selected = instance
			.enumerate_physical_devices()
			.unwrap()
			.into_iter()
			.find_map(|physical| {
				let properties = instance.get_physical_device_properties(physical).unwrap();
				if properties.api_version < vk::API_VERSION_1_1 {
					return None;
				}
				let supported = instance
					.enumerate_device_extension_properties(physical, None)
					.unwrap();
				if !extensions.iter().all(|name| {
					supported
						.iter()
						.any(|item| std::ffi::CStr::from_ptr(item.extension_name.as_ptr()) == *name)
				}) {
					return None;
				}
				instance
					.get_physical_device_queue_family_properties(physical)
					.unwrap()
					.iter()
					.position(|family| family.queue_count > 0)
					.map(|index| (physical, u32::try_from(index).unwrap()))
			});
		let Some((physical, family)) = selected else {
			eprintln!("SKIP: no Vulkan 1.1 device with required extensions {extensions:?}");
			instance.destroy(None);
			return None;
		};
		let properties = instance.get_physical_device_properties(physical).unwrap();
		eprintln!(
			"allocator native host profile: device={:?}, api={}, driver={}, granularity={}, atom={}, extensions={extensions:?}",
			std::ffi::CStr::from_ptr(properties.device_name.as_ptr()),
			properties.api_version,
			properties.driver_version,
			properties.limits.buffer_image_granularity,
			properties.limits.non_coherent_atom_size
		);
		let priority = 1.0;
		let queue = vk::DeviceQueueCreateInfo {
			queue_family_index: family,
			queue_count: 1,
			p_queue_priorities: &priority,
			..Default::default()
		};
		let names: Vec<_> = extensions.iter().map(|name| name.as_ptr()).collect();
		let device = instance
			.create_device(
				physical,
				&vk::DeviceCreateInfo {
					queue_create_info_count: 1,
					p_queue_create_infos: &queue,
					enabled_extension_count: u32::try_from(names.len()).unwrap(),
					pp_enabled_extension_names: names.as_ptr(),
					..Default::default()
				},
				None,
			)
			.unwrap();
		Some((instance, physical, device))
	}
}

fn buffer_info() -> vk::BufferCreateInfo<'static> {
	vk::BufferCreateInfo {
		size: 256,
		usage: vk::BufferUsageFlagBits::TRANSFER_SRC_BIT | vk::BufferUsageFlagBits::TRANSFER_DST_BIT,
		..Default::default()
	}
}

unsafe fn exercise(
	shared: &vk::SynchronizedAllocator<'_>,
	worker: u8,
	options: &AllocationCreateInfo,
) {
	// SAFETY: this worker owns every returned token/resource and mapped extent;
	// uses are host-only, cache operations/holds balance, no pointer survives free.
	unsafe {
		for iteration in 0..32_u8 {
			let mut buffers = Vec::new();
			for _ in 0..4 {
				buffers.push(
					shared
						.create_buffer_with_options(&buffer_info(), options)
						.unwrap(),
				);
			}
			for (buffer, mut allocation) in buffers {
				let pointer = shared.map(&allocation).unwrap();
				// Byte access intentionally leaves the pool/owner lock. The live
				// region and map hold survive other workers' allocations and frees.
				let bytes = std::slice::from_raw_parts_mut(pointer.cast::<u8>(), 256);
				let pattern = worker.wrapping_mul(53).wrapping_add(iteration);
				bytes.fill(pattern);
				std::thread::yield_now();
				shared.flush(&allocation, 0, 256).unwrap();
				shared.invalidate(&allocation, 0, 256).unwrap();
				assert!(bytes.iter().all(|&byte| byte == pattern));
				shared.unmap(&allocation).unwrap();
				shared.destroy_buffer(buffer, &mut allocation).unwrap();
			}
		}
	}
}

unsafe fn run(parallel: bool) {
	// SAFETY: all workers join before explicit child and parent destruction;
	// native buffers/memory and all cache ranges use queried device metadata.
	unsafe {
		let (instance, physical, device) = contexts(&[]).expect("Vulkan 1.1 device");
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let mut pools = Vec::new();
		if parallel {
			let index = allocator
				.find_memory_type_for_buffer_info(
					&buffer_info(),
					&AllocationCreateInfo {
						required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
						..Default::default()
					},
				)
				.unwrap();
			for _ in 0..4 {
				pools.push(
					allocator
						.create_pool(&vk::mem::PoolCreateInfo {
							memory_type_index: index,
							block_size: 65536,
							..Default::default()
						})
						.unwrap(),
				);
			}
		}
		let shared = vk::SynchronizedAllocator::new(allocator);
		if parallel {
			let selected: Vec<_> = pools.iter().collect();
			shared
				.with_pools(&selected, |children| {
					std::thread::scope(|scope| {
						for (worker, (child, pool)) in children.iter().zip(&pools).enumerate() {
							scope.spawn(move || {
								exercise(
									child,
									worker as u8,
									&AllocationCreateInfo {
										pool: Some(pool),
										required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
										..Default::default()
									},
								)
							});
						}
					});
				})
				.unwrap();
		} else {
			std::thread::scope(|scope| {
				for worker in 0..4_u8 {
					let shared = &shared;
					scope.spawn(move || {
						exercise(
							shared,
							worker,
							&AllocationCreateInfo {
								policy: AllocationPolicy::Pooled { block_size: 65536 },
								required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
								..Default::default()
							},
						)
					});
				}
			});
		}
		assert_eq!(shared.statistics().unwrap().allocation_count, 0);
		let mut allocator = shared.into_inner().unwrap();
		for pool in &mut pools {
			allocator.destroy_pool(pool).unwrap();
		}
		allocator.close().unwrap();
		assert_eq!(allocator.statistics().native_allocation_count, 0);
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
		eprintln!("PASS: parallel={parallel}, four workers, 512 buffer/map/cache/cleanup lifecycles");
	}
}

#[test]
fn serialized_native_buffer_mapping_across_host_threads() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: explicit gated host-only native profile, complete cleanup above.
	unsafe { run(false) };
}

#[test]
fn parallel_native_pools_mapping_across_host_threads() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: independent native pool vectors, joined workers and explicit cleanup.
	unsafe { run(true) };
}

#[test]
fn independent_blocks_bind_and_map_concurrently_inside_one_pool() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: exact ordinary buffer requirements, fixed-type pool and independent
	// backing objects. Workers join before explicit matching paired cleanup.
	unsafe {
		let (instance, physical, device) = contexts(&[]).unwrap();
		let mut allocator =
			vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let info = buffer_info();
		let index = allocator
			.find_memory_type_for_buffer_info(
				&info,
				&AllocationCreateInfo {
					required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
					..Default::default()
				},
			)
			.unwrap();
		let temporary = device.create_buffer(&info, None).unwrap();
		let requirements = device.get_buffer_memory_requirements(temporary).unwrap();
		device.destroy_buffer(temporary, None).unwrap();
		let mut pool = allocator
			.create_pool(&vk::mem::PoolCreateInfo {
				memory_type_index: index,
				block_size: requirements.size,
				min_block_count: 4,
				max_block_count: 4,
				..Default::default()
			})
			.unwrap();
		let mut buffers = Vec::new();
		for _ in 0..4 {
			let buffer = device.create_buffer(&info, None).unwrap();
			let allocation = allocator
				.allocate_for_buffer(
					buffer,
					&AllocationCreateInfo {
						pool: Some(&pool),
						..Default::default()
					},
				)
				.unwrap();
			buffers.push((buffer, allocation));
		}
		let shared = vk::SynchronizedAllocator::new(allocator);
		let seeds = buffers
			.iter()
			.map(|(_, allocation)| allocation)
			.collect::<Vec<_>>();
		shared
			.with_blocks(&seeds, |children| {
				std::thread::scope(|scope| {
					for (index, (child, (buffer, allocation))) in children.iter().zip(&buffers).enumerate() {
						scope.spawn(move || {
							child
								.with_allocator(|allocator| {
									allocator.bind_buffer_memory(*buffer, allocation).unwrap();
									for iteration in 0..32_u8 {
										let pointer = allocator.map(allocation).unwrap().cast::<u8>();
										let bytes = std::slice::from_raw_parts_mut(pointer, 256);
										let pattern = (index as u8).wrapping_mul(31).wrapping_add(iteration);
										bytes.fill(pattern);
										allocator.flush(allocation, 0, 256).unwrap();
										allocator.invalidate(allocation, 0, 256).unwrap();
										assert!(bytes.iter().all(|&byte| byte == pattern));
										allocator.unmap(allocation).unwrap();
									}
								})
								.unwrap()
						});
					}
				});
			})
			.unwrap();
		let mut allocator = shared.into_inner().unwrap();
		for (buffer, allocation) in &mut buffers {
			allocator.destroy_buffer(*buffer, allocation).unwrap();
		}
		allocator.destroy_pool(&mut pool).unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
		eprintln!(
			"PASS: four independent blocks in one pool, four native binds, 128 map/cache/unmap cycles"
		);
	}
}

#[test]
fn default_memory_type_scope_preserves_native_buffer_workload() {
	if std::env::var("VULKAN_AVAILABLE").as_deref() != Ok("1") {
		return;
	}
	// SAFETY: queried host-visible default vector, joined workers, explicit cleanup.
	unsafe {
		let (instance, physical, device) = contexts(&[]).unwrap();
		let allocator = vk::Allocator::new_from_physical_device(&instance, physical, &device).unwrap();
		let index = allocator
			.find_memory_type_for_buffer_info(
				&buffer_info(),
				&AllocationCreateInfo {
					required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
					..Default::default()
				},
			)
			.unwrap();
		let temporary = device.create_buffer(&buffer_info(), None).unwrap();
		let requirements = device.get_buffer_memory_requirements(temporary).unwrap();
		device.destroy_buffer(temporary, None).unwrap();
		let indices = (0..allocator.memory_properties().unwrap().memory_type_count)
			.filter(|&candidate| {
				allocator
					.find_memory_type(
						requirements.memory_type_bits & (1 << candidate),
						&AllocationCreateInfo {
							required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
							..Default::default()
						},
					)
					.is_ok()
			})
			.collect::<Vec<_>>();
		assert!(indices.contains(&index));
		let shared = vk::SynchronizedAllocator::new(allocator);
		shared
			.with_memory_types(&indices, |children| {
				std::thread::scope(|scope| {
					for worker in 0..4_u8 {
						let child = &children[usize::from(worker) % children.len()];
						scope.spawn(move || {
							exercise(
								child,
								worker,
								&AllocationCreateInfo {
									policy: AllocationPolicy::Automatic,
									required_flags: vk::MemoryPropertyFlagBits::HOST_VISIBLE_BIT,
									..Default::default()
								},
							)
						});
					}
				})
			})
			.unwrap();

		let mut allocator = shared.into_inner().unwrap();
		allocator.close().unwrap();
		drop(allocator);
		device.destroy(None);
		instance.destroy(None);
		eprintln!("PASS: native default vectors memory_types={indices:?}, 512 buffer lifecycles");
	}
}
