# TrueLearner production/research relocation v1

Outcome: **PASS — behavior-preserving relocation baseline**.

## Lineage

- Remote default branch at migration start: `main`.
- Successor branch: `runtime/truelearner-redesign`.
- Exact PX-C authority parent: `ec87c438aa8c52389fd2734667363ef43acaef93`.
- Oracle import: `a697558`.
- Mechanical relocation: `2a78ddc`.
- Production lockfile: `2d80ec1`.

No promotion to `main` occurred.

## Layout result

- `truelearner/` is the only production Rust workspace.
- `truelearner/crates/core/src/lib.rs` contains the accepted physical runtime.
- `truelearner/crates/core/src/main.rs` is the production composition root.
- `experiments/archive/pxc-authority/` contains the predecessor repository tree.
- `experiments/verification/pxc-relocation/` contains the relocation-only PX-C
  verifier.
- Production package count: `1`.
- Production runtime dependencies: `0`.
- Production dependencies on `experiments/`: `0`.

The relocated physical library SHA-256 is
`e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa`,
exactly matching PX-C authority.

## Targeted E2B validation

Sandbox: `ixipff2fv6d05m5o1o8tb`.

- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets`: PASS.
- `cargo test --workspace`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- Production binary startup: PASS.
- Cargo metadata closure: one package, no dependencies, workspace rooted at
  `truelearner/`: PASS.
- Relocated PX-C development matrix: `16/16` rows, `524/524` clauses: PASS.
- Regenerated development CSV SHA-256:
  `295a28fca72f560d87259624d7f20a171986582c818faa42bbb594399fcd5c89`:
  exact match.
- Regenerated development Markdown SHA-256:
  `22ba439015ca5684c542fefe75983ddcd85f1a43c1b03ce6b13f1a64fe37cb88`:
  exact match.

The authority matrix was not rerun. This result establishes a clean
engineering parent for redesign; it does not advance or replace the frozen
PX-C authority claim.
