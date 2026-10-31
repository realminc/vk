<p align="center">
  <a href="https://dev.realm.software/vk">
    <img src="https://dev.realm.software/images/realm/coverVulkan.webp" width="100%" alt="VK — native Rust Vulkan bindings, cached dispatch and allocator">
  </a>
</p>

# vk

**Vulkan in Rust. Native bindings, cached dispatch and an integrated allocator.**

Registry-generated Vulkan bindings for graphics, games and GPU compute,
including the low-level building blocks for AI and ML workloads. The Cargo
package is **`vkrust`**; the Rust library and import namespace are **`vk`**.

[![Crates.io](https://img.shields.io/crates/v/vkrust.svg)](https://crates.io/crates/vkrust)
[![Documentation](https://docs.rs/vkrust/badge.svg)](https://docs.rs/vkrust/latest/vk/)
[![CI](https://github.com/realminc/vk/actions/workflows/ci.yml/badge.svg)](https://github.com/realminc/vk/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/realminc/vk/blob/main/LICENSE)

[Realm](https://realm.software/) · [Developer guide](https://dev.realm.software/vk) · [Rust API reference](https://docs.rs/vkrust/latest/vk/)

The project is **experimental**. Registry coverage, enabled device capabilities
and runtime qualification are separate; see the
[qualification and limitations](https://github.com/realminc/vk/blob/main/docs/external/qualification.md).

## Install

```toml
[dependencies]
vk = { package = "vkrust", version = "0.9" }
```

No Vulkan SDK, C compiler, C++ library or consumer-side generator is needed.
The default `loader` feature loads the system Vulkan library at runtime.

## Library and Vulkan versions

The package version is **`0.9.0+1.4.365`**:

| Version | Meaning |
| --- | --- |
| `0.9.0` | This Rust library's API release; Git tag `v0.9.0` |
| `+1.4.365` | Generated from Vulkan 1.4 headers, revision 365 |
| `ApplicationInfo::api_version` | The Vulkan API version your application requests at runtime |

Cargo treats the suffix as build metadata, so the dependency requirement stays
`version = "0.9"`. The suffix identifies the pinned API snapshot; it is not a
Vulkan 1.4 driver requirement or runtime qualification claim. Applications can
request Vulkan 1.0, 1.1 or 1.2 and use the commands/features admitted by their
actual implementation. The examples below request Vulkan 1.0.

`vk::HEADER_VERSION` and `vk::HEADER_VERSION_COMPLETE` expose the corresponding
native header constants. See the
[registry provenance](https://github.com/realminc/vk/blob/main/registry/SOURCES.md)
for the exact pinned Khronos commit.

## Two calling styles, one implementation

Use free functions when you prefer Vulkan's C-style flow, or associated methods
when you prefer an object-oriented, Python-like spelling. Both use the same
cached PFNs and canonical operation body.

The examples use the borrowed record setters introduced in v0.5.0. These
retain backing host storage while preparing arguments for either calling style.

| Operation | Free function | Constructor or method |
| --- | --- | --- |
| Create instance | `vk::create_instance(&entry, &info, None)` | `vk::Instance::new(&entry, &info, None)` |
| Enumerate GPUs | `vk::enumerate_physical_devices(&instance)` | `instance.enumerate_physical_devices()` |
| Create device | `vk::create_device(&instance, physical_device, &info, None)` | `vk::Device::new(&instance, physical_device, &info, None)` |
| Retrieve queue | `vk::get_device_queue(&device, family, index)` | `device.get_device_queue(family, index)` |
| Record dispatch | `vk::cmd_dispatch(&device, commands, x, y, z)` | `device.cmd_dispatch(commands, x, y, z)` |
| Destroy device | `vk::destroy_device(device, None)` | `device.destroy(None)` |

Free functions take their dispatch context explicitly. There is no mutable
global dispatch or per-call symbol lookup. Both styles retain Vulkan's unsafe
valid-usage and synchronization requirements; method syntax is not a safe API
or a Python binding.

### Free functions

Create an instance, enumerate physical devices, then explicitly destroy it.
The enumeration result is propagated after cleanup, including on query failure.

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: use the trusted system Vulkan loader; all create-info pointers
    // remain live during creation. This example creates no child objects.
    unsafe {
        let entry = vk::Entry::new()?;
        let application = vk::ApplicationInfo::default()
            .application_name(c"vk example")
            .api_version(vk::API_VERSION_1_0);
        let info = vk::InstanceCreateInfo::default()
            .application_info(&application);

        let instance = vk::create_instance(&entry, &info, None)?;
        let devices = vk::enumerate_physical_devices(&instance);
        vk::destroy_instance(instance, None);

        println!("Physical devices: {}", devices?.len());
    }
    Ok(())
}
```

### Constructors and methods

The equivalent instance lifecycle through `Instance`:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: use the trusted system loader; borrowed create-info storage
    // remains live. No children or outstanding work remain at destruction.
    unsafe {
        let entry = vk::Entry::new()?;
        let application = vk::ApplicationInfo::default()
            .application_name(c"vk example")
            .api_version(vk::API_VERSION_1_0);
        let info = vk::InstanceCreateInfo::default()
            .application_info(&application);

        let instance = vk::Instance::new(&entry, &info, None)?;
        let devices = instance.enumerate_physical_devices();
        instance.destroy(None);

        println!("Physical devices: {}", devices?.len());
    }
    Ok(())
}
```

If native creation succeeds but dispatch initialization cannot recover or
destroy the object, `CallError::InitializationFailed` retains its handle and
library. Applications must handle that variant explicitly to recover ownership;
the short examples propagate it to their caller.

### Borrowed record construction

Use `Type::default()` followed by generated setters to prepare native arguments.
This is the default construction approach, available without an additional
feature flag. It works with both free functions and methods;
these setters construct the same Vulkan records passed to either calling style.
Direct native fields remain available, and stored borrowed records carry
lifetime parameters.

```rust
fn main() -> Result<(), vk::ConstructionError> {
    let application = vk::ApplicationInfo::default()
        .application_name(c"vk example")
        .api_version(vk::API_VERSION_1_0);
    let info = vk::InstanceCreateInfo::default()
        .application_info(&application);

    let priorities = [1.0_f32];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(0)
        .queue_priorities(&priorities)?];
    let device_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)?;

    assert_eq!(info.p_application_info, &application as *const _);
    assert_eq!(device_info.queue_create_info_count, 1);
    Ok(())
}
```

Borrowed setters keep backing strings, records and arrays live through their
last use. Array setters derive checked native counts; reviewed parallel arrays
are set together. `push_next` admits registry-compatible chain types and retains
an exclusive borrow of each inserted node:

```rust,no_run
/// The caller supplies a compatible physical device and Vulkan 1.2 query context.
unsafe fn timeline_semaphore_supported(
    instance: &vk::Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<bool, vk::CallError> {
    let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
    {
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut vulkan12)?;
        // SAFETY: the caller establishes the native query's valid usage.
        unsafe { instance.get_physical_device_features2(physical_device, &mut features)? };
    }
    Ok(vulkan12.timeline_semaphore != 0)
}
# fn main() {}
```

These methods prepare the same native records. They do not create GPU objects,
validate all Vulkan usage, enable features or prove GPU completion. Public raw
pointers remain an unsafe-call obligation; borrowing a pointer array keeps the
array alive, not the objects behind its pointers. Existing bitfield getters and
`set_*` accessors retain their spelling.

Version 0.6.0 completes construction coverage for all 1,552 pinned records/unions,
with 5,450 setters covering byte payloads, SPIR-V words, optional parallel arrays,
initialized writable outputs and reviewed parent/cross-record capacities. Opaque
addresses and foreign objects without a known host extent use explicit unsafe
pointer setters. See the [changelog](CHANGELOG.md) for compatibility changes.

For migration, use `Type { field: value, ..Default::default() }` rather than an
exhaustive literal, and declare stored borrowed records as `Type<'a>`. Records
retaining exclusive chain borrows no longer implement `Copy` or `Clone`; create
fresh array elements with `std::array::from_fn` and borrow single records with
`std::slice::from_ref`. The native union
`AccelerationStructureGeometryDataKHR` uses `ManuallyDrop` for its non-Copy
record members. Native field layout remains subject to the independent C ABI
gate. Raw handle lifetimes and explicit destruction requirements are unchanged.

### Mix styles when recording commands

An existing compute recording can use either spelling without creating another
device or dispatch table:

```rust,no_run
/// # Safety
/// `commands` must be recording outside a render pass, from a pool on this
/// device's compute-capable queue family. `pipeline` must be a compatible live
/// compute pipeline; descriptor sets, push constants and resource barriers
/// required by its shader must already be valid for this dispatch.
unsafe fn record_compute(
    device: &vk::Device,
    commands: vk::CommandBuffer,
    pipeline: vk::Pipeline,
) -> Result<(), vk::CallError> {
    unsafe {
        vk::cmd_bind_pipeline(device, commands, vk::PipelineBindPoint::COMPUTE, pipeline)?;
        device.cmd_dispatch(commands, 1, 1, 1)?;
    }
    Ok(())
}
```

Recording does not submit or wait. The application owns command-buffer state,
feature enablement, object lifetimes and host/GPU synchronization.

## Wrappers and raw handles

`Instance` and `Device` are the two main dispatch wrappers. They combine the
native identity with loaded commands, library lifetime retention and optional
owned creation-capability metadata. This is what lets `device.cmd_dispatch(...)`
know which native handle and PFN table to use.

| Rust type | Meaning |
| --- | --- |
| `vk::Instance` | Instance wrapper with cached instance dispatch |
| `vk::InstanceHandle` | Raw native `VkInstance` identity |
| `vk::Device` | Device wrapper with cached device dispatch |
| `vk::DeviceHandle` | Raw native `VkDevice` identity |

```rust,no_run
fn native_handles(instance: &vk::Instance, device: &vk::Device) {
    let instance_handle: vk::InstanceHandle = instance.handle();
    let device_handle: vk::DeviceHandle = device.handle();
    assert!(!instance_handle.is_null());
    assert!(!device_handle.is_null());
}
```

Handles are copyable identities, not owners or dispatch contexts. PFN signatures
use the raw handle types to preserve the Vulkan ABI. Other native handles keep
their familiar names: `PhysicalDevice`, `Queue`, `Buffer`, `Image` and so on.
Import existing instances/devices with unsafe `new_from_handle` or
`new_from_handle_with_capabilities`; importing must not create duplicate owners.

Native destruction remains explicit. `Drop` never destroys Vulkan objects,
submits work or waits. Destroy children and complete outstanding work before
destroying their device or instance, using callbacks compatible with creation.

## Native Vulkan, Rust spelling

The namespace is flat: bindings, StdVideo definitions, dispatch tables, free
functions and the `Instance` / `Device` wrappers live directly under `vk::`.
The two public subsystem modules are `vk::load` for library loading and
`vk::mem` for the Rust-native allocator.

| Item | Rust convention | Native → Rust example |
| --- | --- | --- |
| Types | `PascalCase` | `VkApplicationInfo` → `vk::ApplicationInfo` |
| Constants | `SCREAMING_SNAKE_CASE` | `VK_SUCCESS` → `vk::Result::SUCCESS` |
| Functions and methods | `snake_case` | `vkCmdDispatch` → `vk::cmd_dispatch` / `device.cmd_dispatch` |
| Struct members | `snake_case` | `pApplicationInfo` → `p_application_info` |
| Raw instance/device handles | Explicit `Handle` suffix | `VkDevice` → `vk::DeviceHandle` |

Vulkan acronyms, vendor suffixes, `StdVideo*` names and `PFN_vk*` aliases remain
recognizable. C-compatible representations preserve layout independently of
field spelling.

```rust
let info = vk::InstanceCreateInfo::default();
assert_eq!(info.s_type, vk::StructureType::INSTANCE_CREATE_INFO);
assert!(info.p_next.is_null());
assert!(vk::DeviceHandle::NULL.is_null());
```

Search the API reference using native names such as `VkDevice`,
`vkCreateInstance`, `VK_SUCCESS` or `pApplicationInfo`; generated rustdoc search
aliases lead to the Rust spellings. Vulkan declarations link to Khronos
reference pages. StdVideo definitions and registry-only constants link to their
pinned Khronos registry source. Reference pages follow Khronos's latest
specification; the bindings follow the
[pinned registry snapshot](https://github.com/realminc/vk/blob/main/registry/SOURCES.md).
The development ABI map retains both C and Rust field names for independent
layout checks; renaming fields does not reorder or change their native storage.

## Loading and capabilities

`vk::Entry::new()` opens the OS Vulkan loader. `FnEntry`, `FnInstance` and
`FnDevice` cache nullable native function pointers. Created wrappers retain
version, extension and enabled-feature provenance; known disabled commands are
filtered from dispatch at construction. Raw imports without metadata report
unknown provenance. Resolving a pointer does not establish feature enablement.

Command-specific arguments, feature requirements and pointer chains remain the
caller's responsibility. The
[API reference](https://docs.rs/vkrust/latest/vk/) documents the Rust contracts
alongside links to the native Vulkan requirements.

## Integrated allocator

Enable `features = ["allocator"]` for the Rust-native, VMA-derived allocator:

```toml
[dependencies]
vk = { package = "vkrust", version = "0.9", features = ["allocator"] }
```

`vk::mem` owns allocator operations and configuration; `vk::Allocator` is the
same type as `vk::mem::Allocator`. This keeps allocator-backed creation distinct
from the native binding `vk::create_buffer`, which creates an unbound buffer.
Use `use vk::mem as vma;` for a familiar local alias without another crate,
wrapper or runtime cost. Both spellings are equally valid; the canonical module
name in this repository is `mem`.

Allocator free functions and methods share one implementation, just like the
binding API. They take the allocator explicitly and retain Rust naming rather
than adding a second set of `vma*` symbols:

| Operation | Free function | Method |
| --- | --- | --- |
| Create, allocate and bind buffer | `vk::mem::create_buffer_with_options(&mut allocator, &info, &options)` | `allocator.create_buffer_with_options(&info, &options)` |
| Create, allocate and bind image | `vk::mem::create_image_with_options(&mut allocator, &info, &options)` | `allocator.create_image_with_options(&info, &options)` |
| Allocate existing buffer's memory | `vk::mem::allocate_for_buffer(&mut allocator, buffer, &options)` | `allocator.allocate_for_buffer(buffer, &options)` |
| Map an allocation | `vk::mem::map(&mut allocator, &allocation)` | `allocator.map(&allocation)` |
| Inspect an allocation | `vk::mem::allocation_info(&allocator, &allocation)` | `allocator.allocation_info(&allocation)` |
| Destroy managed buffer and free memory | `vk::mem::destroy_buffer(&mut allocator, buffer, &mut allocation)` | `allocator.destroy_buffer(buffer, &mut allocation)` |

The following helper returns both objects to its caller for explicit cleanup.
The caller supplies an allocator constructed for its device, preferably with
`Allocator::new_from_physical_device(instance, physical_device, device)`:

```rust,no_run
# #[cfg(feature = "allocator")]
# mod allocator_example {
use vk::mem as vma;

/// # Safety
/// The allocator's device must be live and support this buffer size and usage.
/// The caller must destroy the returned buffer and free its allocation after
/// all host and GPU uses complete.
unsafe fn create_upload_buffer(
    allocator: &mut vk::Allocator<'_>,
) -> Result<(vk::Buffer, vk::Allocation), vk::AllocError> {
    let info = vk::BufferCreateInfo::default()
        .size(4096)
        .usage(vk::BufferUsageFlags::TRANSFER_SRC_BIT)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let options = vma::AllocationCreateInfo {
        usage: vma::MemoryUsage::PreferHost,
        host_access: vma::HostAccess::SequentialWrite,
        required_flags: vk::MemoryPropertyFlags::HOST_VISIBLE_BIT,
        policy: vma::AllocationPolicy::Automatic,
        ..Default::default()
    };

    // Method spelling: allocator.create_buffer_with_options(&info, &options).
    // SAFETY: the caller establishes the native valid-usage requirements.
    unsafe { vma::create_buffer_with_options(allocator, &info, &options) }
}

/// # Safety
/// Complete every host/GPU use and release explicit map holds first. The buffer
/// must still be live and paired with this allocation in this allocator.
unsafe fn destroy_completed_buffer(
    allocator: &mut vk::Allocator<'_>,
    buffer: vk::Buffer,
    allocation: &mut vk::Allocation,
) -> Result<(), vk::AllocError> {
    // Free-function spelling: vma::destroy_buffer(allocator, buffer, allocation).
    // SAFETY: the caller supplies completion and the matching live pair.
    unsafe { allocator.destroy_buffer(buffer, allocation) }
}
# }
# fn main() {}
```

The allocator implements managed buffers/non-disjoint images, mixed-resource
TLSF/linear pools, automatic memory preferences, mapping and cache operations,
budgets, callbacks, statistics, virtual placement and explicit multi-pass
relocation. `SynchronizedAllocator` exposes direct `&self` methods. Its map,
unmap and bind calls use a stable mutex per native backing, so another backing
can allocate/free or map/bind concurrently, including within the same pool.
Placement remains serialized per owner; borrowed pool/type scopes provide
independent placement owners. Borrowed results, relocation and global services
use exclusive `with_allocator` access and wait for active native calls.
`SynchronizedVirtualBlock` optionally serializes host-only placement; VMA's
virtual blocks themselves require external synchronization. Ordinary `Allocator`
remains exclusive and non-Send/non-Sync. The `new_retained` constructor
family produces `Allocator<'static>` backed by an owned `Arc<Device>` clone,
letting an allocator outlive its construction scope and finish on a dedicated
cleanup thread; `SynchronizedAllocator::new_owned` adopts it directly.

The pinned VMA implementation is a development oracle; consumers need no C++
allocator or FFI bridge. The broad allocator service surface is implemented,
but remains **experimental**. Full VMA equivalence, wider concurrent stress,
platform and hardware execution, and comparative performance remain unqualified.
Allocations and native resources need explicit cleanup. Host locks do not order
GPU work or protect escaped mapped bytes; operations and `Drop` never submit or
wait for the GPU. See [qualification and limitations](https://github.com/realminc/vk/blob/main/docs/external/qualification.md)
for the exact scope.

## Features

| Cargo feature | Compiled functionality |
| --- | --- |
| `loader` (default) | Dynamic loading, instance/device wrappers and native operations |
| `allocator` | Experimental Rust-native allocator in `vk::mem`; enables `loader` |
| `platform-*` | Matching platform-specific ABI declarations |
| `provisional` | Provisional registry declarations |

Set `default-features = false` for ABI declarations and raw dispatch tables
without the dynamic loader. Cargo features do not enable Vulkan device
extensions or features.

## Development

```bash
cargo test --workspace --all-features
cargo run -p vkrust-gen -- --registry registry/vk.xml --video-registry registry/video.xml --output . --check
```

Runtime source lives in `src/rs`, generation in `tool/gen`, and tests in
`test/rs`. Binding source and tests are flat; `load` and `mem` have their own
subdirectories. Generated files use singular `*.gen.rs` names. Dev/test builds
disable incremental compilation and debug information.

<p align="center">
  <a href="https://realm.software/">
    <img src="https://raw.githubusercontent.com/realminc/oa/main/sdk/asset/docs/readme/realmIdentityAscii.gif" width="100%" alt="Realm animated ASCII identity">
  </a>
</p>

## License

[MIT](https://github.com/realminc/vk/blob/main/LICENSE). See
[third-party notices](https://github.com/realminc/vk/blob/main/NOTICE.md) for
Khronos registry and AMD allocator provenance. Realm animation shared with
[OA](https://github.com/realminc/oa).
