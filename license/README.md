# Licenses

Original vk code uses the project [MIT license](../LICENSE). Upstream license
texts live together here; [NOTICE.md](../NOTICE.md) identifies the derived
material and copyright owners.

| Material | Pinned source | License text |
| --- | --- | --- |
| Vulkan and StdVideo registries/declarations | Vulkan-Headers `acbb9391c0a597272800cd3f4363661c00a27c77` | [MIT](khronos/MIT.txt), alternative [Apache-2.0](khronos/Apache-2.0.txt) |
| Allocator donor algorithms | VMA 3.4.0 `3aa921224c154a0d2c43912bc88e1c42ce1f7607` | [AMD MIT](vma/MIT.txt) |

vk selects the Khronos MIT option. The unchanged upstream MIT text has its own
copyright year range; the pinned XML copyright declarations and notices remain
authoritative for those inputs. These texts are included in the published crate.
Registry provenance remains in [SOURCES.md](../registry/SOURCES.md).

SDK-only comparison dependencies keep their own upstream licenses in Cargo's
registry cache or the hash-verified donor cache. They are not runtime dependencies
and are not included in the vkrust crate. No tutorial or Ash implementation is
copied into the SDK examples.
