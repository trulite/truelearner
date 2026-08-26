# CK0 junction checkpoint integration implementation audit v1

Status: frozen before CK0 evidence.

Protocol: `746559c9ed2dc8ec53628515ac958b0b57f198bf`
(`ck0-junction-checkpoint-integration-protocol-v1`).

Candidate: `d9f55aebf680d6cc74644c7c1cafbc39066f0ce5`.

## Runtime delta

The production organism remains the single file
`truelearner/crates/core/src/lib.rs`. CK0 changes only native checkpoint body
restoration:

- J0 CELL `live`, generation, and dormant resistance are restored
  independently;
- non-J0 CELL resistance/liveness validation is unchanged;
- dead resident CELL records are found by stored identity while transient
  checkpoint fields are restored, without making the CELL live/resolvable;
- dead ARROW records may retain stale endpoint generations and remain inert;
- live ARROWs still require live generation-matching endpoints;
- ARROW liveness still equals positive ARROW resistance;
- the now-unused checkpoint helper that required a live CELL was removed.

No execution, causal-wave, participation, consequence, learning, decay, J0,
proposal, slot-reuse, or stale-reference law changed.

Canonical runtime SHA-256:
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

## Evaluator

The new evaluator contains the ten preregistered checkpoint families under two
disjoint roots and both Reference and Production mechanics: 20 cases and 40
rows, each with same-mechanics replay and cross-mechanics equality.

Hashes:

- evaluator:
  `fc0c55b7135c55e3b4ba5163b6b846173947acd934bba1a4d65534686bade306`;
- evaluator manifest:
  `77d68046f0458ed92347ae2422da761bd2e8fdf18cf64bdd24ab8deecc897efb`;
- protocol:
  `ffd0185e8d6762af1c776052c47af3ece73ca7ed1eb3c037a09080ab55b8bc78`.

## Targeted validation

Reusable E2B Rust worker: `ifk44bxtlfjlci644r63m`.

At exact candidate commit `d9f55ae`, the following passed:

- evaluator-scoped `cargo fmt --check`;
- evaluator-scoped `cargo check`;
- evaluator-scoped strict Clippy with `-D warnings`.

The initial formatting-only stop ran no compilation. One subsequent targeted
check exposed an unused pre-CK0 checkpoint helper; it was removed before this
freeze, after which the clean validation above passed.

No workspace-wide build, unrelated suite, checkpoint matrix, J0/CV0 replay,
or organism program ran. No Rust command ran locally.

## Evidence boundary

The CK0 matrix has not executed. Its next run must be the sole fresh evidence
execution. J0, CV0, RS2, CE1, FD2, and ARC remain unexecuted on this candidate.
