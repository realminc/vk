# Surface Laptop 5 benchmark inventory — 2026-11-22

This file records a fresh full allocator matrix on the baseline device. The
measured tree is committed work on `dev` at `42426bb` (`v0.13.3` plus two
allocator micro-optimizations: the publication margin predicate is computed
once per `publish`, and the memory-type selection loop hoists flag masks and
heap lookups out of its admitted-bit iteration). In the latest complete
matrix, **23 of 44 combined workloads complete below 1.0× VMA** — every
virtual-placement row, single-buffer pooled lifetimes, the pooled-buffer
reverse/FIFO rows, and newly the separate-buffer reverse row. The worst
measured gap is 1.100× (separate-buffer churn). The below-0.9× target across
the suite is not met.

## Which comparisons exist

| Workload | Baseline | Fresh pairs | Absolute ns/op | Samples | Interleaved | Notes |
|---|---|---|---|---|---|---|
| Virtual blocks, 4 traces × 2 algorithms | VMA | yes | vk / VMA | 20 | yes | host-only placement |
| Pooled buffers, 4 traces × 2 algorithms | VMA | yes | vk / VMA | 20 | yes | block reuse + native growth |
| Separate buffers, 2 traces | VMA | yes | vk / VMA | 20 | yes | one allocation per buffer |
| Pooled/separate images, 4/2 traces × 2/1 algorithms | VMA | yes | vk / VMA | 20 | yes | dedicated-capable layouts |
| Pooled map/unmap cycles, 4 cells | VMA | yes | vk / VMA | 20 | yes | mapping lease + guard |
| Strategy rows (min-memory/time/offset), live-count rows | VMA | yes | vk / VMA | 20 | yes | 8 cells |
| Earlier Ash/Vulkano passes | historical | retained | ns only | — | — | not rerun this pass |

## Environment and interpretation

Same pinned machine as the November 13–16 reports: Surface Laptop 5,
i7-1265U, Iris Xe (Alder Lake GT2), Arch x86-64 kernel 7.3.5-arch1-1-surface,
Mesa 26.3.1, loader 1.4.363, validation layer 1.4.363.1, device API 1.4.363,
memory type 2, queue family 0, CPU pinned to logical processor 2, Rust
release opt-level 3 / thin LTO / one codegen unit, VMA 3.4.0 C++17 -O3.

Every ratio is a paired fresh-process median vk/VMA with a bootstrap 95%
interval; intervals are descriptive and unadjusted for multiple comparisons.
Ratios compare the same workload, not a claim about other devices, drivers or
the whole public feature set. The measured tree is `dev` at `42426bb`, clean
at build time; timing numbers qualify this tree, not a release. Driver and
kernel versions advanced since November 16 (Mesa 26.2.7→26.3.1, kernel
7.3.4→7.3.5), so cross-day absolute medians are not comparable; the paired
vk/VMA ratio is the stable quantity.

## Latest allocator pass: `dev` @ `42426bb` / VMA — November 22

| Cell | combined | 95% CI | allocate vk / VMA ns | free vk / VMA ns |
|---|---|---|---|---|
| virtual-tlsf-reverse | **0.833** | 0.804–0.861 | 11.9 / 15.9 | 6.0 / 5.3 |
| virtual-tlsf-fifo | **0.809** | 0.785–0.836 | 11.9 / 16.4 | 10.2 / 10.8 |
| virtual-tlsf-shuffle | **0.853** | 0.816–0.884 | 11.6 / 15.5 | 10.0 / 10.0 |
| virtual-tlsf-churn | **0.842** | 0.779–0.921 | 14.7 / 19.6 | 9.7 / 8.9 |
| virtual-linear-reverse | **0.714** | 0.695–0.727 | 11.5 / 18.4 | 8.3 / 9.2 |
| virtual-linear-fifo | **0.814** | 0.784–0.825 | 10.8 / 18.0 | 10.6 / 8.6 |
| virtual-linear-shuffle | **0.847** | 0.804–0.856 | 11.3 / 18.9 | 18.8 / 17.0 |
| virtual-linear-churn | **0.763** | 0.750–0.773 | 11.0 / 17.5 | 11.5 / 11.7 |
| buffer-pool-tlsf-reverse | **0.957** | 0.936–0.971 | 162.6 / 155.2 | 65.7 / 81.7 |
| buffer-pool-tlsf-fifo | **0.961** | 0.938–0.985 | 170.9 / 159.8 | 70.6 / 87.3 |
| buffer-pool-tlsf-shuffle | 1.058 | 1.041–1.093 | 164.2 / 152.8 | 90.0 / 86.0 |
| buffer-pool-tlsf-churn | 1.091 | 1.054–1.125 | 183.1 / 162.3 | 85.9 / 88.4 |
| buffer-pool-linear-reverse | **0.951** | 0.914–0.982 | 208.1 / 207.9 | 88.8 / 113.5 |
| buffer-pool-linear-fifo | **0.965** | 0.921–0.975 | 191.9 / 190.2 | 84.0 / 104.0 |
| buffer-pool-linear-shuffle | 1.022 | 0.976–1.084 | 198.3 / 191.9 | 108.6 / 114.8 |
| buffer-pool-linear-churn | **0.981** | 0.937–1.015 | 184.4 / 183.0 | 91.1 / 102.9 |
| image-pool-tlsf-reverse | 1.055 | 1.045–1.072 | 1192.9 / 1093.0 | 126.8 / 149.5 |
| image-pool-tlsf-fifo | 1.061 | 1.041–1.086 | 1196.3 / 1094.6 | 132.7 / 152.8 |
| image-pool-tlsf-shuffle | 1.029 | 1.019–1.050 | 1188.6 / 1132.8 | 134.2 / 159.3 |
| image-pool-tlsf-churn | 1.058 | 1.047–1.067 | 1261.9 / 1163.0 | 134.7 / 156.3 |
| image-pool-linear-reverse | 1.058 | 1.048–1.068 | 1142.0 / 1045.3 | 122.4 / 150.3 |
| image-pool-linear-fifo | 1.040 | 1.032–1.068 | 1111.4 / 1026.0 | 124.3 / 149.8 |
| image-pool-linear-shuffle | 1.048 | 1.035–1.066 | 1107.3 / 1029.7 | 130.4 / 153.3 |
| image-pool-linear-churn | 1.074 | 1.048–1.118 | 1186.1 / 1093.4 | 132.7 / 155.6 |
| buffer-separate-tlsf-reverse | **0.975** | 0.937–0.985 | 387.8 / 363.0 | 121.1 / 169.9 |
| buffer-separate-tlsf-churn | 1.100 | 1.020–1.135 | 395.5 / 321.2 | 146.1 / 162.8 |
| image-separate-tlsf-reverse | 1.036 | 1.025–1.045 | 1470.5 / 1371.8 | 183.6 / 216.5 |
| image-separate-tlsf-churn | 1.033 | 1.026–1.069 | 1544.7 / 1449.0 | 206.8 / 232.1 |
| buffer-pool-tlsf-reverse (map cycle) | 1.024 | 1.013–1.036 | 206.1 / 189.8 | 1859.3 / 1830.4 |
| buffer-pool-tlsf-churn (map cycle) | 1.037 | 1.030–1.053 | 195.1 / 173.4 | 1214.1 / 1171.1 |
| buffer-pool-linear-reverse (map cycle) | **0.998** | 0.982–1.006 | 186.4 / 174.2 | 1710.0 / 1738.4 |
| buffer-pool-linear-churn (map cycle) | 1.015 | 1.006–1.034 | 184.2 / 171.5 | 1201.5 / 1185.9 |
| virtual-tlsf-churn-min-memory | **0.894** | 0.849–0.917 | 12.6 / 16.6 | 8.0 / 6.8 |
| virtual-tlsf-churn-min-time | **0.840** | 0.832–0.890 | 13.9 / 18.1 | 8.7 / 8.3 |
| virtual-tlsf-churn-min-offset | **0.629** | 0.619–0.637 | 46.0 / 79.0 | 8.2 / 7.5 |
| virtual-tlsf-reverse, 1 live | **0.849** | 0.840–0.865 | 24.9 / 31.9 | 20.1 / 20.9 |
| virtual-tlsf-churn, 32 live | **0.757** | 0.726–0.806 | 13.6 / 20.0 | 8.8 / 9.6 |
| virtual-tlsf-churn, 256 live | **0.850** | 0.832–0.859 | 14.7 / 19.3 | 9.1 / 8.9 |
| buffer-pool-tlsf-churn-min-memory | 1.044 | 1.022–1.073 | 170.2 / 152.9 | 77.9 / 82.7 |
| buffer-pool-tlsf-churn-min-time | 1.045 | 1.017–1.064 | 173.7 / 157.6 | 81.7 / 86.9 |
| buffer-pool-tlsf-churn-min-offset | **0.952** | 0.943–0.991 | 209.9 / 221.8 | 80.4 / 85.5 |
| buffer-pool-tlsf-reverse, 1 live | **0.904** | 0.895–0.923 | 179.4 / 186.3 | 70.2 / 85.7 |
| buffer-pool-tlsf-churn, 32 live | 1.080 | 1.062–1.085 | 162.5 / 144.2 | 76.8 / 77.4 |
| buffer-pool-tlsf-churn, 256 live | 1.041 | 1.021–1.085 | 174.7 / 157.3 | 82.1 / 86.6 |

## Latest mapping cycles

Map/write/unmap per cycle is driver-dominated and near-parity on both
backends; the remaining difference sits in the free phase.

| Row | combined | map vk / VMA ns |
|---|---|---|
| tlsf reverse | 1.024 | 8157 / 8171 |
| tlsf churn | 1.037 | 7504 / 7475 |
| linear reverse | 0.998 | 7509 / 7451 |
| linear churn | 1.015 | 7486 / 7421 |

## Changes and rejected experiments

**Kept — single publication guard predicate.** `publish` previously
evaluated margin eligibility twice per operation — two `memory_flags` probes
and two guard-capability checks for the `guard_size`/`guard_offset` pair. The
predicate now computes once and both values derive from it. Semantics and
error order are unchanged; paired spot checks held pooled churn cells inside
the existing noise band with a positive trend.

**Kept — hoisted selection-loop masks.** `find_memory_type_with_preferences`
previously re-read `request.required/preferred_flags.as_raw()`, the avoided
mask and the per-type heap record inside the admitted-bit loop. The raw masks
and the heap record are now loaded once; required/rejected-flag, tile, AMD
device-coherent, protected and heap-size decisions keep their existing order
and tie behavior.

**Rejected since November 16 — handle-recording removal.** The largest
untried lever remains not recording allocator-created handles (~15% on
pooled churn in a spot check), at the price of losing duplicate-handle
detection for raw binds. That weakens the checked contract and is rejected.

## Correctness and stability status

All 44 matrix cells pass on this build under separate Khronos core and
synchronization validation (`nov22-core`, `nov22-sync`). The 3,463-test
workspace suite, including 54 native `mem` tests, passes on the recorded
device. `cargo fmt` and
`git diff --check` pass. Whole-tree strict Clippy remains blocked by the
unfinished `test_pipe_bench.rs` benchmark target (unused-`Result` warnings).
These results qualify the named workloads on this machine only — not Vulkan
conformance, other devices/drivers, WSI, or the full feature set.

## Remaining coverage

Native rows now sit 0.90–1.10; separate-buffer and image allocation still
carry the largest residual (combined 1.033–1.100; allocate phase up to ~1.23
on separate churn). VTune attribution on `buffer-separate-churn` shows the
gap distributed across registry insertion, requirements queries, memory-type
selection and publication rather than one hotspot — the residual is largely
duplicate-handle detection and association bookkeeping that VMA does not
perform. The `Live` record carries ~25 fields per live entry; narrowing its
cold fields into a lazy side table is the next candidate investigation.

## Reproduce the current suite

```sh
python3 sdk/tool/bench.py build --build target/sdk/nov22
python3 sdk/tool/memory.py run --suite full --build target/sdk/nov22 \
  --api 1.4 --memory-type 2 --cpu 2 --samples 20 --warmup 3 --batches 512 \
  --output target/sdk/nov22-timing
L=/usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov22 \
  --api 1.4 --memory-type 2 --validation core --layer-manifest $L \
  --output target/sdk/nov22-core
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov22 \
  --api 1.4 --memory-type 2 --validation sync --layer-manifest $L \
  --output target/sdk/nov22-sync
```

Exploratory paired A/B: `python3 target/sdk/quickab.py <A-vk-bench>
<B-vk-bench> <samples> "<case>:<algorithm>:<trace>:<strategy>:<mapping>,..."`.
Source-line profile: `TOP=40 sh target/sdk/lineprof.sh <case> <trace>`.

## Evidence and exact build identities

| Artifact | SHA-256 / identity |
|---|---|
| `vk-bench` (`target/sdk/nov22`) | `db1931ea7cca9891d29a24ab29a95b46cfb6a7e58411d3c89e8b9f258f0137e8` |
| VMA `vma-bench` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| measured source | `dev` @ `42426bb` (`v0.13.3` + publish predicate fusion + selector hoists), clean tree |

### Complete retained timing-run inventory

| Directory | Content |
|---|---|
| `target/sdk/nov22` | provenance build (build.json, both backends) |
| `target/sdk/nov22-timing` | full 44-cell vk/VMA matrix, 20 samples, this report |
| `target/sdk/nov22-core`, `nov22-sync` | per-cell Khronos core/sync validation, all passed |
| `target/sdk/opt-dev` | exploratory vk-bench-only build (not evidence) |
