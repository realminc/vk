# Qualification and limitations

Status: Experimental. Publication makes the implementation available; it does
not establish Vulkan conformance, production readiness or performance parity.

## Binding coverage

The pinned paired Khronos registries provide 842 selected command spellings,
including all 234 core commands through Vulkan 1.4 and their 100 registry
aliases. All selected spellings have callable operations. This is coverage of
that particular snapshot, not a promise of every future registry revision.
[Registry provenance](../../registry/SOURCES.md) identifies the inputs.

Types, constants, StdVideo records, nullable PFNs and immutable per-context
pointer tables are generated. Free functions and methods delegate to the same
operation. The default loader uses the OS Vulkan library; custom resolvers are
also supported. No Vulkan SDK, generator or C/C++ compiler is needed by a
library consumer. The library currently uses Rust `std`, including with
`--no-default-features`. Rust 1.98.1 is the declared toolchain baseline.

Operations are unsafe and require Vulkan valid usage, correct lifetimes,
synchronization and enabled features. Native cleanup is explicit. Dropping a
wrapper does not destroy native objects or wait for work. Field names are
snake_case while ABI layout and calling conventions retain the native contract.
Version 0.5.0 adds generated borrowed setters and typed
`push_next` construction on the same records. Their lifetimes cover backing host
storage, not native resource ownership or GPU completion. Public pointer fields,
opaque payloads and manually edited pointer/length relationships retain unsafe
caller obligations. Resource ownership and GPU-completion lifetimes remain separate work.

Version 0.6.0 extends construction across every pinned record/union
field: checked byte and element extents, optional parallel arrays, initialized
writable outputs, reviewed parent/cross-record capacities, typed layer settings
and explicit unsafe opaque/external pointer setters. All 1,552 concrete records/
unions and 8,333 fields have construction coverage, with 5,450 methods. New
lifetime parameters and exclusive records change Rust source compatibility.
Output capacities and returned sizes remain distinct, and native calls stay unsafe.

## Evidence and continuous checks

Host gates cover deterministic generation, XML command inventories, loaded
exact-signature forwarding mocks, capability metadata, feature capture, bounded
enumeration and failure recovery. Independent pinned-header C/Rust oracles
compare layouts, bitfields and available prototypes. Protected-platform and
provisional C signatures remain separately unqualified.

The public CI configuration adds Linux host checks, Windows/macOS host tests,
an extracted-crate consumer and explicit Lavapipe compute/load/mem tests.
These are configured gates until an actual workflow run succeeds. Software
Vulkan execution is correctness evidence for that profile, not physical GPU,
presentation, video, ray-tracing or performance qualification.

Recorded physical execution is limited to x86-64 Linux, Intel HD Graphics 4400
(Haswell), Vulkan 1.2 and Mesa. Physical Vulkan 1.3/1.4 execution, other drivers
and architectures, WSI, positive codec/ray-tracing workloads and broader external
interop remain open. The scoped Linux memory-import profile is described below. Feature promotion/equivalence semantics are also incomplete;
pointer availability alone does not establish command valid usage.

## Integrated allocator

The opt-in `allocator` feature exposes `vk::mem` and principal root types such
as `vk::Allocator` and `vk::Allocation`. `use vk::mem as vma` is an optional
caller-local alias. Free allocator operations and methods share canonical
implementations; native Vulkan operations remain at the root. For example,
`vk::create_buffer` creates an unbound native buffer, while
`vk::mem::create_buffer_with_options` creates, allocates and binds it.

The broad VMA-derived service surface is implemented in Rust. It remains
Experimental; service presence is distinct from full donor equivalence.

| Area | Implemented scope |
| --- | --- |
| Resources and placement | Managed ordinary buffers/non-disjoint images, existing-resource allocation/binding, separate and automatic storage, mixed buffer/image granularity packing, checked atom isolation, custom TLSF/linear pools, aliasing and host-only virtual placement. |
| Policy and limits | Explicit and usage-based memory selection, bounded block growth/shrinking, dedication, alignment, local limits, native/cached budgets, opt-in within-budget admission and explicit trimming. |
| Mapping and services | Shared/counted and persistent maps, allocation-relative cache operations, batch allocate/free/cache, host copies, names/user data, detailed statistics/JSON, debug guards/fill and native/CPU callbacks. |
| Optional capabilities | Captured feature admission for core/KHR buffer device address, priority, protected and coherent memory; borrowed allocation chains, export configuration, explicit platform import accounting, FD/Win32 services and raw disjoint-plane bind2. |
| Relocation | Explicit bounded multi-pass Fast/Balanced/Full/Extensive contexts, copy plans, ignored/pinned state, stop predicates, mapped/raw/alias-group moves and explicit commit/abort. Consumer copies, completion, resource replacement and descriptor/pointer updates remain explicit. |
| Host concurrency | Opt-in `SynchronizedAllocator` with direct `&self` operations, dynamic per-backing map/unmap/bind leases and borrowed concurrent custom-pool, default-memory-type and distinct-backing-block phases; separately synchronized host-only `SynchronizedVirtualBlock`. Ordinary `Allocator` remains non-Send/non-Sync. |

`AllocationPolicy::Separate` remains the default. Select `Automatic` explicitly
for heap-sized default pools and native-growth fallback; custom pools retain
their own policy and identities. This is a deliberate Rust API choice, not an
assertion of VMA's default behavior.

Construction retains borrowed device/callback/chain dependencies; the optional
`new_retained` constructors instead hold an owned `Arc<Device>` clone, producing
`Allocator<'static>` that may finish on a dedicated cleanup thread. Native object
lifetimes, callback user-data validity, GPU completion and external ownership
transfers remain unsafe caller obligations. No operation implicitly submits or
waits for GPU work. Native destruction uses explicit cleanup calls; `Drop` frees
CPU metadata only. Explicit `close` requires completed
cleanup, including empty custom-pool destruction. Mapping/cache operations do
not establish GPU synchronization. Never infer pixel layout from an optimal
image's mapped bytes.

### Recorded evidence

The pre-release allocator checkpoint passed 1,024 native-model donor traces
(756,000 operations), 56 virtual traces (112,344 operations) and 1,600 memory
preference cases against pinned VMA 3.4.0. These use synthetic native PFNs and
independent interval/page/accounting models, not physical GPUs. Clang C++
ASan/UBSan emitted no runtime diagnostics. Individual independently checked
upper-stack granularity, linear count and large-domain donor discrepancies stay
scoped regressions; donor agreement alone is insufficient. Mapping hysteresis
is explicitly disabled in both native trace consumers; enabled caching has
separate host tests.

The allocator checkpoint passed 3,438 workspace tests/doctests, including 50
generator cases; the final runtime and extracted-crate reruns each passed 3,388.
Rust AddressSanitizer passed 332 library/memory cases with leak detection, and
ThreadSanitizer passed 22 threading plus 46 memory integration cases with a
matching instrumented standard library. Both reported zero diagnostics. Four
fresh-process repetitions of three independently checked VMA concurrency outcomes
also passed (12 comparisons). This is scoped evidence, not universal race freedom
or Miri qualification. The v0.9.0 owned-retention delta adds native-ledger
scope/lifetime/thread tests but no new sanitizer, donor or hardware profile.
See the [v0.9.0 release notes](https://github.com/realminc/vk/blob/main/docs/external/release/v0.9.0.md)
and actual CI artifacts for the release snapshot's rerun results.

Recorded physical memory profiles use Intel HD Graphics 4400 (Haswell), Mesa,
Vulkan 1.2, x86-64 Linux. Separate core/thread-safety and
synchronization/thread-safety runs cover serialized work, concurrent custom
pools, default-memory-type scopes and independent backing blocks. Each of the
first three profiles completes 512 buffer lifecycles per validation mode;
the block profile uses four backings and 128 cycles. Only one host-visible type
is available on that device; cross-type overlap is proved by mocks, not hardware.

Real Linux OPAQUE_FD and DMA-BUF buffer round trips and an OPAQUE_FD linear-image
round trip also pass both validation modes, checking exact type/size accounting
and bidirectional mapped payloads. Buffer profiles are host-only; both image
aliases transition UNDEFINED to GENERAL through explicit submission before host
pixel writes. These profiles are neither cross-process nor cross-API GPU
ownership/semaphore-handoff qualification. Windows cached-handle branches are
cross-compiled; native Windows execution remains open.

### Differences and remaining qualification

- Direct map/unmap/bind work releases the placement mutex under a live native
  lease and a stable per-backing mutex. Placement/free on other backings in the
  same vector can proceed. Placement stays serialized per owner; pool/type scopes
  provide independent placement owners. Global services, relocation and borrowed
  access wait for leases. This is not lock-free allocation or a contention claim.
  Poisoned owners retain native ownership for explicit consuming recovery.
- Full donor policy/service equivalence and concurrent stress qualification
  remain open. Rust retains granularity padding as free space and does not add a fallible
  driver budget query after resource destruction during free. These placement and budget policies differ from the donor; equivalent
  API availability does not establish identical VMA behavior.
- Defragmentation's full native copy/data-correctness workloads, external
  GPU ownership handoff and platform-specific import execution beyond the
  recorded Linux profiles need broader qualification.
- Native Windows, Android, Metal, Fuchsia, QNX and OHOS execution; non-coherent,
  granularity-greater-than-one, positive device-address and device-group hardware
  profiles remain unqualified. Import property queries and platform resource
  creation remain caller-owned. Managed disjoint construction and tile heaps
  are not admitted.
- Comparative latency, throughput, fragmentation, metadata cost and contention
  versus VMA are unmeasured. No performance superiority or production-readiness
  claim follows from removing the C++ bridge or from Rust syntax.

Leave `allocator` disabled when using another memory manager. Native Vulkan
requirements continue to apply to either route.

## Distribution

The crates.io package is `vkrust`; its library/import name is `vk`:

```toml
[dependencies]
vk = { package = "vkrust", version = "0.8" }
```

The public source target is `realminc/vk`. Private engineering history and
agent/editor configuration are excluded from sanitized snapshots. Generated
source is committed and packaged; consumers do not regenerate it. The generator
is development tooling and is not published as a second crate.

Rustdoc on docs.rs is the initial API reference. A future documentation site
can add Vulkan and OA routes independently of the crate publication workflow.
