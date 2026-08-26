# RB0 recurrence boundary protocol v3 phase freeze

V3 inherits RB0 v1 plus the v2 local-forgetting correction. Before evaluator
implementation, it resolves the phrase “same phases” into the two exact
historical geometries whose difference RB0 must not hide:

- `core0_phase`: relay ARROW phase 0; negative ARROW phase 0. Causal-wave
  succession supplies the local order.
- `rs1_phase`: relay ARROW phase 0; negative ARROW phase 1, matching the RS1
  strength sweep.

Every frozen cycle tuple in the efficacy, threshold, and delay sections runs
under both phase patterns. Within a row, RS1-style and CORE-B always receive
the identical pattern. The one-way control also runs under both.

This addition is diagnostic, not a parameter search: both complete maps are
published, no phase pattern is selected after observation, and the decision
still asks whether a finite ordinary inhibitory region exists rather than
which fixture should be preferred.

