#!/usr/bin/env python3
"""Plan or run a bounded, policy-matched allocator benchmark matrix."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics
import sys

import bench
import qualify


def cells(suite):
    full = []

    def add(case, algorithm="tlsf", trace="reverse", strategy="default", mapping="none", batch_size=128):
        cell = dict(case=case, algorithm=algorithm, trace=trace, strategy=strategy,
                    mapping=mapping, batch_size=batch_size)
        if cell not in full:
            full.append(cell)

    for case in ("virtual", "buffer-pool", "image-pool"):
        for algorithm in ("tlsf", "linear"):
            for trace in ("reverse", "fifo", "shuffle", "churn"):
                add(case, algorithm, trace)
    for case in ("buffer-separate", "image-separate"):
        for trace in ("reverse", "churn"):
            add(case, trace=trace)
    for algorithm in ("tlsf", "linear"):
        for trace in ("reverse", "churn"):
            add("buffer-pool", algorithm, trace, mapping="cycle")
    for case in ("virtual", "buffer-pool"):
        for strategy in ("min-memory", "min-time", "min-offset"):
            add(case, trace="churn", strategy=strategy)
        for count in (1, 32, 256):
            add(case, trace="reverse" if count == 1 else "churn", batch_size=count)
    if suite == "full":
        return full
    return [cell for cell in full if
            (cell["batch_size"] == 128 and cell["strategy"] == "default" and
             ((cell["case"], cell["algorithm"], cell["trace"], cell["mapping"]) in (
                 ("virtual", "tlsf", "reverse", "none"),
                 ("virtual", "tlsf", "churn", "none"),
                 ("virtual", "linear", "fifo", "none"),
                 ("buffer-pool", "tlsf", "churn", "none"),
                 ("buffer-pool", "linear", "fifo", "none"),
                 ("image-pool", "tlsf", "churn", "none"),
                 ("image-separate", "tlsf", "reverse", "none"),
                 ("buffer-pool", "tlsf", "churn", "cycle"))))]


def validate_record(stdout, stderr, case):
    if qualify.DIAGNOSTICS.search(stdout + stderr):
        raise ValueError("validation diagnostics invalidate the workload")
    if case != "virtual" and qualify.LAYER_ACTIVE not in stderr:
        raise ValueError("loader did not confirm validation layer insertion")
    return json.loads(stdout)


def validate(args, selected, output):
    """Execute the real benchmark consumers with layers; never report timings."""
    env = bench.pinned_toolchain(bench.environment())
    env = qualify.validation_environment(env, output, args.layer_manifest.resolve(), args.validation)
    built = json.loads((args.build / "build.json").read_text())
    if bench.manifest(env)["source_sha256"] != built["source_sha256"]:
        raise ValueError("sources changed since SDK build")
    for path, digest in built["artifact_sha256"].items():
        if hashlib.sha256((args.build / path).read_bytes()).hexdigest() != digest:
            raise ValueError("benchmark artifact changed since build")
    (output / "build-provenance.json").write_text(json.dumps(built, indent=2) + "\n")
    record = {"schema": "vk.sdk.memory_validation.v1", "mode": args.validation,
              "limits": "validation execution only; virtual cells are host-only; instrumented times are not benchmark evidence",
              "cells": []}
    failed = False
    for index, cell in enumerate(selected):
        report = {**cell, "status": "running", "native": cell["case"] != "virtual"}
        record["cells"].append(report)
        pair = {}
        try:
            for backend in ("vkrust", "vma"):
                suffix = ".exe" if sys.platform == "win32" else ""
                binary = args.build / (("rust/release/vk-bench" if backend == "vkrust" else "vma-bench") + suffix)
                command = [binary, "--api", args.api, "--memory-type", args.memory_type,
                           "--batches", 1, *[item for key, value in cell.items() for item in ("--" + key.replace("_", "-"), value)]]
                stdout, stderr = bench.run(command, output, f"{index:02d}-{backend}", env)
                sample = validate_record(stdout, stderr, cell["case"])
                workload = argparse.Namespace(**cell, batches=1, api=args.api, device=0, memory_type=args.memory_type)
                bench.verify_sample(sample, cell["case"], backend, workload)
                pair[backend] = sample
            if report["native"] and any(pair["vkrust"]["identity"][key] != pair["vma"]["identity"][key] for key in bench.IDENTITY):
                raise ValueError("reference and vk used different native identities")
            report["status"] = "passed"
        except Exception as error:
            report.update(status="failed", error=str(error))
            failed = True
        (output / "results.json").write_text(json.dumps(record, indent=2) + "\n")
        print(f"{index:02d}-{cell['case']}-{cell['algorithm']}-{cell['trace']}: {report['status']}", flush=True)
    if failed:
        raise RuntimeError("validation failures retained in matrix evidence")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("plan", "run", "validate"), nargs="?", default="plan")
    parser.add_argument("--suite", choices=("smoke", "full"), default="smoke")
    parser.add_argument("--build", type=Path, default=bench.ROOT / "target/sdk/build")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--api", choices=("1.1", "1.2", "1.3", "1.4"), default="1.4")
    parser.add_argument("--memory-type", type=int, default=0)
    parser.add_argument("--cpu", type=int)
    parser.add_argument("--batches", type=int, default=64)
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--warmup", type=int, default=3)
    parser.add_argument("--validation", choices=("core", "sync"), default="core")
    parser.add_argument("--layer-manifest", type=Path)
    args = parser.parse_args()
    selected = cells(args.suite)
    if args.action == "plan":
        print(json.dumps({"cells": selected, "count": len(selected),
            "scope": "VMA comparison: placement, native buffers/images and host map/write/unmap",
            "not_covered": ["multi-block pressure/exhaustion", "cache flush/invalidate", "relocation timing",
                            "contention timing", "external-memory timing", "RSS/native-call counters", "cross-device results"]}, indent=2))
        return
    if not args.output or args.output.exists():
        parser.error("run requires a new --output directory")
    if args.samples < 10 or args.warmup < 1 or not 1 <= args.batches <= 4096 or not 0 <= args.memory_type < 32:
        parser.error("samples>=10, warmup>=1 and valid workload bounds required")
    output = args.output.resolve()
    output.mkdir(parents=True)
    args.build = args.build.resolve()
    if args.action == "validate":
        if not args.layer_manifest:
            parser.error("validate requires --layer-manifest")
        validate(args, selected, output)
        return
    record = {"schema": "vk.sdk.memory_matrix.v1", "cells": [dict(cell, status="pending") for cell in selected]}
    manifest = output / "results.json"

    def save():
        manifest.write_text(json.dumps(record, indent=2) + "\n")

    save()
    for index, cell in enumerate(record["cells"]):
        name = f"{index:02d}-{cell['case']}-{cell['algorithm']}-{cell['trace']}-{cell['strategy']}-{cell['mapping']}-{cell['batch_size']}"
        cell.update(status="running", evidence=name)
        save()
        command = [sys.executable, bench.SDK / "tool/bench.py", "run", "--reference", "vma",
                   "--build", args.build.resolve(), "--output", output / name, "--api", args.api,
                   "--memory-type", args.memory_type, "--batches", args.batches,
                   "--samples", args.samples, "--warmup", args.warmup]
        if args.cpu is not None:
            command += ["--cpu", args.cpu]
        for key in ("case", "algorithm", "trace", "strategy", "mapping", "batch_size"):
            command += ["--" + key.replace("_", "-"), cell[key]]
        try:
            bench.run(command, output, name, bench.environment())
            result = json.loads((output / name / "results.json").read_text())["results"][cell["case"]]
            ratios = []
            for pair in result["pairs"]:
                vk = pair["vkrust"]["measurements"]
                vma = pair["reference"]["measurements"]
                ratios.append((vk["allocate_ns"] + vk["free_ns"]) / (vma["allocate_ns"] + vma["free_ns"]))
            cell.update(status="passed", summary=result["summary"], combined={
                "vk_over_vma": statistics.median(ratios), "ci95": bench.interval(ratios)})
        except Exception as error:
            cell.update(status="failed", error=str(error))
            save()
            raise
        save()
        print(f"{name}: checked, combined vk/VMA {cell['combined']['vk_over_vma']:.3f}", flush=True)


if __name__ == "__main__":
    main()
