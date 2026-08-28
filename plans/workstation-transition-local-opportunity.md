# Test transition-local opportunity at the workstation boundary

## Outcome

Add one research-only candidate that presents an actual returned changed axis as
one additional local motor-opportunity cause beside ordinary generic
exploration. Measure whether the existing learner continues the local depth
arrow into real workstation contact within 120 steps.

The production default remains Generic Workstation Opportunity V1. This is a
discovery candidate, not an adoption.

## Authority

- Path: `research/campaigns/workstation-transition-local-opportunity-v1/protocol.toml`
- Revision: protocol SHA-256
  `de8c3f9140ea2f3512693e236f41218a0440d2fac01a48c1d6cf15fc6d2b2071`
  at preregistration commit `cd13aed`

## Model

Keep the existing generic opportunity arrow unchanged. Under the research
feature only, add the sum:

`GenericOpportunity + ReturnedTransition(component)`

The returned-transition summand maps to both ordinary motor alternatives in
that one physical outcome component. It uses the same event-local phase and
physical origin as the transition return. The accepted learner chooses inside
the component and composes genuinely distinct causes across components.

The candidate has no new durable state. `pending_transitions` already retains
changed axes until their next-step return; local opportunity is derived during
that step and then disappears.

## Invariants

- Production `WorkstationHarness::new`, restore, state, checkpoint bytes,
  shared generic incidence, body, geometry, contact thresholds, and learner
  core remain unchanged.
- Research GenericOnly reproduces every existing parent and historical arm.
- Before any actual transition return, GenericOnly and LocalAfterTransition
  observations are exactly equal.
- Stable sensory sampling alone opens no local opportunity.
- Each returned changed axis adds exactly two inputs with one shared local
  phase and origin; no axis name, direction, target, reward, or device result
  chooses between them.
- Exact replay restores the complete research configuration explicitly.
- Evidence retains same-component and same-direction continuation, full contact
  projection, replay, quiescence, work, digit coactivation, and trace digest.

## Scope

- Extend research-only configuration in
  `truelearner/crates/workstation/src/harness.rs` and reexports.
- Thread explicit research restore configuration through
  `academy/crates/academy-workstation/src/session.rs`.
- Update existing research config construction with GenericOnly.
- Expose generic observation projection from
  `research/experiments/workstation-contact-contingency`.
- Add `research/experiments/workstation-transition-local-opportunity`.
- Exclude production behavior, core learner physics, persistent state,
  checkpoints, geometry, contact logic, image assets, and video.

## Development style

TDD. First add exact initial equality and post-return two-input-per-axis tests,
then implement the smallest feature-gated configuration and incidence mapping.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-transition-local-opportunity/Cargo.toml --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --features research workstation_world`
- `cargo test --locked --manifest-path research/experiments/workstation-digit-separation/Cargo.toml --lib`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
- `cargo clippy --all-targets --locked --manifest-path research/experiments/workstation-transition-local-opportunity/Cargo.toml -- -D warnings`
- `cargo run --quiet --locked --manifest-path research/experiments/workstation-transition-local-opportunity/Cargo.toml -- --output research/campaigns/workstation-transition-local-opportunity-v1/artifacts/transition-local-candidate.json`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`

It must remain strictly under 10 seconds. The full candidate and replay run are
recorded separately.

## Controls and evidence

- Held-out cases: first step without a return, first step after a return,
  multiple returned components, generic-only historical restore, and complete
  contact trace.
- Negative controls: immutable parent digest, unchanged sample, opposing local
  alternatives, no production diff, five-finger coactivation, device-event
  exclusion, replay, quiescence, and work below 2000.
- Falsifiers are frozen in the campaign protocol and arm.
- Evidence: validated plan, candidate and verification receipts, candidate
  summary JSON, campaign convergence, and exact observation-trace digest.

## Risks and rollback

- Reusing only the axis but not the actual transition cause would be semantic
  scheduling. Derive phase and origin from the same returned event.
- Duplicate generic and local inputs may create cancellation or excess work.
  The candidate is rejected on coactivation, no continuation, or work above
  2000.
- Research restore could accidentally fall back to GenericOnly. Require an
  explicit full config restore and exact replay.
- Roll back the feature-gated enum, config field, and experiment; production is
  unchanged throughout.

## Open decisions

None.
