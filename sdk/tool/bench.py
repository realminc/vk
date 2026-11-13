#!/usr/bin/env python3
"""Build or run isolated, fresh-process Vulkan benchmarks. Default: print plan only."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import random
import statistics
import subprocess
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[2]
SDK = ROOT / "sdk"
sys.path.insert(0, str(ROOT / "tool/verify_memory"))
import fetch_donor

CASES = {"startup": "ash", "query": "ash", "virtual": "vma",
         "buffer-pool": "vma", "buffer-separate": "vma"}
REFERENCE_CASES = {"ash": ("startup", "query"), "vma": ("virtual", "buffer-pool", "buffer-separate"),
                   "vulkano": ("startup", "buffer-pool", "buffer-separate")}
SIZES = [64, 256, 4096, 16384, 65536]
IDENTITY = ("device_uuid", "driver_uuid", "vendor_id", "device_id", "device_type",
            "api_version", "driver_version", "queue_family", "device_count")


def select_cases(reference, case):
    if reference == "auto":
        return dict(CASES) if case == "all" else {case: CASES[case]}
    allowed = REFERENCE_CASES[reference]
    if case != "all" and case not in allowed:
        raise ValueError(f"{reference} supports {', '.join(allowed)}; cached getters are not fresh driver queries")
    return {name: reference for name in (allowed if case == "all" else [case])}


def environment():
    env = os.environ.copy()
    env.update(CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="1", CC="clang", CXX="clang++", AR="llvm-ar")
    if sys.platform.startswith("linux"):
        env.update(CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="clang",
                   CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=lld")
    return env


def pinned_toolchain(env):
    channel = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    compiler = Path(subprocess.check_output(["rustup", "which", "--toolchain", channel, "rustc"], text=True).strip())
    env.update(RUSTC=str(compiler), RUSTDOC=str(compiler.with_name("rustdoc")),
               PATH=str(compiler.parent) + os.pathsep + env["PATH"])
    return env


def run(command, output, name, env, cwd=ROOT, timeout=1200):
    """Retain diagnostics and command even on failure or timeout."""
    output.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    try:
        result = subprocess.run([str(x) for x in command], cwd=cwd, env=env,
                                capture_output=True, text=True, timeout=timeout)
        code, stdout, stderr = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        code = -1
        stdout = error.stdout or b""
        stderr = error.stderr or b""
        stdout = stdout.decode(errors="replace") if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode(errors="replace") if isinstance(stderr, bytes) else stderr
    (output / f"{name}.stdout").write_text(stdout)
    (output / f"{name}.stderr").write_text(stderr)
    (output / f"{name}.process.json").write_text(json.dumps({"command": list(map(str, command)),
        "cwd": str(cwd), "exit_code": code, "elapsed_seconds": time.monotonic() - started}, indent=2) + "\n")
    if code:
        raise RuntimeError(f"{name} failed ({code}); retained diagnostics at {output}")
    return stdout, stderr


def manifest(env):
    paths = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "rust-toolchain.toml"]
    for directory in (ROOT / "src/rs", ROOT / "test/rs", ROOT / "registry", SDK):
        paths += [p for p in directory.rglob("*") if p.is_file() and "__pycache__" not in p.parts
                  and not p.is_relative_to(SDK / "target")]
    return {"schema": "vk.sdk.environment.v1", "platform": platform.platform(),
            "machine": platform.machine(), "python": sys.version,
            "source_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
            "git_status": subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT, text=True),
            "environment": {k: v for k, v in env.items() if k.startswith(("VK_", "MESA_", "CARGO_TARGET_")) or k in
                            ("CC", "CXX", "AR", "RUSTFLAGS", "RUSTC", "RUSTDOC", "CARGO_INCREMENTAL", "CARGO_BUILD_JOBS")},
            "source_sha256": {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))}}


def telemetry():
    """Read-only clocks/temperature evidence; no promise of fixed attained clocks."""
    paths = list(Path("/sys/devices/system/cpu/cpufreq").glob("policy*/scaling_cur_freq"))
    paths += list(Path("/sys/class/thermal").glob("thermal_zone*/temp"))
    paths += list(Path("/sys/class/drm").glob("card*/gt_*freq_mhz"))
    values = {}
    for path in sorted(paths):
        try: values[str(path)] = path.read_text().strip()
        except OSError: values[str(path)] = "unreadable"
    return {"monotonic_seconds": time.monotonic(), "values": values,
            "affinity": sorted(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else None}


def verify_sample(record, case, backend, args):
    if backend == "vulkano" and (case not in REFERENCE_CASES["vulkano"] or record.get("comparison_scope") != "checked-managed-api"):
        raise ValueError("Vulkano evidence must identify a supported checked managed-API workload")
    expected = {"schema": "vk.sdk.sample.v1", "case": case, "backend": backend,
                "api_requested": (1 << 22) | (int(args.api.split(".")[1]) << 12),
                "device_index": args.device, "memory_type": args.memory_type,
                "batches": args.batches, "batch_size": args.batch_size,
                "block_size": 64 * 1024 * 1024, "sizes": SIZES}
    if any(record.get(key) != value for key, value in expected.items()):
        raise ValueError("sample workload/identity does not match the requested experiment")
    metrics = record["measurements"]
    if case == "startup":
        keys = ("load_ns", "instance_ns", "device_ns")
    elif case == "query":
        if metrics.get("queries") != args.batches * args.batch_size:
            raise ValueError("query count mismatch")
        keys = ("query_ns",)
    else:
        count = args.batches * args.batch_size
        requested = sum(SIZES[i % len(SIZES)] for i in range(args.batch_size))
        if metrics.get("allocations") != count or metrics.get("requested_byte_checksum") != requested * args.batches:
            raise ValueError("allocation count/request checksum mismatch")
        peak = metrics["peak"]
        if peak["live_count"] != args.batch_size or peak["allocation_bytes"] < requested:
            raise ValueError("peak accounting mismatch")
        if case == "virtual" and peak["allocation_bytes"] != requested:
            raise ValueError("virtual accounting mismatch")
        if case == "buffer-pool" and (peak["native_count"], peak["native_bytes"]) != (1, expected["block_size"]):
            raise ValueError("fixed pool escaped its one-block policy")
        if case == "buffer-separate" and peak["native_count"] != args.batch_size:
            raise ValueError("separate storage was suballocated")
        keys = ("allocate_ns", "free_ns")
    for key in keys:
        if type(metrics.get(key)) is not int or metrics[key] <= 0:
            raise ValueError(f"invalid timer: {key}")
    if case != "virtual":
        identity = record["identity"]
        if any(key not in identity for key in IDENTITY) or identity["device_type"] == 4:
            raise ValueError("missing hardware identity or software/CPU Vulkan device")
        if identity["api_version"] < expected["api_requested"]:
            raise ValueError("device API below request")
        if len(identity["device_uuid"]) != 16 or not any(identity["device_uuid"]):
            raise ValueError("missing device UUID")
    return keys


def interval(values):
    """Paired-process median ratio CI, not a per-call p99 claim."""
    rng = random.Random(0x564B)
    boot = sorted(statistics.median(rng.choices(values, k=len(values))) for _ in range(4000))
    return [boot[math.floor(len(boot) * .025)], boot[math.floor(len(boot) * .975)]]


def summarize(pairs, keys):
    result = {}
    for key in keys:
        rust = [pair["vkrust"]["measurements"][key] for pair in pairs]
        reference = [pair["reference"]["measurements"][key] for pair in pairs]
        ratios = [a / b for a, b in zip(rust, reference)]
        ordered = sorted(rust)
        result[key] = {"vkrust_median_ns": statistics.median(rust),
                       "reference_median_ns": statistics.median(reference),
                       "vkrust_process_p95_ns": ordered[math.ceil(.95 * len(ordered)) - 1],
                       "median_paired_ratio_vk_over_reference": statistics.median(ratios),
                       "paired_median_ratio_95pct_bootstrap_ci": interval(ratios)}
    return result


def build(output, env, reference="auto"):
    if any(env.get(key) for key in ("RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_ENCODED_RUSTFLAGS")):
        raise ValueError("remove custom Rust flags/wrappers before building a comparison baseline")
    run([sys.executable, ROOT / "tool/fetch_headers.py"], output, "fetch-headers", env)
    run([sys.executable, ROOT / "tool/verify_memory/fetch_donor.py"], output, "fetch-vma", env)
    features = "ash-reference,vulkano-reference" if reference == "vulkano" else "ash-reference"
    run(["cargo", "build", "--manifest-path", SDK / "Cargo.toml", "--locked", "--release",
         "--features", features, "--bins", "--target-dir", output / "rust"], output, "build-rust", env)
    headers = ROOT / "target/headers"
    donor = ROOT / "target/verify_memory/donor" / fetch_donor.COMMIT
    flags = ["-std=c++17", "-O3", "-DNDEBUG", "-g0", "-fuse-ld=lld", "-pthread"]
    binary = output / ("vma-bench.exe" if os.name == "nt" else "vma-bench")
    if sys.platform.startswith("linux"):
        flags += ["-ldl"]
    run([env["CXX"], *flags, "-isystem", headers, "-isystem", donor / "include",
         SDK / "src/cpp/vma.cpp", "-o", binary], output, "build-vma", env)
    record = manifest(env)
    suffix = ".exe" if os.name == "nt" else ""
    artifacts = [binary, output / "rust/release" / f"vk-bench{suffix}", output / "rust/release" / f"ash-bench{suffix}"]
    record["references"] = ["ash", "vma"]
    if reference == "vulkano":
        artifacts.append(output / "rust/release" / f"vulkano-bench{suffix}")
        record["references"].append("vulkano")
    record["artifact_sha256"] = {str(p.relative_to(output)): hashlib.sha256(p.read_bytes()).hexdigest() for p in artifacts}
    record["donor_sha256"] = {name: hashlib.sha256((donor / name).read_bytes()).hexdigest() for name in fetch_donor.FILES}
    (output / "build.json").write_text(json.dumps(record, indent=2) + "\n")
    for tool, command in (("rustc", ["rustc", "-Vv"]), ("cargo", ["cargo", "-V"]),
                          ("clang", ["clang++", "--version"])):
        run(command, output, tool, env)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("plan", "build", "run"), nargs="?", default="plan")
    parser.add_argument("--build", type=Path, default=ROOT / "target/sdk/build")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--case", choices=("all", *CASES), default="all")
    parser.add_argument("--reference", choices=("auto", *REFERENCE_CASES), default="auto",
                        help="auto keeps the Ash/VMA baselines; vulkano selects checked managed-API comparisons")
    parser.add_argument("--device", type=int, default=0)
    parser.add_argument("--api", choices=("1.1", "1.2", "1.3", "1.4"), default="1.2")
    parser.add_argument("--memory-type", type=int, default=0)
    parser.add_argument("--batches", type=int, default=32)
    parser.add_argument("--batch-size", type=int, default=128)
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--warmup", type=int, default=2)
    parser.add_argument("--cpu", type=int, help="pin fresh processes to one allowed Linux CPU")
    parser.add_argument("--allow-driver-warning", action="append", default=[],
                        help="retain and accept only this exact known startup-warning line; never suppress stderr")
    args = parser.parse_args()
    if args.samples < 10 or args.warmup < 1 or args.device < 0 or not 0 <= args.memory_type < 32 or not 1 <= args.batches <= 4096 or not 1 <= args.batch_size <= 256:
        parser.error("samples>=10, warmup>=1, device>=0, valid workload bounds required")
    env = environment()
    try:
        selected = select_cases(args.reference, args.case)
    except ValueError as error:
        parser.error(str(error))
    if args.action == "plan":
        print(json.dumps({"cases": selected, "policy": "fresh paired processes, alternating order, no layers, release builds",
                          "build": str(args.build), "run_requires": "explicit action=run and a new --output directory",
                          "qualification": "uncompiled/unexecuted preparation until build and hardware results exist"}, indent=2))
        return
    if args.action == "build":
        build(args.build.resolve(), pinned_toolchain(env), args.reference)
        return
    if not args.output or args.output.exists():
        parser.error("run requires a new --output directory; never overwrite evidence")
    env = pinned_toolchain(env)
    if args.cpu is not None:
        if not hasattr(os, "sched_setaffinity"):
            parser.error("CPU affinity is available only on supported Unix hosts")
        os.sched_setaffinity(0, {args.cpu})
    # Validation, sanitizers, tracing and layers change the measured experiment.
    if any(env.get(k) for k in ("VK_INSTANCE_LAYERS", "VK_LAYER_PATH", "VK_LOADER_LAYERS_ENABLE", "VK_LAYER_ENABLES", "LD_PRELOAD", "DYLD_INSERT_LIBRARIES", "ASAN_OPTIONS", "TSAN_OPTIONS")):
        parser.error("remove validation, instrumentation and injection settings before timing")
    env["VK_LOADER_LAYERS_DISABLE"] = "~all~"
    output = args.output.resolve()
    output.mkdir(parents=True)
    current = manifest(env)
    built = json.loads((args.build / "build.json").read_text())
    if "vulkano" in selected.values() and "vulkano" not in built.get("references", []):
        raise ValueError("build the Vulkano consumer first: bench.py build --reference vulkano")
    if built["source_sha256"] != current["source_sha256"]:
        raise ValueError("sources changed since SDK build; rebuild before timing")
    for path, digest in built["artifact_sha256"].items():
        if hashlib.sha256((args.build / path).read_bytes()).hexdigest() != digest:
            raise ValueError("benchmark artifact changed since build")
    (output / "build-provenance.json").write_text(json.dumps(built, indent=2) + "\n")
    (output / "environment.json").write_text(json.dumps(current, indent=2) + "\n")
    suffix = ".exe" if os.name == "nt" else ""
    binaries = {"vkrust": args.build / "rust/release" / f"vk-bench{suffix}",
                "ash": args.build / "rust/release" / f"ash-bench{suffix}",
                "vma": args.build / f"vma-bench{suffix}"}
    if "vulkano" in selected.values():
        binaries["vulkano"] = args.build / "rust/release" / f"vulkano-bench{suffix}"
    (output / "binaries.json").write_text(json.dumps({key: hashlib.sha256(path.read_bytes()).hexdigest() for key, path in binaries.items()}, indent=2) + "\n")
    results = {}
    for case, reference in selected.items():
        pairs = []
        for sample in range(-args.warmup, args.samples):
            pair = {}
            order = ["vkrust", reference] if sample % 2 == 0 else [reference, "vkrust"]
            for backend in order:
                before = telemetry()
                command = [binaries[backend], "--case", case, "--device", args.device, "--api", args.api,
                           "--memory-type", args.memory_type, "--batches", args.batches, "--batch-size", args.batch_size]
                stdout, stderr = run(command, output, f"{case}-{sample}-{backend}", env)
                (output / f"{case}-{sample}-{backend}.telemetry.json").write_text(json.dumps({"before": before, "after": telemetry()}, indent=2) + "\n")
                diagnostics = [line for line in stderr.splitlines() if line.strip()]
                if any(line not in args.allow_driver_warning for line in diagnostics):
                    raise ValueError("runtime diagnostics invalidate the timing sample; see retained stderr")
                if case == "virtual" and diagnostics:
                    raise ValueError("host-only virtual workload emitted unexpected diagnostics")
                record = json.loads(stdout)
                keys = verify_sample(record, case, backend, args)
                pair["vkrust" if backend == "vkrust" else "reference"] = record
            if case != "virtual" and any(pair["vkrust"]["identity"][key] != pair["reference"]["identity"][key] for key in IDENTITY):
                raise ValueError("paired processes used different native contexts")
            if sample >= 0:
                pairs.append(pair)
        results[case] = {"reference": reference, "samples": args.samples,
                         "comparison_scope": "checked managed API vs caller-managed native API" if reference == "vulkano" else "native public APIs",
                         "summary": summarize(pairs, keys), "pairs": pairs}
        (output / "results.json").write_text(json.dumps({"schema": "vk.sdk.benchmark.v1", "results": results,
            "accepted_exact_startup_warning_lines": args.allow_driver_warning,
            "ratio_interpretation": "vk/reference; below 1 is faster for this workload only",
            "limits": "process aggregate host latency; not per-call p99, GPU throughput, production qualification or full VMA parity"}, indent=2) + "\n")
    print(f"Retained checked paired samples: {output}")


if __name__ == "__main__":
    main()
