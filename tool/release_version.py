#!/usr/bin/env python3
"""Keep library release tags distinct from the pinned Vulkan build metadata."""

import argparse
from pathlib import Path
import re
import tomllib
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
NUMBER = r"(?:0|[1-9][0-9]*)"
TRIPLE = rf"{NUMBER}\.{NUMBER}\.{NUMBER}"
VERSION = re.compile(rf"({TRIPLE})(?:\+({TRIPLE}))?")


def release_version(version):
    match = VERSION.fullmatch(version)
    if match is None:
        raise ValueError("expected X.Y.Z or X.Y.Z+VulkanMajor.VulkanMinor.HeaderRevision")
    return match[1]


def package_version(root=ROOT):
    return tomllib.loads((root / "Cargo.toml").read_text())["workspace"]["package"]["version"]


def validate_registry_version(root, version):
    release_version(version)
    definitions = {}
    for node in ET.parse(root / "registry/vk.xml").findall("types/type"):
        if "vulkan" in node.get("api", "vulkan").split(","):
            definitions[node.findtext("name")] = "".join(node.itertext())
    header = re.search(r"VK_HEADER_VERSION\s+(\d+)\b", definitions["VK_HEADER_VERSION"])
    complete = re.search(
        r"VK_MAKE_API_VERSION\(\s*0\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*VK_HEADER_VERSION\s*\)",
        definitions["VK_HEADER_VERSION_COMPLETE"],
    )
    if header is None or complete is None:
        raise ValueError("unsupported pinned Vulkan header version declarations")
    expected = f"{complete[1]}.{complete[2]}.{header[1]}"
    if version.partition("+")[2] != expected:
        raise ValueError(f"package build metadata must match pinned Vulkan headers: +{expected}")
    generated = (root / "src/rs/constant.gen.rs").read_text()
    if not re.search(rf"pub const HEADER_VERSION: u32 = {header[1]};", generated):
        raise ValueError("generated HEADER_VERSION differs from the pinned registry")
    if not re.search(
        rf"pub const HEADER_VERSION_COMPLETE: u32 = crate::make_api_version\(0, {complete[1]}, {complete[2]}, HEADER_VERSION\);",
        generated,
    ):
        raise ValueError("generated HEADER_VERSION_COMPLETE differs from the pinned registry")
    return expected


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("field", choices=("package", "release", "tag", "vulkan"))
    args = parser.parse_args()
    version = package_version()
    release = release_version(version)
    vulkan = validate_registry_version(ROOT, version)
    print({"package": version, "release": release, "tag": f"v{release}", "vulkan": vulkan}[args.field])
