"""Both implementations agreeing cannot hide missing/poisoned concurrency evidence."""
import importlib.util
from pathlib import Path
import sys
import unittest

OWNER = Path(__file__).resolve().parents[4] / "tool/verify_memory"
SPEC = importlib.util.spec_from_file_location("concurrency_oracle", OWNER / "verify_concurrency.py")
ORACLE = importlib.util.module_from_spec(SPEC)
sys.path.insert(0, str(OWNER))
try:
    SPEC.loader.exec_module(ORACLE)
finally:
    sys.path.pop(0)

VALID = "CONCURRENCY independent-map 1 0 0\nCONCURRENCY independent-bind 1 0 0\nCONCURRENCY same-backing 1 2 2\n"


class ConcurrencyOracleTest(unittest.TestCase):
    def test_accepts_complete_semantic_outcomes(self):
        self.assertEqual(ORACLE.outcomes(VALID), ORACLE.EXPECTED)

    def test_rejects_serial_execution_leaked_storage_or_unbalanced_native_maps(self):
        for output in [VALID.replace("map 1", "map 0"), VALID.replace("bind 1 0 0", "bind 1 1 0"),
                       VALID.replace("1 2 2", "1 2 1"), VALID + VALID.splitlines()[0],
                       VALID.replace("CONCURRENCY independent-bind 1 0 0\n", "")]:
            with self.subTest(output=output), self.assertRaises(ValueError):
                ORACLE.outcomes(output)
