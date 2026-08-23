# PX2-O1 post-forgetting opportunity-window diagnostic v1 result audit

## Frozen outcome

Classification **A — finite opportunity window**.

The preregistered diagnostic executed exactly once and emitted exactly one evidence-spend marker:

`PX2_O1_POST_FORGETTING_OPPORTUNITY_WINDOW_DIAGNOSTIC_V1_EVIDENCE`

There was no rerun, rescue, parameter change, or mechanism change after evidence was spent.

## Result

- `40/40` fresh cells were duplicate-exact across four strata, both direction mirrors, and waits `0, 5, 10, 20, 30`.
- At waits `0–20`, the freshly reacquired direction opportunity remained live, actually traversed, received trace-matched return, matured, and executed exactly in held-out and post-gap use.
- At wait `30`, ordinary pressure had reduced proposal resistance to zero before first use. The direction did not traverse, trace, receive return, mature, or execute.
- Pre-use resistance was exactly `3, 3, 2, 1, 0` for waits `0, 5, 10, 20, 30`.
- Successful arms ended at target resistance `40–43`; failed wait-30 arms ended at zero.
- Old direction acquisition, full forgetting, stale-path refusal, fresh PX0 correspondence, and fresh direction identity controls all passed.
- Source refiring remained zero and every cell reached quiescence.

This establishes a finite opportunity window for newly reacquired physical direction structure. It explains the PX2 GATE v1 lifecycle failure without changing the causal-direction law.

## Frozen artifacts

- Frozen result commit: `08f3587`
  - Tag: `px2-o1-post-forgetting-opportunity-window-v1-classification-a`
- Protocol commit: `2fef093`
  - Tag: `px2-o1-post-forgetting-opportunity-window-protocol-v1`
- Executed implementation commit: `2b1fa3e`
  - Tag: `px2-o1-post-forgetting-opportunity-window-implementation-v1`

- Result CSV: `results/px2_o1_post_forgetting_opportunity_window_v1.csv`
  - SHA-256: `4794a4715f00261a7441c602f60f144dded9d7e28d62336f1a41e40242c9dd2a`
- Result report: `results/px2_o1_post_forgetting_opportunity_window_v1.md`
  - SHA-256: `0483898eb11c35650df168363f296756383367e40e6eb789ff2820d95a7a66cb`
- Protocol: `experiments/px2_o1_post_forgetting_opportunity_window_diagnostic_protocol.md`
  - SHA-256: `5381e8dcc25e2ade38ec9b9c2332ef05a09532cb46b7b2748ac214b6b3aac32d`
- Executed implementation source: `crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs`
  - SHA-256: `af0c781eb0b53a7e972497ab4e247e8db2c74b5cf61e06e4de82a8da7be74151`

## Scientific status

- PX0 remains authoritative.
- PX1 remains authoritative.
- PX2 GATE v1 remains an immutable negative.
- PX2 authority remains absent.
- PX3 remains blocked.
