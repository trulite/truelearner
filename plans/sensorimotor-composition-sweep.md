# Batch sensorimotor composition sweep

```text
frozen local references
         |
         v
closed-loop | sparse-wave | coalition | surface probes
         |          |           |          |
         +----------+-----------+----------+
                            |
                            v
              stage-gated body compositions
                            |
                            v
                   one complete convergence
```

## Outcome

Add a research-only Rust experiment that executes the complete second sensorimotor
prediction ladder through public `Harness` calls. It localizes the one-joint
composition failure, tests wave-scoped sparse allocation and fixed-active scaling,
tests learned coalitions and multisurface association, and records later embodied
compositions as survived, falsified, or inconclusive without changing adopted physics.

## Authority

- Path: `research/constitution.md`, `research/programs/learner/lessons.toml`, and `research/campaigns/sensorimotor-dependency-sweep-v1/convergence.toml`
- Revision: `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6` plus the exact dirty-tree candidate identified by the predecessor verification receipt

## Model

`Arm` identifies one physical transition. A deterministic fixture transforms anonymous
physical inputs through `Harness` into a `ProbeResult` containing survived, falsified,
or inconclusive status, observations, replay equality, and natural quiescence.
Fixtures may supply prerequisites by physical topology or training, but may not encode
the measured output. Composition arms consume actual predecessor outcomes and become
inconclusive when a required composition was falsified.

## Invariants

- No experiment calls `Body` or private learner modules.
- Frozen predecessor campaigns and artifacts remain unchanged.
- Current insufficiency does not establish unique necessity or adoption.
- Same-wave competition remains distinct from independent far-output drive.
- Fixed-active scaling adds dormant surface; it does not activate the added outputs.
- Multisurface success requires useful association, not duplicate strengthening alone.
- Every executed arm replays exactly and quiesces naturally.

## Scope

- Add `research/experiments/sensorimotor-composition-sweep/`.
- Add `research/campaigns/sensorimotor-composition-sweep-v1/`.
- Update learner program claims and lessons after convergence.
- Exclude adopted protocol changes, Academy curriculum changes, benchmark changes,
  frozen predecessor edits, and authority promotion.

## Development style

TDD: each preregistered arm receives a deterministic software test that validates
classification and integrity while preserving negative scientific outcomes.

## Focused tests

- `cargo test --manifest-path research/experiments/sensorimotor-composition-sweep/Cargo.toml --lib` validates every probe and classification.
- `cargo run --quiet --manifest-path research/experiments/sensorimotor-composition-sweep/Cargo.toml -- --all --output-dir research/campaigns/sensorimotor-composition-sweep-v1/artifacts` runs the frozen batch.
- `uv run research/validators/validate_campaign.py --file research/campaigns/sensorimotor-composition-sweep-v1/campaign.toml` validates preregistration.
- `uv run research/validators/validate_convergence.py --file research/campaigns/sensorimotor-composition-sweep-v1/convergence.toml` validates complete fan-in.

## Development loop

The representative warm regression suite is `cargo test --manifest-path research/experiments/sensorimotor-composition-sweep/Cargo.toml --lib`; it must remain strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

Held-out cases are strength ratios, reflected motor signs, causal delays, independent
waves, dormant surface sizes, shuffled sensors, reversed surface order, and unrelated
outputs. Negative controls preserve stale-credit rejection, far-output independence,
monocular disruption, either-output coalition worlds, replay, and natural quiescence.
Evidence is one immutable JSON artifact and result envelope per arm plus convergence.

## Risks and rollback

The main risk is a fixture that supplies the result rather than only a prerequisite.
Detect it by removing the claimed learner transition while retaining fixture topology
and checking that the positive predicate disappears. Another risk is interpreting
all-active scaling as dormant-surface cost; the fixed-active fixture forbids this.
Rollback removes only this successor experiment and campaign.

## Open decisions

None.
