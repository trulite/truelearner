```text
physical link consequence -> organism view -> completed choice
           |                      |
     construction             owner projection
           |                      |
           v                      v
 same link alive?       new learner private view -> missing or present?
```

# Debug the construction-boundary cycle witness

## Outcome

Preserve and analyze existing physical trace events around the first tick-23
arrow change. Determine whether the tick-8 consequence link survives and
participates after construction but becomes invisible to learner two, or
whether the physical witness closes, deallocates, or stops participating. This
is evidence retention only; learner physics remains unchanged.

## Authority

- Path: `research/campaigns/hand-completed-cycle-naturality-v1/convergence.toml`
- Revision: `sha256:d7117c56480f0e2a6f4205eecb554a661eb210d6dd854a3ac97ca625472141cd`

## Model

The physical consequence-bearing link is the object under observation.
Consequence write, construction, traversal, and candidate read are ordered
arrows already present in `PhysicalEvent`. Organism selection projects
consequence from the live link; learner-owned selection projects consequence
from that learner's private `(link, generation)` memory. The debug compares
these two existing projections for the same physical link across construction.

Add no core event. Extend reusable hand evidence with one ordered tagged union
that losslessly retains existing events needed for this question: link
deallocation, qualified traversal, physical and learner consequence writes,
learner construction, candidate preference, surface-path state, output
candidate evaluation, return scheduling, candidate selection, completed-cycle
evaluation, and group-level admission.

A pure experiment-side analyzer starts from the last uniquely completed target
before the first ownership-changing target switch. It obtains the accepted
consequence tick and link set, follows every link through construction to the
failure choice, and returns one typed verdict: `OwnerProjectionGap`,
`PhysicalWitnessDeallocated`, `PhysicalWitnessNotParticipating`, or
`InsufficientExistingTrace`.

## Invariants

- No `PhysicalEvent`, learner state, selection, output, path, memory, clock, or
  work behavior changes.
- Evidence retains existing event order, tick, phase, link, generation, owner,
  target, and decision without reconstructing a semantic action identity.
- Position, direction, hand step, limit, target meaning, score, and expected
  action remain evaluator-only annotations.
- A projection gap requires the same consequence-bearing link to remain
  undeallocated and participate at the failure while the new owner has no
  matching consequence write/read.
- A deallocation or non-participation verdict must be explicit; absence of a
  retained fact yields `InsufficientExistingTrace` rather than inference.
- The exact frozen hand summary and first tick-23 arrow change remain unchanged.

## Scope

- Extend only the hand adapter's evidence structs and observer mapping for
  already existing trace events.
- Add one successor diagnostic campaign and pure witness analyzer.
- Exclude core trace additions, learner or adapter physics, memory copying,
  selection laws, semantic identities, default adoption, and authority
  promotion.

## Development style

TDD. Test ordered retention against raw existing events, then test each pure
verdict with synthetic trace slices before compiling the one-shot hand runner.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml existing_witness_trace`
  proves the adapter preserves existing event order and the exact hand summary.
- `cargo test --locked --manifest-path research/experiments/hand-construction-cycle-witness/Cargo.toml witness_verdict`
  proves the pure analyzer distinguishes projection gap, deallocation,
  non-participation, and insufficient evidence.
- `cargo test --locked --manifest-path research/experiments/hand-construction-cycle-witness/Cargo.toml --no-run`
  compiles the frozen runner without consuming its valid run.

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml existing_witness_trace`.
It must remain strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

Held-out cases are an undeallocated participating witness without new-owner
memory, a deallocated witness, a live but non-participating witness, and missing
retained evidence. Negative controls are raw-event/order agreement, the exact
prior hand and tick-23 failure summaries, child-fresh-memory isolation, replay,
natural quiescence, and zero propagation exhaustion.

The frozen artifact retains the complete decisive event slice from the accepted
consequence through the failure choice and reports the typed verdict with exact
link and generation evidence. The diagnostic is falsified if event retention
changes behavior, if the decisive slice is incomplete, or if the verdict
depends on semantic world knowledge.

## Risks and rollback

An incomplete reducer could again discard the decisive fact. Make the analyzer
return insufficient evidence unless every required existing event is present.
Rollback removes only adapter evidence fields and the successor experiment; no
body or checkpoint migration is required.

## Open decisions

None.
