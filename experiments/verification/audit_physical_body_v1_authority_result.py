#!/usr/bin/env python3
"""Non-executing audit of the frozen Physical Body V1 authority artifacts."""

from __future__ import annotations

import csv
import hashlib
import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = ROOT / "experiments/results/physical_body_v1_authority.csv"
REPORT_PATH = ROOT / "experiments/results/physical_body_v1_authority.md"
OUTPUT_PATH = ROOT / "experiments/results/physical_body_v1_authority_audit.json"

EXPECTED_HASHES = {
    "authority_csv": "37b668f498881ceea60b9a910b34c0b11ca3499e093bad4d90aad984ecc4aad0",
    "authority_report": "170cd1429b1852534dc2650b423128590fec67930dd91e56f2d0f0e80584955b",
    "core": "e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d",
    "arena_format": "8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812",
    "evaluator": "2216d171756c44219b0949c265d037a9564c7f284833c1423851293f0f04613c",
    "body_evaluator": "4a3cfc69eced9097f73bfb4efe973cded40a2d18298e74eed1c733bd87cf706a",
}

PATHS = {
    "authority_csv": CSV_PATH,
    "authority_report": REPORT_PATH,
    "core": ROOT / "truelearner/crates/core/src/lib.rs",
    "arena_format": ROOT / "truelearner/crates/arena-format/src/lib.rs",
    "evaluator": ROOT
    / "experiments/verification/physical-body-v1-authority/src/main.rs",
    "body_evaluator": ROOT
    / "experiments/verification/physical-body-v1-authority/src/body.rs",
}

BODY_CLAUSES = [
    "canonical_arena_round_trip",
    "equivalent_arena_hash",
    "canonical_manifest_hash",
    "cell_reference_compaction",
    "arrow_reference_compaction",
    "compaction_behavior",
    "quiescent_checkpoint_round_trip",
    "quiescent_clock_phase",
    "quiescent_future_behavior",
    "live_checkpoint_round_trip",
    "live_pending_continuation",
    "bounded_capacity",
    "deterministic_reuse_generation",
    "stale_reference_rejected",
    "corrupt_arena_rejected",
    "stale_durable_reference_rejected",
]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def truth_vector(value: str, length: int) -> bool:
    parts = value.split("|")
    return len(parts) == length and all(part == "true" for part in parts)


def main() -> None:
    checks: dict[str, bool] = {}
    observed_hashes = {name: digest(path) for name, path in PATHS.items()}
    checks["hashes_exact"] = observed_hashes == EXPECTED_HASHES

    with CSV_PATH.open(newline="") as source:
        rows = list(csv.DictReader(source))

    expected_roots = list(range(4_100_001, 4_100_017))
    observed_roots = [int(row["root"]) for row in rows]
    checks["rows_and_roots"] = len(rows) == 16 and observed_roots == expected_roots
    checks["origins"] = Counter(int(row["origin"]) for row in rows) == Counter(
        {1_040: 4, 1_170: 4, 1_300: 4, 1_430: 4}
    )
    checks["layouts"] = Counter(
        (row["reverse"], row["reflect"]) for row in rows
    ) == Counter(
        {
            ("false", "false"): 4,
            ("true", "false"): 4,
            ("false", "true"): 4,
            ("true", "true"): 4,
        }
    )
    checks["phase_geometry"] = all(
        int(row["origin"]) % 10 == 0
        and row["construction_tick"] == row["origin"]
        and row["pressure_origin"] == row["origin"]
        and row["first_arrival_tick"] == row["origin"]
        for row in rows
    )
    checks["row_clauses"] = all(truth_vector(row["clauses"], 32) for row in rows)
    checks["row_verdicts"] = all(
        row["passed"] == "true"
        and row["replay"] == "true"
        and row["all_quiet"] == "true"
        and row["outward_only"] == "true"
        for row in rows
    )
    checks["bounds"] = all(
        int(row["max_work"]) <= 200_000 and int(row["max_bytes"]) <= 65_536
        for row in rows
    )
    checks["retained_contrasts"] = all(
        row["paired_held"] == "1"
        and row["selective_held"] == "1"
        and row["retained"] == "1"
        and row["partial"] == "0"
        and row["adjacent_first"] == "0"
        and row["adjacent_second"] == "0"
        and row["duplicated"] == "1"
        and row["resisted"] == "0"
        and row["direct"] == "1"
        and row["duplicate_direct"] == "1"
        and row["open"] == "0"
        and row["fork"] == "0"
        and row["ring"] == "0"
        and row["aged"] == "0"
        for row in rows
    )

    report = REPORT_PATH.read_text()
    checks["report_totals"] = all(
        expected in report
        for expected in (
            "Outcome: **AUTHORITY ESTABLISHED**.",
            "- rows: `16/16`;",
            "- row clauses: `512/512`;",
            "- cumulative global clauses: `12/12`;",
            "- physical-body clauses: `16/16`;",
            "- total clauses: `540/540`;",
            "`true|true|true|true|true|true|true|true|true|true|true|true`",
        )
    )
    checks["body_clauses"] = all(f"- `{name}`: `true`;" in report for name in BODY_CLAUSES)

    gate_pass = all(checks.values())
    result = {
        "gate_pass": gate_pass,
        "evidence_commit": "4b2c773",
        "rows": len(rows),
        "row_clauses": 512,
        "cumulative_global_clauses": 12,
        "physical_body_clauses": 16,
        "total_clauses": 540,
        "maximum_work": max(int(row["max_work"]) for row in rows),
        "maximum_bytes": max(int(row["max_bytes"]) for row in rows),
        "hashes": observed_hashes,
        "checks": checks,
    }
    OUTPUT_PATH.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    if not gate_pass:
        raise SystemExit("PHYSICAL_BODY_V1_AUTHORITY_RESULT_AUDIT_FAIL")
    print("PHYSICAL_BODY_V1_AUTHORITY_RESULT_AUDIT_PASS")


if __name__ == "__main__":
    main()
