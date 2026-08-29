# Workstation quiescent sequential composition

```text
returned output moments -> integrate body -> resample body features
                                                   |
current output moments  -> integrate body <---------+
```

## Outcome

Add one research-only workstation arm that preserves the order of two naturally
quiescent learner runs. It must apply returned-run outputs before forming the
current body observation, then apply current-run outputs separately. This tests
the first non-commuting square found in the frozen opposition trace; it does not
promote the adapter or establish a hand capability.

## Authority

- Path: `lessons.md` lesson 39, `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/output-specific-opposition-trace.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2` plus the frozen uncommitted trace whose schema is `workstation-output-specific-opposition-trace/v1`

## Model

`WorkstationState` is the world object. Outward outputs sharing one physical
tick form one actuator moment, and integration maps that moment to the next
state. A naturally quiescent core `Run` may contain several such ordered
moments. The current batched arm merges all moments from both runs into one
frame, which is not ordered composition. The candidate uses ordinary Rust
control flow to integrate returned output moments in tick order, recompute
`sensory_features` from that intermediate state, run current inputs, and
integrate current output moments in tick order. An empty output sequence is an
identity arrow. Invalid outward physical IDs remain typed
`WorkstationError::UnknownOutput` failures. `Output` does not retain phase, so
this arm makes no finer ordering claim within one tick.

## Invariants

- Every outward output is integrated exactly once and physical ticks retain
  their trace order.
- Current proprioception is formed from the state after the returned effect.
- Movements retain output-tick order; effects from different ticks are not
  collapsed into one movement with both efforts.
- Pending returns describe changed outputs from the final current-input phase,
  because the returned phase is already observed within this composition.
- Output-specific physical-transition incidence remains the only return source.
- Production construction, batched integration, checkpoint bytes, and replay
  remain unchanged.
- No anatomy goal, desired direction, target, surface, or evaluator fact enters
  the learner.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner rules, production promotion, Academy authority claims,
  world/device substep effects, and video evidence.

## Development style

TDD: first express the frozen failure as a focused negative control and the
sequential ordering law as a candidate test, then add the smallest research enum
arm and integration helper needed to pass it.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib` proves the frozen batched opposition still occurs and the sequential arm preserves ordered physical arrows.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation` preserves production behavior and exact replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable` preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields` preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured development budget is strictly under 10 seconds; cold bootstrap is
recorded separately.

## Controls and evidence

The held-out case is the first later opposing choice after a correctly admitted
current transition on another axis. Negative controls are the existing batched
output-specific arm, GenericOnly production identity, exact replay, natural
quiescence, and the semantic firewall. The candidate is falsified if it still
creates a single movement containing opposing efforts, fails to expose the
intermediate body state to current proprioception, changes production replay, or
exceeds the warm budget. Evidence is one bounded test result plus factory
candidate and independent verification receipts; the frozen trace is reused and
not regenerated.

## Risks and rollback

Sequential body integration can expose an intermediate body state while the
external contact sample remains from the start of the Academy step. Therefore
this arm diagnoses body/proprioceptive composition only and cannot support a
device-contact claim. The new research enum arm and helper can be removed without
checkpoint migration or production changes.

## Open decisions

None.
