# AH0 handle-ordering removal implementation audit v1

Status: candidate complete; no retained matrix executed.

Parent: SI0 v2 development-positive `3f889bc`.

## Active runtime closure

The complete active runtime is now in:

`truelearner/crates/core/src/lib.rs`

The former private `mechanics.rs` module was inlined and deleted. The small
`src/main.rs` composition root remains an empty host binary and contains no
runtime state or transitions. `arena-format` remains the separate durable
representation crate.

## Causal ordering changes

- scheduler keys contain only tick, phase, causal wave where enabled, and
  serial;
- `CellId`, `ArrowId`, origin identity, and target identity were removed from
  scheduler selection;
- active CELL and required-junction membership use `HashSet`;
- outgoing adjacency is no longer sorted by `ArrowId`;
- firing, Modulation, and qualified-local transmission order ARROWs only by
  ordinary physical state where the retained sequential runtime needs an
  order;
- local proposals order candidates by geometry and ordinary CELL state, not
  handles;
- signed proposal alternatives are symmetrically `+1, -1`, independent of
  source/target numbering.

SI0 Drive incidence and causal-wave rules are unchanged.

## Remaining handle ordering

Numeric handle ordering remains only in explicitly classified non-causal
operations:

- canonical checkpoint serialization;
- durable-body decode with selectable resident packing;
- resident compaction;
- test/debug normalization.

`CellId` and `ArrowId` still implement `Ord` in `arena-format` because frozen
evaluators and the public durable-format API use ordered containers for
normalization. AH0 prevents `Ord` from entering causal code with a static
audit rather than changing that frozen public surface.

## Development checks already run

On the reusable E2B Rust worker:

- `cargo fmt --all -- --check` passed;
- strict release workspace check with all features passed;
- the unchanged SI0 v2 evaluator compiled;
- the R1-R5 targeted test compiled;
- R6 verification compiled.

No retained AH0 evidence world has executed. The frozen retained gate will run
once in a fresh E2B worker after this implementation is tagged.

RS2, CE1, FD2, ARC, authority, oracle status, and `arch.md` remain unchanged.
