# Workstation intermediate transition incidence

```text
returned output -> body changes -> resampled proprioception -> current choice
                     |                    |
                     +--- Transition -----+
```

## Outcome

Add one research-only workstation arm that carries an actual body change from a
returned output into the immediately resampled proprioceptive inputs as
`PhysicalIncidence::Transition`. This tests whether the unchanged learner then
continues that current arrow instead of selecting a fresh opposite. It does not
hide opportunity, prefer a direction, promote production, or establish contact.

## Authority

- Path: `language.md`; `lessons.md` lessons 35, 41, and 53; `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/aligned-contact-trace.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2` plus frozen trace schema `workstation-aligned-contact-trace/v1`

## Model

The intermediate `WorkstationState` is the object after returned output moments
are integrated. A changed `BodyMovement` is the physical arrow. Resampling maps
that state to ordinary sensor inputs; for proprioceptive features belonging to a
changed axis, the map must preserve the arrow as `PhysicalIncidence::Transition`.
It must also preserve the existing receptor physical ID rather than minting a
fresh external-sample origin. All external light/contact features and motor
opportunities remain `Sample`.
The mixed current input envelope is sent through the existing physical boundary,
where the accepted output laws may reuse an owned consequential path or resolve a
current-transition competition without adapter intervention.

## Invariants

- Only an axis with `BodyMovement.changed == true` in the returned-output phase
  marks its resampled proprioceptive inputs as transitions.
- A transitioned proprioceptive input uses that receptor's existing physical
  identity; it does not receive a fresh sample origin.
- External retina, contact, unchanged proprioception, and every opportunity stay
  samples.
- Direction is derived only from the physical sensor values and existing paths;
  the adapter supplies no direction or target label.
- Each input is sent once, output ticks retain sequential integration, and the
  aligned shared opportunity retains unit strength, origin, tick, and phase.
- The aligned parent remains a negative control whose palm-depth arrows compose
  back to identity.
- Production input incidence, topology, checkpoints, and behavior remain
  unchanged.
- Exact replay, natural quiescence, bounded work, and evaluator isolation remain
  required.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, production promotion, device-world substeps,
  contact authority, and video evidence.

## Development style

TDD: freeze the parent tick-six/tick-nine palm-depth reversal, require the new
arm to preserve the same physical arrow and make non-identity depth progress, then
add only the research enum and mixed-incidence current input envelope.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib intermediate_proprioception_keeps_the_current_physical_arrow` proves local continuation and exact replay.
- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib aligned_generic_opportunity_starts_a_silent_component` preserves the aligned parent.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation` preserves production behavior and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable` preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields` preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured budget is strictly under 10 seconds; cold bootstrap is recorded
separately.

## Controls and evidence

The held-out cases are other axes changed by the returned phase and later
opportunity waves. Negative controls are the aligned parent reversal, unchanged
proprioception as samples, external features as samples, GenericOnly production
identity, exact replay, natural quiescence, and the semantic firewall. The arm
is falsified if palm depth still selects a fresh opposite or composes to identity
at the first reversal, any unchanged/external input becomes a
transition, production changes, or the warm suite is not under 10 seconds.
Expected evidence is one bounded focused result and factory candidate and
verification receipts.

## Risks and rollback

Several axes may truthfully change in one returned phase, increasing current
transition activity. The component product and output-choice trace must still
show one local choice per component and bounded quiet execution. The research
enum and incidence mapping can be removed without checkpoint migration or
production changes.

## Open decisions

None.
