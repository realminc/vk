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
opaque payloads and unreviewed pointer/length relationships retain unsafe caller
obligations. Resource ownership and GPU-completion lifetimes remain separate work.

## Evidence and continuous checks

Host gates cover deterministic generation, XML command inventories, loaded
exact-signature forwarding mocks, capability metadata, feature capture, bounded
enumeration and failure recovery. Independent pinned-header C/Rust oracles
compare layouts, bitfields and available prototypes. Protected-platform and
provisional C signatures remain separately unqualified.

The public CI configuration adds Linux host checks, Windows/macOS host tests,
an extracted-crate consumer and explicit Lavapipe compute/load/memory tests.
These are configured gates until an actual workflow run succeeds. Software
Vulkan execution is correctness evidence for that profile, not physical GPU,
presentation, video, ray-tracing or performance qualification.

Recorded physical execution is limited to x86-64 Linux, Intel HD Graphics 4400
(Haswell), Vulkan 1.2 and Mesa. Physical Vulkan 1.3/1.4 execution, other drivers
and architectures, WSI, positive codec/ray-tracing workloads and external
interop remain open. Feature promotion/equivalence semantics are also incomplete;
pointer availability alone does not establish command valid usage.

## Integrated memory

The opt-in `allocator` feature supplies managed buffers and non-disjoint images,
separate and class-partitioned pooled storage, bounded block shrinking, automatic
memory preferences, explicit budget snapshots/local limits, shared mapping, checked
allocation-relative cache ranges and host-only virtual TLSF placement.
Native objects still require explicit release after exact completion.

Independent failure injection, interval models and pinned VMA differential
oracles exercise selected policies. This is not VMA-equivalent memory
management. Mixed-class block packing, disjoint/external/protected image memory,
custom pools, defragmentation, concurrency and full cross-device qualification remain
open. No allocator speed advantage is claimed. Leave `allocator` disabled when
using a separately qualified memory manager.

## Distribution

The crates.io package is `vkrust`; its library/import name is `vk`:

```toml
[dependencies]
vk = { package = "vkrust", version = "0.5" }
```

The public source target is `realminc/vk`. Private engineering history and
agent/editor configuration are excluded from sanitized snapshots. Generated
source is committed and packaged; consumers do not regenerate it. The generator
is development tooling and is not published as a second crate.

Rustdoc on docs.rs is the initial API reference. A future documentation site
can add Vulkan and OA routes independently of the crate publication workflow.
