# Reuse body reaction work

```text
physical moment -> inspect once -> reusable reaction work -> non-empty edits -> apply
```

## Outcome

Make the compact body's internal wave reaction path reuse variable-sized work,
inspect each moment's participants once, select paths without temporary clones,
use the live-return index without collecting returns, and skip application when
there are no edits. Preserve all observable behavior and the public pure
`react_event` interface.

## Authority

- Path: `truelearner/crates/body/src/{core,engine}.rs` and the shared old/new
  black-box behavior contract
- Revision: content digests
  `c817fc721d06fdbb2809a3d819510bb35641fd68b255e3a6537a657ef9064e19`,
  `4b3f4d26624ee93d85cc8c7888a50009738d531b87c59679e1dbd8e14aab4033`,
  `dcf4f1e2e1cb53b1efab4325264910b486923d85d18ef16e73c80eb446e34529`,
  and `6139ad5985177b579eb5465c5af4e9a2709643eac94e9e06fa0e5dc4f47b8ccf`.

## Model

One physical moment is observed into compact per-change facts exactly once.
Pure selection reads those facts and retained body state into one edit
transaction. `ReactionScratch` owns reusable fact, path, connectivity, world,
winner, edit, and applied-identity buffers. The transaction is applied only when
non-empty, then every logical scratch collection is cleared while capacity is
retained. Public explicit events continue to produce owned `Reaction` values.

## Invariants

- Every physical participant list is traversed once per internal reaction.
- Edit ordering remains path formation and choice, then used-output recording,
  then returned-outcome recording.
- Selection, cause matching, ambiguity handling, retirement, strengthening, and
  outcome consumption remain behaviorally identical.
- Return discovery iterates `Body::live_returns` and creates no return vector.
- Internal world selection creates no per-world candidate clone vector.
- Empty internal reactions do not call `Body::apply`.
- Reusable work is logically empty between waves and is not learning or
  checkpoint authority.
- The public body, `react_event`, black-box scenario, and adapter APIs do not
  change.

## Scope

Change private internal reaction and application plumbing in
`truelearner/crates/body/src/{core,engine}.rs`; add focused private tests and
candidate and verification receipts. Do not change attachment, arena physics,
learning rules, public event reactions, adapters, scenarios, expectations, or
known advanced body-law behavior.

## Development style

Use TDD: add focused preservation tests for single-pass moment facts, empty
reaction identity, reused capacity, ambiguous paths, and return selection before
replacing the internal pipeline.

## Focused tests

- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint --tests`
  checks private plumbing and clone/checkpoint integration.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the changed crate strictly.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment --test calibration --test physics --test behavior_contract`
  checks reaction preservation, physical controls, attachment, and the compact
  adapter.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  checks the old harness contract.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-checkpoint`
  checks quiet cloning and replay.
- `cargo run --release --quiet --manifest-path truelearner/Cargo.toml -p truelearner-body --example engine_cost`
  measures the primitive warmed wave.
- `cargo run --release --quiet --manifest-path truelearner/Cargo.toml -p truelearner-body --example link_cost`
  measures active link propagation.

## Development loop

The representative warm regression is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment
--test calibration --test physics --test behavior_contract`; it must remain
strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

The old and new black-box contracts, engine equivalence laws, attachment, and
checkpoint continuation are held-out controls. Negative controls are quiet
identity, ambiguous path participation, unmatched returns, distance rejection,
and failed enqueue. The change is falsified by changed event or edit ordering,
different choices, retained logical scratch contents, a return-memory scan, a
warm regression at ten seconds or more, or meaningful benchmark regression.
Candidate and independent verification receipts preserve exact evidence.

## Risks and rollback

The main risks are changing edit order, retaining stale scratch values, or
losing new-identity resolution when reusing the edit transaction. Focused and
black-box tests detect these. Rollback restores the owned internal reaction and
temporary selection collections; authoritative body state is unchanged.

## Open decisions

None.
