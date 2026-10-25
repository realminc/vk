#!/usr/bin/env python3
"""Differential enabled-feature capture oracle against pinned XML.

Builds an external Rust consumer. XML is read independently of vk-gen; emitted
Rust declarations identify compiled records and Rust member spellings. Eight
bit planes give each native member a distinct value pattern, detecting swapped,
omitted or misnamed captures. No Vulkan loader or GPU is invoked.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tomllib
import xml.etree.ElementTree as ET


def main():
    repo = Path(__file__).resolve().parent.parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package", type=Path, default=repo)
    parser.add_argument("--output", type=Path, default=repo / "target/verification/features")
    args = parser.parse_args()
    package = args.package.resolve()
    output = args.output.resolve()
    source = (package / "src/rs/type.gen.rs").read_text()
    declarations = dict(re.findall(r"pub struct (\w+) \{\n(.*?)\n\}", source, re.S))
    registry = ET.parse(repo / "registry/vk.xml").getroot()
    base = registry.find("types/type[@name='VkPhysicalDeviceFeatures']")
    assert base is not None
    records = [base]
    records.extend(node for node in registry.findall("types/type")
                   if "VkPhysicalDeviceFeatures2" in node.get("structextends", "").split(",")
                   and node.get("name", "").removeprefix("Vk") in declarations)
    assert len(records) > 200, "unexpectedly incomplete compiled feature inventory"
    # Require every core-introduced Boolean feature record instead of silently
    # accepting missing declarations through the compiled-record intersection.
    # Unified Vulkan11Features first appears in 1.2, not 1.1.
    captured_names = {node.get("name") for node in records}
    for version, expected in [("1.1", (6, 12)), ("1.2", (15, 99)),
                              ("1.3", (14, 30)), ("1.4", (14, 41))]:
        required_types = {item.get("name") for feature in registry.findall("feature")
                          if feature.get("number") == version
                          and "vulkan" in feature.get("api", "").split(",")
                          for group in feature.findall("require")
                          if "vulkan" in group.get("api", "vulkan").split(",")
                          for item in group.findall("type")}
        core = [node for node in registry.findall("types/type")
                if node.get("name") in required_types
                and "VkPhysicalDeviceFeatures2" in node.get("structextends", "").split(",")]
        count = sum(len([m for m in node.findall("member") if m.findtext("type") == "VkBool32"])
                    for node in core)
        assert (len(core), count) == expected, f"pinned core {version} feature inventory changed"
        assert all(node.get("name") in captured_names for node in core), f"missing core {version} feature record"
    lines = ["fn main() {", "let mut comparisons = 0usize;", "// SAFETY: all complete records and input chains are initialized and local.", "unsafe {",
             "let app = vk::ApplicationInfo { api_version: vk::API_VERSION_1_4, ..Default::default() };", "let parent = vk::Capabilities::new_from_instance_create_info(&vk::InstanceCreateInfo { p_application_info: &app, ..Default::default() }, vk::API_VERSION_1_4).unwrap();"]
    members_total = 0
    for node in records:
        native = node.get("name")
        rust_type = native.removeprefix("Vk")
        native_members = [member.findtext("name") for member in node.findall("member")
                          if member.findtext("type") == "VkBool32"]
        rust_members = re.findall(r"pub (\w+): Bool32,", declarations[rust_type])
        assert len(native_members) == len(rust_members) and 0 < len(native_members) < 256, native
        members_total += len(native_members)
        lines.extend(["{", "for plane in 0..8 {", f"let value = vk::{rust_type} {{"])
        for index, field in enumerate(rust_members, 1):
            lines.append(f"{field}: ({index}u32 >> plane) & 1,")
        if native != "VkPhysicalDeviceFeatures":
            lines.append("..Default::default()")
        lines.append("};")
        if native == "VkPhysicalDeviceFeatures":
            lines.append("let info = vk::DeviceCreateInfo { p_enabled_features: &value, ..Default::default() };")
            # Also prove the Features2 base representation independently.
            lines.append("let head = vk::PhysicalDeviceFeatures2 { features: value, ..Default::default() };")
        else:
            lines.append(f"let info = vk::DeviceCreateInfo {{ p_next: (&value as *const vk::{rust_type}).cast(), ..Default::default() }};")
        # Numeric StructureType from the actual typed record avoids duplicating
        # vk-gen's enum-name conversion (e.g. Vulkan11 versus VULKAN_1_1).
        structure = "vk::StructureType::PHYSICAL_DEVICE_FEATURES_2" if native == "VkPhysicalDeviceFeatures" else "value.s_type"
        lines.append(f"let structure = {structure};")
        rounds = "[info, vk::DeviceCreateInfo { p_next: (&head as *const vk::PhysicalDeviceFeatures2).cast(), ..Default::default() }]" if native == "VkPhysicalDeviceFeatures" else "[info]"
        lines.append(f"for info in {rounds} {{")
        lines.append("let snapshot = parent.new_from_device_create_info(&info, vk::API_VERSION_1_4).unwrap();")
        lines.append("let features = snapshot.features().unwrap();")
        lines.append(f"assert_eq!(features.values().iter().filter(|v| v.structure == structure).count(), {len(native_members)});")
        for index, name in enumerate(native_members, 1):
            lines.append(f"assert_eq!(features.enabled(structure, {json.dumps(name)}), Some((({index}u32 >> plane) & 1) != 0), {json.dumps(native + '::' + name)});")
            lines.append("comparisons += 1;")
        lines.extend(["}", "}", "}"])
    lines.extend(["}", f'println!("{len(records)} feature records (all core 1.1–1.4 records included), {members_total} members, {{comparisons}} bit-plane comparisons passed");', "}"])
    (output / "src").mkdir(parents=True, exist_ok=True)
    manifest = tomllib.loads((package / "Cargo.toml").read_text())
    features = sorted(manifest["features"])
    (output / "Cargo.toml").write_text('[package]\nname="vk-feature-oracle"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(package)) + ',features=' + json.dumps(features) + '}\n[profile.dev]\ndebug=0\nincremental=false\n')
    (output / "src/main.rs").write_text("\n".join(lines) + "\n")
    env = os.environ.copy()
    # Archives normalize mtimes: key build caches by content so replacing an
    # extracted same-version package cannot leave a stale compiled dependency.
    digest = hashlib.sha256()
    for path in [package / "Cargo.toml", *sorted((package / "src/rs").rglob("*.rs"))]:
        digest.update(path.relative_to(package).as_posix().encode())
        digest.update(b"\0")
        digest.update(path.read_bytes())
        digest.update(b"\0")
    env["CARGO_TARGET_DIR"] = str(output / "target" / digest.hexdigest()[:16])
    subprocess.run(["cargo", "run", "--offline", "--manifest-path", str(output / "Cargo.toml")], env=env, check=True)


if __name__ == "__main__":
    main()
