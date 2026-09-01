# TrueLearner

TrueLearner is a compact physical learner developed and evaluated through one
production path:

```text
Academy -> WorkstationHarness -> Body
```

Sensors enter as ordinary physical events. Motor effects leave through the
harness. Junction identities remain private, checkpoints remain opaque, and
evaluator knowledge never enters the organism.

## Workspaces

`truelearner/` contains three crates:

- `truelearner-body`: the organism, physical propagation, choice, return, and
  link memory.
- `truelearner-workstation`: physical attachment, external state, observation,
  and direct checkpoint restore.
- `truelearner-behavior-contract`: shared black-box scenarios used to verify
  behavior independently of an adapter.

`academy/` contains eight crates:

- `academy-arc3`: ARC-AGI-3 workstation application adapter and trace evidence.
- `academy-body`: development courses, probes, controls, and evidence.
- `academy-formal`: offline Rust-to-Lean checks over frozen causal evidence.
- `academy-workstation`: the headless physical workstation world.
- `academy-workstation-course`: generic device-to-screen causal development,
  controls, transfer probes, and evidence.
- `academy-workstation2`: a gaze-dependent touchscreen world whose only
  organism inputs are retinal light and hand contact.
- `academy-workstation2-course`: development, shifted-layout probing, and exact
  replay evidence for gaze, touch, virtual keys, and pinch.
- `academy-workstation-review`: causally inert review of frozen recordings.

`formal/` contains pinned Lean projects used by those observer-side checks.

## Development

Agent routing lives in [AGENTS.md](AGENTS.md), with skills under
[`.agents/skills/`](.agents/skills/). Development uses `$dev`: apply
category-theory and TAME lenses, keep changes small, preserve black-box
behavior, and keep representative warm wave time strictly below 25 ns unless
the user expressly approves otherwise.

```sh
cargo test --manifest-path truelearner/Cargo.toml --workspace
cargo test --manifest-path academy/Cargo.toml --workspace
cargo run --release --quiet --manifest-path truelearner/Cargo.toml \
  -p truelearner-body --example engine_cost
```

Design and vocabulary documents are under [`docs/`](docs/). Start with the
[architecture](docs/arch.md), [algorithm](docs/algo.md), and
[language](docs/LANGUAGE.md).
