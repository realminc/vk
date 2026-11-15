# Surface Laptop 5 benchmark inventory — 2026-11-15

This report collects **every measured workload and retained timing run for the
November 15 allocator passes**: a 44-case release/candidate/VMA matrix, its
four mapping measurements, the optimization experiments leading to the
candidate, a second complete vk/VMA matrix over the publication-token pass,
the mutex/read-write-lock study, and the validation record. The
[November 14 inventory](surface-laptop-5-2026-11-14.md) remains unchanged,
including its Ash, Vulkano and earlier VMA comparisons; those were not rerun
in this allocator-only pass.

The candidate is an **uncommitted optimization pass over release v0.13.1**,
compared with the immutable release build and the same VMA executable. Of the
44 combined allocation/free workloads, **41 improve**, **1 regresses** and **2
are inconclusive** by their individual paired-bootstrap 95% intervals. These
are descriptive, unadjusted per-cell intervals, not a multiple-comparison-
adjusted regression gate. **3/44** candidate/VMA medians are below 0.9; **1/44**
has its entire interval below 0.9. The across-the-suite 0.9 target remains
unmet. This is not an allocator-wide, cross-vendor or security ranking.

Default virtual TLSF reverse lifetime improves by **5.6%** to **1.291× VMA**;
pooled TLSF buffer reverse lifetime improves by **5.5%** to **1.298× VMA**.
Minimum-offset virtual churn reaches **0.710× VMA**. The largest remaining
measured gap is pooled TLSF churn with 32 live buffers at **1.450× VMA**.

The nominal regression was cell 30 (linear pooled buffers, reverse free, mapping
cycles): **1.008× release (1.000–1.027)**. A separate 60-triplet repeat with
six warmups and 1,024 batches returned **1.000× (0.991–1.007)**. It did not
establish a repeatable regression or speedup. Both cohorts remain visible;
the repeat does not erase the original result.

## Which comparisons exist

| Workload | Ash | VMA | Vulkano |
| --- | --- | --- | --- |
| Loader / instance / device startup | Retained November 13 paired baseline in the November 14 report | Startup is setup, not separately timed | Retained November 13 managed-API baseline |
| Fresh native memory-properties query | Retained November 13 paired baseline | Not separately timed | Cached getter excluded from native-query ranking |
| Virtual TLSF / linear placement, free order and strategies | No allocator counterpart in this harness | Latest 44-case matrix subset | No virtual-placement consumer |
| Native pooled / separate buffers and images | No managed allocator comparison | Latest matrix | Retained November 13 managed buffers; no image consumer |
| Explicit map / host write / unmap | Not separately benchmarked | Four current pooled-buffer profiles | Automatic mapping has different ownership/policy; no explicit-cycle comparison |
| GPU compute, graphics and relocation | Correctness profiles only | Donor/correctness comparisons; no GPU timing ranking | No matched GPU timing comparison |

A dash or missing counterpart must not be read as a win. Ash is a binding
reference; VMA is the allocator reference. The current candidate has **not**
been timed against Ash or Vulkano in this round: their tables and binaries are
retained in the earlier reports with their own build manifests and are not
relabeled as current builds.

## Environment and interpretation

- Surface Laptop 5, Intel Core i7-1265U / Iris Xe (Alder Lake GT2), Arch Linux
  x86-64, kernel 7.3.4-arch1-1.
- Mesa/vulkan-intel 26.2.7, loader 1.4.363, Khronos layer 1.4.363.1; device API
  1.4.354. Queue family 0, host-visible/coherent/cached/device-local memory
  type 2, buffer/image granularity 1 and non-coherent atom size 64.
- Rust 1.98.1 / LLVM 22.1.8, release opt-level 3, thin LTO, one codegen unit;
  Clang/lld 23.1.5. VMA 3.4.0 pinned at
  `3aa921224c154a0d2c43912bc88e1c42ce1f7607`, C++17 `-O3 -DNDEBUG`.
- Release baseline: private commit
  `9bedf2cd37fbf76566381cf7f412f78f407d47c7` (v0.13.1), immutable retained
  build. Candidate: uncommitted working-tree pass; its dirty source inventory
  is hashed in every cell manifest.
- **20 measured fresh-process triplets per cell, three warmups, 512 batches per
  process**, all six release/candidate/VMA orders rotated, CPU 2. Original
  workload sizes, checksums, free order, occupancy, map payload checks and
  cleanup are unchanged.
- Native VMA retains its default internal mutexes; ordinary vk uses exclusive
  owner access. Virtual blocks are externally synchronized on both sides. This
  is not a matched synchronized-owner comparison.
- No task-launched compilation, profiling or validation overlaps the final
  matrix. Desktop/other-agent activity, clocks and thermals are uncontrolled;
  results remain exploratory. Longer repeats reduce short-sample noise but do
  not establish an idle-machine baseline.
- Timings exclude validation and reject runtime diagnostics. Separate
  core/synchronization runs validate the consumers. Native driver work and
  allocator checks are not individually attributed nanosecond costs.

**All ratios are vk/reference; below one favors vk.** Confidence intervals are
paired-bootstrap 95% intervals over fresh processes (4,000 resamples). Absolute
values are median process aggregates divided by the declared operation count,
not per-call p95/p99. The median paired ratio need not equal the ratio of
displayed medians. Intervals do not eliminate shared-workstation scheduling,
clock or thermal bias.

There are now four distinct harness generations: November 13 baselines;
November 14's original minimal reverse-free consumer; November 14's
image-capable matrix; and this release/candidate/VMA triplet matrix. Do not
stitch their absolute times into one speedup or infer current vk/Vulkano ratios
from current VMA ratios. The tables recompute values from the named retained
JSON rather than mixing cohorts.

## Latest allocator pass: release / candidate / VMA — November 15

Evidence: `mem-next-matrix/`. Every cell runs 20 measured fresh-process
**triplets** after three warmups, rotating all six release/candidate/VMA
process orders. The release executable is the byte-identical v0.13.1 allocator
from `sdk-build-r3/`; the VMA reference is identical in all triplets. CPU 2,
512 batches, type 2 and every workload policy, native check and operation count
match the earlier 44-cell matrices. The paired candidate/release column isolates
this implementation comparison; do not infer gains from changes between the
earlier and latest vk/VMA ratios.

Allocation/free columns are **candidate ns / VMA ns**. Combined excludes the
separately measured mapping cycle; mapping history still affects final free.
The last column is candidate/release combined time with its paired 95% interval.

| ID | Workload | Algorithm / strategy | Trace / live / mapping | Allocate vk / VMA ns | Free vk / VMA ns | vk/VMA combined (95% interval) | Candidate/release (95% interval) |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| 00 | virtual | tlsf / default | reverse / 128 / none | 17.08 / 12.71 | 5.12 / 4.55 | 1.291 (1.271–1.344) | 0.944 (0.906–0.987) |
| 01 | virtual | tlsf / default | fifo / 128 / none | 17.22 / 12.88 | 9.48 / 8.35 | 1.256 (1.218–1.290) | 0.971 (0.934–0.994) |
| 02 | virtual | tlsf / default | shuffle / 128 / none | 17.54 / 12.44 | 9.78 / 8.72 | 1.279 (1.253–1.328) | 0.933 (0.896–0.951) |
| 03 | virtual | tlsf / default | churn / 128 / none | 19.29 / 15.05 | 7.79 / 7.09 | 1.215 (1.203–1.250) | 0.980 (0.943–0.990) |
| 04 | virtual | linear / default | reverse / 128 / none | 12.28 / 14.12 | 6.92 / 7.83 | 0.878 (0.834–0.915) | 0.923 (0.880–0.952) |
| 05 | virtual | linear / default | fifo / 128 / none | 12.12 / 14.09 | 7.80 / 7.15 | 0.944 (0.918–0.964) | 0.926 (0.912–0.955) |
| 06 | virtual | linear / default | shuffle / 128 / none | 11.27 / 13.94 | 13.97 / 13.37 | 0.929 (0.906–0.958) | 0.949 (0.924–0.977) |
| 07 | virtual | linear / default | churn / 128 / none | 11.82 / 13.96 | 9.13 / 9.36 | 0.897 (0.888–0.912) | 0.924 (0.910–0.948) |
| 08 | buffer-pool | tlsf / default | reverse / 128 / none | 190.72 / 133.50 | 75.25 / 71.98 | 1.298 (1.264–1.328) | 0.945 (0.928–0.952) |
| 09 | buffer-pool | tlsf / default | fifo / 128 / none | 192.29 / 133.56 | 86.87 / 76.14 | 1.326 (1.308–1.358) | 0.970 (0.952–0.986) |
| 10 | buffer-pool | tlsf / default | shuffle / 128 / none | 195.75 / 130.86 | 102.51 / 75.29 | 1.446 (1.419–1.464) | 0.957 (0.945–0.970) |
| 11 | buffer-pool | tlsf / default | churn / 128 / none | 204.34 / 130.64 | 92.35 / 73.42 | 1.447 (1.430–1.459) | 0.951 (0.943–0.955) |
| 12 | buffer-pool | linear / default | reverse / 128 / none | 194.04 / 134.23 | 77.79 / 75.55 | 1.292 (1.282–1.319) | 0.985 (0.967–0.993) |
| 13 | buffer-pool | linear / default | fifo / 128 / none | 193.10 / 136.02 | 87.71 / 76.44 | 1.315 (1.301–1.341) | 0.984 (0.968–0.991) |
| 14 | buffer-pool | linear / default | shuffle / 128 / none | 195.29 / 136.07 | 99.29 / 81.87 | 1.355 (1.336–1.390) | 0.969 (0.959–0.988) |
| 15 | buffer-pool | linear / default | churn / 128 / none | 196.40 / 131.44 | 90.63 / 76.77 | 1.365 (1.352–1.386) | 0.969 (0.959–0.980) |
| 16 | image-pool | tlsf / default | reverse / 128 / none | 808.42 / 667.24 | 129.48 / 132.00 | 1.168 (1.161–1.179) | 0.954 (0.951–0.964) |
| 17 | image-pool | tlsf / default | fifo / 128 / none | 806.60 / 667.43 | 143.04 / 134.44 | 1.177 (1.170–1.194) | 0.961 (0.951–0.971) |
| 18 | image-pool | tlsf / default | shuffle / 128 / none | 807.44 / 672.69 | 143.50 / 135.35 | 1.193 (1.172–1.199) | 0.957 (0.943–0.964) |
| 19 | image-pool | tlsf / default | churn / 128 / none | 972.64 / 797.36 | 170.27 / 161.31 | 1.193 (1.172–1.201) | 0.943 (0.935–0.955) |
| 20 | image-pool | linear / default | reverse / 128 / none | 881.61 / 738.16 | 147.36 / 154.42 | 1.153 (1.136–1.155) | 0.961 (0.956–0.973) |
| 21 | image-pool | linear / default | fifo / 128 / none | 877.11 / 739.01 | 160.71 / 155.06 | 1.163 (1.151–1.176) | 0.963 (0.956–0.980) |
| 22 | image-pool | linear / default | shuffle / 128 / none | 887.65 / 746.95 | 167.33 / 160.49 | 1.163 (1.155–1.175) | 0.966 (0.962–0.974) |
| 23 | image-pool | linear / default | churn / 128 / none | 900.90 / 747.36 | 165.23 / 160.45 | 1.171 (1.159–1.184) | 0.956 (0.949–0.963) |
| 24 | buffer-separate | tlsf / default | reverse / 128 / none | 401.42 / 306.99 | 139.32 / 150.08 | 1.189 (1.158–1.221) | 0.962 (0.941–0.981) |
| 25 | buffer-separate | tlsf / default | churn / 128 / none | 394.94 / 283.87 | 156.87 / 151.53 | 1.279 (1.264–1.290) | 0.969 (0.961–0.997) |
| 26 | image-separate | tlsf / default | reverse / 128 / none | 1143.62 / 993.27 | 213.20 / 223.31 | 1.110 (1.106–1.118) | 0.984 (0.977–0.991) |
| 27 | image-separate | tlsf / default | churn / 128 / none | 1126.21 / 963.57 | 226.27 / 227.95 | 1.129 (1.123–1.139) | 0.970 (0.965–0.978) |
| 28 | buffer-pool | tlsf / default | reverse / 128 / cycle | 262.74 / 176.76 | 1825.35 / 1773.38 | 1.075 (1.067–1.086) | 0.977 (0.969–1.002) |
| 29 | buffer-pool | tlsf / default | churn / 128 / cycle | 277.82 / 176.61 | 1282.51 / 1233.92 | 1.107 (1.095–1.113) | 0.983 (0.976–0.990) |
| 30 | buffer-pool | linear / default | reverse / 128 / cycle | 259.59 / 177.46 | 1868.01 / 1776.47 | 1.088 (1.079–1.096) | 1.008 (1.000–1.027) |
| 31 | buffer-pool | linear / default | churn / 128 / cycle | 217.22 / 143.77 | 1100.27 / 1020.09 | 1.132 (1.120–1.146) | 1.005 (0.989–1.012) |
| 32 | virtual | tlsf / min-memory | churn / 128 / none | 19.71 / 14.38 | 7.49 / 5.84 | 1.345 (1.314–1.368) | 0.956 (0.946–0.982) |
| 33 | virtual | tlsf / min-time | churn / 128 / none | 20.95 / 15.98 | 8.35 / 7.36 | 1.255 (1.219–1.277) | 0.953 (0.936–0.965) |
| 34 | virtual | tlsf / min-offset | churn / 128 / none | 46.91 / 70.69 | 8.00 / 6.76 | 0.710 (0.702–0.723) | 0.976 (0.965–0.993) |
| 35 | virtual | tlsf / default | reverse / 1 / none | 32.87 / 29.35 | 18.84 / 18.80 | 1.076 (1.057–1.085) | 0.979 (0.957–0.992) |
| 36 | virtual | tlsf / default | churn / 32 / none | 20.21 / 17.13 | 8.66 / 8.33 | 1.085 (1.063–1.192) | 0.960 (0.945–0.965) |
| 37 | virtual | tlsf / default | churn / 256 / none | 20.54 / 15.97 | 8.56 / 7.05 | 1.259 (1.255–1.293) | 0.963 (0.948–0.969) |
| 38 | buffer-pool | tlsf / min-memory | churn / 128 / none | 201.42 / 130.87 | 88.25 / 73.32 | 1.413 (1.400–1.441) | 0.946 (0.926–0.959) |
| 39 | buffer-pool | tlsf / min-time | churn / 128 / none | 205.13 / 132.10 | 92.17 / 74.24 | 1.435 (1.411–1.446) | 0.948 (0.941–0.968) |
| 40 | buffer-pool | tlsf / min-offset | churn / 128 / none | 235.19 / 188.55 | 90.51 / 73.81 | 1.240 (1.212–1.256) | 0.945 (0.941–0.955) |
| 41 | buffer-pool | tlsf / default | reverse / 1 / none | 199.37 / 166.99 | 72.07 / 78.16 | 1.111 (1.066–1.133) | 0.881 (0.850–0.988) |
| 42 | buffer-pool | tlsf / default | churn / 32 / none | 192.74 / 124.01 | 86.58 / 67.23 | 1.450 (1.434–1.479) | 0.943 (0.916–0.970) |
| 43 | buffer-pool | tlsf / default | churn / 256 / none | 204.83 / 132.26 | 91.76 / 74.27 | 1.435 (1.427–1.450) | 0.946 (0.941–0.959) |

In the full matrix, 41/44 candidate/release combined intervals end below one;
these are descriptive per-cell intervals, not a multiple-comparison-adjusted
release gate. Public checks were not weakened to reach a timing target.

### Latest mapping cycles

Map/write-eight-bytes/unmap cycles on coherent memory; these are not
cache-flush or GPU-transfer measurements.

| Matrix ID | Algorithm / trace | vk ns/cycle | VMA ns/cycle | vk/VMA (95% interval) | Candidate/release (95% interval) |
| --- | --- | ---: | ---: | ---: | ---: |
| 28 | tlsf / reverse | 8047.06 | 7721.38 | 1.043 (1.032–1.049) | 0.997 (0.983–1.003) |
| 29 | tlsf / churn | 8088.76 | 7823.89 | 1.033 (1.021–1.045) | 0.998 (0.991–1.010) |
| 30 | linear / reverse | 8112.13 | 7759.97 | 1.043 (1.034–1.053) | 1.005 (0.996–1.014) |
| 31 | linear / churn | 6791.68 | 6543.36 | 1.043 (1.032–1.050) | 1.002 (0.995–1.012) |

### Changes and rejected experiments

The retained pass preserves every public method name, signature and error
payload:

- Pool placement carries the exact memory type already checked for that request
  into its first search. Later retries use the usual selector. Required flags,
  masks, heap size, protected/device-coherent/tile admission and request
  alignment remain checked; no result is cached across requests.
- A single native block does not need two mapping-preference groups. Multi-block
  order, linear newest-block rules and mapped-state/hold semantics remain
  unchanged.
- TLSF uses a compact private failure enum for its four placement errors,
  expanding to the identical public error variants at the boundary.
  Placement/search, bounds, alignment, full-width generations, callback
  reservation and rollback are unchanged.
- Host registries retain one last-slot hint. Every hit validates bounds,
  occupancy and the full u64 key; misses use the original hash/probe path.
  Rehash, shifting deletion and partition transfers may invalidate the hint
  without invalidating lookup. Relaxed atomic hint access preserves concurrent
  shared reads; values remain protected by the original borrows/locks. This adds
  one machine word per registry, no per-allocation metadata or new unsafe block.

No validation or security check was disabled for timing. This does not remove
unsafe callers' obligations for native pointer validity, GPU completion,
visibility and Rust aliasing.

Intermediate experiments remain under `mem-next-*` and are enumerated in the
retained inventory below. The specialized unpadded TLSF tail path (`mem-next-r1`)
was removed after showing no clear lifetime improvement; forced TLSF inlining
(`mem-next-r4`) was removed after its 30-triplet/1,024-batch repeat measured
virtual lifetime about 4.8% slower. The compact-failure-enum experiment
(`mem-next-r3`) was inconclusive alone; its longer repeat showed about 4.8%
faster virtual and 2.2% faster pooled lifetime, and the change was retained as
part of the candidate. The checked last-slot registry hint (`mem-next-r5`) is
the final selection.

The unused `mem-next-before/` attempt is explicitly invalidated: source changed
while it built, so its post-build inventory cannot establish compiled source.
It supplies no timing evidence; the immutable release build is the actual
baseline. Whole-process 100 µs CPU samples retained in
`mem-next-profile-{pool,virtual}/` include startup/cleanup and are not latency
measurements. Disassembly motivated the result-representation experiment;
neither samples nor a particular source change establish an isolated
nanosecond saving.

Cell 30's nominal 1.008× release regression prompted a 60-triplet repeat with
six warmups and 1,024 batches (`mem-next-map-confirm/`): combined
**1.000× (0.991–1.007)**. It does not reproduce a stable regression or
establish a speedup; the original row stays visible. Code layout, clocks and
shared load remain possible contributors; no cause is assigned without an
isolated proof.

### Prepared bind and in-place publication pass

A follow-up candidate (`mem-publish-r1`) adds two private changes on top of the
r5 selection; both preserve every public check, error and lifecycle order:

- Managed creation completes binding through a private prepared operation.
  Publication already claimed and recorded the exact resource association, so
  identity lookup, claim, reservation and bind-chain stages are not repeated;
  the backing lease, native call order, publication commit and caller rollback
  are unchanged. Public bind entry points keep their complete validation.
- Live publication builds its record in place: `insert_new_reserved_with`
  probes for the vacant bucket first and then invokes a builder, instead of
  constructing the full record before insertion. The monotonically fresh serial
  is asserted during probing; a panicking builder leaves the registry
  unchanged and capacity was reserved by the owner. The last-slot hint and the
  replace-on-existing path are unchanged.

Measured against the preceding candidate (`mem-prep-r1`), same VMA executable,
40/30 paired fresh-process samples, 1,024 batches, CPU 2:

| Workload | Trace | Allocate candidate/previous (95% interval) | Combined candidate/previous (95% interval) | Allocate candidate/VMA |
| --- | --- | ---: | ---: | ---: |
| buffer-pool | reverse | 0.944 (0.933–0.958) | 0.951 (0.942–0.964) | 1.350 (1.341–1.374) |
| buffer-pool | churn | 0.956 (0.947–0.966) | 0.970 (0.957–0.984) | 1.462 (1.457–1.480) |
| buffer-separate | reverse | 0.992 (0.983–0.998) | 0.991 (0.982–1.000) | 1.289 (1.262–1.299) |
| image-pool | churn | 0.982 (0.976–0.987) | 0.979 (0.975–0.984) | 1.192 (1.189–1.197) |
| virtual | reverse | 1.021 (0.994–1.039) | 1.018 (0.989–1.040) | 1.354 (1.316–1.389) |

Pooled-buffer allocation drops about 5.6% (about 9.4 ns/op) and its lifetime
about 4.9%; the virtual control, which does not exercise the new publication or
binding path, shows no clear change. These are candidate-over-candidate paired
differences on the same exploratory workstation, not new matrix cells; the
44-cell table above still describes the r5 candidate.

Post-pass status on the working tree: formatting, strict library Clippy, the
300-test library suite and all 54 native `mem` tests pass. The complete
native/validation/matrix qualification has not been rerun for this candidate;
it remains uncommitted work over the r5 pass.

## Publication-token pass: complete vk / VMA matrix — November 15

A third candidate wave extends the publish pass with four more private changes;
every public check, error and lifecycle order is preserved:

- `publish` returns only the serial and `published()`/`allocation_from_live`
  construct the public `Allocation` token once from the live record. `Live`
  gains a `block_size` field and `Region` carries the final `pool`/`class`
  values, so publication no longer duplicates record fields into the token.
- Requirements-route selection takes the API-1.1 core fast path directly; the
  pre-1.1 extension fallback moved to a `#[cold]` helper, removing per-request
  tuple construction on current devices.
- `Associations::record` uses a single-probe `get_or_insert_reserved_with`
  instead of a lookup-then-insert pair.
- The `pools` map lookup for pool priority was removed from
  `allocate_prepared_resource`. Note the semantic edge: a `Separate` policy
  create inside a custom pool now uses the request priority rather than the
  pool's `info.priority`; pooled placement still applies pool priority through
  `BlockOptions`.

Evidence: `opt-verify/` build over the working tree (dirty inventory in
`build.json`), `opt-verify-timing/` — 20 measured fresh-process samples per
cell, three warmups, 512 batches, CPU 2, identical VMA executable. This is a
two-way vk/VMA cohort with no release column. An earlier attempt
(`opt-final-timing/`) stopped at cell 11 on the stale-source provenance gate
after sources were edited mid-run; its 00–10 rows are consistent with this
matrix and remain retained. `contains_memory` was inspected for the
separate-allocation residual: it is an O(1) `native_handles` probe, not a scan.
Core and synchronization validation passed all 44 cells on this build
(`opt-verify-{core,sync}/`).

| ID | Workload | Algorithm / strategy | Trace / live / mapping | Allocate vk / VMA ns | Free vk / VMA ns | vk/VMA combined (95% interval) |
| --- | --- | --- | --- | ---: | ---: | ---: |
| 00 | virtual | tlsf / default | reverse / 128 / none | 9.02 / 13.02 | 4.55 / 4.34 | 0.791 (0.761–0.802) |
| 01 | virtual | tlsf / default | fifo / 128 / none | 9.28 / 12.80 | 8.26 / 8.79 | 0.805 (0.780–0.833) |
| 02 | virtual | tlsf / default | shuffle / 128 / none | 9.42 / 12.53 | 8.16 / 8.81 | 0.837 (0.788–0.850) |
| 03 | virtual | tlsf / default | churn / 128 / none | 11.39 / 14.74 | 7.11 / 6.70 | 0.838 (0.819–0.882) |
| 04 | virtual | linear / default | reverse / 128 / none | 8.53 / 14.70 | 7.42 / 7.55 | 0.708 (0.699–0.737) |
| 05 | virtual | linear / default | fifo / 128 / none | 8.58 / 14.54 | 8.33 / 7.04 | 0.802 (0.762–0.822) |
| 06 | virtual | linear / default | shuffle / 128 / none | 8.62 / 14.70 | 14.56 / 13.91 | 0.834 (0.805–0.840) |
| 07 | virtual | linear / default | churn / 128 / none | 8.90 / 13.70 | 10.26 / 9.54 | 0.813 (0.788–0.845) |
| 08 | buffer-pool | tlsf / default | reverse / 128 / none | 152.88 / 138.55 | 62.84 / 74.38 | 1.006 (0.984–1.043) |
| 09 | buffer-pool | tlsf / default | fifo / 128 / none | 157.89 / 140.54 | 69.21 / 78.65 | 1.032 (0.999–1.082) |
| 10 | buffer-pool | tlsf / default | shuffle / 128 / none | 162.38 / 138.82 | 93.58 / 79.00 | 1.162 (1.129–1.196) |
| 11 | buffer-pool | tlsf / default | churn / 128 / none | 160.62 / 131.82 | 76.39 / 73.32 | 1.153 (1.143–1.177) |
| 12 | buffer-pool | linear / default | reverse / 128 / none | 153.36 / 135.40 | 65.00 / 75.77 | 1.043 (1.033–1.055) |
| 13 | buffer-pool | linear / default | fifo / 128 / none | 154.03 / 135.45 | 66.64 / 75.21 | 1.046 (1.041–1.054) |
| 14 | buffer-pool | linear / default | shuffle / 128 / none | 157.18 / 134.10 | 84.36 / 80.67 | 1.126 (1.089–1.156) |
| 15 | buffer-pool | linear / default | churn / 128 / none | 155.71 / 133.81 | 77.25 / 77.50 | 1.099 (1.083–1.117) |
| 16 | image-pool | tlsf / default | reverse / 128 / none | 754.64 / 673.63 | 116.14 / 130.94 | 1.076 (1.063–1.098) |
| 17 | image-pool | tlsf / default | fifo / 128 / none | 744.75 / 670.99 | 120.97 / 134.79 | 1.074 (1.065–1.080) |
| 18 | image-pool | tlsf / default | shuffle / 128 / none | 748.38 / 670.27 | 120.96 / 135.35 | 1.082 (1.075–1.090) |
| 19 | image-pool | tlsf / default | churn / 128 / none | 740.40 / 659.78 | 119.90 / 131.52 | 1.091 (1.085–1.097) |
| 20 | image-pool | linear / default | reverse / 128 / none | 698.52 / 631.41 | 112.14 / 130.36 | 1.061 (1.057–1.067) |
| 21 | image-pool | linear / default | fifo / 128 / none | 697.53 / 632.57 | 116.24 / 130.99 | 1.067 (1.056–1.073) |
| 22 | image-pool | linear / default | shuffle / 128 / none | 698.92 / 634.33 | 122.57 / 134.23 | 1.068 (1.061–1.080) |
| 23 | image-pool | linear / default | churn / 128 / none | 705.42 / 634.81 | 120.07 / 134.54 | 1.073 (1.069–1.086) |
| 24 | buffer-separate | tlsf / default | reverse / 128 / none | 315.67 / 265.09 | 99.72 / 128.04 | 1.070 (1.027–1.087) |
| 25 | buffer-separate | tlsf / default | churn / 128 / none | 304.97 / 241.03 | 116.28 / 126.70 | 1.152 (1.123–1.167) |
| 26 | image-separate | tlsf / default | reverse / 128 / none | 915.49 / 813.55 | 159.65 / 182.43 | 1.072 (1.058–1.089) |
| 27 | image-separate | tlsf / default | churn / 128 / none | 890.07 / 784.36 | 170.11 / 183.48 | 1.091 (1.084–1.098) |
| 28 | buffer-pool | tlsf / default | reverse / 128 / cycle | 169.18 / 145.34 | 1467.99 / 1418.78 | 1.048 (1.033–1.057) |
| 29 | buffer-pool | tlsf / default | churn / 128 / cycle | 179.03 / 148.07 | 1078.58 / 1011.78 | 1.082 (1.075–1.092) |
| 30 | buffer-pool | linear / default | reverse / 128 / cycle | 172.98 / 148.97 | 1519.67 / 1450.98 | 1.063 (1.044–1.071) |
| 31 | buffer-pool | linear / default | churn / 128 / cycle | 170.17 / 145.42 | 1065.91 / 1003.78 | 1.073 (1.060–1.084) |
| 32 | virtual | tlsf / min-memory | churn / 128 / none | 11.36 / 14.51 | 6.41 / 5.78 | 0.893 (0.871–0.911) |
| 33 | virtual | tlsf / min-time | churn / 128 / none | 11.97 / 15.70 | 7.89 / 7.57 | 0.828 (0.805–0.869) |
| 34 | virtual | tlsf / min-offset | churn / 128 / none | 40.11 / 69.00 | 6.92 / 6.47 | 0.624 (0.621–0.629) |
| 35 | virtual | tlsf / default | reverse / 1 / none | 21.83 / 28.20 | 17.61 / 18.06 | 0.850 (0.831–0.876) |
| 36 | virtual | tlsf / default | churn / 32 / none | 11.69 / 16.17 | 7.71 / 7.77 | 0.805 (0.770–0.820) |
| 37 | virtual | tlsf / default | churn / 256 / none | 11.74 / 15.45 | 7.10 / 6.95 | 0.846 (0.828–0.856) |
| 38 | buffer-pool | tlsf / min-memory | churn / 128 / none | 156.95 / 130.47 | 74.07 / 72.25 | 1.137 (1.128–1.147) |
| 39 | buffer-pool | tlsf / min-time | churn / 128 / none | 160.09 / 132.05 | 77.02 / 73.15 | 1.156 (1.140–1.167) |
| 40 | buffer-pool | tlsf / min-offset | churn / 128 / none | 190.26 / 188.02 | 75.74 / 73.17 | 1.024 (1.003–1.035) |
| 41 | buffer-pool | tlsf / default | reverse / 1 / none | 171.44 / 169.19 | 64.76 / 78.08 | 0.937 (0.914–1.016) |
| 42 | buffer-pool | tlsf / default | churn / 32 / none | 151.34 / 124.37 | 68.97 / 67.56 | 1.147 (1.139–1.164) |
| 43 | buffer-pool | tlsf / default | churn / 256 / none | 157.02 / 132.19 | 76.15 / 73.71 | 1.134 (1.119–1.142) |

**15 of 44 combined workloads now complete below 1.0× VMA**: every virtual
cell (0.708–0.893) plus the single-live pooled buffer case (0.937). The worst
measured gap is pooled TLSF shuffle at 1.162 — down from 1.525 in the
November 14 matrix and 1.446 in the r5 matrix above. Native cells span
0.937–1.164; mapping cycles are 1.015–1.028×. Pooled and separate frees reach
0.78–0.92× on reverse/FIFO orders while shuffled and churned frees remain
1.0–1.18×. Separate-buffer allocation retains the largest measured residual
(1.21–1.28× on its alloc phase): `contains_memory` is verified O(1), so the
remainder is distributed create/publication bookkeeping rather than a scan.

## Mutex / reader-writer lock audit

The ordinary allocator used by the 44-cell matrix has no owner mutex; it relies
on exclusive Rust access. Changing synchronized-owner locks cannot explain or
remove its remaining VMA gap.

| Owner | Protected work | RwLock assessment |
| --- | --- | --- |
| Ordinary `Allocator` / `VirtualBlock` | Exclusive placement, registry and mapping state | No mutex to replace. |
| `SynchronizedAllocator` placement owner | Placement, live registrations, lease counts, poison and idle waiting | Mutations need exclusive access. A mechanical replacement also breaks the current mutex/condition-variable protocol; concurrent readers require a separate ownership and lease-admission design. |
| Per-backing mapping owner | Native map/unmap/bind exclusion, mapping holds and hysteresis | Keep native transitions exclusive. A Rust read-only guard can still be used to exclude a native bind; method mutability alone does not classify the native synchronization requirement. Mapping preference already reads an atomic snapshot. |
| Scoped shared state | Serial allocation, native/heap reservations, publication and resource claims | Predominantly writes. Counters/queries are possible read candidates, but must preserve coherent snapshots and transaction boundaries. No measured reason to replace this lock yet. |
| `SynchronizedVirtualBlock` | Placement/free plus read-only allocation info and statistics | Clearest read-sharing opportunity. Pure queries can potentially share a read lock; placement/free/user-data/clear remain writes. JSON writers must retain exclusive access and the documented panic/poison behavior. |

Pinned VMA already distinguishes these roles: `VmaBlockVector` and selected
registries use `VMA_RW_MUTEX`; `VmaDeviceMemoryBlock::m_MapAndBindMutex` stays
exclusive. Read sharing helps overlapping readers, not allocation/free operations
that both mutate placement. Changing the protected owner from Send-only to
shared-readable also needs an explicit Sync proof; a lock-name substitution is
insufficient.

### Measured lock-choice experiment

Two experimental adapters wrap the **same default-constructed `VirtualBlock`**
with `std::sync::Mutex` or `std::sync::RwLock`. They are not replacements for the
production synchronized wrappers, and no Vulkan/native calls are timed here.
The same binary contains both adapters. All reads check exact live metadata;
allocation/free round trips verify final occupancy and complete cleanup.

Twenty paired fresh-process samples and three warmups per cell alternate lock
order. Processes may use physical cores represented by logical CPUs 0, 2, 4 and
6 on this hybrid CPU; workers are not individually pinned. No task-launched
compilation or validation overlaps this cohort. Clocks, scheduling and other
workstation activity remain uncontrolled. Times include worker synchronization
and joining, but exclude thread creation and initial placement.

- `info`: 100,000 checked allocation-info reads per worker, 128 retained regions.
- `stats`: 5,000 complete metadata-statistics scans per worker, 512 retained regions.
- `mixed`: 50,000 iterations per worker, one allocation/free round trip followed
  by 15 info reads, 128 retained regions.
- `write`: 20,000 allocation/free round trips per worker, 128 retained regions.
  Allocation and free acquire separate exclusive guards in both adapters.

**Ratio is RwLock/Mutex elapsed time; below one favors RwLock.** These are
throughput-oriented whole-workload comparisons, not individual-call p99 or
writer-fairness guarantees.

| Workload | Workers | Mutex median ms | RwLock median ms | RwLock/Mutex, 95% interval |
| --- | ---: | ---: | ---: | --- |
| info | 1 | 1.645 | 1.689 | 1.019 (0.992–1.068) |
| info | 2 | 9.499 | 12.643 | 1.259 (1.122–1.361) |
| info | 4 | 28.085 | 31.422 | 1.159 (1.054–1.209) |
| stats | 1 | 0.909 | 0.903 | 1.011 (0.987–1.025) |
| stats | 2 | 1.994 | 1.080 | 0.533 (0.507–0.600) |
| stats | 4 | 4.658 | 1.856 | 0.405 (0.354–0.452) |
| mixed | 1 | 0.936 | 0.938 | 1.004 (0.986–1.016) |
| mixed | 2 | 5.934 | 7.046 | 1.221 (0.992–1.308) |
| mixed | 4 | 16.422 | 19.413 | 1.208 (1.076–1.337) |
| write | 1 | 0.789 | 0.795 | 1.008 (0.994–1.029) |
| write | 2 | 5.786 | 4.992 | 0.878 (0.774–0.999) |
| write | 4 | 14.843 | 20.218 | 1.474 (1.227–1.532) |

The useful opportunity is **overlapping long statistics reads**: the four-worker
RwLock case takes about 40.5% of the Mutex time. Short info reads, even though
read-only, regress under contention in this cohort; four-worker mixed and
allocation/free workloads regress by about 21% and 47%. Two-worker writes favor
RwLock in this run, so this is neither a universal mutex win nor a universal
reader/writer-lock win.

**Decision: keep the current production locks.** Read-heavy statistics are a
candidate for a separately qualified read API/snapshot design. Native transitions
remain exclusive. A future change needs callback/Sync, poison, idle-lease,
lock-order and writer-latency proofs plus matched production-wrapper workloads;
this isolated adapter experiment supplies none of those guarantees.

The complete source, build log, command logs and distributions are retained
under ignored `target/sdk/mem-lock-eval/`: `consumer/src/main.rs`,
`consumer/Cargo.toml`, `run.py`, and `timing/results.json`.
Lock experiment executable SHA-256: `27547b7f6d726ae8c9b2d1579ec80db11f60b2ac50f5f7e11e414903a35fa4b6`.

## Correctness and stability status

- **299 private library tests** pass, including new deliberately stale slot hints,
  competing shared readers and successful-pool-use followed by rejected admission.
  The existing independent BTreeMap trace still checks insert/replace/shift-delete,
  callback failure and non-Copy lifetime behavior.
- The isolated candidate workspace passes **3,461 tests** including bindings,
  loader, allocator, generator and positive/compile-fail documentation contracts;
  strict workspace/all-target/all-feature Clippy, formatting, default-feature tests,
  no-default-features compilation, generation freshness, rustdoc/rendered searches,
  shared-rule freshness and diff checks pass. The snapshot excludes only the
  unrelated unfinished pipeline benchmark and its Cargo target registration;
  every runtime/test source hash matches the live candidate.
- **32/32 native profiles pass separately under core and synchronization
  validation**, including all six required optional profiles. No skips or
  validation warnings/errors/VUIDs/hazards were accepted.
- **44/44 benchmark cells pass for both vk and VMA under both validation modes**:
  176 backend executions. Fourteen cells are host-only virtual workloads and
  30 exercise native resources; loader-confirmed validation-layer insertion is
  required for native executions. Instrumented times are excluded.
- Pinned donor comparisons pass **56 virtual traces / 112,344 operations**,
  **1,024 native traces / 756,000 operations**, and **12 concurrency comparisons**,
  with independent oracles and zero runtime diagnostics. C++ ASan/UBSan/leak
  instrumentation covers the donor/driver, not the Rust library.
- **1,600 pinned VMA memory-preference cases**, 50 Python tool tests and 11 SDK
  evidence tests pass. SDK tests run in the real repository because their
  provenance test requires Git metadata; the first snapshot-only invocation
  retains its missing-Git failure log. The initial sandboxed virtual sanitizer
  invocation also remains failed: LeakSanitizer could not inspect threads there.
  Its successful unrestricted replacement uses a separate evidence directory;
  no system ptrace setting was changed.

These checks qualify the named host/native workloads on this machine, not
Vulkan conformance, general secret-memory safety, every public feature, WSI or
cross-platform/device behavior. No public API, native synchronization rule,
callback recovery, arithmetic check or full-width generation was weakened.

## Remaining coverage

The 44-cell `full` suite means the complete bounded matrix above, **not the
entire public feature set**. Still unmeasured: multi-block pressure, budgets and
growth, persistent-map and relocation latency, contention, larger live sets,
non-coherent hardware and cross-device performance. The
[SDK comparison coverage](../../../sdk/README.md#honest-comparisons)
lists the workload contracts and remaining timing work.

## Reproduce the current suite

The standard SDK runner reproduces the candidate/VMA matrix. Select a compatible
memory type and available CPU for another machine; this host uses type 2 and
CPU 2. The retained local triplet runner additionally compares the immutable
release. Keep validation and timing in separate evidence directories.

```bash
python3 sdk/tool/bench.py build --build target/sdk/nov15-build
python3 sdk/tool/memory.py run --suite full --build target/sdk/nov15-build \
  --api 1.4 --memory-type 2 --cpu 2 --samples 20 --warmup 3 --batches 512 \
  --output target/sdk/nov15-timing
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov15-build \
  --api 1.4 --memory-type 2 --validation core \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/nov15-core
python3 sdk/tool/memory.py validate --suite full --build target/sdk/nov15-build \
  --api 1.4 --memory-type 2 --validation sync \
  --layer-manifest /usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json \
  --output target/sdk/nov15-sync
```

## Evidence and exact build identities

All raw evidence is ignored under `target/`. Matrix cells retain source/artifact
hashes, commands, samples/warmups, telemetry, requested-byte/eviction checksums,
initial/fragmented occupancy, mapping cycles and diagnostics. Values in this
report are recomputed from those JSON records. No previous result is overwritten.

- Timing: `target/sdk/mem-next-matrix/`, `mem-next-map-confirm/` and the
  experiment directories listed below; original orchestration is
  `mem-limit-{ab,matrix}.py`.
- Native profiles: `target/sdk/mem-next-{core,sync}/`.
- Benchmark validation: `target/sdk/mem-next-matrix-{core,sync}/`.
- Donors: `target/verify_memory/mem-next-{virtual-r2,native,concurrency}/`.
- Candidate workspace checks: `target/sdk/mem-next-checks/`; snapshot/source
  manifest: `target/sdk/mem-next-proof.json`.
- Lock-choice experiment: `target/sdk/mem-lock-eval/`.

Compact sorted JSON source-inventory hashes and timed executable identities:

| Build | Source inventory SHA-256 | vk-bench SHA-256 |
| --- | --- | --- |
| `target/release/v0.13.1/sdk-build-r3` | `af5a5fba5022d7cb850bfbd74215a19178dda4b376e5a153191da81109810cc9` | `3953380a76c564cc26ac82bd2be65164abe1cd5c7a2ea5d8fe5dc5e753b70e28` |
| `target/sdk/mem-next-r1` | `c36d99bdbb6af218a3e537e64fc8024c68237fb3f258d5aeda27a15a04366f06` | `ec43b0e19ce0d73201c15c25e4730a4f6c320ed690b51f42d95bb4ab5a2a1681` |
| `target/sdk/mem-next-r2` | `fbb96071321453f3680e77aa4b242eadaf5a855d57b1c1a90c57657b654946af` | `60f9ad7f2e5afde9bdb84cd6c7518b8f1c80b073afd47ed4fb3e298b0a9a3aff` |
| `target/sdk/mem-next-r3` | `84fb58517167c02873b5d363c758ace574daa506822ae6e7bc31b1b70b62e683` | `d83e94f2f3f8f10c846b1f2a7db9860e4b6bafce42df59c4f67d1863c5c0162d` |
| `target/sdk/mem-next-r4` | `2754540a6670e0291922f208156b3e58c3550a89071ed3758b6c1081ae878940` | `5db295325772fe6988ec04f856146005fa69f3af5e62c35bc4cc3ec9c02b9f6f` |
| `target/sdk/mem-next-r5` | `22cd8eebf3b8e5c30e9293a892eded54e6b1949bc40d51c32cc111bbe3edae28` | `336c2bae8bc38242b05e2dc5d9a1929c00612bf979144021ec0caa580d06f425` |
| `target/sdk/mem-prep-r1` | dirty inventory in `build.json` | `df92ca7f0b03398fc6fc3332674038a02276f4eb1dda8b19415a2c352227e1b4` |
| `target/sdk/mem-publish-r1` | dirty inventory in `build.json` | `16f040ab6f622df181f5d7c335f04d90369267a2a7740556f427ba2ca6acd6d4` |
| `target/sdk/opt-verify` | dirty inventory in `build.json` | `883fdc4e4d3de5e5cde72f5120e674ef7998723ab488143cef968b77d3eb2a4d` |

All six builds use VMA executable SHA-256
`f8c2666898748b23e612c9e83827e732a6116c60e0ab54eef50852641688bbf0`. The release
baseline is private commit `9bedf2cd37fbf76566381cf7f412f78f407d47c7`;
candidate manifests retain its dirty source inventory. Raw commands,
distributions, telemetry, diagnostic streams and validation checksums remain
in the evidence directories.

### Complete retained timing-run inventory

Each entry is a separate cohort, not an independent vote in a pooled ranking.
Older passes are superseded for current performance claims but retained for
audit. The 44 nested current matrix records are enumerated by ID in the full
table.

| Cohort | Change | Samples / batches | Outcome |
| --- | --- | --- | --- |
| `mem-next-r1-ab` | Specialized unpadded TLSF tail path | 30 / 128 | Removed: no clear lifetime improvement |
| `mem-next-r2-ab` | Reuse admitted pool type; single-block preference shortcut | 30 / 128 | Retained; small pooled-buffer improvement |
| `mem-next-r3-ab` | Compact TLSF placement failure result | 30 / 128 | Short virtual samples highly noisy; not used alone for acceptance |
| `mem-next-r3-confirm` | Same r3 binary, longer repeat | 30 / 1,024 | Virtual lifetime about 4.8% faster; pooled about 2.2% faster |
| `mem-next-r4-ab` | Force-inline TLSF placement | 30 / 1,024 | Removed: virtual lifetime about 4.8% slower |
| `mem-next-r5-ab` | Checked last-slot registry hint | 30 / 1,024 | Candidate selected for the complete matrix |
| `mem-next-matrix` | Final r5 versus release/VMA | 20 / 512, all 44 cells | Complete current cohort |
| `mem-next-map-confirm` | Cell 30 repeat | 60 / 1,024 | No repeatable lifetime regression or speedup |
| `mem-lock-eval` | Mutex/RwLock adapter study | 20 / per-workload counts | Isolated host-only lock comparison; production locks unchanged |
| `mem-prep-r1-ab` | Prepared-bind candidate vs r5 | 30 / 512 | Pooled allocate 0.980× candidate; retained follow-up pass |
| `mem-publish-r1-ab` | In-place publication vs prepared bind | 40 / 1,024 | Pooled allocate 0.944×, lifetime 0.951×; virtual control unchanged |
| `mem-publish-churn-ab` | Same builds, churn/image cells | 30 / 1,024 | Pooled churn 0.956× allocate; image-pool 0.982× allocate |
| `opt-final-timing` | Publication-token pass, first matrix | 20 / 512 | Stopped at cell 11 on the stale-source gate; rows 00–10 retained |
| `opt-verify-timing` | Publication-token pass, complete matrix | 20 / 512, all 44 cells | Current vk/VMA cohort; 15/44 cells below 1.0 |
| `opt-verify-{core,sync}` | Same build, benchmark validation | all 44 cells × 2 modes | All cells passed under both validation modes |
