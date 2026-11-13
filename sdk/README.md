# Vulkan qualification SDK

Development consumers and reference implementations live in this independent
workspace. vkrust consumers do not resolve Ash/Vulkano, compile C++, run a shader compiler
or download VMA. The SDK is excluded from the published crate and the root Cargo
workspace. Its own lockfile pins Rust comparison dependencies; VMA and Khronos
inputs use the existing hash-pinned fetchers.

An optional `vulkano-reference` feature adds the pinned Vulkano 0.35.2 checked
managed-API consumer in `src/rs/vulkano.rs`, alongside `vk.rs` and `ash.rs`.
It is built explicitly so the ordinary Ash/VMA baseline stays lightweight.

**Status:** the Vulkan 1.4
[Surface Laptop 5 baseline](../docs/external/benchmarks/surface-laptop-5-2026-11-13.md)
passes all 32 named profiles in separate core and synchronization-validation
runs, passes the offscreen graphics oracle and records exploratory Ash/VMA/
Vulkano timings. Allocator regressions and measurement limits remain explicit.
Broader qualification remains open. Run correctness gates before timing.

## Prepare the laptop

Use the repository's pinned Rust toolchain, Clang/LLVM/lld, Python 3.11+, a
Vulkan loader, the correct GPU driver, Khronos validation layers and
`glslangValidator`. No GCC is used by the supplied build commands. Start on AC
power, stop competing jobs, record the power/thermal policy, and let the machine
reach a repeatable temperature. Driver-reported API/features are authoritative;
the Iris Xe model name alone does not qualify Vulkan 1.4.

These commands are explicit; the default invocation only prints a plan:

```bash
python3 sdk/tool/bench.py
python3 sdk/tool/qualify.py
python3 sdk/tool/triangle.py
python3 sdk/tool/bench.py build
python3 sdk/tool/triangle.py build
cargo fmt --manifest-path sdk/Cargo.toml --check
cargo clippy --manifest-path sdk/Cargo.toml --all-targets --all-features --locked -- -D warnings
python3 -m unittest discover -s sdk/test/py -v
```

If driver selection is ambiguous, set `VK_DRIVER_FILES` to the intended ICD JSON
before invoking tools. The qualification runner requires that ICD to expose
exactly one physical GPU, because the existing native tests select the first
compatible device. Software/CPU Vulkan is rejected for physical qualification.
Lavapipe remains a separate useful correctness gate.

The SDK binaries land in `target/sdk/build/rust/release/`. Inspect the device,
UUIDs, versions, extensions, memory types and selected features first:

```bash
target/sdk/build/rust/release/vk-bench --case probe --api 1.4
```

Use the toolchain directory from `rustup which cargo` in `PATH` when a system
Cargo would select a different compiler. Record `rustc -Vv` and `clang++ --version`.

## Real hardware qualification

```bash
python3 sdk/tool/qualify.py run --api 1.4 --validation core \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/sl5-core
python3 sdk/tool/qualify.py run --api 1.4 --validation sync \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --require address --require timeline --output target/sdk/sl5-sync
```

The supplied core profiles exercise instance/device creation, version metadata,
feature chains, real compute shaders and fence/readback, separate/pooled/automatic/
linear buffers, optimal images and mixed pools, plus native multithreaded mapping
and binding. Fragmented pooled-buffer, optimal-image, Extensive multi-pass,
intra-block, alias-group and descriptor-rebind profiles each perform real GPU
copies into replacement bindings, wait for completion, commit allocator moves,
validate mapped bytes or readback pixels and release emptied blocks. Each command must run exactly one named
test with its gate enabled.
The runner inspects the installed validation-layer manifest, retains its version,
settings and checksum, and requires loader evidence that the layer was inserted.
Validation warnings/errors invalidate a profile. Unsupported external-import
profiles are recorded as `unsupported`, never `passed`; `--require` makes a
missing optional profile fail the run. Standard host tests remain independent.
Layer activation/settings are evidence of the requested profile; a positive
negative-control suite for every validation subsystem remains a separate gate.

Retain the raw stdout/stderr and `results.json`, including failures. A Vulkan
1.4 feature query is not execution coverage of every Vulkan 1.4 feature. This
runner does not claim video decode, ray tracing, presentation, every extension,
broader relocation algorithms/resources or cross-platform imports have been qualified.
Run the full existing workspace, ABI/donor/package gates too before a release.

## Public-API graphics example

```bash
python3 sdk/tool/triangle.py run \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/sl5-triangle
```

`src/rs/triangle.rs` demonstrates `Instance::new`, `Device::new`, native record
`new().set_*()` construction and `Allocator` methods. It queries and enables
dynamic rendering/synchronization2, draws three shader-generated vertices into a
64×64 RGBA8 image, copies with core 1.3 commands, submits with `queue_submit2`,
waits for an exact fence, invalidates host cache and checks the center, corner,
colors and coverage before writing `triangle.ppm`. All successful-path cleanup
is explicit. Errors terminate this development process; it is not a reusable
resource-owner abstraction.

This deliberately starts with an offscreen pixel oracle. A window/swapchain
tutorial and matched Ash presentation example remain future SDK work. The
[official tutorial](https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/00_Base_code.html)
uses Vulkan-Hpp; vk keeps native Rust records/PFNs and explicit lifetime rules.
No tutorial code or Ash implementation has been copied.

## Honest comparisons

| Workload | Reference | Timed scope |
| --- | --- | --- |
| `startup` | Ash 0.38.0 | Loader opening, instance creation and device creation, separately; cached tables and wrapper metadata are included |
| `query` | Ash 0.38.0 | Repeated real `vkGetPhysicalDeviceMemoryProperties` calls through each public wrapper |
| `virtual` | Pinned VMA 3.4.0 | Host-only TLSF allocation/free, fixed size/alignment trace; no Vulkan driver |
| `buffer-pool` | Pinned VMA 3.4.0 | Native create/allocate/bind/destroy, explicit host-visible type, one preallocated 64 MiB TLSF block |
| `buffer-separate` | Pinned VMA 3.4.0 | Native create/allocate/bind/destroy, one separate native allocation per buffer |

```bash
# Pick a compatible host-visible type from probe; do not assume type zero.
python3 sdk/tool/bench.py run --api 1.4 --memory-type 2 --cpu 2 \
  --samples 30 --warmup 3 --output target/sdk/sl5-bench
```

The example type/CPU indexes are placeholders; choose them from this host's
probe/affinity. Remove qualification-layer environment overrides before timing.
The runner disables implicit/explicit layers, rejects instrumentation/injection
settings and runtime diagnostics, alternates backend order and discards recorded
warmup pairs. Each sample runs in a fresh process. Twenty paired samples are the
default; all samples and failures remain in the evidence directory.

A driver that always emits a known startup warning can be measured only with
an explicit `--allow-driver-warning 'exact warning line'`. The complete stderr
is still retained and the accepted line is recorded in the result; every other
diagnostic fails. Such a run is exploratory and retains that driver's qualification
limits. Host-only virtual placement accepts no driver diagnostics.

In the vk::mem/VMA baseline, both allocators receive the same deterministic size sequence, batch size,
reverse free order, memory type, block extent and allocation/mapping policy.
Token containers are reserved before timing. Correctness inspection, JSON and
statistics are outside the measured intervals. Native token representation and
its copying cost remain implementation-specific and are included. Compiler
barriers retain both implementations' placement operations. No mapping, GPU work,
defragmentation or hidden upload is included in buffer timings. Native counts/bytes and request
checksums must match the declared policies; virtual ranges also get independent
alignment/overlap/extent checks. Cold metadata growth is included identically in
each fresh-process workload, and later batches reuse metadata. Assertions are
disabled in both optimized reference/Rust release builds; failure checks remain.

Results report process-aggregate median/p95 and a paired median ratio with a
bootstrap confidence interval. Ratio **vk/reference < 1** means vk was faster
for that workload. This is not per-call p99 or a universal speed ranking. Keep
raw distributions: native driver creation can dominate binding differences.
Ash and vk retain different table/metadata scope, so startup measures the actual
public APIs, not an isolated instruction-level dispatch cost. Header versions
differ (recorded in the lockfile/pins); all benchmark commands are common core
commands. The Vulkano profile below has different resource/validation/mapping
policies and is explicitly labeled as a managed-API comparison.

Before publishing numbers, add representative fragmented traces, images,
mapping/cache operations, relocation with real copies/data verification,
shared-thread contention, RSS/host allocation and native-call instrumentation,
long steady-state runs, thermal/frequency monitoring and multiple drivers.
The current bounded baseline is useful preparation, not the completed performance
qualification program.

## Vulkano managed-API comparison

```bash
python3 sdk/tool/bench.py plan --reference vulkano
python3 sdk/tool/bench.py build --reference vulkano --build target/sdk/build-vulkano
python3 sdk/tool/bench.py run --reference vulkano --build target/sdk/build-vulkano \
  --api 1.2 --memory-type 1 --cpu 3 --samples 30 --warmup 3 \
  --output target/sdk/vulkano-comparison
```

Select the memory-type/CPU indexes from this host's probe. `--reference auto`
retains the original Ash/VMA cases. `--reference vulkano --case all` runs
startup, pooled buffers and separate buffers; unsupported query/virtual cases
are rejected explicitly. Vulkano caches physical-device properties, so its
cached getter must not be ranked as a fresh native query.

Vulkano uses its checked `Instance::new`, `Device::new` and `Buffer::new` APIs,
`StandardMemoryAllocator` (FreeList placement), `Arc` ownership and normal `Drop`
cleanup. No `_unchecked` functions or raw-PFN replacements are used. The pool
is warmed with one untimed buffer, checked to be exactly 64 MiB, then measured
allocations use `NeverAllocate`. Separate storage uses `AlwaysAllocate`.
Both profiles constrain the selected memory type. Native block/byte counts,
region bounds/non-overlap, live suballocation counts and final cleanup are
checked outside timing. No submitted GPU work or implicit waits are involved.

Vulkano automatically maps host-visible backing. Its mapping/ownership/validation
costs and allocator differ from vk::mem's baseline; separate-buffer timing
therefore includes Vulkano's map/unmap policy. This is a comparison of the two
public resource APIs, not an isolated placement or equal-safety claim. Every
Vulkano record/result carries an explicit managed-API scope and allocator policy.
Vulkano physical-device enumeration eagerly queries/caches more metadata; that
enumeration remains outside the startup timers, as it does for vk and Ash.

## Reproducible clocks

```bash
python3 sdk/tool/clocks.py snapshot --card card0
# Inspect reported base clocks first; choose explicit units/values for this host.
sudo python3 sdk/tool/clocks.py apply --cpu-khz 1800000 --gpu-mhz 400 \
  --card card0 --state /tmp/vk-clocks.json
# Run qualification/benchmarks as the ordinary user, then restore even on failure.
sudo python3 sdk/tool/clocks.py restore --state /tmp/vk-clocks.json
```

Only `apply`/`restore` write controls. Original state is saved before mutation,
exclusive creation protects recovery files, and partial apply errors attempt
restoration. CPU limits cannot exceed a discovered base clock; turbo is disabled
where supported. GPU locking currently supports i915 sysfs controls. Missing
controls fail explicitly; xe/other drivers need a reviewed adapter. Never assume
requested limits guarantee attained clocks or prevent thermal throttling. Clock
locking is not performed automatically and is Linux-specific.

## Platforms and `std`

See [platform and standard-library scope](../docs/external/platforms.md). Android
and iOS do not require `no_std`. Today's `--no-default-features` removes vk's
loader, not Rust's standard library. The optional SDK workflow prepares host
comparisons and target compilation checks; it never runs physical qualification
or publishes a release. No SDK workflow is triggered by a tag.
