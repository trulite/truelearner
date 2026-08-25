# CR0 coupling-necessity discriminator protocol v2

Status: frozen after CR0 v1's immutable measurement negative and before v2
evaluator repair or execution.

Parent: `cr0-coupling-necessity-negative-v1` (`200076f`).

## Eligibility

V1 completed and serialized all 400 physical cases and 800 mechanics rows.
Every functional predicate and same-mechanics replay passed. Its 240
Reference/Production case failures were localized exclusively to equality of
raw live-checkpoint hashes; every preregistered physical observation matched.

V1 also emitted a 25-field header for 26-field rows and printed the aggregate
acceptance boolean where separate replay/mechanics booleans belonged. These are
measurement and packaging defects. No physical counterexample was observed.

## Sole repair class

V2 may make only these changes:

1. Add a `physical_eq` comparison that includes the frozen physical fields:
   ordered transitions, states/measures, event counts, PhysicalWork, physical
   clock, canonical durable body, and quiescence. Raw live-checkpoint hash is
   excluded from this equality but remains serialized as a diagnostic.
2. Append `case_pass` to the CSV header so it matches the existing final row
   field.
3. Track and report replay, mechanics equality, predicates, and aggregate
   acceptance separately. Claim text is conditional on aggregate acceptance.
4. Replace roots `5_100_000/5_200_000` with fresh disjoint roots
   `5_300_000/5_400_000`.

No other evaluator byte may change. In particular, all ten families, durable
states, geometry, thresholds, schedules, physical inputs, expected outcomes,
Reference/Production configurations, replay construction, and decision rules
remain identical to v1.

The physical core and all existing experiment/ARC/Academy/oracle files remain
byte-identical.

## Decision

Any v2 predicate, replay, physical-equality, quiescence, source-boundary, or
artifact failure is an immutable v2 negative.

If all 400 cases and 800 rows pass, classify using v1's frozen decision:

- `coupling necessary` if the neutral retained and threshold controls pass and
  equal-resistance coupling 2 alone crosses the threshold-2 output boundary;
- `coupling unnecessary` if no meaningful efficacy difference appears;
- otherwise `unresolved`.

V2 remains a development discriminator. It does not integrate coupling
plasticity, alter CPC0, resume FD2, run ARC, or advance authority/oracle state.
