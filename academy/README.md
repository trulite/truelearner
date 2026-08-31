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

## Physical workstation

`academy-workstation` supplies the binocular visual world, one articulated
hand, keyboard, touchpad, monitor, collision, and exact world-plus-organism
replay. It talks only to `truelearner-workstation`.

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
academy-body ---------------------> truelearner-workstation -> truelearner-body
academy-workstation --------------> truelearner-workstation -> truelearner-body
academy-workstation-review -> academy-workstation
academy-formal -------------------> truelearner-workstation -> truelearner-body
       |
       +--------------------------> pinned Lean checker (frozen evidence only)
```

Historical research evidence remains under the repository's archive and
research directories. It is evidence, not a production dependency.
