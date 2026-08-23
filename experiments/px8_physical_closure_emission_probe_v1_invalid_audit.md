# PX8 physical closure-emission PROBE v1 invalid-execution audit

Status: **FROZEN INVALID DEVELOPMENT RUN; DOES NOT AUTHORIZE MICRO**.

## Preserved execution

The sole PROBE v1 command emitted one
`PX8_PHYSICAL_CLOSURE_EMISSION_PROBE_EVIDENCE_SPENT` marker and reported
`16/16` rows and `160/160` serialized clauses positive. The raw CSV and report
are preserved unchanged.

| artifact | SHA-256 |
|---|---|
| PROBE v1 CSV | `d33963361b6c2a3f40a9eaa172d296dcdbd8f668afbdd1e26f136e00028e629f` |
| PROBE v1 report | `870691020517c5d8232de24067fa3bd50581fb2c869c3620bb1474bed92ce44b` |
| executed source | `3529ad6ff9a81740a2a1084b973e98a3c9d92a6c58ef952540b29c2c374ae332` |
| protocol | `881f4e3d46ce55aa1d637bfe2cc3cc99fb7e7ca2348fc1256e949fd1e3d36c2b` |

## Invalidating defect

The source selected unrelated-activity occurrence count with
`unrelated_load.max(4)` but serialized `unrelated_load`. Seed 0 therefore
executed four unrelated arrivals while recording load `0`. This violates exact
physical-work serialization and the frozen load schedule. The behavioral
clauses are observationally positive, but the run is invalid development
evidence and cannot authorize MICRO.

No result bit, artifact, physical outcome, or interpretation is changed. The
run is not rerun and its namespace range beginning `0x8_8000_0000` is spent.

## Mechanically unique correction

The defect has one mechanical correction: use and serialize the same exact,
strictly positive unrelated-arrival count. A retry may alter only the load
table and use a fresh namespace. It may not change CELL/ARROW/SPIKE physics,
thresholds, couplings, delays, conditions, clauses, expectations, or any
positive/blocked physical relation.

The invalid execution does not indicate a missing physical edge, new
representation, new substrate law, or scientific ambiguity.
