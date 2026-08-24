# TrueLearner production workspace

`truelearner/` is the only production code surface.

The workspace contains two mechanically focused production packages:

- `truelearner-core`
  - `src/lib.rs`: physical state, resident execution, stable ID/slot
    resolution, compaction, and clocked checkpoint state;
  - `src/main.rs`: the production composition root.
- `truelearner-arena-format`
  - canonical little-endian SoA arena blocks;
  - immutable body manifests and content hashes;
  - validation and corrupt-input rejection;
  - no organism transition behavior.

The V1 runtime executes only from explicit mutable RAM. Durable arena blocks
are immutable, machine-independent bytes. A `BodyVersion` is structural and
timeless; quiescent and live checkpoints add the physical clock and the
transient state required by their restart contracts.

Production crates must not depend on anything under `experiments/`.
Experimental crates may depend on production crates.
