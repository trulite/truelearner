#!/usr/bin/env python3
"""Independently check the Playground dependency and loading boundaries."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
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
            ],
            cwd=ROOT,
            text=True,
        )
    )
    package_names = {package["id"]: package["name"] for package in metadata["packages"]}
    dependencies = {
        node["id"]: [dependency["pkg"] for dependency in node["deps"]]
        for node in metadata["resolve"]["nodes"]
    }
    playground = next(
        package_id
        for package_id, name in package_names.items()
        if name == "academy-playground"
    )
    reached = {playground}
    pending = [playground]
    while pending:
        package_id = pending.pop()
        for dependency in dependencies.get(package_id, []):
            if dependency not in reached:
                reached.add(dependency)
                pending.append(dependency)

    reached_names = {package_names[package_id] for package_id in reached}
    assert "academy-core" not in reached_names, reached_names
    assert "truelearner-core" not in reached_names, reached_names

    default_names = {
        package_names[package_id] for package_id in metadata["workspace_default_members"]
    }
    assert "academy-playground" not in default_names, default_names
    assert "academy-review" in default_names, default_names

    source = (ROOT / "academy/crates/playground/src/main.rs").read_text()
    manifest = (ROOT / "academy/crates/playground/Cargo.toml").read_text()
    assert "data:" not in source
    assert "base64" not in source
    assert "academy-episodes.workspace" not in manifest

    workspace = set(metadata["workspace_members"])
    workspace_dependencies = sorted(
        package_names[package_id] for package_id in reached & workspace
    )
    print(f"Playground workspace dependencies: {workspace_dependencies}")
    print(f"Academy default members: {sorted(default_names)}")
    print("No eager data-URI path or generator dependency")


if __name__ == "__main__":
    main()
