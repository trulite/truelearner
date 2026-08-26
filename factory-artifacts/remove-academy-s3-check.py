#!/usr/bin/env python3
"""Check that the living Academy graph has no AWS/S3 storage infrastructure."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]
ACADEMY = ROOT / "academy"


def main() -> None:
    roots = [
        ACADEMY / "Cargo.toml",
        ACADEMY / "Cargo.lock",
        ACADEMY / "README.md",
        ACADEMY / "docs",
        ACADEMY / "crates",
    ]
    terms = (
        "academy-storage",
        "academy_storage",
        "aws-config",
        "aws-sdk-s3",
        "ACADEMY_S3_",
    )
    matches = [
        (str(path.relative_to(ROOT)), term)
        for root in roots
        for path in ([root] if root.is_file() else root.rglob("*"))
        if path.is_file()
        for term in terms
        if term in path.read_text(errors="ignore")
    ]
    assert not matches, matches
    assert not (ACADEMY / "crates/academy-storage").exists()
    assert not (ACADEMY / "docs/s3_storage_v1.md").exists()
    assert 'name = "aws-' not in (ACADEMY / "Cargo.lock").read_text()

    metadata = json.loads(
        subprocess.check_output(
            [
                "cargo",
                "metadata",
                "--locked",
                "--manifest-path",
                "academy/Cargo.toml",
                "--format-version",
                "1",
                "--no-deps",
            ],
            cwd=ROOT,
            text=True,
        )
    )
    workspace_names = {package["name"] for package in metadata["packages"]}
    assert "academy-storage" not in workspace_names, workspace_names
    print(f"Academy workspace packages: {sorted(workspace_names)}")
    print("No living AWS/S3 storage crate, dependency, configuration, or docs")


if __name__ == "__main__":
    main()
