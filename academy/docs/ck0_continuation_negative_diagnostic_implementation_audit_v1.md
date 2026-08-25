# CK0 continuation negative diagnostic implementation audit v1

Status: frozen before diagnostic execution.

Protocol: `49ccd611a6fe9c03a5c34ee1e37ca605f86f1e56`
(`ck0-continuation-negative-diagnostic-protocol-v1`).

Evaluator candidate: `3f6eed1bd391317ffb3f6e56e9a7e068f912a048`.

The CK0 runtime remains byte-identical at SHA-256
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.
No runtime or frozen CK0 evaluator file changed.

The diagnostic evaluator recreates only the two opaque continuation families,
with the same roots, mechanics, topology, checkpoints, and future inputs. It
serializes uninterrupted, restored-default-observer, and
restored-explicit-observer continuations field by field.

Hashes:

- evaluator:
  `c27afd1f2d6b935ead7e94b66b2091c177a57a435365eb9b4375f32ecec0873a`;
- evaluator manifest:
  `b8a17e917bab18a2b4770dd8100e28580be4512746befdb73b5adaee33c90e87`;
- protocol:
  `e71f40564533287b73f84a244545aa5c901a0a4677c240d76d0bb2aea593f106`.

Targeted formatting, check, and strict Clippy passed on reusable E2B worker
`ifk44bxtlfjlci644r63m`. No physical diagnostic, workspace-wide build,
unrelated suite, or project program ran. No Rust command ran locally.

The next execution must be the sole fresh diagnostic run.
