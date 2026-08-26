# RS2 learned inhibitory topology post-CK0 implementation audit v1

Status: frozen before post-CK0 physical evidence.

Protocol: `8aa7500`
(`rs2-learned-inhibitory-topology-post-ck0-protocol-v1`).

Candidate evaluator: `1570f1263f9667896ee65d1ed515a842d3bfa2ba`.

## Frozen surface

The canonical single-file runtime is unchanged from CK0 v2:

- `truelearner/crates/core/src/lib.rs`;
- SHA-256
  `078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

The consolidated RS2 evaluator changed only its output directory, report
heading, failure label, and positive sentinel. Its nine families, roots,
phases, physical worlds, observations, predicates, checkpoint continuation,
limits, and Reference/Production paths are unchanged.

Frozen hashes:

- evaluator:
  `7a3ea0885bdb1bf2fa99067a9b8c8b8107cce483d06fb320b687cc28440f975b`;
- evaluator manifest:
  `3b09fceafb20f0052fedf74dc3585b6a2dcaad8a615918fdc2d50c5b58ce7b16`;
- protocol:
  `8d28d98f988b13aaeb7bedf16a2d882292291550db6a48a0fe00ab8da545f77c`.

## Targeted validation

Reusable E2B Rust worker: `ifk44bxtlfjlci644r63m`.

At exact candidate commit `1570f12`, the following passed:

- evaluator-scoped `cargo fmt --check`;
- evaluator-scoped `cargo check`;
- evaluator-scoped strict Clippy with `-D warnings`.

No workspace-wide build, unrelated test suite, physical matrix, or project
program ran during validation. No Rust command ran locally.

## Evidence boundary

The complete `180`-case/`360`-row post-CK0 matrix has not executed. Its next
execution is the sole fresh evidence run. Any failure freezes the gate
negative without evaluator or runtime repair.
