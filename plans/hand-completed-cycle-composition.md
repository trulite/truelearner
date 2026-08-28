```text
output fires -> world changes -> truthful return closes -> same used path gets one successor choice
       |                                                            |
       +---------------- open-return coherence ---------------------+
```

# Compose completed physical control cycles

## Outcome

Add one opt-in learner protocol that composes an open output-return transaction
with exactly one successor choice after a truthful physical-transition return
closes. Test it with the unchanged official batched hand adapter. This is
discovery evidence, not adoption or an Academy capability claim.

## Authority

- Path: `research/campaigns/hand-effect-composition-laws-v1/convergence.toml`
- Revision: `sha256:4baad6ac07b71c5923a4542d4596911f2bd17cb66ee5c3b44a15baa318f6ef57`

## Model

The state is the product of the learner body and external joint world. One
closed arrow is `input -> output -> actual transition -> accepted return ->
updated paths`. Cumulative control requires the codomain of one closed arrow to
be a valid domain for its successor.

`RecursiveLearnerCompletedCycle` extends the coherent-effect protocol. During
local output competition, open-return coherence remains first. When no unique
open transaction wins, exactly one candidate carrying the uniquely latest
recent consequence from an accepted physical transition wins across a changed
ownership view. Multiple equally recent consequences, no consequence, and
stale consequence fall back to the inherited rule. Because this cumulative
protocol already requires physical-transition returns, an unchanged sample
cannot create or refresh the successor.

No new memory is added. The arrow uses the existing owner-local or physical
`last_consequence_tick` recorded by actual participating links. Mutation stays
inside ordinary return and choice boundaries; the official world remains
batched.

## Invariants

- No position, direction, limit, hand step, desired action, score, or motor
  meaning enters the Harness or learner.
- Only an accepted physical-transition consequence can supply completed-cycle
  continuation; samples and rejected returns cannot.
- Open-return coherence has priority over completed-cycle continuation, and a
  unique latest completed cycle has priority over ordinary replacement.
- Equal-latest, missing, stale, unrelated fixture, and old-protocol cases keep
  inherited selection.
- At a clamp, absence of actual transition prevents refresh and the bounded
  recent fact expires, so the output cannot lock forever.
- The batched adapter, old protocols, reflection, checkpoint replay, natural
  quiescence, zero propagation exhaustion, and warm cost remain unchanged.

## Scope

- Extend core protocol predicates, local candidate selection, typed trace, and
  focused harness tests.
- Extend reusable hand evidence with completed-cycle evaluation events.
- Add a new frozen campaign comparing completed-cycle composition with coherent
  and root-fresh batched parents.
- Exclude sequential adapter evidence, official force-law changes, default
  adoption, new durable state, semantic output identity, and authority promotion.

## Development style

TDD. Add mixed-ownership completed-cycle, equal-latest ambiguity, sample, stale,
reflection, protocol-scope, replay, and quiescence fixtures before the frozen
hand run.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary completed_cycle`
  proves accepted-transition continuation, bounded release, ambiguity, and old
  protocol isolation.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml completed_cycle`
  proves reusable trace evidence and exact official batching.
- `cargo test --locked --manifest-path research/experiments/hand-completed-cycle-composition/Cargo.toml --no-run`
  compiles the frozen evidence runner without consuming its one valid run.

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml completed_cycle`.
It must remain strictly under 10 seconds. Cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases are a completed transition whose next view changes ownership,
two equally recent completed candidates, an unchanged sample, an unanswered
return that ages out, reflection, and a clamped output. Negative controls are
the frozen coherent and root-fresh protocols, the exact batched adapter, replay,
quiescence, propagation, and unrelated ownership.

Evidence records each candidate's consequence tick, current owner, whether the
local group crosses ownership views, the unique latest tick, admission, emitted
outputs, actual net movement, opposing-output steps, both contacts and escapes,
perturbation recovery, replay, quiescence, propagation, and work.

The candidate is falsified if no completed cycle is selected across the first
ownership discontinuity, a sample refreshes it, ambiguity selects it, it fails
to release without new transition, the official batched parent changes, or the
one-joint trajectory does not improve. Complete control additionally requires
both contacts, both escapes, and perturbation recovery.

## Risks and rollback

Cross-view continuation could leak another learner's private consequence or
lock a clamped output. Restrict it to one uniquely latest consequence under the
physical-transition-only cumulative protocol, preserve typed controls, and
require bounded expiry. Rollback removes the new protocol and selection branch
while retaining traces and frozen evidence.

## Open decisions

None.
