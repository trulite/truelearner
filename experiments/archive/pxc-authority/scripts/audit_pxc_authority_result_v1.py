#!/usr/bin/env python3
"""Audit serialized PX-C authority evidence without executing the runtime."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
CSV = ROOT / "results/pxc_continuous_authority_v1.csv"
REPORT = ROOT / "results/pxc_continuous_authority_v1.md"
DEVELOPMENT = ROOT / "results/pxc_continuous_development_v1.csv"
FIREWALL = ROOT / "results/pxc_authority_firewall_v1/audit.json"
OUTPUT = ROOT / "results/pxc_authority_result_audit_v1"
OBSERVATIONS = [
    "batches", "paired_held", "selective_held", "retained", "partial",
    "adjacent_first", "adjacent_second", "duplicated", "resisted", "direct",
    "duplicate_direct", "open", "fork", "ring", "aged", "formation_updates",
    "formation_modulation", "age_deallocations", "max_work", "max_bytes",
    "batch_outputs", "batch_work", "batch_bytes", "all_quiet", "outward_only",
    "replay", "clauses", "passed",
]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    rows = list(csv.DictReader(CSV.open()))
    development = list(csv.DictReader(DEVELOPMENT.open()))
    firewall = json.loads(FIREWALL.read_text())
    report = REPORT.read_text()
    roots = [int(row["root"]) for row in rows]
    origins = [int(row["origin"]) for row in rows]
    clauses = [row["clauses"].split("|") for row in rows]
    observations = [tuple(row[key] for key in OBSERVATIONS) for row in rows]
    development_observations = [tuple(row[key] for key in OBSERVATIONS) for row in development]
    result = {
        "rows": len(rows),
        "roots_exact": roots == list(range(3_200_001, 3_200_017)),
        "origins_exact": origins == [520, 650, 780, 910] * 4,
        "moduli_exact": [origin % 10 for origin in origins] == [0] * 16,
        "timing_exact": len(rows) == 16 and all(row["construction_tick"] == row["pressure_origin"] == row["first_arrival_tick"] == row["origin"] for row in rows),
        "clauses_exact": len(clauses) == 16 and all(len(values) == 32 and all(value == "true" for value in values) for values in clauses),
        "rows_pass": len(rows) == 16 and all(row["passed"] == row["all_quiet"] == row["outward_only"] == row["replay"] == "true" for row in rows),
        "bounds_exact": len(rows) == 16 and max(int(row["max_work"]) for row in rows) == 105446 and max(int(row["max_bytes"]) for row in rows) == 39248,
        "authority_invariance": len(set(observations)) == 1,
        "development_authority_observations_exact": len(set(development_observations + observations)) == 1,
        "development_authority_roots_disjoint": not {int(row["root"]) for row in development} & set(roots),
        "report_exact": all(token in report for token in ["AUTHORITY ESTABLISHED", "rows: `16/16`", "row clauses: `512/512`", "global clauses: `12/12`", "total clauses: `524/524`", "maximum per-batch work: `105446`", "maximum resident bytes: `39248`", "natural quiescence: `true`", "outward-only boundary: `true`", "exact replay: `true`"]),
        "firewall_pass": firewall.get("gate_pass") is True,
        "csv_sha256": sha(CSV),
        "report_sha256": sha(REPORT),
        "firewall_sha256": sha(FIREWALL),
        "runtime_sha256": sha(ROOT / "crates/pxr0-physical-runtime/src/lib.rs"),
        "evaluator_sha256": sha(ROOT / "arms/pxc-continuous-organism/src/main.rs"),
        "active_gate_sha256": sha(ROOT / "results/pxc_active_gate_v1/audit.json"),
    }
    result["gate_pass"] = (
        result["rows"] == 16 and result["roots_exact"] and result["origins_exact"]
        and result["moduli_exact"] and result["timing_exact"] and result["clauses_exact"]
        and result["rows_pass"] and result["bounds_exact"] and result["authority_invariance"]
        and result["development_authority_observations_exact"]
        and result["development_authority_roots_disjoint"] and result["report_exact"]
        and result["firewall_pass"]
        and result["runtime_sha256"] == "e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa"
        and result["evaluator_sha256"] == "55f6ad153f58b803d587814ed554521689b2586da4cac60805f600c73f06fb6d"
        and result["active_gate_sha256"] == "ddde757611b7cb106271d37ec1249be4e79a409af1a3fafab5e910241afe14c8"
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    lines = [
        "# PX-C authority result audit v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- rows / row clauses / global clauses: `{result['rows']}/16` / `512/512` / `12/12`;",
        f"- authority invariance / development agreement: `{result['authority_invariance']}/{result['development_authority_observations_exact']}`;",
        f"- replay / quiescence / outward-only: `{result['rows_pass']}`;",
        f"- maximum work / resident bytes: `105446/200000` / `39248/65536`;",
        f"- CSV SHA-256: `{result['csv_sha256']}`;",
        f"- report SHA-256: `{result['report_sha256']}`;",
        f"- firewall SHA-256: `{result['firewall_sha256']}`.", "",
        "`PXC_AUTHORITY_RESULT_AUDIT_V1_OK`" if result["gate_pass"] else "`PXC_AUTHORITY_RESULT_AUDIT_V1_FAIL`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(lines) + "\n")
    print(json.dumps(result, sort_keys=True))
    if result["gate_pass"]:
        print("PXC_AUTHORITY_RESULT_AUDIT_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
