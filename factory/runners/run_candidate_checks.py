#!/usr/bin/env python3
"""Run exact candidate checks and generate a content-addressed receipt."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import time
from typing import Any


REQUIRED_CHECKS = {"fmt", "check", "clippy", "focused-tests", "regression-suite"}
REGRESSION_LIMIT_SECONDS = 10.0


def output(*args: str, cwd: Path, text: bool = True) -> Any:
    return subprocess.check_output(args, cwd=cwd, text=text).strip() if text else subprocess.check_output(args, cwd=cwd)


def git_paths(cwd: Path, base: str | None) -> list[str]:
    comparison = f"{base}..HEAD" if base else "HEAD"
    tracked = output("git", "diff", "--name-only", comparison, cwd=cwd).splitlines()
    working = output("git", "diff", "--name-only", cwd=cwd).splitlines()
    staged = output("git", "diff", "--cached", "--name-only", cwd=cwd).splitlines()
    untracked = output("git", "ls-files", "--others", "--exclude-standard", cwd=cwd).splitlines()
    return sorted(set(tracked + working + staged + untracked))


def tree_digest(cwd: Path, base: str | None, paths: list[str]) -> str:
    comparison = f"{base}..HEAD" if base else "HEAD"
    digest = hashlib.sha256()
    digest.update(output("git", "diff", "--binary", comparison, cwd=cwd, text=False))
    digest.update(output("git", "diff", "--binary", cwd=cwd, text=False))
    digest.update(output("git", "diff", "--cached", "--binary", cwd=cwd, text=False))
    tracked = set(output("git", "ls-files", cwd=cwd).splitlines())
    for name in paths:
        if name not in tracked:
            digest.update(name.encode())
            digest.update((cwd / name).read_bytes())
    return digest.hexdigest()


def parse_check(value: str) -> tuple[str, str]:
    name, separator, command = value.partition("=")
    if not separator or not name or not command:
        raise argparse.ArgumentTypeError("checks must use NAME=COMMAND")
    return name, command


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plan", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--cwd", type=Path, default=Path.cwd())
    parser.add_argument("--base", help="optional Git base revision for committed changes")
    parser.add_argument("--scope", action="append", required=True)
    parser.add_argument("--check", action="append", type=parse_check, required=True)
    args = parser.parse_args()

    cwd = args.cwd.resolve()
    plan = args.plan.resolve()
    checks = dict(args.check)
    missing = REQUIRED_CHECKS - checks.keys()
    if missing:
        raise SystemExit("missing checks: " + ", ".join(sorted(missing)))

    changed_paths = git_paths(cwd, args.base)
    started = datetime.now(timezone.utc).isoformat()
    results: list[dict[str, object]] = []
    exit_code = 0
    for name, command in args.check:
        check_started = time.perf_counter()
        result = subprocess.run(shlex.split(command), cwd=cwd, capture_output=True, text=True)
        duration_seconds = time.perf_counter() - check_started
        result_digest = hashlib.sha256((result.stdout + "\n" + result.stderr).encode()).hexdigest()
        within_budget = name != "regression-suite" or duration_seconds < REGRESSION_LIMIT_SECONDS
        results.append(
            {
                "name": name,
                "command": command,
                "status": "passed" if result.returncode == 0 and within_budget else "failed",
                "exit_code": result.returncode,
                "duration_seconds": round(duration_seconds, 6),
                "output_sha256": result_digest,
            }
        )
        if result.returncode != 0 or not within_budget:
            exit_code = result.returncode or 2
            break

    receipt = {
        "schema": "rust-candidate/v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "started_at": started,
        "plan": {
            "path": str(plan),
            "sha256": hashlib.sha256(plan.read_bytes()).hexdigest(),
        },
        "candidate": {
            "revision": output("git", "rev-parse", "HEAD", cwd=cwd),
            "tree_sha256": tree_digest(cwd, args.base, changed_paths),
        },
        "scope": args.scope,
        "changed_paths": changed_paths,
        "checks": results,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print(args.output)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
