"""The donor preference oracle must not depend on an unpinned local Vulkan SDK."""
import hashlib
import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
OWNER = ROOT / 'tool/verify_memory'
SPEC = importlib.util.spec_from_file_location('policy_oracle', OWNER / 'verify_policy.py')
ORACLE = importlib.util.module_from_spec(SPEC)
sys.path.insert(0, str(OWNER))
try:
    SPEC.loader.exec_module(ORACLE)
finally:
    sys.path.pop(0)


class PolicyOracleTest(unittest.TestCase):
    def fixture(self, root):
        source = root / 'source'
        files = {'vulkan/vulkan_core.h': b'core', 'vulkan/vk_platform.h': b'platform'}
        for name, data in files.items():
            path = source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        (root / 'registry').mkdir()
        (root / 'registry/ABI_SHA256SUMS').write_text(''.join(
            f'{hashlib.sha256(data).hexdigest()}  {name}\n' for name, data in files.items()
        ))
        return source, files

    def test_stages_exact_pinned_inputs_and_poisoned_umbrella(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, files = self.fixture(root)
            (source / 'vulkan/vulkan.h').write_text('unverified local SDK umbrella')
            (source / 'extra.h').write_text('unverified extra header')
            destination = root / 'staged'
            with patch.object(ORACLE, 'ROOT', root):
                ORACLE.stage_headers(source, destination)
            for name, data in files.items():
                self.assertEqual((destination / name).read_bytes(), data)
            self.assertTrue((destination / 'vulkan/vulkan.h').read_text().startswith('#error'))
            self.assertFalse((destination / 'extra.h').exists())
            self.assertEqual(len(list(destination.rglob('*.h'))), len(files) + 1)

    def test_hash_mismatch_writes_no_partial_staging(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, _ = self.fixture(root)
            (source / 'vulkan/vk_platform.h').write_text('corrupt')
            destination = root / 'staged'
            with patch.object(ORACLE, 'ROOT', root), self.assertRaisesRegex(ValueError, 'pinned header mismatch'):
                ORACLE.stage_headers(source, destination)
            self.assertFalse(destination.exists())


if __name__ == '__main__':
    unittest.main()
