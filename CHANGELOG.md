# Changelog

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
