# Compose physical reaction into every body wave

```text
physical moment -> form paths -> choose/send -> apply edits -> next wave
```

## Outcome

Make normal `Body::step` and `Body::run` consume each retained physical moment
through the body's reaction law before releasing that moment. A nearby sensor
and motor opportunity must therefore produce the same outward action as the old
black-box harness, and the unchanged learning and checkpoint scenarios must be
allowed to proceed. Report warmed wall-clock time per primitive physical wave;
do not claim benchmark authority from that engineering measurement. This slice
does not claim the compact body already satisfies every higher competition or
recursive-membership law.

## Authority

- Path: `arch.md`, `LANGUAGE.md`, `algo.md`,
  `truelearner/crates/body/src/{arena,core,engine,physics}.rs`,
  `truelearner/crates/body/tests/body_laws.rs`, and
  `truelearner/crates/body/tests/behavior_contract.rs`
- Revision: content digests
  `02d837a8dc205aae7b088147226c94aa08783898a653550334718bbdf0cc003f`,
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`,
  `62363a087f660caa5ea6418fc0dd1c85195ebf0a0745d19ee58894b3a160224b`,
  `607968f296b1db5d2612a69765b573159c9705171d55be7fee6d04e272623d01`,
  `b2893936b6746a86a8f291c53e6f6cc59e2e1538f123b44d917eb6e59745b6f8`,
  `8162cf58c7ed6907ef5ddd79900449cfe6d82d644b26284da5ab1b67a4ec4366`,
  `dcf4f1e2e1cb53b1efab4325264910b486923d85d18ef16e73c80eb446e34529`,
  and `6139ad5985177b579eb5465c5af4e9a2709643eac94e9e06fa0e5dc4f47b8ccf`.

## Model

The input object is the completed `PhysicalMoment`: changed junctions and the
exact boundary or link arrivals that participated. A private reaction reads
that moment and local bidirectional incidence, then returns ordinary junction,
link, send, and link-memory edits. The engine applies those edits before the
moment is released. Formation, choice, output participation, returned outcome,
strengthening, reuse, and membership are successive physical transformations;
observation remains a separate read-only effect.

The first ladder rung is `sensor fires -> local path forms -> chosen path sends`.
Only after it passes may action, returned outcome, learning, and checkpoint
replay be evaluated. A reaction or application failure remains a typed
`RunError`; a partially applied reaction is forbidden.

## Invariants

- Reaction input contains only body state, physical changes, exact participants,
  cause, and time; no scenario IDs, expected effects, or evaluator knowledge.
- Boundary sensor changes may form paths only through local zero-impulse links
  at distance one or two; distance three and outward effects remain inert.
- A physical moment is reacted exactly once, after ordinary transmission and
  before its participant evidence is reused.
- Only an actually transmitting path can open a return, receive an outcome,
  strengthen, or be reused.
- Choice remains one per connected outcome component and independent across
  disconnected components and construction order.
- Quiet identity, repeated-step equality, exact cause, natural quiescence,
  checkpoint replay, observer purity, and dormant-body work remain unchanged.
- Existing black-box scenarios and all 28 body-law expectations remain
  unchanged, and no body law that passed before this slice may regress.

## Scope

Change only the compact body reaction and engine internals in
`truelearner/crates/body/src/{core,engine,arena,physics}.rs`, with focused private
tests only if an absent fact cannot be checked by the unchanged law suite. Add
candidate and verification receipts. Do not change adapters, shared scenarios,
expected effects, old core, Academy, workstation, or accepted architecture.

## Development style

Use TDD with the unchanged red ladder. First make the single nearby-action body
law and new-adapter scenario pass, then run learning, replay, the remaining 28
laws, and held-out kernel controls. Stop and preserve the first new divergence
instead of weakening a test.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test body_laws one_connected_world_chooses_exactly_one_action -- --exact`
  checks the first missing path-to-action transition.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract local_action_is_the_same_on_the_compact_body -- --exact`
  checks the same transition through the new adapter.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract`
  checks quiet, local/distant action, learning, checkpoint replay, and property
  variants through the new adapter.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test body_laws`
  records the broader physical frontier and checks that every previously green
  primitive and negative control remains green; unresolved higher laws remain
  preserved as failures rather than becoming part of this slice's claim.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint`
  checks held-out kernel, attachment, calibration, and checkpoint behavior.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the changed crate strictly.
- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo run --release --manifest-path truelearner/Cargo.toml -p truelearner-body --example engine_cost`
  measures warmed primitive-wave wall time over nine five-million-wave samples.

## Development loop

The representative warm regression is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-body --test body_laws`; it must remain
strictly under 10 seconds. Compilation and the release benchmark are recorded
separately.

## Controls and evidence

Negative controls are no nearby motor, distance three, outward reentry, mixed
cause, repeated and expired samples, pre-opening, duplicate consequence, and
dormant growth. Held-out controls are engine, attachment, calibration,
checkpoint, observer purity, and every body law green at the recorded 13/28
baseline. Higher competition and recursive-membership failures are retained
frontier evidence, not acceptance gates for this slice. The candidate is
falsified by a changed oracle, semantic information entering reaction, a path
without physical incidence, duplicate action or credit, observer-dependent
state, loss of quiet, regression of a previously green law, or a warm regression
at ten seconds or more. Evidence is the passing new-adapter suite, preserved
controls, validated receipts, recorded full-law frontier, and raw benchmark
summary.

## Risks and rollback

The main risks are reacting twice, choosing before a complete moment is known,
crediting a path that did not transmit, or mutating before validation succeeds.
Keep reaction at one engine boundary and preserve exact participant identities.
Rollback removes the reaction call and its physical projection, returning to
the recorded 13-pass/15-fail body-law baseline without touching adapters or
expectations.

## Open decisions

None.
