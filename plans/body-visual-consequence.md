# Give gaze a truthful textured world

```text
seed + pixel position -> static low-contrast texture
                                  |
target + hands -------------------+-> LightField -> HumanHarness
                                                        |
                                              learner gaze movement
                                                        |
                                                        v
                                       changed sampled light at new focus
```

## Outcome

Make the generated Body Discovery world spatially informative between its
sparse target, distractor, and hand pixels. A gaze movement that crosses a
raster cell should encounter a different ordinary light value, allowing
`GazeContingency` to measure repeated real visual consequences.

The claim is limited to truthful static visual consequence. The world does not
select a gaze direction, encode a target route, emit correctness, or change the
learner, body, course evaluator, or capability threshold.

## Authority

- Path: `academy.md` Body discovery and Evidence rules; `arch.md` Boundaries;
  `plans/body-discovery-academy.md`;
  `research/campaigns/motor-participation-v3/adjudication.toml`
- Revision: parent commit
  `bf633bc932be4684bc7070d1e6fb37b2614ef811`; prerequisite learner candidate
  `25e2364ff903f8a1ba12d7c747424366d53e5040`; `academy.md` SHA-256
  `c7fad69026cfab2db0bb95d95aed99fb5f6538c294cb9bd83adb7fb94fc0050b`

## Model

- A pure `background_light(seed, x, y) -> u8` transformation assigns every
  raster cell a bounded low-contrast value derived only from world seed and
  physical pixel position.
- `render_eye` first maps that transformation over the fixed 33×33 field, then
  overlays the existing target, distractors, palm, and fingertips by the
  existing maximum-light composition.
- `FlatWorld::sample` remains the only effectful step counter boundary. For a
  non-passive world, equal seed and equal body state produce an equal field.
- `HumanHarness` performs the unchanged foveated sampling. The existing course
  evaluator compares actual before/after focus samples without receiving or
  creating the texture.
- Construction fails through the existing `LightField` and `WorldSample`
  results; no new persistent or fallible state is introduced.

## Invariants

- Background values are deterministic, static, bounded below object light, and
  depend only on seed plus physical raster coordinates.
- Adjacent horizontal, vertical, and diagonal raster cells differ, so crossing
  a cell boundary changes ordinary light without implying a preferred motion.
- Target, distractor, palm, and fingertip pixels retain higher contrast and the
  same physical locations.
- A still body in a non-passive world receives the same field on repeated
  samples. Passive world motion remains evaluator-controlled external motion
  and cannot be credited as self movement.
- Capability, target meaning, expected direction, success, score, action, and
  evaluator state remain absent from organism-visible samples and checkpoints.
- Development may learn; probe mutation remains discarded; exact replay and
  natural quiescence remain required.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Change `academy/crates/academy-body/src/world.rs` to generate and test the
  static spatial texture before existing object overlays.
- Strengthen `academy/crates/academy-body/tests/body_course.rs` only if needed
  to assert that fresh `GazeContingency` development and probe evidence now
  pass while probe state remains discarded.
- Record the resulting Body Discovery frontier without changing later lesson
  criteria.

Exclude `truelearner-core`, `truelearner-human`, gaze/motor physics, body
integration, proprioception, outcome timing, course capability thresholds,
target selection, action injection, evaluator feedback, rendering, voice, and
ears.

## Development style

TDD. Add failing world laws for adjacent-cell distinction and repeated static
sampling, plus a fresh-seed `GazeContingency` boundary assertion, before adding
the smallest background transformation.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes texture determinism, adjacency, still-world identity, and the
  unchanged evaluator negatives.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes fresh development/probe evidence, probe isolation, replay, and
  the semantic firewall.
- `cargo test --locked --manifest-path truelearner/Cargo.toml --workspace`
  establishes that the external-world change leaves learner and human harness
  behavior unchanged.
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check` and
  `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`
  establish canonical warning-free Rust.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured budget is strictly under 10 seconds. Record cold compilation and
full course execution separately.

## Controls and evidence

- Held-out cases: seeds `31001` and `91003`, both eyes, horizontal and vertical
  cell boundaries, diagonal movement, field edges, and repeated identical body
  state.
- Negative controls: a uniform field still produces no visual consequence;
  repeated static-world sampling changes nothing; passive world change without
  gaze is not self movement; serialized samples contain no course, target,
  direction, success, score, or action field.
- Laws: the background transformation is total and deterministic; adjacent-cell
  distinction is preserved under seed offset; equal world plus body inputs map
  to equal fields; object overlay is monotone by light value.
- Falsifiers: a one-cell gaze change can remain visually identical; a still
  body sees changing static light; texture obscures body objects; a capability
  or desired direction leaks; the evaluator is weakened; replay differs;
  quiescence fails; or the warm regression reaches 10 seconds.
- Evidence: validated plan, focused tests, held-out Body Discovery receipts,
  exact replay, semantic-firewall control, candidate receipt, and independent
  verification receipt.
- Not applicable because this is an Academy capability course rather than an
  external official benchmark; no benchmark score or accepted learner-law
  authority is claimed.

## Risks and rollback

- A texture can accidentally become a directional code. Use a stationary
  position-only field with no body, target, capability, or evaluator input and
  test all neighboring directions.
- Excess background contrast can hide hands or targets. Keep it below the
  existing distractor and body values and preserve maximum-light overlays.
- Passing one gaze lesson could expose a later honest failure. Preserve the
  first next failure and do not alter later evaluation in this change.
- Roll back the background initialization and its tests. Existing sparse object
  placement, learner checkpoints, and evidence formats require no migration.

## Open decisions

None.
