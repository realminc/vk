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
    let _: vk::DeviceHandle = vk::DeviceHandle::NULL;
    let _: Option<vk::StdVideoVP9ColorConfig> = None;
    let words = [0_u32; 5];
    let shader = vk::ShaderModuleCreateInfo::default().code(&words);
    assert_eq!(shader.code_size, 20);
    let swaps = [vk::SwapchainKHR::NULL];
    let indices = [0_u32];
    let mut results = [vk::Result::NOT_READY];
    let present = vk::PresentInfoKHR::default()
        .swapchains(&swaps, &indices, Some(&mut results)).unwrap();
    assert_eq!(present.swapchain_count, 1);
    std::hint::black_box(&present);
    assert_eq!(results[0], vk::Result::NOT_READY);
    let mut bytes = [0_u8; 13];
    let binary = vk::PipelineBinaryDataKHR::default().data(&mut bytes).unwrap();
    assert_eq!(binary.data_size, 13);
    std::hint::black_box(&binary);
    bytes[0] = 1;
    assert_eq!(bytes[0], 1);
    let layer_values = [3_i64, 9];
    let setting = vk::LayerSettingEXT::default().values_int64(&layer_values).unwrap();
    assert_eq!(setting.value_count, 2);
    assert_eq!(setting.r#type, vk::LayerSettingTypeEXT::INT64_EXT);
    #[cfg(feature = "loader")]
    {
        let _: Option<vk::Entry> = None;
        let _ = vk::create_instance;
        let _ = vk::Instance::new;
    }
    #[cfg(feature = "allocator")]
    {
        let _ = vk::Allocator::new_with_granularity;
        let _ = vk::Allocator::allocate_with_options;
        let _ = vk::Allocator::find_memory_type;
        let _ = vk::Allocator::find_memory_type_for_buffer_info;
        let _ = vk::Allocator::find_memory_type_for_image_info;
        let _ = vk::mem::allocate_with_options;
        let _ = vk::mem::find_memory_type;
        let _ = vk::mem::find_memory_type_for_buffer_info;
        let _ = vk::mem::find_memory_type_for_image_info;
        let _ = vk::Allocator::allocate_for_buffer;
        let _ = vk::Allocator::allocate_for_image;
        let _ = vk::mem::allocate_for_buffer;
        let _ = vk::mem::allocate_for_image;
        let _ = vk::Allocator::bind_buffer_memory;
        let _ = vk::Allocator::bind_image_memory;
        let _ = vk::mem::bind_buffer_memory;
        let _ = vk::mem::bind_image_memory;
        let _ = vk::Allocator::trim;
        let _ = vk::Allocator::trim_with_budget;
        let _ = vk::mem::trim;
        let _ = vk::mem::trim_with_budget;
        assert_eq!(vk::mem::TrimStats::default().block_bytes, 0);
        let _ = vk::Allocator::create_image_with_options;
        let _ = vk::mem::create_image_with_options;
        let _ = vk::Allocator::query_budget;
        let _ = vk::Allocator::allocation_info;
        let _ = vk::mem::allocation_info;
        let _: Option<vk::AllocationInfo> = None;
        let _: Option<vk::Pool> = None;
        let _ = vk::Allocator::create_pool;
        let _ = vk::mem::create_pool;
        let _ = vk::Allocator::destroy_pool;
        let _ = vk::mem::pool_statistics;
        let options = vk::mem::AllocationCreateInfo {
            usage: vk::mem::MemoryUsage::PreferDevice,
            policy: vk::mem::AllocationPolicy::Adaptive { block_size: 65536, min_block_size: 4096 },
            ..Default::default()
        };
        assert!(!options.persistent_map);
        let mut block = vk::VirtualBlock::new(4096).unwrap();
        let mut allocation = block.allocate(64, 16, vk::mem::PlacementStrategy::Default).unwrap();
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
