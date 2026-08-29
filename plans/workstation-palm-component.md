# Workstation palm component

```text
palm horizontal outcome --\
palm vertical outcome ----- shared palm junction -- one local choice
palm depth outcome --------/
```

## Outcome

Add one research-only workstation arm that connects the existing horizontal,
vertical, and depth outcome components of the palm through one shared physical
junction. This tests whether ordinary connected-component competition composes
the three axes into reachable palm poses without a target, planner, visual
label, or new choice law.

## Authority

- Path: `language.md`; `lessons.md` lessons 0a, 0b, 29, 54, 55, 56, and 57;
  retained causal-delta two-step, boundary, and 120-step corner-cycle witnesses
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

The three palm translation axes are distinct arrows on one physical palm. The
current morphology gives each axis a separate anchor, so connected-component
composition treats them as three independent causal worlds. The candidate
reuses one anchor junction for the sensors and output outcomes of all three
palm translation axes. Existing connected-component output and opportunity
factorization then makes alternatives compete within the palm while preserving
composition with eyes, wrist, digits, and other disconnected physical parts.

## Invariants

- Palm horizontal, vertical, and depth outcomes and proprioceptors share one
  existing-kind anchor junction only in the research arm.
- Their motor outputs, outcome sources, proprioceptors, positions, and world
  effects remain distinct.
- No target, surface rectangle, contact depth, desired direction, pose memory,
  visual label, or new core choice rule is introduced.
- Causal-delta action-effect arrows, ordered effect integration, aligned generic
  opportunity, replay, natural quiet, evaluator isolation, and production stay
  unchanged.
- The uncoupled causal-delta corner cycle remains a negative control.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner changes, checkpoint changes, production promotion,
  changes to keyboard/touchpad/monitor geometry, and authority claims.

## Development style

TDD: first require one external step to emit at most one changed palm translation
axis. Then run the retained contact trajectory and stop at either the first real
pressure sample or the first repeated pose cycle.

## Focused tests

- A one-step palm-component fixture proves that the three palm arrows compete
  locally instead of all firing as disconnected products.
- A bounded 120-step fixture tests the first real keyboard/touchpad pressure
  sample and preserves the compact pose trajectory at failure.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves production behavior and replay under the strict warm budget.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core physical_diagnostics_are_opt_in_pure_and_replayable`
  preserves diagnostic purity.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its budget is strictly under 10 seconds. The research contact fixture is
recorded separately and stopped at its first falsifier.

## Controls and evidence

The held-out case is the real keyboard/touchpad contact surface. The primary
negative control is the uncoupled causal-delta arm's exact repeating corner
cycle. Further negative controls are non-palm component independence,
GenericOnly production, replay, natural quiet, and the semantic firewall.
Falsifiers are multiple palm translation axes changing in the first arm step,
no real contact before a repeated pose cycle, loss of a causal-delta
action-effect arrow, production change, or a warm suite at or above 10 seconds.
Expected evidence is the focused morphology witness, the stopped contact result,
and factory candidate and verification receipts.

## Risks and rollback

A shared palm anchor may serialize translations too strongly or choose a cycle
that still misses the surface. That is a clean morphology falsifier, not a
reason to add target knowledge. The research arm and shared-anchor construction
can be removed without checkpoint migration.

## Open decisions

None.
