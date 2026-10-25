#!/usr/bin/env python3
"""Replay pinned VMA virtual-TLSF traces against the public Rust API and interval model."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import subprocess
import fetch_donor

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def checked(args, **kwargs):
    try:
        return subprocess.run(args, check=True, text=True, capture_output=True, timeout=600, **kwargs)
    except subprocess.CalledProcessError as error:
        raise SystemExit(f"command failed: {args}\n{error.stdout[-8000:]}\n{error.stderr[-8000:]}") from error


def replay(binary, trace, environment):
    result = checked([str(binary)], input=trace, env=environment)
    if result.stderr.strip():
        raise SystemExit(f"unexpected replay diagnostics from {binary}:\n{result.stderr}")
    return result.stdout


def corpus(seed, capacity, strategy, steps):
    rng = random.Random(seed)
    lines = [f"b {capacity}"]
    ids, next_id = [], 0
    boundaries = [1, 7, 8, 9, 63, 64, 65, 248, 249, 256, 257, 511, 512, 513, 1023, 1024, 1025]
    for step in range(steps):
        if step % 251 == 250:
            lines.append("c")
            ids.clear()
        elif ids and rng.randrange(3) == 0:
            identity = ids.pop(rng.randrange(len(ids)))
            lines.append(f"f {identity}")
        else:
            size = rng.choice(boundaries) if rng.randrange(2) else rng.randrange(1, 4097)
            if capacity > 1 << 32 and rng.randrange(4) == 0:
                size <<= 20
            lines.append(f"a {next_id} {size} {1 << rng.randrange(13)} {strategy}")
            ids.append(next_id)
            next_id += 1
    lines.append("c")
    return "\n".join(lines) + "\n"


def model(trace, output):
    live, capacity = {}, None
    commands, rows = trace.splitlines(), output.splitlines()
    assert len(commands) == len(rows)
    for step, (command, row) in enumerate(zip(commands, rows)):
        words, values = command.split(), row.split()
        op = words[0]
        assert values[0] == op, step
        if op == "b":
            capacity = int(words[1])
        elif op == "a":
            identity, size, alignment, strategy = map(int, words[1:])
            assert int(values[1]) == identity and int(values[4]) == size
            cursor, first_fit = 0, None
            for start, end in sorted((o, o + n) for o, n in live.values()) + [(capacity, capacity)]:
                aligned = (cursor + alignment - 1) // alignment * alignment
                if aligned <= start and size <= start - aligned:
                    first_fit = aligned
                    break
                cursor = end
            if int(values[2]):
                offset = int(values[3])
                assert first_fit is not None and offset % alignment == 0 and offset + size <= capacity, (step, command)
                if strategy == 3:
                    assert offset == first_fit, (step, command)
                assert all(offset + size <= o or o + n <= offset for o, n in live.values()), step
                live[identity] = offset, size
            else:
                assert first_fit is None, (step, command, "false exhaustion")
        elif op == "f":
            assert values[1] == words[1]
            live.pop(int(words[1]), None)
        elif op == "c":
            live.clear()
        else:
            raise AssertionError(command)
        cursor, spans = 0, []
        for start, end in sorted((o, o + n) for o, n in live.values()) + [(capacity, capacity)]:
            if start > cursor:
                spans.append(start - cursor)
            cursor = end
        allocated = sum(n for _, n in live.values())
        assert list(map(int, values[-5:])) == [len(live), allocated, capacity - allocated, len(spans), max(spans, default=0)], (step, command, row)


def main():
    if not __debug__:
        raise SystemExit("verification requires Python assertions; do not use -O")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--headers', type=Path, required=True)
    parser.add_argument('--package', type=Path, default=ROOT)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/verify_memory/virtual')
    parser.add_argument('--steps', type=int, default=2000)
    parser.add_argument('--sanitize', action='store_true')
    args = parser.parse_args()
    if args.steps <= 0:
        parser.error('--steps must be positive')
    donor = ROOT / 'target/verify_memory/donor' / fetch_donor.COMMIT
    for name, digest in fetch_donor.FILES.items():
        assert hashlib.sha256((donor / name).read_bytes()).hexdigest() == digest, name
    headers = args.headers.resolve()
    for line in (ROOT / 'registry/ABI_SHA256SUMS').read_text().splitlines():
        digest, name = line.split()
        assert hashlib.sha256((headers / name).read_bytes()).hexdigest() == digest, name
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    if args.sanitize:
        environment['ASAN_OPTIONS'] = 'halt_on_error=1:abort_on_error=1:detect_leaks=1'
        environment['UBSAN_OPTIONS'] = 'halt_on_error=1:print_stacktrace=1'
    cpp = os.environ.get('CXX', 'c++')
    flags = ['-std=c++17', '-O1', '-g0']
    if args.sanitize:
        flags += ['-fsanitize=address,undefined', '-fno-omit-frame-pointer']
    checked([cpp, *flags, '-I', str(headers), '-I', str(donor / 'include'), str(HERE / 'oracle.cpp'), '-o', str(root / 'vma-trace')])
    consumer = root / 'consumer'
    consumer.mkdir(exist_ok=True)
    manifest = '[package]\nname="vk-virtual-trace"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(args.package.resolve())) + ',features=["allocator"]}\n[profile.dev]\ndebug=0\nincremental=false\n[[bin]]\nname="vk-virtual-trace"\npath=' + json.dumps(str(HERE / 'trace.rs')) + '\n'
    (consumer / 'Cargo.toml').write_text(manifest)
    checked(['cargo', 'build', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'rust-target')])
    count, operations = 0, 0
    for strategy in range(4):
        for capacity in [257, 4099, 65537, (1 << 36) + 3]:
            for seed in [0xacbb9391, 0x3aa92122]:
                trace = corpus(seed, capacity, strategy, args.steps)
                name = f'{strategy}-{capacity}-{seed}'
                (root / (name + '.trace')).write_text(trace)
                c = replay(root / 'vma-trace', trace, environment)
                rust = replay(root / 'rust-target/debug/vk-virtual-trace', trace, environment)
                (root / (name + '.vma')).write_text(c)
                (root / (name + '.rust')).write_text(rust)
                if c != rust:
                    for step, (left, right) in enumerate(zip(c.splitlines(), rust.splitlines())):
                        if left != right:
                            raise AssertionError((name, step, trace.splitlines()[step], left, right))
                    raise AssertionError((name, 'output length mismatch'))
                model(trace, rust)
                count += 1
                operations += len(trace.splitlines())
    # Reproduce the pinned donor's narrow first-map loss above class 31.
    # This intentional full-u64 correction is kept separate from parity traces.
    maximum = (1 << 64) - 1
    large = f'b {maximum}\na 0 {1 << 55} 1 0\na 1 {maximum - (1 << 55) - 1} 1 0\nf 0\na 2 {1 << 54} {1 << 63} 1\nc\n'
    (root / 'large-domain.trace').write_text(large)
    c = replay(root / 'vma-trace', large, environment)
    rust = replay(root / 'rust-target/debug/vk-virtual-trace', large, environment)
    (root / 'large-domain.vma').write_text(c)
    (root / 'large-domain.rust').write_text(rust)
    model(large, rust)
    assert c.splitlines()[4].split()[:3] == ['a', '2', '0']
    assert rust.splitlines()[4].split()[:4] == ['a', '2', '1', '0']
    report = {'donor': fetch_donor.COMMIT, 'package': str(args.package.resolve()), 'traces': count, 'operations': operations, 'sanitize_cpp': args.sanitize, 'large_domain_regressions': 1, 'runtime_diagnostics': 0,
              'cxx': checked([cpp, '--version']).stdout.splitlines()[0], 'rustc': checked(['rustc', '--version']).stdout.strip()}
    (root / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
