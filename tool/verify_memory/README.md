# Allocator donor inputs

The runtime is native Rust. This development directory owns reproducible VMA
inputs for algorithm review and executable virtual/native allocator oracles;
Cargo never invokes it or builds C++.

```bash
python3 tool/verify_memory/fetch_donor.py
python3 tool/verify_memory/fetch_donor.py --check
```

`fetch_donor.py` owns the exact VMA 3.4.0 commit and SHA-256 inventory. Inputs
are cached under ignored `target/verify_memory/donor/<commit>/`. A corrupt or
modified cache fails verification. No local source archive is substituted for
the pin. The upstream license is also shipped as `LICENSE-VMA-MIT`.

The first native port follows `VmaAllocator_T::GetBufferMemoryRequirements`
and `VmaAllocator_T::AllocateDedicatedMemory`: core 1.1 or explicitly enabled
KHR dedication uses requirements2 and the matching buffer dedication chain;
legacy 1.0 uses its exact legacy query. Default buffers get separate storage.
Opt-in homogeneous buffer blocks use checked native policy, shared mapping and
atom isolation; required/preferred dedication retains separate storage.
Rust-specific changes are explicit errors, immutable dispatch admission,
fallible bookkeeping reservation, checked identities and explicit cleanup.

The native baseline uses independent PFN traces in
`test/rs/memory/test_lifecycle.rs`. Virtual placement additionally uses the actual
pinned VMA implementation through `oracle.cpp`, a public-API Rust trace consumer
in `trace.rs`, and an independent sorted interval model in `verify_virtual.py`.
Consumer Cargo builds never invoke this harness or compile C++.

```bash
python3 tool/verify_memory/verify_virtual.py --headers <pinned-header-include-dir>
python3 tool/verify_memory/verify_virtual.py --headers <pinned-header-include-dir> --sanitize
```

`--package <extracted-crate>` selects an actual packaged consumer; `--output`
selects an ignored evidence directory and `--steps` controls operations per trace.
The default corpus has 32 traces, four donor strategies and 64,064 operations.
VMA and header inputs are hash-verified before compilation. Every ordinary
placement, failure and statistic must match exactly and independently satisfy
the interval model. Sanitizer mode compiles the donor with ASan/UBSan, forces
halt-on-error/leak checks and rejects replay stderr as well as nonzero exits.
Python optimization mode is rejected so validation assertions cannot disappear.

Large-domain behavior is qualified separately: the port widens the donor's u32
first-level map to u64. A checked replay shows VMA reporting exhaustion for a
feasible class above bit 31 while Rust reuses offset zero; the independent model
validates the corrected placement. This is a deliberate correctness adaptation,
not a speed claim or blanket VMA parity result. Ordinary parity traces stay
within the donor's working bitmap domain. All full traces and both outputs are
retained before comparison; failures identify a replay step.

The native oracle runs the actual pinned VMA allocator through a synthetic
Vulkan driver and the public Rust allocator through a separately written driver.
It performs no GPU or loader calls and introduces no runtime dependency.

```bash
python3 tool/verify_memory/verify_native.py --headers <pinned-header-include-dir> --sanitize
python3 -m unittest discover -s test/py/tool -v
```

`--package`, `--output` and `--steps` have the same meanings as in the virtual
oracle. The default native corpus has 96 traces: three atom sizes, coherent and
non-coherent types, separate and fixed-size pooled storage, all four strategies
and two seeds. It injects native host/device OOM, create/map/bind failure, checks
clean retry, shared/persistent mapping, cache translation, required flags, masks,
preferred memory types and separate-storage fallback. An independent model
checks every native effect, binding, non-overlapping region/atom, mapping hold,
resource lifetime and allocator statistic. Poisoned-output tests prove those
checks reject corrupt traces. Full input/stdout/stderr/exit evidence is saved
before validation, including failed replay diagnostics. Optimized Python is
rejected. ASan/UBSan instruments the C++ VMA oracle and synthetic driver only.

The comparison uses Vulkan 1.0 requirements and an explicit 64 KiB VMA pool fixed
to type zero, with mapping hysteresis disabled. Separate storage uses VMA's
dedicated policy. Exact outcomes, selected types and separate native allocation
attempts must match. Native pooled size classes/offsets and empty-block caching
differ by documented policy; each trace must independently satisfy the model.
VMA may implicitly unmap dedicated memory on free; Rust explicitly releases its
hold. No timing, automatic VMA policy, pooled cross-type fallback, image or
real-driver parity claim follows from these traces.

Image granularity and coverage-guided fuzzing remain planned. Atom isolation
and shared mapping also have mock/native tests in
`test/rs/memory/test_pooled.rs` and `test_allocator.rs`;
non-coherent physical hardware remains unqualified. See the
[public qualification and limitations](../../docs/external/qualification.md).

Resource-aware preferences are independently checked against executable pinned
VMA `FindMemoryPreferences`. `verify_policy.py --headers <include-dir>` checks the
1,600-case generated fixture used by ordinary Rust unit tests; `--refresh` is the
explicit donor-vector update path. This oracle is development-only. Cargo consumers
need neither the C++ compiler nor the donor. Managed image/failure/chain tests live
in `test/rs/memory/test_image.rs`; gated real clear/copy/readback tests live in
`test/rs/memory/test_image_native.rs`. Image blocks are partitioned by resource
class; existing buffer differential traces do not qualify mixed-class packing.

The preference oracle includes the pinned `vulkan_core.h` directly and tells
VMA those declarations are already present, matching the other donor oracles.
Its verifier stages only hash-verified headers and injects an error-only
`vulkan/vulkan.h` sentinel, so an installed or cached SDK cannot conceal an
accidental dependency on an unpinned umbrella header.
