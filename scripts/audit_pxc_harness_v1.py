#!/usr/bin/env python3
"""Freeze PX-C matrix geometry, interface use, publication, and namespace separation."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "arms/pxc-continuous-organism/src/main.rs"
CARGO = ROOT / "arms/pxc-continuous-organism/Cargo.toml"
PROTOCOL = ROOT / "experiments/pxc_continuous_organism_protocol_v1.md"
OUTPUT = ROOT / "results/pxc_harness_audit_v1"
RUNTIME = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
RUNTIME_SHA = "e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa"
EVALUATOR_SHA = "55f6ad153f58b803d587814ed554521689b2586da4cac60805f600c73f06fb6d"
DEVELOPMENT_ORIGINS = [0, 130, 260, 390]
AUTHORITY_ORIGINS = [520, 650, 780, 910]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def section(text: str, left: str, right: str) -> str:
    start = text.index(left)
    end = text.index(right, start)
    return text[start:end]


def parse_cases(text: str) -> list[tuple[int, bool, bool, int]]:
    return [
        (int(root.replace("_", "")), reverse == "true", reflect == "true", int(origin.replace("_", "")))
        for root, reverse, reflect, origin in re.findall(
            r"Case::new\((\d[\d_]*)\s*,\s*(false|true)\s*,\s*(false|true)\s*,\s*(-?\d[\d_]*)\)", text
        )
    ]


def quadrants(rows: list[tuple[int, bool, bool, int]], origins: list[int]) -> bool:
    expected = [
        (reverse, reflect, origin)
        for reverse, reflect in [(False, False), (True, False), (False, True), (True, True)]
        for origin in origins
    ]
    return [(reverse, reflect, origin) for _, reverse, reflect, origin in rows] == expected


def main() -> int:
    text = SOURCE.read_text()
    development = parse_cases(section(text, "const DEVELOPMENT_CASES:", "const AUTHORITY_CASES:"))
    authority = parse_cases(section(text, "const AUTHORITY_CASES:", "#[derive(Clone, Copy, Debug"))
    run_body = section(text, "fn run(case: Case)", "fn row(")
    main_body = section(text, "fn main()", "fn replay(")
    dev_roots = [row[0] for row in development]
    auth_roots = [row[0] for row in authority]
    dev_origins = [row[3] for row in development]
    auth_origins = [row[3] for row in authority]
    constructor = run_body.index("PlasticSubstrate::new()")
    clock = run_body.index("space.advance_time(case.origin)")
    topology = run_body.index("build_cascade(")
    direct_enter = len(re.findall(r"\.enter\s*\(", text))
    direct_propagate = len(re.findall(r"\.propagate\s*\(", text))
    result_paths = [
        ROOT / "results/pxc_continuous_development_v1.csv",
        ROOT / "results/pxc_continuous_development_v1.md",
        ROOT / "results/pxc_continuous_authority_v1.csv",
        ROOT / "results/pxc_continuous_authority_v1.md",
    ]
    result = {
        "development_roots_exact": dev_roots == list(range(3_100_001, 3_100_017)),
        "authority_roots_exact": auth_roots == list(range(3_200_001, 3_200_017)),
        "namespaces_disjoint": not set(dev_roots) & set(auth_roots),
        "development_origins": dev_origins,
        "authority_origins": auth_origins,
        "origins_exact": dev_origins == DEVELOPMENT_ORIGINS * 4 and auth_origins == AUTHORITY_ORIGINS * 4,
        "origin_moduli": [origin % 10 for origin in dev_origins + auth_origins],
        "balanced_quadrants": quadrants(development, DEVELOPMENT_ORIGINS) and quadrants(authority, AUTHORITY_ORIGINS),
        "substrate_constructor_count": text.count("PlasticSubstrate::new()"),
        "empty_clock_before_topology": constructor < clock < topology,
        "arrive_call_sites": len(re.findall(r"\.arrive\s*\(", text)),
        "advance_time_call_sites": len(re.findall(r"\.advance_time\s*\(", text)),
        "direct_enter_calls": direct_enter,
        "direct_propagate_calls": direct_propagate,
        "no_reconstruction_or_clone": text.count("PlasticSubstrate::new()") == 1 and ".clone()" not in run_body,
        "anonymous_boundary_input": "SpikeInput" in text and "origin_physical:" in text,
        "outward_crossing_only": "crossing.to_region == OUTWARD_REGION" in text,
        "timing_fields_serialized": all(token in text for token in ["construction_tick", "pressure_origin", "first_arrival_tick", "last_arrival_tick"]),
        "work_memory_quiescence_serialized": all(token in text for token in ["max_work", "max_bytes", "all_quiet", "outward_only", "replay"]),
        "row_clause_count": 32,
        "global_clause_count": 12,
        "unconditional_publication_before_assertions": main_body.index("publish(mode.csv()") < main_body.index("assert!(") and main_body.index("publish(mode.report()") < main_body.index("assert!("),
        "mode_markers_frozen": all(token in text for token in ["PXC_CONTINUOUS_DEVELOPMENT_EVIDENCE_SPENT_V1", "PXC_CONTINUOUS_AUTHORITY_EVIDENCE_SPENT_V1"]),
        "outputs_absent_at_freeze": not any(path.exists() for path in result_paths),
        "runtime_sha256": sha(RUNTIME),
        "runtime_sha_exact": sha(RUNTIME) == RUNTIME_SHA,
        "evaluator_sha256": sha(SOURCE),
        "evaluator_sha_exact": sha(SOURCE) == EVALUATOR_SHA,
        "protocol_sha256": sha(PROTOCOL),
        "dependency_exact": CARGO.read_text().split("[dependencies]", 1)[1].split("[workspace]", 1)[0].strip() == 'pxr0-physical-runtime = { path = "../../crates/pxr0-physical-runtime" }',
    }
    result["gate_pass"] = (
        result["development_roots_exact"] and result["authority_roots_exact"]
        and result["namespaces_disjoint"] and result["origins_exact"]
        and result["origin_moduli"] == [0] * 32 and result["balanced_quadrants"]
        and result["substrate_constructor_count"] == 1 and result["empty_clock_before_topology"]
        and result["arrive_call_sites"] == 9 and result["advance_time_call_sites"] == 3
        and direct_enter == direct_propagate == 0 and result["no_reconstruction_or_clone"]
        and result["anonymous_boundary_input"] and result["outward_crossing_only"]
        and result["timing_fields_serialized"] and result["work_memory_quiescence_serialized"]
        and result["unconditional_publication_before_assertions"] and result["mode_markers_frozen"]
        and result["outputs_absent_at_freeze"] and result["runtime_sha_exact"]
        and result["evaluator_sha_exact"] and result["dependency_exact"]
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    lines = [
        "# PX-C harness freeze audit v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- development roots: `{dev_roots[0] if dev_roots else 'missing'}..{dev_roots[-1] if dev_roots else 'missing'}`; authority roots: `{auth_roots[0] if auth_roots else 'missing'}..{auth_roots[-1] if auth_roots else 'missing'}`;",
        f"- development origins: `{dev_origins}`; authority origins: `{auth_origins}`; all moduli zero: `{result['origin_moduli'] == [0] * 32}`;",
        f"- constructor / arrive / advance-time call sites: `{result['substrate_constructor_count']}/{result['arrive_call_sites']}/{result['advance_time_call_sites']}`;",
        f"- direct enter / propagate: `{direct_enter}/{direct_propagate}`;",
        f"- publication before assertions: `{result['unconditional_publication_before_assertions']}`;",
        f"- runtime / evaluator hashes exact: `{result['runtime_sha_exact']}/{result['evaluator_sha_exact']}`.", "",
        "`PXC_HARNESS_AUDIT_V1_OK`" if result["gate_pass"] else "`PXC_HARNESS_AUDIT_V1_FAIL`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(lines) + "\n")
    print(json.dumps(result, sort_keys=True))
    if result["gate_pass"]:
        print("PXC_HARNESS_AUDIT_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
