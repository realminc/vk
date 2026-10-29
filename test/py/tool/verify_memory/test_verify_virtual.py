"""Poisoned-output tests for the independent virtual-placement oracle."""
import importlib.util
from pathlib import Path
import subprocess
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
OWNER = ROOT / 'tool/verify_memory'
SPEC = importlib.util.spec_from_file_location('virtual_oracle', OWNER / 'verify_virtual.py')
ORACLE = importlib.util.module_from_spec(SPEC)
sys.path.insert(0, str(OWNER))
try:
    SPEC.loader.exec_module(ORACLE)
finally:
    sys.path.pop(0)

TRACE = 'b 64\na 0 16 8 0\na 1 8 8 0\nf 0\nc\n'
ROWS = ['b 64 0 0 64 1 64', 'a 0 1 0 16 1 16 48 1 48',
        'a 1 1 16 8 2 24 40 1 40', 'f 0 1 8 56 2 40', 'c 0 0 64 1 64']


class VirtualOracleTest(unittest.TestCase):
    def test_accepts_valid_placement_free_and_clear_statistics(self):
        ORACLE.model(TRACE, '\n'.join(ROWS))

    def test_rejects_overlap_alignment_bounds_false_exhaustion_and_bad_accounting(self):
        for index, poison in [
            (2, 'a 1 1 8 8 2 24 40 1 40'),
            (2, 'a 1 1 17 8 2 24 40 1 40'),
            (2, 'a 1 1 64 8 2 24 40 1 40'),
            (1, 'a 0 0 0 16 0 0 64 1 64'),
            (3, 'f 0 2 8 56 2 40'),
        ]:
            rows = ROWS.copy()
            rows[index] = poison
            with self.subTest(poison=poison), self.assertRaises(AssertionError):
                ORACLE.model(TRACE, '\n'.join(rows))

    def test_requires_lowest_offset_only_for_min_offset_strategy(self):
        output = 'b 64 0 0 64 1 64\na 0 1 8 8 1 8 56 2 48\n'
        ORACLE.model('b 64\na 0 8 8 0\n', output)
        with self.assertRaises(AssertionError):
            ORACLE.model('b 64\na 0 8 8 3\n', output)

    def test_optimized_python_cannot_silently_disable_verification(self):
        result = subprocess.run([sys.executable, '-O', str(OWNER / 'verify_virtual.py'), '--help'], capture_output=True, text=True, timeout=30)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('verification requires Python assertions', result.stderr)

    def test_rejects_successful_process_with_runtime_sanitizer_diagnostics(self):
        with patch.object(ORACLE, 'checked', return_value=SimpleNamespace(stdout='', stderr='runtime error: poisoned')):
            with self.assertRaisesRegex(SystemExit, 'unexpected replay diagnostics'):
                ORACLE.replay(Path('mock-oracle'), '', {})

class LinearOracleTest(unittest.TestCase):
    def test_linear_stack_is_not_first_fit_and_rejects_wrong_cursor_and_false_exhaustion(self):
        trace = 'b 64 1\na 0 16 8 0 0\na 1 16 8 0 0\nf 0\na 2 16 8 3 0\nc\n'
        rows = ['b 64 0 0 64 1 64', 'a 0 1 0 16 1 16 48 1 48',
                'a 1 1 16 16 2 32 32 1 32', 'f 0 1 16 48 2 32',
                'a 2 1 32 16 2 32 32 2 16', 'c 0 0 64 1 64']
        ORACLE.model(trace, '\n'.join(rows))
        for poison in ['a 2 1 0 16 2 32 32 1 32', 'a 2 0 0 16 1 16 48 2 32']:
            bad = rows.copy(); bad[4] = poison
            with self.subTest(poison=poison), self.assertRaises(AssertionError):
                ORACLE.model(trace, '\n'.join(bad))

    def test_upper_alignment_and_bounds_are_independent_of_donor_agreement(self):
        trace = 'b 64 1\na 0 17 8 0 1\nc\n'
        valid = 'b 64 0 0 64 1 64\na 0 1 40 17 1 17 47 2 40\nc 0 0 64 1 64\n'
        ORACLE.model(trace, valid)
        for poison in [valid.replace('1 40 17', '1 32 17'), valid.replace('1 40 17', '1 48 17')]:
            with self.assertRaises(AssertionError): ORACLE.model(trace, poison)


if __name__ == '__main__':
    unittest.main()
