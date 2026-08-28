# Admit causal construction in the truthful hand

```text
                              return delivery
                         direct outcome   physical surface
                      +-----------------+-----------------+
 surface path absent  | frozen 15/16    | delivery only   |
                      | hand reference  | negative control|
                      +-----------------+-----------------+
 surface path present | symmetric links | complete causal |
                      | only control     | reentry candidate|
                      +-----------------+-----------------+
                                          |
                              closure -> child -> hand retry
```

## Outcome

Add a research-only successor experiment and eight-arm campaign that test whether the
already-surviving anonymous return-reentry conditions admit a causal owner in the
truthful reflected one-joint world. Separate the physical surface-to-outcome path from
delivery through the actual post-movement proprioceptive surface in a fixed 2x2
factorial. Run the complete cell first; if it constructs a child, test the child's
fresh owner-local boundary and re-evaluate the sixteen-step hand predicate plus the
fixed perturbation. Preserve a clean falsification at the earliest failed transition.
Do not change production physics, the prior hand experiment, frozen evidence,
Academy capability, adoption, or authority.

## Authority

- Path: `research/campaigns/developmental-hand-proprioception-decomposition-v1/convergence.toml`,
  `research/campaigns/recursive-learner-fresh-memory-v1/convergence.toml`,
  `research/programs/learner/program.toml`, and `research/programs/learner/lessons.toml`
- Revision: hand convergence SHA256
  `d14bd14ab06db6bc5c93d8861c15f0d199c57ede4d2aa8ee92e583f79b738e99`;
  frozen hand protocol SHA256
  `9ab7a51253c03f1dc1f4f9d1dabfcc5574e174bf9d77d5978467f16b2892bb61`;
  return-reentry artifact SHA256
  `08fe6a67c04a9aa5d35d348f3ba5b3ff4b547ada3d1837a2203eadca6c3f3e34`;
  return-reentry convergence SHA256
  `ff9925fd267a6e7d80c02a19e06047ed25859f16e19dd8536a40a3160de3cf76`;
  source revision `b94482267e96baffecc9576ddb6878918d9a4974`

## Model

Create `SurfacePath::{Absent, Symmetric}` and
`ReturnDelivery::{DirectOutcome, PhysicalSurface}`. Their product is
`AdmissionCell`; each factorial contrast changes exactly one typed axis.
`AdmissionEvidence` owns four `TrialOutcome`s and derives every arm result without
rerunning a cell.

The frozen cell is `Absent + DirectOutcome` and must reproduce the prior truthful-
recursive semantic trace: sixteen steps, fifteen movements, both signs, upper reach
and release, no lower reach, and zero constructions. The complete cell is
`Symmetric + PhysicalSurface`.

`Symmetric` adds a delay-three drive link from every anonymous position sensor to
both directional outcome surfaces. The links are deliberately symmetric: topology
contains no preferred direction, limit, desired pose, or answer. Only the outcome
surface with a live return from an actually emitted motor output can close that
output's causal path. `Absent` adds none.

`DirectOutcome` preserves the old delivery: after movement, the next consequence is
externally incident on the emitted motor's outcome surface with that outcome's
physical origin. `PhysicalSurface` instead stores the actual anonymous sensors active
after the movement and later drives those sensors with their own physical origins.
Those incidences can reach an outcome only through a `Symmetric` path. It never sends
an outcome, motor, direction, limit, target, success, or evaluator identity as the
origin.

```text
JointState
  -> deliver pending direct outcome or actual post-movement surfaces
  -> expose current anonymous position incidence and both motor opportunities
  -> Harness::send
  -> integrate emitted signed output and reflect the external joint
  -> remember only the physical surfaces active after actual movement
  -> next JointState | PhysicalStop
```

Use the frozen `RecursiveLearnerConstruction` protocol, capacities of 16,384
junctions and 65,536 links, initial position zero, reflected limits at minus/plus
four, sixteen primary steps, exact checkpoint replay, and natural quiescence.
`PhysicalStop` retains the prior exact junction-capacity, link-capacity, and warm-time
classification; unexpected panics remain software failures.

Run the complete cell before the three controls, then measure independent controls
concurrently within the campaign's declared parallel budget. Record every completed
step's world state, outputs, work, execution cost, topology, physical trace summary,
learners, replay, quiescence, and first stop.

Construction admission requires, in one bounded completed run, at least two accepted
causal-closure observations for one anonymous surface/output lineage, one
`LearnerConstructed` event, exact replay, natural quiescence, and no physical stop.
It does not require the joint to close. If construction fails, child freshness,
joint retry, and perturbation are inconclusive and remain unrun.

If a child forms, continue the same physical run and separately require the first
owner-bearing return to be admitted through actual participation, no consequential
owner read before that child's own consequence write, and no copied parent or sibling
preference signal. This is the fresh-boundary predicate, not a full-control claim.
Then evaluate the unchanged hand predicate: reach and leave both limits within the
same sixteen steps. Only a primary hand survivor runs `8 -> +4 -> 16`; the mirrored
`8 -> -4 -> 16` schedule remains held-out verification.

The factorial localization reports surface-path main effects, physical-delivery main
effects, their interaction, first trace divergence, and the earliest incomplete
transition among incidence, outcome traversal, return scheduling, origin admission,
reverse consolidation, closure, construction, fresh owner admission, private write,
private read, continuation, reversal, release, and physical cost.

## Invariants

- Do not modify `truelearner-core`, the prior hand experiment, prior campaigns,
  artifacts, results, convergence, receipts, program claims, or authority.
- The `Absent + DirectOutcome` cell must reproduce the frozen truthful-recursive
  trace before any new cell is interpreted.
- Change exactly one typed axis in each factorial contrast; capacities, sensor and
  motor geometry, initial state, ordinary timing, integration, reflection, horizon,
  learner protocol, and evaluator remain fixed.
- Symmetric paths connect every sensor to both outcome surfaces with the same delay,
  coupling, resistance, and mode. No code may choose a path from movement direction.
- Physical-surface delivery contains only junction identity already exposed by the
  world and that junction's truthful physical origin. It supplies no consequence or
  score to a surface that was not actually active after movement.
- Only an actually emitted motor output may leave a live return that a later surface
  can close. Correlated, disconnected, duplicate, shuffled-origin, and one-closure
  controls construct no child.
- A child construction is not hand success. Child freshness, hand closure, and
  perturbation recovery are separate conditional predicates.
- Execute exactly sixteen primary steps unless a declared physical stop occurs;
  never stop at success, extend failure, enlarge capacity, or repair a stopped world.
- Replay includes Harness plus external position and pending physical surfaces;
  compare histories and canonical checkpoint bytes but exclude wall-clock samples.
- Tests assert frozen predicates and total classifications, never force the candidate
  to survive.
- Keep the representative warm suite strictly under 10 seconds.

## Scope

- Add `research/experiments/developmental-hand-construction-admission/` as a separate
  Rust crate. It may depend on the prior hand and fresh-memory experiment crates for
  immutable integrity results but must own its new world and treatment types.
- Add protocol, campaign, eight arm manifests, artifacts, result envelopes, and
  convergence under
  `research/campaigns/developmental-hand-construction-admission-v1/`.
- Add candidate and independent verification receipts under `factory-artifacts/`.
- Exclude core or public-API changes, mutation of earlier experiments or evidence,
  adaptive topology, evaluator-selected construction, finger or later-hand rungs,
  Academy, benchmarks, downward ablations, adoption, and authority.

## Development style

TDD. First encode the two axes and frozen cell. Add the symmetric-topology and
physical-delivery controls, then typed stop/replay behavior. Add pure predicates for
construction admission, conditional fresh boundary, hand retry, perturbation gate,
and factorial localization. Unit tests accept any evidence-bearing scientific
classification. Finish with the CLI, campaign envelopes, convergence, and receipts.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml admission_cells_change_exactly_one_axis`
  checks the product model and common fixture.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml frozen_truthful_recursive_reference_is_exact`
  checks the immutable prior trace.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml symmetric_surface_paths_are_direction_blind`
  checks equal all-sensor-to-both-outcome topology.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml physical_delivery_uses_only_active_surface_origins`
  checks the semantic firewall and post-movement provenance.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml construction_and_dependent_gates_follow_frozen_predicates`
  checks total classification without asserting success.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml duplicate_disconnected_and_single_closure_controls_construct_none`
  checks causal construction boundaries.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml factorial_localizes_first_incomplete_transition`
  checks both main effects, interaction, prefix use, and transition ordering.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml fixed_perturbation_is_conditional_and_external`
  checks the no-consequence `8 -> +4 -> 16` gate.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml --lib -- --test-threads=1`
  runs the complete new suite from one shared measurement.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves all active core regressions.
- `cargo run --quiet --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml -- --all --output-dir research/campaigns/developmental-hand-construction-admission-v1/artifacts`
  emits the complete candidate first and all eight artifacts.
- `uv run research/validators/validate_campaign.py --file research/campaigns/developmental-hand-construction-admission-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/developmental-hand-construction-admission-v1/convergence.toml`
  validate frozen lineage and complete fan-in.
- `cargo fmt --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml --all-targets`,
  and `cargo clippy --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml --lib -- --test-threads=1`.
It must finish strictly under 10 seconds after cold bootstrap; candidate and
verification receipts independently record warm durations.

## Controls and evidence

Held-out cases are mirrored perturbation, a shuffled physical origin, replay from the
prefix immediately before first closure, and capacity exhaustion under physical-
surface delivery. Negative controls are the frozen hand trace, each one-axis cell,
one closure only, an exact duplicate return, a disconnected surface, the immutable
return-reentry artifact, inherited false-return/fresh-memory controls, fixed capacity,
reflection, quiescence, and runtime. Killing falsifiers are reference drift,
asymmetric or direction-selected paths, inactive or semantic surface delivery, false
construction, copied owner state, swallowed panic, replay inequality, over-budget
execution, attribution without matched contrasts, or advancing dependent gates after
construction failure. Expected evidence is eight artifacts, eight results, one
convergence record, and validated candidate and independent verification receipts.
No authority evidence is produced.

## Risks and rollback

The largest risk is that symmetric sensor paths themselves change ordinary incidence
before delayed physical delivery, making topology rather than reentry sufficient.
The path-only cell measures that effect and the factorial prevents misattribution.
Another risk is treating any learner count as proof of correct causality; require two
accepted closures on one live lineage and retain disconnected, duplicate, shuffled,
and single-closure controls. Physical-surface delivery may increase topology or work;
preserve fixed capacity and report the first stop instead of enlarging resources.
Parallel controls may vary wall time, so exclude time from replay while enforcing the
bound independently. Rollback removes only the new experiment, successor campaign,
and its receipts; all prior code and evidence remain byte-for-byte intact.

## Open decisions

None.
