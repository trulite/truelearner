# DS-C0 leak and negative-control audit

All 19 C0 controls passed for MICRO seed 100 and GATE seeds 100..104.

## Transfer and structural controls

- fresh occurrence populations are disjoint;
- bijective occurrence relabeling preserves coupling;
- pulse/propagation allocation order reversal preserves coupling;
- opaque-handle permutation executes a distinct real route and still forms the
  corresponding coupling;
- two interleaved executions create two eligibility traces and two disjoint
  correctly paired couplings;
- unrelated temporally close distractor activity does not alter the pair.

Pairing is therefore determined by actual root-to-terminal propagation
continuity, not timing proximity, vector position, allocation order, or an
evaluator-supplied action menu.

## Negative and lifetime controls

- no selected execution creates no eligibility and no coupling;
- selected execution without evidence creates no coupling;
- evidence after tick 3 finds the trace stale and does not couple;
- reversed physical propagation does not couple;
- missing terminal activity does not couple;
- two duplicate live traces matching one evidence path are ambiguous and
  abstain;
- cleanup erases every temporary cell, arrow, occurrence reference, and
  coupling.

## Semantic and identity leak controls

- coupling polarity fields: 0;
- DS1 updates: 0;
- persistent C0 bytes: 0;
- stable handles, route roots, destinations, episode IDs, and filler tokens in
  persistent state: 0;
- evaluator effect paths into the C0 workspace: 0;
- correctness/reward/accepted/rejected update paths: 0;
- result artifacts and definitive execution: 0.

Source mutation tests detect inserted update or evaluator-effect paths. Frozen
R0's existing 22 leak/negative/lifetime controls also pass unchanged.
