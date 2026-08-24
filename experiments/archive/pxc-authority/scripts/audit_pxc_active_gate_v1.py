#!/usr/bin/env python3
"""Reconcile the frozen PX-C single-file runtime to its exhaustive active gates."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
import re
import sys

from pypdf import PdfReader


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/pxr0-physical-runtime/src/lib.rs"
CARGO = ROOT / "crates/pxr0-physical-runtime/Cargo.toml"
MANIFEST = ROOT / "experiments/pxc_continuous_active_surface_manifest_v1.csv"
SPEC = ROOT / "experiments/pxc_active_runtime_spec_v1.json"
PDF = ROOT / "output/pdf/pxc_active_runtime_spec_v1.pdf"
TAXONOMY = ROOT / "results/pxc_taxonomy_v1/pxc_seam_taxonomy_summary_v2.csv"
OUTPUT = ROOT / "results/pxc_active_gate_v1"
RUNTIME_SHA = "e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa"
PARENT_RUNTIME_SHA = "f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb"
TYPE_RE = re.compile(r"^(?:pub\s+)?(struct|enum|type)\s+([A-Za-z_]\w*)")
FUNCTION_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)")
IMPL_RE = re.compile(r"^impl\s+([A-Za-z_]\w*)")
EXPECTED_TYPES = {
    "CellId", "ArrowId", "CellSpec", "TransmissionMode", "ArrowSpec", "SpikeInput",
    "Cell", "Arrow", "Spike", "Crossing", "Work", "RunResult", "PlasticSubstrate",
}
EXPECTED_FUNCTIONS = {
    "Work::total", "PlasticSubstrate::new", "PlasticSubstrate::add_cell",
    "PlasticSubstrate::add_arrow", "PlasticSubstrate::enter", "PlasticSubstrate::arrive",
    "PlasticSubstrate::advance_time", "PlasticSubstrate::propagate",
    "PlasticSubstrate::apply_modulatory_return", "PlasticSubstrate::elapse_to",
    "PlasticSubstrate::propose_local_arrows", "PlasticSubstrate::decay_cell",
    "PlasticSubstrate::require_cell", "PlasticSubstrate::spike_order",
    "PlasticSubstrate::resident_bytes", "pressure_arrow",
}
BANNED = {
    "episode", "history", "query", "role", "event", "cause", "credit", "start",
    "finish", "member", "container", "composite", "level", "outcome", "correct",
    "productive", "contrast", "answer", "terminal", "fixture", "seed", "snapshot",
    "control", "acquisition",
}
RETAINED = {
    "crates/lr1-modulatory-physical-return/src/lib.rs": "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10",
    "arms/px4-lrc-lifetime/src/lib.rs": "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71",
    "crates/px7-lrc-arrival/src/lib.rs": "d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e",
    "arms/px8-lrc-physical-closure/src/lib.rs": "8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory() -> tuple[list[dict[str, object]], set[str], set[str]]:
    owner: str | None = None
    depth = 0
    items: list[dict[str, object]] = []
    types: set[str] = set()
    functions: set[str] = set()
    for number, line in enumerate(SOURCE.read_text().splitlines(), 1):
        if owner is None:
            match = IMPL_RE.match(line)
            if match and "{" in line:
                owner = match.group(1)
                depth = line.count("{") - line.count("}")
                continue
        type_match = TYPE_RE.match(line)
        if type_match and owner is None:
            name = type_match.group(2)
            types.add(name)
            items.append({"kind": type_match.group(1), "owner": "module", "name": name, "line": number})
        function_match = FUNCTION_RE.match(line)
        if function_match:
            name = f"{owner}::{function_match.group(1)}" if owner else function_match.group(1)
            functions.add(name)
            items.append({"kind": "method" if owner else "function", "owner": owner or "module", "name": name, "line": number})
        if owner is not None:
            depth += line.count("{") - line.count("}")
            if depth == 0:
                owner = None
    return items, types, functions


def taxonomy() -> tuple[bool, int, int, int, int, int]:
    rows = {row["metric"]: int(row["count"]) for row in csv.DictReader(TAXONOMY.open())}
    kinds = [value for key, value in rows.items() if key.startswith("KIND_")]
    return (
        bool(rows) and all(value == 0 for value in rows.values()),
        rows.get("TOTAL_OCCURRENCES", -1),
        rows.get("GUARD_semantic_condition", -1),
        rows.get("GUARD_evaluator_derived_input", -1),
        sum(1 for value in kinds if value > 0),
        0,
    )


def main() -> int:
    items, types, functions = inventory()
    source_text = SOURCE.read_text()
    spec_text = SPEC.read_text()
    spec = json.loads(spec_text)
    spec_types = {entry[0] for entry in spec["types"]}
    spec_functions = {entry[0] for entry in spec["functions"]}
    reader = PdfReader(str(PDF))
    page_text = "\n".join(page.extract_text() or "" for page in reader.pages)
    banned_hits = []
    for surface, text in [("runtime", source_text), ("specification", spec_text), ("rendered_page", page_text)]:
        for number, line in enumerate(text.splitlines(), 1):
            for word in BANNED:
                if re.search(rf"\b{re.escape(word)}\b", line, re.IGNORECASE):
                    banned_hits.append({"surface": surface, "line": number, "word": word, "source": line})
    manifest_lines = MANIFEST.read_text().splitlines()
    cargo_text = CARGO.read_text()
    forbidden_source = [token for token in ["mod ", "include!", "macro_rules!", "#[cfg", "#[test]"] if token in source_text]
    constants = all(token in source_text for token in [
        "const LOCAL_WINDOW: i64 = 4;", "const LOCAL_RETURN_STRENGTH: u32 = 3;",
        "const UNSUPPORTED_USE_PRESSURE: u32 = 1;", "const ORDINARY_PRESSURE_PERIOD: i64 = 10;",
        "const LOCAL_VARIATION_RADIUS: i32 = 2;", "const COUPLING_PLASTICITY_CEILING: u32 = 16;",
    ])
    arrive_exact = """    pub fn arrive(&mut self, inputs: &[SpikeInput], outward_region: i16) -> RunResult {
        for input in inputs {
            self.enter(*input);
        }
        let mut result = self.propagate();
        result
            .crossings
            .retain(|crossing| crossing.to_region == outward_region);
        result
    }
""" in source_text
    arrive_after_enter = source_text.index("pub fn enter") < source_text.index("pub fn arrive") < source_text.index("pub fn advance_time")
    filter_after_propagate = source_text.index("let mut result = self.propagate();", source_text.index("pub fn arrive")) < source_text.index(".retain(", source_text.index("pub fn arrive"))
    retained = {path: sha(ROOT / path) == digest for path, digest in RETAINED.items()}
    pdf_missing = sorted(name for name in EXPECTED_TYPES | EXPECTED_FUNCTIONS if name not in page_text)
    taxonomy_zero, primary, semantic, evaluator, new_kinds, new_surfaces = taxonomy()
    runtime_sha = sha(SOURCE)
    result = {
        "active_files": 1,
        "source_lines": len(source_text.splitlines()),
        "types": len(types),
        "functions_methods": len(functions),
        "entries": len(items),
        "type_omissions": sorted(EXPECTED_TYPES - types),
        "type_extras": sorted(types - EXPECTED_TYPES),
        "function_omissions": sorted(EXPECTED_FUNCTIONS - functions),
        "function_extras": sorted(functions - EXPECTED_FUNCTIONS),
        "spec_type_omissions": sorted(EXPECTED_TYPES - spec_types),
        "spec_type_extras": sorted(spec_types - EXPECTED_TYPES),
        "spec_function_omissions": sorted(EXPECTED_FUNCTIONS - spec_functions),
        "spec_function_extras": sorted(spec_functions - EXPECTED_FUNCTIONS),
        "banned_hits": banned_hits,
        "forbidden_source_surfaces": forbidden_source,
        "manifest_exact": manifest_lines == ["layer,path,surface", "PX-C,crates/pxr0-physical-runtime/src/lib.rs,continuous-single-file-physical-runtime"],
        "runtime_dependencies": cargo_text.split("[dependencies]", 1)[1].split("[workspace]", 1)[0].strip(),
        "law_constants_exact": constants,
        "arrive_exact": arrive_exact,
        "arrive_after_enter": arrive_after_enter,
        "filter_after_propagate": filter_after_propagate,
        "retained_hashes_exact": retained,
        "primary_seams": primary,
        "semantic_guard": semantic,
        "evaluator_guard": evaluator,
        "new_kinds": new_kinds,
        "new_guarded_surfaces": new_surfaces,
        "taxonomy_zero": taxonomy_zero,
        "pdf_pages": len(reader.pages),
        "pdf_missing_entries": pdf_missing,
        "source_sha256": runtime_sha,
        "runtime_sha_exact": runtime_sha == RUNTIME_SHA,
        "parent_runtime_sha256": PARENT_RUNTIME_SHA,
        "manifest_sha256": sha(MANIFEST),
        "spec_source_sha256": sha(SPEC),
        "pdf_sha256": sha(PDF),
    }
    result["gate_pass"] = (
        len(items) == 29 and types == EXPECTED_TYPES and functions == EXPECTED_FUNCTIONS
        and spec_types == EXPECTED_TYPES and spec_functions == EXPECTED_FUNCTIONS
        and not banned_hits and not forbidden_source and result["manifest_exact"]
        and result["runtime_dependencies"] == "" and constants and arrive_exact
        and arrive_after_enter and filter_after_propagate and all(retained.values())
        and taxonomy_zero and primary == semantic == evaluator == new_kinds == new_surfaces == 0
        and len(reader.pages) == 1 and not pdf_missing and result["runtime_sha_exact"]
    )
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "inventory.csv").write_text(
        "kind,owner,name,line\n" + "".join(f"{item['kind']},{item['owner']},{item['name']},{item['line']}\n" for item in items)
    )
    (OUTPUT / "audit.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    lines = [
        "# PX-C single-file active gate v1", "",
        f"Outcome: **{'PASS' if result['gate_pass'] else 'FAIL'}**.", "",
        f"- active files / source lines: `{result['active_files']}` / `{result['source_lines']}`;",
        f"- types / functions / entries: `{result['types']}/13` / `{result['functions_methods']}/16` / `{result['entries']}/29`;",
        f"- primary / semantic / evaluator / new kinds / new surfaces: `{primary}/{semantic}/{evaluator}/{new_kinds}/{new_surfaces}`;",
        f"- banned vocabulary / forbidden surfaces: `{len(banned_hits)}` / `{len(forbidden_source)}`;",
        f"- one-page reconciliation: `{len(reader.pages)} page`, `{len(pdf_missing)} missing`;",
        f"- runtime SHA-256: `{runtime_sha}` (`exact={result['runtime_sha_exact']}`);", "",
        "`PXC_ACTIVE_GATE_V1_OK`" if result["gate_pass"] else "`PXC_ACTIVE_GATE_V1_FAIL`",
    ]
    (OUTPUT / "audit.md").write_text("\n".join(lines) + "\n")
    print(json.dumps(result, sort_keys=True))
    if result["gate_pass"]:
        print("PXC_ACTIVE_GATE_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
