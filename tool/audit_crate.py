#!/usr/bin/env python3
"""Audit the vkrust registry archive before extraction, compilation or upload."""

from pathlib import Path, PurePosixPath
import argparse
import posixpath
import re
import tarfile
import tomllib


def audit(path, version, repository=None):
    root = f"vkrust-{version}"
    allowed = {
        "Cargo.toml", "Cargo.toml.orig", "Cargo.lock", ".cargo_vcs_info.json",
        "README.md", "CHANGELOG.md", "NOTICE.md", "LICENSE", "license/khronos/Apache-2.0.txt",
        "LICENSE-VMA-MIT", "rustfmt.toml", "docs/external/qualification.md", "registry/SOURCES.md",
    }
    prefixes = ("src/rs/", "test/rs/core/", "test/rs/load/", "test/rs/memory/")
    umbrellas = {f"test/rs/test_{name}.rs" for name in ("core", "load", "memory")}
    if path.stat().st_size > 10 * 1024 * 1024:
        raise ValueError("crate archive exceeds 10 MiB")
    files = {}
    total = 0
    with tarfile.open(path, "r:gz") as archive:
        for member in archive:
            parts = PurePosixPath(member.name).parts
            if not parts or parts[0] != root or ".." in parts or member.name.startswith("/"):
                raise ValueError(f"unsafe archive path: {member.name}")
            if member.isdir():
                continue
            if not member.isfile():
                raise ValueError(f"non-file archive entry: {member.name}")
            name = "/".join(parts[1:])
            if name in files or not (name in allowed or name in umbrellas or name.startswith(prefixes)):
                raise ValueError(f"unexpected archive path: {name}")
            total += member.size
            if total > 100 * 1024 * 1024:
                raise ValueError("uncompressed archive exceeds 100 MiB")
            stream = archive.extractfile(member)
            if stream is None:
                raise ValueError(f"unreadable entry: {name}")
            files[name] = stream.read()
    required = allowed - {".cargo_vcs_info.json"}
    if not required <= files.keys() or not umbrellas <= files.keys() or "src/rs/lib.rs" not in files:
        raise ValueError("archive is missing required source, tests, notices or metadata")
    manifest = tomllib.loads(files["Cargo.toml"].decode())
    package = manifest["package"]
    if (package["name"], package["version"], package["license"]) != ("vkrust", version, "MIT"):
        raise ValueError("archive identity or MIT license mismatch")
    if repository is not None and package["repository"] != repository:
        raise ValueError("archive repository mismatch")
    if manifest["lib"]["name"] != "vk" or manifest["lib"]["path"] != "src/rs/lib.rs":
        raise ValueError("archive must preserve the vk namespace")
    for table in [manifest, *manifest.get("target", {}).values()]:
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            for name, dependency in table.get(section, {}).items():
                if isinstance(dependency, dict) and ("git" in dependency or "path" in dependency):
                    raise ValueError(f"non-registry dependency: {name}")
    if manifest.get("bin") or manifest.get("package", {}).get("build"):
        raise ValueError("library package must not build a generator")
    # Include private unit-test path targets too: consumers must be able to run
    # cargo test on the archive without the private source checkout.
    for name, contents in files.items():
        if not name.endswith(".rs"):
            continue
        text = contents.decode()
        paths = re.findall(r'#\[path\s*=\s*"([^"\n]+)"\]', text)
        paths += re.findall(r'\binclude(?:_bytes|_str)?!\(\s*"([^"\n]+)"\s*\)', text)
        for relative in paths:
            target = posixpath.normpath(posixpath.join(posixpath.dirname(name), relative))
            if target not in files:
                raise ValueError(f"missing Rust build input: {target} (from {name})")
    print(f"Verified {path.name}: {len(files)} files, MIT, vk namespace, registry-only dependencies")
    return files


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("version")
    parser.add_argument("--repository")
    args = parser.parse_args()
    audit(args.archive, args.version, args.repository)
