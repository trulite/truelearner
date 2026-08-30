# Collapse the harness into the body crate

```text
body laws -> body::harness helpers -> Body + body::attachment
```

## Outcome

Remove the redundant `truelearner-new-harness` crate. Keep one small
`truelearner-body::harness` module containing the existing physical test setup
and observation helpers, and move the unchanged 28-law acceptance suite to
`truelearner/crates/body/tests/body_laws.rs`.

## Authority

- Path: `truelearner/crates/body/src/attachment.rs`,
  `truelearner/crates/body/src/lib.rs`, and
  `truelearner/crates/new-harness/tests/body_laws.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; the pre-move law
  suite runs 13 passed and 15 failed, with no ignored tests.

## Model

The body owns physical laws and generic attachment. The single harness module
owns only reusable construction, input scheduling, run capture, and event
projection used to exercise those laws. `body_laws.rs` owns assertions only.
Moving the same helpers and laws across the crate boundary must preserve their
behavior exactly.

## Invariants

- `attachment.rs` remains the only attachment implementation.
- The harness composes public body operations and adds no physical law.
- All 28 law names and assertions remain unchanged.
- The observed 13-pass/15-fail baseline remains unchanged after the move.
- No `truelearner-new-harness` workspace package remains.
- Old core, behavior-contract scenarios, and body physics are unchanged.

## Scope

Add `truelearner/crates/body/src/harness.rs`, export it from the body library,
move the law suite under body tests, remove the new-harness package and workspace
entry, and refresh the workspace lockfile. Do not change law expectations,
body physics, attachment, learning, core, or other consumers. Historical plans
and receipts remain historical evidence.

## Development style

Use implementation-first because this is a mechanical move: extract the existing helper block without semantic
changes, move the existing law assertions unchanged, then compare exact test
names and the 13/15 result before and after.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test body_laws --no-run`
  checks the moved suite and body-owned harness compile together.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test body_laws`
  must reproduce exactly 13 passed and 15 failed.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --lib`
  holds the passing physical kernel tests.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body --tests`
  checks the simplified crate boundary.
- `cargo tree --manifest-path truelearner/Cargo.toml -p truelearner-body --edges normal`
  confirms the body acquires no dependency.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test
body_laws --no-run`; it must remain strictly under 10 seconds. The known failing
law execution is recorded separately as behavioral evidence.

## Controls and evidence

Held-out cases are the body library tests and workspace dependency tree.
Existing negative controls inside `body_laws` remain unchanged, including
distance, repeated samples, ambiguity, expiry, and dormant size. The primary
control is exact before/after preservation of all 28 test names and the
13-pass/15-fail result. The change is falsified by a renamed, removed, newly
passing, or newly failing law; any new-harness package residue; a body dependency
addition; or a change to attachment or physics. Expected artifacts are the
validated plan, passing structural checks, preserved failure output, and a
candidate receipt for the structural gates.

## Risks and rollback

The risk is accidentally changing a test while moving helpers. Compare the
complete result and keep assertions untouched. Rollback restores the workspace
member and moves the helper/law files back to the standalone crate.

## Open decisions

None.
