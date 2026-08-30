# Add an external quiet-body checkpoint service

```text
exclusive Body borrow
        |
        v
drain existing signals -> quiet Body -> clone -> Checkpoint
        |                                      |
        `--------- observed events ------------'
                                               |
                                               v
                                         restore clone
```

## Outcome

Add a separate `truelearner-checkpoint` crate that captures an in-memory copy of
a `truelearner-body::Body` only after existing activity has drained to natural
quiet. The service forwards every drained physical event to the caller and
releases exclusive access when the call returns. Restoring a checkpoint returns
an independent body clone with the same future behavior.

This claim covers in-process checkpoints only. Durable bytes, files, versions,
checksums, external sensor buffering, and attachment remain outside scope.

## Authority

- Path: `truelearner/crates/body/src/engine.rs`,
  `truelearner/crates/body/src/arena.rs`,
  `truelearner/crates/body/src/core.rs`, and
  `truelearner/crates/body/tests/engine.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`;
  content digests `655154f72bb1e6800c85a4f30227e565723b675276401e37be3f6136936cdab1`,
  `ade66a13509797a79b8b481661f7f8f16338ca488ef4b0f8ef18cbf72ff64c65`,
  `12a1c6e51dfcb7dbe8c2ba21e72b6172e92be986663e4cc5186e4d32d4bb6789`,
  and `1dc04d8e957f07e51632e653db813646242a2c4abf0a5534fe5f0ae6331af564`

## Model

`checkpoint` is an external transformation from an exclusively borrowed body
to `(same quiet body, Checkpoint)`. Exclusive `&mut Body` ownership is the input
gate: no other safe Rust code can call `input` while the service drains and
copies. `Body::run` composes existing internal arrows until quiet, forwarding
their events. `Checkpoint::restore` is a pure clone that may be called more than
once.

For every successfully captured body `B`, checkpoint `C`, and future input
transformation `f`, `f(B)` and `f(C.restore())` must emit equal events, return
equal work, and reach observationally equal continuations. Capturing an already
quiet body is identity. A moment-limit error produces no checkpoint and returns
the still-owned body to the caller for continued draining.

The checkpoint owns a private `Body`, not arenas, bytes, or a public inspection
surface. Because `Body::clone` already includes its arena, link learning memory,
clock, and activity, the service adds no state representation to the body.

## Invariants

- `truelearner-body` receives no checkpoint code or dependency.
- Capture holds one exclusive mutable borrow for the complete drain-and-copy
  operation.
- A returned checkpoint always contains a naturally quiet body.
- All physical events produced while draining are delivered exactly once to
  the supplied observer.
- Failure at the moment limit returns `RunError` and no checkpoint.
- Restores are independent clones; changing one restored body cannot change the
  checkpoint or another restore.
- Junction sensor memory, physical link behavior, link roles, learning memory,
  body time, and identity are preserved by the body clone.
- The service contains no file, serialization, checksum, attachment, or signal
  buffer policy.

## Scope

Add `truelearner/crates/checkpoint/Cargo.toml`,
`truelearner/crates/checkpoint/src/lib.rs`, and
`truelearner/crates/checkpoint/tests/checkpoint.rs`. Add the crate to the
`truelearner` workspace and default members so ordinary workspace verification
includes it. Update the workspace lockfile mechanically.

Do not change `truelearner/crates/body`, the rewritten body-law suite, old core,
embodiment, workstation, Academy, research, attachment, or durable persistence.

## Development style

Use TDD. Write the quiet identity, drain observation, continuation, sensor
memory, link role, time, independent restore, and failure/recovery tests with
the service implementation as one complete candidate.

## Focused tests

- `cargo test -p truelearner-checkpoint` establishes the service contract and
  continuation laws.
- `cargo test -p truelearner-body` is the held-out check that the body is
  unchanged and its physical laws remain green.
- `cargo clippy -p truelearner-checkpoint --all-targets --no-deps -- -D warnings`
  checks the service and its tests without turning an unchanged body style lint
  into a checkpoint failure.
- `cargo tree -p truelearner-checkpoint --edges normal` establishes that the
  service depends only on `truelearner-body`.

## Development loop

The representative warm regression suite is
`cargo test -p truelearner-checkpoint -p truelearner-body`. It must complete
strictly under 10 seconds when warm; cold compilation is recorded separately.

## Controls and evidence

The existing body suite is held-out because it does not specify the checkpoint
implementation. Negative controls are a self-loop that exceeds the drain limit,
an input before the restored clock, a repeated sensor sample, and two restores
mutated independently. The candidate is falsified by a checkpoint returned
before quiet, a lost drain event, unequal future continuation, shared restored
state, a body source change, or a warm regression at or above 10 seconds.

Evidence is the exact command output and time, dependency tree, unchanged-body
diff check, changed-path list, tree digest, candidate receipt, and independent
verification receipt.

## Risks and rollback

The main risk is mistaking cloning for durable persistence; the API and crate
documentation explicitly call the result in-memory and process-local. Another
risk is silently swallowing drain events; the mandatory observer and event
equality test detect it. Runaway activity is bounded by the caller-supplied
moment limit and remains in the body after failure.

Rollback is removal of the checkpoint crate and its workspace membership. The
body needs no rollback because it is not changed.

## Open decisions

None.
