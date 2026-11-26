# Surface Laptop 5 benchmark inventory — 2026-11-26

This file records a fresh full allocator matrix on the baseline device. The
measured tree is committed work on `dev` at `05bb378` (`v0.13.4` plus one
allocator checkpoint: the publication path is force-inlined so the new live
record is no longer staged through a misaligned stack copy, the repeated
`allocation_exports` check inside `allocate_native` is removed, and managed
paths reserve publication once). In this matrix, **23 of 44 combined
workloads complete below 1.0× VMA** — the same count as November 22. Twenty
of those 23 have a 95% interval wholly below 1.0 (21 on November 22), and 13
cells are below 0.9× (14 on November 22). 23 of the 30 native-resource cells
improved; their geometric-mean ratio moved from 1.022 to 1.007. The worst
measured gap is 1.091× (pooled-buffer linear shuffle, interval
0.980–1.172); the previous worst cell, separate-buffer churn, moved from
1.100× to 1.033×. The below-0.9× target across the suite is not met.

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

Same pinned machine as the November 13–22 reports: Surface Laptop 5,
i7-1265U, Iris Xe (Alder Lake GT2), Arch x86-64 kernel 7.3.5-arch1-1-surface,
Mesa 26.3.1, loader 1.4.363, validation layer 1.4.363.1, device API 1.4.363,
memory type 2, queue family 0, CPU pinned to logical processor 2, Rust
release opt-level 3 / thin LTO / one codegen unit, VMA 3.4.0 C++17 -O3. The
VMA reference binary is byte-identical to November 22.

Every ratio is a paired fresh-process median vk/VMA with a bootstrap 95%
interval; intervals are descriptive and unadjusted for multiple comparisons.
Ratios compare the same workload, not a claim about other devices, drivers or
the whole public feature set. The measured tree is `dev` at `05bb378`, clean
at build time; timing numbers qualify this tree, not a release.

This pass ran under heavier workstation load than November 22: the desktop
search extractor (`localsearch-extractor-3`) was crash-looping at roughly 270
core dumps per minute and the one-minute load average ranged from 4 to 16.
Pairing and interleaving cancel common drift, but per-cell intervals are
wider and single cells (notably pooled-buffer linear shuffle, which moved from
1.022 to 1.091 with an interval reaching 0.980) carry more noise than the
November 22 matrix. Cross-day absolute medians are not comparable.

## Latest allocator pass: `dev` @ `05bb378` / VMA — November 26

| Cell | combined | 95% CI | Nov 22 | allocate vk / VMA ns | free vk / VMA ns |
|---|---|---|---|---|---|
| virtual-tlsf-reverse | **0.810** | 0.769–0.839 | 0.833 | 13.5 / 18.3 | 7.1 / 7.0 |
| virtual-tlsf-fifo | **0.818** | 0.764–0.848 | 0.809 | 13.3 / 17.0 | 11.3 / 12.6 |
| virtual-tlsf-shuffle | **0.849** | 0.821–0.879 | 0.853 | 13.5 / 17.5 | 11.9 / 12.5 |
| virtual-tlsf-churn | **0.860** | 0.831–0.941 | 0.842 | 16.2 / 20.7 | 9.5 / 9.2 |
| virtual-linear-reverse | **0.697** | 0.607–0.711 | 0.714 | 12.2 / 20.9 | 9.9 / 10.1 |
| virtual-linear-fifo | **0.792** | 0.781–0.927 | 0.814 | 12.9 / 21.5 | 13.0 / 10.0 |
| virtual-linear-shuffle | **0.832** | 0.786–0.858 | 0.847 | 13.1 / 21.0 | 20.6 / 19.7 |
| virtual-linear-churn | **0.760** | 0.739–0.806 | 0.763 | 12.7 / 20.6 | 13.4 / 14.4 |
| buffer-pool-tlsf-reverse | **0.924** | 0.903–0.936 | 0.957 | 181.8 / 183.0 | 79.2 / 96.3 |
| buffer-pool-tlsf-fifo | **0.923** | 0.919–0.944 | 0.961 | 176.6 / 179.7 | 81.2 / 99.9 |
| buffer-pool-tlsf-shuffle | 1.042 | 1.013–1.063 | 1.058 | 181.0 / 177.6 | 107.0 / 100.1 |
| buffer-pool-tlsf-churn | 1.024 | 1.002–1.046 | 1.091 | 189.8 / 180.0 | 95.0 / 98.5 |
| buffer-pool-linear-reverse | **0.925** | 0.904–1.047 | 0.951 | 205.2 / 206.4 | 93.5 / 112.8 |
| buffer-pool-linear-fifo | **0.931** | 0.918–0.942 | 0.965 | 178.2 / 181.0 | 83.8 / 99.9 |
| buffer-pool-linear-shuffle | 1.091 | 0.980–1.172 | 1.022 | 250.7 / 229.6 | 143.5 / 137.1 |
| buffer-pool-linear-churn | **0.950** | 0.930–0.964 | 0.981 | 177.8 / 179.6 | 92.5 / 102.1 |
| image-pool-tlsf-reverse | 1.045 | 1.018–1.058 | 1.055 | 1380.1 / 1290.3 | 150.3 / 176.2 |
| image-pool-tlsf-fifo | 1.044 | 0.995–1.069 | 1.061 | 1354.9 / 1267.0 | 153.0 / 177.6 |
| image-pool-tlsf-shuffle | 1.043 | 1.037–1.056 | 1.029 | 1160.7 / 1077.6 | 130.8 / 152.1 |
| image-pool-tlsf-churn | 1.049 | 1.041–1.071 | 1.058 | 1227.4 / 1138.7 | 133.1 / 154.7 |
| image-pool-linear-reverse | 1.038 | 1.010–1.080 | 1.058 | 1132.2 / 1046.5 | 125.7 / 154.1 |
| image-pool-linear-fifo | 1.049 | 1.028–1.068 | 1.040 | 1123.5 / 1035.7 | 128.2 / 151.5 |
| image-pool-linear-shuffle | 1.038 | 1.026–1.063 | 1.048 | 1145.3 / 1049.0 | 137.2 / 157.2 |
| image-pool-linear-churn | 1.043 | 1.022–1.071 | 1.074 | 1202.6 / 1112.8 | 138.6 / 160.7 |
| buffer-separate-tlsf-reverse | **0.966** | 0.948–1.026 | 0.975 | 339.5 / 306.3 | 112.2 / 145.1 |
| buffer-separate-tlsf-churn | 1.033 | 1.018–1.064 | 1.100 | 327.6 / 293.1 | 129.2 / 149.5 |
| image-separate-tlsf-reverse | 1.037 | 1.018–1.060 | 1.036 | 1421.9 / 1367.3 | 178.5 / 214.3 |
| image-separate-tlsf-churn | 1.055 | 1.045–1.062 | 1.033 | 1432.7 / 1326.2 | 188.9 / 213.6 |
| buffer-pool-tlsf-reverse (map cycle) | 1.011 | 1.002–1.036 | 1.024 | 183.5 / 180.1 | 1776.6 / 1753.2 |
| buffer-pool-tlsf-churn (map cycle) | 1.031 | 1.013–1.072 | 1.037 | 195.0 / 178.3 | 1249.4 / 1199.3 |
| buffer-pool-linear-reverse (map cycle) | **0.985** | 0.968–1.000 | 0.998 | 177.9 / 180.3 | 1713.7 / 1736.4 |
| buffer-pool-linear-churn (map cycle) | 1.025 | 1.008–1.045 | 1.015 | 180.9 / 173.4 | 1216.4 / 1193.7 |
| virtual-tlsf-churn-min-memory | **0.909** | 0.885–0.937 | 0.894 | 12.9 / 16.3 | 7.9 / 6.7 |
| virtual-tlsf-churn-min-time | **0.839** | 0.819–0.864 | 0.840 | 14.0 / 18.3 | 8.7 / 8.7 |
| virtual-tlsf-churn-min-offset | **0.624** | 0.619–0.630 | 0.629 | 45.7 / 78.8 | 8.3 / 7.4 |
| virtual-tlsf-reverse, 1 live | **0.844** | 0.828–0.848 | 0.849 | 25.0 / 32.3 | 19.9 / 20.9 |
| virtual-tlsf-churn, 32 live | **0.810** | 0.755–0.875 | 0.757 | 19.7 / 29.0 | 11.6 / 13.6 |
| virtual-tlsf-churn, 256 live | **0.848** | 0.836–0.868 | 0.850 | 13.7 / 18.4 | 8.5 / 8.1 |
| buffer-pool-tlsf-churn-min-memory | 1.017 | 0.987–1.036 | 1.044 | 159.6 / 152.2 | 78.1 / 82.6 |
| buffer-pool-tlsf-churn-min-time | 1.006 | 0.983–1.030 | 1.045 | 164.5 / 156.8 | 81.5 / 86.2 |
| buffer-pool-tlsf-churn-min-offset | **0.929** | 0.914–0.961 | 0.952 | 200.1 / 218.9 | 81.3 / 84.8 |
| buffer-pool-tlsf-reverse, 1 live | **0.911** | 0.902–0.928 | 0.904 | 171.0 / 182.0 | 71.0 / 84.4 |
| buffer-pool-tlsf-churn, 32 live | 1.057 | 1.042–1.087 | 1.080 | 159.9 / 149.5 | 82.3 / 79.1 |
| buffer-pool-tlsf-churn, 256 live | 1.030 | 1.001–1.044 | 1.041 | 164.0 / 155.0 | 82.4 / 85.9 |

## Latest mapping cycles

Map/write/unmap per cycle is driver-dominated and near-parity on both
backends.

| Row | combined | map vk / VMA ns |
|---|---|---|
| tlsf-reverse | 1.011 | 7702 / 7650 |
| tlsf-churn | 1.031 | 7813 / 7610 |
| linear-reverse | 0.985 | 7504 / 7525 |
| linear-churn | 1.025 | 7690 / 7607 |

## Changes and rejected experiments

**Kept — inlined publication.** `publish` and its registry insertion were
separate calls that received a caller-built `Region` by reference and
assembled the 248-byte live record on the stack before copying it into the
registry slot with overlapping 16-byte moves. Those loads read bytes written
by narrower stores and failed store-to-load forwarding. VTune
`LD_BLOCKS.STORE_FORWARD` counts in vk code fell from 16.8 to 12.8 per pooled
operation (VMA's own code: 1.9). Semantics and error order are unchanged.

**Kept — one export admission per native allocation.** Every
`allocate_native` caller (separate storage, pooled growth, custom-pool
minimum blocks) already validates the exact memory type and allocation chain
with `allocation_exports`; the second call inside `allocate_native` is
removed.

**Kept — one publication reservation.** Managed creation reserves
publication before native effects; `allocate_storage` no longer repeats it.
The public raw `allocate` now reserves before entering the shared storage
path. A partitioned allocator therefore consumes one shared serial per
publication instead of two.

**Rejected — slab-backed registry values.** Moving registry values into a
LIFO-reused slab, so probe-slot shifts and rehashes move only keys and
indices, was 5–8% slower: the extra dependent load on every lookup cost more
than the avoided value moves.

**Rejected — smaller live record.** An ablation adding 128 bytes to the live
record cost only 0–2%, so narrowing cold fields would return under 1%.

**Rejected — 64-byte TLSF nodes, in-place registry construction, pool-state
forwarding and scalar image-extent reads.** None moved paired spot checks
outside noise. VMA performs the same TLSF search (10.66 block checks per
image placement versus 10.65 in vk); the image-path stall attributed to
`validate_image` is an allocator-state cache miss after the driver call, not a
store-forwarding failure.

## Correctness and stability status

All 44 matrix cells pass on this build under separate Khronos core and
synchronization validation (`nov26-core`, `nov26-sync`). The 3,463-test
workspace suite (`VULKAN_AVAILABLE=1 cargo test --workspace --all-features`)
passes on the recorded device with the committed allocator source.
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets
--all-features --locked -- -D warnings` and `git diff --check` pass. These
results qualify the named workloads on this machine only — not Vulkan
conformance, other devices/drivers, WSI, or the full feature set.

## Remaining coverage

Native rows now sit 0.91–1.09. On the profiled pooled-buffer churn cell, vk
code retires about 720 instructions per allocate/free pair against about 470
in VMA's own code; VMA additionally spends roughly 200–260 cycles per pair in
its internal locks. The excess is spread across about 60 functions at 2–6
instructions each: seven to nine live-record lookups per pair, four pool
lookups, the duplicate-handle association registry, validation layers and
token construction. Image rows carry an additional allocator-state cache-miss
cost after each driver create. A fused fast path for unchained ordinary
pooled and separate creation is the remaining candidate; removing checked
contracts (handle recording, token validation) remains rejected.

## Reproduce the current suite

```sh
python3 sdk/tool/bench.py build --build target/sdk/nov26
python3 sdk/tool/memory.py run --suite full --build target/sdk/nov26 \
  --api 1.4 --memory-type 2 --cpu 2 --samples 20 --warmup 3 --batches 512 \
  --output target/sdk/nov26-timing
L=/usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov26 \
  --api 1.4 --memory-type 2 --validation core --layer-manifest $L \
  --output target/sdk/nov26-core
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov26 \
  --api 1.4 --memory-type 2 --validation sync --layer-manifest $L \
  --output target/sdk/nov26-sync
```

Exploratory paired A/B: `python3 target/sdk/quickab.py <A-vk-bench>
<B-vk-bench> <samples> "<case>:<algorithm>:<trace>:<strategy>:<mapping>,..."`.
Store-forwarding attribution: VTune `runsa` with
`LD_BLOCKS.STORE_FORWARD:sa=2000,CPU_CLK_UNHALTED.THREAD:sa=20000`, grouped by
module, function and address.

## Evidence and exact build identities

| Artifact | SHA-256 / identity |
|---|---|
| `vk-bench` (`target/sdk/nov26`) | `f0b0283b0b16f74639dd552460d600adca664b3f0a4e49f0570aca93d0b9d7dd` |
| VMA `vma-bench` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| measured source | `dev` @ `05bb378` (`v0.13.4` + inlined publication, single export admission and reservation), clean tree |

### Complete retained timing-run inventory

| Directory | Content |
|---|---|
| `target/sdk/nov26` | provenance build (build.json, both backends) |
| `target/sdk/nov26-timing` | full 44-cell vk/VMA matrix, 20 samples, this report |
| `target/sdk/nov26-core`, `nov26-sync` | per-cell Khronos core/sync validation, all passed |
| `target/sdk/base`, `e-*` | exploratory vk-bench-only A/B builds (not evidence) |
