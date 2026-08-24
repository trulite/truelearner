# DS-C0 — anonymous evidence-to-choice coupling protocol

Status: **PREREGISTERED; DEVELOPMENT-ONLY ENABLING GATE; NOT CLAIM ELIGIBLE**

This protocol is frozen from exact parent
`d6b75128de7ad4bfb79b2dd4535a0b3d81cabcf0` /
`ds1-after-r0-composition-collapse-handoff`. M0
`1d74c0ed0b515446161a63a6d43ecbe27514dc85` remains authoritative.
DS-E0, DS-A0, DS-A1, DS-R0, and DS-C0 are enabling-only. M1 does not exist.

## Sole question

> Can an actual selected route execution create a temporary anonymous
> eligibility trace that survives until the actual DS-R0 evidence surface
> arrives, then forms a local coupling between the returned evidence and that
> executed interaction without assigning outcome direction or updating DS1?

DS-C0 reconstructs only credit-assignment topology:

```text
selected physical execution
        ↓
temporary eligibility trace
        ↓
anonymous returned evidence
        ↓
temporary local coupling
```

It may not decide whether the evidence is favorable, unfavorable, correct,
wrong, useful, accepted, or rejected. It may not call or wrap frozen DS1
`apply_consequence`, alter boundary-role strengths, compare action outcomes,
or perform held-out reconstruction.

## Immutable lineage

Stage 0 requires exact hashes for:

- parent commit `d6b75128de7ad4bfb79b2dd4535a0b3d81cabcf0`;
- frozen R0 mechanism
  `f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51`;
- frozen stage-8 retry mechanism
  `36c33cb3595001416b4763c29cdba88b5c9567caadc61d8d002177e972ffacce`;
- frozen stage-8 handoff
  `729dd43af12ac5ef35d07f2ddba0609f807344d1e40c4804cf29d478cdd405e6`;
- marked frozen DS1 learner
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`;
- results-tree digest
  `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

Frozen R0 must be executed through a read-only generated composition copy. A
format/access macro may expose only already-existing execution activity and
the already-existing R0 evidence surface. It may not modify frozen code,
invent a route, synthesize an evidence member, or expose evaluator effect to
the C0 mechanism.

## Allowed organism-visible state

DS-C0 may consume only:

- the fresh root and later SPIKE occurrences emitted by the actual selected
  route;
- local temporal precedence among those occurrences;
- physical propagation relations emitted by that route;
- the four anonymous fields already present in the actual R0 evidence surface;
- temporary local CELL activation and ARROW formation/decay.

Physical SPIKE occurrence identity may persist only inside the current
temporary interaction workspace. No occurrence, handle, route root, choice
index, destination, episode identifier, or filler-derived token may enter
persistent state.

The organism-visible C0 mechanism may not consume the evaluator effect
fingerprint, expected route, selected-best marker, seed, work ledger, or test
outcome. Evaluator-side code may verify those properties after execution.

## C0-A — temporary eligibility

Actual selected-route execution must create exactly one short-lived temporary
CELL-like eligibility trace anchored by ordinary physical propagation. The
trace exists before the R0 evidence surface is presented. It contains no
outcome direction and no stable action label.

The trace must:

- transfer across fresh occurrence identities and bijective relabeling;
- survive the preregistered local evidence delay;
- expire outside its temporary lifetime;
- disappear when the route did not execute;
- remain distinct under two interleaved executions;
- clean up completely at the interaction boundary.

## C0-B — anonymous coupling

When the actual R0 evidence surface arrives during the trace lifetime, ordinary
local temporal/propagation continuity may form exactly one temporary ARROW-like
coupling from the eligible executed interaction to the later evidence member.

The coupling must already be determined by physical activity, R0 relations,
and temporary eligibility. No bridge may interpret an opaque choice token or
map a route index to an operation.

Two interleaved executions must pair with their own returned evidence by
physical continuity, not temporal proximity or allocation order. Ambiguous
evidence matching multiple live traces must abstain.

## Ordered stages and stopping rule

0. exact parent/frozen lineage and frozen R0 controls;
1. actual E0/A1/DS1/R0 chain produces one selected physical execution and one
   exact R0 evidence surface;
2. selected execution creates exactly one temporary anonymous eligibility
   trace before evidence arrival;
3. the trace survives the allowed local delay and expires after its frozen
   lifetime;
4. actual returned evidence encounters the trace through matching physical
   temporal/propagation continuity;
5. exactly one anonymous temporary coupling forms with no polarity;
6. fresh-ID, relabeling, layout, handle-permutation, and interleaving controls;
7. ambiguity, distractor, no-execution, no-evidence, stale, and shuffled
   physical-relation controls;
8. leak, no-update, lifetime, work, and complete-cleanup audits.

The first false stage is the only collapse; all later stages are `BLOCKED`.
Passage through stage 8 yields only **DS-C0 DEVELOPMENT IMPLEMENTATION READY**.
It does not retry DS1 and cannot create M1.

## Required controls

For MICRO seed 100 and GATE seeds 100..104:

- all 22 frozen R0 controls remain positive;
- the exact actual R0 target has two roots, two opaque handles, one frozen DS1
  choice, one route execution, three activity occurrences, two propagation
  relations, one return relation, and four exact evidence fields;
- eligibility is absent without selected execution;
- coupling is absent without eligibility or without returned evidence;
- same timing with broken/shuffled propagation does not couple;
- different timing with valid propagation couples only within lifetime;
- evidence from the unexecuted/permuted alternative does not attach to the
  selected trace unless that alternative was physically executed;
- two interleaved traces and evidence surfaces produce two correct disjoint
  couplings;
- duplicate live traces for one evidence path are ambiguous and abstain;
- bijective relabeling, allocation/layout reversal, and opaque-handle
  permutation preserve structural behavior;
- stale traces, delayed-beyond-lifetime evidence, unrelated distractors, and
  missing terminal activity do not couple;
- persistent C0 bytes, retained occurrences, retained handles/roots, outcome
  fields, and semantic update edges are all zero;
- frozen DS1 update count remains zero;
- source/path zeros are mutation-sensitive;
- cleanup erases every C0 CELL, ARROW, trace, coupling, and occurrence
  reference.

## Work, execution, and freeze

Count primitive observations, temporal/propagation comparisons, temporary
CELL creation, temporary ARROW formation, SPIKE delivery, expiry checks, and
cleanup. Report frozen E0/A1/R0 work separately. Maintenance/carrying are zero.
No downstream DS1 update, strength, or held-out work is imputed.

- MICRO: seed 100;
- GATE: seeds 100..104;
- deterministic, single-threaded, release execution;
- `--definitive` rejects before the harness with status 2 and preserves the
  results digest;
- validate formatting, strict release Clippy, focused release tests, MICRO,
  and GATE only;
- validate the exact implementation on persistent E2B using only
  `/Users/satya/.cache/truelearner/ds-c0-anonymous-coupling-e2b.json` and leave
  the sandbox running;
- create no result artifact, cumulative claim, M1, DS1 retry, semantic-credit
  gate, or DS2 work.

Freeze mechanism/source fingerprints, access provenance, leak/negative
controls, dependency manifest, physical ledger, validation, and readiness or
first-collapse handoff.
