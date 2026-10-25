# vk

Rust Vulkan bindings generated from the paired, pinned Khronos registries.
The Cargo package is `vkrust`; the Rust library and import namespace are `vk`.
The project is experimental. Registry coverage is not driver qualification.

```toml
[dependencies]
vk = { package = "vkrust", version = "0.3" }
```

```rust
let application = vk::ApplicationInfo {
    api_version: vk::API_VERSION_1_0,
    ..Default::default()
};
assert_eq!(application.s_type, vk::StructureType::APPLICATION_INFO);
```

`vk::core` owns all Vulkan ABI declarations, including StdVideo codec records,
enums and constants, alongside raw handles, cached PFN tables and native
operations. Its exports are available at the root for umbrella-style imports. The same
`Instance` / `Device` wrappers appear at the root and under `core`; their raw
identities are consistently named `InstanceHandle` / `DeviceHandle`.

Free functions and associated methods share one cached-PFN implementation:
`vk::create_instance(&entry, ...)` / `vk::Instance::new(&entry, ...)`,
and `vk::cmd_dispatch(&device, ...)` / `device.cmd_dispatch(...)`.
`vk::load::Entry::new()` opens the OS library and loads entry pointers. Optional
`vk::memory` supplies experimental separate allocations, opt-in pooled buffers
and host-only virtual placement. It does not provide full VMA functionality.

Fields and parameters use snake_case, including `s_type` and `p_next`.
Vulkan type acronyms, vendor tags, associated constants and `PFN_vk*` aliases
remain recognizable. C-compatible representations preserve layout independently
of field spelling. Wrapper constructors use `new` / `new_from_*`; raw value
conversions retain `from_raw`. Native object destruction is explicit; Drop
never destroys Vulkan objects or waits.

Created instances/devices retain owned version/extension provenance through
`capabilities()`. Commands outside that admission policy are removed from cached
dispatch once at construction; ordinary calls still use cached PFNs. Raw-handle
imports report unknown provenance and remain the unsafe caller's responsibility.
Device snapshots own exact enabled feature records, with explicit
`Features::require` checks. Metadata-bearing imports use
`new_from_handle_with_capabilities`. Command-specific feature/argument valid
usage remains the caller's responsibility. If native
creation succeeds but dispatch initialization cannot be rolled back,
`CallError::InitializationFailed` retains the handle/library for explicit recovery.

No Vulkan SDK, C compiler or C++ dependency is needed to build the library.
The default `loader` feature opens the OS Vulkan loader at runtime.
Platform definitions require the matching `platform-*` feature; beta definitions
require `provisional`. These compilation features do not enable device features.

```bash
cargo test --workspace --all-features
cargo run -p vkrust-gen -- --registry registry/vk.xml --video-registry registry/video.xml --output . --check
```

Runtime source lives in `src/rs`, generation in `tool/gen`, tests in `test/rs`.
Tests mirror module ownership through core/load/memory directories. Codec tests
live under core; registry-derived API tests use `test_*.gen.rs` names.
Dev/test builds disable incremental compilation and debug information.
Generated source files use singular `*.gen.rs` names under `core/`.
The [qualification and limitations](docs/external/qualification.md) describe current scope;
[registry provenance](registry/SOURCES.md) records the pinned inputs.

See [third-party notices](NOTICE.md) for registry and allocator provenance.

Licensed under [MIT](LICENSE).
