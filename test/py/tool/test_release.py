"""Failure gates for public inventory, registry retries and archive inputs."""

import contextlib
import importlib.util
import io
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import urllib.error

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "tool"))
import audit_crate
import audit_public
import publish_crate


def make_archive(directory, mutation=None, version="0.3.0"):
    files = {
        "Cargo.toml": b'''[package]\nname="vkrust"\nversion="0.3.0"\nlicense="MIT"\nrepository="https://github.com/realminc/vk"\n[lib]\nname="vk"\npath="src/rs/lib.rs"\n''',
        "Cargo.toml.orig": b"", "Cargo.lock": b"", "rustfmt.toml": b"",
        "README.md": b"", "CHANGELOG.md": b"", "NOTICE.md": b"",
        "LICENSE": b"", "license/khronos/Apache-2.0.txt": b"", "LICENSE-VMA-MIT": b"",
        "docs/external/qualification.md": b"", "registry/SOURCES.md": b"",
        "src/rs/lib.rs": b'#[cfg(test)]\n#[path="../../test/rs/load/test_entry.rs"]\nmod tests;\n',
        "test/rs/load/test_entry.rs": b"", "test/rs/test_lib.rs": b"",
        "test/rs/test_command.gen.rs": b"",
        "test/rs/test_load.rs": b"", "test/rs/test_memory.rs": b"",
    }
    files["Cargo.toml"] = files["Cargo.toml"].replace(b'version="0.3.0"', f'version="{version}"'.encode())
    if mutation:
        mutation(files)
    archive = Path(directory) / f"vkrust-{version}.crate"
    with tarfile.open(archive, "w:gz") as tar:
        for name, data in files.items():
            member = tarfile.TarInfo(f"vkrust-{version}/" + name)
            member.size = len(data)
            tar.addfile(member, io.BytesIO(data))
    return archive


class ArchiveTest(unittest.TestCase):
    def test_full_metadata_version_is_preserved_in_archive_identity(self):
        with tempfile.TemporaryDirectory() as directory, contextlib.redirect_stdout(io.StringIO()):
            version = "0.4.0+1.4.365"
            archive = make_archive(directory, version=version)
            audit_crate.audit(archive, version, audit_public.PUBLIC_REPOSITORY)
            with self.assertRaises(ValueError):
                audit_crate.audit(archive, "0.4.0", audit_public.PUBLIC_REPOSITORY)

    def check(self, mutation=None):
        with tempfile.TemporaryDirectory() as directory, contextlib.redirect_stdout(io.StringIO()):
            return audit_crate.audit(make_archive(directory, mutation), "0.3.0", audit_public.PUBLIC_REPOSITORY)

    def test_archive_preserves_namespace_and_private_test_inputs(self):
        files = self.check()
        self.assertIn("src/rs/lib.rs", files)
        self.assertIn("test/rs/test_command.gen.rs", files)

    def test_missing_test_input_and_private_inventory_fail(self):
        for mutation in (
            lambda files: files.pop("test/rs/load/test_entry.rs"),
            lambda files: files.update({"../escape": b""}),
            lambda files: files.update({".cursor/rules/private.mdc": b""}),
            lambda files: files.update({"tool/gen/main.rs": b""}),
            lambda files: files.update({"test/rs/tool/gen/test_generation.rs": b""}),
            lambda files: files.update({"Cargo.toml": files["Cargo.toml"].replace(b'license="MIT"', b'license="Apache-2.0"')}),
            lambda files: files.update({"Cargo.toml": files["Cargo.toml"] + b'[dependencies]\nlocal={path="../private"}\n'}),
        ):
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                self.check(mutation)

    def test_symlink_and_duplicate_archive_entries_fail(self):
        for mode in ("symlink", "duplicate"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as directory:
                archive = make_archive(directory)
                with tarfile.open(archive) as tar:
                    entries = [(member, tar.extractfile(member).read()) for member in tar]
                with tarfile.open(archive, "w:gz") as tar:
                    for member, data in entries:
                        tar.addfile(member, io.BytesIO(data))
                    extra = tarfile.TarInfo("vkrust-0.3.0/README.md")
                    if mode == "symlink":
                        extra.type = tarfile.SYMTYPE
                        extra.linkname = "/etc/passwd"
                    tar.addfile(extra, io.BytesIO(b""))
                with self.assertRaises(ValueError):
                    audit_crate.audit(archive, "0.3.0")


class RegistryTest(unittest.TestCase):
    def test_build_metadata_is_url_encoded_for_registry_requests(self):
        error = urllib.error.HTTPError("https://example.invalid", 404, "absent", {}, None)
        with patch.object(publish_crate.urllib.request, "urlopen", side_effect=error) as request:
            self.assertIsNone(publish_crate.metadata("0.4.0+1.4.365"))
        self.assertTrue(request.call_args.args[0].full_url.endswith("/0.4.0%2B1.4.365"))
        error.close()

    def test_only_not_found_means_absent(self):
        for status in (401, 403, 429, 500):
            error = urllib.error.HTTPError("https://example.invalid", status, "failure", {}, None)
            with self.subTest(status=status), patch.object(publish_crate.urllib.request, "urlopen", side_effect=error):
                with self.assertRaises(urllib.error.HTTPError):
                    publish_crate.metadata("0.3.0")
            error.close()
        error = urllib.error.HTTPError("https://example.invalid", 404, "absent", {}, None)
        with patch.object(publish_crate.urllib.request, "urlopen", side_effect=error):
            self.assertIsNone(publish_crate.metadata("0.3.0"))
        error.close()

    def test_retry_requires_exact_unyanked_checksum(self):
        publish_crate.verify_existing({"checksum": "expected", "yanked": False}, "expected")
        for existing in ({"checksum": "other", "yanked": False}, {"checksum": "expected", "yanked": True}, {}):
            with self.subTest(existing=existing), self.assertRaises(ValueError):
                publish_crate.verify_existing(existing, "expected")


if __name__ == "__main__":
    unittest.main()
