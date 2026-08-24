# PXR0 single-file physical-runtime development protocol v2

Status: **PREREGISTERED DEVELOPMENT V2; EVIDENCE UNSPENT; NO AUTHORITY CLAIM**.

Parent result is exact commit
`381aef6a69acdf008180eeab9b6fcecef70f4e1e` / tag
`pxr0-single-file-physical-runtime-development-negative-v1`. The canonical
runtime is immutable at SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
PXR0 v2 may change evaluator/audit tooling only. It may not change the runtime,
physical law, PXR0 authority, or PX-C authority.

## Exact v1 diagnosis and v2 repair boundary

V1 translated arrivals/topology construction to origins `0,137,274,411` while
ordinary pressure remained on absolute ten-tick epochs. Origins 137 and 274
therefore placed pressure at different relative times during eligibility. Those
were physically different phases, not equivalent translations.

Every v2 world first creates an empty `PlasticSubstrate`, calls existing
`advance_time(origin)` before adding a cell or arrow, then constructs identical
topology and applies identical relative arrival timings. No topology may exist
while the empty clock is moved. For an invariance row, `origin % 10 == 0`, so
the retained pressure epoch lands exactly at construction tick and the first
arrival occurs at that same origin.

The four invariance origins are exactly `0,130,260,390`. Each appears four
times across roots `1_175_001..1_175_016`; reverse/reflected layout quadrants
each appear four times. Every row records:

```text
construction_tick = origin
pressure_origin = origin
first_arrival_tick = origin
construction_minus_pressure = 0
first_arrival_minus_construction = 0
origin_modulus = 0
```

The 24 functional clauses are byte-for-byte the v1 protocol clauses. No
predicate, bound, physical world, or counterexample is weakened. Each complete
row is independently replayed and serialized unconditionally before aggregate
assertions.

## Separately interpreted phase controls

Twelve fresh controls use roots `1_176_001..1_176_012` and origins:

```text
3,6,9,133,136,139,263,266,269,393,396,399
```

Each control also advances an empty substrate before construction. Its pressure
origin is `origin - origin % 10`; construction and first arrival ticks equal
`origin`; construction-minus-pressure is therefore 3, 6, or 9; and
first-arrival-minus-construction is zero. These controls intentionally change
relative pressure phase. Their functional observations may lawfully differ and
are never included in invariance clauses.

Each control serializes six independent safety clauses:

1. byte-exact complete replay;
2. every advance naturally quiescent;
3. maximum per-advance work at most `20000`;
4. maximum resident memory at most `8192` bytes;
5. incomplete, blocked, open, branch, and cycle worlds remain outward-silent;
6. aged structure stays silent and changed experience creates exactly one
   bounded proposal.

The control CSV must include root, origin, modulus, construction tick, pressure
origin, first arrival tick, both deltas, paired updates/impulses, formation
updates, outward observations, bounds, replay, six clauses, and pass state.

## Frozen v2 matrix and gates

The v2 evaluator is non-organism tooling at
`arms/pxr0-successor-readiness-v2/src/main.rs`, depending only on the unchanged
PXR0 runtime. It writes invariance and phase-control CSVs plus one Markdown
report through staging files before aggregate assertions.

Ten global clauses require: exact invariance roots; balanced layout quadrants;
balanced origins; every invariance timing record at modulus/deltas zero; all
invariance namespaces disjoint; exact phase-control roots/origins/moduli and
timing records; immutable runtime hash; one-file/inventory/page/dependency/
leakage gates positive; exactly 16 invariance rows/384 clauses plus 12 controls/
72 clauses; and publication with exact replay everywhere. Success is
`466/466` clauses.

Before conformance, a fresh targeted-validation sandbox must re-run formatting,
Clippy, the 101-to-28 movement audit, one-file/exhaustive inventory,
dependency-direction, banned-vocabulary, taxonomy-v2 zero ceilings, retained
hashes, and existing one-page PDF reconciliation. It may compile the unchanged
kernel and new harness but may not run the matrix.

After that validation is frozen positive, a second fresh sandbox runs the
complete v2 matrix exactly once. No rescue, row filtering, schedule tuning,
physics change, or predicate change is allowed. If any phase-preserving row
fails, freeze negative and stop. If all 466 clauses pass, freeze PXR0
development readiness and stop for joint Rust review. Neither outcome advances
PXR0 authority or PX-C.
