# Workstation output-specific return source

```text
decrease output -> decrease outcome --\
                                      +-> one connected axis component
increase output -> increase outcome --/
          actual changed effort -> return only its exact outcome
```

## Outcome

Add one research-only workstation morphology in which opposing controls retain
distinct outcome sources while remaining in one connected axis component. An
actual uniquely caused body change returns through that exact source. This is
development evidence only; production and accepted learner physics stay fixed.

## Authority

- Path: `arch.md`, `research/campaigns/workstation-return-bearing-opportunity-composition-v1/convergence.toml`
- Revision: `dfe9338`

## Model

`BodyControl` is the tagged output alternative, `BodyMovement` is the physical
effect, and `Sites::outcomes` is the return codomain. Production retains the
current axis quotient. The candidate builds a coproduct of two outcome sources
per axis, connects both through the existing axis anchor, and uses a total
`unique_effect_control` transformation from persisted proprioceptive effort to
an optional `BodyControl`. `None` covers unchanged, absent, or ambiguous causes.
The actual changed proprioceptive receptors carry `PhysicalTransition` incidence
through their ordinary links to that control's distinct outcome source; the
outcome then returns along the used path. Only the boundary sends effects;
diagnostics remain observer-only.

## Invariants

- Opposing outcomes stay in one connected outcome component.
- Only an actual changed axis with exactly one nonzero directional effort gets
  an output-specific transition return.
- Ambiguous opposing effort creates no exact return; no direction is guessed.
- Equal local opportunity treats both alternatives identically.
- A transition enters through the actual changed proprioceptive receptors; it
  is never injected directly into an outcome junction.
- GenericOnly production topology, checkpoint bytes, behavior, and replay stay
  unchanged.
- Candidate save/restore preserves the exact next return through existing
  `WorkstationState` effort; no additional held state is added.
- No anatomy, target, surface, desired direction, reward, or evaluator fact
  enters the learner.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-output-specific-return-source/`
- candidate and verification receipts
- Excludes core learner changes, production promotion, contact claims, and
  checkpoint format changes.

## Development style

TDD: add topology, unique/ambiguous cause, and exact replay fixtures before the
bounded candidate runner.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-output-specific-return-source/Cargo.toml --lib`
  proves one exact returned arrow or kills the candidate.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior and checkpoint replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Budget: strictly under 10 seconds; cold bootstrap recorded separately.

## Controls and evidence

Held-out ambiguity: both efforts nonzero returns no exact source. Negative control
cases: unchanged sample, unrelated axis, GenericOnly identity, exact replay,
natural quiescence, and no broad five-finger coactivation. Killing falsifier:
the exact prior output candidate still lacks a live path/current-transition
admission. Expected artifacts are a factory candidate receipt, independent
verification receipt, and one research preflight or bounded result.

## Risks and rollback

Distinct sources could accidentally split one axis into two competition
components or change production topology. Topology and production identity
tests detect both. The feature-only enum arm and candidate experiment can be
removed without migrating production checkpoints.

## Open decisions

None.
