"""Vulkan build metadata must not change release tags or runtime API claims."""

import contextlib
import io
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "tool"))
import check_release
import release_version


class VersionTest(unittest.TestCase):
    def fixture(self, directory):
        root = Path(directory)
        (root / "registry").mkdir()
        (root / "registry/vk.xml").write_text('''<registry><types>
<type><name>VK_HEADER_VERSION</name> 365</type>
<type api="vulkan,vulkanbase"><name>VK_HEADER_VERSION_COMPLETE</name>
VK_MAKE_API_VERSION(0, 1, 4, VK_HEADER_VERSION)</type>
<type api="vulkansc"><name>VK_HEADER_VERSION_COMPLETE</name>
VK_MAKE_API_VERSION(1, 1, 0, VK_HEADER_VERSION)</type>
</types></registry>''')
        (root / "src/rs").mkdir(parents=True)
        (root / "src/rs/constant.gen.rs").write_text(
            'pub const HEADER_VERSION: u32 = 365;\n'
            'pub const HEADER_VERSION_COMPLETE: u32 = crate::make_api_version(0, 1, 4, HEADER_VERSION);\n'
        )
        return root

    def test_metadata_is_distinct_from_release_version(self):
        self.assertEqual(release_version.release_version("0.4.0+1.4.365"), "0.4.0")
        self.assertEqual(release_version.release_version("0.3.0"), "0.3.0")

    def test_malformed_or_ambiguous_versions_fail(self):
        for version in ("", "0.4", "v0.4.0", "00.4.0", "0.4.0+1.4", "0.4.0+1.4.0365",
                        "0.4.0+1.4.365+extra", "0.4.0-preview", "0.4.0+latest"):
            with self.subTest(version=version), self.assertRaises(ValueError):
                release_version.release_version(version)

    def test_suffix_matches_selected_vulkan_xml_not_vulkansc(self):
        with tempfile.TemporaryDirectory() as directory:
            root = self.fixture(directory)
            self.assertEqual(release_version.validate_registry_version(root, "0.4.0+1.4.365"), "1.4.365")
            for version in ("0.4.0", "0.4.0+1.3.365", "0.4.0+1.4.364", "0.4.0+1.0.365"):
                with self.subTest(version=version), self.assertRaisesRegex(ValueError, "pinned Vulkan"):
                    release_version.validate_registry_version(root, version)

    def test_generated_header_drift_fails(self):
        for before, after in (("365;", "364;"), ("(0, 1, 4,", "(0, 1, 3,")):
            with self.subTest(after=after), tempfile.TemporaryDirectory() as directory:
                root = self.fixture(directory)
                constants = root / "src/rs/constant.gen.rs"
                constants.write_text(constants.read_text().replace(before, after))
                with self.assertRaisesRegex(ValueError, "generated HEADER_VERSION"):
                    release_version.validate_registry_version(root, "0.4.0+1.4.365")

    def test_release_tag_and_note_use_library_version(self):
        with tempfile.TemporaryDirectory() as directory:
            root = self.fixture(directory)
            (root / "Cargo.toml").write_text(
                '[workspace.package]\nversion="0.4.0+1.4.365"\nrust-version="1.98.1"\n'
            )
            (root / "Cargo.lock").write_text(
                '[[package]]\nname="vkrust"\nversion="0.4.0+1.4.365"\n'
                '[[package]]\nname="vkrust-gen"\nversion="0.4.0+1.4.365"\n'
            )
            (root / "rust-toolchain.toml").write_text('[toolchain]\nchannel="1.98.1"\n')
            note = root / "docs/external/release/v0.4.0.md"
            note.parent.mkdir(parents=True)
            note.write_text("# v0.4.0\n")
            with patch.object(check_release, "__file__", str(root / "tool/check_release.py")), \
                 patch.object(check_release.subprocess, "run") as git, \
                 contextlib.redirect_stdout(io.StringIO()):
                with patch.dict(os.environ, GITHUB_REF="refs/tags/v0.4.0"):
                    check_release.main()
                git.assert_called_once_with(
                    ["git", "merge-base", "--is-ancestor", "HEAD", "origin/main"], cwd=root, check=True
                )
                for tag in ("v0.4.0+1.4.365", "v0.3.0"):
                    with self.subTest(tag=tag), patch.dict(os.environ, GITHUB_REF="refs/tags/" + tag), \
                         self.assertRaisesRegex(ValueError, "tag and manifest"):
                        check_release.main()
                note.unlink()
                with patch.dict(os.environ, GITHUB_REF="refs/tags/v0.4.0"), \
                     self.assertRaisesRegex(ValueError, "release notes"):
                    check_release.main()


if __name__ == "__main__":
    unittest.main()
