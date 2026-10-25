# Registry Sources

## Vulkan-Headers snapshot

| Field | Value |
|---|---|
| Repository | https://github.com/KhronosGroup/Vulkan-Headers |
| Commit | `acbb9391c0a597272800cd3f4363661c00a27c77` |
| Date | 2026-10-14 |
| Branch | `main` |

## Vendored files

| File | SHA-256 |
|---|---|
| `vk.xml` | `4ab953836756807a93cee72bc3c63554826b445af43003e0d7d7e876818428ca` |
| `video.xml` | `d018b914014c06605e367a3b929670511e6f6de2f225c405a8b5e2d912408b76` |

Both files taken from the same commit. Do **not** mix XMLs from different revisions.

## Upstream license

The Khronos Vulkan-Headers registry is distributed under Apache-2.0 OR MIT, as declared in each pinned XML.
Copyright notice from upstream:

```
vk.xml: Copyright 2015-2026 The Khronos Group Inc.
video.xml: Copyright 2021-2026 The Khronos Group Inc.

SPDX-License-Identifier: Apache-2.0 OR MIT
```

This project selects the MIT option for derived material. Project MIT terms
are preserved in `LICENSE`; the upstream Apache option is retained separately
in `license/khronos/Apache-2.0.txt`. See `NOTICE.md` for Khronos copyright notices.
`ABI_SHA256SUMS` separately pins the headers used by the development ABI oracle.

## Regeneration

To reproduce the pinned files from source:

```bash
git clone --filter=blob:none https://github.com/KhronosGroup/Vulkan-Headers.git vendor/Vulkan-Headers
cd vendor/Vulkan-Headers
git checkout acbb9391c0a597272800cd3f4363661c00a27c77
cp registry/vk.xml ../../registry/vk.xml
cp registry/video.xml ../../registry/video.xml
cd ../..
sha256sum -c registry/SHA256SUMS
```

Verify the hashes match `SHA256SUMS` before use.
