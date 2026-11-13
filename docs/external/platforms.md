# Platforms and standard-library scope

The crate generates the pinned Vulkan ABI independently of an installed SDK.
Compiling a platform declaration does not establish runtime loader, window-system,
driver or external-memory support on that platform.

| Target | Loader policy | Build/runtime obligations |
| --- | --- | --- |
| Linux | `libvulkan.so.1` | Install the intended ICD and validation layers; select it explicitly for qualification. Enable matching `platform-xlib`, `platform-xcb` or `platform-wayland` declarations for WSI. |
| Windows | `vulkan-1.dll` | Driver/loader installation, native Windows ABI and matching `platform-win32` declarations. Host tests do not qualify Win32 memory import or presentation. |
| macOS | `libvulkan.dylib`, or explicit `Entry::new_from_path` | Supply a Vulkan implementation such as MoltenVK and its loader. Query/enable portability enumeration and portability-subset extensions when required; they are not enabled automatically. Metal declarations use `platform-metal`. |
| Android | `libvulkan.so` | Cross-build/link with the Android NDK and Rust target standard library, package native libraries correctly, and supply a compatible device/API level. `platform-android` exposes native surface declarations. |
| iOS | `libvulkan.dylib`, custom path or caller-owned resolver | Supply a compatible implementation, platform linking/signing and permitted loading route. Default dynamic loading does not promise an accepted iOS distribution configuration. |

Existing release CI has Windows/macOS host checks and Linux software Vulkan.
The optional SDK workflow adds Android, Linux ARM64, Windows GNU and iOS ARM64
**compilation checks**. These configured checks are not execution evidence until
their actual runs pass. Final cross-linking additionally needs the platform SDK,
linker and deployment configuration. ABI/provisional/platform-specific C oracle
qualification and physical cross-platform execution remain separate work.

No platform is automatically granted Vulkan features based on its OS or GPU name.
Applications request an API version, query support, enable required extensions/
feature records, and manage resource completion. On portability implementations,
supported features and limits can differ from desktop drivers.

## What uses `std` today?

`std` means Rust's standard library. It includes platform services and
allocator-backed collections and re-exports portable primitives.
vk currently requires it even with `default-features = false`:

| Area | Current uses | A possible `no_std` route |
| --- | --- | --- |
| Generated ABI and borrowed construction | `std::ffi`, `fmt`, `ops`, `ptr`, `mem`, `marker`, `result`, error traits | Emit corresponding `core` paths from the generator; update handwritten construction and independent generation checks together. |
| Dispatch wrappers and capability metadata | Enumeration `Vec`s, strings, sets, shared library ownership, errors | `alloc` for storage, `core` for primitives, an explicit caller-owned resolver. Dynamic loading stays separately gated. |
| Dynamic loader | `libloading`, OS paths, error strings, `Arc<Library>` | Keep `std` for OS loading; provide no automatic loader substitute in a raw profile. |
| Allocator | CPU allocation/callbacks, storage, locks/condition variables, thread/panic handling, platform handles and I/O diagnostics | Separate host allocation from synchronization/OS services; gate services until an explicit replacement is reviewed. |

There is **no `no_std` feature implemented yet**. A future design can default
to `std`, make `loader` imply `std`, and first qualify a real
`#![no_std]` raw-binding/borrowed-construction profile. An `alloc` profile for
owned wrappers follows only with explicit resolver/allocator/synchronization
contracts. Existing flags must not silently claim freestanding support.

This does not require another repository that re-exports `alloc::vec::Vec`.
It also does not inherently make code faster or eliminate heap allocation.
`core::ptr::copy_nonoverlapping` has an unsafe non-overlap/extent contract and
may lower to the same LLVM copy intrinsic as a standard-library path. OA's
specialized copy policies need workload evidence before use here.

Rust's [Android targets support `std`](https://doc.rust-lang.org/rustc/platform-support/android.html).
The [iOS target](https://doc.rust-lang.org/rustc/platform-support/apple-ios.html)
also supports ordinary Rust platform builds. Mobile deployment and `no_std`
are different requirements. [Ash enables `std` by default](https://docs.rs/ash/latest/ash/)
and offers an `alloc`-requiring profile without it; its dynamically loaded
entry route requires `std`.

## Stability gate

This release claims no stable designation. A stable designation requires reviewed
API/semver commitments, release/package/ABI evidence, positive and negative
capability tests, clean validation, native relocation/concurrency/interop
qualification and representative multi-driver coverage. Benchmark results must
identify the workload and environment; one GPU or faster median cannot establish
full VMA equivalence or production readiness. See [qualification](qualification.md).
