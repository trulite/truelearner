#!/usr/bin/env python3
"""Validate an independent verification receipt and candidate lineage."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import artifact_path, load_json, placeholder, require_list, require_text, sha256
from validate_candidate import validate as validate_candidate


VERDICTS = {"supported", "refuted", "inconclusive"}
REGRESSION_LIMIT_SECONDS = 10.0


def validate(receipt_path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_json(receipt_path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "rust-verification/v1":
        errors.append("schema must be rust-verification/v1")

    candidate = value.get("candidate_receipt")
    if not isinstance(candidate, dict):
        errors.append("candidate_receipt must be an object")
    else:
        path_text = require_text(candidate.get("path"), "candidate_receipt.path", errors)
        digest = require_text(candidate.get("sha256"), "candidate_receipt.sha256", errors)
        if path_text:
            path = artifact_path(receipt_path, path_text)
            if not path.is_file():
                errors.append(f"candidate receipt does not exist: {path}")
            else:
                if digest and sha256(path) != digest:
                    errors.append("candidate_receipt.sha256 does not match")
                errors.extend(f"candidate: {item}" for item in validate_candidate(path))

    if value.get("verdict") not in VERDICTS:
        errors.append("verdict must be supported, refuted, or inconclusive")
    count = value.get("independent_test_count")
    if not isinstance(count, int) or count < 1:
        errors.append("independent_test_count must be at least 1")

    checks = require_list(value.get("checks"), "checks", errors)
    seen: set[str] = set()
    for index, check in enumerate(checks):
        if not isinstance(check, dict):
            errors.append(f"checks[{index}] must be an object")
            continue
        name = require_text(check.get("name"), f"checks[{index}].name", errors)
        require_text(check.get("command"), f"checks[{index}].command", errors)
        duration = check.get("duration_seconds")
        if isinstance(duration, bool) or not isinstance(duration, (int, float)) or duration < 0:
            errors.append(f"checks[{index}].duration_seconds must be non-negative")
        if name:
            seen.add(name)
        if name == "regression-suite" and isinstance(duration, (int, float)) and duration >= REGRESSION_LIMIT_SECONDS:
            errors.append("independent regression-suite must complete in under 10 seconds")
        if check.get("status") != "passed" or check.get("exit_code") != 0:
            errors.append(f"verification check did not pass: {name or index}")
    if "regression-suite" not in seen:
        errors.append("verification must independently time regression-suite")

    for field in ("findings", "residual_uncertainty"):
        items = require_list(value.get(field), field, errors)
        for item in items:
            if placeholder(item):
                errors.append(f"{field} contains a placeholder")
    artifacts = require_list(value.get("artifacts"), "artifacts", errors)
    for index, artifact in enumerate(artifacts):
        if not isinstance(artifact, dict):
            errors.append(f"artifacts[{index}] must be an object")
            continue
        require_text(artifact.get("path"), f"artifacts[{index}].path", errors)
        digest = require_text(artifact.get("sha256"), f"artifacts[{index}].sha256", errors)
        if digest and (len(digest) != 64 or any(character not in "0123456789abcdefABCDEF" for character in digest)):
            errors.append(f"artifacts[{index}].sha256 must be a SHA-256 digest")
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
    print("Rust verification receipt is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
