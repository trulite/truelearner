# FD1 negative-v1 C3 diagnostic protocol v1

Status: frozen before diagnostic evaluator edits.

Parent event: `fd1-consequence-consolidation-negative-v1` (`3d1ffb7`).

## Question

Which member of the frozen C3 `same_durable_state_same_future` conjunction was
false in FD1 v1?

This diagnostic cannot change, rescue, or relabel FD1 v1. It may only classify
the already-observed negative.

## Frozen scope

Recreate C3 with disjoint roots `4_500_000` and `4_600_000`, creation phases
`0..9`, and both Reference and Production mechanics.

For each world, serialize separately:

- candidate state after the early age-1 consolidation;
- candidate state after the late age-9 consolidation;
- candidate state 39 ticks after each consolidation;
- candidate state 40 ticks after each consolidation;
- PhysicalWork accumulated during each 39-tick future interval;
- equality of each state pair;
- equality of future PhysicalWork;
- exact same-mechanics replay;
- exact Reference/Production diagnostic observation;
- natural quiescence.

`ArrowState` must expose liveness, resistance, local decay load, participation,
plastic support, and stale-generation resolution. The diagnostic must not
collapse these values into a single composite predicate before serialization.

## Classification

- If a candidate state pair differs, report the exact field and values.
- If only future PhysicalWork differs while all candidate-state pairs match,
  classify C3 as an evaluator/fixture measurement defect: C3 supplied different
  absolute-age histories outside the normalized candidate state and incorrectly
  required total whole-body work equality.
- If normalized candidate state differs, classify the candidate physical law as
  a scientific negative at C3.
- Any replay, mechanics, or quiescence mismatch is classified separately and
  stops the diagnostic.

No formula, runtime state, event order, decay law, evaluator schedule, or C3
expectation may be repaired inside the diagnostic.

## Boundary

No C0-C2 rerun, C4-C6 execution, FD0 replay, FD1 v2, ARC, CPC/PQLC, RC0,
authority, oracle, or `arch.md` change is authorized.
