#!/usr/bin/env python3
"""Validate a neutral frozen research protocol."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import load_toml, require_list, require_text


FORBIDDEN_COUPLING = ("$rust-plan", "$rust-implement", "$rust-verify", "factory/", "candidate receipt")


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-protocol/v1":
        errors.append("schema must be research-protocol/v1")
    for field in ("id", "program", "claim", "parent_authority", "question"):
        require_text(value.get(field), field, errors)
    if value.get("status") != "preregistered":
        errors.append("status must be preregistered before execution")
    for field in (
        "observables",
        "frozen_variables",
        "permitted_changes",
        "positive_predicates",
        "negative_controls",
        "stop_conditions",
    ):
        require_list(value.get(field), field, errors)
    policy = value.get("run_policy")
    if not isinstance(policy, dict):
        errors.append("run_policy must be a table")
    elif policy.get("max_valid_runs") != 1 or policy.get("fresh_environment") is not True:
        errors.append("run_policy must require one valid run in a fresh environment")
    lowered = path.read_text(encoding="utf-8").lower()
    for token in FORBIDDEN_COUPLING:
        if token in lowered:
            errors.append(f"protocol directly couples to implementation factory: {token}")
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
    print("Research protocol is valid and neutral.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
