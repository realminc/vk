#!/usr/bin/env python3
"""Fetch and hash-verify the development-only VMA donor; never used by Cargo."""

import argparse
import hashlib
from pathlib import Path
import urllib.request

COMMIT = "3aa921224c154a0d2c43912bc88e1c42ce1f7607"
FILES = {
    "include/vk_mem_alloc.h": "8487b7995ad3b263eb73bc5b9a77d71aa69b6bef5d58a715c02d2663afd81f1a",
    "LICENSE.txt": "52df2c03d6cfc9ffec13c9d3626c530fc9ce0cbe41d5ea3d10cd46edeb1aeb38",
    "src/Tests.cpp": "14fe46bf02147d97709ec4297d4f73be09c52683ae88611463452ebdbb240721",
}
ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "target/verify_memory/donor" / COMMIT)
    parser.add_argument("--check", action="store_true", help="verify existing inputs without network or writes")
    args = parser.parse_args()
    # Validate every input before publishing any downloaded content.
    inputs = {}
    for name, expected in FILES.items():
        path = args.output / name
        if path.exists() or args.check:
            data = path.read_bytes()
        else:
            url = f"https://raw.githubusercontent.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator/{COMMIT}/{name}"
            with urllib.request.urlopen(url, timeout=30) as response:
                data = response.read()
        actual = hashlib.sha256(data).hexdigest()
        if actual != expected:
            raise SystemExit(f"{name}: SHA-256 mismatch: expected {expected}, got {actual}")
        inputs[path] = data
    if not args.check:
        for path, data in inputs.items():
            if not path.exists():
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(data)
    print(f"Verified VMA 3.4.0 {COMMIT}: {len(inputs)} inputs at {args.output.resolve()}")


if __name__ == "__main__":
    main()
