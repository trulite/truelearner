```text
external hand step
      |
      +-- returned consequence phase -- work + exact comparison source
      |
      `-- current input phase --------- work + exact comparison source
                         |
                         `-- finite activity | waste | runaway
```

# Attribute downstream hand cost and make one activity decision

## Outcome

Add reusable, behavior-inert comparison attribution to core diagnostics and
retain the complete existing work/cost breakdown for every reflected-hand phase.
Freeze one successor campaign that explains the first positive step-four spike,
the largest step-eleven spike, and the full downstream excess, then classifies
the changed execution as finite useful activity, unexplained waste, or runaway.
No learner optimization or new behavior rule is part of this change.

## Authority

- Path: `research/campaigns/hand-prefix-and-activity-work-localization-v1/convergence.toml`
- Revision: `sha256:78cdbd2a8e769a5fcd3bc978f943d860636409073c2f43271b274fbfcb6ef34e`

## Model

Each external step is the ordered composition of an optional returned-consequence
phase and one current-input phase. Each phase maps its actual physical inputs to
outputs, structured `Work`, `ExecutionCost`, and trace. Evidence combines phase
measurements field by field; the sum must preserve the step measurement and all
step sums must preserve the run measurement.

Core comparison cost has two exhaustive sources: finding the minimum causal key
across scheduled firings and selecting the minimum firing inside the chosen time
bucket. Add one counter for each source and a total function whose sum must equal
the existing comparison counter. The experiment aligns external step numbers for
description only; after step three, parent and candidate phases are explicitly
different arrows because their physical inputs differ.

The final pure classifier returns `FiniteUsefulActivity` only when every major
positive spike is reconciled to comparison sources, receives more actual external
input or performs more actual physical work than its parent phase, produces the
new boundary interaction or learner construction, and remains naturally quiet.
It returns `Runaway` for non-quiescence or propagation exhaustion, and
`UnexplainedWaste` when positive work lacks those physical causes.

## Invariants

- Timing-wheel scheduling, ordering, mutation, outputs, and total comparisons are
  unchanged; the two attribution counters exactly partition the existing total.
- Phase evidence observes the same `Run` values already returned to the adapter;
  it does not send extra input, read the body again, or alter world effects.
- Per-phase structured work and cost preserve every public field, including queue,
  batch, frontier, arena, adjacency, allocation, bytes, and structural measures.
- Parent and candidate trajectories, total work, replay, quiescence, exhaustion,
  and immutable predecessor artifacts remain exact.
- Post-repair parent/candidate phases are never called equivalent computations.
- Evaluator position, contact, step labels, and the activity decision never enter
  learner execution.
- The predecessor's failed absolute cost gate remains frozen and the bounded law
  remains opt-in.

## Scope

- Add two comparison-attribution fields and one reconciliation method to core
  `ExecutionCost`; increment them beside the two existing timing-wheel increments.
- Add public phase, work, and execution-cost evidence projections to the existing
  developmental-hand adapter.
- Add focused partition and phase-conservation tests.
- Add `hand-downstream-work-class-localization` and one frozen four-arm diagnostic
  campaign covering controls, boundary-input activity, construction activity,
  unexplained-waste/runaway alternatives, adjudication, and convergence.
- Add factory candidate and verification receipts plus durable learner lessons.
- Exclude learner physics, cost total changes, force/adaptor behavior, optimization,
  lower-side behavior, cost-contract revision, adoption, and authority promotion.

## Development style

TDD. First add core tests that fail until the two comparison sources partition the
total, adapter tests that fail until phase sums preserve each step, and pure
classifier tests for all three decisions. Compile and freeze all manifests before
the one valid evidence run.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core timing_wheel_comparison_attribution`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml reflected_hand_phase_work_preserves_step_totals`
- `cargo test --locked --manifest-path research/experiments/hand-downstream-work-class-localization/Cargo.toml`
- `cargo clippy --locked --manifest-path research/experiments/hand-downstream-work-class-localization/Cargo.toml -- -D warnings`
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-downstream-work-class-localization-v1/campaign.toml`

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases are a timing-wheel bucket tie, a global minimum-key search, a hand
step with both phases, and the classifier's unexplained-waste and runaway paths.
Negative controls are immutable predecessor artifacts, exact live parent and
candidate summaries, accepted defaults, exact replay, natural quiescence, zero
exhaustion, and unchanged total comparison cost. Falsifiers are a comparison
partition mismatch, a phase/step/run conservation mismatch, an unexplained major
spike, hidden work by the lifetime law, or a changed execution. The artifact must
retain every phase, not only steps four and eleven.

## Risks and rollback

The main risk is mistaking correlation for attribution. The comparison partition
is exact; physical activity fields are retained separately and the final decision
requires both. Another risk is instrumentation perturbation, detected by frozen
summary and conservation controls. Rollback removes the two diagnostic counters,
phase projections, experiment, and campaign without touching learner state or the
predecessor evidence.

## Open decisions

None.
