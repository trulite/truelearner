#!/usr/bin/env python3
"""Bind the canonical PXR0 v2 review to frozen source and development evidence."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
AUTHORITY = ROOT / "crates/lr1-modulatory-physical-return/src/lib.rs"
REVIEW = ROOT / "experiments/pxr0_v2_canonical_rust_review_v1.md"
OUTPUT = ROOT / "results/pxr0_v2_acceptance_v1"
EXPECTED = {
    "crates/pxr0-physical-runtime/src/lib.rs": "f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb",
    "crates/lr1-modulatory-physical-return/src/lib.rs": "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10",
    "experiments/pxr0_active_surface_manifest_v1.csv": "fc68a856cfd4524c55aef098927c7ad1bc1da628f879dce3270d2c741199998d",
    "experiments/pxr0_single_file_physical_runtime_spec_v1.json": "ccd1509df27ba09f4420dfec18585fe10ae787426bd76bba89ea14e9d1832462",
    "output/pdf/pxr0_single_file_physical_runtime_spec_v1.pdf": "3c5cb9277c9e01f77c9b37479c7e9f35f5de86b29fd5981c2327351113f553a0",
    "results/pxr0_successor_readiness_v2.csv": "d1bf714bdf24bbee10c362727abec02f42066cedd05ee807c88ef2c645a96d5e",
    "results/pxr0_phase_controls_v2.csv": "6900a8d6a5a504bed95ea729acec522c5cf28e30169779cad9d34f76588fbb7f",
    "results/pxr0_successor_readiness_v2.md": "82b234bb9db445922885af29fd1b31097057372dc8417ddaa88e75cea4758848",
    "results/pxr0_static_gate_v2/audit.json": "423759a2f53cf28b5a0296ef8a7ac7b58edd93578f9254fbfe28fc4f0c63f7de",
    "results/pxr0_static_gate_v2/inventory.csv": "c385c8046eea9eac920cca8902961ba9dccff86ee3d372195aef7a2d5de587b9",
    "results/pxr0_taxonomy_v2_rerun/pxc_seam_taxonomy_summary_v2.csv": "55a318766e289645a0da947f3cdfeeac82d3c3aa39744a2a68ff910746c911db",
    "results/pxr0_v2_harness_audit_v1/audit.json": "ea56af0f8accd0aeb5e185821c66fa1ff689c1239994f1006048163e7003c9cc",
    "experiments/pxr0_v2_canonical_rust_review_v1.md": "9252f3d9ed4415196dc6437b98e740209fda4768e61f300bbb6bd106c5149d7f",
}
TYPE_RE = re.compile(r"^(?:pub\s+)?(?:struct|enum|type)\s+([A-Za-z_]\w*)")
FUNCTION_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)")
IMPL_RE = re.compile(r"^impl\s+([A-Za-z_]\w*)")
EXPECTED_TYPES = {
    "CellId", "ArrowId", "CellSpec", "TransmissionMode", "ArrowSpec", "SpikeInput",
    "Cell", "Arrow", "Spike", "Crossing", "Work", "RunResult", "PlasticSubstrate",
}
EXPECTED_FUNCTIONS = {
    "Work::total", "PlasticSubstrate::new", "PlasticSubstrate::add_cell",
    "PlasticSubstrate::add_arrow", "PlasticSubstrate::enter",
    "PlasticSubstrate::advance_time", "PlasticSubstrate::propagate",
    "PlasticSubstrate::apply_modulatory_return", "PlasticSubstrate::elapse_to",
    "PlasticSubstrate::propose_local_arrows", "PlasticSubstrate::decay_cell",
    "PlasticSubstrate::require_cell", "PlasticSubstrate::spike_order",
    "PlasticSubstrate::resident_bytes", "pressure_arrow",
}
BANNED = {
    "episode", "history", "query", "role", "event", "cause", "credit", "start",
    "finish", "answer", "correct", "wrong", "composite", "level", "fixture",
    "seed", "reset", "cleanup", "handoff", "mechanism", "evaluator",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory(text: str) -> tuple[set[str], set[str]]:
    types: set[str] = set()
    functions: set[str] = set()
    owner: str | None = None
    depth = 0
    for line in text.splitlines():
        if owner is None:
            match = IMPL_RE.match(line)
            if match and "{" in line:
                owner = match.group(1)
                depth = line.count("{") - line.count("}")
                continue
        type_match = TYPE_RE.match(line)
        if type_match and owner is None:
            types.add(type_match.group(1))
        function_match = FUNCTION_RE.match(line)
        if function_match:
            functions.add(f"{owner}::{function_match.group(1)}" if owner else function_match.group(1))
        if owner is not None:
            depth += line.count("{") - line.count("}")
            if depth == 0:
                owner = None
    return types, functions


def main() -> int:
    source = SOURCE.read_text()
    authority = AUTHORITY.read_text()
    review = REVIEW.read_text()
    types, functions = inventory(source)
    hashes = {path: sha(ROOT / path) for path in EXPECTED}
    hash_exact = {path: hashes[path] == digest for path, digest in EXPECTED.items()}
    taxonomy = list(csv.DictReader((ROOT / "results/pxr0_taxonomy_v2_rerun/pxc_seam_taxonomy_summary_v2.csv").open()))
    review_markers = {
        "a_external_boundary_law": "legitimate physical boundary law; not residual orchestration" in review,
        "b_region_scope": "causally inert except for outward/inter-region" in review,
        "c_observer_status": "both are causally inert observer surfaces" in review,
        "d_cell_fields": "`live` is causal; `generation` is read but currently immutable" in review,
        "e_pressure_phase": "pressure phase is intrinsic retained substrate state" in review,
        "inventory_types": "### Types/state/results: 13" in review,
        "inventory_functions": "### Functions/methods: 15" in review,
        "positive_verdict": "No genuine runtime defect or new-law ambiguity is present" in review,
    }
    banned_hits = sorted({word for word in BANNED if re.search(rf"\b{re.escape(word)}\b", source, re.IGNORECASE)})
    region_lines = [number for number, line in enumerate(source.splitlines(), 1) if "region" in line]
    result = {
        "hashes": hashes,
        "hashes_exact": hash_exact,
        "source_lines": len(source.splitlines()),
        "types": sorted(types),
        "functions_methods": sorted(functions),
        "inventory_exact": types == EXPECTED_TYPES and functions == EXPECTED_FUNCTIONS,
        "external_gate_exact": all(token in source for token in [
            "let external_arrival = spike.arrow.is_none();",
            "if external_arrival {",
            "self.propose_local_arrows(source, &mut work);",
        ]),
        "external_gate_retained_authority": all(token in authority for token in [
            "let external_arrival = spike.arrow.is_none();",
            "if external_arrival {",
            "self.propose_local_arrows(source, &mut work);",
        ]),
        "region_lines": region_lines,
        "region_surface_exact": region_lines == [23, 58, 100, 101, 156, 293, 298, 299],
        "work_not_branch_token": not re.search(r"(?:if|while|match)\s+[^\n{]*\bwork\b", source),
        "resident_terminal_observer": "resident_bytes: self.resident_bytes()," in source,
        "cell_resistance_scaffold": source.count("resistance: spec.resistance,") == 2
        and not re.search(r"(?:cell|target|self\.cells\[[^]]+\])\.resistance", source),
        "cell_generation_immutable": source.count("generation: 1,") == 2 and "target.generation =" not in source and "cell.generation =" not in source,
        "cell_live_construction_and_reads": "live: spec.resistance > 0," in source and "if !target.live" in source and "&& cell.live" in source,
        "pressure_phase_exact": all(token in source for token in [
            "const ORDINARY_PRESSURE_PERIOD: i64 = 10;",
            "pressure_tick: i64,",
            "tick.saturating_sub(self.pressure_tick) / ORDINARY_PRESSURE_PERIOD",
            "pressure_steps.saturating_mul(ORDINARY_PRESSURE_PERIOD)",
        ]),
        "review_markers": review_markers,
        "banned_hits": banned_hits,
        "hidden_surfaces": [token for token in ["mod ", "include!", "macro_rules!", "#[cfg", "#[test]", "unsafe {"] if token in source],
        "taxonomy_zero": bool(taxonomy) and all(int(row["count"]) == 0 for row in taxonomy),
        "development_result_exact": all(token in (ROOT / "results/pxr0_successor_readiness_v2.md").read_text() for token in [
            "rows: `16/16`", "row clauses: `384/384`", "phase controls: `12/12`",
            "phase-control clauses: `72/72`", "global clauses: `10/10`",
            "total clauses: `466/466`", "maximum work: `15039`",
            "maximum resident bytes: `6000`", "natural quiescence: `true`",
            "exact replay: `true`", "PXR0 authority: `false`; PX-C authority: `false`",
        ]),
    }
    result["gate_pass"] = (
        all(hash_exact.values()) and result["source_lines"] == 474 and result["inventory_exact"]
        and result["external_gate_exact"] and result["external_gate_retained_authority"]
        and result["region_surface_exact"] and result["work_not_branch_token"]
        and result["resident_terminal_observer"] and result["cell_resistance_scaffold"]
        and result["cell_generation_immutable"] and result["cell_live_construction_and_reads"]
        and result["pressure_phase_exact"] and all(review_markers.values())
        and not banned_hits and not result["hidden_surfaces"] and result["taxonomy_zero"]
        and result["development_result_exact"]
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    report = [
        "# PXR0 v2 acceptance audit v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- kernel: `{hashes['crates/pxr0-physical-runtime/src/lib.rs']}`; exact: `{hash_exact['crates/pxr0-physical-runtime/src/lib.rs']}`;",
        f"- inventory: `{len(types)}/13` types + `{len(functions)}/15` functions/methods; exact: `{result['inventory_exact']}`;",
        f"- external boundary gate copied from retained authority: `{result['external_gate_retained_authority']}`;",
        f"- region source lines: `{region_lines}`; boundary-only exact: `{result['region_surface_exact']}`;",
        f"- observer counters non-branching / resident bytes terminal: `{result['work_not_branch_token']}/{result['resident_terminal_observer']}`;",
        f"- CELL resistance scaffold / generation immutable / live causal patterns: `{result['cell_resistance_scaffold']}/{result['cell_generation_immutable']}/{result['cell_live_construction_and_reads']}`;",
        f"- pressure phase exact: `{result['pressure_phase_exact']}`;",
        f"- review questions and inventories complete: `{all(review_markers.values())}`;",
        f"- banned/hidden surfaces: `{len(banned_hits)}/{len(result['hidden_surfaces'])}`; taxonomy zero: `{result['taxonomy_zero']}`;",
        f"- frozen development result exact: `{result['development_result_exact']}`.", "",
        "`PXR0_V2_ACCEPTED_PARENT_V1`" if result["gate_pass"] else "`PXR0_V2_ACCEPTANCE_NEGATIVE_V1`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(report) + "\n")
    print(json.dumps(result, sort_keys=True))
    return 0 if result["gate_pass"] else 1


if __name__ == "__main__":
    sys.exit(main())
