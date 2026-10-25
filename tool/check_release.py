#!/usr/bin/env python3
"""Require matching owned versions and, for a tag, verified main ancestry."""

import os
from pathlib import Path
import re
import subprocess
import tomllib


def main():
    root = Path(__file__).resolve().parents[1]
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    version = manifest["workspace"]["package"]["version"]
    lock = tomllib.loads((root / "Cargo.lock").read_text())["package"]
    for name in ("vkrust", "vkrust-gen"):
        rows = [row for row in lock if row["name"] == name]
        if len(rows) != 1 or rows[0]["version"] != version:
            raise ValueError(f"lockfile version mismatch: {name}")
    toolchain = tomllib.loads((root / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    if manifest["workspace"]["package"]["rust-version"] != toolchain:
        raise ValueError("toolchain and declared Rust baseline disagree")
    ref = os.environ.get("GITHUB_REF", "")
    if ref.startswith("refs/tags/"):
        if ref != f"refs/tags/v{version}" or not re.fullmatch(r"\d+\.\d+\.\d+", version):
            raise ValueError("release tag and manifest version disagree")
        subprocess.run(["git", "merge-base", "--is-ancestor", "HEAD", "origin/main"], cwd=root, check=True)
        if not (root / f"docs/external/release/v{version}.md").is_file():
            raise ValueError("missing versioned release notes")
    print(f"Verified release identity: vkrust/vk {version}, Rust {toolchain}")


if __name__ == "__main__":
    main()
