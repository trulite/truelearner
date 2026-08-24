# PX3 LR-C physical event organization definitive protocol v2

Status: **PREREGISTERED MEASUREMENT REPAIR; V2 EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

## Immutable parent

V1 remains the permanent definitive negative recorded at commit
`26d3cbd`, tag `px3-lrc-physical-event-definitive-negative-v1`:

```text
lifecycle       0/16 rows, 176/192 clauses
recursion      16/16 rows, 192/192 clauses
joint          16/32 rows, 368/384 clauses
```

Frozen v1 result-audit SHA-256:
`668aeb5802194a98002edd95bb55d09100825637c1f3c4a5ca1a711b9e0565a2`.

V1 failed only L5 because the evaluator required the global native structural
proposal counter to remain zero. An externally fired independent effect
lawfully proposed one weak `effect -> P` Drive ARROW. Direct observation showed
zero joint `P -> effect` candidates, zero P firing and zero plasticity updates.

## Sole permissible change

V2 changes only L5 measurement. Physics, topology, schedules, thresholds,
couplings, Drive/Modulatory modes, context, timing, resistance, pressure,
reversal, recursion and L0--L4/L6--L11 predicates remain byte-identical.

L5 must separately serialize and require:

```text
independent effect path firing                  1
world-return relay firing                       1
modulatory transmitter firing                   1
Modulatory crossing to P                        1
unrelated native effect -> P proposal            1  (allowed)
joint P -> effect candidate count                0
joint P -> effect candidate traversal            0
joint P -> effect resistance                     0
P firing                                         0
plasticity updates                               0
quiescent                                     true
```

The global structural proposal counter remains serialized but is not a verdict
predicate. No mechanism or organism-visible state is added.

## Fresh disjoint identities

V2 uses sixteen lifecycle seeds `93001..93016` and sixteen recursion seeds in
`94003..94071`, with exactly four rows in each normal/reversed and
forward/reflected stratum. Namespace remains `seed << 32`; therefore no v1 or
development identity is reused.

Every row executes twice from fresh complete state for exact replay. The
conjunctive verdict remains exactly `32/32` rows and `384/384` independently
serialized clauses.

## Freeze, preflight and one-shot execution

Both v2 sources must be committed and tagged before either result runs. A
mechanical source-diff audit must show that the only non-identity/publication
change is the L5 direct observation and predicate.

Two distinct fresh E2B preflight sandboxes must pass formatting, release tests,
strict Clippy, frozen hashes, fresh matrix identity, refusal, no-world preflight
and artifact absence. Then both halves execute once and concurrently in two new
E2B sandboxes. No rerun, rescue or amendment is allowed after either v2 marker.

Registered commands:

```text
cargo run --release --manifest-path arms/px3-lrc-lifecycle/Cargo.toml -- --definitive
cargo run --release --manifest-path arms/px3-lrc-recursion/Cargo.toml -- --definitive
```

Registered artifacts:

```text
results/px3_lrc_lifecycle_definitive_v2.csv
results/px3_lrc_lifecycle_definitive_v2.md
results/px3_lrc_recursion_definitive_v2.csv
results/px3_lrc_recursion_definitive_v2.md
```

A full positive permits a separately frozen PX3 authority handoff. It does not
authorize PX4 execution in this workflow.
