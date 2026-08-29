# Workstation causal delta return

```text
actual output outcome --physical movement--> changed proprioceptor
          |                                      |
          +------- one carried physical arrow ---+
```

## Outcome

Add one research-only workstation arm in which each changed active
proprioceptor is targeted by a transition carrying the physical identity of the
output-specific outcome that caused the movement. This tests whether the
existing input span can preserve the action-effect arrow through resampling,
without adding held paths, learner memory, desired direction, or hidden pose
knowledge.

## Authority

- Path: `language.md`; `lessons.md` lessons 0a, 0b, 35, 41, 43, 45, and 53;
  frozen schemas `workstation-intermediate-transition-contact-trace/v1` and
  `workstation-effect-receptor-contact-trace/v1`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

`PhysicalInput` already represents an arrow from `origin_physical` to a target
junction. The target remains the changed proprioceptor. For a real movement,
the origin becomes the existing output-specific outcome junction selected by
the axis's actual nonzero effort. Unchanged active proprioceptors remain sample
incidences with stable receptor origins.

The same pure source projection is used for next-step returns and the immediate
sequential resample. If the physical effort does not identify exactly one
output, the arm emits no causal transition for that axis instead of inventing a
cause.

## Invariants

- Every transition targets a changed active proprioceptor junction.
- Every transition origin is the existing outcome source of exactly one actual
  output for the same axis.
- Unchanged proprioceptors remain samples with stable receptor origins.
- External features and generic opportunities remain samples.
- No new checkpoint state, learner memory, held path, desired direction, or
  world-label input is added.
- Output choice, aligned opportunity, ordered effect integration, replay,
  natural quiet, work bounds, evaluator isolation, and production stay
  unchanged.
- Whole-axis, effect-only, and stable-receptor delta arms remain explicit
  negative controls.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, checkpoint changes, production promotion,
  device-world changes, and contact authority.

## Development style

TDD: add a two-step witness at the retained first wall. Require the second palm
depth step to continue the first output rather than choose a fresh opposite.
Only if that square commutes, test the first upper-boundary release and then the
existing keyboard/touchpad contact trajectory.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib causal_delta_carries_the_output_arrow_through_the_receptor`
  proves the first two-step square and exact replay.
- A bounded boundary fixture, added only after the first witness passes, proves
  that a real limit transition releases rather than chatters.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior and replay under the strict warm budget.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its budget is strictly under 10 seconds. Boundary and contact research runs are
recorded separately and stopped at their first falsifier.

## Controls and evidence

The held-out cases are upper-boundary release and the existing real
keyboard/touchpad contact trajectory. The first negative control is the
already-frozen stable-receptor delta failure at the second palm-depth step.
Further negative controls are whole-axis upper chatter, effect-only near-origin
oscillation, unchanged receptor bins, GenericOnly production, replay, natural
quiet, and the semantic firewall. Falsifiers are a fresh opposite at the second
step, an outcome origin from the wrong axis or direction, boundary chatter,
loss of quiet or replay, production change, or a warm suite at or above 10
seconds.

## Risks and rollback

An outcome origin could be too broad if several outputs caused one axis
movement. The exact-one-effort guard makes that case a sample rather than a
fabricated transition. The research enum arm and pure source projection can be
removed without checkpoint migration.

## Open decisions

None.
