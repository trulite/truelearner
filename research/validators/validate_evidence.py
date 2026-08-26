#!/usr/bin/env python3
"""Validate a neutral research evidence envelope."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import load_json, require_digest, require_list, require_text


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_json(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-evidence/v1":
        errors.append("schema must be research-evidence/v1")
    require_text(value.get("experiment"), "experiment", errors)
    require_digest(value.get("protocol_sha256"), "protocol_sha256", errors)
    require_digest(value.get("subject_digest"), "subject_digest", errors)
    require_text(value.get("producer"), "producer", errors)
    environment = value.get("environment")
    if not isinstance(environment, dict) or not environment:
        errors.append("environment must be a non-empty object")
    observations = value.get("observations")
    if not isinstance(observations, dict) or not observations:
        errors.append("observations must be a non-empty object")
    artifacts = require_list(value.get("artifacts"), "artifacts", errors)
    for index, artifact in enumerate(artifacts):
        if not isinstance(artifact, dict):
            errors.append(f"artifacts[{index}] must be an object")
            continue
        require_text(artifact.get("path"), f"artifacts[{index}].path", errors)
        require_digest(artifact.get("sha256"), f"artifacts[{index}].sha256", errors)
    if value.get("completed") is not True:
        errors.append("completed must be true for adjudication")
    if "verdict" in value or "authority" in value:
        errors.append("neutral evidence must not contain a scientific verdict or authority transition")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", required=True, type=Path)
    args = parser.parse_args()
    errors = validate(args.file.resolve())
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    print("Research evidence is valid and neutral.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
