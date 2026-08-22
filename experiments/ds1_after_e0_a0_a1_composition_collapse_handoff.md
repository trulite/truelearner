# Unchanged DS1 after DS-E0 + DS-A0/A1 composition collapse handoff

Outcome: **CUMULATIVE DEVELOPMENT COLLAPSE AT STAGE 7 — frozen DS1 receives
naturally existing post-choice evidence**.

This is development-only dependency evidence. It is not a definitive result,
does not advance the cumulative prefix, and creates no M1.

## Exact progression

Across MICRO seed 100 and GATE seeds 100..104:

```text
0  M0 / correspondence lineage and fingerprints             READY
1  actual learned E0 event                                  READY
2  exact E0 -> A1 transfer                                  READY
3  two independently installed executable A1 roots          READY
4  two opaque one-to-one alternatives visible to DS1        READY
5  byte-identical frozen DS1 chooses one                     READY
6  selected root physically executes                        READY
7  natural post-choice evidence reaches frozen DS1          COLLAPSE
8  unchanged DS1 update                                     BLOCKED
9  boundary-role reconstruction                             BLOCKED
```

The new result is therefore narrower than “consequence is missing.” Physical
execution returns an evaluator-visible normalized effect, but the current
substrate has no organism-visible path carrying any post-choice evidence from
that execution into frozen DS1. No observer, boolean consequence, semantic
translation, or `apply_consequence` call was added.

## Per-seed signature

Every GATE seed produced:

- one exact actual E0 target event;
- two local A1 candidates and three mature anonymous templates;
- two installed roots, four route CELLs, and two live ARROWs before bridge;
- two structurally unique routes, two normalized effects, and two opaque
  handles;
- frozen DS1 choice arity two and exactly one `choose` call;
- one selected ordinary execution and one handle-permutation control
  execution;
- two SPIKE propagations, one ARROW traversal, and two state mutations in the
  ordinary selected execution;
- zero post-choice evidence events and zero DS1 updates.

Seeds 100, 102, and 104 selected opaque index 0; seeds 101 and 103 selected
index 1. Reversing the opaque bridge order changed the physically executed
route while preserving the pre-choice root/effect inventory.

## Frozen lineage

- enabling parent: `3f12055bf6434044095c3e5ca00e23b35806b630` /
  `ds-a1-affordance-multiplicity-readiness`;
- protocol: `711a19955c401007ddee446bf8ff3670c896a83c` /
  `ds1-after-e0-a0-a1-composition-retry-protocol`;
- implementation: `613444e5c41be1f71884c0677ff2634c9ea34146` /
  `ds1-after-e0-a0-a1-composition-retry-implementation`;
- collapse handoff: the commit containing this file /
  `ds1-after-e0-a0-a1-composition-collapse-handoff`.

## Validation

The exact implementation snapshot passed locally and on persistent E2B:

```text
cargo fmt --all -- --check
cargo clippy --release --bin ds1_after_e0_a0_a1_composition_retry -- -D warnings
cargo test --release --bin ds1_after_e0_a0_a1_composition_retry   # 21 passed
cargo run --release --quiet --bin ds1_after_e0_a0_a1_composition_retry -- --micro
cargo run --release --quiet --bin ds1_after_e0_a0_a1_composition_retry -- --gate
```

`--definitive` rejected before the harness with status 2. The results-tree
digest remained
`491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.
No result artifact was written.

E2B used only
`/Users/satya/.cache/truelearner/ds1-after-e0-a0-a1-composition-e2b.json`.
Sandbox `ibwytthxkqjxqnm8rer7m` remains running with an 86,400-second timeout.

## Status

M0 `1d74c0e` remains authoritative. E0+A0+A1 remain enabling-only. M1 is
absent. The next dependency may be investigated only through a separate
preregistration; this lane added no rescue mechanism.

