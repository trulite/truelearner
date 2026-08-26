# Give receptors an anatomical physical topology

```text
retina -> eye axis site -----------+
touch  -> palm or finger site -----+-> local paths -> paired motors
body   -> its own axis site -------+                 decrease | increase
                                                        equal distance

owned observations -> Academy -> repeated causal consequence, not one motion
```

## Outcome

Replace ordinal sensory-to-motor placement in `HumanHarness` with a fixed
anatomical map. Retinal receptors occupy eye-axis sites, touch receptors occupy
their hand or digit sites, and every proprioceptor occupies its source axis
site. Each site is equally distant from the axis's decrease and increase motor,
so anatomy supplies locality without selecting a direction.

Strengthen Academy's `GazeContingency` evidence so one accidental gaze movement
cannot pass. Require repeated net gaze movements with externally observable
visual consequences. Preserve silence or balanced coactivation as
`MissingExploration`; do not repair it with a chosen action.

## Authority

- Path: `arch.md` Accepted law, Boundaries, and Forward design; `LANGUAGE.md`;
  `algo.md`; `academy.md` Body discovery and Evidence rules;
  `plans/truthful-proprioception.md`
- Revision: parent commit `766c39c1e527fbe29c7d7445e1d6902b2de5334e`;
  `arch.md` SHA-256
  `f5b1157b8d0479cb9575980f148090932043ce7d8c9da72d70fdb887cb4b4963`;
  `academy.md` working-tree SHA-256
  `32f8b03dd347b8fff357be05ad5c3efe5f6e3fc824b547c74d6d02e5d7e05f39`;
  prerequisite plan SHA-256
  `2e948591f5de2e9b3b8464a949985043e81fe77fdddc7ad8bc5f75dff9650830`

## Model

- `ReceptorSite` is the pure projection from one fixed sensory feature index to
  one `BodyAxis`. It carries no direction, target, capability, or evaluator
  meaning.
- Retinal offsets project only to horizontal gaze, vertical gaze, or vergence.
  Both eyes use the same axis topology; eye identity remains in the sensory
  junction identity rather than changing motor meaning.
- Palm contact projects to that hand's contact-force axis. Each fingertip
  projects to the matching digit-flexion axis. Proprioceptive position,
  velocity, effort, and limit channels all project to their exact source axis.
- `AxisLayout` assigns disjoint physical neighborhoods. One axis has a neutral
  receptor position with decrease and increase motors at equal unit distance;
  different axes are farther apart than the core local variation radius.
- Harness construction composes `feature -> receptor site -> physical position`
  and `control -> axis plus direction -> physical position`. It removes all
  modulo aliasing. Fixed capacities cover the maximum paths this topology can
  form without changing core allocation law.
- The Academy evaluator zips each world sample with its owned observation. A
  causal gaze consequence requires a changed gaze axis and a changed light
  value between the before/after eye focus in the same static sample. At least
  two such consequences are required for `GazeContingency`.
- Harness stepping, actuator integration, physical outcome, replay, generation,
  evaluation, and serialization remain separate transformations around the
  unchanged public core `Harness`.

## Invariants

- Every sensory value and motor opportunity still enters through the public
  core `Harness`; outward crossings remain the only source of body effort.
- No receptor site prefers decrease or increase. Both axis motors are equally
  local, and crossing order cannot change the anatomical map.
- Retina cannot form a local path to hand axes. Touch cannot form one to the
  wrong side or digit. Proprioception cannot form one to another axis.
- Different axes are outside one another's local variation radius.
- Feature identity and physical location remain deterministic, bounded, and
  checkpoint/replay stable.
- The fixed map contains only body topology. It contains no expected movement,
  corrective reflex, task direction, target coordinate, score, or action.
- Academy credits only repeated net gaze changes that alter actual sampled
  light; canceled effort, passive world change, or one accidental movement
  cannot pass `GazeContingency`.
- Academy evaluator state remains outside `WorldSample`, physical outcome, and
  durable learner state. Probes remain cloned and discarded.
- Core physics and accepted authority remain unchanged. The result is
  development evidence only.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change physical motor and sensory placement plus capacity declarations in
  `truelearner/crates/human/src/harness.rs`.
- Add pure topology laws and public-harness replay/work controls in the human
  crate tests.
- Change only gaze-contingency evaluation and its negative/positive controls in
  `academy/crates/academy-body/src/course.rs`.
- Update the human and Academy body contracts to describe neutral anatomical
  locality and repeated causal gaze evidence.

Exclude `truelearner-core`, actuator integration, proprioceptive values,
readiness strength or timing, path choice, learner thresholds, outcome law,
world-selected actions, directional reflexes, curriculum scheduling, later
capability evaluators, accepted authority, voice, and ears.

## Development style

TDD. Add failing topology laws and the one-motion Academy negative control
before replacing placement and evaluation.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`
  establishes anatomical ownership, equal motor distance, axis separation, and
  removal of modulo aliasing.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
  establishes bounded construction, exact replay, truthful movement, and
  transactional failure through the public boundary.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes repeated causal gaze evidence and rejects accidental, canceled,
  and visually inert movement.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes first-failure preservation, probe isolation, leakage controls,
  and exact replay with the new topology.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml` and
  `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 -p academy-body --lib`
  establish compatibility.
- Both workspace `cargo fmt --check`, `cargo check --workspace --locked`, and
  `cargo clippy --workspace --all-targets --locked -- -D warnings` commands
  establish canonical warning-free builds.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib && cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured warm budget is strictly under 10 seconds. Record cold bootstrap
and full course execution separately.

## Controls and evidence

- Held-out cases: every feature index, both eyes, both palms, all ten
  fingertips, all eight receptors on all 25 axes, reversed control ordering,
  disjoint world seed, and varied raster content.
- Negative controls: one causal gaze movement; repeated gaze movement over
  uniform light; passive changing light without gaze movement; canceled gaze
  effort; wrong-side and wrong-digit locality; read and replay mutation checks.
- Laws: receptor mapping is total and deterministic; every receptor is equally
  distant from its two axis motors; all other motors are outside local radius;
  left/right reflection preserves receptor kind and reflects side; save/restore
  is identity on the next complete step.
- Falsifiers: `% CONTROL_COUNT` or ordinal aliasing remains; a receptor is local
  to an unrelated axis or preferred direction; a single or visually inert gaze
  movement passes; evaluator data enters input; construction exhausts capacity;
  replay differs; core controls change; or the warm suite reaches 10 seconds.
- Evidence: validated plan, topology and Academy controls, preserved course
  failure, exact held-out replay and semantic firewall, candidate receipt, and
  independent verification receipt.
- Not applicable because this is not an official benchmark: no scorecard or
  authority verdict is produced.

## Risks and rollback

- Neutral locality may eliminate all movement because the current learner has
  no asymmetric motor exploration. Preserve that result; it identifies a
  learner frontier and does not justify a directional body shortcut.
- Pairing both motors doubles possible local paths. Declare capacity from the
  bounded receptor/bin/axis topology and test construction plus worst observed
  work.
- Gaze consequence can be confused with world motion. Compare before/after
  focus within the same recorded sample and retain passive-motion controls.
- A topology map can become a hidden controller. Review that it maps only
  receptor origin to axis and never to direction or desired result.
- Roll back placement and the gaze evaluator together. The accepted core and
  prior immutable output receipts remain unchanged.

## Open decisions

None.
