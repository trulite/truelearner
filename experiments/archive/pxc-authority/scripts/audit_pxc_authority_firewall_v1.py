#!/usr/bin/env python3
"""Bind immutable PX-C development evidence before the authority spend."""

from __future__ import annotations

import csv
import hashlib
import json
import os
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "results/pxc_authority_firewall_v1"
EXPECTED = {
    "crates/pxr0-physical-runtime/src/lib.rs": "e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa",
    "arms/pxc-continuous-organism/src/main.rs": "55f6ad153f58b803d587814ed554521689b2586da4cac60805f600c73f06fb6d",
    "experiments/pxc_active_runtime_spec_v1.json": "e3b001695ac8bb8bb1c12d41fede6bc2a3668f952f26809cd3815184e97919f0",
    "experiments/pxc_continuous_active_surface_manifest_v1.csv": "2fea14579f6d4c7ca99bda1f112fc2f570b11bceac897f6d9c0df255ae35f140",
    "output/pdf/pxc_active_runtime_spec_v1.pdf": "01e7827b58deb52a14d12d11ea5a25e313e13013819e9220fd9c14b41ad15958",
    "results/pxc_active_gate_v1/audit.json": "ddde757611b7cb106271d37ec1249be4e79a409af1a3fafab5e910241afe14c8",
    "results/pxc_harness_audit_v1/audit.json": "f4b9a465a635508f24779810c4946a7ba5579e1e6c157ac6647cbb71f7317e53",
    "results/pxc_taxonomy_v1/pxc_seam_taxonomy_summary_v2.csv": "55a318766e289645a0da947f3cdfeeac82d3c3aa39744a2a68ff910746c911db",
    "results/pxc_continuous_development_v1.csv": "295a28fca72f560d87259624d7f20a171986582c818faa42bbb594399fcd5c89",
    "results/pxc_continuous_development_v1.md": "22ba439015ca5684c542fefe75983ddcd85f1a43c1b03ce6b13f1a64fe37cb88",
    "experiments/pxc_continuous_organism_development_result_v1.md": "f48ed9a9f68d039aa3caa4d8c2de783f8643a45f2ce5121694004a9233f47289",
    "results/pxr0_v2_acceptance_v1/audit.json": "fb30e4db84d5e1396b8751be16d83ca2c9ef2315f8aaee4e8a1d419630e846a7",
    "results/pxr0_successor_readiness_v2.csv": "d1bf714bdf24bbee10c362727abec02f42066cedd05ee807c88ef2c645a96d5e",
    "results/pxr0_phase_controls_v2.csv": "6900a8d6a5a504bed95ea729acec522c5cf28e30169779cad9d34f76588fbb7f",
}
AUTHORITY_OUTPUTS = [
    "results/pxc_continuous_authority_v1.csv",
    "results/pxc_continuous_authority_v1.md",
]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def true(value: str) -> bool:
    return value == "true"


def main() -> int:
    hashes = {path: sha(ROOT / path) for path in EXPECTED}
    rows = list(csv.DictReader((ROOT / "results/pxc_continuous_development_v1.csv").open()))
    roots = [int(row["root"]) for row in rows]
    origins = [int(row["origin"]) for row in rows]
    clauses = [row["clauses"].split("|") for row in rows]
    active = json.loads((ROOT / "results/pxc_active_gate_v1/audit.json").read_text())
    harness = json.loads((ROOT / "results/pxc_harness_audit_v1/audit.json").read_text())
    taxonomy = {row["metric"]: int(row["count"]) for row in csv.DictReader((ROOT / "results/pxc_taxonomy_v1/pxc_seam_taxonomy_summary_v2.csv").open())}
    evaluator = (ROOT / "arms/pxc-continuous-organism/src/main.rs").read_text()
    result = {
        "authority_protocol_commit": os.environ.get("PXC_AUTHORITY_PROTOCOL_COMMIT", ""),
        "hashes": hashes,
        "hashes_exact": hashes == EXPECTED,
        "development_rows": len(rows),
        "development_roots_exact": roots == list(range(3_100_001, 3_100_017)),
        "development_origins_exact": origins == [0, 130, 260, 390] * 4,
        "development_clauses_exact": len(clauses) == 16 and all(len(values) == 32 and all(value == "true" for value in values) for values in clauses),
        "development_rows_pass": len(rows) == 16 and all(true(row["passed"]) and true(row["all_quiet"]) and true(row["outward_only"]) and true(row["replay"]) for row in rows),
        "development_bounds": len(rows) == 16 and max(int(row["max_work"]) for row in rows) == 105446 and max(int(row["max_bytes"]) for row in rows) == 39248,
        "active_gate_pass": active.get("gate_pass") is True and active.get("entries") == 29 and active.get("pdf_pages") == 1,
        "zero_surface_gate": all(active.get(key) == 0 for key in ["primary_seams", "semantic_guard", "evaluator_guard", "new_kinds", "new_guarded_surfaces"]),
        "harness_gate_pass": harness.get("gate_pass") is True and harness.get("namespaces_disjoint") is True,
        "taxonomy_zero": bool(taxonomy) and all(value == 0 for value in taxonomy.values()),
        "authority_outputs_absent": all(not (ROOT / path).exists() for path in AUTHORITY_OUTPUTS),
        "authority_staging_absent": all(not (ROOT / f"{path}.staging").exists() for path in AUTHORITY_OUTPUTS),
        "authority_mode_frozen": "PXC_CONTINUOUS_AUTHORITY_EVIDENCE_SPENT_V1" in evaluator and "--authority" in evaluator,
    }
    result["gate_pass"] = (
        bool(result["authority_protocol_commit"]) and result["hashes_exact"]
        and result["development_rows"] == 16 and result["development_roots_exact"]
        and result["development_origins_exact"] and result["development_clauses_exact"]
        and result["development_rows_pass"] and result["development_bounds"]
        and result["active_gate_pass"] and result["zero_surface_gate"]
        and result["harness_gate_pass"] and result["taxonomy_zero"]
        and result["authority_outputs_absent"] and result["authority_staging_absent"]
        and result["authority_mode_frozen"]
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    lines = [
        "# PX-C authority firewall v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- bound artifacts exact: `{result['hashes_exact']}`;",
        f"- development rows/clauses/pass: `{result['development_rows']}/16` / `{result['development_clauses_exact']}` / `{result['development_rows_pass']}`;",
        f"- active/harness/taxonomy-zero: `{result['active_gate_pass']}/{result['harness_gate_pass']}/{result['taxonomy_zero']}`;",
        f"- primary/semantic/evaluator/new-kind/new-surface zero: `{result['zero_surface_gate']}`;",
        f"- authority outputs/staging absent: `{result['authority_outputs_absent']}/{result['authority_staging_absent']}`.", "",
        "`PXC_AUTHORITY_FIREWALL_V1_OK`" if result["gate_pass"] else "`PXC_AUTHORITY_FIREWALL_V1_FAIL`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(lines) + "\n")
    print(json.dumps(result, sort_keys=True))
    if result["gate_pass"]:
        print("PXC_AUTHORITY_FIREWALL_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
