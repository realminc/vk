#!/usr/bin/env python3
"""Compare pinned native VMA and the public Rust allocator over checked synthetic PFNs."""
import argparse
import hashlib
import itertools
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


def corpus(atom, coherent, separate, strategy, seed, steps, custom_pool=False, address=False, automatic_pool=False, existing=False):
    assert not (separate and custom_pool)
    assert not automatic_pool or custom_pool
    mode = 2 if automatic_pool else int(custom_pool)
    extra = f' {mode} {int(address)}' if automatic_pool or address else (' 1' if custom_pool else '')
    lines = [f'b {atom} {int(coherent)} {int(separate)}' + extra]
    oom = -1 if seed == 0x3aa92122 else -2
    def allocate(identity, size=257, alignment=8, bits=1, preferred=0, persistent=0):
        lines.append(f'a {identity} {size} {alignment} {bits} 2 {preferred} {persistent} {strategy}' + (f' 0 0 {int(existing)}' if existing else ''))
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
    if automatic_pool:
        # Two large dedicated allocations remain owned by the same pool even
        # though pooled blocks have a two-block ceiling. Test both BDA usages.
        for identity in [9000, 9001]:
            allocate(identity, size=32769, persistent=1)
            lines += [f'm {identity}', f'h {identity} 1 32768', f'u {identity}']
        lines += ['f 9000', 'f 9001']
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


def linear_corpus(atom, coherent, strategy, seed, address):
    lines = [f'b {atom} {int(coherent)} 0 3 {int(address)}']
    identity = 0
    def allocate(size=257, upper=0, persistent=0):
        nonlocal identity
        token = identity
        identity += 1
        lines.append(f'a {token} {size} 64 1 2 0 {persistent} {strategy} {upper}')
        return token
    def observe(token, size):
        lines.extend([f'm {token}', f'h {token} 0 {WHOLE}', f'i {token} {size - 1} 1', f'u {token}'])
    # Every partial failure unwinds a new upper placement before a clean retry.
    for site in [0, 3, 1, 2]:
        lines.append(f'x {site} 0 -2')
        allocate(upper=1, persistent=1)
        lines.append('x -1 0 0')
        token = allocate(upper=1, persistent=1)
        observe(token, 257)
        lines.append(f'f {token}')
    # Fill one block exactly, then FIFO retire/reuse through two complete wraps.
    queue = [allocate(4096, persistent=i % 2) for i in range(16)]
    for _ in range(32):
        lines.append(f'f {queue.pop(0)}')
        token = allocate(4096, persistent=identity % 2)
        observe(token, 4096)
        queue.append(token)
    lines.extend(f'f {token}' for token in queue)
    rng = random.Random(seed)
    # Coexisting lower/upper stacks, middle frees, LIFO rewind, and all-at-once
    # recovery. Every batch fits independently of its legal free ordering.
    for batch in range(8):
        live = []
        for step in range(48):
            if live and rng.randrange(4) == 0:
                lines.append(f'f {live.pop(rng.randrange(len(live)))}')
            else:
                size = rng.choice([1, 7, 63, 64, 65, 257, 1025])
                token = allocate(size, upper=int(step % 2 == 0), persistent=rng.randrange(2))
                observe(token, size)
                live.append(token)
        lines.extend(f'f {token}' for token in reversed(live))
    lines.append('c')
    return '\n'.join(lines) + '\n'


def mixed_corpus(atom, coherent, granularity, linear, strategy, seed, existing=False):
    """Bounded live sets fit both bucket policies; placement is independently checked."""
    lines = [f'b {atom} {int(coherent)} 0 {3 if linear else 1} 0 {granularity}']
    identity = 0
    def allocate(size, kind, persistent=0):
        nonlocal identity
        token = identity; identity += 1
        lines.append(f'a {token} {size} 64 1 2 0 {persistent} {strategy} 0 {kind}' + (f' {int(existing)}' if existing else ''))
        return token
    def observe(token, size):
        lines.extend([f'm {token}', f'h {token} 0 {WHOLE}', f'i {token} {size - 1} 1', f'u {token}'])
    # Image creation and partial bind/map failures preserve an existing buffer.
    sentinel = allocate(257, 0)
    for site in [3, 1, 2]:
        lines.append(f'x {site} 0 -2')
        allocate(65, 2, 1)
        lines.append('x -1 0 0')
        token = allocate(65, 2, 1)
        observe(token, 65)
        lines.append(f'f {token}')
    lines.append(f'f {sentinel}')
    if linear:
        # Equal whole-page chunks give a valid FIFO ring for every tested limit.
        queue = [allocate(4096, i % 3) for i in range(16)]
        for i in range(32):
            lines.append(f'f {queue.pop(0)}')
            token = allocate(4096, i % 3)
            observe(token, 4096)
            queue.append(token)
        lines.extend(f'f {token}' for token in queue)
    rng = random.Random(seed)
    for _ in range(8):
        live = []
        for index in range(8):
            size = rng.choice([1, 7, 63, 64, 65, 257, 1025])
            token = allocate(size, index % 3, rng.randrange(2))
            observe(token, size)
            live.append(token)
        # Arbitrary holes challenge TLSF's page neighbors; linear retains tombstones.
        rng.shuffle(live)
        for token in live[:4]: lines.append(f'f {token}')
        replacements = [allocate(65, i % 3) for i in range(4)]
        lines.extend(f'f {token}' for token in live[4:] + replacements)
    lines.append('c')
    return '\n'.join(lines) + '\n'


def cache_corpus(atom, coherent, linear, minimum, partial=False):
    lines = [f'b {atom} {int(coherent)} 0 {3 if linear else 1} 0 1 {0 if partial else 65536} {minimum} {2 if partial else 1}']
    def allocate(identity, size):
        lines.append(f'a {identity} {size} 64 1 2 0 0 0')
    if partial:
        for identity in range(3): allocate(identity, 32768)
        lines += ['f 2', 'f 0', 'm 1', f'h 1 0 {WHOLE}', 'u 1', 'f 1']
    else:
        for identity in range(2):
            allocate(identity, 257)
            lines += [f'm {identity}', f'h {identity} 0 {WHOLE}', f'u {identity}', f'f {identity}']
    lines.append('c')
    return '\n'.join(lines) + '\n'


UPPER_GRANULARITY_REGRESSION = (
    'b 1 1 0 3 0 512\n'
    'a 0 128 1 1 2 0 0 0 1 0\n'
    'a 1 256 1 1 2 0 0 0 1 2\n'
    'f 1\nf 0\nc\n')


def check_linear_pool_count(quick, detailed, live):
    # This corpus wraps a full single-block FIFO ring one region at a time.
    # The pinned quick-stat bug omits the complete wrapped prefix; validate that
    # precise deviation rather than allowing arbitrary undercounts elsewhere.
    offsets = [value['offset'] for value in live.values()]
    split = next((i + 1 for i, (a, b) in enumerate(zip(offsets, offsets[1:])) if a > b), None)
    assert split is not None, 'donor quick count deviation outside active ring'
    assert detailed == len(live) and quick == split, 'donor ring quick/detailed count'


def model(trace, output, donor):
    """Independent interval, PFN lifecycle, mapping, cache and accounting model.

    Native addresses and bucket choices are deliberately not compared. Empty
    blocks may remain in VMA or an explicitly selected Rust custom pool until
    pool destruction. Default Rust blocks still release their final region.
    """
    config, *commands = trace.splitlines()
    settings = config.split()
    assert 4 <= len(settings) <= 10 and settings[0] == 'b'
    if len(settings) >= 5:
        assert settings[4] in ('0', '1', '2', '3')
    if len(settings) >= 6:
        assert settings[5] in ('0', '1')
    _, atom, coherent, separate = settings[:4]
    atom, coherent, separate = int(atom), bool(int(coherent)), bool(int(separate))
    custom_pool = len(settings) >= 5 and settings[4] != '0'
    automatic_pool = len(settings) >= 5 and settings[4] == '2'
    linear_pool = len(settings) >= 5 and settings[4] == '3'
    address = len(settings) >= 6 and settings[5] == '1'
    granularity = int(settings[6]) if len(settings) >= 7 else 1
    heap_limit = int(settings[7]) if len(settings) >= 8 else 0
    minimum = int(settings[8]) if len(settings) >= 9 else 0
    maximum = int(settings[9]) if len(settings) >= 10 else (2 if automatic_pool else 1)
    assert minimum >= 0 and (maximum == 0 or minimum <= maximum)
    assert granularity > 0 and granularity & (granularity - 1) == 0
    assert not (separate and custom_pool)
    flags = [2 | (4 if coherent else 0), 2 | 8 | (4 if coherent else 0), 1]
    memory, buffers, live = {}, {}, {}
    failure, position, summaries = (-1, 0, 0), 0, []
    rows = output.splitlines()
    for step, command in enumerate(commands):
        words = command.split()
        op = words[0]
        identity = int(words[1]) if op not in ('x', 'c') else 0
        resource_class = int(words[10]) if op == 'a' and len(words) > 10 else 0
        assert resource_class in (0, 1, 2)
        raw_resource = op == 'a' and len(words) > 11 and words[11] == '2'
        existing_resource = raw_resource or op == 'a' and len(words) > 11 and words[11] == '1'
        if raw_resource: resource_class = 4
        elif existing_resource and resource_class != 0: resource_class = 3
        actual_size = int(words[2]) if op == 'a' else 0
        # Pinned native TLSF rounds optimal-image extents for limits <=256.
        # Rust keeps exact requirement/atom extents and free neighbor padding.
        if donor and not separate and not linear_pool and resource_class in (2, 3, 4) and 1 < granularity <= 256:
            actual_size = (actual_size + granularity - 1) // granularity * granularity
        attempts, caches, pool_counts = [], [], []
        before_mapped = any(n['mapped'] for n in memory.values())
        before_bytes = sum(n['size'] for n in memory.values())
        before_pooled = [h for h, n in memory.items() if not n['dedicated']]
        before_live_handles = {a['handle'] for a in live.values()}
        before_empty = [h for h in before_pooled if h not in before_live_handles]
        freed_handle = live[identity]['handle'] if op == 'f' else None
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
                if address:
                    index, size, status, handle, allocation_flags = values
                    dedicated = separate or automatic_pool and size == actual_size
                    expected_flags = 2 if not dedicated or raw_resource or resource_class == 0 and (existing_resource or identity % 2 == 0) else 0
                    assert allocation_flags == expected_flags, (step, 'device address allocation flags')
                else:
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
                if automatic_pool:
                    assert size == int(words[2]) or size in (8192, 16384, 32768, 65536), (step, 'automatic block size')
                else:
                    assert size == (int(words[2]) if separate else 65536)
                attempts.append(index)
                if status == 0:
                    assert handle not in memory
                    memory[handle] = {'size': size, 'index': index, 'mapped': False,
                                      'dedicated': separate or automatic_pool and size == actual_size}
            elif kind == 'bind':
                buffer, handle, offset, status = values
                assert op == 'a' and buffer in buffers and buffers[buffer] is None
                native = memory[handle]
                assert offset % int(words[3]) == 0 and offset + int(words[2]) <= native['size']
                if custom_pool:
                    assert offset % 256 == 0, (step, 'custom pool alignment')
                assert status == (failure[2] if failure[0] == 2 else 0)
                if status == 0:
                    end = offset + actual_size
                    reserved_end = (end + atom - 1) // atom * atom if not coherent else end
                    for existing in live.values():
                        if existing['handle'] != handle:
                            continue
                        existing_end = existing['offset'] + existing['size']
                        if not coherent:
                            existing_end = (existing_end + atom - 1) // atom * atom
                        assert reserved_end <= existing['offset'] or existing_end <= offset
                        if resource_class >= 3 or existing['class'] >= 3 or (resource_class == 2) != (existing['class'] == 2):
                            assert ((end - 1) // granularity < existing['offset'] // granularity or
                                    (existing_end - 1) // granularity < offset // granularity), (step, 'buffer/image granularity conflict')
                    buffers[buffer] = handle, offset, actual_size
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
                assert not memory[handle]['mapped'] or donor and memory[handle]['dedicated'] and op in ('a', 'f')
                assert all(binding is None or binding[0] != handle for binding in buffers.values())
                del memory[handle]
            elif kind == 'linear_pool_count':
                quick, detailed = values
                assert donor and linear_pool and 0 <= quick < detailed, (step, 'donor count regression scope')
                pool_counts.append((quick, detailed))
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
                assert buffers[buffer] == (handle, offset, size) and size == actual_size
                assert memory[handle]['index'] == index
                assert failure[0] not in (2, 3)
                live[identity] = dict(buffer=buffer, handle=handle, offset=offset, size=size,
                                      explicit=False, persistent=bool(int(words[7])), **{'class': resource_class})
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
        for quick, detailed in pool_counts:
            check_linear_pool_count(quick, detailed, live)
        expected_handles = {a['handle'] for a in live.values()}
        if (not donor and not custom_pool) or separate:
            assert set(memory) == expected_handles, (step, 'native rollback/release')
        else:
            assert expected_handles <= memory.keys()
            pooled = {h for h, n in memory.items() if not n['dedicated']}
            assert maximum == 0 or len(pooled) <= maximum, (step, 'pool block ceiling')
            assert op == 'c' or len(pooled) >= minimum, (step, 'pool minimum floor')
            assert all(not n['dedicated'] or h in expected_handles for h, n in memory.items()), (step, 'dedicated storage leak')
        if heap_limit:
            assert sum(n['size'] for n in memory.values()) <= heap_limit, (step, 'local heap ceiling')
        if op == 'f' and custom_pool and len(before_pooled) > minimum:
            if heap_limit and before_bytes >= heap_limit and freed_handle not in expected_handles:
                assert freed_handle not in memory, (step, 'pressured empty spare retained')
            elif len(settings) >= 10 and freed_handle in expected_handles and before_pooled[-1] in before_empty:
                assert before_pooled[-1] not in memory, (step, 'partial free retained last empty spare')
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
    cpp = os.environ.get('CXX', 'clang++')
    checked([cpp, *flags, '-isystem', str(headers), '-isystem', str(donor / 'include'), str(HERE / 'native_oracle.cpp'), '-o', str(root / 'vma-native')])
    consumer = root / 'consumer'
    consumer.mkdir(exist_ok=True)
    manifest = '[package]\nname="vk-native-trace"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nvk={package="vkrust",path=' + json.dumps(str(args.package.resolve())) + ',features=["allocator"]}\n[profile.dev]\ndebug=0\nincremental=false\n[[bin]]\nname="vk-native-trace"\npath=' + json.dumps(str(HERE / 'native_trace.rs')) + '\n'
    (consumer / 'Cargo.toml').write_text(manifest)
    checked(['cargo', 'build', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'rust-target')])
    traces, operations, custom_pool_traces, address_traces, automatic_pool_traces = 0, 0, 0, 0, 0
    for atom, coherent, separate, pool_mode, address in itertools.product([1, 64, 256], [False, True], [False, True], range(3), [False, True]):
        custom_pool, automatic_pool = pool_mode != 0, pool_mode == 2
        if separate and custom_pool:
            continue
        for strategy in range(4):
            for seed in [0x3aa92122, 0xacbb9391]:
                trace = corpus(atom, coherent, separate, strategy, seed, args.steps, custom_pool, address, automatic_pool)
                name = f'{atom}-{int(coherent)}-{int(separate)}-{strategy}-{seed}' + ('-auto-pool' if automatic_pool else '-custom-pool' if custom_pool else '') + ('-address' if address else '')
                (root / (name + '.trace')).write_text(trace)
                outputs = []
                for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]:
                    output = record_replay(binary, trace, environment, root / (name + '.' + suffix))
                    outputs.append(output)
                left = model(trace, outputs[0], True)
                right = model(trace, outputs[1], False)
                assert left == right, name
                traces += 1
                custom_pool_traces += int(custom_pool)
                address_traces += int(address)
                automatic_pool_traces += int(automatic_pool)
                operations += len(trace.splitlines()) - 1
    linear_pool_traces = 0
    donor_linear_count_regressions = 0
    for atom, coherent, address, strategy, seed in itertools.product([1, 64, 256], [False, True], [False, True], range(4), [0x3aa92122, 0xacbb9391]):
        trace = linear_corpus(atom, coherent, strategy, seed, address)
        name = f'linear-{atom}-{int(coherent)}-{strategy}-{seed}' + ('-address' if address else '')
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        donor_linear_count_regressions += outputs[0].count("e linear_pool_count ")
        traces += 1
        linear_pool_traces += 1
        custom_pool_traces += 1
        address_traces += int(address)
        operations += len(trace.splitlines()) - 1
    mixed_traces = 0
    for atom, coherent, granularity, linear, strategy, seed in itertools.product(
            [1, 64], [False, True], [1, 64, 256, 512, 4096], [False, True], range(4), [0x3aa92122, 0xacbb9391]):
        trace = mixed_corpus(atom, coherent, granularity, linear, strategy, seed)
        name = f'mixed-{atom}-{int(coherent)}-{granularity}-{int(linear)}-{strategy}-{seed}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; mixed_traces += 1; custom_pool_traces += 1
        linear_pool_traces += int(linear)
        donor_linear_count_regressions += outputs[0].count('e linear_pool_count ')
        operations += len(trace.splitlines()) - 1
    trace = UPPER_GRANULARITY_REGRESSION
    (root / 'upper-granularity-regression.trace').write_text(trace)
    outputs = [record_replay(binary, trace, environment, root / ('upper-granularity-regression.' + suffix))
               for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
    try:
        model(trace, outputs[0], True)
    except AssertionError as error:
        assert error.args == ((1, 'buffer/image granularity conflict'),), error
    else:
        raise AssertionError('pinned upper granularity regression no longer reproduced; review donor')
    model(trace, outputs[1], False)
    assert 'e bind 102 101 65024 0' in outputs[0]
    assert 'e bind 102 101 64768 0' in outputs[1]
    cache_traces = 0
    for partial, atom, coherent, linear, minimum in itertools.product([False, True], [1, 64], [False, True], [False, True], [0, 1]):
        trace = cache_corpus(atom, coherent, linear, minimum, partial)
        name = f'cache-{int(partial)}-{atom}-{int(coherent)}-{int(linear)}-{minimum}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; cache_traces += 1; custom_pool_traces += 1; linear_pool_traces += int(linear)
        operations += len(trace.splitlines()) - 1
    existing_traces = 0
    for atom, coherent, granularity, linear, strategy in itertools.product([1, 64], [False, True], [1, 64, 256, 512, 4096], [False, True], [0, 3]):
        trace = mixed_corpus(atom, coherent, granularity, linear, strategy, 0x3aa92122, True)
        name = f'existing-mixed-{atom}-{int(coherent)}-{granularity}-{int(linear)}-{strategy}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; existing_traces += 1; mixed_traces += 1; custom_pool_traces += 1; linear_pool_traces += int(linear)
        operations += len(trace.splitlines()) - 1
        donor_linear_count_regressions += outputs[0].count('e linear_pool_count ')
    for atom, coherent, separate, address in itertools.product([1,64], [False,True], [False,True], [False,True]):
        trace = corpus(atom, coherent, separate, 0, 0x3aa92122, args.steps, False, address, False, True)
        name = f'existing-buffer-{atom}-{int(coherent)}-{int(separate)}-{int(address)}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; existing_traces += 1; address_traces += int(address)
        operations += len(trace.splitlines()) - 1
    raw_traces = 0
    for atom, coherent, granularity, linear, strategy in itertools.product([1,64], [False,True], [1,64,256,512,4096], [False,True], [0,3]):
        trace = mixed_corpus(atom, coherent, granularity, linear, strategy, 0x3aa92122, 2)
        name = f'raw-mixed-{atom}-{int(coherent)}-{granularity}-{int(linear)}-{strategy}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; raw_traces += 1; mixed_traces += 1; custom_pool_traces += 1; linear_pool_traces += int(linear)
        operations += len(trace.splitlines()) - 1
        donor_linear_count_regressions += outputs[0].count('e linear_pool_count ')
    for atom, coherent, separate, address in itertools.product([1,64], [False,True], [False,True], [False,True]):
        trace = corpus(atom, coherent, separate, 0, 0x3aa92122, args.steps, False, address, False, 2)
        name = f'raw-buffer-{atom}-{int(coherent)}-{int(separate)}-{int(address)}'
        (root / (name + '.trace')).write_text(trace)
        outputs = [record_replay(binary, trace, environment, root / (name + '.' + suffix))
                   for binary, suffix in [(root / 'vma-native', 'vma'), (root / 'rust-target/debug/vk-native-trace', 'rust')]]
        assert model(trace, outputs[0], True) == model(trace, outputs[1], False), name
        traces += 1; raw_traces += 1; address_traces += int(address)
        operations += len(trace.splitlines()) - 1
    sources = [HERE / name for name in ['native_oracle.cpp', 'native_trace.rs', 'verify_native.py', 'verify_virtual.py', 'fetch_donor.py']]
    sources += [ROOT / 'test/py/tool/verify_memory' / name for name in ['test_verify_native.py', 'test_verify_virtual.py']]
    hashes = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sources}
    package_hashes = {str(path.relative_to(args.package.resolve())): hashlib.sha256(path.read_bytes()).hexdigest() for path in (args.package.resolve() / 'src/rs').rglob('*.rs')}
    report = dict(donor=fetch_donor.COMMIT, package=str(args.package.resolve()), traces=traces, custom_pool_traces=custom_pool_traces, address_traces=address_traces, automatic_pool_traces=automatic_pool_traces, linear_pool_traces=linear_pool_traces, mixed_traces=mixed_traces, cache_traces=cache_traces, existing_traces=existing_traces, raw_traces=raw_traces, donor_upper_granularity_regressions=1, donor_linear_count_regressions=donor_linear_count_regressions, operations=operations, sanitize_cpp=args.sanitize, runtime_diagnostics=0, source_sha256=hashes, package_source_sha256=package_hashes,
                  cxx=checked([cpp, '--version']).stdout.splitlines()[0], rustc=checked(['rustc', '--version']).stdout.strip())
    (root / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({key: value for key, value in report.items() if not key.endswith('sha256')}))


if __name__ == '__main__':
    main()
