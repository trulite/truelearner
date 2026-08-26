# Sense the neutral body pose

```text
physical axis state -> negative | neutral | positive position receptors
                                             |
                                  ordinary local learner paths
                                             |
                                  paired decrease/increase motors
```

## Outcome

Represent an axis resting at its neutral pose as real proprioceptive input.
Every eye and hand axis gets the same neutral-position receptor, allowing a
fresh learner to encounter its initially still body before any motor route
exists. Signed off-neutral position, velocity, effort, and limits remain
separate truthful channels.

This repairs missing sensory presence; it does not inject motion, choose an
axis or direction, teach an action, alter learner physics, or weaken Academy
evidence.

## Authority

- Path: `academy.md` Body discovery and Evidence rules; `arch.md` Boundaries;
  `LANGUAGE.md`; `algo.md`; `plans/truthful-proprioception.md`;
  `plans/anatomical-receptor-topology.md`; prerequisite candidate receipt
  `factory-artifacts/body-visual-consequence-candidate.json`
- Revision: parent commit `bf633bc932be4684bc7070d1e6fb37b2614ef811`;
  prerequisite candidate tree
  `ce74c9440951e7847e13f878485ffcfe9aa3c6f6ec22c9b1f95bb2aea2cb659b`;
  candidate receipt SHA-256
  `2f300ce75d71e4a3bdfd0b2b1cab349baf108f8e068053755c491355e287a83a`;
  verification receipt SHA-256
  `dd55324b35592c4c3f0c68484eb994b5761a38bafc56752d591610e681226531`

## Model

- `sensory_features` is the pure projection from a `WorldSample` plus
  `HumanState` to fixed sensory channels.
- Each `AxisProprioception` maps to nine channels: negative position,
  positive position, neutral position, negative velocity, positive velocity,
  decrease effort, increase effort, lower limit, and upper limit.
- The neutral-position channel has maximum receptor magnitude exactly when
  signed position is zero. It is silent off neutral. The existing signed
  channels remain silent at zero and preserve their existing magnitudes away
  from zero.
- Harness admission, magnitude binning, anatomical placement, local path
  formation, paired motor opportunity, body integration, physical outcome,
  and checkpoint replay remain unchanged compositions.
- Academy continues to observe owned physical movement after the harness step
  and evaluates `HandContingency` without sending capability or desired action
  information into the organism.

## Invariants

- A neutral pose is sensed on all 25 axes, including both palms, wrists,
  contact-force axes, spread, thumb opposition, and all ten fingers.
- The neutral channel identifies presence at a physical joint, not a preferred
  direction. It remains equally local to decrease and increase motors.
- Exactly one of negative, neutral, or positive position presence is active for
  an axis; opposite signs never share a channel.
- Velocity, opposing effort, and limit semantics are unchanged.
- The body emits no control, readiness preference, success, reward, capability,
  target, or evaluator field. Outward learner crossings remain the only source
  of movement.
- Equal input and state produce equal sensory features; save and restore retain
  exact next-step behavior and natural quiescence.
- Core learner physics and the already validated visual-world prerequisite are
  unchanged.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change `truelearner/crates/human/src/harness.rs` to add one neutral-position
  receptor per anatomical axis and update its unit laws.
- Update `truelearner/crates/human/README.md` to state that neutral position is
  explicitly sensed.
- Strengthen `academy/crates/academy-body/tests/body_course.rs` to require that
  the unmodified learner acquires `HandContingency` in development and probe,
  while preserving probe isolation and the next honest course failure.

Exclude `truelearner-core`, learner choice or strength, motor readiness,
anatomical locality, actuator integration, outcome timing, Academy worlds,
capability thresholds, curriculum ordering, evaluator feedback, target
selection, semantic actions, accepted authority, voice, and ears.

## Development style

TDD. First express neutral/signed receptor exclusivity and the expected fresh
`HandContingency` boundary, then add the single generic channel and update the
contract.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`
  establishes neutral presence, signed-channel preservation, anatomical
  ownership, and equal motor locality.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
  establishes public boundary behavior, transactionality, checkpoint replay,
  and bounded work with the enlarged fixed input topology.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes fresh development/probe acquisition, probe isolation, semantic
  firewall, exact replay, and preservation of the next failure.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml` and
  `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establish workspace and Academy compatibility.
- Both workspace `cargo fmt --check`, `cargo check --workspace --locked`, and
  `cargo clippy --workspace --all-targets --locked -- -D warnings` commands
  establish canonical warning-free Rust.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`

Its measured warm budget is strictly under 10 seconds. Record cold compilation
and full Academy course execution separately.

## Controls and evidence

- Held-out cases: a neutral fresh state, positive and negative gaze positions,
  both sides, all 25 axes, course seeds `31001` and `91003`, exact checkpoint
  replay, and the first capability after `HandContingency`.
- Negative controls: a nonzero position silences neutral; zero velocity remains
  silent; the neutral receptor alone cannot move the body; probe state is
  discarded; serialized samples contain no course or evaluator fields.
- Laws: signed-position projection is total and exclusive; reflection swaps
  sign but preserves neutral; every axis owns exactly nine receptors; equal
  state maps to equal features; save/restore is identity on the next step.
- Competing repairs rejected by controls: tonic opponent coding would change
  signed-position semantics; body-driven readiness or motor babble would inject
  outputs; hand-specific jitter would leak curriculum structure.
- Falsifiers: any neutral axis lacks input; neutral prefers a motor direction;
  an off-neutral axis retains neutral presence; sign, replay, quiescence, or
  transactionality changes; `HandContingency` remains silent; an evaluator
  field leaks; or the warm regression reaches 10 seconds.
- Evidence: validated plan, focused and held-out tests, preserved next failure,
  exact replay, semantic-firewall control, candidate receipt, and independent
  verification receipt.
- Not applicable because this is Academy body-development evidence rather than
  an official external benchmark; no benchmark score or authority promotion is
  claimed.

## Risks and rollback

- One receptor per axis increases fixed harness size and work. Retain a bounded
  declared capacity and measure the warm suite and course receipts.
- A neutral signal could be mistaken for direction. Keep one shared neutral
  receptor at the axis center and re-run equal-distance locality laws.
- Passing hand contingency may expose a later honest failure. Preserve and
  report it without changing later worlds or evaluators.
- Pre-release private checkpoints have a different fixed sensor count. No
  public checkpoint format is promised; roll back the new channel, tests, and
  contract together if replay or work controls fail.

## Open decisions

None.
