# Changelog

## 0.9.0+1.4.365 — 2026-10-31

- Add owned device retention to the allocator: `new_retained`,
  `new_retained_with_granularity` and `new_retained_from_physical_device`
  produce `Allocator<'static>` through a `DeviceRetention` reference-or-Arc
  device, letting an allocator outlive its construction scope and finish on a
  dedicated cleanup thread. `SynchronizedAllocator::new_owned` adopts such an
  allocator directly. The borrowed constructors keep their contracts.
- See the [release notes](https://github.com/realminc/vk/blob/main/docs/external/release/v0.9.0.md).

## 0.8.0+1.4.365 — 2026-10-29

- Add direct synchronized allocator operations and `SynchronizedVirtualBlock`.
  Native map/unmap/bind leases use stable per-backing locks, allowing placement
  and free on independent backings in the same vector. Retain ownership through
  publication/failure, exclude pending resource claims and preserve poison recovery.
- Add pinned-VMA concurrent comparison and packaged public API proofs. Preserve
  the measured hardware/platform limits and deliberate placement policies.
- Refresh the README cover/Realm footer and developer links, add the crate
  homepage, and consolidate internal verification reports into one allocator
  summary. See the [release notes](https://github.com/realminc/vk/blob/main/docs/external/release/v0.8.0.md).

## 0.7.0+1.4.365 — 2026-10-29

- Rename the allocator module and mirrored tests to `mem`. Use `vk::mem` or a
  caller-local `use vk::mem as vma`; principal root type re-exports are unchanged.
  Allocator free operations and methods retain one canonical implementation,
  separate from the root Vulkan operations. Update archive audits, donor fixture
  paths and the public software-Vulkan CI target for the new layout. Initialize
  lazy test-fixture mutexes before arming host allocation failure injection on
  Darwin; the callback-only metadata checks remain enforced.
- Expand the Experimental allocator across mixed buffer/image storage, automatic
  and custom TLSF/linear pools, mapping/cache services, budgets, callbacks,
  diagnostics, external imports/exports and explicit multi-pass relocation.
  Full VMA equivalence, unrestricted mixed-phase concurrency, broader platform
  execution and comparative performance remain unqualified. See the
  [release notes](https://github.com/realminc/vk/blob/main/docs/external/release/v0.7.0.md)
  for consolidated scope, evidence and migration.

The following entries record implementation checkpoints; qualification applies
only to the profiles described in the release notes.


- Extend borrowed allocator concurrency to default memory types and distinct
  backing blocks within one pool. Preserve map holds, block order, aggregate
  reservations and cached Win32 export roots; route partition metadata in one
  pass. Add real Linux DMA-BUF buffer and opaque-FD linear-image import profiles.
  Broad platform execution, unrestricted mixed-phase concurrency and comparative
  VMA performance remain unqualified.

- Add borrowed concurrent custom-pool scopes to SynchronizedAllocator with
  aggregate in-flight native reservations, lossless mapping/token restoration,
  callback-owned metadata and poison-preserving recovery. Add real Linux opaque
  file-descriptor import/export and concurrent-pool runtime profiles; broader
  qualification remains open.

- Add explicit platform import backing-size/type accounting, resource-specific
  no-query imports, Metal texture native-zero accounting and correct host-pointer
  dedication policy. Add opt-in per-allocator serialized threading with scoped
  unsafe access, poison refusal and owner-preserving recovery. Broader
  platform/driver execution and qualification remain open.

- Route native allocator registries, names, block/placement storage and service/
  relocation scratch through borrowed CPU allocation/free callbacks. Preserve
  fallible reservation and explicit Vulkan cleanup, remove heap token identities
  and keep caller-owned result/writer allocations separate. Broader donor,
  platform, hardware, concurrency and performance qualification remain open.

- Add borrowed CPU allocation/free callbacks for virtual TLSF/linear blocks,
  including growth and diagnostic scratch storage. Preserve fallible placement,
  host-only cleanup and callback lifetimes; replace virtual heap identities with
  checked non-reusing tokens. Native CPU metadata routing and broad qualification
  remain open.

- Add unqualified compatible-vector incremental free-space ordering, mapped-block
  preferences from host intent, reverse MinTime search and trailing-spare release.
  Keep linear newest-block policy, exact domain isolation, mapping holds and
  explicit native cleanup. Consolidated VMA qualification remains open.

- Add cooperative relocation stop predicates, distinct interrupted-pass status,
  panic rollback and raw/alias-group/mapped allocation moves. Preserve explicit
  and persistent map counts, alias permissions and guard metadata; keep ordinary
  free's mapped-allocation rejection. These additions remain unqualified.

- Add unqualified Extensive relocation contexts with persistent evacuation,
  resource-class packing and cleanup phases. Retain evacuated blocks through
  the context, release surplus empty blocks explicitly at finish, preserve pool
  minimums and allow trim recovery after host-only context Drop. Keep broad
  qualification and remaining relocation/platform/metadata features open.

- Add unqualified Fast/Balanced/Full relocation policies, per-domain fullness
  ordering retained across context passes, donor average-gap copy heuristics,
  explicit destination placement and ignored-block destination-only tracking.
  Replace the unreleased `within_block` switch with `algorithm`; default to
  Balanced. Consolidated qualification remains open.

- Implement another unqualified allocator batch: borrowed request/pool allocation
  chains, separate native-chain resource creation, ordinary external imports,
  per-type exports and FD/Win32 handle services, automatic default pools and
  mapping hysteresis, persistent relocation contexts and raw disjoint-plane
  binding. Preserve explicit Vulkan cleanup and import handle-transfer semantics.
  Full validation and remaining platform/metadata/relocation work are pending.

- Checkpoint an unqualified native allocator service batch: mapping reference
  counts, batch/cache/copy services, aliasing controls, native callbacks,
  metadata/statistics/JSON, cached budgets, priority/protected/coherent feature
  admission, debug fill/guards and explicit relocation passes. Comprehensive
  tests, donor/package/GPU qualification and remaining VMA features are pending.

- Add resource-aware memory-type queries with exact core/KHR maintenance4 and
  temporary-resource fallback, raw separate/pooled allocation with conservative
  granularity and checked binding, and explicit minimum allocation alignment.
  Reuse managed creation validation, policy, placement, mapping and rollback.
  `AllocationCreateInfo` gains `min_allocation_alignment` (default zero).

- Add allocation and checked binding for existing buffers/images, with shared
  managed-resource requirements/dedication/map paths, explicit memory policy,
  conservative unknown-image granularity and retained caller objects on failure.
  Validate raw separate bindings and reject repeated or mismatched library binds.
  Preserve resource-before-storage rollback and guard duplicate native outputs.

- Add explicit allocator `trim` / `trim_with_budget` operations with released
  native block/byte counts, heap-specific pressure and pool minimum preservation.
  Follow VMA's local-pressure and partial-free empty-spare cleanup without
  hot-path native budget queries, host metadata allocation or live relocation.

- Pack buffers and linear/optimal images into shared TLSF/linear native blocks
  with checked buffer/image granularity pages, independent non-coherent atoms
  and explicit failure rollback. Query the device limit or supply it through
  `Allocator::new_with_granularity`; manual construction without it stays
  conservative. Keep padding free and correct a reproduced pinned-VMA
  upper-stack page conflict with an independent regression proof.

- Add pinned-VMA linear placement to native custom pools and virtual blocks:
  explicit individual/free-at-once and multi-block stack lifetimes, single-block
  ring wrap and downward double-stack allocation. Preserve checked generations,
  atom padding, mapping, accounting and failure rollback. Reject unsupported
  upper-address routes before native creation; no native destruction in Drop.

- Add VMA-style automatic custom pools with `block_size = 0`: heap-derived
  preferred sizes, bounded initial growth/shrinking, reuse across block sizes
  and pool-owned dedicated fallback. Keep fixed-type compatibility, required
  dedication, explicit destruction and full pool accounting on every route.

- Admit managed buffer device address from captured enabled core/KHR features.
  Follow pinned VMA allocation flags for dedicated/raw storage and every pooled
  block, including custom-pool minimums. Preserve dedication and native failure
  rollback; capture/replay and the incompatible EXT path remain excluded.

- Add native fixed-type TLSF custom pools based on pinned VMA block-vector
  semantics: explicit block sizes, transactional minimums, maximum block counts,
  extra alignment, empty-block retention, checked statistics and explicit
  destruction. Reuse existing placement, mapping and memory accounting.
  Policy records borrow their selected pool; stored types may need a lifetime
  annotation.

- Add checked live `AllocationInfo` snapshots with native binding metadata,
  actual selected memory properties and allocation-relative mapping pointers.
  Free and method forms share the same implementation; snapshots neither map
  storage nor retain its lifetime.
- Add native allocator policy profiles for storage, persistent upload,
  readback, separate device/host buffers, ordinary images and raw
  video-session-style requirements. Extend the pinned VMA replay and actual
  packaged-consumer proofs to use the live snapshot API.

## 0.6.0+1.4.365 — 2026-10-26

- Complete the pinned record-construction inventory, including unions, checked
  byte payloads, SPIR-V words, optional parallel arrays, initialized writable
  storage, direct/pointer-table alternatives, native-width video counts and
  reviewed parent/cross-record extents. Generate typed layer-setting values and
  explicit unsafe setters for opaque addresses and foreign objects.
- Add an exhaustive generated field audit, independent paired-XML coverage and
  ordering checks, more borrow-checker fixtures and extracted-crate consumers.
  All 1,552 concrete records/unions and 8,333 fields are covered, with 5,450
  construction methods. Newly borrowed/exclusive records change Rust source
  compatibility; native ABI stays fixed.

## 0.5.0+1.4.365 — 2026-10-26

- Generate borrowed record setters, checked native counts and typed extension
  chains without adding a separate builder or execution layer. Propagate
  lifetimes through records, aliases and PFNs; preserve native header field
  names and generated item spacing. Add independent borrow-checker fixtures,
  public construction contracts and checked parallel/descriptor recipes.
- This is a Rust source compatibility change: stored borrowed types need
  lifetime annotations, exhaustive literals need a marker/default update, and
  exclusive chain records are non-Copy/non-Clone. Native C ABI and command
  coverage remain independently verified boundaries; GPU lifetimes stay unsafe.

## 0.4.1+1.4.365 — 2026-10-25

- Build the VMA preference oracle from hash-verified core/video headers only,
  with an error-only umbrella sentinel to prevent a local SDK from hiding
  unpinned include dependencies. Add regression coverage for staging and hashes.

- Identify the pinned Vulkan headers in Cargo build metadata (`+1.4.365`),
  independently of the library release tag and runtime API request. Verify the
  suffix against pinned XML and generated header constants before publication.

- Add generator-owned Khronos references and native-name rustdoc search aliases
  for Vulkan declarations, commands, constants and record members. Share the
  branded README with the crate introduction, compile its calling-style
  examples, and check the rendered links/search index. Expand crate discovery
  metadata for graphics, GPU compute, game development and AI/ML use cases.

- Extend the native Rust allocator with managed images, isolated resource-class
  pools, admitted video/format-list creation chains, VMA-derived automatic memory
  preferences, bounded adaptive block retries, local heap limits and explicit
  native/local budget snapshots. Native lifetime management remains explicit.

- Flatten Vulkan and StdVideo binding source into `src/rs`, with mirrored flat
  binding tests. Use `vk::` directly; the `vk::core` module path is removed.
  `load` and `memory` remain public subsystem modules. Regenerate all binding
  artifacts through the updated generator and run the `bindings` test target.

## 0.3.0 — 2026-10-25

First public development release. Experimental; not Vulkan conformance or VMA parity.

- License original code and the published crate under MIT while preserving
  Khronos provenance and AMD MIT attribution.
- Add sanitized, version-aligned public snapshots and public-only CI/release
  gates, real archive/test/consumer verification, docs.rs metadata and exact
  artifact checks for optional crates.io publication. Keep the generator as
  development-only tooling and retain Rust import namespace `vk`.
- Declare Rust 1.98.1 and preserve explicit native lifetime/cleanup contracts.

- Add a development-only pinned native VMA comparison over independent synthetic
  Vulkan drivers, public Rust allocator calls and an interval/lifecycle model.
  Check mapping, atom isolation, cache ranges, accounting, compatible-type OOM
  fallback and failure rollback; retain explicit policy and qualification limits.

- Add opt-in ordinary pooled buffers, atom-isolated TLSF regions, shared explicit
  and persistent block mappings, allocation-relative cache ranges and local
  region/reservation/native accounting. Keep default buffer storage separate.
- Retry only recoverable native allocation OOM across compatible memory types;
  preserve dedication, limits and transactional buffer/placement/map rollback.
  Exclude tile heaps; retain explicit native cleanup and no native Drop.

- Port VMA virtual TLSF placement to native Rust with checked arena identities,
  four donor search strategies, aligned split/coalesce, clear and detailed statistics.
- Add canonical virtual-block free operations and delegated methods under `memory`;
  keep virtual operations independent of native memory and GPU lifetimes.
- Compare public Rust placements/offsets/accounting with a pinned executable VMA
  oracle and independent interval models. Widen the first-level bitmap and test
  the pinned donor's large-range exhaustion counterexample separately.

- Query/enforce maintenance3 allocation-size limits through admitted core/KHR
  properties2 routes; retain explicit unknown limits for legacy/raw metadata.
- Inject actual host bookkeeping OOM in unit tests before native acquisition;
  verify retained ownership and successful retry for empty/full tracking tables.
- Add a native Vulkan 1.0 KHR memory profile covering dedicated requirements,
  matching allocation chains, queried size admission and compute/readback.

- Harden separate buffer allocation with capability-admitted core/KHR
  requirements2 and matching dedicated allocation chains; preserve legacy 1.0.
  Managed creation now requires device creation provenance on imported handles.
- Add queried allocator construction, checked managed-buffer destruction,
  explicit close and canonical `memory` free functions with delegated methods.
- Reserve live bookkeeping before native acquisition; test rollback, ownership,
  missing PFNs, mapping state and native failure with independent PFN traces.
- Pin development-only VMA 3.4.0 donor inputs and package its MIT attribution.
  Broader native policy and full VMA qualification remain open.

## 0.2.3 — 2026-10-21

Experimental Git release. Vendor spelling corrections are source changes;
see the release note for migration and qualification limits.

- Complete all 842 pinned command spellings, including Vulkan 1.3/1.4,
  extension and protected platform free-function/method operations. Preserve
  explicit native pointer/count/output contracts for direct forwarding.
- Test every new operation through loaded exact-ABI mocks; add independent
  XML/Rust and pinned-header C prototype checks and complex output-storage tests.
  Native execution above Vulkan 1.2 remains unqualified.
- Keep vendor command suffixes atomic (`arm`, `amd`, `amdx`, `nvx` and all
  registered tags) without an acronym allowlist.

- Complete the ten remaining Vulkan 1.2 commands and thirteen registry aliases;
  require all 178 core spellings through 1.2 and 42 aliases in the XML coverage gate.
- Preserve native 64-bit address returns without status conversion; test exact
  ABI forwarding, errors, callbacks and independent alias PFNs.
- Add an explicit 1.2 host-query-reset and two-subpass render-pass2 native profile.

- Complete Vulkan 1.1 command wrappers and all 26 promoted KHR spellings, with
  independent pinned-XML coverage through 1.1.
- Preserve caller-initialized output arrays/chains and native counts/statuses;
  expose descriptor-template payloads under their native unsafe pointer contract.
- Require all six 1.1 feature records/twelve bits in the external feature oracle;
  add explicit 1.1 feature/query and descriptor-template/base-dispatch profiles.

## 0.2.2 — 2026-10-20

- Complete Vulkan 1.0 native operation coverage with 58 additional paired wrappers,
  including graphics/render-pass commands, queries, byte slices and bounded
  enumerations. Add an independent pinned-XML coverage gate.
- Record deferred NVIDIA CUDA Rust safety research and the Khronos parity gates.

- Capture owned typed enabled-feature records from legacy and pNext inputs;
  provide explicit requirement admission and bounded chain validation.
- Add metadata-bearing instance/device imports with one-time dispatch filtering.
- Add 30 paired timeline semaphore, synchronization2 and copy2 command wrappers,
  preserving independent KHR spellings, native status codes and checked counts.

- Capture owned creation version/extension provenance and generate exact command
  admission from core providers, extension scope and conditional dependencies.
  Filter cached dispatch once; keep aliases independent and raw imports explicit.
- Harden instance/device creation against null outputs and initialization failure.
  Preflight device resolvers, roll back through available destruction routes and
  preserve unrecoverable native handles with their library for explicit recovery.
- Bound enumeration retries and recheck void enumerations for truncated results.
  Add fault-injection/lifetime tests and an independent XML capability oracle.

## 0.2.1 — 2026-10-20

- Unify Vulkan and StdVideo ABI exports under `core` and the crate root; keep
  codec ABI tests in the core target.
- Preserve pinned XML declaration order across generated families and API tests.
- Generate 54 additional paired native operations with checked slices, retained
  query chains, native status codes and caller-visible partial pipeline outputs.
  Add registered `test_command.gen.rs` API contracts and handwritten failure mocks;
  exercise the compute/readback workload entirely through public operations.

- Standardize Rust wrapper construction on `new` / `new_from_*`, including
  entry loading, native-handle imports and generator-owned dispatch tables.
  Native Vulkan operation names and plain `from_raw` conversions are unchanged.
- Specify the native Rust, VMA-derived memory design and staged qualification
  gates without adding unimplemented allocator APIs.
- Mirror runtime and generator test ownership under `test/rs`, with no inline
  test bodies in source files. Register explicit module Cargo targets.
- Disable incremental compilation and debug information for dev/test builds.

## 0.2.0 — 2026-10-20

Experimental, source-breaking release. See the tagged release evidence in Git history.

- Adopt the `vk` library namespace and adjacent source modules;
  place generated families in singular `*.gen.rs` files.
- Standardize snake_case fields and distinguish raw instance/device handles.
- Share cached-PFN command bodies between free functions and methods.
- Correct dispatch ancestry, library retention, enumeration retry, protection
  propagation, native allocation and mapped-range checks.
- Add an independent pinned-header ABI oracle and a real compute/readback proof.
- Replace unsupported milestone-completion claims with scoped qualification.
