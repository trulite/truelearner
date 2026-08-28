```text
owned surface fires
        |
        v
retained path snapshot -> output-candidate evaluation -> existing choice/output trace
        |                              |
        `-------- diagnostics only ----'
                       |
                       v
          one immutable localization artifact
```

# Localize post-construction autonomous reuse

## Outcome

One unchanged reflected-hand run records enough causal detail to decide whether
the first post-construction stall is caused by missing reverse-path lifetime or
traversal, missing executable motor-candidate formation, ambiguous ownership, or
a missing owner-local consequence read. The change makes no behavioral claim and
must not alter outputs, learner state, work accounting, replay, or quiescence.

## Authority

- Path: `research/campaigns/hand-physical-boundary-member-novelty-v1/convergence.toml`
- Revision: `b94482267e96baffecc9576ddb6878918d9a4974`

## Model

The observed states are an owned surface incidence, its retained complete paths,
an output incidence, candidate ownership, projected drive versus threshold, the
owner-local consequence value, and the existing selection/output result.
Observation maps runtime state to trace events; the trace is consumed by the
hand adapter and a diagnostic experiment. Trace production is a one-way effect:
no trace field participates in selection or mutation. Absence remains explicit:
no path snapshot means no owned surface fired, no candidate event means no path
reached an output, and a non-executable candidate records why it was rejected.

## Invariants

- Physical tracing off produces exactly the pre-change state transitions and cost.
- Physical tracing on changes only diagnostic trace contents, never outputs or state.
- Every evaluated output incidence is recorded, including single and rejected candidates.
- Ownership distinguishes organism, exactly one learner, and ambiguous ownership.
- Path lifetime, traversal, candidate executability, and consequence lookup remain separate observations.
- The hand world, step schedule, limits, and success oracle remain unchanged.
- No anatomy, direction, desired pose, benchmark stage, or evaluator identity enters core physics.

## Scope

- Add diagnostic event types and observation in `truelearner/crates/core/src/trace.rs`,
  `path.rs`, `choose.rs`, and the public re-export in `core.rs`.
- Extend the existing hand adapter with per-step diagnostic summaries without
  changing its default serialized evidence schema.
- Add a diagnostic experiment and campaign for the frozen boundary candidate and
  matched temporal reference.
- Exclude path-retention, selection-reentry, ranking, lifetime, and benchmark behavior changes.

## Development style

TDD: add focused trace tests that assert the event chain and observational
equivalence before producing the campaign artifact.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml --test harness_boundary autonomous_reuse_diagnostics -- --nocapture`
  establishes path/candidate/read ordering and trace-on versus trace-off equality.
- `cargo test --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml`
  preserves the unchanged hand adapter and its replay controls.
- `cargo test --manifest-path research/experiments/hand-autonomous-reuse-localization/Cargo.toml`
  establishes deterministic one-pass localization and matched-reference equality.

## Development loop

`cargo test --manifest-path research/experiments/hand-autonomous-reuse-localization/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out case: the temporal reference must localize the same first break without
  inheriting boundary-novelty diagnostics.
- Negative controls: trace-on and trace-off state/output equality; an unowned
  surface; ambiguous ownership; a live but subthreshold output candidate.
- Falsifiers: diagnostics change behavior; cannot order retained path, traversal,
  candidate, ownership, and consequence lookup; or the earlier step-six
  consequence/consolidation disappears.
- Expected artifact: a frozen per-step transition table identifying the first
  absent transition, exact replay, quiescence, and cost.

## Risks and rollback

Tracing could accidentally recompute mutable decisions or increase non-tracing
work. Detect this with trace-on/off equality and unchanged work assertions. Roll
back by removing the new events, their emission, and adapter aggregation; no
persistent state format changes are permitted.

## Open decisions

None.
