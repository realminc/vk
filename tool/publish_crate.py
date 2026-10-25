#!/usr/bin/env python3
"""Publish only an archive identical to the CI-verified release artifact.

Re-runs admit an existing non-yanked version only when its registry checksum
matches. A failed API request is never treated as an absent crate version.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
import urllib.error
import urllib.request
from urllib.parse import quote

from audit_crate import audit
from audit_public import PUBLIC_REPOSITORY
from release_version import ROOT, package_version, release_version, validate_registry_version


def metadata(version):
    request = urllib.request.Request(
        f"https://crates.io/api/v1/crates/vkrust/{quote(version, safe='')}",
        headers={"User-Agent": "vk-release (https://github.com/realminc/vk)"},
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)["version"]
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise
        return None


def verify_existing(existing, digest):
    if existing.get("yanked") is not False or existing.get("checksum") != digest:
        raise ValueError("registry version differs from verified archive or is yanked")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("--verify-only", action="store_true")
    args = parser.parse_args()
    tag = os.environ.get("GITHUB_REF_NAME", "")
    if os.environ.get("GITHUB_REPOSITORY") != "realminc/vk" or not re.fullmatch(r"v\d+\.\d+\.\d+", tag):
        raise ValueError("publication requires a realminc/vk version tag")
    version = package_version()
    if tag != f"v{release_version(version)}":
        raise ValueError("publication tag and package release version disagree")
    validate_registry_version(ROOT, version)
    archive = args.archive.resolve()
    if archive.name != f"vkrust-{version}.crate":
        raise ValueError("archive filename must include the complete package version")
    audit(archive, version, PUBLIC_REPOSITORY)
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    existing = metadata(version)
    if args.verify_only:
        # Bounded propagation wait after a successful upload; never republishes.
        for _ in range(11):
            if existing is not None:
                break
            time.sleep(5)
            existing = metadata(version)
        if existing is None:
            raise ValueError("published version not found")
    elif existing is None:
        if not os.environ.get("CARGO_REGISTRY_TOKEN"):
            raise ValueError("missing crates.io publishing credential")
        subprocess.run(["cargo", "package", "-p", "vkrust", "--locked", "--no-verify"], check=True)
        rebuilt = Path("target/package") / archive.name
        if rebuilt.read_bytes() != archive.read_bytes():
            raise ValueError("repackaged archive differs from verified CI artifact")
        # --no-verify skips repeating compilation, not packaging or the byte gate.
        subprocess.run(["cargo", "publish", "-p", "vkrust", "--locked", "--no-verify"], check=True)
        existing = metadata(version)
        for _ in range(11):
            if existing is not None:
                break
            time.sleep(5)
            existing = metadata(version)
        if existing is None:
            raise ValueError("upload completed but registry confirmation is pending")
    verify_existing(existing, digest)
    request = urllib.request.Request(
        f"https://crates.io/api/v1/crates/vkrust/{quote(version, safe='')}/download",
        headers={"User-Agent": "vk-release (https://github.com/realminc/vk)"},
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        if response.read() != archive.read_bytes():
            raise ValueError("downloaded crate differs from verified archive")
    print(f"Verified crates.io vkrust {version}: {digest}")


if __name__ == "__main__":
    main()
