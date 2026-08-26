# Integrate actuator effort and return truthful proprioception

```text
outward crossings -> opposing effort per body axis -> one net state change
                                                        |
                                                        v
next input <- position + velocity + effort + limits <- body state
                                                        |
                                                        v
                                     Academy evaluates net change only
```

## Outcome

Make `HumanHarness` treat paired decrease/increase outputs as opposing physical
effort on one body axis. The body applies their bounded net impulse once and
returns signed position, signed velocity, separate opposing effort, and joint
limit receptors as ordinary input on the next step. Equal opposing effort is
observable as effort but produces no movement or movement outcome.

Make Body Discovery derive movement evidence only from the resulting net body
change. This supports truthful visual-touch body experience; it does not add an
action selector, preference, reward, semantic control, or learner law.

## Authority

- Path: `arch.md` Accepted law, Accepted body, and Boundaries; `LANGUAGE.md`;
  `algo.md`; `academy.md` Body discovery and Evidence rules;
  `plans/human-shaped-harness.md`; `plans/body-discovery-academy.md`
- Revision: parent commit `766c39c1e527fbe29c7d7445e1d6902b2de5334e`;
  `arch.md` SHA-256
  `f5b1157b8d0479cb9575980f148090932043ce7d8c9da72d70fdb887cb4b4963`;
  `academy.md` working-tree SHA-256
  `6420f1e7c7e7ce41f923c330f83c2e057e0c2cd2ca894f59a94326f30ff68dc4`;
  prerequisite plan SHA-256 values
  `7e4797714f79ab7ad5601b40a4c0e5c4ef9bf9df093cba6bb008836739c47137`
  and
  `88bc8336181ec59e468996fc82c68bd154404b992bde9299db17ceb748474b5d`

## Model

- `BodyAxis` identifies the 25 independent physical degrees of freedom: three
  eye axes and eleven axes for each hand, including all five digits. It contains
  no task or action meaning.
- `ActuatorFrame` folds all outward crossings in one harness step into opposing
  decrease and increase effort for each axis. Folding order is irrelevant;
  effort is bounded and the net impulse is `increase - decrease`.
- `HumanState::integrate` is a pure transformation from the prior body state and
  one actuator frame to the next state plus one `BodyMovement` per active axis.
  Each axis changes at most once per step. Velocity is the actual signed delta
  after bounds, not the requested impulse.
- Persistent body dynamics retain the last actual velocity and both effort
  magnitudes for every axis. Position and limit state are derived from the pose,
  so contradictory copies cannot exist. Checkpoints contain the complete pose
  and dynamics.
- `AxisProprioception` is an owned read of axis, signed position relative to its
  neutral pose, signed velocity, separate opposing efforts, and lower/upper
  limit contact. `HumanState::proprioception` returns all axes in fixed order.
- The sensory projection composes retina, touch, then fixed-capacity receptor
  channels. Every signed position and velocity uses separate negative and
  positive magnitude channels; effort and limits also use separate channels.
  These anonymous values enter only through the public core `Harness`.
- `HumanHarness::step` decodes outward crossings into an actuator frame,
  integrates once, and sets a pending physical outcome only when pose changed.
  Effort without movement remains sensory information but is not reinterpreted
  as success.
- Body Discovery evaluates gaze, hand, digit, and contact capability from
  `BodyMovement.changed`, where `changed` now means a nonzero net pose change.
  Evaluator names and verdicts remain outside every `WorldSample`.

## Invariants

- The learner continues to receive anonymous physical input and emit anonymous
  outward crossings through the unchanged public core `Harness`.
- The body never selects, suppresses, or remaps a learner output. It combines
  opposing outputs only by the same fixed signed-force law on every axis.
- Crossing order cannot change effort, net impulse, pose, velocity, or outcome.
- Equal opposing effort preserves pose, reports zero velocity and both efforts,
  and cannot be credited by Academy as movement.
- A net impulse changes only its owned axis, saturates without wraparound, and
  reports the actual bounded delta as velocity.
- Proprioception covers both eyes, both palms, both wrists, both force/spread/
  opposition axes, and all ten fingers in a deterministic fixed capacity.
- Proprioceptive sign is preserved; positions on opposite sides of neutral and
  velocities in opposite directions cannot share a receptor channel.
- Reading remains inert. Save and restore preserve the exact next sensory
  projection, crossings, integrated state, effort, velocity, outcome, and work.
- Academy sees owned observations only and never sends an axis, direction,
  effort, capability, expected result, or correctness field to the learner.
- Core physics and accepted authority remain unchanged. This is development
  body and curriculum evidence only.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change `truelearner/crates/human/src/state.rs`, `harness.rs`, `checkpoint.rs`,
  and public exports to add axis integration, persisted dynamics, and owned
  proprioception.
- Update `truelearner/crates/human/tests/human_harness.rs` and unit laws for
  cancellation, order invariance, sign, bounds, complete receptor coverage,
  transactional failure, and exact restart.
- Change `academy/crates/academy-body/src/course.rs` and its tests so capability
  evidence uses net physical movement. Preserve the first resulting failure.
- Update the human crate contract and Academy Body Discovery prose where they
  describe body-position input or movement evidence.

Exclude changes to `truelearner-core`, path formation, path choice, strength,
outcome return, learner thresholds, Academy scheduling, world targets, semantic
actions, hidden arbitration, preferences, reward, accepted-body authority,
voice, or ears.

## Development style

TDD. Add failing pure integration and Academy cancellation tests first, then
implement axis types, dynamics, sensory projection, checkpoint preservation,
and the smallest evaluator adaptation.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`
  establishes signed integration, cancellation, order invariance, bounds, and
  receptor-channel separation.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
  establishes public proprioception coverage, next-step consequences,
  transactionality, and exact checkpoint replay.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes that canceled effort is not credited as movement.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes probe isolation, leakage controls, first-failure preservation,
  and exact replay with the corrected body.
- `cargo test --locked --manifest-path truelearner/Cargo.toml --workspace`
  and `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 -p academy-body --lib`
  establish core and Academy compatibility.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check` and
  `cargo fmt --all --manifest-path academy/Cargo.toml -- --check` establish
  canonical formatting.
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
  and `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`
  establish warning-free workspaces.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib && cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured warm budget is strictly under 10 seconds. Record cold bootstrap
separately; course execution and evidence serialization are not warm regression
time or physical time.

## Controls and evidence

- Held-out cases: unequal multi-impulse effort, reversed crossing order, neutral
  and non-neutral starting poses, movement into both bounds, an unused digit,
  mirrored side/axis cases, and a disjoint course seed.
- Negative controls: equal opposing effort changes no pose and earns no movement
  outcome; zero effort resets effort and velocity without moving pose; reading
  changes no checkpoint; an invalid sample changes neither pose nor dynamics;
  evaluator-only fields remain absent from serialized samples.
- Laws: actuator-frame fold is associative and commutative up to bounded sums;
  zero frame is identity on pose; disjoint axes commute; derived position agrees
  with pose; actual velocity equals next minus prior position; save/restore is
  identity on the next complete step.
- Falsifiers: crossing order affects motion; equal opposing effort moves or is
  credited; sign is lost; a finger lacks position input; effort is treated as
  correctness; Academy injects a control; checkpoint replay differs; core tests
  change; or the warm suite reaches 10 seconds.
- Evidence: validated plan, focused tests, preserved failing course evidence,
  exact replay, candidate receipt tied to the complete dirty-tree scope, and an
  independent verification receipt with a fresh-seed control.
- Not applicable because no official benchmark is run: no external scorecard
  or accepted authority verdict is produced.

## Risks and rollback

- More receptor channels increase body size and work. Keep a fixed declared
  capacity, measure physical work, and reject an unbounded or non-quiescent run.
- A motor combiner could become hidden action selection. Use only signed bounded
  addition per axis and test order invariance; add no winner, priority, or goal.
- Effort can be confused with reward. Emit it only as ordinary next-step sensory
  input and return a movement outcome only for actual pose change.
- Checkpoint layout changes can omit dynamics. Validate the complete private
  envelope and exact next-step replay; no public persistence compatibility is
  claimed before release.
- Roll back the human state/harness and Academy evaluator changes together. The
  accepted core and prior Academy records require no migration.

## Open decisions

None.
