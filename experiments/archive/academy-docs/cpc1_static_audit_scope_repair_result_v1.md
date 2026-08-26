# CPC1 static-audit scope repair result v1

Status: positive audit-only repair.

The v1 audit failure was a false positive in its own expression:

```text
participation.*pressure
```

That expression matched two independent evaluator CSV columns on one string
literal. It did not identify a candidate-law branch.

Audit v2 retained all artifact counts, report predicates, checksums, and
forbidden semantic categories. It narrowed the scan to physical candidate
sources and anchored conditional matching to Rust `if` statements.

Fresh E2B sandbox `i332g83jvpdkkgceihcn0` ran only audit v2 and reported:

```text
curve.csv: OK
controls.csv: OK
report.md: OK
CPC1_STATIC_AUDIT_V2_OK physical_cases=620 mechanics_rows=1240
```

No Rust source, evaluator, world, candidate constant, or result artifact
changed. No compilation or physical matrix ran during the repair.
