# PX3-D1 participation-gated pair learning execution protocol v1

Status: **PREREGISTERED; D1/D1-R EVIDENCE UNSPENT**.

This file freezes the executable surface for the already-preregistered D1
scientific protocol. It authorizes implementation and E2B preflight, followed
by one write-once evidence execution after the implementation freeze passes.

## Commands and artifacts

- package: `arms/px3-d1-participation-gated-pair-learning`;
- preflight command:
  `cargo run --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release -- --d1`;
- CSV: `results/px3_d1_participation_gated_pair_learning_v1.csv`;
- Markdown: `results/px3_d1_participation_gated_pair_learning_v1.md`;
- hidden staging paths use the same basenames plus `.staging`.

`--preflight` audits frozen hashes, the exact matrix, absent result/staging
paths and absent D2/MICRO/GATE surfaces. It constructs no substrate, calls no
propagation and writes nothing. `--d1` repeats these audits before emitting
`PX3_D1_PARTICIPATION_GATED_PAIR_LEARNING_EVIDENCE` and running the matrix. Any
other argument exits nonzero.

## Exact row order

For each seed `2901, 2909`, rows occur in this order:

1. `dormant-baseline`;
2. `return-only`;
3. `a-alone`;
4. `a4-alone`;
5. `a-repeated`;
6. `a-then-b-late`;
7. `ab-one-return`;
8. `ab-recurrent-1-1`;
9. `ab-recurrent-2-1`;
10. `ab-recurrent-4-4`;
11. `ab-no-return`;
12. `ab-recurrent-heldout-matrix`;
13. `d1r-ab-no-return-late-a`.

Every row executes twice from a fresh world. The CSV contains exactly 26 rows,
unique and ordered by `(seed, scenario)`.

Rows 1--12 receive `core_applicable=true` and a `core_pass` bit. Row 13 alone
receives `d1r_applicable=true` and a `d1r_positive` bit. No aggregate pass bit
may make one verdict depend on the other.

## Required serialization

Each row records:

- seed, scenario, raw couplings and scheduled entries;
- source firings, raw crossings/impulses, outlet firings and unit PX1 trace
  firings/ticks per primitive;
- all six opportunity firings;
- all six candidate traversals and native crossing-impulse sums;
- consequence firings and shared-return arrivals;
- candidate liveness/resistance at construction, after first exposure, before
  the recurrent exposure, after training and after the tick-50 gap;
- trained/crossed/gapped/singleton held-out consequences;
- local return updates, pressure, deallocation, full native work, persistent
  bytes, complete/permanent fingerprints, quiescence and exact replay.

No field infers coupling from resistance. Native candidate crossing impulses
are the only carried-coupling observation.

## Independent verdicts

D1 core is positive only if all 24 applicable rows satisfy the frozen
trajectories and controls. D1-R is positive only if both provenance rows show
that late A cannot strengthen eligible AB while consequence return is blocked.
A positive core and negative D1-R is a permitted, interpretable result.

Artifacts publish through create-new staging, sync and atomic rename even for a
negative result. Scientific failure must be preserved, not corrected or rerun.
No D2, MICRO, GATE, definitive or authority command, module or artifact path is
permitted in this implementation.
