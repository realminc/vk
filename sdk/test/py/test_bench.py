"""Poisoned evidence must fail independently of allocator agreement."""
import copy
from pathlib import Path
import sys
import re
from types import SimpleNamespace
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "tool"))
import bench
import qualify


class EvidenceTest(unittest.TestCase):
    def test_source_manifest_excludes_sdk_build_artifacts(self):
        sources = bench.manifest({})["source_sha256"]
        self.assertFalse(any(path.startswith("sdk/target/") for path in sources))

    def test_named_native_profiles_enable_the_actual_rust_gate(self):
        profiles = [*qualify.CORE, *(profile[:3] for profile in qualify.OPTIONAL.values())]
        for target, name, gate in profiles:
            owner, function = name.split("::")
            umbrella = (bench.ROOT / f"test/rs/test_{target}.rs").read_text()
            module = re.search(r'#\[path = "([^"]+)"\]\s*mod ' + owner + r';', umbrella)
            self.assertIsNotNone(module, name)
            source = (bench.ROOT / "test/rs" / module.group(1)).read_text()
            body = source.split("fn " + function + "(", 1)[1].split("#[test]", 1)[0]
            self.assertIn(f'std::env::var("{gate}")', body, name)

    def setUp(self):
        self.args = SimpleNamespace(api="1.2", device=0, memory_type=0, batches=2, batch_size=5)
        requested = sum(bench.SIZES)
        self.record = {"schema": "vk.sdk.sample.v1", "case": "buffer-pool", "backend": "vma",
            "api_requested": (1 << 22) | (2 << 12), "device_index": 0, "memory_type": 0,
            "batches": 2, "batch_size": 5, "block_size": 64 * 1024 * 1024, "sizes": bench.SIZES,
            "identity": {"device_uuid": [1] * 16, "driver_uuid": [2] * 16, "vendor_id": 1,
                "device_id": 1, "device_type": 1, "api_version": (1 << 22) | (2 << 12),
                "driver_version": 1, "queue_family": 0, "device_count": 1},
            "measurements": {"allocations": 10, "allocate_ns": 100, "free_ns": 100,
                "requested_byte_checksum": requested * 2,
                "peak": {"live_count": 5, "native_count": 1, "native_bytes": 64 * 1024 * 1024,
                         "allocation_bytes": requested}}}

    def test_accept_complete_sample(self):
        self.assertEqual(bench.verify_sample(self.record, "buffer-pool", "vma", self.args), ("allocate_ns", "free_ns"))

    def test_vulkano_selection_keeps_cached_queries_out_of_native_comparison(self):
        self.assertEqual(bench.select_cases("auto", "all"), bench.CASES)
        self.assertEqual(bench.select_cases("vulkano", "all"), {
            "startup": "vulkano", "buffer-pool": "vulkano", "buffer-separate": "vulkano"})
        for case in ("query", "virtual"):
            with self.assertRaises(ValueError): bench.select_cases("vulkano", case)

    def test_vulkano_sample_requires_managed_scope(self):
        record = copy.deepcopy(self.record)
        record["backend"] = "vulkano"
        with self.assertRaises(ValueError): bench.verify_sample(record, "buffer-pool", "vulkano", self.args)
        record["comparison_scope"] = "checked-managed-api"
        self.assertEqual(bench.verify_sample(record, "buffer-pool", "vulkano", self.args), ("allocate_ns", "free_ns"))

    def test_reject_poisoned_samples(self):
        mutations = [
            lambda r: r.update(case="buffer-separate"),
            lambda r: r.update(sizes=[64]),
            lambda r: r["measurements"].update(allocations=9),
            lambda r: r["measurements"].update(requested_byte_checksum=0),
            lambda r: r["measurements"].update(allocate_ns=0),
            lambda r: r["measurements"]["peak"].update(live_count=0),
            lambda r: r["measurements"]["peak"].update(native_count=2),
            lambda r: r["measurements"]["peak"].update(native_bytes=1024),
            lambda r: r["identity"].update(device_type=4),
            lambda r: r["identity"].update(device_uuid=[0] * 16),
            lambda r: r["identity"].pop("driver_uuid"),
        ]
        for mutation in mutations:
            record = copy.deepcopy(self.record)
            mutation(record)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                bench.verify_sample(record, "buffer-pool", "vma", self.args)

    def test_ratio_direction_and_confidence(self):
        pairs = [{"vkrust": {"measurements": {"query_ns": 20}},
                  "reference": {"measurements": {"query_ns": 10}}} for _ in range(10)]
        summary = bench.summarize(pairs, ["query_ns"])["query_ns"]
        self.assertEqual(summary["median_paired_ratio_vk_over_reference"], 2)
        self.assertEqual(summary["paired_median_ratio_95pct_bootstrap_ci"], [2, 2])


if __name__ == "__main__":
    unittest.main()
