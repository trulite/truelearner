#!/usr/bin/env python3
"""Validate the decision-bearing surface of a Rust factory plan."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


REQUIRED = (
    "Outcome",
    "Authority",
    "Model",
    "Invariants",
    "Scope",
    "Development style",
    "Focused tests",
    "Development loop",
    "Controls and evidence",
    "Risks and rollback",
    "Open decisions",
)
HEADING = re.compile(r"^##\s+(.+?)\s*$")
FENCE = re.compile(r"^\s*```")
PLACEHOLDERS = {"", "tbd", "todo", "n/a", "na", "-", "...", "?"}


def split_sections(text: str) -> dict[str, str]:
    sections: dict[str, str] = {}
    current: str | None = None
    lines: list[str] = []
    in_fence = False
    for line in text.splitlines():
        if FENCE.match(line):
            in_fence = not in_fence
        match = None if in_fence else HEADING.match(line)
        if match:
            if current is not None:
                sections[current] = "\n".join(lines).strip()
            current = match.group(1).strip()
            lines = []
        elif current is not None:
            lines.append(line)
    if current is not None:
        sections[current] = "\n".join(lines).strip()
    return sections


def placeholder(value: str) -> bool:
    stripped = value.strip().strip("-*` ").lower()
    return stripped in PLACEHOLDERS or re.search(r"<[^>]+>", value) is not None


def validate(text: str) -> list[str]:
    sections = {key.lower(): value for key, value in split_sections(text).items()}
    errors: list[str] = []
    for heading in REQUIRED:
        body = sections.get(heading.lower())
        if body is None:
            errors.append(f"missing heading: {heading}")
        elif placeholder(body):
            errors.append(f"empty or placeholder section: {heading}")

    authority = sections.get("authority", "").lower()
    if "path:" not in authority or "revision:" not in authority:
        errors.append("Authority must identify Path and Revision")

    style = sections.get("development style", "").lower()
    if "tdd" not in style and "implementation-first" not in style:
        errors.append("Development style must select TDD or implementation-first")

    tests = sections.get("focused tests", "").lower()
    if "cargo " not in tests and "python" not in tests and "./" not in tests:
        errors.append("Focused tests must name at least one exact command")

    loop = sections.get("development loop", "").lower()
    if "regression" not in loop:
        errors.append("Development loop must name the regression suite")
    if not re.search(r"(?:under|<)\s*10(?:\.0+)?\s*(?:s|sec|secs|second|seconds)\b", loop):
        errors.append("Development loop must set a strict under-10-second budget")

    controls = sections.get("controls and evidence", "").lower()
    if "held-out" not in controls and "not applicable because" not in controls:
        errors.append("Controls and evidence must address held-out cases")
    if "negative control" not in controls and "not applicable because" not in controls:
        errors.append("Controls and evidence must address negative controls")

    open_decisions = sections.get("open decisions", "").strip().lower().rstrip(".")
    if open_decisions not in {"none", "no open decisions"}:
        errors.append("Open decisions must be resolved before handoff")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", required=True, type=Path)
    args = parser.parse_args()
    errors = validate(args.file.read_text(encoding="utf-8"))
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    print("Rust plan is valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
