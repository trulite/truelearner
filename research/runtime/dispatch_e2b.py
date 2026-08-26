#!/usr/bin/env python3
"""Dispatch independent E2B arm commands concurrently and collect neutral results.

The batch manifest is an adapter. It is deliberately separate from research
program, protocol, and evidence schemas, and it never imports factory state.
"""

from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib
from typing import Any


OUTCOMES = {"survived", "falsified", "inconclusive", "infrastructure-failed"}


def digest(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def resolve(parent: Path, value: object) -> Path:
    path = Path(str(value))
    return path if path.is_absolute() else (parent / path).resolve()


def validate_batch(path: Path) -> tuple[dict[str, Any], list[str]]:
    errors: list[str] = []
    try:
        value = tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        return {}, [str(error)]
    if value.get("schema") != "research-e2b-batch/v1":
        errors.append("schema must be research-e2b-batch/v1")
    if not isinstance(value.get("id"), str) or not value["id"].strip("<> "):
        errors.append("id must be non-placeholder text")
    mode = value.get("mode")
    if mode not in {"discovery", "authority"}:
        errors.append("mode must be discovery or authority")
    maximum = value.get("max_parallel")
    if not isinstance(maximum, int) or maximum < 1:
        errors.append("max_parallel must be positive")
    if mode == "authority" and maximum != 1:
        errors.append("authority batches must run exactly one arm")
    if not isinstance(value.get("output_directory"), str):
        errors.append("output_directory must be set")

    arms = value.get("arm")
    if not isinstance(arms, list) or not arms:
        errors.append("arm must contain at least one table")
        arms = []
    ids: set[str] = set()
    for index, arm in enumerate(arms):
        if not isinstance(arm, dict):
            errors.append(f"arm[{index}] must be a table")
            continue
        arm_id = arm.get("id")
        if not isinstance(arm_id, str) or not arm_id.strip("<> "):
            errors.append(f"arm[{index}].id must be non-placeholder text")
        elif arm_id in ids:
            errors.append(f"duplicate arm id: {arm_id}")
        else:
            ids.add(arm_id)
        cwd = arm.get("cwd")
        if not isinstance(cwd, str) or not resolve(path.parent, cwd).is_dir():
            errors.append(f"arm[{index}].cwd must be an existing directory")
        command = arm.get("command")
        if not isinstance(command, list) or not command or not all(isinstance(item, str) and item for item in command):
            errors.append(f"arm[{index}].command must be a non-empty string array")
        elif not any("e2b" in item.lower() for item in command):
            errors.append(f"arm[{index}].command must invoke an E2B adapter")
        elif mode == "authority" and "--state-file" in command:
            errors.append(f"arm[{index}] authority command cannot reuse E2B state")
        protocol = arm.get("protocol_sha256")
        if not isinstance(protocol, str) or len(protocol) != 64:
            errors.append(f"arm[{index}].protocol_sha256 must be a SHA-256 digest")
        if not isinstance(arm.get("result_path"), str):
            errors.append(f"arm[{index}].result_path must be set")
        timeout = arm.get("timeout_seconds")
        if not isinstance(timeout, int) or timeout < 1:
            errors.append(f"arm[{index}].timeout_seconds must be positive")
    return value, errors


def read_result(path: Path, arm_id: str) -> tuple[str, dict[str, Any] | None, str | None]:
    if not path.is_file():
        return "unclassified", None, "arm did not produce its neutral result"
    try:
        result = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        return "unclassified", None, str(error)
    if not isinstance(result, dict) or result.get("schema") != "research-arm-result/v1":
        return "unclassified", None, "result schema must be research-arm-result/v1"
    if result.get("arm") != arm_id:
        return "unclassified", None, "result arm does not match batch arm"
    outcome = result.get("outcome")
    if outcome not in OUTCOMES:
        return "unclassified", None, "result outcome is invalid"
    return str(outcome), result, None


def run_arm(batch_path: Path, output_directory: Path, arm: dict[str, Any]) -> dict[str, Any]:
    arm_id = str(arm["id"])
    cwd = resolve(batch_path.parent, arm["cwd"])
    log_directory = output_directory / arm_id
    log_directory.mkdir(parents=True, exist_ok=True)
    started = datetime.now(timezone.utc).isoformat()
    try:
        completed = subprocess.run(
            arm["command"],
            cwd=cwd,
            capture_output=True,
            timeout=arm["timeout_seconds"],
        )
        stdout = completed.stdout or b""
        stderr = completed.stderr or b""
        exit_code: int | None = completed.returncode
        timeout = False
    except subprocess.TimeoutExpired as error:
        stdout = error.stdout or b""
        stderr = error.stderr or b""
        exit_code = None
        timeout = True
    (log_directory / "stdout.log").write_bytes(stdout)
    (log_directory / "stderr.log").write_bytes(stderr)

    result_path = resolve(cwd, arm["result_path"])
    outcome, result, result_error = read_result(result_path, arm_id)
    if timeout:
        outcome = "infrastructure-failed"
        result_error = "arm exceeded timeout"
    elif exit_code != 0 and result is None:
        outcome = "runner-failure"
        result_error = result_error or f"adapter exited {exit_code}"
    record: dict[str, Any] = {
        "arm": arm_id,
        "protocol_sha256": arm["protocol_sha256"],
        "started_at": started,
        "finished_at": datetime.now(timezone.utc).isoformat(),
        "adapter_exit_code": exit_code,
        "outcome": outcome,
        "stdout_sha256": digest(stdout),
        "stderr_sha256": digest(stderr),
    }
    if result_error:
        record["error"] = result_error
    if result is not None:
        record["result_sha256"] = digest(result_path.read_bytes())
    return record


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--batch", required=True, type=Path)
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    batch_path = args.batch.resolve()
    batch, errors = validate_batch(batch_path)
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    if args.dry_run:
        print(f"E2B batch is valid: {batch['id']} ({len(batch['arm'])} arms)")
        return 0

    output_directory = resolve(batch_path.parent, batch["output_directory"])
    output_directory.mkdir(parents=True, exist_ok=True)
    records: list[dict[str, Any]] = []
    with ThreadPoolExecutor(max_workers=batch["max_parallel"]) as executor:
        futures = {
            executor.submit(run_arm, batch_path, output_directory, arm): arm["id"]
            for arm in batch["arm"]
        }
        for future in as_completed(futures):
            records.append(future.result())
    records.sort(key=lambda item: str(item["arm"]))
    summary = {
        "schema": "research-e2b-batch-result/v1",
        "batch": batch["id"],
        "mode": batch["mode"],
        "completed_at": datetime.now(timezone.utc).isoformat(),
        "arms": records,
    }
    summary_path = output_directory / "batch-result.json"
    summary_path.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(summary_path)
    bad = {"runner-failure", "infrastructure-failed", "unclassified"}
    return 1 if any(record["outcome"] in bad for record in records) else 0


if __name__ == "__main__":
    raise SystemExit(main())
