# Make the harness the only public body boundary

```text
Academy / integration tests
      | send                 ^ run
      | read                 | owned observation
      v                      |
Harness ---------------------+
      |
      v
private Body [selected Protocol]
```

## Outcome

Academy and behavioral Core tests construct, drive, and observe the organism
only through `Harness`. `Body` becomes crate-private and no public API returns a
body, arena, resident slot, or mutable implementation reference.

This is an API membrane change only. Physical transitions, stable identities,
outputs, work, traces, time, checkpoints, replay, and durable formats remain
unchanged. It adds no physical law, Academy semantic, or second protocol.

## Authority

- Path: `arch.md` sections 1–3 and 25; `academy.md` sections 1, 10, and 14–16;
  `LANGUAGE.md`; `algo.md`; `/Users/satya/work/br/AGENTS.md`
- Revision: commit `4b0309ac200f50e3b43ebddd60a1195877c08f07`;
  `arch.md` SHA-256
  `a2ddfc631bd2a3472503d9a44345f738e573d84685b5cbcf1b8994ae72fb6a6d`;
  `academy.md` SHA-256
  `445b94767b59bbe7f11054f9f77330d7ee4286246d4d1c566a5fe8e38227efdb`

## Model

- `HarnessBuilder` owns a private body during construction. It selects capacity,
  outward region, tracing, and `Protocol`; adds junctions and links; configures
  the outcome source and fixture-only link trigger; and is consumed by `build`.
- `Harness` uniquely owns the live private body. Its runtime transformations are
  `send`, `advance_to`, `read`, `save`, and `restore`.
- `send` maps an ordered input batch to a `Run`; `advance_to` maps a later tick to
  `Work`; both mutate only the owned body.
- `read` returns an owned `HarnessObservation`: clock, selected protocol,
  canonical `ArenaBody`, held-return count, and owned per-link strength, life,
  and participation readings. Canonical bytes, fingerprints, and diagnostics
  derive purely from that value.
- `save` maps a harness to the existing `Checkpoint`; `restore` yields a harness
  or `CheckpointError`.
- Stable arena, junction, and link identities remain physical protocol values.
  Resident slots, schedules, mutable arenas, and `Body` remain implementation
  details.
- `Protocol` stays in the private body and checkpoint. Future variants must
  preserve the same run, observation, and restart laws; Academy never branches
  on the selected implementation. Keep the existing enum and function table;
  add no trait hierarchy.
- Construction, send, time advance, and restore contain mutation. Reads,
  diagnostics, hashing, and evaluation remain causally inert. Existing physical
  precondition behavior is outside this membrane-only change.

## Invariants

- The physical story in `algo.md` and all junction/link transitions are unchanged.
- The same checkpoint and ordered inputs preserve outputs, work, trace, clock,
  body bytes, and natural quiescence.
- Repeated reads are equal; inserting a read before send changes neither the run
  nor the next checkpoint.
- `build` transfers unique ownership and exposes no body alias.
- Academy and external Core tests cannot name or construct `Body`.
- Inputs remain anonymous physical arrivals; outputs remain outward crossings;
  evaluator-only knowledge never enters organism state.
- Stable IDs, canonical `ArenaBody`, and existing causally inert link readings
  remain observable; resident slots and mutable state do not.
- Protocol selection survives checkpoint restore without changing public behavior.
- Existing replay, pressure phase, work, diagnostics, and fingerprints stay exact.

## Scope

- In `truelearner/crates/core/src/core.rs`, replace public `Core` with `Harness`;
  add `HarnessBuilder`, `HarnessObservation`, and owned link readings; remove
  `body()`; forward fixture construction, protocol selection, send, read, time,
  and checkpoint operations.
- In `body.rs`, `format.rs`, `input.rs`, `physics.rs`, and `lib.rs`, reduce body
  APIs to crate visibility and stop exporting `Body`; do not alter algorithms.
- Move `src/tests.rs` to `tests/harness_boundary.rs` so Rust privacy enforces the
  harness boundary. Express fixture triggers through the builder before `build`.
- Migrate `academy-core/src/lib.rs`, `academy-core/src/a1.rs`, and
  `academy-arc3/src/sensorimotor.rs` from construction/body reads to builder,
  send, read, and checkpoint calls.
- Update affected imports and names in both workspaces.

Exclude physical-law changes, new protocol variants, checkpoint/schema changes,
opaque semantic ports, buffers, framebuffers, storage, distribution, archived
research/evidence, Playground UI, and unrelated cleanup.

## Development style

TDD. First express the existing Core worlds as an external harness integration
test and migrate Academy call sites to the intended API. Then implement the
smallest facade and visibility changes that restore compilation and preserve the
existing assertions.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary -- --ignored`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-arc3 --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-arc3 --lib -- --ignored`
- `python3 -c "from pathlib import Path; roots=[Path('academy'),Path('truelearner/crates/core/tests')]; bad=[str(p) for r in roots for p in r.rglob('*.rs') if 'truelearner_core::Body' in p.read_text() or 'boundary.body()' in p.read_text()]; assert not bad, bad"`
- `cargo check --workspace --locked --manifest-path truelearner/Cargo.toml`
- `cargo check --workspace --locked --manifest-path academy/Cargo.toml`
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

The tests establish ordinary and adversarial Core behavior, Academy/ARC3 replay
and controls, compiler-enforced body privacy, workspace compatibility, format,
and strict lint cleanliness.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`

Pre-change warm baseline: `1.18 seconds`, strictly under 10 seconds. The prior
compilation-bearing baseline was `5.00 seconds`; record cold bootstrap separately
in candidate evidence and exclude it from the warm gate.

## Controls and evidence

- Held-out cases: run the two ignored Core boundary tests and two ignored ARC3
  boundary tests without changing assertions or ignored reasons.
- Negative controls: far output forms no path; unsupported input strengthens
  nothing; unsupported repetition does not mature; shuffled action meaning stays
  external; blocked or absent return stays silent.
- Laws: repeated read identity, read-before-send preservation, checkpoint/run
  preservation, and protocol selection preservation.
- Falsifiers: any changed output, work, trace, pressure phase, fingerprint,
  checkpoint bytes, replay verdict, natural quiescence, public body escape, or
  warm regression at or above 10 seconds rejects the candidate.
- Evidence: validated plan, factory-generated candidate receipt with exact digest
  and checks, and independent verification receipt tied to that digest.
- Not applicable because this is an engineering membrane refactor: do not create
  a research arm or run a frozen authority evaluator.

## Risks and rollback

- A live builder merely renames `Body`; prevent send/read after construction and
  consume the builder on `build`.
- An observation can leak resident state; return owned canonical values only.
- External tests may depend on private mutation; configure initial triggers in
  the builder, never through a live escape hatch.
- Academy diagnostics may become causal; derive them only from post-run reads.
- Facade forwarding may alter time, filtering, or restart; exact replay, body
  bytes, adversarial tests, and preservation laws detect this.
- Roll back the facade, test move, and Academy migration to commit
  `4b0309ac200f50e3b43ebddd60a1195877c08f07`; no persistence migration is needed.

## Open decisions

None.
