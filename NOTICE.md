# Third-party notices

vk's original code and published crate are licensed under MIT. The following upstream
notices and licenses also apply to the derived material.

## Khronos registries

The generated Vulkan and StdVideo declarations derive from the paired
Vulkan-Headers registry snapshot recorded in [registry/SOURCES.md](registry/SOURCES.md).

Copyright 2015-2026 The Khronos Group Inc. (vk.xml).
Copyright 2021-2026 The Khronos Group Inc. (video.xml).

Upstream offers Apache-2.0 OR MIT; this project uses the MIT option.
The upstream declaration is preserved in the pinned inputs.
Project MIT terms are in LICENSE. The upstream Apache option is retained
separately in license/khronos/Apache-2.0.txt; it is not the project license.

## Vulkan Memory Allocator

The native Rust memory implementation adapts algorithms from Vulkan Memory
Allocator 3.4.0, commit `3aa921224c154a0d2c43912bc88e1c42ce1f7607`.
Source comments identify individual donor algorithms and Rust adaptations.

Copyright (c) 2017-2026 Advanced Micro Devices, Inc. All rights reserved.
The complete upstream MIT notice is preserved in LICENSE-VMA-MIT.

The runtime does not link VMA or compile its C++ implementation. Development
oracles fetch and verify the pinned donor separately. This attribution does
not imply AMD or Khronos endorsement or full VMA equivalence.
