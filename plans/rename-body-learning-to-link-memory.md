# Name per-link state as link memory

```text
arena link N <-> link_memory N
```

## Outcome

Rename the compact body's private `learning` vector to `link_memory` so its name
matches what it contains: mutable state for every physical link. Preserve all
behavior, layout, indexing, attachment, cloning, and checkpoint semantics. Do
not add the proposed return mapping in this change.

## Authority

- Path: `truelearner/crates/body/src/{engine,core,attachment}.rs` and
  `truelearner/crates/checkpoint/`
- Revision: content digests
  `f9f3829890d1a5a71fdd4028ff792c360719068185514928b835db2aba83d3fe`,
  `024ece56d981d1d93b68ef6a1bee909a7285fcecfbff9c91f93ac0d7cee3b598`,
  `c63d1ef9995a462704a5b869070be8d266a9e28cb05983d94396c8e7ad25e4a3`,
  `23e99deef779610d49256ac1427e70d3b5a4ce82d4a83b3f73cdfbad39a1d2a4`,
  and `a25120d37470a6e975296b1df6299ed7ffca004f43b7254a24604ff85708946f`.

## Model

`Arena` owns immutable physical link topology. `Body::link_memory` owns the
parallel mutable `LinkMemory` entry for each arena link. Link construction and
attachment extend both collections in identical order; a link's slot remains
the total lookup from physical link identity to its memory.

## Invariants

- `arena.link_count() == link_memory.len()` remains true after construction and
  attachment.
- Every `LinkId` addresses the same memory entry before and after the rename.
- No field, type, layout, behavior, visibility, or persistence representation
  changes.
- The return mapping remains a separate proposed body-owned derived index.

## Scope

Rename the private field and local attachment variables in
`truelearner/crates/body/src/{engine,core,attachment}.rs`, including private unit
tests. Add candidate and verification receipts. Do not alter reaction logic,
the arena, adapters, scenarios, body laws, checkpoint code, or return indexing.

## Development style

Use implementation-first because this is a private mechanical rename with
unchanged types and behavior.

## Focused tests

- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint --tests`
  checks all renamed references and the checkpoint boundary.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the changed crate strictly.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment --test calibration --test physics --test behavior_contract`
  checks reaction, attachment, physical controls, and the new adapter.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-checkpoint`
  checks clone and replay preservation.

## Development loop

The representative warm regression is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment
--test calibration --test physics --test behavior_contract`; it must remain
strictly under 10 seconds.

## Controls and evidence

The new-adapter behavior suite, attachment state-preservation tests, engine link
identity tests, and checkpoint replay are held-out controls. Negative controls
are invalid attachment, failed enqueue, quiet identity, and distance rejection.
The change is falsified by a remaining private `.learning` field reference, a
changed test result, link/memory misalignment, or a warm run at ten seconds or
more. No new behavioral control is needed because the change is name-only.

## Risks and rollback

The only material risk is renaming one side of attachment or link construction
without the other. Compilation and attachment/checkpoint tests detect that.
Rollback restores the previous private identifier.

## Open decisions

None.
