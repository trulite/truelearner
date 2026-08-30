# Bypass irrelevant body reactions

```text
changed meeting -> classify once -> irrelevant: continue
                               `-> relevant: react -> apply non-empty edits
```

## Outcome

Bypass internal reaction construction for physical moments that contain neither
a boundary capable of forming, choosing, or closing a return nor a completed
learned path. Remove scratch movement from that common path. Preserve physical
events, learning behavior, and public APIs.

## Authority

- Path: `truelearner/crates/body/src/{core,engine}.rs`, old/new black-box
  contracts, and release samples of `engine_cost` and `link_cost`
- Revision: content digests
  `93a4b21cc190bed4f48bc5b68595a4ec83b16a51f4e1ff95982c369e7dfaf56f`,
  `2d7cebeec544d52e926d42522b3a6f9ace2c2dcfd9a7ea7f539133339c95b987`,
  `6139ad5985177b579eb5465c5af4e9a2709643eac94e9e06fa0e5dc4f47b8ccf`,
  and `1dc04d8e957f07e51632e653db813646242a2c4abf0a5534fe5f0ae6331af564`.

## Model

Each changed meeting classifies its participants once as boundary input, no
completed path, one completed path, or many completed paths. The classification
is retained in the moment. A read-only `ReactionView` borrows arena topology,
link memory, and the live-return index separately from activity scratch. A
moment is relevant exactly when it has a completed path, or a boundary whose
surface has an existing path or local morphology, or any live return. Only
relevant moments enter reaction construction.

## Invariants

- Each changed meeting's participant chain is traversed exactly once.
- Every moment that could create, choose, return, retire, strengthen, or consume
  an edit remains relevant.
- Ordinary boundary firing in an inert body and ordinary drive propagation are
  irrelevant and bypass reaction construction.
- Ambiguous completed paths remain relevant and remain behaviorally inert where
  required.
- Return relevance reads `live_returns`; link memory remains authoritative.
- The common irrelevant path does not move or replace `ReactionScratch`.
- Relevant edit ordering and application semantics remain unchanged.
- Public body, event, reaction, adapter, and scenario APIs do not change.

## Scope

Change private classification, read-only reaction access, and step routing in
`truelearner/crates/body/src/{core,engine}.rs`; update private focused tests and
add receipts. Do not alter arena physics, learning laws, attachment, checkpoints,
public reactions, scenarios, adapters, or expectations.

## Development style

Use TDD: first require inert boundary and ordinary drive waves to leave reaction
scratch untouched while existing learning and ambiguity controls remain green.

## Focused tests

- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint --tests`
  checks private borrowing and checkpoint integration.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the changed crate strictly.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment --test calibration --test physics --test behavior_contract`
  checks the fast path, learning path, ambiguity, and compact adapter.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  checks the old harness contract.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-checkpoint`
  checks quiet cloning and replay.
- `cargo run --release --quiet --manifest-path truelearner/Cargo.toml -p truelearner-body --example engine_cost`
  measures inert boundary waves against the 40.01 ns median profile baseline.
- `cargo run --release --quiet --manifest-path truelearner/Cargo.toml -p truelearner-body --example link_cost`
  measures ordinary drive propagation against the 35.17 ns per-link baseline.

## Development loop

The representative warm regression is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-body --lib --test engine --test attachment
--test calibration --test physics --test behavior_contract`; it must remain
strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

The old/new black-box contracts, path reuse and return scenarios, ambiguous
participation, engine equivalence, attachment, and checkpoint continuation are
held-out controls. Negative controls are inert boundary waves, ordinary drive
waves, quiet identity, unmatched returns, distance rejection, and failed
enqueue. The change is falsified by a missed learning edit, changed output,
scratch growth on an irrelevant wave, a newly failing control, or meaningful
regression from either sampled baseline. Candidate, verification, and fresh
release profiles preserve evidence.

## Risks and rollback

The main risk is falsely classifying a potentially learning-bearing boundary as
irrelevant. The relevance predicate therefore over-approximates: uncertainty
enters the reaction path. Focused learning and ambiguity controls detect missed
edits. Rollback removes the relevance gate and restores unconditional internal
reaction construction.

## Open decisions

None.
