# CORE1-E23 — First Participation Differential Audit Result v1

## Status

**POSITIVE LOCALIZATION.** The first physical divergence is
`contact_live`. E23 changes no physics and proposes no mechanism.

Reference replay and Reference/Production observations were exact. Both lanes
naturally quiesced, the unchanged E14 lane remained silent, and the frozen E16
lane produced action 1.

## Five-stage result

```text
                                      E14       E16
context physically present            yes       yes
complete contact structure live        no        yes
route eligible at perturbation         no        yes
route-arrow participation recorded     yes       yes
motor route participates               no        yes
```

The first unequal stage is therefore the second one, not credit, consequence,
variation policy, or motor output.

## Physical trace

Both lanes resolved the same spatial context, `779`. In both, the candidate
source, context-trace CELL, and babbler each fired once.

Unchanged E14:

```text
complete contact pairs before action   0
complete contact pairs after action    0
paired proposals during action         0
eligible pairs at babbler              0
motor firings                           0
outward action                       none
physical work                          30
physical tick                           3
```

Frozen E16 seed-0 first participation:

```text
complete contact pairs before action   2
complete contact pairs after action    2
paired proposals during action         0
positive pairs before action           1
eligible pairs at babbler              1
motor firings                           1
outward action                           1
physical work                          48
physical tick                          11
```

The zero paired proposals in E16's compared action admission are important:
its executable contact topology already existed when the perturbation arrived.
That topology was produced during the frozen two-prime unresolved-context
history. E14 presents a fresh context and babble together; it enters with no
complete pair and leaves with none.

## Non-monotonic trace detail

E14's coarse `drive_emitted` predicate is true because an isolated outgoing
candidate arrow records a positive local-participation delta. But there is no
complete source-to-contact-to-motor pair, no positive pair eligible at the
babbler event, no Drive delivery to the motor, no motor firing, and no outward
crossing.

Thus “some outgoing arrow participated” is weaker than “an executable route
participated.” E23 localizes the missing event more precisely as:

> A complete generated source → contact → motor route is not live when the
> E14 perturbation arrives.

This is the first broken physical transition. Everything after it—including
E22's participation-born credit path—is causally unreachable.

## Interpretation boundary

E23 does not establish why E14 fails to form or retain the complete pair. The
two frozen lanes differ in prior physical history: E16 presents the unresolved
context twice and recovers for one tick before perturbation; E14 combines its
first context presentation and teaching babble in one admission. That is the
next localization boundary, not a mechanism recommendation.

Credit remains closed:

```text
E22: executable route participates
  -> temporary physical return topology
  -> later consequence
  -> PQLC

E23: unchanged E14
  -> context source and trace fire
  -> no complete live route
  -> participation chain never begins
```

## Evidence note

The protocol requested raw per-link stem/outgoing counts and materials. The
evaluator reduced those observations to complete-pair counts, positive-pair
counts, eligibility-at-babbler counts, and participation deltas, but did not
persist every raw per-link value. The decisive zero-pair versus two-pair
localization and its material eligibility reduction are present in the frozen
matrix. This bounded measurement omission was discovered after the one-shot
run; no rerun occurred.

## Validation and provenance

- protocol commit: `29ec202`;
- observer/instrumentation commit: `7c52ceb`;
- evidence marker: emitted once;
- definitive differential run: once;
- CORE1 runtime: byte-identical;
- E14 and E16 evaluators: byte-identical;
- strict release E23 Clippy: passed;
- focused blocked-return Academy control: passed;
- formatting and `git diff --check`: passed;
- core0-only Academy Clippy remains blocked by inherited E22 code that refers
  to core1-only transient-continuation fields; E23 introduced no code in that
  path and the required core1 build passed.

Evidence:

- `experiments/results/core1_e23_first_participation_differential_audit_v1/matrix.csv`
- `experiments/results/core1_e23_first_participation_differential_audit_v1/report.md`

SHA-256:

- protocol:
  `8302123f72a439a349e2b711459a1e297ffe59c9fd4845c0e1ef3accd12b6586`;
- implementation audit:
  `c4d9b9c094774d2f3c74c61c650576f3051069676532939537d7912c52f7cb12`;
- E23 evaluator:
  `9b8d5edc9d76e1bfac8437d60db7f882d8cfb71d2c1b21b16d5a55aa688ccfbd`;
- result matrix:
  `172caa0c6407c30146943b634482c7a0d89d12bc5a3cad3182d9ed3a598cf197`;
- generated report:
  `1197f797d3d0a724504051d8d9a7479edc698fde72d7d19f9ed983c54372439d`.
