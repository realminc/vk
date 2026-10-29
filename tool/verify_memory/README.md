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
Opt-in mixed-resource blocks use checked native policy, shared mapping and
atom isolation; required/preferred dedication retains separate storage.
Rust-specific changes are explicit errors, immutable dispatch admission,
fallible bookkeeping reservation, checked identities and explicit cleanup.

The native baseline uses independent PFN traces in
`test/rs/mem/test_lifecycle.rs`. Virtual placement additionally uses the actual
pinned VMA implementation through `oracle.cpp`, a public-API Rust trace consumer
in `trace.rs`, and an independent sorted interval model in `verify_virtual.py`.
Consumer Cargo builds never invoke this harness or compile C++.

```bash
python3 tool/verify_memory/verify_virtual.py --headers <pinned-header-include-dir>
python3 tool/verify_memory/verify_virtual.py --headers <pinned-header-include-dir> --sanitize
```

`--package <extracted-crate>` selects an actual packaged consumer; `--output`
selects an ignored evidence directory and `--steps` controls operations per trace.
The default corpus has 56 traces and 112,344 operations: 32 TLSF strategy
traces plus 24 linear stack/ring/double-stack traces.
VMA and header inputs are hash-verified before compilation. Every ordinary
placement, failure and statistic must match exactly and independently satisfy
the interval model and, for linear placement, an independent chronological-live
envelope model. The model has no paired vectors or tombstone counters. Sanitizer mode compiles the donor with ASan/UBSan, forces
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
oracle. The default native corpus has 1,024 traces and 756,000 operations: the original
480 traces include 96 fixed-size TLSF,
96 automatic TLSF and 96 single-block linear Rust/VMA custom-pool traces, with
240 device-address traces: three
atom sizes, coherent and non-coherent types, separate/fixed/automatic pooled
storage, all four strategies and two seeds. The explicit custom-pool
mode adds a fixed type-zero 64 KiB pool, a one-block ceiling and 256-byte extra
alignment, with explicit cache release at pool destruction. It injects native
host/device OOM, create/map/bind failure, checks
clean retry, shared/persistent mapping, cache translation, required flags, masks,
preferred memory types and separate-storage fallback. An independent model
checks every native effect, binding, non-overlapping region/atom, mapping hold,
resource lifetime and allocator statistic. Poisoned-output tests prove those
checks reject corrupt traces. Full input/stdout/stderr/exit evidence is saved
before validation, including failed replay diagnostics. Optimized Python is
rejected. ASan/UBSan instruments the C++ VMA oracle and synthetic driver only.

The optional fifth header field selects default (`0`), explicit (`1`) or
automatic (`2`) TLSF or fixed-size linear (`3`) custom pools; a sixth field
selects KHR buffer-device-address enablement with its complete captured dependency
and feature metadata. Address-mode traces alternate ordinary and address buffers.
Each native allocation then reports its flags, independently checked against
VMA's contract: every pooled block and each separate address buffer has exactly
`DEVICE_ADDRESS_BIT`, while a separate ordinary buffer has no flag. Poisoned
flag outputs are rejected even when both allocators would agree on placement.
This synthetic profile does not establish physical address-query/shader support.

Automatic-pool traces use a 512 KiB synthetic heap: VMA's preferred size is
64 KiB and pooled blocks range from 8 to 64 KiB, with a two-block ceiling and
256-byte extra alignment. Two large allocations exercise pool-owned dedicated
storage beyond that ceiling and both ordinary/address allocation flags.
The independent model checks permitted native sizes, the pooled-block ceiling,
mandatory properties, absence of orphaned dedicated storage and full cleanup.
Both public pool-statistics APIs must agree with each driver's independently
tracked allocation and native-block counts/bytes. Variable placements and growth
attempts are policy-specific; exact native block sequences are not compared.

The comparison uses Vulkan 1.0 requirements and fixed-type pools, with mapping
hysteresis explicitly disabled in both drivers. The independent mapping model
requires native maps to equal explicit/persistent holds; enabled map-cache behavior
has separate host and concurrency tests. The Rust consumer drops its explicitly
closed allocator's CPU metadata before destroying the borrowed device.
Separate storage uses VMA's
dedicated policy. Exact outcomes, selected types and separate native allocation
attempts must match. Native pooled size classes/offsets and empty-block caching
differ by documented policy; each trace must independently satisfy the model.
VMA may implicitly unmap dedicated memory on free; Rust explicitly releases its
hold. Default Rust blocks release empty storage immediately; selected custom pools
retain their minimum/one empty spare until explicit destruction. No timing,
complete automatic VMA policy, pooled cross-type fallback, all image configurations or real-driver parity
claim follows from these traces.

Mixed buffer/image page separation has the additional corpus below;
coverage-guided fuzzing remains planned. Atom isolation
and shared mapping also have mock/native tests in
`test/rs/mem/test_pooled.rs` and `test_allocator.rs`;
non-coherent physical hardware remains unqualified. See the
[public qualification and limitations](../../docs/external/qualification.md).

Resource-aware preferences are independently checked against executable pinned
VMA `FindMemoryPreferences`. `verify_policy.py --headers <include-dir>` checks the
1,600-case generated fixture used by ordinary Rust unit tests; `--refresh` is the
explicit donor-vector update path. This oracle is development-only. Cargo consumers
need neither the C++ compiler nor the donor. Managed image/failure/chain tests live
in `test/rs/mem/test_image.rs`; gated real clear/copy/readback tests live in
`test/rs/mem/test_image_native.rs`. Mixed custom-pool TLSF and linear-upper
image/buffer profiles require shared native memory and exact page separation;
core/synchronization validation are distinct from synthetic donor comparisons.

The preference oracle includes the pinned `vulkan_core.h` directly and tells
VMA those declarations are already present, matching the other donor oracles.
Its verifier stages only hash-verified headers and injects an error-only
`vulkan/vulkan.h` sentinel, so an installed or cached SDK cannot conceal an
accidental dependency on an unpinned umbrella header.

Linear native traces fill a 64 KiB block, retire/reuse its FIFO prefixes through
two full ring wraps, exercise coexisting lower/upper stacks and arbitrary middle
release, and inject native acquisition/create/map/bind failures before clean retry.
An optional tenth allocation field requests upper placement. These linear
cases use fixed bounded batches; native `--steps` scales the older seeded cases. The resource/atom,
mapping, cache, BDA and local accounting checks remain unchanged.

The pinned donor has a quick-statistics bug: `VmaBlockMetadata_Linear::AddStatistics`
starts reading its second ring vector at the first vector's null-prefix count.
`vmaGetPoolStatistics` therefore undercounts that corpus's wrapped prefix;
`vmaCalculatePoolStatistics` and total detailed statistics remain correct.
The unchanged pinned donor is used directly. `e linear_pool_count` records both
counts at every discrepancy, and the independent model requires the exact FIFO
ring deviation plus correct detailed/live counts. Other pool modes still require
quick/detailed equality. Poisoned counts and deviations outside active rings fail.
Rust's public pool counts always match its independently live allocations.

The virtual donor consumer uses a non-null, never-dereferenced user-data sentinel:
linear detailed statistics uses that field to identify live entries. Rust has no
such implicit user-data requirement. These are scoped donor corrections, not
unqualified VMA equivalence or a performance claim. Native multi-block linear,
automatic linear and additional image paths have separate mock/native hardware
gates rather than being implied by the fixed single-block differential corpus.

Mixed-resource native comparison adds 320 traces and 188,800 operations across
five granularity limits (1/64/256/512/4096), two atom sizes, coherent/non-coherent
types, TLSF/linear pools, four strategies and two seeds. The optional seventh
header field is `bufferImageGranularity`; an optional eleventh allocation field
selects buffer (`0`), linear image (`1`) or optimal image (`2`). Exact image PFNs
feed actual native VMA and Rust managed-image paths through independent synthetic
drivers. Resource creation, mapping and binding failures, arbitrary free ordering,
replacement holes and mixed FIFO ring wraps retain independent lifetime, mapping,
cache, accounting and all-pairs page checks. Ordinary native offsets remain
policy-specific. For pinned TLSF limits <=256, VMA rounds optimal-image extents;
its model records that actual size while Rust retains requirement/atom extents
and free neighbor padding. Poisoned page sharing and out-of-scope size rounding
must fail independently of allocator agreement.

An additional `upper-granularity-regression.trace` is deliberately separate from
ordinary comparison. With a 64 KiB linear pool, 512-byte granularity and 256-byte
minimum alignment, pinned VMA places an upper 128-byte buffer at 65280, then a
256-byte optimal image at 65024. Both occupy page 127. The independent checker
must reject exactly the second bind's granularity conflict. Rust instead places
the image at 64768 (page 126), completes the trace and passes the full model.
The donor/header hashes stay unchanged; sanitizer diagnostics are never allowed.
This is an explicit correctness correction, not relaxed ordinary parity or a
claim about other VMA versions. `--steps` does not scale fixed mixed batches.

Empty-cache policy comparison adds 32 fixed-pool traces across TLSF/linear,
coherent/non-coherent types, two atom sizes and minimum counts zero/one. The
optional eighth header field supplies a local heap ceiling (zero leaves it
unrestricted); ninth/tenth fields specify minimum/maximum pool blocks. Rust
uses its local ceiling and VMA uses `pHeapSizeLimit`. Exact-ceiling cases check
last-spare eviction and minimum retention; two-block cases check partial-free
cleanup of the last empty block. The independent model rejects retained pressured
spares, released minimums, ceiling violations and missed qualified partial cleanup.
VMA's 80% fallback-budget heuristic differs from Rust's explicit local ceiling;
these bounded cases deliberately qualify the shared outcomes at the exact ceiling,
not general threshold equivalence. Explicit Rust maintenance has no invented VMA
API counterpart: its query/failure/identity/maps/floor and per-heap selection are
verified through `test_trim.rs`, actual native profiles and extracted consumers.

Existing-resource replay adds 96 cases: 80 mixed TLSF/linear cases across five
granularity limits plus 16 buffer separate/default-pool/BDA cases. An optional
twelfth allocation field selects caller creation followed by real
`vmaAllocateMemoryForBuffer/Image` and `vmaBindBuffer/ImageMemory`, or their public
Rust equivalents. Each consumer owns and destroys its resource on failure only
after asserting allocation/binding retained that handle. No failed native output
is treated as an owned object. Existing-image tiling is unknown to the allocator,
so the independent all-pairs model requires page separation from every class,
including itself. Native TLSF small-page rounding includes this donor class.
Separate existing buffers conservatively carry BDA allocation flags for unknown
usage when enabled. Poisoned self-sharing and missing conservative flags fail
independently of donor agreement. No advanced binding offsets/chains, aliasing,
external allocation or inferred resource-usage support follows from this corpus.

Raw requests use the twelfth allocation field `2` (field `1` retains existing-
resource allocation). Both consumers query ordinary native requirements and call
actual `vmaAllocateMemory` / `Allocator::allocate_with_options`, then bind and
explicitly destroy/free. The extra 96 traces / 58,072 operations cover default
and custom TLSF/linear pools, coherent/non-coherent atoms, granularity through
4096, ordinary raw separate storage, shared/persistent maps and BDA flags.
Unknown resource classes conflict with every native class, even other raw buffers.
The independent model checks that row and rejects poisoned same-page placement
and missing raw-address flags. These are synthetic driver checks, not physical
non-coherent or positive BDA qualification.
