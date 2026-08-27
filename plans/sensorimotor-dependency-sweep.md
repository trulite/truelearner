# Batch sensorimotor dependency falsification

```text
forecasted transitions
        |
        v
isolated public-Harness fixtures
        |
        +--> current transition survives
        `--> current sufficiency is falsified
        |
        v
one frozen result per arm -> one convergence
```

## Outcome

Add a research-only Rust experiment that runs every forecasted sensorimotor
dependency as an independent killing fixture in one batch. A failed arm rejects
the sufficiency of the current candidate physics for that transition; it does not
prove that the forecasted mechanism is uniquely necessary or adopt new organism
physics.

## Authority

- Path: `research/constitution.md`, `research/programs/learner/lessons.toml`, and `research/campaigns/sensorimotor-opportunity-v1/convergence.toml`
- Revision: `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6` plus the frozen predecessor campaign artifacts

## Model

`Dependency` names an observable transition. `Fixture` transforms neutral physical
inputs through the public `Harness` boundary into a `ProbeResult`. Every result is
one of survived, falsified, or inconclusive and carries observations, its killing
falsifier, replay equality, and natural quiescence. Each fixture constructs only
anonymous topology and supplies declared prerequisites through physical inputs;
the learner never receives body-part, action, target, score, or evaluator identity.

The independent probes cover causal credit, delayed lifetime, alternative exposure,
superseded closure, consequential continuation, no-consequence release, reversible
competition, sparse shared opportunity, coalition credit, multisurface alignment,
two-stage composition, scale selectivity, consolidated reuse, and the frozen
one-joint composition. The batch runner
is the only I/O boundary; probe transformations remain deterministic.

## Invariants

- Academy and research code call only `HarnessBuilder`, `Harness::send`,
  `Harness::read`, `Harness::save`, `Harness::restore`, and `Harness::advance_to`.
- Existing frozen campaign sources and artifacts remain byte-for-byte unchanged.
- A later fixture may supply prerequisites but may not encode the output it measures.
- Separately driven far outputs remain a control distinct from one shared fan-out.
- Every target event is replayed from a checkpoint and every run must quiesce naturally.
- Failed probes remain failed; no oracle, control, or predecessor result is weakened.

## Scope

- Add `research/experiments/sensorimotor-dependency-sweep/`.
- Add `research/campaigns/sensorimotor-dependency-sweep-v1/` manifests and results.
- Update the learner research program only with the neutral sweep claim and final
  executed limitations.
- Exclude changes to adopted protocol defaults, Academy curricula, benchmark code,
  frozen predecessor campaigns, and authority promotion.

## Development style

TDD: define one deterministic assertion for every preregistered probe, then implement
the smallest public-Harness fixture that makes the killing observation measurable.

## Focused tests

- `cargo test --manifest-path research/experiments/sensorimotor-dependency-sweep/Cargo.toml --lib` checks all dependency predicates and status classification.
- `cargo run --quiet --manifest-path research/experiments/sensorimotor-dependency-sweep/Cargo.toml -- --all --output-dir research/campaigns/sensorimotor-dependency-sweep-v1/artifacts` emits one exact artifact per arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/sensorimotor-dependency-sweep-v1/campaign.toml` checks preregistration structure.
- `uv run research/validators/validate_convergence.py --file research/campaigns/sensorimotor-dependency-sweep-v1/convergence.toml` checks complete fan-in after execution.

## Development loop

The representative warm regression suite is `cargo test --manifest-path research/experiments/sensorimotor-dependency-sweep/Cargo.toml --lib`; it must remain strictly under 10 seconds. Record cold dependency bootstrap separately.

## Controls and evidence

Held-out cases are left-right reflection, delayed return at 20 ticks, separate-drive
far-output independence, and single-route recall inside a larger body. Negative controls
are shuffled local credit, a shared fan-out that must not be confused with
independent drives, withholding consequence before release, and duplicate local
surface wiring. Falsifiers live in the frozen protocol and arm manifests. Evidence
is one JSON artifact and result envelope per arm plus an all-arm convergence record.

## Risks and rollback

A fixture can smuggle the answer by selecting a motor or translating evaluator state.
Detect this through topology review and the explicit authority audit. A broad fan-out
can also be mistaken for independent drives; preserve both fixtures and compare them.
Rollback removes only the new experiment and successor campaign because no default
physics or predecessor evidence is changed.

## Open decisions

None.
