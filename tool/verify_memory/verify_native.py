#!/usr/bin/env python3
"""Compare pinned native VMA and the public Rust allocator over checked synthetic PFNs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import subprocess

import fetch_donor
from verify_virtual import checked

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
WHOLE = (1 << 64) - 1


def record_replay(binary, trace, environment, output):
    """Retain stdout, stderr and exit code before accepting any replay result."""
    result = subprocess.run([str(binary)], input=trace, env=environment, text=True,
                            capture_output=True, timeout=600, check=False)
    output.write_text(result.stdout)
    Path(str(output) + '.stderr').write_text(result.stderr)
    Path(str(output) + '.process.json').write_text(json.dumps({'binary': str(binary), 'exit_code': result.returncode}) + '\n')
    if result.returncode or result.stderr.strip():
        raise SystemExit(f'replay failed or emitted diagnostics; retained evidence: {output}')
    return result.stdout


def corpus(atom, coherent, separate, strategy, seed, steps):
    lines = [f'b {atom} {int(coherent)} {int(separate)}']
    oom = -1 if seed == 0x3aa92122 else -2
    def allocate(identity, size=257, alignment=8, bits=1, preferred=0, persistent=0):
        lines.append(f'a {identity} {size} {alignment} {bits} 2 {preferred} {persistent} {strategy}')
    # Failure before first block, followed by a clean retry.
    lines.append('x 3 0 -2')
    allocate(6)
    lines.append(f'x 0 0 {oom}')
    allocate(0)
    lines.append('x -1 0 0')
    allocate(1)
    allocate(2, size=65, persistent=1)
    lines += ['m 1', 'm 2', 'h 1 1 256', f'i 2 0 {WHOLE}', 'u 2', 'f 2', 'u 1']
    # Keep a sentinel alive, making reuse/mapping failures observable independently
    # of VMA's intentional empty-block cache and Rust's immediate empty release.
    lines.append('x 1 0 -5')
    lines += ['m 1']
    allocate(5, size=129, persistent=1)
    lines += ['x -1 0 0', 'm 1', 'u 1']
    lines.append('x 2 0 -2')
    allocate(3, size=129, persistent=1)
    lines.append('x -1 0 0')
    allocate(4, size=129, persistent=1)
    lines += ['h 4 1 128', 'f 4']
    rng = random.Random(seed)
    live, identity = [], 10
    for _ in range(steps):
        if live and (len(live) >= 16 or rng.randrange(3) == 0):
            lines.append(f'f {live.pop(rng.randrange(len(live)))}')
        else:
            size = rng.choice([1, 7, 8, 9, 63, 64, 65, 127, 128, 129, 255, 256, 257])
            allocate(identity, size, 1 << rng.randrange(9), persistent=rng.randrange(2))
            lines += [f'm {identity}', f'h {identity} 0 {WHOLE}', f'i {identity} {size - 1} 1', f'u {identity}']
            live.append(identity)
            identity += 1
    lines += [f'f {identity}' for identity in live] + ['f 1']
    if separate:
        # Type 1 wins preferred HOST_CACHED; native OOM must retry type 0.
        lines.append(f'x 0 1 {oom}')
        allocate(identity + 1, bits=7, preferred=8, persistent=1)
        lines += [f'f {identity + 1}']
        allocate(identity + 2, bits=2, preferred=8)  # no compatible fallback in mask
        lines.append('x -1 0 0')
        allocate(identity + 3, bits=7, preferred=8)
        lines += [f'f {identity + 3}']
    lines.append('c')
    return '\n'.join(lines) + '\n'


def model(trace, output, donor):
    """Independent interval, PFN lifecycle, mapping, cache and accounting model.

    Native addresses and bucket choices are deliberately not compared. Empty
    blocks may remain only in VMA until explicit pool/allocator destruction.
    """
    config, *commands = trace.splitlines()
    _, atom, coherent, separate = config.split()
    atom, coherent, separate = int(atom), bool(int(coherent)), bool(int(separate))
    flags = [2 | (4 if coherent else 0), 2 | 8 | (4 if coherent else 0), 1]
    memory, buffers, live = {}, {}, {}
    failure, position, summaries = (-1, 0, 0), 0, []
    rows = output.splitlines()
    for step, command in enumerate(commands):
        words = command.split()
        op = words[0]
        identity = int(words[1]) if op not in ('x', 'c') else 0
        attempts, caches = [], []
        before_mapped = any(n['mapped'] for n in memory.values())
        while position < len(rows) and rows[position].startswith('e '):
            event = rows[position].split(); position += 1
            kind, values = event[1], list(map(int, event[2:]))
            if kind == 'create':
                buffer, status = values
                assert op == 'a' and status == (failure[2] if failure[0] == 3 else 0), (step, event)
                assert (buffer == 0) == (status != 0)
                if status == 0:
                    assert buffer not in buffers
                    buffers[buffer] = None
            elif kind == 'allocate':
                index, size, status, handle = values
                assert op == 'a' and 0 <= index < 3
                required = int(words[5]) | (2 if int(words[7]) else 0)
                assert int(words[4]) & (1 << index) and flags[index] & required == required
                if separate:
                    candidates = [i for i in range(3) if int(words[4]) & (1 << i) and flags[i] & required == required]
                    candidates.sort(key=lambda i: ((int(words[6]) & ~flags[i]).bit_count(), i))
                    assert len(attempts) < len(candidates) and index == candidates[len(attempts)], (step, 'preferred-type order')
                expected_status = failure[2] if failure[:2] == (0, index) else 0
                assert status == expected_status and (handle == 0) == (status != 0), (step, event)
                assert size == (int(words[2]) if separate else 65536)
                attempts.append(index)
                if status == 0:
                    assert handle not in memory
                    memory[handle] = {'size': size, 'index': index, 'mapped': False}
            elif kind == 'bind':
                buffer, handle, offset, status = values
                assert op == 'a' and buffer in buffers and buffers[buffer] is None
                native = memory[handle]
                assert offset % int(words[3]) == 0 and offset + int(words[2]) <= native['size']
                assert status == (failure[2] if failure[0] == 2 else 0)
                if status == 0:
                    end = offset + int(words[2])
                    reserved_end = (end + atom - 1) // atom * atom if not coherent else end
                    for existing in live.values():
                        if existing['handle'] != handle:
                            continue
                        existing_end = existing['offset'] + existing['size']
                        if not coherent:
                            existing_end = (existing_end + atom - 1) // atom * atom
                        assert reserved_end <= existing['offset'] or existing_end <= offset
                    buffers[buffer] = handle, offset, int(words[2])
            elif kind == 'destroy':
                buffer, = values
                assert buffer in buffers
                del buffers[buffer]
            elif kind == 'map':
                handle, offset, size, status = values
                native = memory[handle]
                assert not native['mapped'] and offset == 0 and size in (WHOLE, native['size'])
                assert status == (failure[2] if failure[0] == 1 else 0)
                if status == 0:
                    native['mapped'] = True
            elif kind == 'unmap':
                handle, = values
                assert memory[handle]['mapped']
                memory[handle]['mapped'] = False
            elif kind == 'free':
                handle, = values
                # VMA frees persistently mapped dedicated memory directly;
                # vkFreeMemory implicitly unmaps. Rust releases its hold first.
                assert not memory[handle]['mapped'] or donor and separate and op in ('a', 'f')
                assert all(binding is None or binding[0] != handle for binding in buffers.values())
                del memory[handle]
            elif kind in ('flush', 'invalidate'):
                handle, offset, size = values
                assert op == ('h' if kind == 'flush' else 'i') and not coherent
                allocation = live[identity]
                assert allocation['explicit'] or allocation['persistent']
                assert handle == allocation['handle'] and memory[handle]['mapped']
                start, length = map(int, words[2:])
                if length == WHOLE:
                    length = allocation['size'] - start
                absolute = allocation['offset'] + start
                begin = absolute // atom * atom
                end = min((absolute + length + atom - 1) // atom * atom, memory[handle]['size'])
                assert (offset, size) == (begin, end - begin), (step, event, allocation)
                caches.append(kind)
            else:
                raise AssertionError(event)
        assert position < len(rows), (step, 'missing result')
        row = rows[position].split(); position += 1
        assert row[:3] == ['r', op, str(identity)] and len(row) == 14, (step, row)
        ok, buffer, handle, offset, size, index, count, used, blocks, native_bytes, mapped = map(int, row[3:])
        assert ok in (0, 1)
        if op == 'a':
            assert identity not in live
            if ok:
                assert buffers[buffer] == (handle, offset, size) and size == int(words[2])
                assert memory[handle]['index'] == index
                assert failure[0] not in (2, 3)
                live[identity] = dict(buffer=buffer, handle=handle, offset=offset, size=size,
                                      explicit=False, persistent=bool(int(words[7])))
            else:
                assert (buffer, handle, offset, size, index) == (0, 0, 0, 0, 0)
                assert not buffers.keys() - {a['buffer'] for a in live.values()}, (step, 'buffer rollback')
                assert failure[0] >= 0, (step, 'unexpected exhaustion')
            if failure[0] in (2, 3):
                assert not ok
            elif failure[0] == 1 and int(words[7]) and (separate or not before_mapped):
                assert not ok
            elif failure[0] == 0:
                candidates = [i for i in range(3) if int(words[4]) & (1 << i) and flags[i] & int(words[5]) == int(words[5])]
                assert bool(ok) == any(i != failure[1] for i in candidates)
            else:
                assert ok
        else:
            assert (buffer, handle, offset, size, index) == (0, 0, 0, 0, 0)
            if op == 'm':
                a = live[identity]
                expected = memory[a['handle']]['mapped'] or failure[0] != 1
                assert bool(ok) == expected
                assert not a['explicit']
                a['explicit'] = bool(ok)
            elif op == 'u':
                assert ok and live[identity]['explicit']
                live[identity]['explicit'] = False
            elif op == 'f':
                assert ok and not live[identity]['explicit']
                del live[identity]
            elif op in ('h', 'i'):
                a = live[identity]
                assert ok and (a['explicit'] or a['persistent'])
                assert len(caches) == (0 if coherent or int(words[3]) == 0 else 1)
            elif op == 'x':
                failure = tuple(map(int, words[1:]))
            elif op == 'c':
                assert ok and not live and not buffers and not memory
            else:
                raise AssertionError(command)
        assert not buffers.keys() - {a['buffer'] for a in live.values()}, (step, 'leaked temporary resource')
        expected_handles = {a['handle'] for a in live.values()}
        if not donor or separate:
            assert set(memory) == expected_handles, (step, 'native rollback/release')
        else:
            assert expected_handles <= memory.keys() and len(memory) <= 1
        for h, native in memory.items():
            intervals = sorted((a['offset'], a['offset'] + a['size']) for a in live.values() if a['handle'] == h)
            cursor = 0
            for begin, end in intervals:
                assert begin >= cursor and end <= native['size'], (step, 'overlap/bounds')
                if not coherent:
                    assert begin % atom == 0, (step, 'shared noncoherent atom')
                    cursor = min((end + atom - 1) // atom * atom, native['size'])
                else:
                    cursor = end
            holds = any(a['handle'] == h and (a['explicit'] or a['persistent']) for a in live.values())
            assert native['mapped'] == holds, (step, 'mapping reference imbalance')
        assert [count, used, blocks, native_bytes, mapped] == [len(live), sum(a['size'] for a in live.values()), len(memory), sum(n['size'] for n in memory.values()), sum(n['mapped'] for n in memory.values())], (step, 'accounting')
        # Compare outcomes/type selection and separate-storage attempt order exactly.
        # Pooled offsets/cache addresses and empty-block retention remain policy-specific.
        summaries.append((op, identity, ok, index if op == 'a' and ok else 0, tuple(attempts) if separate else ()))
    assert position == len(rows) and commands[-1] == 'c'
    return summaries


def main():
    if not __debug__:
        raise SystemExit('verification requires Python assertions; do not use -O')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--headers', type=Path, required=True)
    parser.add_argument('--package', type=Path, default=ROOT)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/verify_memory/native')
    parser.add_argument('--steps', type=int, default=200)
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
    flags = ['-std=c++17', '-O1', '-g0', '-Wall', '-Wextra', '-Werror']
    if args.sanitize:
        flags += ['-fsanitize=address,undefined', '-fno-omit-frame-pointer']
        environment['ASAN_OPTIONS'] = 'halt_on_error=1:abort_on_error=1:detect_leaks=1'
        environment['UBSAN_OPTIONS'] = 'halt_on_error=1:print_stacktrace=1'
    cpp = os.environ.get('CXX', 'c++')
    checked([cpp, *flags, '-isystem', str(headers), '-isystem', str(donor / 'include'), str(HERE / 'native_oracle.cpp'), '-o', str(root / 'vma-native')])
    consumer = root / 'consumer'
    consumer.mkdir(exist_ok=True)
    manifest = '[package]\nname="vk-native-trace"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(args.package.resolve())) + ',features=["allocator"]}\n[profile.dev]\ndebug=0\nincremental=false\n[[bin]]\nname="vk-native-trace"\npath=' + json.dumps(str(HERE / 'native_trace.rs')) + '\n'
    (consumer / 'Cargo.toml').write_text(manifest)
    checked(['cargo', 'build', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'rust-target')])
    traces, operations = 0, 0
    for atom in [1, 64, 256]:
        for coherent in [False, True]:
            for separate in [False, True]:
                for strategy in range(4):
                    for seed in [0x3aa92122, 0xacbb9391]:
                        trace = corpus(atom, coherent, separate, strategy, seed, args.steps)
                        name = f'{atom}-{int(coherent)}-{int(separate)}-{strategy}-{seed}'
                        (root / (name + '.trace')).write_text(trace)
                        outputs = []
                        for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]:
                            output = record_replay(binary, trace, environment, root / (name + '.' + suffix))
                            outputs.append(output)
                        left = model(trace, outputs[0], True)
                        right = model(trace, outputs[1], False)
                        assert left == right, name
                        traces += 1
                        operations += len(trace.splitlines()) - 1
    sources = [HERE / name for name in ['native_oracle.cpp', 'native_trace.rs', 'verify_native.py', 'verify_virtual.py', 'fetch_donor.py']]
    sources += [ROOT / 'test/py/tool/verify_memory' / name for name in ['test_verify_native.py', 'test_verify_virtual.py']]
    hashes = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sources}
    package_hashes = {str(path.relative_to(args.package.resolve())): hashlib.sha256(path.read_bytes()).hexdigest() for path in (args.package.resolve() / 'src/rs').rglob('*.rs')}
    report = dict(donor=fetch_donor.COMMIT, package=str(args.package.resolve()), traces=traces, operations=operations, sanitize_cpp=args.sanitize, runtime_diagnostics=0, source_sha256=hashes, package_source_sha256=package_hashes,
                  cxx=checked([cpp, '--version']).stdout.splitlines()[0], rustc=checked(['rustc', '--version']).stdout.strip())
    (root / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({key: value for key, value in report.items() if not key.endswith('sha256')}))


if __name__ == '__main__':
    main()
