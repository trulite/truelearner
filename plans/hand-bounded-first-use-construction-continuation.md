```text
construction projects outcome -> held for first matching choice -> ordinary lifetime
                                      | exact live link             |
                                      `---- one competition --------'
```

# Bound construction continuation to its first matching choice

## Outcome

Add a new opt-in learner protocol in which an exact consequence projected during
learner construction remains eligible through its first matching local output
competition, then immediately returns to ordinary consequence lifetime. Test the
proposed law first in small benchmark-blind fixtures and then once in the
unchanged reflected hand. The solve succeeds only if target eleven continues the
physical arrow at tick twenty-three and the hand reaches the upper boundary while
the integrated parent, ordinary stale release, replay, quiescence, and all exact
lineage boundaries remain intact.

## Authority

- Path: `research/campaigns/hand-complete-construction-naturality-contract-v1/convergence.toml`
- Revision: `sha256:16582b6023366d642d6346c29e7594abd1746868edb9a96603da5c1c191784b8`

## Model

An owner-local consequence memory has one of two lifetime states: `Ordinary` or
`HeldForFirstChoice`. Same-tick construction projection creates the latter only
under the new protocol. An ordinary accepted outcome writes `Ordinary` and thus
replaces any pending construction lifetime on that exact link generation.

A pure candidate read returns both the latest consequence tick and the latest
held tick carried by an actual live completing input. Completed-cycle resolution
compares recent ordinary ticks and held ticks using the existing unique-latest
rule. Missing, stale, and ambiguous results remain explicit.

The effect boundary follows resolution. For every executable candidate in a real
local output competition, exact live completing held memories transition once
from `HeldForFirstChoice` to `Ordinary`, even when another priority wins the final
output. Single candidates, blocked paths, wrong generations, dead links, and
non-completing links do not consume the state. This is a one-use lifetime for an
existing outcome, not a held path, action history, direction memory, or global
recency extension.

## Invariants

- The existing `RecursiveLearnerConstructionOutcomeComposition` protocol and
  accepted default retain their exact behavior; the new law is a separate opt-in
  protocol.
- Construction projection still requires a live exact-generation link in the
  learner's construction lineage with consequence tick equal to construction.
- The original consequence tick is never refreshed by holding or consuming it.
- Only a matching executable completing path in a multi-candidate local choice
  may consume held state, and each exact memory can be consumed at most once.
- Consumption happens after completed-cycle resolution and does not depend on
  the final winning output or evaluator knowledge.
- Ordinary fresh, stale, missing, replacement, deallocation, ambiguity, sibling,
  and no-consequence release semantics remain unchanged.
- Checkpoint restore preserves the lifetime state for newly written checkpoints;
  exact replay remains unchanged. Cross-version checkpoint migration is outside
  this candidate's scope.
- Trace records the exact owner, link, generation, consequence tick, and target
  for every consumed construction continuation.
- No position, direction, hand identity, expected target, score, or answer enters
  the learner law.

## Scope

- Add one lifetime enum to owner-local consequence memory and total read, write,
  and consume transformations in `truelearner/crates/core/src/learner.rs`.
- Add one opt-in protocol variant and inherit the exact parent protocol bindings.
- Extend completed-cycle candidate resolution and trace with held-first-use
  eligibility and exact consumption events.
- Preserve snapshot validation, deterministic serialization, observation
  fingerprint behavior, and bounded memory accounting.
- Extend the developmental-hand evidence adapter for the new trace event.
- Add one successor experiment and frozen campaign with constructed-world,
  integrated-parent, global-stale, exact-lineage, replay, quiescence, work, and
  unchanged-hand controls.
- Exclude adapter force changes, global recency changes, held paths, parent-memory
  copying, semantic direction, default adoption, and authority promotion.

## Development style

TDD. First add focused failing tests for lifetime transitions, unique held
eligibility, one-use consumption, losing-candidate consumption, delayed first
match, and ordinary stale controls. Then add the new protocol and experiment.
Compile all frozen gates before executing the one valid hand run.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core bounded_construction_continuation`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core construction_outcome_composition`
- `cargo test --locked --manifest-path research/experiments/hand-bounded-first-use-construction-continuation/Cargo.toml`
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-bounded-first-use-construction-continuation-v1/campaign.toml`

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases include reflection, a delayed first matching competition, a held
candidate that loses to a higher-priority law, and a second stale competition
after consumption. Negative controls cover the exact integrated parent, ordinary
global stale evidence, wrong generation, dead, replaced, unrelated,
non-completing and sibling links, ambiguity, single-candidate non-consumption,
fresh child memory, exact replay, Production-equivalent protocol execution,
natural quiescence, zero propagation exhaustion, and bounded work. The hand arm
is killed if target eleven is not uniquely admitted at tick twenty-three, if the
position does not move from plus three to plus four, or if any integrity control
fails. Later lower-side behavior is retained but does not alter this first-wall
predicate.

## Risks and rollback

The main risk is unbounded stale influence. It is detected by the second-match,
losing-candidate, ambiguity, and ordinary-stale controls. Another risk is
consuming state during mere sampling or candidate construction; tests require an
executable multi-candidate choice with the exact completing generation. Snapshot
validation and same-version replay cover the new lifetime state; old binary
checkpoint migration is explicitly excluded. Rollback
removes the new protocol variant, lifetime state, consume event, successor
experiment, and campaign without touching the integrated parent.

## Open decisions

None.
