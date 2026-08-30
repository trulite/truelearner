# Preserve physical evidence through one body frontier

```text
boundary or link -> queued firing -> junction meeting -> retained physical moment
```

## Outcome

Preserve exactly four physical facts through `truelearner-body`: whether an
arrival came from the boundary or a link, the transmitting link identity,
cause agreement accumulated at the meeting, and the participants and changes
of the current frontier. Do not change reaction laws, learning behavior, or the
public observer API.

## Authority

- Path: `truelearner/crates/body/src/engine.rs`,
  `truelearner/crates/body/src/core.rs`, and
  `truelearner/crates/body/tests/engine.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a` with the current
  uncommitted compact-body implementation as the candidate base.

## Model

An arrival origin is `None` for a boundary input and `Some(LinkId)` for a link
transmission. A frontier is the queued firing vector plus the changed events;
each change points to the linked list of firings that met at its junction.
Meeting cause is accumulated once as arrivals enter: one common cause is
preserved and disagreement becomes zero. `Edit::Send` composes through its
named link rather than re-entering through the boundary input function.

## Invariants

- External inputs have no transmitting link.
- Ordinary transmission and `Edit::Send` retain the exact link identity.
- Equal meeting causes are preserved; mixed causes become zero independent of
  input order.
- Participant construction is linear in active firings and performs no arena
  scan.
- The current frontier uses reusable vectors and linked indices, not one vector
  allocation per junction.
- Public events, physical behavior, slot sizes, `react`, and edit semantics are
  unchanged.

## Scope

Change `truelearner/crates/body/src/engine.rs`, the `Edit::Send` scheduling call
in `truelearner/crates/body/src/core.rs`, and focused engine tests. Do not change
attachment, calibration, arena slots, the new-harness contract, checkpoints,
or semantic reaction laws.

## Development style

Use TDD for origin, participant-chain, and cause-agreement laws, then implement
the compact frontier record and run all body and checkpoint regressions.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test engine`
  checks unchanged kernel behavior.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body engine::tests`
  checks boundary origin, link origin, exact participants, and cause agreement.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint`
  checks body and quiet-copy regressions.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the compact implementation.

## Development loop

The representative warm regression is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint`.
It must remain strictly under 10 seconds.

## Controls and evidence

Focused controls cover one boundary arrival, one ordinary link arrival, an
applied send, and several same-cause arrivals. Negative controls mix causes in
both orders and retain a dormant body. Attachment, repeated `step` equality,
checkpoint continuation, and arena slot size are held-out cases. The change is
falsified by lost link identity, observer-visible behavioral change,
order-dependent cause, work proportional to dormant body size, or a changed
arena slot size.

## Risks and rollback

The main risks are falsely labeling `Edit::Send` as boundary input and retaining
stale participant indices across frontiers. Centralize enqueue origin and clear
the reusable frontier before each pop. Rollback restores the two source files
and focused tests.

## Open decisions

None.
