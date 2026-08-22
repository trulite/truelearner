# DS-D0 stage-8b single-property discrimination protocol

Status: **PREREGISTERED DIAGNOSTIC; DEVELOPMENT ONLY; NOT CLAIM ELIGIBLE**

This diagnostic forks only from exact frozen stage-8b collapse
`7ea5680046b57fcbd81e31996e49be3ec3e9fc36` /
`ds1-after-c0-composition-collapse-handoff`. M0
`1d74c0ed0b515446161a63a6d43ecbe27514dc85` remains authoritative.
E0+A0+A1+R0+C0 remain enabling-only. M1 is absent.

## Sole question

> If exactly one candidate property is evaluator-supplied to a read-only
> diagnostic fixture, can the byte-identical frozen DS1 update edge be
> physically reached for the same episode and choice?

This is dependency discrimination, not a learned replacement, cumulative
composition, functional recovery, or evidence that the organism possesses the
property. A positive arm only nominates a later developmental prerequisite.

## Frozen matrix

All arms use the same parent, DS1 hash, C0 episode construction, choices, and
MICRO/GATE seeds. No property combinations are permitted in this round.

| Arm | Only additional diagnostic property |
|---|---|
| `OWNERSHIP_ONLY` | C0 temporal ownership; no additional field |
| `TEMPORAL_CONTRAST` | two local times, with no evaluative orientation |
| `ALTERNATIVE_COMPARISON` | two opaque outcome magnitudes associated with the two already-existing alternatives |
| `POLARITY` | one supplied favorable/unfavorable bit |
| `OUTCOME_CHANGE` | one before/after magnitude pair |

The temporal arm may not turn later/earlier into favorable/unfavorable.
Alternative comparison may only compare the already-supplied two magnitudes at
the already-frozen selected index. Outcome change may only compare its supplied
after/before magnitudes. The polarity arm may forward its one bit unchanged.
No arm may inspect evaluator correctness or expected roles.

## Exact diagnostic boundary

The diagnostic reproduces the exact deterministic E0 target episode consumed
by frozen R0/C0 for each seed. A marked byte-identical frozen DS1 learner:

1. consumes that neighborhood;
2. makes the same tie-broken choice reported by C0;
3. receives no update for an arm whose sole property cannot populate the
   existing `positive: bool` input;
4. receives exactly one call to its existing `apply_consequence` only when the
   named property alone mechanically yields that boolean.

The fixture accessor is diagnostic-only and must live outside the marked
learner slice. The marked slice SHA-256 remains
`adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`.

No arm trains to competence, observes held-out reconstruction, persists the
injected property, or changes C0. Reachability is:

```text
candidate property alone yields existing update input
AND
frozen apply_consequence call executes
AND
frozen credit_updates increases by exactly one
```

## Anti-combination and leak rules

- Every matrix cell contains exactly one arm enum value.
- Candidate-property payloads have disjoint schemas.
- No arm reads another arm's payload or output.
- No arm receives evaluator correctness, role truth, expected choice, source or
  target labels, economic feedback, or a learned/cumulative adapter.
- The two alternative magnitudes are fixed before the chosen index is read;
  they are not constructed to favor the selected action.
- Polarity and outcome-change signs vary across seeds.
- Temporal contrast remains non-evaluative even though its times are ordered.
- `OWNERSHIP_ONLY` reproduces the frozen stage-8b zero-update baseline.
- Source and runtime inventories must independently count candidate-to-bool
  mappings, diagnostic update edges, and runtime updates.

## Execution

- MICRO seed: 100; GATE seeds: 100..104.
- Five arms per seed execute as independent deterministic cells through
  `parallel_map_ordered`; returned rows are restored to fixed seed/arm order.
- Run only the one-episode reachability screen. Do not automatically promote a
  positive arm to learning or functional gates.
- `--definitive` rejects before the harness with status 2.
- Preserve the results digest
  `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.
- Validate formatting, strict release Clippy, focused tests, MICRO, and GATE
  locally and on persistent E2B using only
  `/Users/satya/.cache/truelearner/ds-d0-stage8b-discrimination-e2b.json`.
- Leave the E2B sandbox running.

## Frozen outputs

For every seed/arm report:

- exact-parent and DS1 fingerprints;
- frozen C0 choice and diagnostic choice agreement;
- property fields visible;
- whether that property alone yielded the update input;
- reachable update edges;
- runtime DS1 update delta;
- persistent bytes added (must be zero);
- diagnostic-only classification.

Then report the set of individually sufficient properties. If none is
sufficient, a separately preregistered pairwise round may follow. If one or
more are sufficient, each remains only a candidate for a separate learned
enabling experiment. No DS-D0 result advances the authoritative lineage.
