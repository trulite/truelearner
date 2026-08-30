# Make local link composition available to physical reaction

```text
source <- LinkId -> destination      emission -> link-local cause and time
```

## Outcome

Add exactly two body capabilities: traverse a link's local incidence in both
directions without scanning the arena, and retain the cause and time of each
link's latest admitted transmission. Preserve physical behavior, public APIs,
reaction laws, and application semantics.

## Authority

- Path: `truelearner/crates/body/src/arena.rs`,
  `truelearner/crates/body/src/engine.rs`, and
  `truelearner/crates/body/src/core.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a` with the current
  uncommitted compact-body evidence-preservation candidate as base.

## Model

Every physical link stores its source and destination. The arena keeps stable
incoming adjacency beside its existing outgoing adjacency; link construction
and body attachment remap both directions. A successful link enqueue records
that link's cause and source time in its existing memory plus one validity bit.
Boundary input records no link transmission. `react` and `apply` remain
unchanged.

## Invariants

- Incoming and outgoing incidence describe the same link exactly once.
- Incoming iteration preserves link construction order.
- Appending an arena remaps source, destination, incoming heads and tails, and
  incoming-next identities exactly once.
- Junction and link propagation slots remain 32 bytes.
- Only a successfully enqueued link transmission updates transmission memory.
- Boundary input updates no link memory.
- Recording a transmission is `O(1)`; local traversal is `O(local degree)`;
  dormant links add no wave work.

## Scope

Change `truelearner/crates/body/src/arena.rs`, transmission recording in
`truelearner/crates/body/src/engine.rs`, the private `LinkMemory` representation
in `truelearner/crates/body/src/core.rs`, and focused unit tests. Do not change
`react`, `apply`, attachment or calibration APIs, public events, acceptance
tests, checkpoints, or arena slot alignment.

## Development style

Use TDD for stable reverse incidence, arena append remapping, ordinary
transmission, applied send, boundary identity, and delayed multi-link
participation. Then run all body and checkpoint regressions.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body arena::tests`
  checks bidirectional incidence, append remapping, and 32-byte slots.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body engine::tests`
  checks boundary identity and cross-frontier transmission cause and time.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint`
  checks held-out body, attachment, calibration, and checkpoint behavior.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the compact implementation.

## Development loop

The representative warm regression is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-checkpoint`.
It must remain strictly under 10 seconds.

## Controls and evidence

Focused controls cover two incoming links, a delayed two-link chain, and an
applied send. Negative controls are boundary input and a link that fails to
enqueue. Attachment, construction-order independence, dormant-body work,
checkpoint continuation, and slot size are held-out cases. The change is
falsified by an incorrect source, reordered incidence, stale remapped identity,
memory updated without emission, any public behavior change, or work
proportional to dormant arena size.

## Risks and rollback

The main risks are corrupting incoming identities during append and recording
transmission before enqueue succeeds. Build incoming adjacency in lockstep with
outgoing adjacency and record only after successful enqueue. Rollback restores
the three source files and focused unit tests.

## Open decisions

None.
