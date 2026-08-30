# Index live return links

```text
link_memory (authority) -> live_returns: Vec<ReturnEntry> (derived index)
```

## Outcome

Add the requested `live_returns` vector to the compact body. Keep it sorted and
duplicate-free by `(cause, LinkId)`, and use it to find live return links without
scanning every link's memory. Preserve existing body and adapter behavior.

## Authority

- Path: `truelearner/crates/body/src/{core,engine,attachment}.rs`
- Revision: content digests
  `217ecec281caf4132121bf192b1c88792304aa126a856aedf3014ae62e7da86a`,
  `96925a5fe0ab5f75fa7db20415c9230290b30867d3ed9f088817c59751b14d18`,
  and `f798324229cf70c6751625adc3b017b8ab820f3fdd9be02d40651ffecefc1aa2`.

## Model

`LinkMemory::role` and `LinkMemory::live` remain authoritative. A
`ReturnEntry { cause, link }` is a persistent derived projection of each live
`LinkRole::Return`. Link creation, retirement, role changes, and attachment
maintain the projection. Cloning and checkpointing preserve it as body state.

## Invariants

- `live_returns` contains exactly one entry for every live return-role link and
  no entry for any other link.
- Entries are sorted and unique by `(cause, LinkId)`.
- Return lookup reads the index and does not scan all `link_memory` entries.
- `link_memory` remains authoritative; the index contains no independent
  learning or outcome state.
- Attachment remaps indexed `LinkId`s by the same link offset as the arena and
  link memory.
- Existing black-box behavior and public body APIs do not change.

## Scope

Change private code in `truelearner/crates/body/src/{core,engine,attachment}.rs`,
add focused invariant tests, and add candidate and verification receipts. Do not
change the black-box scenario format, adapters, public APIs, physical reaction
rules, arena representation, or behavior expectations.

## Development style

Use TDD: add focused tests for insertion, removal, sorting, retirement, and
attachment remapping before replacing the full-memory return scan.

## Focused tests

- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint --tests`
  checks body and checkpoint integration.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the changed crate strictly.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment --test calibration --test physics --test behavior_contract`
  checks index invariants, reaction, attachment, physical controls, and both
  black-box paths.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-checkpoint`
  checks clone and replay preservation.
- `cargo run --release --quiet --manifest-path truelearner/Cargo.toml -p truelearner-body --example engine_cost`
  measures one warmed primitive wave.

## Development loop

The representative warm regression is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment
--test calibration --test physics --test behavior_contract`; it must remain
strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

The old and new black-box adapter suites, attachment tests, engine tests, and
checkpoint replay are held-out controls. Negative controls include role change,
link retirement, invalid attachment, failed enqueue, quiet identity, and
distance rejection. The change is falsified by an indexed non-return or retired
link, a missing live return, an unsorted or duplicate index, changed behavior,
or a warm regression at ten seconds or more. Candidate and independent
verification receipts record exact commands and digests.

## Risks and rollback

The material risk is index drift at a mutation boundary. Focused invariant
tests and debug assertions detect drift. Rollback removes the derived field and
restores the previous `link_memory` scan without changing authoritative state.

## Open decisions

None.
