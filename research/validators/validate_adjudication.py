#!/usr/bin/env python3
"""Validate scientific adjudication against exact protocol and evidence."""

from __future__ import annotations

import argparse
from pathlib import Path

from common import artifact_path, load_toml, placeholder, require_list, require_text, sha256
from validate_evidence import validate as validate_evidence
from validate_protocol import validate as validate_protocol


VERDICTS = {"positive", "negative", "inconclusive"}


def validate(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        value = load_toml(path)
    except (OSError, ValueError) as error:
        return [str(error)]
    if value.get("schema") != "research-adjudication/v1":
        errors.append("schema must be research-adjudication/v1")

    protocol_text = require_text(value.get("protocol_path"), "protocol_path", errors)
    evidence_text = require_text(value.get("evidence_path"), "evidence_path", errors)
    protocol = artifact_path(path, protocol_text) if protocol_text else None
    evidence = artifact_path(path, evidence_text) if evidence_text else None
    if protocol is not None:
        if not protocol.is_file():
            errors.append(f"protocol does not exist: {protocol}")
        else:
            if value.get("protocol_sha256") != sha256(protocol):
                errors.append("protocol_sha256 does not match")
            errors.extend(f"protocol: {item}" for item in validate_protocol(protocol))
    if evidence is not None:
        if not evidence.is_file():
            errors.append(f"evidence does not exist: {evidence}")
        else:
            if value.get("evidence_sha256") != sha256(evidence):
                errors.append("evidence_sha256 does not match")
            errors.extend(f"evidence: {item}" for item in validate_evidence(evidence))
    if protocol and protocol.is_file() and evidence and evidence.is_file():
        try:
            protocol_digest = sha256(protocol)
            import json

            evidence_value = json.loads(evidence.read_text(encoding="utf-8"))
            if evidence_value.get("protocol_sha256") != protocol_digest:
                errors.append("evidence does not target the exact protocol")
        except (OSError, ValueError):
            pass

    if value.get("verdict") not in VERDICTS:
        errors.append("verdict must be positive, negative, or inconclusive")
    for field in ("predicate_results", "control_results", "scientific_findings", "residual_uncertainty"):
        items = require_list(value.get(field), field, errors)
        if any(placeholder(item) for item in items):
            errors.append(f"{field} contains a placeholder")

    sufficiency = value.get("sufficiency")
    integration = value.get("integration")
    adoption = value.get("adoption")
    authority = value.get("authority")
    if sufficiency not in {"established", "not-established"}:
        errors.append("sufficiency is invalid")
    if integration not in {"not-attempted", "passed", "failed"}:
        errors.append("integration is invalid")
    if adoption not in {"not-adopted", "adopted"}:
        errors.append("adoption is invalid")
    if authority not in {"not-promoted", "promoted"}:
        errors.append("authority is invalid")
    if adoption == "adopted" and integration != "passed":
        errors.append("adoption requires passed integration")
    if authority == "promoted":
        if adoption != "adopted" or value.get("verdict") != "positive":
            errors.append("authority promotion requires a positive adopted result")
        authorized_by = value.get("authorized_by")
        if not isinstance(authorized_by, str) or placeholder(authorized_by):
            errors.append("authority promotion requires explicit authorized_by")
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
    print("Research adjudication is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
