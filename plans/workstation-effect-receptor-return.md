# Workstation effect-receptor return

```text
proprioception = position x velocity x effort x limits
                                 |          |
physical action return ----------+----------+
```

## Outcome

Add one research-only workstation arm that carries a changed action through the
velocity and effort factors of proprioception, not through every currently
active factor. This tests whether a real reversal at a body limit returns as the
same reversed arrow for a second step. It adds no held state, direction label,
target knowledge, production promotion, or contact claim.

## Authority

- Path: `language.md`; `lessons.md` lessons 35, 41, and 53; `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/intermediate-transition-contact-continuation.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2` plus frozen trace schema `workstation-intermediate-transition-contact-trace/v1`

## Model

Each axis's nine proprioceptive channels are a product: signed position, center,
signed velocity, directional effort, and limits. A just-applied action directly
changes velocity and effort. Sending every active factor as a transition maps
static positive position into the return and can select the old arrow. The
candidate defines one pure feature classifier for velocity/effort offsets and
uses it both at the next-step return and at the immediate sequential resample.
Other active factors remain ordinary samples with fresh sample origins.

## Invariants

- Only velocity and effort channels use effect-born transition incidence and
  stable receptor physical IDs in this arm.
- Position, center, limit, retina, contact, and opportunity inputs remain
  samples; their values are not guessed to have changed.
- Opposing directions still share one connected component and the learner alone
  chooses the output.
- Opportunity alignment, output-tick order, exact replay, natural quiescence,
  bounded work, and evaluator isolation remain intact.
- The whole-axis intermediate-transition arm remains as the negative control.
- Production topology, incidence, checkpoint bytes, and behavior remain
  unchanged.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, stored receptor deltas, production promotion,
  device-world changes, and contact authority.

## Development style

TDD: first prove the pure receptor-factor classification, preserve the two-step
forward continuation, and require the first upper-limit reversal to persist for
two outward steps before implementing the new research enum arm.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib effect_receptor_return_preserves_boundary_release` proves the reversed arrow persists and replay is exact.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation effect_transition_uses_only_velocity_and_effort_factors` proves the product-factor classifier.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation` preserves production behavior and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable` preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields` preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured budget is strictly under 10 seconds; the longer boundary fixture is
recorded separately and stopped at its first witness.

## Controls and evidence

The held-out cases are other axes and the later real surface trajectory.
Negative controls are the frozen whole-axis return chatter, static
position/limit channels as samples, GenericOnly production identity, exact
replay, natural quiescence, and the semantic firewall. The candidate is
falsified if the first decrease from the upper limit is followed by an increase,
static factors become transitions, forward continuation regresses, production
changes, or the warm suite is not under 10 seconds. Expected evidence is one
early-stopped boundary witness plus factory candidate and verification receipts.

## Risks and rollback

Excluding position and limit factors may remove useful consequences that require
true receptor-delta tracking later. That later need would justify explicit
receptor-state comparison, not marking the full axis. The research enum and pure
classifier can be removed without checkpoint migration or production changes.

## Open decisions

None.
