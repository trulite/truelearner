# PX3 recursive compression GATE execution protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT**.

- package: `arms/px3-recursive-compression-gate`;
- preflight:
  `cargo run --manifest-path arms/px3-recursive-compression-gate/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-recursive-compression-gate/Cargo.toml --release -- --gate`;
- artifacts: `results/px3_recursive_compression_gate_v1.csv` and `.md`,
  with corresponding hidden `.staging` paths.

Exactly four rows execute in seed order `3401, 3409, 3413, 3419`, crossing
normal/reversed insertion with forward/reflected distance-one proposal
geometry. Each complete continuing physical world and all of its checkpoint
controls execute twice for exact replay.

Preflight audits frozen inputs and the executable surface but constructs no
world, propagates nothing, writes nothing and emits no evidence marker.
Evidence emits `PX3_RECURSIVE_COMPRESSION_GATE_EVIDENCE` once and atomically
publishes even a negative result.

No correction, rescue or rerun follows the sole execution. Definitive evidence,
authority and any PX4 surface are absent.
