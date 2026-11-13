# Surface Laptop 5 qualification and benchmark — 2026-11-13

This run replaces the earlier Haswell exploratory reports with one current
Vulkan 1.4 hardware record. It finds real vk allocator regressions against VMA,
a split result against Vulkano, and no correctness or validation failure across
32 native profiles. It does not establish a universal speed ranking, Vulkan
conformance or production readiness.

## Environment and method

- Microsoft Surface Laptop 5, Intel Core i7-1265U and Intel Iris Xe Graphics
  (Alder Lake GT2), Arch Linux x86-64, kernel 7.3.4.
- Mesa/vulkan-intel 26.2.7, Vulkan loader 1.4.363, validation layer 1.4.363.1;
  one physical device exposing Vulkan 1.4.354.
- Device UUID `8680a846-0c00-0000-0002-000000000000`; driver UUID
  `0a5ed6ac-2efd-3f29-95ce-dc0968ffd9c8`.
- Host-visible, coherent, cached and device-local memory type 2; queue family 0;
  buffer/image granularity 1 and non-coherent atom size 64.
- Rust 1.98.1/LLVM 22.1.8; Clang/lld 23.1.5; Rust release opt-level 3,
  thin LTO and one codegen unit; native VMA built as C++17 `-O3 -DNDEBUG`.
- Candidate vkrust `0.12.0+1.4.365` at commit `1389161`; Ash `0.38.0+1.3.281`,
  pinned native VMA 3.4.0 and Vulkano 0.35.2.
- AC power. Timing processes were pinned to logical CPU 2, a P-core sibling.
  The existing `intel_pstate` powersave policy and turbo remained enabled;
  CPU/GPU clocks were not locked. Per-process clock and thermal snapshots were
  retained, but attained clocks and desktop activity were not controlled.
- Validation and all explicit/implicit layers were disabled during timing.
  Each backend ran in a fresh process with alternating paired order. Runtime
  diagnostics invalidated a sample; none occurred.

Every case used 30 measured process pairs after five discarded warmups and
4,096 operations per query/allocation process (32 batches of 128). Startup
measures one load/instance/device sequence per process. Each allocator batch
has at most 128 live entries and frees in reverse order. Sizes repeat 64, 256,
4,096, 16,384 and 65,536 bytes. Pooled native storage uses one preallocated
64 MiB block; separate storage uses one native allocation per buffer.

Ratios below are the median paired **vk/reference** aggregate host times. Below
one favors vk. Confidence intervals are paired-bootstrap 95% intervals over
fresh processes, not per-call latency bounds. Approximate per-operation times
divide the median process aggregate by its operation count.

## Ash and VMA confirmation

| Operation | vk median | Reference median | Paired ratio | 95% interval |
| --- | ---: | ---: | ---: | ---: |
| Loader opening | 0.081 ms | Ash 0.082 ms | 1.02 | 0.96–1.05 |
| Instance creation | 109.0 ms | Ash 109.8 ms | 1.55 | 0.33–2.98 |
| Device creation | 1.784 ms | Ash 1.783 ms | 1.03 | 0.98–1.06 |
| Memory-properties query | 14.72 ns | Ash 9.68 ns | 1.52 | 1.43–1.60 |
| Virtual allocate | 23.89 ns | VMA 13.96 ns | 1.72 | 1.66–1.75 |
| Virtual free | 13.83 ns | VMA 4.34 ns | 3.18 | 3.10–3.22 |
| Pooled buffer create/allocate/bind | 289.3 ns | VMA 146.0 ns | 2.04 | 1.93–2.10 |
| Pooled buffer destroy/free | 91.6 ns | VMA 76.2 ns | 1.23 | 1.16–1.27 |
| Separate buffer create/allocate/bind | 646.5 ns | VMA 300.5 ns | 2.10 | 2.04–2.18 |
| Separate buffer destroy/free | 149.0 ns | VMA 138.9 ns | 1.06 | 1.00–1.13 |

Instance creation measured bimodally (~55 ms and ~168 ms clusters on both
backends) under this session's driver power state, so its paired interval is
not meaningful; loader and device creation stay at parity.

Combining allocation and free within each pair gives **2.05×** for virtual
placement (1.99–2.11), **1.74×** for pooled native buffers (1.68–1.81), and
**1.79×** for separate native buffers (1.71–1.87), all slower than VMA on this
workload. Separate destroy/free is near parity; the regression concentrates in
create/allocate/bind, where the allocator performs additional checked
bookkeeping.

The measured property query is a void command returning a 520-byte record. vk
zero-initializes it and retains a recoverable nullable-command check. Ash checks
for null while loading its function table and installs a panic stub when the
pointer is absent; its public query therefore uses `MaybeUninit` and performs no
per-call null check. Inspection of these optimized binaries confirms that both
loops make the driver call and copy the observable 520-byte result, while only
vk's loop emits the 520-byte `memset` and nullable-PFN branch. The measured
difference belongs to those public wrapper contracts and must not be generalized
to command recording, GPU throughput or unrelated queries.

The allocator ratios are not explained by that binding query path. The virtual
case never calls Vulkan and already shows a host-placement gap. On native
creation, vk additionally preserves recoverable generation-checked allocation/
pool identities, preflights inverse PFNs and policy/capability constraints,
reserves fallible callback-owned bookkeeping before effects, checks null or
duplicate native handles, records resource associations and publishes detailed
accounting. VMA also validates and accounts internally, so this source-contract
inventory is not a causal decomposition of the 1.74–2.05× combined result.
Separate free near parity localizes that workload's regression to
create/allocate/bind; pooled and virtual placement remain independently slow.
Native-call counts, instruction profiles and check-disabled variants were not
measured, and should precede any claim that recoverability alone explains the
gap.

## Vulkano managed-API confirmation

Vulkano uses checked public APIs, `Arc` ownership, the standard FreeList
allocator and ordinary `Drop` cleanup. Its host-visible memory is automatically
mapped. vk remains caller-managed and its Vulkan calls are unsafe. This is a
public resource-API comparison, not equal ownership/safety policy or an isolated
placement-algorithm comparison.

| Workload | Paired ratio | 95% interval |
| --- | ---: | ---: |
| Combined loader + instance + device | 1.55 | 0.34–2.90 |
| Pooled create/allocate/bind | 1.73 | 1.66–1.85 |
| Pooled destroy/free | 1.12 | 1.08–1.14 |
| Pooled allocation + free combined | **1.53** | 1.46–1.59 |
| Separate create/allocate/bind | 0.264 | 0.25–0.27 |
| Separate destroy/free | 0.182 | 0.18–0.19 |
| Separate allocation + free combined | **0.245** | 0.23–0.25 |

vk is slower for the pooled workload despite Vulkano's additional ownership
and validation. Separate storage favors vk by approximately 4.08× combined,
while Vulkano also performs automatic mapping/unmapping. Individual startup
phase ratios are not useful: Vulkano performs more eager work while loading and
less while creating the instance, and this session's bimodal instance times
make the combined startup interval meaningless.

## Correctness and graphics qualification

Separate Khronos core and synchronization-validation runs each passed all 32
named hardware profiles on `surface-build-r8/` with no skips, warnings, errors,
VUIDs or synchronization hazards. Coverage includes:

- Vulkan 1.0–1.2 instance/device/feature operations, timeline semaphores and
  buffer device address;
- compute/fence/readback, descriptor templates, separate/default/custom/
  automatic/linear buffers and five image/mixed-pool profiles;
- defragmentation end to end: fragmented pooled-buffer and optimal-image
  replacement binds, GPU copies under exact layout transitions, fence
  completion, relocation commits, mapped payload verification, empty-block
  trimming, persistent-context Extensive multi-pass moves with ignored-source
  exclusion, intra-block Balanced/Full compaction, alias-group recreation and
  post-commit descriptor rebind;
- serialized and parallel host mapping/binding profiles;
- OPAQUE_FD buffer and image import plus DMA-BUF buffer import.

The first core run exposed a fixture assumption: the 7×9 optimal image reports
4,112 bytes with 65,536-byte alignment on Iris Xe, so a hard-coded 64 KiB
single-block mixed-resource pool left no room for its readback buffer. The test
now probes both resources, intersects their memory-type bits and derives a
checked 128 KiB block from native extents/alignment/granularity. The focused
profile and both complete validation modes pass after that correction.

The offscreen graphics oracle also passed with synchronization validation:
Vulkan 1.3 dynamic rendering and synchronization2 drew three shader-generated
vertices, submitted with `queue_submit2`, read back a 64×64 RGBA8 image, checked
the center/corner and exactly 1,152 red pixels, then completed explicit cleanup.
This is not WSI or presentation evidence.

## Retained evidence

Ignored raw evidence under `target/sdk/`:

- `surface-build-r10/`: source-only manifest/build from commit `1389161`
  (clean tree) that produced the timed Ash/VMA binaries.
- `surface-index-confirm-20261113-{startup,query,virtual,buffer-pool,buffer-separate}/`
  and `surface-index-confirm-20261113-startup-r2/`: the paired confirmation
  runs behind the Ash/VMA table.
- `surface-build-vulkano-r10/` and
  `surface-index-vulkano-20261113-{startup,buffer-pool,buffer-separate}/`:
  the managed-API comparison build and paired runs.
- `surface-core-20261113-r8/` and `surface-sync-20261113-r6/`: complete
  32-profile validation runs including intra-block, alias and descriptor
  relocations.
- `surface-build-r8/`: source-only manifest/build used by the 32-profile runs.
- `surface-triangle/` and `surface-triangle-run-20261113/`: shaders, executable,
  validation output and pixel oracle.
- `surface-core-20261113/` and `surface-core-20261113-r2/`: retained invocation
  and pre-fix failures; neither is counted as qualification evidence.

Timing-build source-inventory SHA-256 (compact sorted JSON path/hash map):
`45834efe873484e63735dbfda3f2cedbb93faee10e66abf8b21603c3d1a85ab6`, recorded
by `surface-build-r10/` against commit
`1389161eac0917b2c45be174a08acd93d20a7dae` with a clean tree. The manifest
excludes ignored `sdk/target` artifacts; a Python evidence test guards that
boundary.

| Artifact | SHA-256 |
| --- | --- |
| vk-bench | `5875981ec89e5e3ade5f20a789007e290c46e4f582ac6a0287bcb3dffb946d7d` |
| ash-bench | `615fb4641c130a473828dd64af9ad5be359ad13e88165d38e7c8152285a87f45` |
| vma-bench | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |
| vulkano-bench | `f6346377e38aef0f618afe83aa39d97973dec8946eca307a51cfaa5a80ef5469` |

These measurements cover bounded unfragmented traces. Fragmented/image/map/
cache/relocation/contention benchmarks, fixed-clock runs, RSS/native-call
instrumentation, additional hardware/vendors, WSI and presentation remain open.
No GPU rendering or compute throughput comparison was timed.
