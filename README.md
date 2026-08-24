# TrueLearner

This branch contains the authoritative Physical Body V1 production organism
and separates it from its research history.

- `truelearner/` is the complete production Rust workspace.
- `experiments/` contains archived research code, protocols, evaluators,
  generated evidence, and audit tooling.
- `arch.md` is the accepted PXR0/PX-C + Physical Body V1 architectural oracle.

Physical Body V1 preserves the authoritative PX-C physics and adds stable
arena identity, canonical durable bodies, compaction invariance, and clocked
quiescent/live restart. Boundary Buffers V1 adds bounded FIFO staging for
`SpikeInput` and outward `Crossing` values, transactional backpressure, and
exact buffered live continuation. Cold storage, visual framebuffers, and
distributed execution remain future successors.
