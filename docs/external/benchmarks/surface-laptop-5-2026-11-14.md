# Surface Laptop 5 benchmark inventory — 2026-11-14

This report collects **every measured workload and retained timing run currently
available**, with one-to-one results for the implemented Ash, VMA and Vulkano
comparisons. It includes the latest **44-case committed/candidate/VMA matrix**,
its four mapping measurements, the earlier complete matrix, allocator
optimization experiments and retained binding/managed-API baselines. The
[November 13 report](surface-laptop-5-2026-11-13.md) is unchanged.

The current allocator beats VMA in the measured **minimum-offset virtual churn**
case: **0.752× combined host time (95% interval 0.706–0.769)**. Default native
buffer creation remains slower. Map/write/unmap is about 3% slower on this coherent
memory type. The new pass reduces pooled buffer lifetime time by about 7–13%
and linear virtual stack lifetime time by 23% against the committed allocator.
The below-0.9× target is met in one of 44 combined workloads; it is not met
across the suite. This is not an allocator-wide, cross-vendor or security ranking.

## Which comparisons exist

| Workload | Ash | VMA | Vulkano |
| --- | --- | --- | --- |
| Loader / instance / device startup | Retained November 13 paired baseline | Startup is setup, not separately timed | Retained November 13 managed-API baseline |
| Fresh native memory-properties query | Retained November 13 paired baseline | Not separately timed | Cached getter excluded from native-query ranking |
| Virtual TLSF / linear placement, free order and strategies | No allocator counterpart in this harness | Latest 44-case matrix subset | No virtual-placement consumer |
| Native pooled / separate buffers | No managed allocator comparison | Latest matrix | Retained November 13 managed buffers |
| Native pooled / separate images | No managed allocator comparison | Latest matrix | Not implemented in the comparison harness |
| Explicit map / host write / unmap | Not separately benchmarked | Four current pooled-buffer profiles | Automatic mapping has different ownership/policy; no explicit-cycle comparison |
| GPU compute, graphics and relocation | Correctness profiles only | Donor/correctness comparisons; no GPU timing ranking | No matched GPU timing comparison |

A dash or missing counterpart must not be read as a win. Ash is a binding
reference; VMA is the allocator reference. Vulkano retains checked APIs, Arc
ownership, FreeList placement and automatic mapping. Its buffer results compare
public resource APIs with different ownership/mapping contracts. The current
allocator has **not** been freshly timed against Ash or Vulkano in this round:
their tables below explicitly retain the older paired vk binary and date.

## Environment and interpretation

- Surface Laptop 5, Intel Core i7-1265U / Iris Xe (Alder Lake GT2), Arch Linux
  x86-64, kernel 7.3.4-arch1-1.
- Mesa/vulkan-intel 26.2.7, loader 1.4.363, Khronos layer 1.4.363.1; device API
  1.4.354. Device UUID `8680a846-0c00-0000-0002-000000000000`; driver UUID
  `0a5ed6ac-2efd-3f29-95ce-dc0968ffd9c8`.
- Host-visible/coherent/cached/device-local memory type 2, queue family 0,
  buffer/image granularity 1 and non-coherent atom size 64.
- Rust 1.98.1 / LLVM 22.1.8; Clang/lld 23.1.5. Rust release opt-level 3, thin
  LTO, one codegen unit; native VMA C++17 `-O3 -DNDEBUG`.
- References: Ash 0.38.0+1.3.281, VMA 3.4.0 pinned at
  `3aa921224c154a0d2c43912bc88e1c42ce1f7607`, Vulkano 0.35.2.
- Latest candidate: vkrust 0.13.0+1.4.365 over commit `da68c3b`, with the dirty
  tree/source inventories retained. Historical binding/managed comparisons use
  their own v0.12-era build manifests; they are not relabeled as current builds.
- Processes pinned to logical CPU 2. Other agents were compiling; desktop,
  attained clocks and thermals were uncontrolled. These results remain
  exploratory. No fixed-clock regression baseline or CPU/GPU throughput claim.
- Timing disables explicit/implicit layers and rejects runtime diagnostics.
  Core/synchronization validation uses separate runs; its times are excluded.

**All ratios are vk/reference; below one favors vk.** Confidence intervals are
paired-bootstrap 95% intervals over fresh processes. Absolute values are median
process aggregates divided by the declared operation count, not per-call p95/p99.
The median paired ratio need not equal the ratio of displayed medians. Intervals
do not eliminate shared-workstation scheduling, clock or thermal bias.

There are three distinct harness generations: November 13 baselines; November
14's original minimal reverse-free consumer; and the current image-capable,
runtime-policy matrix. Do not stitch their absolute times into one speedup or
infer current vk/Vulkano ratios from current VMA ratios. The tables recompute
values from the named retained JSON rather than mixing cohorts.

## Latest bottleneck pass: committed / candidate / VMA — November 14

Evidence: `mem-limit-matrix/`, candidate `mem-limit-r5/`. Every cell runs
20 measured fresh-process **triplets** after three warmups, rotating all six
committed/candidate/VMA process orders. The committed executable is the
byte-identical allocator from `da68c3b` retained in `mem-push-qualified-r2/`;
the two vk consumers share the same expanded harness. The VMA reference is
identical in all triplets. CPU 2, 64 batches, type 2 and every workload policy,
native check and operation count match the earlier 44-cell matrix.
No validation, profiling or compilation launched by this task overlaps this
full timing cohort. Other workstation activity and clocks remain uncontrolled.
The paired committed/candidate column isolates this implementation comparison;
do not infer gains from changes between the earlier and latest vk/VMA ratios.

Allocation/free columns are **candidate ns / VMA ns**. Combined excludes the
separately measured mapping cycle; mapping history still affects final free.
The last column is candidate/committed combined time with its paired 95% interval.

| ID | Workload | Algorithm / strategy | Trace / live / mapping | Allocate vk / VMA ns | Free vk / VMA ns | vk/VMA combined (95% interval) | Candidate/committed (95% interval) |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| 00 | virtual | tlsf / default | reverse / 128 / none | 24.05 / 15.56 | 6.45 / 5.35 | 1.451 (1.428–1.468) | 1.004 (0.958–1.037) |
| 01 | virtual | tlsf / default | fifo / 128 / none | 24.35 / 16.89 | 12.36 / 11.09 | 1.325 (1.275–1.359) | 0.996 (0.951–1.034) |
| 02 | virtual | tlsf / default | shuffle / 128 / none | 29.32 / 19.61 | 17.03 / 13.97 | 1.402 (1.358–1.465) | 1.024 (1.004–1.098) |
| 03 | virtual | tlsf / default | churn / 128 / none | 31.97 / 23.65 | 11.98 / 11.07 | 1.272 (1.029–1.298) | 0.980 (0.954–0.998) |
| 04 | virtual | linear / default | reverse / 128 / none | 19.07 / 20.98 | 9.64 / 10.26 | 0.906 (0.884–0.934) | 0.768 (0.756–0.786) |
| 05 | virtual | linear / default | fifo / 128 / none | 17.46 / 19.14 | 10.99 / 9.31 | 1.005 (0.993–1.032) | 0.792 (0.781–0.815) |
| 06 | virtual | linear / default | shuffle / 128 / none | 17.42 / 19.23 | 18.87 / 18.58 | 0.969 (0.928–0.983) | 0.988 (0.982–1.007) |
| 07 | virtual | linear / default | churn / 128 / none | 17.50 / 18.79 | 12.46 / 13.24 | 0.942 (0.925–0.953) | 0.820 (0.808–0.831) |
| 08 | buffer-pool | tlsf / default | reverse / 128 / none | 253.63 / 163.17 | 96.91 / 86.40 | 1.401 (1.374–1.423) | 0.931 (0.889–0.956) |
| 09 | buffer-pool | tlsf / default | fifo / 128 / none | 250.15 / 162.82 | 109.40 / 89.90 | 1.421 (1.396–1.432) | 0.919 (0.904–0.923) |
| 10 | buffer-pool | tlsf / default | shuffle / 128 / none | 266.82 / 167.00 | 135.02 / 95.92 | 1.525 (1.505–1.539) | 0.906 (0.888–0.931) |
| 11 | buffer-pool | tlsf / default | churn / 128 / none | 265.66 / 162.65 | 121.22 / 91.31 | 1.533 (1.504–1.546) | 0.923 (0.892–0.950) |
| 12 | buffer-pool | linear / default | reverse / 128 / none | 242.39 / 162.11 | 97.06 / 90.22 | 1.348 (1.331–1.366) | 0.869 (0.849–0.883) |
| 13 | buffer-pool | linear / default | fifo / 128 / none | 248.42 / 167.37 | 105.69 / 93.73 | 1.348 (1.326–1.389) | 0.873 (0.860–0.888) |
| 14 | buffer-pool | linear / default | shuffle / 128 / none | 247.72 / 159.01 | 118.38 / 96.73 | 1.437 (1.413–1.451) | 0.884 (0.875–0.911) |
| 15 | buffer-pool | linear / default | churn / 128 / none | 248.10 / 161.98 | 114.62 / 93.93 | 1.438 (1.407–1.454) | 0.873 (0.853–0.895) |
| 16 | image-pool | tlsf / default | reverse / 128 / none | 997.48 / 806.62 | 161.55 / 159.00 | 1.219 (1.195–1.237) | 0.953 (0.933–0.964) |
| 17 | image-pool | tlsf / default | fifo / 128 / none | 1171.94 / 933.26 | 202.84 / 190.28 | 1.229 (1.213–1.281) | 0.964 (0.933–0.984) |
| 18 | image-pool | tlsf / default | shuffle / 128 / none | 2094.79 / 1436.55 | 387.63 / 271.58 | 1.226 (1.099–1.611) | 0.996 (0.865–1.258) |
| 19 | image-pool | tlsf / default | churn / 128 / none | 1174.10 / 927.46 | 207.27 / 188.37 | 1.237 (1.218–1.252) | 0.953 (0.935–0.963) |
| 20 | image-pool | linear / default | reverse / 128 / none | 1087.10 / 879.60 | 184.07 / 183.90 | 1.206 (1.198–1.230) | 0.951 (0.938–0.964) |
| 21 | image-pool | linear / default | fifo / 128 / none | 1061.73 / 865.32 | 190.74 / 182.32 | 1.190 (1.151–1.204) | 0.950 (0.929–0.963) |
| 22 | image-pool | linear / default | shuffle / 128 / none | 1098.53 / 886.14 | 211.30 / 192.93 | 1.230 (1.200–1.255) | 0.947 (0.927–0.974) |
| 23 | image-pool | linear / default | churn / 128 / none | 1059.52 / 866.57 | 198.15 / 183.45 | 1.208 (1.191–1.254) | 0.942 (0.929–0.967) |
| 24 | buffer-separate | tlsf / default | reverse / 128 / none | 437.13 / 325.97 | 151.26 / 155.83 | 1.232 (1.212–1.274) | 0.973 (0.955–0.994) |
| 25 | buffer-separate | tlsf / default | churn / 128 / none | 422.02 / 291.64 | 163.04 / 152.14 | 1.314 (1.286–1.345) | 0.976 (0.956–0.995) |
| 26 | image-separate | tlsf / default | reverse / 128 / none | 1201.88 / 1026.32 | 223.99 / 227.38 | 1.128 (1.115–1.149) | 0.991 (0.983–1.009) |
| 27 | image-separate | tlsf / default | churn / 128 / none | 1188.82 / 1003.13 | 239.78 / 232.31 | 1.153 (1.135–1.166) | 0.973 (0.959–0.981) |
| 28 | buffer-pool | tlsf / default | reverse / 128 / cycle | 272.93 / 172.53 | 1788.53 / 1701.68 | 1.086 (1.063–1.115) | 0.955 (0.933–0.982) |
| 29 | buffer-pool | tlsf / default | churn / 128 / cycle | 292.43 / 173.00 | 1266.68 / 1206.08 | 1.137 (1.124–1.155) | 0.960 (0.942–0.971) |
| 30 | buffer-pool | linear / default | reverse / 128 / cycle | 312.55 / 203.31 | 2009.17 / 1940.13 | 1.097 (1.069–1.114) | 0.951 (0.926–1.010) |
| 31 | buffer-pool | linear / default | churn / 128 / cycle | 299.83 / 189.25 | 1381.65 / 1282.65 | 1.136 (1.119–1.172) | 0.961 (0.932–0.969) |
| 32 | virtual | tlsf / min-memory | churn / 128 / none | 27.20 / 18.82 | 9.74 / 7.67 | 1.372 (1.145–1.458) | 0.992 (0.802–1.035) |
| 33 | virtual | tlsf / min-time | churn / 128 / none | 30.45 / 30.47 | 12.37 / 10.84 | 1.037 (0.988–1.184) | 1.006 (0.984–1.050) |
| 34 | virtual | tlsf / min-offset | churn / 128 / none | 74.61 / 107.13 | 11.29 / 9.64 | 0.752 (0.706–0.769) | 1.022 (0.892–1.133) |
| 35 | virtual | tlsf / default | reverse / 1 / none | 64.66 / 55.80 | 29.55 / 29.12 | 1.064 (1.043–1.148) | 1.005 (0.959–1.041) |
| 36 | virtual | tlsf / default | churn / 32 / none | 30.75 / 24.80 | 12.45 / 12.49 | 1.193 (1.047–1.250) | 0.983 (0.864–1.028) |
| 37 | virtual | tlsf / default | churn / 256 / none | 32.85 / 22.18 | 12.26 / 9.81 | 1.308 (1.242–1.468) | 0.982 (0.931–1.022) |
| 38 | buffer-pool | tlsf / min-memory | churn / 128 / none | 299.57 / 181.45 | 130.41 / 98.67 | 1.519 (1.491–1.551) | 0.912 (0.894–0.921) |
| 39 | buffer-pool | tlsf / min-time | churn / 128 / none | 295.96 / 180.85 | 135.61 / 100.63 | 1.527 (1.515–1.558) | 0.935 (0.922–0.956) |
| 40 | buffer-pool | tlsf / min-offset | churn / 128 / none | 350.51 / 264.86 | 134.84 / 103.11 | 1.325 (1.307–1.348) | 0.934 (0.927–0.953) |
| 41 | buffer-pool | tlsf / default | reverse / 1 / none | 427.22 / 418.31 | 137.88 / 121.30 | 1.061 (0.962–1.115) | 0.893 (0.855–0.915) |
| 42 | buffer-pool | tlsf / default | churn / 32 / none | 363.76 / 206.33 | 155.06 / 111.54 | 1.607 (1.494–1.953) | 0.912 (0.871–0.947) |
| 43 | buffer-pool | tlsf / default | churn / 256 / none | 340.32 / 209.92 | 152.13 / 120.18 | 1.490 (1.457–1.611) | 0.920 (0.901–0.953) |

### Latest mapping cycles

| Matrix ID | Algorithm / trace | vk ns/cycle | VMA ns/cycle | vk/VMA (95% interval) | Candidate/committed (95% interval) |
| --- | --- | ---: | ---: | ---: | ---: |
| 28 | tlsf / reverse | 8505.73 | 8191.78 | 1.031 (1.014–1.053) | 1.027 (0.986–1.038) |
| 29 | tlsf / churn | 8584.51 | 8286.31 | 1.029 (1.017–1.044) | 1.020 (1.002–1.035) |
| 30 | linear / reverse | 9633.30 | 9257.18 | 1.035 (1.021–1.053) | 1.015 (0.992–1.048) |
| 31 | linear / churn | 9432.91 | 9082.41 | 1.035 (1.022–1.050) | 1.013 (0.999–1.027) |

### Changes and rejected experiments

The retained pass preserves every public function name/signature and the
ordinary/synchronized canonical binding route:

- Empty create/bind extension chains bypass the general bounded chain walker;
  non-empty chains keep duplicate/cycle/tag/capability checks. Resource options
  are borrowed when unchanged, and internal allocation layers borrow immutable
  chain policy rather than repeatedly copying platform import metadata.
- Binding snapshots only its authoritative live handle, serial, type and byte
  range; it no longer rebuilds a full public token and scans for block size.
  Ordinary execution borrows the device; native leases explicitly retain it
  before dropping the owner mutex. Backing locks, leases, claims and post-success
  publication remain intact.
- Linear lookup first validates the full offset/generation at live edges, then
  retains binary search for interior releases. Tail retirement pops directly;
  donor ring/double-stack cleanup and compaction still run. No token/index ABI
  or generation width changes. Stale-token compaction/reuse tests were added.
- Type selection visits only admitted mask bits in increasing index order,
  preserving preference ties and the 32-type boundary. Single-block vectors
  avoid a useless ordering scan. Free snapshots ownership once across temporary
  guard mapping, and association retirement reuses its already-probed bucket.
  Guards, mapped-release rejection, native-output admission and rollback remain.

The compact split-index registry (`mem-limit-r2/`) was **rejected** after its
30-triplet comparison increased pooled lifetime time by about 7.5% and separate
lifetime time by about 3.1%. The original registry remains. Its added indirection
and buffers did not produce a net win for these workloads. Initial chain-policy
build `mem-limit-r4/` failed a Rust borrow check; corrected `mem-limit-r4b/`
passed. The borrowed-chain experiment's individual timings were inconclusive;
no isolated speedup is attributed to that representation change.

In the full matrix, 31/44 candidate/committed combined intervals end below one;
these are descriptive per-cell intervals, not a multiple-comparison-adjusted
release gate. Linear virtual reverse/FIFO/churn lifetimes fall by 18–23%, with
reverse free alone about 45% faster. Unmapped pooled TLSF buffers improve by
7–9% across free orders; pooled linear buffers improve by 12–13%. Ordinary
TLSF virtual placement remains a bottleneck. Only minimum-offset virtual churn
is below 0.9× VMA throughout its interval; linear reverse is 0.906×, and the
slowest native pooled row remains about 1.61×. Mapping cycles remain about 3%
slower than VMA. Public checks were not weakened to reach a timing target.

Virtual TLSF shuffled retirement shows a **1.024× candidate/committed** combined
regression (1.004–1.098) in the full matrix, despite no TLSF algorithm change.
A longer retained repeat, `mem-limit-shuffle-confirm/`, uses 60 triplets after
six warmups and 1,024 batches: combined **0.992× (0.980–1.010)** and free
**1.003× (0.952–1.033)**. It does not reproduce a stable regression or establish
a speedup. The original row stays visible. Code layout, clocks and shared load
remain possible contributors; no cause is assigned without an isolated proof.

Intermediate comparisons remain in `mem-limit-r{1,2,3,4,5}-ab/` and
`mem-limit-r3-linear-ab/`, 30 measured triplets after six warmups, 64 batches.
The final small r5 comparison overlapped a donor build and is exploratory;
the full matrix above has no such task-launched compiler/profiler overlap.
Whole-process 100 µs CPU samples in `mem-limit-profile-{before,r1}-pool/` point
to resource allocation setup, placement, binding and registry bookkeeping.
They include startup/cleanup and cannot price individual safety checks.
VMA also validates requirements and accounts for allocations. vk additionally
checks generation-bearing tokens, duplicate native outputs and recoverable
callback/metadata failures; no measured percentage is assigned to each check.
Ash's nullable-PFN policy does not explain this allocator comparison.

Synchronization policy also differs: this native VMA consumer leaves
`VmaAllocatorCreateInfo::flags` at zero, enabling VMA's internal mutexes.
Ordinary vk uses exclusive Rust access; its synchronized wrapper is a separate
API and is not timed here. Virtual placement is externally synchronized on both
sides. The native rows therefore compare these configured public APIs, not two
identically synchronized internals. An externally synchronized VMA cohort and a
matched synchronized-vk cohort remain unmeasured.

### Follow-up correctness and stability status

- **32/32 named native profiles pass under each of core and synchronization
  validation**, with all six optional profiles required and no skips or
  validation diagnostics: `mem-limit-{core,sync}/`.
- **44/44 benchmark configurations pass for both vk and VMA under each
  validation mode**, 176 executions total, including 30 native and 14 host-only
  virtual cells per mode: `mem-limit-matrix-{core,sync}/`. Layer insertion is
  checked on every native process. Instrumented times are excluded.
- Pinned donor and independent models pass **56 virtual traces / 112,344
  operations**, **1,024 native traces / 756,000 operations**, and **12
  concurrency comparisons**, with zero diagnostics. C++ ASan/UBSan/leak checks
  cover the donor/driver, not Rust sanitizer instrumentation.
- Added stale-generation tests cross linear compaction, upper/lower retirement
  and offset reuse. Memory-type selection tests cover every declared count
  through 32 and reject out-of-count mask bits. The seeded registry oracle now
  checks conditional retirement and retained-value mutation against BTreeMap.
- Workspace all-feature tests, all-target strict Clippy, formatting, generator
  freshness and no-default-feature compilation pass in a disposable source
  snapshot excluding the unrelated pipeline benchmark registration/source.
  The live-tree allocator tests/strict Clippy and 50 Python tool tests pass.
  All 11 SDK evidence tests, warnings-denied rustdoc, rendered references and
  shared-rule synchronization pass. Numeric tables and local links are checked
  against the retained JSON; earlier matrix values and November 13 stay intact.
  The unrelated `Cargo.toml` / `test/rs/test_pipe_bench.rs` work is preserved.

## Ash: complete retained paired baseline — November 13

Source: `surface-index-confirm-20261113-startup-r2/` and
`surface-index-confirm-20261113-query/`, 30 measured pairs per case. Each query
process performs 4,096 fresh native queries. Startup measures one sequence.

| Operation | Paired vk median | Ash median | vk/Ash | 95% interval |
| --- | ---: | ---: | ---: | ---: |
| Loader opening | 81.410 µs | 81.995 µs | 1.018 | 0.949–1.077 |
| Instance creation | 109.044 ms | 109.834 ms | 1.552 | 0.331–2.983 |
| Device creation | 1.784 ms | 1.783 ms | 1.027 | 0.977–1.059 |
| Combined startup | 110.800 ms | 111.721 ms | 1.527 | 0.339–2.922 |
| Fresh memory-properties query | 14.726 ns | 9.683 ns | 1.538 | 1.456–1.617 |

Instance creation was bimodal (~55 and ~168 ms clusters on both backends), so
its ratio and the combined startup ratio are not useful rankings. Ash performs
resolver-null checks while loading tables, installing panic stubs for missing
commands. vk retains recoverable nullable-PFN checks. For the 520-byte void
property output, vk zero-initializes while Ash uses MaybeUninit; optimized-binary
inspection found both result copies, but only vk's memset and per-call branch.
That wrapper contract does not explain allocator performance or GPU work.

## Vulkano: complete retained managed-API baseline — November 13

Sources: `surface-index-vulkano-20261113-{startup,buffer-pool,buffer-separate}/`,
30 measured pairs per case; 4,096 allocations/frees per buffer process. These
are the original paired vk/Vulkano consumers, not current optimized vk timings.

| Operation | Paired vk median | Vulkano median | vk/Vulkano | 95% interval |
| --- | ---: | ---: | ---: | ---: |
| Loader opening | 0.083 ms | 28.830 ms | 0.003 | 0.003–0.003 |
| Instance creation | 108.258 ms | 26.274 ms | 4.456 | 2.724–6.412 |
| Device creation | 1.961 ms | 1.999 ms | 0.959 | 0.935–0.991 |
| Combined startup | 110.292 ms | 112.812 ms | 1.552 | 0.337–2.901 |
| buffer-pool create/allocate/bind | 284.299 ns | 164.854 ns | 1.728 | 1.661–1.851 |
| buffer-pool destroy/free | 89.908 ns | 82.092 ns | 1.121 | 1.083–1.140 |
| buffer-pool combined | 373.990 ns | 246.523 ns | 1.532 | 1.454–1.592 |
| buffer-separate create/allocate/bind | 634.338 ns | 2380.826 ns | 0.264 | 0.251–0.272 |
| buffer-separate destroy/free | 146.356 ns | 791.953 ns | 0.182 | 0.176–0.189 |
| buffer-separate combined | 782.910 ns | 3150.062 ns | 0.245 | 0.232–0.252 |

Vulkano performs more eager work during loading, so individual startup phases
are not equal work; the bimodal instance times also invalidate a stable startup
ranking. Its checked Buffer::new/Drop path uses Arc ownership, FreeList placement
and automatically mapped host-visible backing. vk is caller-managed and unsafe.
The one-block pooled and one-native-allocation-per-buffer policies, memory type,
sizes, live counts and native accounting match; ownership, mapping and token
representation do not. The separate result therefore includes Vulkano map/unmap
and ownership costs. No unchecked Vulkano APIs or cached-query substitution.

## VMA: complete retained pre-optimization baseline — November 13

Sources: `surface-index-confirm-20261113-{virtual,buffer-pool,buffer-separate}/`;
30 pairs per case, 4,096 allocations and frees. This historical cohort is kept
separate from both November 14 optimization passes.

| Operation | Paired vk median | VMA median | vk/VMA | 95% interval |
| --- | ---: | ---: | ---: | ---: |
| virtual allocate | 23.895 ns | 13.960 ns | 1.722 | 1.663–1.772 |
| virtual free | 13.829 ns | 4.343 ns | 3.163 | 3.074–3.215 |
| virtual combined | 37.721 ns | 18.544 ns | 2.051 | 1.991–2.108 |
| buffer-pool allocate | 289.248 ns | 146.047 ns | 2.035 | 1.927–2.091 |
| buffer-pool free | 91.560 ns | 76.168 ns | 1.224 | 1.161–1.260 |
| buffer-pool combined | 381.195 ns | 220.958 ns | 1.744 | 1.675–1.808 |
| buffer-separate allocate | 646.477 ns | 300.499 ns | 2.118 | 2.062–2.199 |
| buffer-separate free | 148.983 ns | 138.924 ns | 1.067 | 1.034–1.134 |
| buffer-separate combined | 796.615 ns | 437.564 ns | 1.789 | 1.707–1.873 |

## Earlier VMA matrix: all 44 configurations — November 14

Source: `mem-push-matrix/results.json`, build `mem-push-compact/`. Each cell has
20 fresh-process pairs after three warmup pairs, alternating vk/VMA order,
64 batches and the live count shown below. Sizes repeat 64, 256, 4,096, 16,384,
65,536 bytes; virtual alignment is 64. Native pools are one preallocated 64 MiB
block, min/max one. Separate storage uses one native allocation per resource.
Images are R8_UNORM optimal 2D, width 64, height requested bytes / 64, one mip,
one layer and transfer-src/dst usage. No GPU accesses occur during timing.

Reverse is LIFO; FIFO reverses retirement order; shuffle uses a fixed xorshift64
permutation. Churn frees odd-indexed regions while even regions remain live,
then replaces the removed sizes in reverse order before final cleanup. It
measures the extra allocations and frees: a 128-live process has 8,192 operations
per side normally or 12,288 with churn. Counts scale with the listed live count.

Token storage is pre-reserved; allocator metadata growth remains timed. Trace
permutation, native requirements/type/alignment/extent and overlap inspection,
initial/fragmented occupancy, JSON and cleanup accounting checks are untimed.
The pinned VMA linear virtual consumer uses a never-dereferenced user-data
sentinel because its statistics uses that field to identify live entries; vk
requires no such sentinel. Both consumers execute the same declared trace and
Python independently checks the eviction checksum and operation/byte counts.

Every row below is one complete measured cell. Allocation/free columns show
**vk ns / VMA ns**. Combined is the paired ratio of allocation + free, with its
95% interval; mapping is shown separately immediately after the full table.

| ID | Workload | Algorithm / strategy | Trace / live / mapping | Allocate vk / VMA ns | Free vk / VMA ns | Combined vk/VMA (95% interval) |
| --- | --- | --- | --- | ---: | ---: | ---: |
| 00 | virtual | tlsf / default | reverse / 128 / none | 27.32 / 27.51 | 8.05 / 9.35 | 1.020 (0.920–1.446) |
| 01 | virtual | tlsf / default | fifo / 128 / none | 27.68 / 18.47 | 14.17 / 12.58 | 1.330 (1.280–1.389) |
| 02 | virtual | tlsf / default | shuffle / 128 / none | 27.38 / 18.73 | 14.67 / 13.45 | 1.305 (0.933–1.333) |
| 03 | virtual | tlsf / default | churn / 128 / none | 31.21 / 29.87 | 11.83 / 10.77 | 1.095 (1.030–1.287) |
| 04 | virtual | linear / default | reverse / 128 / none | 23.57 / 21.84 | 25.45 / 11.58 | 1.128 (0.941–1.224) |
| 05 | virtual | linear / default | fifo / 128 / none | 19.26 / 21.35 | 19.90 / 10.81 | 1.226 (0.973–1.252) |
| 06 | virtual | linear / default | shuffle / 128 / none | 26.36 / 28.16 | 32.18 / 24.96 | 1.023 (0.860–1.214) |
| 07 | virtual | linear / default | churn / 128 / none | 20.00 / 28.64 | 21.34 / 15.16 | 0.944 (0.912–1.002) |
| 08 | buffer-pool | tlsf / default | reverse / 128 / none | 325.36 / 188.15 | 118.56 / 99.90 | 1.535 (1.505–1.555) |
| 09 | buffer-pool | tlsf / default | fifo / 128 / none | 330.80 / 194.05 | 133.58 / 105.79 | 1.551 (1.523–1.568) |
| 10 | buffer-pool | tlsf / default | shuffle / 128 / none | 330.53 / 188.00 | 159.03 / 107.69 | 1.657 (1.640–1.672) |
| 11 | buffer-pool | tlsf / default | churn / 128 / none | 334.59 / 185.27 | 145.67 / 101.93 | 1.656 (1.641–1.698) |
| 12 | buffer-pool | linear / default | reverse / 128 / none | 332.66 / 190.48 | 125.65 / 104.49 | 1.560 (1.536–1.574) |
| 13 | buffer-pool | linear / default | fifo / 128 / none | 338.49 / 194.29 | 131.32 / 105.80 | 1.563 (1.546–1.578) |
| 14 | buffer-pool | linear / default | shuffle / 128 / none | 344.53 / 192.95 | 146.43 / 116.47 | 1.585 (1.556–1.624) |
| 15 | buffer-pool | linear / default | churn / 128 / none | 341.63 / 189.12 | 143.87 / 108.38 | 1.631 (1.605–1.656) |
| 16 | image-pool | tlsf / default | reverse / 128 / none | 1269.59 / 973.52 | 205.71 / 189.36 | 1.292 (1.260–1.314) |
| 17 | image-pool | tlsf / default | fifo / 128 / none | 1243.97 / 948.75 | 213.26 / 190.03 | 1.303 (1.286–1.340) |
| 18 | image-pool | tlsf / default | shuffle / 128 / none | 1279.65 / 958.75 | 236.93 / 194.99 | 1.317 (1.293–1.342) |
| 19 | image-pool | tlsf / default | churn / 128 / none | 1251.76 / 937.99 | 218.11 / 187.58 | 1.298 (1.288–1.319) |
| 20 | image-pool | linear / default | reverse / 128 / none | 1061.88 / 831.71 | 188.44 / 170.12 | 1.244 (1.237–1.266) |
| 21 | image-pool | linear / default | fifo / 128 / none | 1488.00 / 1266.91 | 271.51 / 238.66 | 1.265 (1.159–1.461) |
| 22 | image-pool | linear / default | shuffle / 128 / none | 1079.60 / 853.94 | 208.38 / 182.05 | 1.267 (1.249–1.295) |
| 23 | image-pool | linear / default | churn / 128 / none | 1042.48 / 800.15 | 194.23 / 169.37 | 1.277 (1.265–1.302) |
| 24 | buffer-separate | tlsf / default | reverse / 128 / none | 470.77 / 333.69 | 159.50 / 159.55 | 1.280 (1.268–1.303) |
| 25 | buffer-separate | tlsf / default | churn / 128 / none | 459.58 / 308.10 | 177.23 / 160.33 | 1.371 (1.333–1.388) |
| 26 | image-separate | tlsf / default | reverse / 128 / none | 1268.77 / 1077.17 | 236.08 / 232.48 | 1.158 (1.135–1.168) |
| 27 | image-separate | tlsf / default | churn / 128 / none | 987.99 / 822.37 | 197.09 / 187.06 | 1.172 (1.138–1.181) |
| 28 | buffer-pool | tlsf / default | reverse / 128 / cycle | 245.06 / 145.38 | 1516.42 / 1379.96 | 1.162 (1.132–1.182) |
| 29 | buffer-pool | tlsf / default | churn / 128 / cycle | 263.05 / 140.82 | 1085.03 / 952.95 | 1.229 (1.206–1.251) |
| 30 | buffer-pool | linear / default | reverse / 128 / cycle | 265.34 / 144.25 | 1530.09 / 1376.99 | 1.184 (1.161–1.199) |
| 31 | buffer-pool | linear / default | churn / 128 / cycle | 262.07 / 140.92 | 1077.92 / 959.91 | 1.205 (1.192–1.221) |
| 32 | virtual | tlsf / min-memory | churn / 128 / none | 19.55 / 13.45 | 7.14 / 5.48 | 1.432 (1.396–1.480) |
| 33 | virtual | tlsf / min-time | churn / 128 / none | 22.16 / 15.30 | 8.24 / 7.22 | 1.313 (1.289–1.353) |
| 34 | virtual | tlsf / min-offset | churn / 128 / none | 50.15 / 74.24 | 8.27 / 6.71 | 0.691 (0.670–0.732) |
| 35 | virtual | tlsf / default | reverse / 1 / none | 45.57 / 43.02 | 20.91 / 22.62 | 1.023 (0.979–1.046) |
| 36 | virtual | tlsf / default | churn / 32 / none | 23.04 / 17.28 | 9.20 / 8.62 | 1.237 (1.210–1.270) |
| 37 | virtual | tlsf / default | churn / 256 / none | 20.23 / 14.44 | 7.74 / 6.46 | 1.331 (1.321–1.383) |
| 38 | buffer-pool | tlsf / min-memory | churn / 128 / none | 268.53 / 148.40 | 111.41 / 81.78 | 1.664 (1.625–1.729) |
| 39 | buffer-pool | tlsf / min-time | churn / 128 / none | 241.91 / 136.01 | 105.24 / 74.74 | 1.640 (1.603–1.659) |
| 40 | buffer-pool | tlsf / min-offset | churn / 128 / none | 273.24 / 194.66 | 101.69 / 74.12 | 1.381 (1.366–1.438) |
| 41 | buffer-pool | tlsf / default | reverse / 1 / none | 384.98 / 345.85 | 121.38 / 98.05 | 1.172 (1.090–1.217) |
| 42 | buffer-pool | tlsf / default | churn / 32 / none | 240.47 / 132.97 | 101.82 / 70.54 | 1.673 (1.605–1.724) |
| 43 | buffer-pool | tlsf / default | churn / 256 / none | 244.38 / 137.05 | 103.82 / 76.29 | 1.627 (1.616–1.645) |

### Earlier mapping: all four timed cycle profiles

The map timer includes map + an eight-byte host write + unmap. A second mapping
and independent payload read/check occur outside timing. These checks affect
mapping-hysteresis history before final free in both implementations, so do not
compare these free times to unmapped lifetimes as if they were the same policy.
There is no cache flush/invalidate or GPU visibility claim on this coherent type.

| Matrix ID | Algorithm / trace | vk ns/cycle | VMA ns/cycle | vk/VMA | 95% interval |
| --- | --- | ---: | ---: | ---: | ---: |
| 28 | tlsf / reverse | 7249.88 | 7204.97 | 1.006 | 0.995–1.012 |
| 29 | tlsf / churn | 7280.88 | 7270.72 | 1.001 | 0.981–1.012 |
| 30 | linear / reverse | 7265.65 | 7213.63 | 1.005 | 0.997–1.013 |
| 31 | linear / churn | 7282.42 | 7150.01 | 1.009 | 0.999–1.023 |

The clearest observed allocator lead is minimum-offset virtual churn (ID 34).
vk walks the physical list forward rather than materializing VMA's temporary
candidate list. This is an existing policy adaptation exposed by the broader
benchmark, not a claim that this optimization pass newly created the whole lead.
Linear virtual churn's combined interval crosses one; default virtual timings
are noisy. Native pooled buffer creation still trails VMA. The four mapping
ratios are near parity. No geometric mean is used to hide workload differences.

## Every November 14 implementation comparison

These are rotating original/optimized/VMA triplets: 30 measured triplets after
six warmups, 32 batches of 128. Each experiment uses an identical reference
binary on both sides. Phase 1/admission use the earlier minimal consumer;
compact/outlining use the expanded consumer. Compare within a row, not across
harness generations. Values below combine allocation and free.

| Experiment / status | Workload | Before vk/VMA | After vk/VMA | After/before vk (95% interval) |
| --- | --- | ---: | ---: | ---: |
| Phase 1 / retained | virtual | 1.997 | 1.183 | 0.596 (0.557–0.641) |
| Phase 1 / retained | buffer-pool | 1.780 | 1.642 | 0.920 (0.909–0.933) |
| Phase 1 / retained | buffer-separate | 1.899 | 1.337 | 0.696 (0.685–0.720) |
| Admission reuse / retained | virtual | 1.124 | 1.130 | 1.066 (1.018–1.098) |
| Admission reuse / retained | buffer-pool | 1.599 | 1.558 | 0.943 (0.936–0.967) |
| Admission reuse / retained | buffer-separate | 1.341 | 1.318 | 0.984 (0.957–1.021) |
| Compact node / retained | virtual | 1.555 | 1.503 | 0.977 (0.954–0.990) |
| Compact node / retained | buffer-pool | 1.559 | 1.549 | 0.981 (0.976–0.988) |
| Compact node / retained | buffer-separate | 1.290 | 1.289 | 1.000 (0.987–1.017) |
| Neighbor outlining / discarded | virtual | 1.447 | 1.472 | 0.974 (0.954–1.054) |
| Neighbor outlining / discarded | buffer-pool | 1.504 | 1.510 | 0.997 (0.986–1.006) |
| Neighbor outlining / discarded | buffer-separate | 1.295 | 1.289 | 1.010 (0.958–1.040) |

The admission pass reduced pooled combined time by about 6% in its matched run;
separate storage was inconclusive. Its virtual result regressed despite no
virtual algorithm change; that result remains unattributed rather than being
silently dropped. The compact-node pass improved default virtual/pooled time by
about 2–3%; separate storage was unchanged. The outlining experiment showed no
clear gain and was removed. Shared load/code-layout sensitivity remain reasons
to repeat on an idle machine before adopting regression thresholds.

Retained implementation changes preserve public names/signatures and checks:

- Callback-owned native-backing index replaces a scan of every live allocation.
  Reservation precedes native allocation, and retirement/scoped restoration
  maintain the index without losing duplicate-output rejection.
- Virtual free checks the owner once and delegates generation/liveness checking
  to metadata; forwarding/capacity paths inline while growth/rehash stays cold.
  Registry insertion probes once and redundant association/pool lookups are removed.
- Managed creation reuses its already-validated options/publication admission;
  native requirements, output validity, binding and rollback remain checked.
- TLSF nodes shrink from **72 to 56 bytes on x86-64**, reusing free-list storage
  for live generation/user data. No pointer punning, narrowed generations or
  unchecked array access is added. Near-u64-exhaustion/reuse/user-data proofs pass.
- The free-range visitor no longer emits the tail twice. The stronger physical
  range oracle first reproduced the duplicate, then verified exact-once visits
  across split/coalesce/reuse traces. This fixes diagnostic and relocation
  candidate evidence rather than weakening a check to improve a benchmark.
- Allocation snapshots reuse the validated live record for their metadata fields.
  This path was not separately timed, so no isolated latency gain is claimed.

CPU sampling of an intermediate build used 100 µs userspace IP samples over
whole processes, including startup/cleanup. It identifies forwarding and native
placement/binding/bookkeeping work but does not assign nanoseconds to individual
checks or establish equal driver work. Ash's property-wrapper null policy is
not an explanation for allocator ratios. VMA also validates and accounts.

## Earlier correctness and stability status

- Final allocator: **32/32 named native profiles passed separately under core
  and synchronization validation**, all six optional profiles required, with
  no skips or validation warnings/errors/VUIDs/hazards.
- Expanded benchmark programs: **44/44 cells passed on both vk and VMA in each
  validation mode**: 88 backend executions per mode, 176 total. Of the 44 cells,
  30 exercise native resources and 14 are host-only virtual cells. Instrumented
  timings are excluded. Layer insertion is required for every native execution.
- Pinned donor: **56 virtual traces / 112,344 operations**, **1,024 native
  traces / 756,000 operations**, and **12 concurrency comparisons**, independent
  oracles and zero diagnostics. C++ ASan/UBSan/leak detection covers the donor
  and driver; it is not Rust sanitizer coverage.
- Workspace tests, allocator-target strict Clippy, generator freshness,
  no-default-features compilation, Python evidence tests, SDK all-feature
  tests/strict Clippy, scoped formatting, rustdoc and rendered links pass.
  Whole-tree Clippy/formatting remain blocked by the independently edited
  `test/rs/test_pipe_bench.rs`; that work is preserved.

Core profiles include real compute/fence/readback, buffer/image placement,
per-backing mapping/binding concurrency, buffer/image and multi-pass/alias/
descriptor relocation copies, timeline/address work and OPAQUE_FD/DMA-BUF imports.
The retained offscreen graphics oracle passed; graphics throughput was not timed.
These are named workloads, not Vulkan conformance, WSI or universal safety proof.
Unsafe native pointer/chain validity, GPU completion, visibility and Rust
reference aliasing remain caller obligations.

## Remaining coverage

The 44-cell `full` suite means the complete bounded matrix above, **not the
entire public feature set**. Still unmeasured: multi-block pressure/exhaustion,
automatic/adaptive growth and budgets, ring wrap/upper-stack latency, persistent
map lifetimes, cache flush/invalidate on actual non-coherent hardware, uploads,
allocation-info/statistics/JSON latency, real-copy relocation phases, contention,
external-memory timing, debug instrumentation overhead, RSS/CPU allocation and
native-call counters, cross-vendor/platform results and matched GPU workloads.
Broader Ash command-recording and Vulkano image/explicit-map consumers are also
open. The [SDK comparison coverage](../../../sdk/README.md#honest-comparisons)
lists the workload contracts and remaining timing work.

## Reproduce the current suite

Choose a compatible host memory type/CPU; type 2 and CPU 2 are specific to this run.
Keep validation and timing in separate evidence directories.

```bash
python3 sdk/tool/bench.py build --build target/sdk/memory-build
python3 sdk/tool/memory.py run --suite full --build target/sdk/memory-build \
  --api 1.4 --memory-type 2 --cpu 2 --samples 20 --warmup 3 --batches 64 \
  --output target/sdk/memory-timing
python3 sdk/tool/memory.py validate --suite full --build target/sdk/memory-build \
  --api 1.4 --memory-type 2 --validation core \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/memory-core
python3 sdk/tool/memory.py validate --suite full --build target/sdk/memory-build \
  --api 1.4 --memory-type 2 --validation sync \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/memory-sync
```

## Evidence and exact build identities

All raw evidence is ignored under `target/`. Matrix cells retain source/artifact
hashes, commands, samples/warmups, telemetry, requested-byte/eviction checksums,
initial/fragmented occupancy, mapping cycles and diagnostics. Values in this
report are recomputed from those JSON records. No previous result is overwritten.

- Latest timing matrix: `target/sdk/mem-limit-matrix/`; earlier: `mem-push-matrix/`.
- Latest native profiles: `target/sdk/mem-limit-{core,sync}/`.
- Latest benchmark validation: `target/sdk/mem-limit-matrix-{core,sync}/`.
- Latest donor logs: `target/verify_memory/mem-limit-{virtual,native,concurrency}`.
- Latest local orchestration: `target/sdk/mem-limit-{ab,matrix,report}.py`.
- Final native profiles: `target/sdk/mem-push-{core,sync}/`.
- Benchmark-program validation: `target/sdk/mem-push-matrix-{core,sync}/`.
- Donor evidence/logs: `target/verify_memory/mem-push-{virtual,native,concurrency}`.
- Immutable triplet orchestration: `target/sdk/mem-ab.py`; CPU sampling:
  `target/sdk/mem-profile.py`, `mem-opt-profile-r3-{pool,virtual}/`.
- Retained graphics: `target/sdk/surface-triangle-run-20261113/`.

Compact sorted JSON source-inventory hashes and timed executable identities:

| Build | Source inventory SHA-256 | vk-bench SHA-256 | VMA SHA-256 |
| --- | --- | --- | --- |
| mem-opt-before | `d7be191e139cbe39da20e347210e3bad86fcdfc0b798b287c166d6ff4ddcc163` | `e03dddbd348a6d58727ece479f5b4dcb6b99c8259cd0072fccf9f5bd46e05563` | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |
| mem-opt-r4 | `5e9f786f9445653f195c6cb3b80e67e257c705aafacb512c71679159e39ee388` | `b62e607680c70ae3d5afcb844f856ae7c790e279acc8c20691442048974ba3ca` | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |
| mem-push-admission | `26135d09b15f9fb21fab9046b134a17d48836e7a977bcf434d313843c98a4ed5` | `c6ff27c50af847f768a253cf0698b3db6f072663fa4d9d174806d3e5faee4347` | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |
| mem-push-suite-r2 | `980efe54fc7e5951e3ba792b151472acdcbbef99d597b47807f621b16219894f` | `3cebd789a174be54292236eea8c799e940be2d5f38b07758faf060aecf548fab` | `79232c7839d786ab35103d1da01744108dc07bddfcd58cad023b03afe0069493` |
| mem-push-compact | `ab6cdd4f17fd7da8ea2ba1d4a0f2c7c3b34640cf58c39d7f2033657d6398defc` | `c3ada6ea384eb9610bd022b25345d785ae4803f726b89936982e822f66019cc7` | `79232c7839d786ab35103d1da01744108dc07bddfcd58cad023b03afe0069493` |
| mem-push-final | `d3fd8edca98f666623fa8ff7a2eb27624dd12fdbeaa71ef82f29cfcead1ef0f4` | `c3ada6ea384eb9610bd022b25345d785ae4803f726b89936982e822f66019cc7` | `79232c7839d786ab35103d1da01744108dc07bddfcd58cad023b03afe0069493` |
| mem-push-qualified-r2 | `7ed71d5bcf2c126c67e91467cb8aca6ac689c69f24b4ccbd330bf179f57a43ab` | `c3ada6ea384eb9610bd022b25345d785ae4803f726b89936982e822f66019cc7` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| mem-limit-r1 | `4dac183496c8ea52029559d638c17f4c6d0e0aff53db1b1649cc50bc0b762e1a` | `ed7a6c84f382beea30d37253b9cedd8c45d3d2dc6201c447e04ab7df012bb854` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| mem-limit-r2 | `9d7881e9cb656da9ec16e49ba1634c21ecde5ad5fee910b4389d7a0fdf4cf73e` | `68c9da37dfa69d7619a8322d8dfa2442b7c917891c106b99b3ccaa7cefda559f` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| mem-limit-r3 | `4749824a89d247215f2d3f632cd13906fe04ea1fdb4e66262a1ddb2ed974931a` | `57218a4bee3299d066e216df5f72866fdf1d0820d0f24ce833816be58eeaf77b` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| mem-limit-r4b | `73c55170222cad95259c0a75f65f0eb44f989a5639bcdd18d3b7104279b4a2c5` | `513cb6acb8e8c9b4a4af7328f7cafa50eb6b2a7f79c2d998ba223d720702f887` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| mem-limit-r5 | `24a8a112099353f3b8756ec567dbb7e265627da638997453a644454c5b1713d9` | `4ccf3a7aeb3708db8f574beab5ce41e0910d803fd5b9c28ac886486fceb2f3ab` | `f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0` |
| surface-build-r10 | `45834efe873484e63735dbfda3f2cedbb93faee10e66abf8b21603c3d1a85ab6` | `5875981ec89e5e3ade5f20a789007e290c46e4f582ac6a0287bcb3dffb946d7d` | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |
| surface-build-vulkano-r10 | `45834efe873484e63735dbfda3f2cedbb93faee10e66abf8b21603c3d1a85ab6` | `3a18ed23fb9f71c3626b88c89341bfa2475d8908142508787eab222d4e213787` | `52b58b8793037f0baeabb70ceee338afacb3aea9d444753f043eb88c25679504` |

The **earlier** matrix and qualification vk-bench files are **byte-identical** at
`c3ada6ea384eb9610bd022b25345d785ae4803f726b89936982e822f66019cc7`.
The latest qualification inventory adds scheduler/layer tests and the validator;
its VMA consumer also adds an untimed native-backing extent assertion. The timed
matrix VMA hash remains the one in `mem-push-compact/`; it is not relabeled as the
newer validation executable. Historical reference executable hashes are retained
in their build manifests, including Ash and Vulkano.

| Historical timed reference | Build | SHA-256 |
| --- | --- | --- |
| Ash | surface-build-r10 | `615fb4641c130a473828dd64af9ad5be359ad13e88165d38e7c8152285a87f45` |
| Vulkano | surface-build-vulkano-r10 | `f6346377e38aef0f618afe83aa39d97973dec8946eca307a51cfaa5a80ef5469` |

### Complete retained timing-run inventory

Each entry is a separate cohort, not an independent vote in a pooled ranking.
Older passes are superseded for current performance claims but retained for audit.
The 44 nested current matrix records are enumerated by ID in the full table.

| Evidence directory under target/sdk | Workloads / reference | Measured samples | Status |
| --- | --- | ---: | --- |
| `mem-limit-r1-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Retained setup/binding experiment |
| `mem-limit-r2-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Rejected split-registry experiment |
| `mem-limit-r3-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Retained selector/retirement experiment |
| `mem-limit-r3-linear-ab/` | virtual, buffer-pool; linear / immutable VMA | 30 triplets/case | Retained edge-lookup experiment |
| `mem-limit-r4-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Borrowed-chain representation; isolated gain inconclusive |
| `mem-limit-r5-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Exploratory; donor compilation overlap |
| `mem-limit-shuffle-confirm/` | virtual TLSF shuffle / committed, candidate, VMA | 60 triplets/case | Longer repeat; no stable regression or speedup |
| `mem-limit-matrix/` | committed/candidate/VMA matrix: 44 passed | 20 triplets/cell | Complete latest cohort |
| `mem-opt-ab-r4/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Retained before/after experiment |
| `mem-opt-before-pool-hw/` | buffer-pool / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-before-separate-hw/` | buffer-separate / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-before-virtual/` | virtual / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r1-pool/` | buffer-pool / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r1-separate/` | buffer-separate / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r1-virtual/` | virtual / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r2-pool/` | buffer-pool / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r2-separate/` | buffer-separate / vma | 20 pairs/case | Retained; cohort-specific |
| `mem-opt-r3-pool/` | buffer-pool / vma | 30 pairs/case | Retained; cohort-specific |
| `mem-opt-r3-separate/` | buffer-separate / vma | 30 pairs/case | Retained; cohort-specific |
| `mem-opt-r3-virtual/` | virtual / vma | 30 pairs/case | Retained; cohort-specific |
| `mem-push-admission-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Retained before/after experiment |
| `mem-push-compact-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Retained before/after experiment |
| `mem-push-fit-ab/` | virtual, buffer-pool, buffer-separate / immutable VMA | 30 triplets/case | Discarded outlining experiment |
| `mem-push-matrix/` | VMA matrix: 44 passed | 20 pairs/passed cell | Complete |
| `mem-push-smoke/` | VMA matrix: 1 failed, 2 passed, 5 pending | 10 pairs/passed cell | Partial/failed; not full qualification |
| `mem-push-smoke-r2/` | VMA matrix: 8 passed | 10 pairs/passed cell | Complete |
| `surface-ash-vma-20261113/` | startup / ash; query / ash; virtual / vma; buffer-pool / vma; buffer-separate / vma | 30/30/30/30/30 pairs/case | Retained; cohort-specific |
| `surface-buffer-pool-confirm-20261113/` | buffer-pool / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-buffer-separate-confirm-20261113/` | buffer-separate / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-buffer-pool/` | buffer-pool / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-buffer-separate/` | buffer-separate / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-query/` | query / ash | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-startup/` | startup / ash | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-startup-r2/` | startup / ash | 30 pairs/case | Retained; cohort-specific |
| `surface-index-confirm-20261113-virtual/` | virtual / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-index-vulkano-20261113-buffer-pool/` | buffer-pool / vulkano | 30 pairs/case | Retained; cohort-specific |
| `surface-index-vulkano-20261113-buffer-separate/` | buffer-separate / vulkano | 30 pairs/case | Retained; cohort-specific |
| `surface-index-vulkano-20261113-startup/` | startup / vulkano | 30 pairs/case | Retained; cohort-specific |
| `surface-query-confirm-20261113/` | query / ash | 50 pairs/case | Retained; cohort-specific |
| `surface-startup-confirm-20261113/` | startup / ash | 50 pairs/case | Retained; cohort-specific |
| `surface-virtual-confirm-20261113/` | virtual / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-assoc-20261113/` | buffer-pool / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-assoc-sep-20261113/` | buffer-separate / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-assoc-virt-20261113/` | virtual / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-index-20261113-buffer-pool/` | buffer-pool / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-index-20261113-buffer-separate/` | buffer-separate / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vma-index-20261113-virtual/` | virtual / vma | 30 pairs/case | Retained; cohort-specific |
| `surface-vulkano-20261113/` | startup / vulkano; buffer-pool / vulkano; buffer-separate / vulkano | 30/30/30 pairs/case | Retained; cohort-specific |
| `surface-vulkano-buffer-pool-confirm-20261113/` | buffer-pool / vulkano | 30 pairs/case | Retained; cohort-specific |
| `surface-vulkano-buffer-separate-confirm-20261113/` | buffer-separate / vulkano | 30 pairs/case | Retained; cohort-specific |
| `surface-vulkano-startup-confirm-20261113/` | startup / vulkano | 50 pairs/case | Retained; cohort-specific |

`mem-opt-before-pool/` retains the failed sandbox invocation (no device access),
not an accepted timing result. The first matrix smoke attempt failed VMA linear
statistics without the required sentinel; it remains marked failed/pending.
Successful replacement runs retain new directories. Configured/planned cells,
profiling samples and validation runs are never counted as timing successes.
