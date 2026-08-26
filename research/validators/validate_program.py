#!/usr/bin/env python3
"""Validate a research program, claim graph, and active frontier."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import load_toml, placeholder, require_list, require_text


STATUSES = {
    "proposed",
    "preregistered",
    "executed",
    "positive",
    "negative",
    "inconclusive",
    "audited",
    "authoritative",
    "retired",
}


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-program/v1":
        errors.append("schema must be research-program/v1")
    require_text(value.get("id"), "id", errors)
    require_text(value.get("thesis"), "thesis", errors)
    require_text(value.get("pre_frontier_checkpoint"), "pre_frontier_checkpoint", errors)

    claims_raw = require_list(value.get("claim"), "claim", errors)
    claims: dict[str, dict] = {}
    for index, claim in enumerate(claims_raw):
        if not isinstance(claim, dict):
            errors.append(f"claim[{index}] must be a table")
            continue
        claim_id = require_text(claim.get("id"), f"claim[{index}].id", errors)
        require_text(claim.get("statement"), f"claim[{index}].statement", errors)
        if claim.get("status") not in STATUSES:
            errors.append(f"claim[{index}].status is invalid")
        require_list(claim.get("falsifiers"), f"claim[{index}].falsifiers", errors)
        require_list(claim.get("limitations"), f"claim[{index}].limitations", errors)
        dependencies = claim.get("depends_on", [])
        if not isinstance(dependencies, list):
            errors.append(f"claim[{index}].depends_on must be a list")
        if claim_id:
            if claim_id in claims:
                errors.append(f"duplicate claim id: {claim_id}")
            claims[claim_id] = claim

    frontier = require_list(value.get("active_frontier"), "active_frontier", errors)
    for claim_id in frontier:
        if placeholder(claim_id) or claim_id not in claims:
            errors.append(f"active frontier references unknown claim: {claim_id}")
    for claim_id, claim in claims.items():
        for dependency in claim.get("depends_on", []):
            if dependency not in claims:
                errors.append(f"claim {claim_id} depends on unknown claim: {dependency}")

    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(claim_id: str) -> None:
        if claim_id in visiting:
            errors.append(f"claim dependency cycle includes: {claim_id}")
            return
        if claim_id in visited:
            return
        visiting.add(claim_id)
        for dependency in claims.get(claim_id, {}).get("depends_on", []):
            if dependency in claims:
                visit(dependency)
        visiting.remove(claim_id)
        visited.add(claim_id)

    for claim_id in claims:
        visit(claim_id)
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
    print("Research program is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
