# Surface Laptop 5 benchmark inventory — 2026-11-16

This file records the current allocator timing runs on the baseline device.
Two passes are retained. The first splits each host registry into a compact
key array and a parallel value array; the second merges duplicate-handle
checking with association recording, resolves the requirements route once per
create, and looks each pool up once during option validation. In the latest
complete matrix, **21 of 44 combined workloads complete below 1.0× VMA** —
every virtual-placement row, single-buffer pooled lifetimes, and the
pooled-buffer reverse/FIFO rows. The worst measured gap narrows to 1.104×
(separate-buffer churn). The below-0.9× target across the suite is not met.

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

Same pinned machine as the November 14/15 reports: Surface Laptop 5,
i7-1265U, Iris Xe (Alder Lake GT2), Arch x86-64 kernel 7.3.4-arch1-1,
Mesa 26.2.7, loader 1.4.363, validation layer 1.4.363.1, device API 1.4.354,
memory type 2, queue family 0, CPU pinned to logical processor 2, Rust
release opt-level 3 / thin LTO / one codegen unit, VMA 3.4.0 C++17 -O3.

Every ratio is a paired fresh-process median vk/VMA with a bootstrap 95%
interval; intervals are descriptive and unadjusted for multiple comparisons.
Ratios compare the same workload, not a claim about other devices, drivers or
the whole public feature set. The candidate tree is uncommitted work on top
of `v0.13.2` (`95e763d`); timing numbers qualify this tree, not a release.

## Latest allocator pass: candidate merges 1–3 / VMA — November 16

| Cell | combined | 95% CI | allocate vk / VMA ns | free vk / VMA ns |
|---|---|---|---|---|
| virtual-tlsf-reverse | **0.823** | 0.775–0.873 | 9.8 / 13.3 | 4.7 / 4.2 |
| virtual-tlsf-fifo | **0.824** | 0.785–0.867 | 9.7 / 13.5 | 8.6 / 9.0 |
| virtual-tlsf-shuffle | **0.843** | 0.800–0.946 | 9.5 / 12.9 | 9.3 / 8.8 |
| virtual-tlsf-churn | **0.834** | 0.803–0.885 | 17.7 / 24.0 | 11.0 / 11.0 |
| virtual-linear-reverse | **0.740** | 0.705–0.749 | 9.1 / 14.5 | 7.2 / 7.1 |
| virtual-linear-fifo | **0.801** | 0.781–0.815 | 9.3 / 15.2 | 9.0 / 7.8 |
| virtual-linear-shuffle | **0.832** | 0.781–0.888 | 8.9 / 14.6 | 15.2 / 13.9 |
| virtual-linear-churn | **0.768** | 0.744–0.787 | 16.3 / 25.5 | 18.4 / 18.9 |
| buffer-pool-tlsf-reverse | **0.959** | 0.943–0.987 | 169.9 / 160.5 | 68.9 / 87.3 |
| buffer-pool-tlsf-fifo | **0.964** | 0.944–0.980 | 162.9 / 151.0 | 68.7 / 85.6 |
| buffer-pool-tlsf-shuffle | 1.095 | 1.066–1.119 | 148.6 / 134.0 | 83.9 / 77.4 |
| buffer-pool-tlsf-churn | 1.074 | 1.063–1.113 | 228.8 / 198.3 | 107.4 / 112.7 |
| buffer-pool-linear-reverse | **0.957** | 0.945–0.975 | 141.4 / 134.9 | 60.7 / 76.4 |
| buffer-pool-linear-fifo | **0.961** | 0.936–0.987 | 143.4 / 135.3 | 61.7 / 77.9 |
| buffer-pool-linear-shuffle | 1.058 | 1.043–1.107 | 148.5 / 132.8 | 81.2 / 81.2 |
| buffer-pool-linear-churn | **0.998** | 0.980–1.017 | 212.1 / 199.0 | 103.6 / 117.1 |
| image-pool-tlsf-reverse | 1.079 | 1.064–1.093 | 755.7 / 669.4 | 114.0 / 132.2 |
| image-pool-tlsf-fifo | 1.077 | 1.061–1.084 | 752.5 / 676.6 | 119.6 / 136.6 |
| image-pool-tlsf-shuffle | 1.074 | 1.058–1.082 | 747.5 / 673.2 | 118.2 / 136.4 |
| image-pool-tlsf-churn | 1.088 | 1.079–1.093 | 1125.6 / 997.3 | 177.0 / 199.9 |
| image-pool-linear-reverse | 1.048 | 1.030–1.063 | 696.7 / 640.8 | 109.8 / 132.2 |
| image-pool-linear-fifo | 1.051 | 1.042–1.060 | 695.7 / 639.5 | 114.4 / 133.2 |
| image-pool-linear-shuffle | 1.048 | 1.035–1.058 | 697.2 / 644.9 | 121.0 / 137.7 |
| image-pool-linear-churn | 1.050 | 1.042–1.062 | 1047.1 / 955.4 | 176.1 / 201.9 |
| buffer-separate-tlsf-reverse | 1.049 | 1.028–1.058 | 308.9 / 262.7 | 98.0 / 126.4 |
| buffer-separate-tlsf-churn | 1.104 | 1.080–1.119 | 440.4 / 360.3 | 169.2 / 190.8 |
| image-separate-tlsf-reverse | 1.038 | 1.029–1.060 | 896.4 / 830.5 | 156.3 / 182.6 |
| image-separate-tlsf-churn | 1.062 | 1.046–1.081 | 1319.8 / 1201.2 | 248.4 / 277.6 |
| buffer-pool-tlsf-reverse (map cycle) | 1.019 | 1.006–1.046 | 165.4 / 152.1 | 1501.9 / 1487.5 |
| buffer-pool-tlsf-churn (map cycle) | 1.061 | 1.055–1.078 | 259.3 / 223.5 | 1618.7 / 1535.0 |
| buffer-pool-linear-reverse (map cycle) | 1.019 | 1.000–1.026 | 198.0 / 181.2 | 1780.6 / 1765.5 |
| buffer-pool-linear-churn (map cycle) | 1.032 | 1.025–1.039 | 243.8 / 224.6 | 1587.4 / 1544.5 |
| virtual-tlsf-churn-min-memory | **0.867** | 0.854–0.882 | 17.3 / 22.0 | 9.9 / 8.9 |
| virtual-tlsf-churn-min-time | **0.850** | 0.826–0.886 | 18.9 / 23.8 | 11.7 / 11.1 |
| virtual-tlsf-churn-min-offset | **0.625** | 0.619–0.629 | 62.5 / 107.3 | 11.1 / 10.3 |
| virtual-tlsf-reverse, 1 live | **0.848** | 0.830–0.867 | 0.2 / 0.3 | 0.2 / 0.2 |
| virtual-tlsf-churn, 32 live | **0.746** | 0.714–0.802 | 4.7 / 7.1 | 3.1 / 3.1 |
| virtual-tlsf-churn, 256 live | **0.848** | 0.821–0.860 | 36.2 / 47.7 | 22.4 / 21.2 |
| buffer-pool-tlsf-churn-min-memory | 1.049 | 1.038–1.064 | 223.4 / 199.5 | 103.9 / 112.2 |
| buffer-pool-tlsf-churn-min-time | 1.087 | 1.070–1.135 | 236.0 / 200.3 | 111.1 / 113.4 |
| buffer-pool-tlsf-churn-min-offset | **0.961** | 0.937–0.983 | 278.7 / 288.8 | 106.5 / 115.1 |
| buffer-pool-tlsf-reverse, 1 live | **0.928** | 0.885–0.941 | 1.2 / 1.3 | 0.5 / 0.6 |
| buffer-pool-tlsf-churn, 32 live | 1.055 | 1.039–1.096 | 53.6 / 48.6 | 25.6 / 26.8 |
| buffer-pool-tlsf-churn, 256 live | 1.065 | 1.056–1.078 | 458.2 / 401.9 | 218.4 / 229.9 |

Movement against the earlier same-day map-only matrix: pooled TLSF FIFO
crosses below 1.0 (0.964), reverse deepens to 0.959, shuffle 1.112→1.095,
churn 1.100→1.074; pooled linear reverse/FIFO/churn drop to 0.957/0.961/0.998;
separate-buffer reverse 1.107→1.049 and churn 1.152→1.104; separate-image
reverse 1.065→1.038 and churn 1.119→1.062; map cycles 1.062–1.068→1.019–1.061.
Virtual rows move within noise (min-offset churn 0.625).

The earlier map-only reading of a 1–4% separate-cell regression against the
previous day's v0.13.2 matrix is not reproduced here: the combined tree
measures every separate cell below its November 15 value on the same-day
paired matrix, consistent with the quick 0.99–1.00× same-day spot checks —
cross-day matrix comparisons carry that much drift.

## Latest mapping cycles

| Row | combined | map vk / VMA ns |
|---|---|---|
| tlsf reverse | 1.019 | 1502 / 1488 |
| tlsf churn | 1.061 | 1619 / 1535 |
| linear reverse | 1.019 | 1781 / 1766 |
| linear churn | 1.032 | 1587 / 1545 |

## Changes and rejected experiments

**Kept — merged duplicate check and record.** After `vkCreateBuffer` /
`vkCreateImage`, one registry probe (`insert_absent_reserved`) now both
rejects an already-live handle and records the new owner under the serial the
publication reserve already fixed; every failure path releases the entry
again. Imports and partitioned allocators keep the deferred path. The
requirements route preflighted before creation is passed through instead of
being recomputed for the memory-requirements query. One host-reservation
failure path initially retried and continued; the per-site allocation-failure
injection test caught it and the error now propagates. The helper is forced
inline — an out-of-line build measured images 2–3% slower.

**Kept — single pool lookup in option validation.** `validate` admits the
pool once; the forced-separate, upper-address and priority checks share the
returned state. Error order is unchanged; pool priority still flows through
`BlockOptions` on the pool-owned path.

**Kept — split key/value registry (`map-soa`).** `HostMap` stored
`Option<Entry<V>>` buckets, so every probe loaded an entry bucket — up to
~216 bytes for `Live` — to compare one key. VTune attributed ~110 cycles/op
across registry hashing/probing on `buffer-separate-churn`. The registry now
keeps `keys: Option<u64>` (16-byte probe slots) parallel to `values`.
Backward-shift deletion, the checked last-slot hint, 75% load reserve and
panic-atomic lazy publication are unchanged; occupancy rides the key option,
so every u64 key remains valid. The width-based index now uses
`leading_zeros` instead of `count_ones` (baseline x86-64 has BSR but no
POPCNT). Paired measurement: pooled cells 0.947–0.956 combined in the
map-only matrix; combined-tree numbers above.

**Rejected — publication-field threading.** Extending `Region`/`BlockOptions`/
`StorageOptions` to fill `resource`/`can_alias`/`imported`/`export_types`
inside `publish` removed two post-publish lookups but regressed pooled
allocation ~4% (wider struct initialization and movement exceeded the saved
probe). Reverted.

**Rejected — fingerprint-tag SoA.** A three-array layout (tag bytes + keys +
values, Swiss-style 7-bit fingerprints) added a third dependent load on the
dominant ~1-probe path and measured ~2–3% slower on native cells. Reverted.

**Rejected — destroy-token single validation.** The second `free` validation
is a cached-slot lookup and the guard check exits immediately; nothing
measurable to remove.

## Correctness and stability status

All 44 matrix cells pass on the candidate build under separate Khronos core
and synchronization validation (`merge123-core`, `merge123-sync`). The
2,777-test library suite and 54 native `mem` tests pass on the recorded
device. Strict library Clippy, `cargo fmt` and `git diff --check` pass.
Whole-tree Clippy remains blocked by the unrelated unfinished
`test_pipe_bench.rs` (kept untracked). These results qualify the named
workloads on this machine only — not Vulkan conformance, other
devices/drivers, WSI, or the full feature set.

## Remaining coverage

Native rows now sit 0.93–1.10; separate-buffer and image allocation still
carry the largest residual (combined 1.038–1.104; allocate phase up to ~1.22
on separate churn). The remaining gap is distributed create/publication
bookkeeping plus the duplicate-tracking cost itself — the untried lever is
not recording allocator-created handles at all (~15% on pooled churn in a
spot check) at the price of losing duplicate-handle detection for raw binds.
The publication-field-threading retry, destruction preflight and TLSF
placement search/split remain candidate investigations.

## Reproduce the current suite

```sh
python3 sdk/tool/memory.py run --suite full --build target/sdk/merge123 \
  --api 1.4 --memory-type 2 --cpu 2 --samples 20 --warmup 3 --batches 512 \
  --output target/sdk/merge123-timing
L=/usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json
python3 sdk/tool/memory.py validate --suite full --build target/sdk/merge123 \
  --api 1.4 --memory-type 2 --validation core --layer-manifest $L \
  --output target/sdk/merge123-core
python3 sdk/tool/memory.py validate --suite full --build target/sdk/merge123 \
  --api 1.4 --memory-type 2 --validation sync --layer-manifest $L \
  --output target/sdk/merge123-sync
```

Exploratory paired A/B: `python3 target/sdk/quickab.py <A-vk-bench>
<B-vk-bench> <samples> "<case>:<algorithm>:<trace>:<strategy>:<mapping>,..."`.
Source-line profile: `TOP=40 sh target/sdk/lineprof.sh <case> <trace>`.

## Evidence and exact build identities

| Artifact | SHA-256 / identity |
|---|---|
| latest `vk-bench` (`target/sdk/merge123`) | `8a33dad97bcdaccb1bd639c3b31a044d3e5d969842301469f4fb368bbf7e213f` |
| map-only `vk-bench` (`target/sdk/map-soa`) | `0e1c89bd2f7ba14e1ab9b35ffe8c4c11aa9e17a82c05ce631322fd3ab80cfc70` |
| VMA `vma-bench` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| candidate source | `v0.13.2` (`95e763d`) + `allocator.rs`, `binding.rs`, `host/map.rs`, `pool.rs`, `request.rs`, `selector.rs`, `synchronized/resource.rs`, `test_map.rs`, `sdk/Cargo.lock` |

### Complete retained timing-run inventory

| Directory | Content |
|---|---|
| `target/sdk/merge123-timing` | full 44-cell vk/VMA matrix, 20 samples, this report |
| `target/sdk/merge123-core`, `merge123-sync` | per-cell Khronos core/sync validation, all passed |
| `target/sdk/map-soa-timing` | earlier same-day 44-cell matrix (map-only tree) |
| `target/sdk/map-soa-core`, `map-soa-sync` | map-only validation, all passed |
| `target/sdk/lf-check` | focused `buffer-pool-linear-fifo` rerun (alloc 1.12) |
| `target/sdk/opt-verify-timing` | November 15 verified matrix (v0.13.2 build) |
| `target/sdk/mem-next-matrix` | November 15 release/candidate matrix |
| `/tmp/vdbg-buffer-separate-churn` | VTune source-line profile inputs |
