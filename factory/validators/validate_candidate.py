#!/usr/bin/env python3
"""Validate a candidate receipt and its exact plan lineage."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import artifact_path, load_json, placeholder, require_list, require_text, sha256
from validate_plan import validate as validate_plan


REQUIRED_CHECKS = {"fmt", "check", "clippy", "focused-tests", "regression-suite"}
REGRESSION_LIMIT_SECONDS = 10.0


def within(path: str, scopes: list[str]) -> bool:
    normalized = path.strip("/")
    for scope in scopes:
        prefix = scope.strip("/")
        if prefix in {"", "."} or normalized == prefix or normalized.startswith(prefix + "/"):
            return True
    return False


def validate(receipt_path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_json(receipt_path)
    except (OSError, ValueError) as error:
        return [str(error)]

    if value.get("schema") != "rust-candidate/v1":
        errors.append("schema must be rust-candidate/v1")

    plan = value.get("plan")
    if not isinstance(plan, dict):
        errors.append("plan must be an object")
    else:
        path_text = require_text(plan.get("path"), "plan.path", errors)
        digest = require_text(plan.get("sha256"), "plan.sha256", errors)
        if path_text:
            path = artifact_path(receipt_path, path_text)
            if not path.is_file():
                errors.append(f"plan does not exist: {path}")
            else:
                if digest and sha256(path) != digest:
                    errors.append("plan.sha256 does not match plan content")
                errors.extend(f"plan: {item}" for item in validate_plan(path.read_text(encoding="utf-8")))

    candidate = value.get("candidate")
    if not isinstance(candidate, dict):
        errors.append("candidate must be an object")
    else:
        require_text(candidate.get("revision"), "candidate.revision", errors)
        require_text(candidate.get("tree_sha256"), "candidate.tree_sha256", errors)

    scopes_raw = require_list(value.get("scope"), "scope", errors)
    scopes = [str(item) for item in scopes_raw if not placeholder(item)]
    if len(scopes) != len(scopes_raw):
        errors.append("scope contains a placeholder")

    changed = require_list(value.get("changed_paths"), "changed_paths", errors)
    for item in changed:
        if placeholder(item):
            errors.append("changed_paths contains a placeholder")
        elif scopes and not within(str(item), scopes):
            errors.append(f"changed path is outside declared scope: {item}")

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
            errors.append("regression-suite must complete in under 10 seconds")
        if check.get("status") != "passed" or check.get("exit_code") != 0:
            errors.append(f"check did not pass: {name or index}")
    missing = REQUIRED_CHECKS - seen
    if missing:
        errors.append("missing required checks: " + ", ".join(sorted(missing)))
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
    print("Rust candidate receipt is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
