#!/usr/bin/env python3
"""Check a sanitized public checkout before building or publishing it."""

import argparse
from pathlib import Path, PurePosixPath
import re
import subprocess
import tomllib

PUBLIC_REPOSITORY = "https://github.com/realminc/vk"
PRIVATE_PATHS = (
    ".cursor", ".devin", ".vscode", ".zed", "AGENTS.md",
    "docs/" + "internal", "tool/" + "release", "test/py/tool/" + "release",
)
SECRET = re.compile(
    rb"AKIA[0-9A-Z]{16}|ghp_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}"
    rb"|-----BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY-----|cio[A-Za-z0-9]{32,}"
)
WORKSTATION = re.compile(rb"/home/[A-Za-z0-9._-]+/|C:[/\\]Users[/\\][A-Za-z0-9._-]+[/\\]")
CREDENTIAL_NAME = re.compile(r"^(?:\.env(?:\..*)?|\.pypirc|credentials?(?:\..*)?|secrets?(?:\..*)?|id_(?:rsa|dsa|ecdsa|ed25519))$")


def audit(root):
    paths = subprocess.check_output(["git", "ls-files", "-z"], cwd=root).decode().split("\0")
    for name in filter(None, paths):
        if any(name == prefix or name.startswith(prefix + "/") for prefix in PRIVATE_PATHS):
            raise ValueError(f"private source path: {name}")
        path = root / name
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"public input must be a regular file: {name}")
        if any(CREDENTIAL_NAME.fullmatch(part) for part in PurePosixPath(name).parts):
            raise ValueError(f"credential-shaped filename: {name}")
        data = path.read_bytes()
        if SECRET.search(data) or WORKSTATION.search(data):
            # Never include matched secret contents in diagnostics.
            raise ValueError(f"secret-shaped content or workstation path: {name}")
        if name.endswith(".md"):
            for target in re.findall(r"\]\(([^\s)]+)", data.decode()):
                if ":" in target or target.startswith("#"):
                    continue
                local = target.split("#", 1)[0]
                if local and not (path.parent / local).exists():
                    raise ValueError(f"broken local document link in {name}: {target}")
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    package = manifest["workspace"]["package"]
    if package["repository"] != PUBLIC_REPOSITORY or package["license"] != "MIT":
        raise ValueError("public repository identity or MIT license mismatch")
    if manifest["package"]["name"] != "vkrust" or manifest["lib"]["name"] != "vk":
        raise ValueError("public package/import identity mismatch")
    marker = tomllib.loads((root / ".vk-public-snapshot").read_text())
    if marker.get("version") != package["version"] or not re.fullmatch("[0-9a-f]{40}", marker.get("source_commit", "")):
        raise ValueError("invalid snapshot provenance")
    if tomllib.loads((root / "tool/gen/Cargo.toml").read_text())["package"].get("publish") is not False:
        raise ValueError("generator must stay unpublished")
    print("Verified sanitized public inventory, MIT license and vk namespace")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path.cwd())
    args = parser.parse_args()
    audit(args.root.resolve())
