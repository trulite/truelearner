# CK0 junction checkpoint integration implementation audit v2

Status: frozen before CK0 v2 evidence.

Protocol: `f297bc1c6dcb6638dfa80b4c80dcb26a9319c214`
(`ck0-junction-checkpoint-integration-protocol-v2`).

Evaluator candidate: `cd8c8ab4a3c4427ecd7f1fbc48839e608a661c42`.

The runtime is byte-identical to CK0 v1 at
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

Only the evaluator measurement boundary changed:

- restored checkpoints explicitly re-enable physical tracing;
- Reference/Production equality excludes diagnostic raw checkpoint hashes;
- equality compares explicit PhysicalWork fields;
- legacy total remains serialized diagnostically;
- output labels identify v2.

World construction, all ten families, two roots, both mechanics, inputs,
checkpoints, direct predicates, replay, and quiescence are unchanged.

Hashes:

- evaluator:
  `f99086b78a07789932ec969cd53674e14f48ac14350b0730a0b48e3c8611029d`;
- evaluator manifest:
  `77d68046f0458ed92347ae2422da761bd2e8fdf18cf64bdd24ab8deecc897efb`;
- protocol:
  `ac0ace31d9e926bac10d1750a6ee8ae5b2d44e5b4540cf1c5f692288fceaf9d3`.

Targeted formatting, check, and strict Clippy passed on reusable E2B worker
`ifk44bxtlfjlci644r63m`. No v2 matrix, J0/CV0 replay, RS2, unrelated suite,
workspace-wide build, or project program ran. No Rust command ran locally.

The next execution must be the sole fresh CK0 v2 matrix.
