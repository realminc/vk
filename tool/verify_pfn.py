#!/usr/bin/env python3
"""Check every pinned Vulkan command prototype independently of vk-gen.

Compiles XML-derived C type assertions against hash-verified headers and an
external Rust consumer against the actual package. Protected platform C
prototypes absent from vulkan_core.h are reported separately, never qualified
from host Rust compilation. No Vulkan implementation or GPU is invoked.
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


ROOT = Path(__file__).resolve().parents[1]


def declaration(node):
    return (node.text or "") + "".join(
        ("" if child.tag == "comment" else "".join(child.itertext()))
        + (child.tail or "") for child in node
    )


def admitted_commands(registry):
    commands = {}
    for node in registry.findall("commands/command"):
        name = node.get("name") or node.findtext("proto/name")
        if name:
            commands[name] = node
    selected = set()
    providers = [node for node in registry.findall("feature")
                 if "vulkan" in node.get("api", "").split(",")]
    providers += [node for node in registry.findall("extensions/extension")
                  if "vulkan" in node.get("supported", "").split(",")]
    for provider in providers:
        for group in provider.findall("require"):
            if "vulkan" not in group.get("api", "vulkan").split(","):
                continue
            selected.update(node.get("name") for node in group.findall("command")
                            if "vulkan" in node.get("api", "vulkan").split(","))
    pending = list(selected)
    while pending:
        alias = commands[pending.pop()].get("alias")
        if alias and alias not in selected:
            selected.add(alias)
            pending.append(alias)
    assert len(selected) == 842, "pinned Vulkan command inventory changed"
    result = {}
    for name, node in commands.items():
        if name not in selected:
            continue
        seen = set()
        while node.get("alias"):
            target = node.get("alias")
            assert target not in seen, f"alias cycle: {name}"
            seen.add(target)
            node = commands[target]
        result[name] = node
    return result


def rust_base(name):
    primitives = {"void": "std::ffi::c_void", "char": "std::ffi::c_char",
                  "int": "i32", "float": "f32", "double": "f64", "size_t": "usize"}
    if name in primitives:
        return primitives[name]
    if re.fullmatch(r"u?int(8|16|32|64)_t", name):
        return name.replace("uint", "u").replace("int", "i").removesuffix("_t")
    return "vk::" + {"VkInstance": "InstanceHandle", "VkDevice": "DeviceHandle"}.get(
        name, name.removeprefix("Vk"))


def rust_type(node, parameter=True):
    """Read C declarators independently, including const at each pointer level."""
    text = declaration(node)
    native = node.findtext("type")
    identifier = node.findtext("name")
    prefix, remainder = text.split(native, 1)
    declarator, suffix = remainder.split(identifier, 1)
    result = rust_base(native)
    constant = "const" in prefix.split()
    for level in declarator.split("*")[1:]:
        result = f"*{'const' if constant else 'mut'} {result}"
        constant = "const" in level.split()
    extents = re.findall(r"\[([^\]]+)\]", suffix)
    if parameter and extents:
        extents = extents[1:]
    for extent in reversed(extents):
        size = extent if extent.isdecimal() else "vk::" + extent.removeprefix("VK_") + " as usize"
        result = f"[{result}; {size}]"
    if parameter and "[" in suffix:
        result = f"*{'const' if constant else 'mut'} {result}"
    if not parameter and native == "void" and "*" not in declarator:
        result = "()"
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--headers", required=True, type=Path)
    parser.add_argument("--package", default=ROOT, type=Path)
    parser.add_argument("--output", default=ROOT / "target/verification/pfn", type=Path)
    args = parser.parse_args()
    headers, package, output = (path.resolve() for path in (args.headers, args.package, args.output))
    output.mkdir(parents=True, exist_ok=True)
    for line in (ROOT / "registry/ABI_SHA256SUMS").read_text().splitlines():
        expected, name = line.split()
        assert hashlib.sha256((headers / name).read_bytes()).hexdigest() == expected, name
    for line in (ROOT / "registry/SHA256SUMS").read_text().splitlines():
        expected, name = line.split()
        assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == expected, name
    registry = ET.parse(ROOT / "registry/vk.xml").getroot()
    commands = admitted_commands(registry)
    callbacks = {node.findtext("proto/name").removeprefix("PFN_") for node in registry.findall("types/type[@category='funcpointer']")}
    actual = set(re.findall(r"pub type PFN_(vk\w+)\s*=", (package / "src/rs/pfn.gen.rs").read_text()))
    actual -= callbacks
    assert actual == commands.keys(), f"PFN inventory mismatch: {actual ^ commands.keys()}"
    cc = os.environ.get("CC", "cc")
    includes = "#include <vulkan/vulkan_core.h>\n"
    processed = subprocess.run([cc, "-E", "-P", "-x", "c", "-I", str(headers), "-"],
                               input=includes, text=True, capture_output=True, check=True).stdout
    present = set(re.findall(r"\bPFN_(vk\w+)\b", processed))
    c = [includes]
    rust = ["#![allow(non_snake_case)]", "fn main() {"]
    checks = []
    for index, (name, node) in enumerate(commands.items()):
        params = [p for p in node.findall("param") if "vulkan" in p.get("api", "vulkan").split(",")]
        if name in present:
            result = node.findtext("proto/type")
            c.append(f"typedef {result} (VKAPI_PTR *Oracle_{index})({', '.join(map(declaration, params)) or 'void'});")
            c.append(f'_Static_assert(__builtin_types_compatible_p(PFN_{name}, Oracle_{index}), "{name}");')
        signature = ", ".join(f"_: {rust_type(p)}" for p in params)
        returns = rust_type(node.find("proto"), parameter=False)
        checks.append(f"unsafe extern \"system\" fn check_{index}({signature}) -> {returns} {{\n"
                      "// SAFETY: never called; all admitted ABI returns have a valid zero representation.\n"
                      "unsafe { std::mem::zeroed() }\n}")
        rust.append(f"let _: vk::PFN_{name} = Some(check_{index});")
    rust += ["}", *checks]
    (output / "prototype.c").write_text("\n".join(c) + "\n")
    subprocess.run([cc, "-std=c11", "-Werror", "-fsyntax-only", "-I", str(headers), str(output / "prototype.c")], check=True)
    (output / "src").mkdir(exist_ok=True)
    (output / "src/main.rs").write_text("\n".join(rust) + "\n")
    (output / "Cargo.toml").write_text('[package]\nname="vk-pfn-oracle"\nversion="0.0.0"\nedition="2024"\n[workspace]\n'
                                      '[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(package)) +
                                      ',default-features=false,features=[' +
                                      ','.join(json.dumps(feature) for feature in
                                               tomllib.loads((package / 'Cargo.toml').read_text())["features"]
                                               if feature not in ("default", "loader", "allocator")) + ']}\n'
                                      '[profile.dev]\ndebug=0\nincremental=false\n')
    digest = hashlib.sha256()
    for path in [package / "Cargo.toml", *sorted((package / "src/rs").rglob("*.rs"))]:
        digest.update(path.relative_to(package).as_posix().encode() + b"\0" + path.read_bytes() + b"\0")
    subprocess.run(["cargo", "check", "--offline", "--manifest-path", str(output / "Cargo.toml")], check=True,
                   env={**os.environ, "CARGO_TARGET_DIR": str(output / "target" / digest.hexdigest()[:16])})
    missing = sorted(commands.keys() - present)
    report = {"rust_prototypes": len(commands), "c_prototypes": len(commands.keys() & present),
              "c_unqualified": missing}
    (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"PFN verified: {report['rust_prototypes']} independent XML/Rust prototypes; "
          f"{report['c_prototypes']} pinned-header C prototypes; {len(missing)} protected C prototypes unqualified")


if __name__ == "__main__":
    main()
