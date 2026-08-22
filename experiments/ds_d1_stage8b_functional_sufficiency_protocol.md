# DS-D1 stage-8b functional-sufficiency diagnostic protocol

Status: **PREREGISTERED DIAGNOSTIC; DEVELOPMENT ONLY; NOT CLAIM ELIGIBLE**

This protocol consumes only frozen DS-D0 outcome
`b599e601e9a7257c647cf5ca8f4188d77d024f02` /
`ds-d0-stage8b-single-property-discrimination-complete`. M0 remains
`1d74c0ed0b515446161a63a6d43ecbe27514dc85`; M1 is absent.

## Sole question

> For each individually update-reachable property, does repeated
> evaluator-supplied experience cause byte-identical frozen DS1 to update,
> separate the two alternative strengths, and recover the held-out boundary
> role?

This is still diagnostic. It does not show that a property is learned,
organism-observable, or admissible for cumulative composition.

## Frozen arms

Run exactly three independent arms, with no combinations:

1. `ALTERNATIVE_COMPARISON` — two supplied alternative outcome magnitudes;
2. `POLARITY` — one supplied favorable/unfavorable bit;
3. `OUTCOME_CHANGE` — one supplied before/after magnitude pair.

All arms use the same E0 episode schedule, fresh occurrence identities,
contexts, frozen DS1 tie breakers, seeds, acquisition count, and held-out
episodes. Each property is evaluator-side and erased after its update.

## Evaluator-only role relation

For diagnosis only, the evaluator derives the hidden boundary alternative from
the same anonymous signature relation used by the frozen isolated DS1 anchor:

```text
expected alternative = (gap - 1) XOR witness attachment
```

This relation never enters the learner. It only constructs each arm's supplied
property:

- comparison: expected alternative has the larger supplied magnitude;
- polarity: selected alternative receives `choice == expected`;
- outcome change: expected selection produces positive signed change and the
  other negative signed change.

Each arm then independently maps only its own payload into frozen DS1's
existing `positive: bool` parameter. This controlled alignment is necessary to
test functional sufficiency, but it is evaluator semantics and therefore can
never advance M0.

## Three outcome tiers

For every seed/arm report independently:

1. **UPDATE** — `credit_updates` increases exactly once per acquisition episode;
2. **DIVERGENCE** — for every learned signature, the two alternative strengths
   differ and the mature choice equals the evaluator-only role relation;
3. **RECOVERY** — frozen `frozen_choice` is correct on every fresh held-out E0
   episode, with zero abstentions.

Failure at one tier blocks later tiers for that arm's classification. The
other arms continue independently.

## Encoding-equivalence audit

If multiple arms recover fully, do not rank them by runtime. First compare:

- exact boolean update trace;
- persistent learner fingerprint;
- update count, strengths, mature choices, and held-out choices.

If these are identical, classify the arms as **functionally equivalent
encodings at the frozen DS1 boundary**. That means this diagnostic cannot
identify which channel should be learned. Subsequent selection must prioritize:

1. minimal information content;
2. observability from currently allowed substrate physics;
3. absence of evaluator semantics;
4. only then work and storage.

No claim of current substrate observability may be inferred here.

## Execution and controls

- MICRO: seed 100, 16 acquisition and 8 held-out episodes;
- GATE: seeds 100..104, 32 acquisition and 16 held-out episodes;
- three arms per seed execute through `parallel_map_ordered`;
- exact frozen DS1 learner SHA-256:
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`;
- exact DS-D0 parent, C0, E0, M0, and results fingerprints;
- no combined arm, persistent property field, C0 edit, learned prerequisite,
  cumulative retry, M1, or economics;
- `--definitive` rejects before execution with status 2;
- focused format/Clippy/tests plus release MICRO/GATE locally and on E2B;
- use only
  `/Users/satya/.cache/truelearner/ds-d1-functional-sufficiency-e2b.json` and
  leave the E2B sandbox running;
- preserve results digest
  `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

Freeze the three-tier matrix and encoding-equivalence result. Do not implement
a learned prerequisite in this lane.
