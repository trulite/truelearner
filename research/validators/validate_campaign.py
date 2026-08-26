#!/usr/bin/env python3
"""Validate a parallel falsification-first campaign and all declared arms."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import artifact_path, load_toml, placeholder, require_digest, require_list, require_text


ARM_KINDS = {"diagnostic", "solve", "control", "composition", "hybrid"}


def validate_arm(path: Path, campaign_id: str) -> tuple[list[str], str]:
    errors: list[str] = []
    try:
        arm = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)], ""
    if arm.get("schema") != "research-arm/v1":
        errors.append("schema must be research-arm/v1")
    arm_id = require_text(arm.get("id"), "id", errors)
    if arm.get("campaign") != campaign_id:
        errors.append("campaign does not match parent campaign")
    if arm.get("kind") not in ARM_KINDS:
        errors.append("kind is invalid")
    require_text(arm.get("mechanism"), "mechanism", errors)
    require_text(arm.get("prediction"), "prediction", errors)
    require_list(arm.get("falsifiers"), "falsifiers", errors)
    require_text(arm.get("source_revision"), "source_revision", errors)
    require_digest(arm.get("protocol_sha256"), "protocol_sha256", errors)
    budget = arm.get("budget_minutes")
    if not isinstance(budget, int) or budget < 1:
        errors.append("budget_minutes must be a positive integer")
    gates = arm.get("gates")
    if not isinstance(gates, dict):
        errors.append("gates must be a table")
    else:
        require_text(gates.get("tiny_fixture"), "gates.tiny_fixture", errors)
        require_text(gates.get("full_evidence"), "gates.full_evidence", errors)
    return errors, arm_id


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-campaign/v1":
        errors.append("schema must be research-campaign/v1")
    campaign_id = require_text(value.get("id"), "id", errors)
    require_text(value.get("program"), "program", errors)
    kind = value.get("kind")
    if kind not in {"diagnostic", "solve"}:
        errors.append("kind must be diagnostic or solve")
    if value.get("mode") not in {"discovery", "authority"}:
        errors.append("mode must be discovery or authority")
    require_text(value.get("hypothesis"), "hypothesis", errors)
    divergence = value.get("first_divergence")
    if kind == "solve":
        require_text(divergence, "first_divergence", errors)
        require_text(value.get("missing_transition"), "missing_transition", errors)
    elif divergence not in {"unknown", None} and placeholder(divergence):
        errors.append("diagnostic first_divergence must be unknown or meaningful")
    require_text(value.get("prediction"), "prediction", errors)
    require_list(value.get("falsifiers"), "falsifiers", errors)
    require_text(value.get("positive_reference"), "positive_reference", errors)
    require_list(value.get("negative_controls"), "negative_controls", errors)

    maximum = value.get("max_parallel_arms")
    if not isinstance(maximum, int) or maximum < 2:
        errors.append("max_parallel_arms must be at least 2")
    rounds = value.get("max_rounds")
    if not isinstance(rounds, int) or rounds < 1:
        errors.append("max_rounds must be positive")
    budget = value.get("budget")
    if not isinstance(budget, dict) or any(
        not isinstance(budget.get(field), int) or budget[field] < 1
        for field in ("total_sandbox_minutes", "max_minutes_per_arm")
    ):
        errors.append("budget must contain positive sandbox minute limits")
    preflight = value.get("preflight")
    if not isinstance(preflight, dict) or not all(
        preflight.get(field) is True
        for field in ("tiny_fixture", "hidden_authority_audit", "reference_replay_equality", "natural_quiescence")
    ):
        errors.append("all preflight integrity gates must be enabled")
    convergence = value.get("convergence")
    if not isinstance(convergence, dict) or convergence.get("after_each_round") is not True:
        errors.append("convergence.after_each_round must be true")
    authority = value.get("authority")
    if not isinstance(authority, dict) or authority.get("fresh_sandbox") is not True:
        errors.append("authority evidence must use a fresh sandbox")
    elif authority.get("max_valid_runs") != 1 or authority.get("requires_frozen_protocol") is not True:
        errors.append("authority must require one valid run from a frozen protocol")

    arm_paths = require_list(value.get("arm_paths"), "arm_paths", errors)
    if len(arm_paths) < 2:
        errors.append("a campaign must declare at least two parallel arms")
    arm_ids: set[str] = set()
    kinds: set[str] = set()
    for arm_path in arm_paths:
        if placeholder(arm_path):
            errors.append("arm_paths contains a placeholder")
            continue
        resolved = artifact_path(path, arm_path)
        arm_errors, arm_id = validate_arm(resolved, campaign_id)
        errors.extend(f"arm {arm_path}: {item}" for item in arm_errors)
        if arm_id:
            if arm_id in arm_ids:
                errors.append(f"duplicate arm id: {arm_id}")
            arm_ids.add(arm_id)
        try:
            kinds.add(str(load_toml(resolved).get("kind")))
        except (OSError, ValueError):
            pass
    if value.get("interaction_expected") is True and not kinds.intersection({"composition", "hybrid"}):
        errors.append("interaction_expected requires a composition or hybrid arm")
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
    print("Research campaign is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
