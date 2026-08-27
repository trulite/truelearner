# Compose and test the complete sensorimotor candidate

```text
source fires -> local path -> signed admission -> motor output -> world changes
                   |                                |
                   `------ finite return <----------+
                                      |
surface participates -> reverse path consolidation -> later surface execution
```

## Outcome

Compose every relevant established sensorimotor lesson into one experimental
candidate, bound structural work to the active source neighborhood, and let an actual
surface consequence consolidate a reverse executable route through the same causal
return. Test the candidate upward from physical continuity through full-body control.
On failure, localize the first broken transition; run no removal ablation. On full
success, rerun the complete ladder under preregistered downward ablations. Keep all
behavior experimental and leave default protocol authority unchanged.

## Authority

- Path: `research/constitution.md`, `research/programs/learner/lessons.toml`, `research/campaigns/sensorimotor-participation-continuity-v1/convergence.toml`, and `research/campaigns/sensorimotor-synthesis-ladder-v1/forecast.md`
- Revision: `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6`; lessons digest `d9e8bb799e61fe704dc576e33c8a924f2ebc21e48b5990c20d8ded640146cb84`; parent convergence digest `f536bce2f6e4d4f21140e281f35e12512fd9d57255f0ed07907acef31f8846a1`

## Model

The states are unformed, locally formed, executable, participated, return-open,
consequence-associated, reverse-executable, recalled, motor-integrated, and
world-changed. Source-local topology maps an external firing to only nearby physical
outputs and maps a second path link back through its own adjacency. A surface firing
that actually reaches an open action return maps that causal pair to an executable
surface path with the action path's physical sign; unrelated or stale surfaces have
no such transformation. Current proprioceptive incidence and learned signed drive
compose before motor threshold evaluation. Harness maps emitted outputs to world
motion and later physical consequences back to inputs; evaluation remains outside the
learner.

## Invariants

- Default protocols and every frozen parent classification remain unchanged.
- The learner contains no action, surface, axis, digit, eye, ear, voice, body-part,
  target, score, episode, answer, or evaluator identity.
- Structural lookup follows the active source and physical position; it does not use
  an evaluator-selected active set or semantic fovea.
- Local indexing preserves the exact formed path set, output sequence, replay, and
  quiescence of its global reference.
- Reverse consolidation requires an actual surface firing, an open finite return, and
  the executable action path that created that return; duplicate, unrelated, and
  stale surfaces cannot consolidate or credit.
- Motor opportunity changes timing eligibility only; learned signed drive still
  determines direction and crossing the physical threshold remains necessary.
- Tests and research call only public `Harness`; body motion and consequences occur
  outside the learner.
- The upward ladder stops after its first failed prerequisite. Downward ablations run
  only after the exact complete candidate passes the full ladder.

## Scope

- Add source-local output and path indexes/helpers in `truelearner/crates/core/src/`
  and include every behavior-affecting index in checkpoint reconstruction.
- Add the smallest candidate-only reverse-consolidation and motor-opportunity
  transformations plus public physical trace observations.
- Add focused Harness regressions in
  `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/sensorimotor-synthesis-ladder/` and the matching campaign,
  arm manifests, artifacts, results, convergence, and factory receipts.
- Permit timing corrections in the external sensorimotor world when they represent
  anonymous current proprioception rather than evaluator action selection.
- Exclude default adoption, Academy or benchmark changes, semantic controllers,
  language, authority promotion, and edits to frozen evidence.

## Development style

TDD. First add Harness tests for exact local structural equality, reverse execution,
current-arrival motor integration, unrelated and stale controls, checkpoint replay,
and natural quiescence. Then implement the smallest local transformations. Build the
upward experiment only after those public-boundary tests pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  establishes public-boundary locality, reverse execution, motor integration,
  unchanged defaults, replay, and quiescence.
- `cargo test --manifest-path research/experiments/sensorimotor-synthesis-ladder/Cargo.toml --lib`
  establishes the preregistered synthesis reference, upward gates, conditional stop,
  and conditional-ablation policy.
- `cargo run --quiet --manifest-path research/experiments/sensorimotor-synthesis-ladder/Cargo.toml -- --all --output-dir research/campaigns/sensorimotor-synthesis-ladder-v1/artifacts`
  reproduces one immutable artifact per arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/sensorimotor-synthesis-ladder-v1/campaign.toml`
  validates the campaign and arm lineage.
- `uv run research/validators/validate_convergence.py --file research/campaigns/sensorimotor-synthesis-ladder-v1/convergence.toml`
  validates complete fan-in after execution.

## Development loop

The representative warm regression is
`cargo test --manifest-path research/experiments/sensorimotor-synthesis-ladder/Cargo.toml --lib`.
Its measured duration must remain strictly under 10 seconds; cold compilation is
recorded separately.

## Controls and evidence

Held-out cases are 4, 64, and 1024 dormant outputs; two useful surfaces in both
orders; duplicate, unrelated, and delay-200 surfaces; current versus one-tick-shifted
proprioception; incumbent ratios 1, 2, 4, and 8; trained and untrained shared-origin
pairs; reflected joint limits; repeated axes; ten digits; binocular disparity;
delayed acoustic return; and full multimodal composition. Negative controls are all
unchanged protocols, global structural reference equality, no-consequence release,
untrained fanout, unrelated and stale return, either-member-alone output, and no
ablation before full success. Expected evidence is one immutable artifact and result
per arm, one convergence, one candidate receipt, and one independent verification
receipt.

## Risks and rollback

The main risks are hiding semantic association in physical origin, treating a fixture
timing correction as learned capability, or optimizing away a valid path. Origin
reflection, unrelated and stale controls, shifted-timing controls, exact path/output
equality, replay, frozen references, and leakage audits detect these failures.
Rollback removes successor-only indexes and candidate transformations while leaving
the parent experimental and default protocols intact.

## Open decisions

None.
