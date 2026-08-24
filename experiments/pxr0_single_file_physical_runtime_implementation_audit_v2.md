# PXR0 single-file physical-runtime development v2 implementation audit

Status: **V2 HARNESS FROZEN; DEVELOPMENT EVIDENCE UNSPENT; NO AUTHORITY CLAIM**.

This tooling-only candidate descends from frozen v1 negative commit
`381aef6a69acdf008180eeab9b6fcecef70f4e1e` / tag
`pxr0-single-file-physical-runtime-development-negative-v1` through frozen v2
protocol commit `c3f64032e68a4bf75dd51086adad60a2f79e6bda` / tag
`pxr0-single-file-physical-runtime-protocol-v2`. It does not spend PXR0
successor authority or PX-C authority.

## Immutable organism boundary

The canonical runtime remains exactly
`crates/pxr0-physical-runtime/src/lib.rs`, 474 lines, 13 types and 15
functions/methods, with SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
No runtime, retained PX0-PX8+LR-C source, active-surface manifest, exhaustive
inventory, specification source, or rendered one-page PDF changed in v2.

The only v2 implementation surfaces are non-organism tooling:

| path | responsibility | SHA-256 |
|---|---|---|
| `arms/pxr0-successor-readiness-v2/Cargo.toml` | isolated evaluator manifest | `3f7f1b8e6aa3e091186ebe820b328d4de158b97c9a943a4baf31c6a88617b319` |
| `arms/pxr0-successor-readiness-v2/src/main.rs` | frozen v2 development matrix | `0558795819955a2186a06408108c2e0d9da89f0ec222b9bbcfc3622bb5bc7f91` |
| `scripts/audit_pxr0_static_gate_v2.py` | immutable runtime/inventory/page/taxonomy gate | `0fff2d19ae0a86e5397585071d035e669d06a3a2e3661cf4017f29c001569ecd` |
| `scripts/audit_pxr0_v2_harness_v1.py` | schedule geometry and construction-order gate | `918bb9c29e7ed1790762b90bca42b17ee4f5c14bac0838f44b178c6c6f607ead` |

## Empty-clock construction and timing reconciliation

There are exactly three `PlasticSubstrate::new()` construction paths in the
evaluator: `RecursiveBody::new`, `compact`, and `PairBody::new`. Each path
immediately calls `advance_time(origin)` while the substrate is empty and
before its first `add_cell` or `add_arrow`. All later arrivals retain their v1
relative timing. No topology is exposed to pre-experience pressure.

The 16 invariance rows use fresh roots `1_175_001..1_175_016` and origins
`0,130,260,390`, once in each reverse/reflection quadrant. For every row:

```text
construction_tick = pressure_origin = first_arrival_tick = origin
construction_minus_pressure = first_arrival_minus_construction = origin % 10 = 0
```

The 12 separate phase controls use fresh roots `1_176_001..1_176_012` and
origins `3,6,9,133,136,139,263,266,269,393,396,399`. Each records construction
and first arrival at `origin`, retained pressure origin at the preceding
multiple of ten, and a construction-minus-pressure delta of `3`, `6`, or `9`.
Each control also serializes its matching phase-zero root and whether its full
registered functional observation differs. Difference is observational only;
it is not a success predicate.

## Frozen interpretation and serialization

The 24 v1 functional clauses remain unchanged for every invariance row. Each
phase control has only the six preregistered replay, quiescence, work, memory,
negative-world, and stale/reproposal safety clauses. Phase-control observations
cannot fail or weaken an invariance clause merely because their phase differs.

The evaluator writes the 16-row CSV, 12-control CSV, and complete Markdown
report through staging paths before aggregate assertions. Every row records
root, layout, origin, modulus, construction tick, pressure origin, first
arrival tick, both timing deltas, observations, bounds, replay, clauses, and
pass state. The complete success sentinel is exactly 466/466 clauses.

## Pre-evidence formatting record and remaining spend

Fresh formatting-only E2B sandbox `i0j7cgatdoizuk9jo3mm4`, using unique state
file `/Users/satya/.cache/truelearner/pxr0-v2-format-20260824-a.json`, accepted
`cargo fmt --check` for both the unchanged runtime and v2 evaluator. It ran no
project binary, audit, test, Clippy, or matrix.

One new targeted-validation E2B worker may now format-check and Clippy the two
crates, rerun the 101-to-28 extraction map, taxonomy-v2 zero ceilings, static
one-file/inventory/dependency/vocabulary/page reconciliation, and v2 harness
geometry audit. It must not run the matrix. If positive and frozen, exactly one
second fresh E2B worker may execute the complete v2 development matrix once.
No rescue run, physics edit, schedule tuning, successor-authority claim, or
PX-C evidence is authorized.
