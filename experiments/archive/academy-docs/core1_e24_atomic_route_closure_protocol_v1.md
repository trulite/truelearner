# CORE1-E24 — Atomic Route Closure Protocol v1

## Status

Preregistered after E23 positive localization `b96d9d2` and before E24
implementation or runtime observation. E24 tests one physical hypothesis with
one candidate. It is not another audit or a tournament.

## Hypothesis

> E14 fails because route formation is not atomic. Source-side and motor-side
> fragments appear, but they do not become a complete live
> source → contact → motor route in the same physical window.

E23 established the predicted baseline:

```text
source fires                    yes
context trace fires             yes
babbler fires                   yes
isolated candidate activity     yes
complete live contact pair       no
motor participation              no
```

## Sole candidate

Under one experimental CORE1 flag, generic local variation changes only the
formation timing of the subdivision it already knows how to make.

When a source firing proposes a new local direct Drive candidate, the same
`propose_generic_local_edits` event must also create that candidate's existing
subdivision before the source firing enumerates outgoing arrows:

```text
source fires near live target opportunity
-> direct local candidate is proposed
  == same proposal event ==
-> contact CELL is proposed at source position
-> source -> contact stem is proposed
-> contact -> target outgoing is proposed
```

The complete route uses the unchanged candidate sign, material magnitude,
resistance, target, distance-derived delay, contact threshold, and ordinary
tentative lifetime. Both existing generic signs remain symmetric. No sign is
selected or suppressed. The original direct candidate may remain exactly as in
the base mechanism.

Atomic means the new direct candidate is eligible for subdivision in the same
source-firing event rather than only after a later source firing. Formation may
not inspect a future input, queued babbler, action identifier, consequence,
usefulness, context label, evaluator diagnostic, or outcome.

No extra route protection, TTL, timeout, OPEN state, completion, refractory
rule, variation schedule, propagation gain, delay change, PQLC change, or E22
credit change is permitted.

## Frozen base

Reuse candidate CORE1 commit `bc74c7c` plus E23 observation-only trace exposure.
Keep unchanged:

- E14 root `93_000_000`, five deterministic frames, action map, available
  actions, curriculum `[1,4,2,3]`, support timing, and closing observation;
- E22 atomic participation-born credit return as candidate-default physics;
- ordinary generic variation, continuous material, pressure, decay,
  propagation, work accounting, and natural quiescence;
- Reference, exact Reference replay, and Production equality.

CORE1 runtime changes may add only one experimental flag and same-event
inclusion of newly proposed direct candidates in the already-existing
subdivision loop. Academy may expose one enable method. E14 and E16 frozen
evaluators remain byte-identical and are not rerun.

## Gate 1 — atomic formation

Run only the first frozen E14 teaching turn with E24 enabled. Under Reference,
exact Reference replay, and Production require:

- context source, context trace, and babbler fire;
- at least one paired contact proposal occurs in that admission;
- at least one complete source → contact → motor pair is live when the
  admission quiesces;
- passive USED-PENDING remains zero;
- exact replay, exact mechanics, bounded work, and natural quiescence.

If no complete route forms, freeze E24 as an implementation/hypothesis
negative. No repair is permitted.

## Gate 2 — participation falsifier

Using that same first-turn observation, require a motor crossing/action.

If complete live routes exist but action remains absent, the preregistered
hypothesis is falsified:

> Atomic route closure repairs formation but is not sufficient for
> participation; the first remaining break is downstream of formation.

Stop immediately. Do not run consequence, the five-turn regimen, autonomous
probes, or any repair.

## Gate 3 — unchanged E14 learning chain

Only if the first action participates, run the complete frozen E14 teaching
and closure regimen with the E24 flag as the sole addition. Require the
unchanged E14 acceptance predicate:

```text
teaching actions  [1,4,2,3]
updates           [0,1,1,1,1]
natural quiescence throughout
```

Additionally record nonzero later Modulatory delivery, nonzero PQLC updates,
temporary credit-return cleanup, and passive pending zero.

After the frozen closing observation, clone/recover the trained organism and
probe each of the four taught frames once with no babble or support. Record the
autonomous action vector. Full E24 success requires `[1,4,2,3]`.

Any Gate-3 failure is frozen without repair or another seed.

## Controls and rejection screen

- E24 disabled: unchanged E23/E14 first-turn `0` complete pairs and no action.
- No source firing: no atomic closure.
- Non-local or non-Drive candidates: no atomic closure.
- Atomic closure itself performs no material update or Modulatory delivery.
- No route may be supplied by the evaluator or chosen by action/usefulness.
- Existing positive and negative signs must both remain present.
- No post-admission reconstruction is allowed.
- No E22 return edge may be created before an outgoing contact route actually
  participates.

Reject exactness on replay/mechanics mismatch, non-quiescence, unbounded work,
or any frozen-evaluator/runtime change outside the allowed surface.

## Evidence discipline

Provide a non-executing check mode, strict release Clippy, static diff/hash
audit, then run the staged evaluator once. Emit the evidence marker before
Gate 1. The first failed gate stops later gates.
