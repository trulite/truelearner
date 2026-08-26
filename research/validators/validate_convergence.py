#!/usr/bin/env python3
"""Validate research fan-in, frozen failures, and new-arm lineage."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import load_toml, placeholder, require_list, require_text


OUTCOME_FIELDS = ("survived", "falsified", "inconclusive", "infrastructure_failed")


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-convergence/v1":
        errors.append("schema must be research-convergence/v1")
    require_text(value.get("campaign"), "campaign", errors)
    if not isinstance(value.get("round"), int) or value["round"] < 1:
        errors.append("round must be a positive integer")

    accounted: set[str] = set()
    falsified: list[str] = []
    for field in OUTCOME_FIELDS:
        arms = value.get(field)
        if not isinstance(arms, list):
            errors.append(f"{field} must be a list")
            continue
        for arm in arms:
            if placeholder(arm):
                errors.append(f"{field} contains a placeholder")
            elif arm in accounted:
                errors.append(f"arm appears in more than one outcome: {arm}")
            else:
                accounted.add(str(arm))
        if field == "falsified":
            falsified = [str(arm) for arm in arms]
    if not accounted:
        errors.append("convergence must account for at least one arm")
    for field in ("shared_mechanisms", "next_discriminators"):
        require_list(value.get(field), field, errors)
    if falsified:
        require_list(value.get("strongest_counterexamples"), "strongest_counterexamples", errors)

    new_arms = value.get("new_arm", [])
    if not isinstance(new_arms, list):
        errors.append("new_arm must be an array of tables")
    for index, arm in enumerate(new_arms):
        if not isinstance(arm, dict):
            errors.append(f"new_arm[{index}] must be a table")
            continue
        require_text(arm.get("id"), f"new_arm[{index}].id", errors)
        if arm.get("kind") not in {"diagnostic", "solve", "composition", "hybrid"}:
            errors.append(f"new_arm[{index}].kind is invalid")
        parents = require_list(arm.get("parents"), f"new_arm[{index}].parents", errors)
        for parent in parents:
            if parent not in accounted:
                errors.append(f"new_arm[{index}] has unknown parent: {parent}")
        require_list(arm.get("imports"), f"new_arm[{index}].imports", errors)
        require_text(arm.get("interaction_prediction"), f"new_arm[{index}].interaction_prediction", errors)
        require_list(arm.get("falsifiers"), f"new_arm[{index}].falsifiers", errors)
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
    print("Research convergence record is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
