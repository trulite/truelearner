# RS2 learned inhibitory topology implementation audit v6

Status: frozen before v6 physical evidence.

Protocol: `67af753` (`rs2-learned-inhibitory-topology-protocol-v6`).
Candidate: `f13f82c0b2748b6cbe4a4e29df8b22a8ffcf0e5e`.

## Frozen surface

The canonical runtime is byte-identical to CK0:

- `truelearner/crates/core/src/lib.rs`;
- SHA-256
  `078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

The evaluator-only delta is limited to:

- continuing recurrence observed from non-quiescence and repeated firing at
  both recurrence junctions;
- positive relation re-execution observed from WS0 positive Drive incidence
  plus the selected-contact firing;
- v6 output labels.

No world, timing, topology, training/probe sequence, seed, identity mapping,
learning law, mechanics path, work limit, or other predicate changed.

Frozen hashes:

- evaluator:
  `696e6107feeefb042d46aeb5904cdaf48d5db9c592e7372388ce373fe5251060`;
- evaluator manifest:
  `3b09fceafb20f0052fedf74dc3585b6a2dcaad8a615918fdc2d50c5b58ce7b16`;
- protocol:
  `9e857036a8e2f25ce359277fec135139b239d47bad6e649d2ed1872a847ad07f`.

## Targeted validation

Reusable E2B Rust worker: `ifk44bxtlfjlci644r63m`.

At exact candidate commit `f13f82c`, evaluator-scoped formatting, check, and
strict Clippy all passed. No workspace-wide build, unrelated test suite, or
physical matrix ran. No Rust command ran locally.

The next complete `180`-case/`360`-row matrix is v6's sole fresh evidence run.
