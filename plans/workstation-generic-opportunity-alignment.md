# Workstation generic opportunity alignment

```text
sensor input --------> balanced motor paths --\
                                               +--> one local choice --> output
one shared opportunity (same tick and phase) -/
```

## Outcome

Add one research-only workstation arm in which the existing anonymous shared
opportunity meets the balanced sensor paths at the motor choice. This tests
whether previously silent body components can form a first output without
adding strength, direction, target, surface, or evaluator knowledge. It composes
with the already tested sequential-effect arm but does not promote either arm or
establish contact capability.

## Authority

- Path: `language.md`; `lessons.md` lessons 54, 55, 56, and 57; `research/experiments/developmental-hand-multi-joint/src/lib.rs`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2` plus `workstation-sequential-opposition-trace/v1`

## Model

In the core language, balanced sensor links meet at each motor junction but
cannot choose a path without opportunity. The accepted multi-joint harness sends
one generic opportunity at `tick + 2`, phase zero, where those paths arrive. The
workstation currently sends the same unit at `tick + 1`, phase 20,000, so the two
arrows never compose. The candidate adds a research enum arm whose ordinary
opportunity coordinates match the accepted timing. All output selection remains
inside the unchanged connected-component learner law. Invalid inputs and outputs
retain their existing typed failures.

## Invariants

- Opportunity remains one unit, one physical origin, and one phase for all
  controls; it is not copied into direction-specific facts.
- Opposing controls in one axis remain in one connected outcome component and
  still compete normally.
- Disconnected body components retain the accepted product composition.
- Sequential output-tick integration, physical-transition incidence, exact
  replay, natural quiescence, and bounded work remain intact.
- The parent sequential arm remains a negative control with silent palm depth.
- Production timing, topology, checkpoints, and behavior remain unchanged.
- No desired direction, target, keyboard geometry, contact state, or evaluator
  fact enters the learner.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, production promotion, contact authority, and
  device-world changes.

## Development style

TDD: freeze the parent palm-depth silence, require the aligned arm to make one
executable palm-depth choice or outward crossing, then add only the research enum
and opportunity-coordinate transformation.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib aligned_generic_opportunity_starts_a_silent_component` proves the local timing composition and exact replay.
- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib sequential_world_effects_preserve_two_ordered_arrows` preserves the parent solve.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation` preserves production behavior and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable` preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields` preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured budget is strictly under 10 seconds; cold bootstrap is recorded
separately.

## Controls and evidence

The held-out cases are the other previously silent axes and later return-bearing
choices. Negative controls are the unaligned sequential parent, GenericOnly
production identity, exact replay, natural quiescence, and the semantic firewall.
The candidate is falsified if palm depth remains non-executable, opportunity
strength or direction differs between opponents, the parent changes, production
replay changes, or the warm suite is not under 10 seconds. Expected evidence is
one bounded focused result and factory candidate and verification receipts.

## Risks and rollback

Correctly aligned opportunity may reveal excessive simultaneous exploration or
a later trajectory-coherence failure. Those are later trace walls, not reasons
to alter this timing oracle. The new research enum arm and two coordinate
methods can be removed without checkpoint migration or production changes.

## Open decisions

None.
