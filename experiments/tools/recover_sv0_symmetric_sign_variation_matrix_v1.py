#!/usr/bin/env python3
"""Recover two unquoted diagnostic list fields from immutable SV0 raw CSV."""

from __future__ import annotations

import csv
import hashlib
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT / "experiments/results/sv0_symmetric_sign_variation_v1/matrix.csv"
EXPECTED_RAW = "b1e147f2c2f8f29b43a705469e75d72deadf4008b299f08344db40ca5ef4310a"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def take_list(text: str) -> tuple[str, str]:
    if not text.startswith("["):
        raise ValueError("diagnostic list must begin with '['")
    end = text.find("]")
    if end < 0:
        raise ValueError("diagnostic list has no closing ']'")
    value = text[: end + 1]
    remainder = text[end + 1 :]
    if not remainder.startswith(","):
        raise ValueError("diagnostic list must be followed by comma")
    return value, remainder[1:]


def recover_line(line: str) -> list[str]:
    prefix = line.rstrip("\n").split(",", 9)
    if len(prefix) != 10:
        raise ValueError("SV0 raw row lacks fixed prefix")
    fixed, remainder = prefix[:9], prefix[9]
    initial_crossings, remainder = take_list(remainder)
    death_ages, remainder = take_list(remainder)
    suffix = remainder.split(",")
    row = [*fixed, initial_crossings, death_ages, *suffix]
    if len(row) != 27:
        raise ValueError(f"recovered row has {len(row)} fields, expected 27")
    return row


def main() -> None:
    if digest(RAW) != EXPECTED_RAW:
        raise SystemExit("immutable SV0 raw matrix hash mismatch")
    output = Path(sys.argv[1]) if len(sys.argv) > 1 else RAW.with_name("matrix_recovered.csv")
    audit = Path(sys.argv[2]) if len(sys.argv) > 2 else RAW.with_name("recovery_audit.txt")
    lines = RAW.read_text().splitlines(keepends=True)
    header = lines[0].rstrip("\n").split(",")
    if len(header) != 27:
        raise SystemExit("SV0 header does not have 27 fields")
    rows = [recover_line(line) for line in lines[1:]]
    if len(rows) != 144:
        raise SystemExit(f"SV0 matrix has {len(rows)} rows, expected 144")
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", newline="") as handle:
        writer = csv.writer(handle, lineterminator="\n")
        writer.writerow(header)
        writer.writerows(rows)
    recovered_hash = digest(output)
    audit.write_text(
        "raw_sha256=" + EXPECTED_RAW + "\n"
        "recovered_rows=144\n"
        "recovered_fields_per_row=27\n"
        "physical_values_changed=false\n"
        "organism_rerun=false\n"
        "recovered_sha256=" + recovered_hash + "\n"
        "SV0_MATRIX_RECOVERY_V1_PASS\n"
    )
    print(audit.read_text(), end="")


if __name__ == "__main__":
    main()
