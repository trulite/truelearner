# Teach body discovery in Academy

```text
generated light + contact world
               |
               v
          HumanHarness
               |
     owned physical observation
               |
       +-------+--------+
       |                |
 development       fresh probe clone
 commits learning   discards mutation
       |                |
       +-------> external evidence
```

## Outcome

Add a headless Body Discovery Academy course that lets a learner discover and
coordinate two foveated eyes and two five-finger hands through their actual
visual, touch, and body-position consequences. It develops and separately
probes gaze control, independent hand and digit control, self/world
discrimination, contact, visual reaching, tap/hold/release, dragging, pinching,
and two-hand coordination.

The supported claim is visual-touch body fluency in generated flat worlds. It
does not claim language, object meaning, realistic human development, general
tool use, computer use, ARC competence, or accepted-body authority.

## Authority

- Path: `academy.md` Development loop, Capability evidence, Scheduling, and
  Evidence rules; `arch.md` Boundaries and Forward design; `LANGUAGE.md`;
  `algo.md`; `plans/human-shaped-harness.md`
- Revision: parent commit `766c39c1e527fbe29c7d7445e1d6902b2de5334e`;
  prerequisite plan SHA-256
  `7e4797714f79ab7ad5601b40a4c0e5c4ef9bf9df093cba6bb008836739c47137`;
  `academy.md` working-tree SHA-256
  `8e4b8ba126bdd726d2a2ec1c389b3c7fef329d572d2b620969447ad2e422cecd`;
  `arch.md` SHA-256
  `f5b1157b8d0479cb9575980f148090932043ce7d8c9da72d70fdb887cb4b4963`

## Model

- Add `academy-body`, depending on `academy-core` and the completed
  `truelearner-human` development crate. `BodyCourse` uniquely owns a public
  `HumanHarness`; Academy and tests never construct, retain, or inspect either
  underlying body.
- `BodyCapability` is evaluator-only and forms a prerequisite graph:
  `GazeContingency -> GazeControl`; `HandContingency -> DigitSeparation`;
  both branches feed `SelfWorld -> Contact -> VisualReach -> TapHoldRelease ->
  DragPinch -> Bimanual`. These names, states, and gates never enter a
  `WorldSample` or the harness.
- `BodyWorld` is a generated flat light/contact world. It owns initial pose,
  contrast patches, contact planes, movable inert shapes, occluders, and
  evaluator expectations. Its only organism-visible projection is the dumb
  pair of raster light fields and contact samples accepted by `HumanHarness`.
- `BodyExperience` is `Development`, `Probe`, `Transfer`, `Retention`, or
  `Control`. Development steps commit the resulting `HumanCheckpoint`. Every
  other mode runs from a cloned checkpoint and discards all mutation after
  recording evidence, so evaluation cannot teach the durable learner.
- One turn composes `world sample -> HumanHarness::step -> owned observation ->
  deterministic world transition -> next sample` until natural quiescence or a
  declared physical-work bound. Academy never supplies a body control, outward
  port, movement direction, action call, babble choice, answer, reward, or
  correctness input.
- Actual movement changes light and body-position input; actual contact changes
  touch and visible world state. These raw physical consequences return through
  the harness. Academy records whether an external capability condition was
  met but never converts that verdict into organism input or outcome.
- The course begins with passive and stillness controls, then presents safe
  varied worlds with every physical control still available. If no unguided
  outward exploration occurs within the frozen work budget, return
  `MissingExploration` and stop. Do not inject or select an eye, hand, finger,
  direction, or pressure opportunity as a curriculum workaround.
- Development stages use broad consequences before narrow composition:
  movement on a high-contrast field; independent eye/hand/digit consequences;
  passive external motion; broad contact planes; visible contact marks; fresh
  reach locations; press/hold/release surfaces; movable shapes; two-hand
  constraints. Difficulty changes only external geometry, initial pose, noise,
  and distractors.
- `BodyEvidence` records the generated case and mode, complete admitted samples,
  outward crossings, body movements, contacts, world transitions, physical
  work, learning updates, checkpoints and fingerprints, natural quiescence,
  capability verdict, first failure, and exact replay result.
- `BodySchedule` advances only from committed development plus passing fresh
  probes. A failed prerequisite reopens that prerequisite. Retention is spaced
  by intervening physical work, never wall time.
- A final body-fluency probe composes gaze, one-hand touch, dragging, pinching,
  and two-hand coordination in fresh generated worlds. It remains an Academy
  capability probe, not an ARC or external benchmark score.
- Generation, evaluation, evidence serialization, and scheduling are pure or
  causally inert around the single effectful `HumanHarness` step. Invalid
  worlds, budgets, checkpoints, replay, or evidence fail closed.

## Invariants

- Every organism-visible event is admitted through `HumanHarness`; Academy and
  tests read only owned public observations and checkpoints.
- Course stage, capability name, expected motion, target, success, score,
  teaching mode, and evaluator state never enter the organism-visible sample,
  physical outcome, or durable learner state.
- The course never chooses, injects, remaps, suppresses, or directly fires a
  body control. All eyes, hands, and digits remain physically available in
  every development and probe world.
- A world transition follows only observed body movement or independent frozen
  world dynamics. Evaluator success cannot cause a physical transition.
- Passive world motion is distinguishable from self-caused motion; absent or
  blocked contact produces the corresponding actual sensory consequence.
- Development commits learning. Probes, transfers, retention checks, and
  controls leave the durable checkpoint byte-for-byte unchanged.
- Cases regenerate pose, side, digit, location, geometry, contrast, timing,
  occlusion, and distractors. Fixed fixture memory cannot satisfy a capability.
- A capability advances only from recorded fresh evidence after all
  prerequisites pass. Silence, ambiguity, budget exhaustion, replay divergence,
  or a failed negative control cannot be promoted.
- Every experience reaches natural quiescence, stays inside declared work and
  memory bounds, and replays exactly from its recorded checkpoint and samples.
- The body topology, core physical law, accepted controls, and human Harness
  mechanics remain unchanged by curriculum implementation.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Add `academy/crates/academy-body` as a headless library and
  `academy-body-course` evidence binary; add the crate to the Academy workspace
  without adding it to Playground or review dependencies.
- Add concrete modules for generated flat worlds, capability prerequisites,
  deterministic scheduling, development/probe execution, evidence, replay, and
  an atomic JSON receipt.
- Add public-boundary tests that use only `HumanHarness` operations and owned
  observations. Add generated fixture seeds for development while reserving
  disjoint seeds for held-out controls.
- Update `academy.md` and `academy/README.md` to place Body Discovery before
  interface use and ARC, and document one `cargo run` command for the headless
  fixture course.

Exclude changes to `truelearner-core` or `truelearner-human`, body geometry,
hidden exploration or control mechanisms, direct body access, Playground UI,
rendering, sound, voice, semantic object labels, DOM/accessibility input,
keyboard meaning, text production, ARC integration, official scoring, body
authority promotion, and benchmark tuning.

## Development style

TDD. First freeze leakage, probe-isolation, no-exploration, passive-motion,
blocked-contact, independent-side, and exact-replay tests. Then implement the
smallest generated worlds and prerequisite scheduler that satisfy those tests,
one course stage at a time.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
- `cargo run --locked --manifest-path academy/Cargo.toml -p academy-body --bin academy-body-course -- --seed 31001 --output output/body-course-fixture`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

These establish the course graph, physical-only projection, learner-owned
exploration, probe isolation, generated transfer, controls, bounded execution,
exact replay, unchanged body mechanics, and compatibility with existing
Academy and ARC behavior.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured warm budget is strictly under 10 seconds. Record cold bootstrap
separately. Preserve the current Academy boundary regression as a compatibility
check and exclude compilation, receipt serialization, and rendering from
physical work.

## Controls and evidence

- Held-out cases: disjoint seeds vary starting gaze, hand pose, laterality,
  selected digit, contact location, shape, contrast, occlusion, and distractor
  motion; the composed body-fluency probe uses unseen layouts and stage order.
- Negative controls: passive visual motion without body movement; eye motion
  with stationary hands; one-hand motion with the other stationary; one-digit
  contact with all others quiet; proximity without contact; blocked movement;
  contact removed after approach; unrelated visible motion during a body
  output; and probe execution followed by exact durable-checkpoint comparison.
- Transfer controls: mirror worlds without renaming physical sides, new initial
  poses, scaled contact surfaces, novel paths to the same contact relation, and
  interleaved previously acquired controls.
- Laws: evaluation projection cannot reach `WorldSample`; probe execution is
  identity on durable state; world generation and replay are deterministic;
  disjoint side movements commute; capability advancement is monotone only with
  new admissible evidence.
- Falsifiers: any Academy-chosen body output, babble action, supplied route,
  evaluator field in input or outcome, hidden eye-hand coupling, probe learning,
  passive motion credited as self-motion, fixture-only success, changed body
  mechanics, replay divergence, failed control, missing quiescence, exceeded
  budget, or warm regression at or above 10 seconds rejects the candidate.
- Evidence: validated plan; generated development, fresh probe, transfer,
  retention, interference, and negative-control records; preserved first
  failure; exact replay transcript; body and checkpoint fingerprints; physical
  cost; immutable course receipt; candidate receipt; and independent
  verification receipt.
- Not applicable because this is an Academy development course rather than an
  external scored benchmark: no official scorecard or benchmark verdict is
  produced.

## Risks and rollback

- Curriculum shaping can become action teaching. Keep all controls available,
  forbid output injection in types and leakage tests, and let geometry alter
  only the consequences of learner-produced movement.
- A fresh learner may remain silent. Preserve `MissingExploration` as the first
  failure and route it to learner-physics research; do not repair it in Academy.
- Raw sensory change may strengthen coincidence rather than self-causation.
  Passive-motion, unrelated-motion, blocked-motion, timing, and replay controls
  detect false attribution.
- Sequential probes can teach accidentally. Restore or clone the exact durable
  checkpoint for every non-development experience and compare canonical bytes
  afterward.
- Generated worlds can leak stable positions. Use content-derived disjoint
  seeds, regenerate all physical context, and reserve holdout seeds before
  development.
- Roll back by removing `academy-body` from the workspace and deleting its crate
  and living-document references. The human Harness and accepted core require
  no checkpoint or data migration.

## Open decisions

None.
