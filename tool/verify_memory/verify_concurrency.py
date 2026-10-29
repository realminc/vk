#!/usr/bin/env python3
"""Compare deterministic backing-concurrency outcomes with the pinned VMA donor."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import fetch_donor

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
EXPECTED = {
    "independent-map": [1, 0, 0],
    "independent-bind": [1, 0, 0],
    "same-backing": [1, 2, 2],
}


def outcomes(output):
    records = {}
    for line in output.splitlines():
        if not line.startswith("CONCURRENCY "):
            continue
        fields = line.split()
        if len(fields) != 5 or fields[1] in records:
            raise ValueError(f"invalid/duplicate concurrency evidence: {line}")
        records[fields[1]] = [int(value) for value in fields[2:]]
    if records != EXPECTED:
        raise ValueError(f"incomplete or incorrect concurrency evidence: {records}")
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--headers", type=Path, required=True)
    parser.add_argument("--package", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=ROOT / "target/verify_memory/concurrency")
    parser.add_argument("--repeat", type=int, default=4)
    parser.add_argument("--sanitize", action="store_true")
    args = parser.parse_args()
    if args.repeat <= 0:
        parser.error("--repeat must be positive")
    donor = ROOT / "target/verify_memory/donor" / fetch_donor.COMMIT
    for name, expected in fetch_donor.FILES.items():
        if hashlib.sha256((donor / name).read_bytes()).hexdigest() != expected:
            raise ValueError(f"pinned donor mismatch: {name}")
    headers = args.headers.resolve()
    for line in (ROOT / "registry/ABI_SHA256SUMS").read_text().splitlines():
        expected, name = line.split()
        if hashlib.sha256((headers / name).read_bytes()).hexdigest() != expected:
            raise ValueError(f"pinned header mismatch: {name}")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    records = []
    environment = os.environ.copy()
    if args.sanitize:
        environment.update(ASAN_OPTIONS="halt_on_error=1:abort_on_error=1:detect_leaks=1",
                           UBSAN_OPTIONS="halt_on_error=1:print_stacktrace=1")

    def run(name, command, cwd=ROOT):
        result = subprocess.run(command, cwd=cwd, env=environment, text=True,
                                capture_output=True, timeout=600)
        for stream in ("stdout", "stderr"):
            (output / f"{name}.{stream}").write_text(getattr(result, stream))
        records.append(dict(name=name, command=command, exit_code=result.returncode))
        (output / "commands.json").write_text(json.dumps(records, indent=2) + "\n")
        if result.returncode:
            raise RuntimeError(f"{name} failed; inspect {output}:\n{result.stdout[-4000:]}\n{result.stderr[-4000:]}")
        return result

    binary = output / "vma-concurrency"
    flags = ["-std=c++17", "-O1", "-g0", "-Wall", "-Wextra", "-Werror", "-pthread"]
    if args.sanitize:
        flags += ["-fsanitize=address,undefined", "-fno-omit-frame-pointer"]
    run("compile", [os.environ.get("CXX", "clang++"), *flags,
                    "-isystem", str(headers), "-isystem", str(donor / "include"),
                    str(HERE / "concurrency_oracle.cpp"), "-o", str(binary)])
    for iteration in range(args.repeat):
        native = run(f"vma-{iteration}", [str(binary)])
        if native.stderr.strip():
            raise ValueError(f"unexpected donor runtime diagnostics: {native.stderr}")
        rust = run(f"rust-{iteration}", ["cargo", "test", "-p", "vkrust", "--lib",
                   "--all-features", "--locked", "synchronized::pools::native::",
                   "--", "--nocapture", "--test-threads=1"], args.package.resolve())
        if outcomes(native.stdout) != outcomes(rust.stdout):
            raise ValueError(f"concurrency outcome mismatch at iteration {iteration}")
    paths = [HERE / "concurrency_oracle.cpp", Path(__file__),
             ROOT / "test/rs/mem/synchronized/test_native.rs"]
    source_paths = list((args.package / "src/rs/mem").rglob("*.rs"))
    report = dict(donor=fetch_donor.COMMIT, comparisons=args.repeat * len(EXPECTED),
                  repeats=args.repeat, outcomes=EXPECTED, sanitize_cpp=args.sanitize,
                  source_sha256={str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
                  runtime_sha256={str(p.relative_to(args.package)): hashlib.sha256(p.read_bytes()).hexdigest() for p in source_paths})
    (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"Verified {report['comparisons']} pinned-donor concurrency comparisons; diagnostics: zero")


if __name__ == "__main__":
    main()
