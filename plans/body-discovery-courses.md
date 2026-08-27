# Split Body Discovery into honest courses

```text
eye control -> hand control -> eye-hand coordination
                                      |
                         touch manipulation -> two-hand coordination

binocular light -> stereo disparity -> vergence consequence
```

## Outcome

Represent Body Discovery as five explicit Academy courses instead of one flat
capability list: Eye Control, Hand and Finger Control, Eye-Hand Coordination,
Touch-Guided Manipulation, and Two-Hand Coordination. Eye and hand foundations
can develop independently; coordination and manipulation remain gated by their
actual capability prerequisites.

Add a bounded binocular-depth capability to Eye Control. Its claim is only that
the world presents a visible stereo disparity and that repeated vergence changes
produce consequences in both eyes. It does not claim distance estimation,
object permanence, reaching in depth, or general 3D understanding.

This is an Academy curriculum, world, and evidence change. It neither supplies
an action nor changes learner or body physics.

## Authority

- Path: `academy.md` Purpose, Body discovery, and Evidence rules; `arch.md`
  boundaries; `.agents/skills/academy/SKILL.md`
- Revision: frozen parent commit
  `e6813d07f6faca9a6a5c02d8673c100ef6834367`

## Model

- `BodyCourseKind` is the ordered external curriculum object. Each kind owns a
  non-empty ordered slice of `BodyCapability` values.
- Concatenating course slices yields `BodyCapability::ORDER` exactly once.
- A course result is one of `Acquired`, `Failed(capability)`, or `NotReached`.
  A failed capability closes that course. Another foundation course may still
  run when its own prerequisites are present; dependent courses are not reached.
  The compatibility `first_failure` field records the earliest failed capability
  in flattened order while per-course results preserve every reached frontier.
- Development for an acquired capability remains durable. When a capability is
  not acquired, the curriculum preserves its failed evidence but restores the
  checkpoint from before that lesson so the failed attempt cannot contaminate an
  independent course.
- Eye Control contains gaze contingency, gaze control, and binocular depth.
  Hand and Finger Control contains hand contingency and digit separation.
  Eye-Hand Coordination contains self/world coordination. Touch-Guided
  Manipulation contains contact, visual reach, tap/hold/release, and drag/pinch.
  Two-Hand Coordination contains bimanual control.
- A deterministic external depth band maps to a bounded horizontal disparity.
  The two target projections are equal and opposite around one world-space
  center. Nearer bands have greater disparity.
- `BinocularDepth` passes only when a stereo target is present and at least two
  changed vergence steps change sampled light in both eyes. Development and
  probe effects remain the existing harness effects.

## Invariants

- Academy and tests interact with the organism only through `HumanHarness`.
- Course, capability, target, depth band, disparity, expected movement,
  evaluator verdict, and score never enter `WorldSample` or a checkpoint.
- A common target in both eyes, monocular change, ordinary horizontal or
  vertical gaze, one accidental vergence movement, and passive world motion
  cannot establish binocular depth.
- Stereo projections stay within body coordinates and are symmetric around the
  external target center. The learner sees only the resulting light fields.
- Capability prerequisites precede their capability in the flattened course
  order. Existing course order after Eye Control remains unchanged.
- Probe mutation is discarded. Exact replay, natural quiescence, physical-work
  bounds, and the corrected digit-separation evaluator remain mandatory.
- A failed lesson cannot mutate the starting checkpoint of a later independent
  foundation course; its recorded working checkpoint remains replayable.
- Current failure at honest digit separation is still measured in the independent
  Hand and Finger Control course unless existing behavior demonstrates otherwise.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change `academy/crates/academy-body/src/course.rs` to add explicit course
  structure, course results, binocular evidence, and controls.
- Change `academy/crates/academy-body/src/world.rs` to render a deterministic
  stereo target for the binocular-depth experience and test its geometry.
- Change `academy/crates/academy-body/src/evidence.rs` and `src/lib.rs` to
  expose and record the split curriculum.
- Strengthen `academy/crates/academy-body/tests/body_course.rs` to check the
  reached, failed, and not-reached courses through the public harness-facing
  course API.
- Update `academy.md` and `academy/README.md` with the course split and bounded
  binocular claim.

Exclude `truelearner-core`, `truelearner-human`, learner physics, body physics,
motor selection, action injection, reward, evaluator feedback, general 3D
reasoning, depth-directed reaching, later capability-evaluator audits, ARC-AGI,
and Playground UI.

## Development style

TDD. Add pure controls for the course partition, stereo geometry, and
binocular-evidence falsifiers before connecting them to full course execution.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes course partition laws, stereo geometry, binocular evidence, and
  all existing evaluator/world controls.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes public course progress, current first failure, probe isolation,
  exact replay, and the semantic firewall.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
  establishes that the frozen learner and body remain passing.
- Academy workspace format, check, and clippy commands establish canonical,
  warning-free Rust.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured warm budget is strictly under 10 seconds. Record cold compilation
and the full generated-course run separately.

## Controls and evidence

- Held-out cases: equal images, nonzero symmetric disparity, two depth bands,
  horizontal gaze without vergence, one vergence consequence, monocular light
  change, repeated binocular consequences, seeds `31001` and `91003`.
- Negative controls: serialized samples contain no course/evaluator fields;
  probe checkpoints remain unchanged; learner and human files remain
  byte-identical to the frozen parent.
- Laws: course concatenation is exact; prerequisites are earlier; stereo
  projection is symmetric and bounded; nearer means larger disparity; an
  independent foundation remains runnable after another foundation fails; a
  course with an unmet prerequisite is not reached.
- Falsifiers: a flat target passes binocular depth; one eye alone passes;
  non-vergence movement passes; curriculum metadata reaches the organism;
  replay or quiescence changes; digit separation is silently weakened; a
  learner/body file changes; or the warm regression reaches 10 seconds.
- Evidence: validated plan, focused unit controls, public integration tests,
  two held-out generated course runs, exact replay, candidate receipt, and
  independent verification receipt.
- Not applicable because this changes an Academy developmental curriculum and
  does not promote learner authority or run an official external benchmark.

## Risks and rollback

- Different per-eye textures can create visual consequences unrelated to the
  target. Requiring an explicit stereo target in the same sample bounds the
  claim to exposure plus binocular motor contingency, not learned target depth.
- Inserting a capability changes later deterministic experience seeds. The
  repository is pre-release and the evidence schema advances with the new
  curriculum structure; old fixtures remain historical evidence.
- The current learner may fail the new binocular capability. That is an honest
  frontier result. Roll back the course/world/evidence change together rather
  than weakening the capability.

## Open decisions

None.
