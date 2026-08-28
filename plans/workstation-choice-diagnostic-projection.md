# Project workstation choice diagnostics

## Outcome

Expose a research-only, serializable projection of the core's already enabled
physical candidate, continuation, and choice events in each workstation step.
Use it to classify the first contact-relevant direction break in the falsified
transition-local candidate within 48 focused steps.

This is observation only. It makes no learner or body change and no capability
claim.

## Authority

- Path: `research/campaigns/workstation-return-bearing-choice-localization-v1/protocol.toml`
- Revision: protocol SHA-256
  `b297bb5d8a0dfd5aaa1d08fc995ae722bd414a46a46f2f27cbd317e47ecbbd0d`
  at preregistration commit `fda2087`

## Model

The core already emits a causally inert `PhysicalTransition` trace. Add a
research-only natural projection:

`PhysicalTransition -> ResearchChoiceDiagnostic`

Map only motor-junction targets to their existing anonymous `BodyControl` at
the body boundary. Retain candidate executability and drive, transition
continuation admission, and resolved choice targets and bases. Production
`WorkstationStepObservation` has no such field.

The isolated experiment joins prior movement, returned component, projected
choice, outward crossing, and resulting movement, then stops observation after
the first direction break.

## Invariants

- No core learner, production workstation field, checkpoint, state, work,
  output, movement, or session order changes.
- Diagnostic projection is compiled only with the research feature.
- Every projected control comes from the existing motor junction table; unknown
  nonmotor targets are skipped rather than guessed.
- Candidate and choice facts preserve tick and phase.
- Exact candidate replay includes identical projected diagnostics.
- The observer stop occurs after a naturally quiescent completed step and is
  not treated as organism quiescence or capability evidence.

## Scope

- Add feature-gated diagnostic types and projection in
  `truelearner/crates/workstation`.
- Add `research/experiments/workstation-return-bearing-choice-localization`.
- Exclude production API and serialization, core trace semantics, candidate
  ranking, opportunities, contact logic, geometry, and video.

## Development style

TDD. First require a research step to expose mapped candidate and resolved
choice diagnostics while production tests remain unchanged, then implement the
smallest projection.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-choice-localization/Cargo.toml --lib`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
- `cargo clippy --all-targets --locked --manifest-path research/experiments/workstation-return-bearing-choice-localization/Cargo.toml -- -D warnings`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`

It must remain strictly under 10 seconds. The focused diagnostic replay is
recorded separately.

## Controls and evidence

- Held-out cases: motor and nonmotor diagnostic targets, resolved choice with
  optional continuation target, same-direction return, first direction break,
  and exact replay.
- Negative controls: core diagnostic purity, production compile without the
  field, semantic firewall, no physics diff, natural quiescence, and bounded
  48-step observation.
- Falsifiers are frozen in the protocol.
- Evidence: plan, candidate and verification receipts, first-break JSON, and
  convergence.

## Risks and rollback

- Mapping junction identity to a semantic label inside the learner would be
  hidden authority. Map only after the run at the body observer boundary.
- Copying the full core event enum would couple schemas. Project only the three
  event families required by the frozen question.
- Roll back the feature-gated projection and isolated experiment; no physical
  state or production behavior changes.

## Open decisions

None.
