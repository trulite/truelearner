#!/usr/bin/env python3
import csv
from pathlib import Path
import sys


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


if len(sys.argv) != 3:
    raise SystemExit("usage: compare_ah0_cpc0_parent_v1.py PARENT.csv CANDIDATE.csv")

parent_path = Path(sys.argv[1])
candidate_path = Path(sys.argv[2])
parent = rows(parent_path)
candidate = rows(candidate_path)

if len(parent) != len(candidate):
    raise SystemExit(f"row-count mismatch: {len(parent)} != {len(candidate)}")

raw_differences = 0
for index, (expected, actual) in enumerate(zip(parent, candidate, strict=True), start=1):
    if expected.keys() != actual.keys():
        raise SystemExit(f"header mismatch at row {index}")
    for field in expected:
        if field == "raw_trace_hash":
            raw_differences += int(expected[field] != actual[field])
            continue
        if expected[field] != actual[field]:
            raise SystemExit(
                f"physical mismatch row={index} field={field}: "
                f"{expected[field]} != {actual[field]}"
            )

print(f"rows={len(parent)}")
print(f"raw_order_only_differences={raw_differences}")
print("AH0_CPC0_PARENT_DIFFERENTIAL_NORMALIZED_EXACT_V1")
