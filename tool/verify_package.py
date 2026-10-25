#!/usr/bin/env python3
"""Verify the actual Cargo archive and execute a separate public API consumer."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import tomllib

from audit_crate import audit

ROOT = Path(__file__).resolve().parents[1]


def run(*args, cwd=ROOT, env=None):
    subprocess.run(args, cwd=cwd, env=env, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--allow-dirty", action="store_true", help="local candidate only, never release CI")
    args = parser.parse_args()
    package = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]
    version = package["version"]
    output = ROOT / "target/verification/package"
    output.mkdir(parents=True, exist_ok=True)
    archive = ROOT / "target/package" / f"vkrust-{version}.crate"
    archive.parent.mkdir(parents=True, exist_ok=True)
    # Isolate Cargo's archive verification from workspace dep-info. Rebuilding
    # the same package identity from extracted source must not poison host tests.
    with tempfile.TemporaryDirectory(prefix="build-", dir=output) as temporary:
        env = dict(os.environ, CARGO_TARGET_DIR=temporary, CARGO_INCREMENTAL="0")
        command = ["cargo", "package", "-p", "vkrust", "--locked"]
        if args.allow_dirty:
            command.append("--allow-dirty")
        run(*command, env=env)
        staged = Path(temporary) / "package" / archive.name
        files = audit(staged, version, package["repository"])
        for name, data in files.items():
            if name not in {"Cargo.toml", "Cargo.toml.orig", "Cargo.lock", ".cargo_vcs_info.json"}:
                if (ROOT / name).read_bytes() != data:
                    raise ValueError(f"archive source mismatch: {name}")
        shutil.copyfile(staged, archive)
    with tempfile.TemporaryDirectory(prefix="vk-consumer-") as temporary:
        temporary = Path(temporary)
        with tarfile.open(archive) as tar:
            tar.extractall(temporary, filter="data")
        extracted = temporary / f"vkrust-{version}"
        env = dict(os.environ, CARGO_TARGET_DIR=str(temporary / "target"), CARGO_INCREMENTAL="0")
        run("cargo", "test", "--manifest-path", str(extracted / "Cargo.toml"), "--all-features", "--locked", env=env)
        consumer = temporary / "consumer"
        (consumer / "src").mkdir(parents=True)
        (consumer / "Cargo.toml").write_text(
            '[package]\nname="vk-package-proof"\nversion="0.0.0"\nedition="2024"\n[workspace]\n'
            '[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(extracted)) +
            ',default-features=false}\n[features]\nloader=["vk/loader"]\nallocator=["loader","vk/allocator"]\n'
            '[profile.dev]\ndebug=0\nincremental=false\n'
        )
        (consumer / "src/main.rs").write_text('''fn main() {
    let info = vk::InstanceCreateInfo::default();
    assert_eq!(info.s_type, vk::StructureType::INSTANCE_CREATE_INFO);
    assert!(vk::InstanceHandle::NULL.is_null());
    assert_eq!(std::mem::size_of::<vk::PFN_vkCreateInstance>(), std::mem::size_of::<usize>());
    let _: vk::core::DeviceHandle = vk::DeviceHandle::NULL;
    let _: Option<vk::StdVideoVP9ColorConfig> = None;
    #[cfg(feature = "loader")]
    {
        let _: Option<vk::Entry> = None;
        let _ = vk::create_instance;
        let _ = vk::Instance::new;
    }
    #[cfg(feature = "allocator")]
    {
        let mut block = vk::VirtualBlock::new(4096).unwrap();
        let mut allocation = block.allocate(64, 16, vk::memory::PlacementStrategy::Default).unwrap();
        assert_eq!(block.allocation_info(&allocation).unwrap().offset, 0);
        block.free(&mut allocation).unwrap();
    }
}
''')
        for features in ([], ["--features", "loader"], ["--features", "allocator"]):
            run("cargo", "run", "--quiet", *features, cwd=consumer, env=env)
        run("cargo", "clippy", "--all-features", "--", "-D", "warnings", cwd=consumer, env=env)
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    report = {"archive": archive.name, "sha256": digest, "version": version, "repository": package["repository"], "consumer": "passed"}
    (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
