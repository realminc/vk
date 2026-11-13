#!/usr/bin/env python3
"""Prepare/build/run the offscreen public-API triangle. Default: plan, no native work."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import sys

import bench
import qualify


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("plan", "build", "run"), nargs="?", default="plan")
    parser.add_argument("--build", type=Path, default=bench.ROOT / "target/sdk/triangle")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--device", type=int, default=0)
    parser.add_argument("--layer-manifest", type=Path)
    args = parser.parse_args()
    if args.action == "plan":
        print("Vulkan 1.3 offscreen RGBA8 triangle: dynamic rendering, sync2, copy2, submit2, native allocator, exact pixel oracle. WSI and timing are separate.")
        return
    env = bench.pinned_toolchain(bench.environment())
    build = args.build.resolve()
    if args.action == "build":
        build.mkdir(parents=True, exist_ok=True)
        for stage in ("vert", "frag"):
            bench.run(["glslangValidator", "-V", "--target-env", "vulkan1.3", "-S", stage,
                bench.SDK / f"src/shader/triangle.{stage}", "-o", build / f"triangle.{stage}.spv"], build, f"shader-{stage}", env)
        bench.run(["cargo", "build", "--manifest-path", bench.SDK / "Cargo.toml", "--locked", "--release",
            "--features", "triangle", "--bin", "vk-triangle", "--target-dir", build / "rust"], build, "build", env)
        record = bench.manifest(env)
        suffix = ".exe" if os.name == "nt" else ""
        paths = [build / "rust/release" / f"vk-triangle{suffix}", build / "triangle.vert.spv", build / "triangle.frag.spv"]
        record["artifact_sha256"] = {str(p.relative_to(build)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
        (build / "build.json").write_text(json.dumps(record, indent=2) + "\n")
        return
    if not args.output or args.output.exists() or args.device < 0 or not args.layer_manifest:
        parser.error("run requires a new --output directory, --layer-manifest and nonnegative device index")
    env["VK_SDK_DEVICE"] = str(args.device)
    output = args.output.resolve()
    output.mkdir(parents=True)
    env = qualify.validation_environment(env, output, args.layer_manifest.resolve(), "sync")
    built = json.loads((build / "build.json").read_text())
    if built["source_sha256"] != bench.manifest(env)["source_sha256"]:
        raise ValueError("sources changed since triangle build")
    for path, digest in built["artifact_sha256"].items():
        if hashlib.sha256((build / path).read_bytes()).hexdigest() != digest:
            raise ValueError("triangle artifact changed since build")
    (output / "environment.json").write_text(json.dumps(bench.manifest(env), indent=2) + "\n")
    suffix = ".exe" if os.name == "nt" else ""
    stdout, stderr = bench.run([build / "rust/release" / f"vk-triangle{suffix}",
        build / "triangle.vert.spv", build / "triangle.frag.spv", output / "triangle.ppm"], output, "triangle", env)
    if qualify.DIAGNOSTICS.search(stdout + stderr): raise RuntimeError("triangle emitted validation diagnostics")
    if qualify.LAYER_ACTIVE not in stderr: raise RuntimeError("validation layer activation was not confirmed")
    record = json.loads(stdout)
    if record.get("center_and_corner") != "passed" or record.get("cleanup") != "passed" or not 500 <= record["red_pixels"] < 2000:
        raise ValueError("incomplete triangle evidence")
    (output / "results.json").write_text(json.dumps(record, indent=2) + "\n")
    print(f"Offscreen pixel oracle passed; validation output retained at {output}. This is not presentation or performance evidence.")


if __name__ == "__main__":
    main()
