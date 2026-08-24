#!/usr/bin/env python3
"""Statically freeze PXR0 v2 schedule geometry and empty-clock construction."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "arms/pxr0-successor-readiness-v2/src/main.rs"
CARGO = ROOT / "arms/pxr0-successor-readiness-v2/Cargo.toml"
OUTPUT = ROOT / "results/pxr0_v2_harness_audit_v1"
RUNTIME_SHA = "f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb"
INVARIANT_ORIGINS = [0, 130, 260, 390]
CONTROL_ORIGINS = [3, 6, 9, 133, 136, 139, 263, 266, 269, 393, 396, 399]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def body(text: str, start: str, end: str) -> str:
    left = text.index(start)
    right = text.index(end, left)
    return text[left:right]


def positions(section: str, new: str, advance: str, add: str) -> bool:
    return section.count(new) == 1 and section.index(new) < section.index(advance) < section.index(add)


def main() -> int:
    text = SOURCE.read_text()
    cases = body(text, "const CASES:", "const CONTROL_CASES:")
    controls = body(text, "const CONTROL_CASES:", "#[derive(Clone, Copy, Debug")
    case_rows = re.findall(r"Case::new\((\d[\d_]*)\s*,\s*(false|true)\s*,\s*(false|true)\s*,\s*(-?\d[\d_]*)\)", cases)
    control_rows = re.findall(r"Case::new\((\d[\d_]*)\s*,\s*(false|true)\s*,\s*(false|true)\s*,\s*(-?\d[\d_]*)\)", controls)
    case_roots = [int(row[0].replace("_", "")) for row in case_rows]
    case_origins = [int(row[3].replace("_", "")) for row in case_rows]
    control_roots = [int(row[0].replace("_", "")) for row in control_rows]
    control_origins = [int(row[3].replace("_", "")) for row in control_rows]
    recursive = body(text, "fn new(layout: Layout)", "fn learn_twice")
    compact = body(text, "fn compact(case: Case", "struct PairBody")
    pair = body(text, "fn new(case: Case, offset: u64)", "fn participate")
    construction_order = {
        "RecursiveBody::new": positions(recursive, "PlasticSubstrate::new()", "space.advance_time(layout.shift)", "space.add_cell"),
        "compact": positions(compact, "PlasticSubstrate::new()", "space.advance_time(case.shift)", "space.add_cell"),
        "PairBody::new": positions(pair, "PlasticSubstrate::new()", "space.advance_time(case.shift)", "space.add_cell"),
    }
    runtime_path = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
    result = {
        "invariance_roots_exact": case_roots == list(range(1_175_001, 1_175_017)),
        "invariance_origins": case_origins,
        "invariance_origins_exact": case_origins == INVARIANT_ORIGINS * 4,
        "invariance_moduli": [origin % 10 for origin in case_origins],
        "layout_quadrants_exact": [(row[1], row[2]) for row in case_rows] == [("false", "false")] * 4 + [("true", "false")] * 4 + [("false", "true")] * 4 + [("true", "true")] * 4,
        "control_roots_exact": control_roots == list(range(1_176_001, 1_176_013)),
        "control_origins": control_origins,
        "control_origins_exact": control_origins == CONTROL_ORIGINS,
        "control_moduli": [origin % 10 for origin in control_origins],
        "construction_order": construction_order,
        "substrate_constructor_count": text.count("PlasticSubstrate::new()"),
        "empty_advance_count": text.count("space.advance_time(layout.shift)") + text.count("space.advance_time(case.shift)"),
        "timing_columns_present": all(token in text for token in ["construction_tick", "pressure_origin", "first_arrival_tick", "construction_minus_pressure", "first_arrival_minus_construction", "origin_modulus"]),
        "unconditional_publish_before_assertions": text.index("publish(CSV_STAGE") < text.index("assert!(") and text.index("&control_csv(&controls)") < text.index("assert!(") and text.index("publish(MD_STAGE") < text.index("assert!("),
        "phase_effect_serialized": all(token in text for token in ["phase_zero_root", "differs_from_phase_zero", "functional_observation"]),
        "runtime_sha256": sha(runtime_path),
        "runtime_sha_exact": sha(runtime_path) == RUNTIME_SHA,
        "harness_dependencies_exact": "pxr0-physical-runtime = { path = \"../../crates/pxr0-physical-runtime\" }" in CARGO.read_text(),
        "evaluator_sha256": sha(SOURCE),
    }
    result["gate_pass"] = (
        result["invariance_roots_exact"] and result["invariance_origins_exact"]
        and result["invariance_moduli"] == [0] * 16 and result["layout_quadrants_exact"]
        and result["control_roots_exact"] and result["control_origins_exact"]
        and result["control_moduli"] == [3, 6, 9] * 4
        and all(construction_order.values()) and result["substrate_constructor_count"] == 3
        and result["empty_advance_count"] == 3 and result["timing_columns_present"]
        and result["unconditional_publish_before_assertions"] and result["phase_effect_serialized"]
        and result["runtime_sha_exact"]
        and result["harness_dependencies_exact"]
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    report = [
        "# PXR0 v2 harness geometry audit v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- invariance origins: `{','.join(map(str, case_origins))}`; moduli: `{','.join(map(str, result['invariance_moduli']))}`;",
        f"- control origins: `{','.join(map(str, control_origins))}`; moduli: `{','.join(map(str, result['control_moduli']))}`;",
        f"- empty-clock construction order: `{construction_order}`;",
        f"- substrate constructors / empty advances: `{result['substrate_constructor_count']}/3` / `{result['empty_advance_count']}/3`;",
        f"- unconditional publication before assertions: `{result['unconditional_publish_before_assertions']}`;",
        f"- phase-zero comparison serialized: `{result['phase_effect_serialized']}`;",
        f"- immutable runtime SHA-256 exact: `{result['runtime_sha_exact']}`;",
        f"- evaluator SHA-256: `{result['evaluator_sha256']}`.", "",
        "`PXR0_V2_HARNESS_AUDIT_V1_OK`" if result["gate_pass"] else "`PXR0_V2_HARNESS_AUDIT_V1_FAIL`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(report) + "\n")
    print(json.dumps(result, sort_keys=True))
    if result["gate_pass"]:
        print("PXR0_V2_HARNESS_AUDIT_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
