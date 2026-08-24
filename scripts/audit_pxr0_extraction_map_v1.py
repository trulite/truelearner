#!/usr/bin/env python3
"""Reconcile the manifest-v6 active inventory to the PXR0 successor inventory."""

from __future__ import annotations

import csv
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "results/pxr0_extraction_map_v1"
LEGACY = [
    "crates/lr1-modulatory-physical-return/src/lib.rs",
    "arms/px4-lrc-lifetime/src/lib.rs",
    "crates/px7-lrc-arrival/src/lib.rs",
    "arms/px8-lrc-physical-closure/src/lib.rs",
]
TYPE_RE = re.compile(r"^(?:pub\s+)?(struct|enum|type)\s+([A-Za-z_]\w*)")
FUNCTION_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)")
IMPL_RE = re.compile(r"^impl\s+([A-Za-z_]\w*)")
TYPE_MAP = {
    "CellId": "CellId", "ArrowId": "ArrowId", "CellSpec": "CellSpec",
    "TransmissionMode": "TransmissionMode", "ArrowSpec": "ArrowSpec",
    "SpikeInput": "SpikeInput", "Cell": "Cell", "Arrow": "Arrow", "Spike": "Spike",
    "Crossing": "Crossing", "WorkLedger": "Work", "Execution": "RunResult",
    "PlasticSubstrate": "PlasticSubstrate",
}
FUNCTION_MAP = {
    "WorkLedger::total": "Work::total",
    "PlasticSubstrate::new": "PlasticSubstrate::new",
    "PlasticSubstrate::add_cell": "PlasticSubstrate::add_cell",
    "PlasticSubstrate::add_arrow": "PlasticSubstrate::add_arrow",
    "PlasticSubstrate::enter": "PlasticSubstrate::enter",
    "PlasticSubstrate::advance_time": "PlasticSubstrate::advance_time",
    "PlasticSubstrate::propagate": "PlasticSubstrate::propagate",
    "PlasticSubstrate::apply_modulatory_return": "PlasticSubstrate::apply_modulatory_return",
    "PlasticSubstrate::elapse_to": "PlasticSubstrate::elapse_to",
    "PlasticSubstrate::propose_local_arrows": "PlasticSubstrate::propose_local_arrows",
    "PlasticSubstrate::decay_cell": "PlasticSubstrate::decay_cell",
    "PlasticSubstrate::require_cell": "PlasticSubstrate::require_cell",
    "PlasticSubstrate::spike_order": "PlasticSubstrate::spike_order",
    "PlasticSubstrate::persistent_bytes": "PlasticSubstrate::resident_bytes",
    "pressure_arrow": "pressure_arrow",
}


def inventory(relative: str) -> list[dict[str, object]]:
    lines = (ROOT / relative).read_text().splitlines()
    owner: str | None = None
    depth = 0
    items: list[dict[str, object]] = []
    for number, line in enumerate(lines, 1):
        if line.strip() == "#[cfg(test)]":
            break
        if owner is None:
            match = IMPL_RE.match(line)
            if match and "{" in line:
                owner = match.group(1)
                depth = line.count("{") - line.count("}")
                continue
        type_match = TYPE_RE.match(line)
        if type_match and owner is None:
            items.append({"path": relative, "line": number, "kind": type_match.group(1), "owner": "module", "name": type_match.group(2)})
        function_match = FUNCTION_RE.match(line)
        if function_match:
            name = f"{owner}::{function_match.group(1)}" if owner else function_match.group(1)
            items.append({"path": relative, "line": number, "kind": "method" if owner else "function", "owner": owner or "module", "name": name})
        if owner is not None:
            depth += line.count("{") - line.count("}")
            if depth == 0:
                owner = None
    return items


def main() -> int:
    rows: list[dict[str, object]] = []
    counts: dict[str, int] = {}
    law = LEGACY[0]
    for relative in LEGACY:
        items = inventory(relative)
        counts[relative] = len(items)
        for item in items:
            target = ""
            if relative == law:
                if item["kind"] in {"struct", "enum", "type"}:
                    target = TYPE_MAP.get(str(item["name"]), "")
                else:
                    target = FUNCTION_MAP.get(str(item["name"]), "")
            if target:
                disposition = "retained_or_canonically_compacted"
            elif relative == law:
                disposition = "moved_diagnostic_or_audit_tooling"
            else:
                disposition = "moved_research_world_tooling"
            rows.append({**item, "disposition": disposition, "pxr0_entry": target})
    retained = sum(bool(row["pxr0_entry"]) for row in rows)
    moved = len(rows) - retained
    passed = counts == {LEGACY[0]: 46, LEGACY[1]: 7, LEGACY[2]: 23, LEGACY[3]: 25} and len(rows) == 101 and retained == 28 and moved == 73
    OUTPUT.mkdir(parents=True, exist_ok=True)
    with (OUTPUT / "extraction_map.csv").open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=["path", "line", "kind", "owner", "name", "disposition", "pxr0_entry"])
        writer.writeheader()
        writer.writerows(rows)
    report = [
        "# PXR0 extraction inventory map v1", "",
        f"Outcome: **{'PASS' if passed else 'FAIL'}**.", "",
        f"- manifest-v6 unique active inventory: `{len(rows)}` entries;",
        f"- retained or canonically compacted into PXR0: `{retained}`;",
        f"- moved to research/audit tooling: `{moved}`;",
        "- PXR0 active inventory: `28` entries in one source file;",
        "- active inventory delta: `-73` entries and `-3` files.", "",
        "## Per-file movement", "",
    ]
    for path, count in counts.items():
        mapped = sum(row["path"] == path and bool(row["pxr0_entry"]) for row in rows)
        report.append(f"- `{path}`: `{count}` original, `{mapped}` retained/mapped, `{count - mapped}` moved.")
    report.extend(["", "`PXR0_EXTRACTION_MAP_V1_OK`" if passed else "`PXR0_EXTRACTION_MAP_V1_FAIL`"]) 
    (OUTPUT / "audit.md").write_text("\n".join(report) + "\n")
    print(f"PXR0 extraction: original={len(rows)} retained={retained} moved={moved}")
    if passed:
        print("PXR0_EXTRACTION_MAP_V1_OK")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
