"""Poisoned native-call traces must fail independently of donor/Rust agreement."""
import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
OWNER = ROOT / 'tool/verify_memory'
SPEC = importlib.util.spec_from_file_location('native_oracle', OWNER / 'verify_native.py')
ORACLE = importlib.util.module_from_spec(SPEC)
sys.path.insert(0, str(OWNER))
try:
    SPEC.loader.exec_module(ORACLE)
finally:
    sys.path.pop(0)

TRACE = 'b 64 0 0\na 0 65 8 1 2 0 0 0\na 1 65 8 1 2 0 0 0\nm 0\nh 0 1 64\nu 0\nf 1\nf 0\nc\n'
OUTPUT = '''e create 100 0
e allocate 0 65536 0 101
e bind 100 101 0 0
r a 0 1 100 101 0 65 0 1 65 1 65536 0
e create 102 0
e bind 102 101 128 0
r a 1 1 102 101 128 65 0 2 130 1 65536 0
e map 101 0 65536 0
r m 0 1 0 0 0 0 0 2 130 1 65536 1
e flush 101 0 128
r h 0 1 0 0 0 0 0 2 130 1 65536 1
e unmap 101
r u 0 1 0 0 0 0 0 2 130 1 65536 0
e destroy 102
r f 1 1 0 0 0 0 0 1 65 1 65536 0
e destroy 100
e free 101
r f 0 1 0 0 0 0 0 0 0 0 0 0
r c 0 1 0 0 0 0 0 0 0 0 0 0
'''


class NativeOracleTest(unittest.TestCase):
    def test_accepts_actual_block_sharing_atom_isolation_and_cache_translation(self):
        ORACLE.model(TRACE, OUTPUT, False)
        ORACLE.model(TRACE, OUTPUT, True)

    def test_rejects_overlap_alignment_bounds_type_relaxation_and_bad_accounting(self):
        for poison in [
            OUTPUT.replace('102 101 128', '102 101 0'),
            OUTPUT.replace('102 101 128', '102 101 129'),
            OUTPUT.replace('102 101 128', '102 101 65536'),
            OUTPUT.replace('e allocate 0', 'e allocate 2'),
            OUTPUT.replace('2 130 1 65536', '2 131 1 65536'),
        ]:
            with self.subTest(poison=poison), self.assertRaises(AssertionError):
                ORACLE.model(TRACE, poison, False)

    def test_rejects_false_exhaustion_missing_bind_and_leaked_temporaries(self):
        for poison in [
            OUTPUT.replace('r a 1 1 102 101 128 65 0', 'r a 1 0 0 0 0 0 0'),
            OUTPUT.replace('e bind 102 101 128 0\n', ''),
            OUTPUT.replace('e destroy 102\n', ''),
        ]:
            with self.subTest(poison=poison), self.assertRaises(AssertionError):
                ORACLE.model(TRACE, poison, False)

    def test_rejects_bad_cache_range_mapping_lifetime_and_premature_memory_release(self):
        for poison in [
            OUTPUT.replace('e flush 101 0 128', 'e flush 101 0 64'),
            OUTPUT.replace('e unmap 101\n', ''),
            OUTPUT.replace('e destroy 100\ne free 101', 'e free 101\ne destroy 100'),
            OUTPUT.replace('e map 101 0 65536 0', 'e map 101 8 65528 0'),
        ]:
            with self.subTest(poison=poison), self.assertRaises(AssertionError):
                ORACLE.model(TRACE, poison, False)

    def test_empty_block_cache_is_only_admitted_for_donor_until_explicit_close(self):
        output = OUTPUT.replace('e free 101\nr f 0 1 0 0 0 0 0 0 0 0 0 0',
                                'r f 0 1 0 0 0 0 0 0 0 1 65536 0\ne free 101')
        ORACLE.model(TRACE, output, True)
        with self.assertRaises(AssertionError):
            ORACLE.model(TRACE, output, False)

    def test_preferred_type_order_is_independent_of_cross_implementation_agreement(self):
        trace = 'b 64 0 1\na 0 65 8 3 2 0 0 0\nf 0\nc\n'
        output = '\n'.join(OUTPUT.splitlines()[:4]) + '\n' + '\n'.join(OUTPUT.splitlines()[-4:]) + '\n'
        output = output.replace('65536', '65')
        ORACLE.model(trace, output, False)
        with self.assertRaisesRegex(AssertionError, 'preferred-type order'):
            ORACLE.model(trace.replace('3 2 0 0 0', '3 2 8 0 0'), output, False)

    def test_keeps_failed_replay_evidence_and_rejects_exit_zero_diagnostics(self):
        for code, stderr in [(0, 'runtime error: poisoned'), (1, ''), (0, '')]:
            with tempfile.TemporaryDirectory() as folder:
                output = Path(folder) / 'trace.vma'
                result = SimpleNamespace(stdout='partial native trace', stderr=stderr, returncode=code)
                with patch.object(ORACLE.subprocess, 'run', return_value=result):
                    if code or stderr:
                        with self.assertRaisesRegex(SystemExit, 'retained evidence'):
                            ORACLE.record_replay(Path('mock-oracle'), TRACE, {}, output)
                    else:
                        self.assertEqual(ORACLE.record_replay(Path('mock-oracle'), TRACE, {}, output), result.stdout)
                self.assertEqual(output.read_text(), result.stdout)
                self.assertEqual(Path(str(output) + '.stderr').read_text(), stderr)
                self.assertIn(f'"exit_code": {code}', Path(str(output) + '.process.json').read_text())

    def test_optimized_python_cannot_disable_model_checks(self):
        result = subprocess.run([sys.executable, '-O', str(OWNER / 'verify_native.py'), '--help'],
                                capture_output=True, text=True, timeout=30)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('verification requires Python assertions', result.stderr)


if __name__ == '__main__':
    unittest.main()
