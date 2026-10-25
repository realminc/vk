#!/usr/bin/env python3
"""Fetch only hash-verified pinned C headers for development ABI oracles."""

import argparse
import hashlib
from pathlib import Path
import re
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "target/headers")
    args = parser.parse_args()
    source = (ROOT / "registry/SOURCES.md").read_text()
    commit = re.search(r"\| Commit \| `([0-9a-f]{40})`", source).group(1)
    verified = {}
    for row in (ROOT / "registry/ABI_SHA256SUMS").read_text().splitlines():
        digest, name = row.split()
        target = args.output / name
        if target.exists():
            data = target.read_bytes()
        else:
            url = f"https://raw.githubusercontent.com/KhronosGroup/Vulkan-Headers/{commit}/include/{name}"
            with urllib.request.urlopen(url, timeout=30) as response:
                data = response.read()
        if hashlib.sha256(data).hexdigest() != digest:
            raise ValueError(f"pinned header hash mismatch: {name}")
        verified[target] = data
    for target, data in verified.items():
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    print(f"Verified {len(verified)} C headers at {commit}")


if __name__ == "__main__":
    main()
