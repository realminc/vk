#!/usr/bin/env python3
"""Compare generated Rust record layouts with the exact pinned Khronos C headers.

Development gate only: requires a C compiler, cargo, and --headers <include-dir>.
No headers or C compiler are needed to build the published Rust library.
"""
import argparse
import hashlib
import os
from pathlib import Path
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]


def run(args, **kwargs):
    return subprocess.run(args, check=True, text=True, capture_output=True, **kwargs).stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--headers', type=Path, required=True)
    args = parser.parse_args()
    headers = args.headers.resolve()
    for line in (ROOT / 'registry/ABI_SHA256SUMS').read_text().splitlines():
        expected, name = line.split()
        actual = hashlib.sha256((headers / name).read_bytes()).hexdigest()
        if actual != expected:
            raise SystemExit(f'header does not match pinned snapshot: {name}')
    cc = os.environ.get('CC', 'cc')
    source = '#include <vulkan/vulkan_core.h>\n'
    processed = run([cc, '-E', '-P', '-x', 'c', '-I', str(headers), '-'], input=source)
    present = set(re.findall(r'typedef\s+(?:struct|union)\s+((?:Vk|StdVideo)\w+)\s*\{', processed))
    records = {}
    names = {}
    for line in (ROOT / 'tool/gen/abi.gen.tsv').read_text().splitlines():
        if not line.startswith('#'):
            c_type, rust_type, c_field, rust_field = line.split('\t')
            names[c_type, c_field] = rust_type, rust_field
    for filename in ('vk.xml', 'video.xml'):
        for node in ET.parse(ROOT / 'registry' / filename).findall('./types/type'):
            name = node.get('name')
            if name in present and node.get('category') in ('struct', 'union'):
                records[name] = node
    if len(records) < 1000:
        raise SystemExit(f'incomplete core ABI inventory: only {len(records)} records')
    with tempfile.TemporaryDirectory(prefix='vk-abi-') as directory:
        tmp = Path(directory)
        c = [source, '#include <stdio.h>\n#include <stddef.h>\nint main(void) {']
        rust = ['use vk::core::*;\nfn main() {']
        checks = 0
        bitfields = 0
        for name, node in sorted(records.items()):
            rust_name = next(rust_type for (c_type, _), (rust_type, _) in names.items() if c_type == name)
            c.append(f'printf("{name} %zu %zu\\n", sizeof({name}), _Alignof({name}));')
            rust.append(f'println!("{name} {{}} {{}}", std::mem::size_of::<{rust_name}>(), std::mem::align_of::<{rust_name}>());')
            checks += 1
            members = [m for m in node.findall('member') if m.get('api') is None or 'vulkan' in m.get('api').split(',')]
            for member in members:
                field = member.findtext('name')
                _, rust_field = names[name, field]
                # Only declarator text, never XML comment descriptions.
                suffix = member.find('name').tail or ''
                if ':' in suffix:
                    width = int(suffix.split(':', 1)[1].strip())
                    value = (1 << width) - 1
                    c.append(f'{{ {name} value = {{0}}; value.{field} = {value}u; unsigned char *p = (unsigned char *)&value; printf("{name}.{field} bits"); for (size_t i = 0; i < sizeof(value); ++i) printf(" %02x", p[i]); puts(""); }}')
                    # Byte buffers initialize padding too, making byte comparison defined.
                    rust.append(f'{{ let mut bytes = std::mem::MaybeUninit::<{rust_name}>::zeroed(); let p = bytes.as_mut_ptr(); unsafe {{ (*p).set_{rust_field.removeprefix("r#")}({value}); let b = std::slice::from_raw_parts(p.cast::<u8>(), std::mem::size_of::<{rust_name}>()); print!("{name}.{field} bits"); for v in b {{ print!(" {{v:02x}}"); }} println!(); }} }}')
                    bitfields += 1
                else:
                    c.append(f'printf("{name}.{field} %zu\\n", offsetof({name}, {field}));')
                    rust.append(f'println!("{name}.{field} {{}}", std::mem::offset_of!({rust_name}, {rust_field}));')
                checks += 1
        # Test scalar widths and constant C expression semantics independently.
        for c_name, rust_name in [('VkFlags64', 'Flags64'), ('VK_QUEUE_FAMILY_IGNORED', 'QUEUE_FAMILY_IGNORED'), ('VK_WHOLE_SIZE', 'WHOLE_SIZE')]:
            if c_name.startswith('Vk'):
                c.append(f'printf("{c_name} %zu\\n", sizeof({c_name}));')
                rust.append(f'println!("{c_name} {{}}", std::mem::size_of::<{rust_name}>());')
            else:
                c.append(f'printf("{c_name} %llu %zu\\n", (unsigned long long){c_name}, sizeof({c_name}));')
                rust.append(f'println!("{c_name} {{}} {{}}", {rust_name}, std::mem::size_of_val(&{rust_name}));')
            checks += 1
        c.append('return 0; }')
        rust.append('}')
        (tmp / 'abi.c').write_text('\n'.join(c))
        # C padding is zeroed explicitly before any bitfield mutation.
        c_text = (tmp / 'abi.c').read_text().replace('#include <stddef.h>', '#include <stddef.h>\n#include <string.h>')
        c_text = re.sub(r'(\w+) value = \{0\};', r'\1 value; memset(&value, 0, sizeof(value));', c_text)
        (tmp / 'abi.c').write_text(c_text)
        run([cc, '-std=c11', '-I', str(headers), str(tmp / 'abi.c'), '-o', str(tmp / 'c-abi')])
        expected = run([str(tmp / 'c-abi')]).splitlines()
        (tmp / 'Cargo.toml').write_text('[package]\nname="vk-abi-oracle"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nvk={package="vkrust",path=' + repr(str(ROOT)) + ', default-features=false}\n')
        (tmp / 'src').mkdir()
        # Keep LLVM from spending minutes on one enormous main function.
        # Each emitted check is a complete statement; chunking preserves order.
        statements = rust[1:-1]
        chunks = [statements[i:i + 200] for i in range(0, len(statements), 200)]
        rust = ['use vk::core::*;', 'fn main() {']
        rust.extend(f'check_{i}();' for i in range(len(chunks)))
        rust.append('}')
        for i, chunk in enumerate(chunks):
            rust.extend([f'fn check_{i}() {{', *chunk, '}'])
        (tmp / 'src/main.rs').write_text('\n'.join(rust))
        actual = run(['cargo', 'run', '--quiet', '--manifest-path', str(tmp / 'Cargo.toml')], env={**os.environ, 'CARGO_TARGET_DIR': str(ROOT / 'target/abi-oracle')}).splitlines()
        if actual != expected:
            import difflib
            print('\n'.join(difflib.unified_diff(expected, actual, fromfile='pinned C', tofile='Rust')))
            raise SystemExit('ABI comparison failed')
        print(f'ABI verified: {len(records)} records, {checks} layout/value checks, {bitfields} bitfield byte comparisons')


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError as exc:
        raise SystemExit(exc.stderr) from exc
