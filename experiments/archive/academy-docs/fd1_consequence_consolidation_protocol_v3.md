# FD1 consequence consolidation protocol v3

Status: frozen before v3 evaluator repair or execution.

Parent: `fd1-consequence-consolidation-negative-v2` (`31c65d5`).

## Eligibility

V2 serialized all 280 rows and isolated a single observation-time defect in
C3. Its internal normalized state comparison, replay, Reference/Production
equality, and quiescence all passed. Six other families passed completely.

V3 is eligible as a one-line-class measurement repair. V1 and v2 remain
immutable negatives.

## Sole repair

In C3, construct each serialized late `Point` from the state already captured
at that physical time:

```text
late_after captured at age 9  -> after_consequence Point
late_last  captured at age 48 -> last_live Point
late_dead  captured at age 49 -> death Point
```

Do not call `late.point(...)` after the world has advanced to age 49 for the
earlier labels.

No core byte, physical law, schedule, family, predicate, expected resistance,
expected death age, mechanics comparator, work comparator, or acceptance gate
may change.

## Fresh matrix

Run C0-C6 under roots `4_900_000` and `5_000_000`, phases `0..9`, Reference
and Production, and exact same-mechanics replay. Retain v2's rule that all rows
and gate booleans are written before the final assertion.

## Cumulative FD0 control

Only after all 140 v3 physical cases pass, execute the unchanged FD0 evaluator
once and require exact hashes:

- matrix `5b9cfaf5e6ac93d56b07b7c0346bb95d46c1e33bc539a74897e40e40fb97dc99`;
- report `1450c5bdf8a133aed39688cb57c53586f43dd47ed4cf10243b6b305b17f38384`.

## Decision

Any focused, mechanics, replay, quiescence, FD0 hash, or frozen-core failure is
an immutable v3 negative. A complete pass establishes FD1 development
readiness only.

RC0, broader cumulative CPC/PQLC replay, ARC, authority, oracle, and `arch.md`
remain blocked.
