# WS0 complete causal-wave semantics implementation audit v2

Status: frozen before WS0 v2 evidence.

Protocol: `ca341b6` (`ws0-complete-causal-wave-semantics-protocol-v2`).

The runtime candidate is byte-identical to WS0 v1:

- SHA-256:
  `d12b02bbb85645a916a5690d5ce5ebfd8e5c9d6820025a0c6d315a55aa0180a9`;
- one active runtime file;
- no new organism type or dependency.

The sole v2 change removes the unexercised `cv0j0` feature from the evaluator
and deletes its unreachable CV0/J0-only trace match arms. Directly exercised
features are `ce0 + rs0/pqlc0 + si0`.

The 14 worlds, five permutations, inputs, topology, predicates, normalizer,
checkpoint continuation, and output format are unchanged.

Frozen evaluator hashes:

- source:
  `a9e66333f0a657e81e3d79fc74f81ed7bbffd10884edb28318486149d83a7f42`;
- Cargo manifest:
  `cecc66b54cbaeadc0c0de5c71d5f6392ad131ba6f9196ea2ef794157fa4fcf59`.

Targeted E2B worker `ifk44bxtlfjlci644r63m`, commit `70c32f3`:

- rustfmt check: PASS;
- release `cargo check`: PASS;
- strict release Clippy: PASS.

No v2 physical world had executed when this audit was frozen.
