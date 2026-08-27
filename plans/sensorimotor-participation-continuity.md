# Restore sensorimotor participation continuity

```text
actual path participates
        |
        +--> finite return --> local consequence
        |                         |
        `--> executable link <----+--> next candidate read --> admission
                                      |
                            same lineage on every
                            cooperating participant
```

## Outcome

Localize the exact write/read discontinuities behind recent-route and learned-pair
selection, correct the invalid zero-distance surface fixture, and apply only the
smallest participation-continuity repairs supported by those traces. Test the three
isolated predicates, the already surviving physical boundaries, and stage-gated
one-joint and embodied recompositions through public `Harness`. Keep all behavior
experimental; do not change default protocol authority.

## Authority

- Path: `research/constitution.md`, `research/programs/learner/lessons.toml`, `research/campaigns/sensorimotor-physical-boundaries-v1/convergence.toml`, and `research/campaigns/sensorimotor-participation-continuity-v1/protocol.toml`
- Revision: `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6`; parent protocol `68f6e6bfd2c1542a8052cdcb8bd6209308796eedaf3366f574401d5476c0e797`; successor protocol `da7cb7500f74fdd8fcdc2acc16c1c446a15d56551f12d74707a29cd9f3dc4d3a`

## Model

The relevant states are an executable drive link, an open finite return, a local
consequence record, and a ready output candidate. Public observations map links to
their bounded consequence state; optional physical tracing maps candidate evaluation
to admission. A solve may change only the transformation that carries an existing
participation record into the next candidate evaluation. Surface localization changes
only the research fixture from forbidden distance zero to path-forming distance one.
Embodied worlds map Harness outputs to physical motion and feed consequences back as
later Harness inputs; evaluator predicates remain outside the learner.

## Invariants

- Default protocols and all frozen parent classifications remain unchanged.
- The experimental learner contains no action, surface, route, coalition, axis,
  direction, body-part, target, score, episode, answer, or evaluator identity.
- Recent eligibility never mutates durable strength and expires physically.
- Coalition preservation arises only when every admitted member actually participated
  in the same bounded consequence event; untrained shared-origin fanout stays sparse.
- A surface claim requires a formed executable path, an opened finite return, and a
  delivered local update in that order.
- Duplicate, unrelated, and stale surface returns do not credit.
- Stable dormant topology remains outside active aging work.
- Checkpoint replay includes every state that affects outputs or cost.
- Tests and research fixtures call only public `Harness`; all runs quiesce naturally.
- Dependent body stages become inconclusive after an actual prerequisite failure.

## Scope

- Extend experimental observations/tracing and, only after localization, the smallest
  candidate selection or lineage transformation in `truelearner/crates/core/src/`.
- Add focused public-boundary regressions in
  `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/sensorimotor-participation-continuity/` and complete the
  corresponding campaign, convergence, receipts, and evidence-backed program lessons.
- Permit a small public protocol parameter on the existing sensorimotor research world
  only if needed to run stage-gated compositions through Harness.
- Exclude default adoption, Academy changes, benchmark changes, language or semantic
  controllers, and edits to frozen predecessor evidence.

## Development style

TDD. Add public Harness trace/observation assertions and the invalid-versus-valid
surface contrast first. Freeze each localization before implementing a solve. Add
isolated solve controls before any body composition.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  establishes public observations, default preservation, candidate selection, replay,
  lifetime, and quiescence.
- `cargo test --manifest-path research/experiments/sensorimotor-participation-continuity/Cargo.toml --lib`
  establishes all preregistered localizations, solves, controls, and stage gates.
- `cargo run --quiet --manifest-path research/experiments/sensorimotor-participation-continuity/Cargo.toml -- --all --output-dir research/campaigns/sensorimotor-participation-continuity-v1/artifacts`
  reproduces the immutable discovery artifacts.
- `uv run research/validators/validate_campaign.py --file research/campaigns/sensorimotor-participation-continuity-v1/campaign.toml`
  validates frozen arm lineage.
- `uv run research/validators/validate_convergence.py --file research/campaigns/sensorimotor-participation-continuity-v1/convergence.toml`
  validates complete fan-in.

## Development loop

The representative warm regression is
`cargo test --manifest-path research/experiments/sensorimotor-participation-continuity/Cargo.toml --lib`.
It must remain strictly under 10 seconds; record cold bootstrap separately.

## Controls and evidence

Held-out cases are incumbent ratios 1, 2, 4, and 8; expiry; distance-zero versus
distance-one path formation; one and two surfaces in both orders; duplicate,
unrelated, and stale returns; trained and untrained shared-origin pairs;
independent origins; either-alone output; delay 200; 1024 dormant outputs; reflected
joint limits; and stage-gated multimodal composition. Falsification is valid evidence.
Negative controls are unchanged default protocols, no-consequence release, the
distance-zero no-path fixture, duplicate and unrelated surfaces, stale delay, untrained
shared fanout, independent origins, either-alone output, and dormant stable topology.
Expected artifacts are one immutable arm artifact and result envelope per declared
arm, one convergence record, one candidate receipt, and one independent verification
receipt.

## Risks and rollback

The main risk is mistaking fixture repair for learner progress or allowing external
origin labels to become semantic IDs. The invalid fixture remains a negative control,
and origin reflection, untrained fanout, replay, default equality, and leakage audits
detect those failures. Rollback removes successor-only observation/selection changes
and the new experiment; the parent candidate and default protocols remain intact.

## Open decisions

None.
