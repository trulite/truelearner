# Cumulative DS3 event/container-boundary expectation freeze

Protocol identifier: `ds3-cumulative-event-boundary-expectation-v1`

Status: **PREREGISTERED BEFORE ANY PORT IMPLEMENTATION**. This document freezes
the scientific expectation and the first-probe contract for cumulative DS3
before any learner-facing code is written, run, or tuned against M2.

## Frozen authoritative parent

```text
M2 = 162a5b2082a8c1ac9ede45bc5178fecf3509b476
tags ds2-cumulative-causal-direction-definitive-positive,
     m2-cumulative-causal-direction-authoritative
```

The authoritative cumulative prefix is DS0 + DS1 + DS2. Isolated DS3 evidence
does not advance the prefix. Cumulative DS3 may begin only because DS2 is now
frozen positive.

## Frozen isolated DS3 mechanism

The byte-frozen reconstruction mechanism to be ported is isolated DS3 at
branch `bb/ds3-isolated-event-boundary-de-supply-thr_wpuyfghsb9`:

| Artifact | SHA-256 |
|---|---|
| `experiments/ds3_isolated_event_boundary_protocol.md` | `9c1af368724cd743503399822ea57254eabf95a2275ec411a55c09cda0817e3e` |
| `src/ds3_event_boundary.rs` | `a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0` |
| `src/bin/ds3_isolated_event_boundary.rs` | `37bdf23dd82cd00bfdb119b9ed5fedab3c288b34f7b241d1b3546469bf6335c4` |

Implementation commit `bac0222`; final validation commit `10f7101`; readiness
audit records ISOLATED implementation-ready with a positive GATE compilation
trigger (`12 > 8 > 6`) that remains blocked pending separate preregistration.

Its strict dependency manifest names exactly two supplied hard channels that
M2 has since learned to produce organically:

```text
supplied boundary roles            -> learned M1 boundary-role substrate
supplied learner-visible causal    -> learned M2 role-relative direction
annotations                           with retained physical execution
```

## Frozen expectation

> DS3 must first attempt direct mechanistic composition from authoritative
> M2. No new representation should be added unless the cumulative probe
> exposes a specific missing physical dependency.

This expectation is frozen before touching M2 code. It forbids, in priority
order:

1. inventing a new event/container representation instead of composing;
2. adding any adapter that manufactures evidence the M2 ancestor does not
   actually expose;
3. tuning thresholds, signatures, transition rules, or invalidation behavior
   after seeing cumulative output;
4. restoring any removed channel under a new field, token, role, index, or
   name.

If the unchanged port composes, that is stronger architectural evidence than
DS2 itself: learned interaction roles plus learned causal direction become the
substrate for learning higher organization without specifying what that
organization must look like. If it collapses, the exact first collapse point
and the isolated-to-cumulative dependency delta are themselves the result.

## First cumulative DS3 probe (ruthless composition)

> Take byte-identical isolated DS3 as far as possible on M2, replacing its
> supplied boundary-role and causal inputs only by their already-existing
> learned M2 counterparts. Add nothing else.

Permitted work is limited by the mechanical cumulative-port rule in
`desupply_parallel_pipeline_protocol.md`:

- wiring the frozen mechanism to evidence that actually exists in the M2
  parent;
- evaluator-side reporting, artifact paths, matrix enumeration, and accounting
  aggregation that cannot affect organism decisions;
- purely mechanical type/module-path changes proven behavior-neutral.

Any change to learner behavior, organism-visible evidence, persistent
representation, proposal/consolidation rules, or invalidation creates a new
mechanism and may not be reported as the unchanged port.

### Preregistered first-collapse classification

Adopting the isolated manifest's predicted risks in order, the port result
must be classified at the first stage that fails, without repair:

```text
C1  candidate opening          M2 exposes no learned counterpart usable as an
                               open/extend/close boundary observation
C2  continuation validation    no learned counterpart constrains causal
                               reset/continue transitions
C3  recurrence matching        M2 equality semantics cannot support
                               occurrence-free fingerprint reuse
C4  signature discrimination   no stable functional-relation or propagation
                               class exists in M2 evidence
C5  functional scoring         evaluator wiring lacks comparable consequences
                               while organism behavior remains viable
```

A collapse freezes a `CUMULATIVE NEGATIVE` dependency/compositional result at
the exact lettered point. Later stages are `BLOCKED`. No rescue, rerun, or
threshold change follows within this lane.

## Developmental-ordering note (frozen interpretation frame)

The prefix is beginning to establish a developmental ordering rather than only
removing primitives:

```text
continuity/correspondence -> interaction roles -> causal direction
                          -> event organization (this gate)
                          -> control/start/finish -> persistence
                          -> plasticity allocation -> credit
```

This ordering was not designed; it was discovered by failures and successful
cumulative compositions. It is recorded here as interpretation context only
and grants no additional permission to this lane.

## Locks

- Development MICRO/GATE are development-only evidence; they never advance the
  prefix and write nothing under `results/`.
- Exactly one write-once cumulative DEFINITIVE mode exists after separate
  preregistration of its matrix; it executes once on E2B and is never rerun or
  rescued.
- All Rust formatting, compilation, Clippy, tests, MICRO, GATE, and DEFINITIVE
  execution use the shared E2B runner with the dedicated state file
  `/Users/satya/.cache/truelearner/ds3-cumulative-e2b.json`; local commands may
  inspect text, Git state, and hashes only.
- The dedicated sandbox is reused, never killed, and left running.
