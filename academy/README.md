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
harness, episode runner, and desktop episode gallery are not part of this
workspace.

## Body Discovery

Run the headless body course from the repository root:

```sh
cargo run --locked --manifest-path academy/Cargo.toml \
  -p academy-body --bin academy-body-course -- \
  --seed 31001 --with-workstation --output output/body-course
```

The course develops and probes eye, hand, eye-hand, and workstation-contact
behavior. Development may commit learning; probes use cloned checkpoints and
cannot teach the durable organism. Output is an immutable receipt plus a
content-addressed transcript and the opaque completed-body checkpoint.

With `--with-workstation`, the course also branches from the post-TapHoldRelease
checkpoint into a generic workstation lesson. An external demonstration alone,
a passive screen change, and key motion without a screen response are separate
controls. Development is credited only when an organism-caused key event is
followed by a changed monitor frame returning through that exact crossing. The
learned branch passes fresh and normal-depth probes, but not a shifted-hand
transfer, so its state is `Acquired`, not `General`.

The receipt emits both artifacts: `body-checkpoint-*` is the body that completes
all twelve Body Discovery claims, while `workstation-body-checkpoint-*` is the
branch that acquired device-to-screen causality. Later ContactDrag learning
currently makes the key path inaccessible; the retention ladder records that
failure rather than claiming that the two branches have composed.

TapHoldRelease development records an external demonstration,
tests unaided imitation, and, when needed, restores the pre-demonstration body
for a non-learning press-depth ladder before self-caused practice on a softer
key. Each ladder rung clones the same checkpoint, changes only the external
press threshold, and stops at the first missing press. Contact continuation
requires a recent local motor crossing supplied by the world transition; boundary
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
stability rung. Repeated-use automaticity has separate constructed-body and
changed-light workstation mechanism evidence. The course now emits a separate
automatic repeated-use receipt after seven returned uses, observed retained-link
reuse at normal depth, equal external behavior, reduced physical work, passive
interference, controls, checkpoint retention, and exact replay. This does not
upgrade the twelve body capability states or imply general workstation skill.

## ARC-AGI-3 diagnostic

`academy-arc3` runs ARC-AGI-3 as an application on the same physical
Workstation2 touchscreen. The 64×64 frame occupies a central game viewport.
Generic bezel controls surround it. A completed control activation maps to
actions 1–5 or 7; a completed content activation maps to point action 6. The
offered catalog changes only which generic controls are lit and touch-active.
It cannot choose or inhibit body movement. Game identity, score, terminal
state, action budget, and evaluator data remain in Python.

The course acquires the ARC prerequisites through `Sequence` and stops at the
independent `Drag` capability. ARC may start from that frontier checkpoint.
Runs remain `plumbing-negative-control` evidence until the body passes the
generic game-surface probe. The pinned fixture preserves the physical trace and
exact fresh-process replay. Server-selected and private holdouts remain
untouched.

```sh
cargo run --release --locked --manifest-path academy/Cargo.toml \
  -p academy-workstation2-course --example course -- \
  256 fresh 11 \
  /tmp/workstation2-arc-entry.bin
cargo build --release --locked --manifest-path academy/Cargo.toml \
  -p academy-arc3 --bin academy-arc3-capstone-agent
cd academy/capstones/arc3
uv run capstone.py --mode fixture \
  --agent ../../target/release/academy-arc3-capstone-agent \
  --workstation2-checkpoint /tmp/workstation2-arc-entry.bin \
  --output /tmp/truelearner-arc3-fixture
```

## Crates

- `academy-arc3`: the blind Rust ARC-AGI-3 sensorimotor boundary and trace agent.
- `academy-body`: Body Discovery development, probes, controls, replay, and evidence.
- `academy-formal`: offline Rust-to-Lean checking of frozen causal evidence.
- `academy-workstation2`: the headless touchscreen world and generic applications.
- `academy-workstation2-course`: the pre-ARC screen-use ladder and controls.

The runtime dependency direction is:

```text
academy-body ----------------------> truelearner-workstation -> truelearner-body
academy-workstation2-course -> academy-workstation2 --------^
academy-formal ------------------------------------------------^
       |
       +--------------------------> pinned Lean checker (frozen evidence only)
academy-arc3 -------------> academy-workstation2 -------------^
```

Historical research evidence remains under the repository's archive and
research directories. It is evidence, not a production dependency.
