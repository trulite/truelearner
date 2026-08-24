# R1-R5 Mechanical Optimization Handoff

## Status

Development equivalence is positive for R1 through R5. The physical law remains
singular, the slow reference path remains available permanently, and every
mechanical prefix is independently selectable.

The implementation is ready for review and for a separately frozen production
selection/authority decision. It is not yet appropriate to delete the reference
path or declare the full R5 configuration the default.

## Review targets

- `truelearner/crates/core/src/mechanics.rs` — scheduler and resident stores
- `truelearner/crates/core/src/lib.rs` — singular law, mechanical selection,
  frontier/adjacency integration, exact batching, differential unit coverage
- `experiments/verification/boundary-buffers-v1/src/main.rs` — accepted-corpus
  differential runner and first-divergence reporting
- `experiments/protocols/r1_r5_mechanical_optimization.md` — frozen protocol
- `experiments/results/r1_r5_mechanical_optimization_development.md` — result

## Next engineering decisions

1. Optimize the SoA access path without changing its durable representation.
2. Instrument allocation and bytes-touched counters before using them for
   optimization decisions.
3. Decide whether SIMD is worth a separate R5-S discriminator; it is not part
   of the current result.
4. Keep full-substrate transactional cloning as the correctness path until the
   optimized execution machinery has a separately accepted rollback design.

## Repository state boundary

The untracked root file `academy.md` predates this work and remains untouched.
