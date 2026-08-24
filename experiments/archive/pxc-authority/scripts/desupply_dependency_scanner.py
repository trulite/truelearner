#!/usr/bin/env python3
"""Static path scanner for DS4--DS8 development manifests.

The scanner reports source presence only.  A present symbol is still marked
UNRESOLVED when the manifest requires a runtime probe; this tool never asserts
that two signals are semantically equivalent or physically connected.
"""

from __future__ import annotations

import argparse
import csv
import json
from dataclasses import asdict, dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "experiments" / "desupply_ds4_ds8_dependency_manifest.csv"


@dataclass(frozen=True)
class ScanRow:
    stage: str
    order: int
    kind: str
    requirement: str
    provider_path: str
    required_symbols: list[str]
    syntax: str
    physical: str
    note: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--stage", required=True, choices=[f"DS{n}" for n in range(4, 9)])
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--json", action="store_true", dest="as_json")
    return parser.parse_args()


def scan(manifest: Path, stage: str) -> list[ScanRow]:
    rows: list[ScanRow] = []
    with manifest.open(newline="", encoding="utf-8") as handle:
        for raw in csv.DictReader(handle):
            if raw["stage"] != stage:
                continue
            provider = ROOT / raw["provider_path"]
            symbols = [symbol for symbol in raw["required_symbols"].split("|") if symbol]
            source = provider.read_text(encoding="utf-8") if provider.is_file() else ""
            found = sum(symbol in source for symbol in symbols)
            if not provider.is_file() or found == 0:
                syntax = "ABSENT"
                physical = "ABSENT"
            elif found != len(symbols):
                syntax = "PARTIAL"
                physical = "UNRESOLVED"
            else:
                syntax = "PRESENT"
                physical = "UNRESOLVED" if raw["runtime_probe"] == "required" else "PRESENT"
            rows.append(
                ScanRow(
                    stage=stage,
                    order=int(raw["order"]),
                    kind=raw["kind"],
                    requirement=raw["requirement"],
                    provider_path=raw["provider_path"],
                    required_symbols=symbols,
                    syntax=syntax,
                    physical=physical,
                    note=raw["note"],
                )
            )
    return sorted(rows, key=lambda row: row.order)


def print_text(rows: list[ScanRow]) -> None:
    if not rows:
        raise SystemExit("no dependency rows found for requested stage")
    print(f"{rows[0].stage} static dependency scan")
    for row in rows:
        print(f"{row.order:02d} {row.kind}: syntax={row.syntax} physical={row.physical}")
        print(f"   requires: {row.requirement}")
        print(f"   provider: {row.provider_path}")
        if row.note:
            print(f"   note: {row.note}")
    absent = next((row for row in rows if row.physical == "ABSENT"), None)
    if absent is None:
        print("first absent physical prerequisite: NONE STATICALLY; run the first unresolved probe")
    else:
        print(f"first absent physical prerequisite: {absent.order:02d} {absent.requirement}")


def main() -> None:
    args = parse_args()
    rows = scan(args.manifest, args.stage)
    if args.as_json:
        print(json.dumps([asdict(row) for row in rows], indent=2))
    else:
        print_text(rows)


if __name__ == "__main__":
    main()
