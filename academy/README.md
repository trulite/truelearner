# Academy

Academy develops and probes the compact TrueLearner body through the public
`WorkstationHarness`. Academy owns worlds, lessons, probes, and evidence. The
organism receives only ordinary physical sensor input and returns motor effects.

```text
Academy Body / Workstation
            |
            v
    WorkstationHarness
            |
            v
    truelearner_body::Body
```

There is one production organism path. The former core, embodiment, semantic
harness, episode runner, ARC adapter, and desktop episode gallery are not part
of this workspace.

## Body Discovery

Run the headless body course from the repository root:

```sh
cargo run --locked --manifest-path academy/Cargo.toml \
  -p academy-body --bin academy-body-course -- \
  --seed 31001 --output output/body-course
```

The course develops and probes eye, hand, eye-hand, and workstation-contact
behavior. Development may commit learning; probes use cloned checkpoints and
cannot teach the durable organism. Output is an immutable receipt plus a
content-addressed transcript.

TapHoldRelease development records a cause-tagged external demonstration,
tests unaided imitation, and, when needed, restores the pre-demonstration body
for a non-learning press-depth ladder before self-caused practice on a softer
key. Each ladder rung clones the same checkpoint, changes only the external
press threshold, and stops at the first missing press. Contact continuation
requires an exact motor parent supplied by the world transition; boundary
completion releases only the antagonist in the same actuator component. Only
organism-caused events can satisfy the subsequent normal-key probe.

The workstation manipulation rung is split into `ContactDrag`, `ThumbContact`,
and `PinchDrag`. Rigid contact cancels only inward palm-depth effort and leaves
lateral slip free. Cursor motion is progress; drag release is terminal closure
with the exact current lateral parent. A separate light side patch begins just
outside the thumb, so actual thumb opposition must cross from no contact into
contact. PinchDrag adds a two-contact object: thumb and a selected ordinary
digit must remain in contact and undergo the same physical translation before
the object moves. Squeezing alone, one contact, or motion without one exact
palm-transport parent cannot pass.

Reference seed `31_001` now acquires all twelve bounded Body Discovery claims.
Changed-world transfer and post-learning retention are reported separately:
`ContactDrag`, `ThumbContact`, and `PinchDrag` are `Stable`. ContactDrag's
retention setup records and exactly replays an external lateral displacement;
the displacement receives no organism credit, and only the learner's later
causally parented drag can pass. The bounded course is complete through this
stability rung. Repeated low-cost automaticity remains a separate future
contract.

## Physical workstation

`academy-workstation` supplies the binocular visual world, one articulated
hand, keyboard, touchpad, monitor, collision, and exact world-plus-organism
replay. Keys expose physical press/release hysteresis and one visible
long-press consequence after two held steps. It talks only to
`truelearner-workstation`.

Record and render an observer-only workstation run with:

```sh
cargo run --release --locked --manifest-path academy/Cargo.toml \
  -p academy-workstation-review --bin academy-workstation-record -- \
  output/workstation-run --steps 48 --seed 82001
```

The recording and derived frames or video never return to the learner.

## Crates

- `academy-body`: Body Discovery development, probes, controls, replay, and evidence.
- `academy-formal`: offline Rust-to-Lean checking of frozen causal evidence.
- `academy-workstation`: the headless physical workstation world.
- `academy-workstation-review`: causally inert rendering of frozen workstation recordings.

The runtime dependency direction is:

```text
academy-body ----------> academy-workstation
      |                         |
      +-------------------------+--> truelearner-workstation -> truelearner-body
academy-workstation-review -> academy-workstation
academy-formal -------------------> truelearner-workstation -> truelearner-body
       |
       +--------------------------> pinned Lean checker (frozen evidence only)
```

Historical research evidence remains under the repository's archive and
research directories. It is evidence, not a production dependency.
