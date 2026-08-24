# PX3 integrated MICRO execution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

- package: `arms/px3-integrated-micro-reversal`;
- preflight:
  `cargo run --manifest-path arms/px3-integrated-micro-reversal/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-integrated-micro-reversal/Cargo.toml --release -- --micro`;
- artifacts: `results/px3_integrated_micro_reversal_v1.csv` and `.md`,
  with corresponding hidden `.staging` paths.

Exactly two rows execute in seed order `3301,3309`, normal then mirrored
insertion, and each complete physical world executes twice for exact replay.
Preflight constructs no world, propagates nothing, writes nothing and emits no
evidence marker. Evidence emits `PX3_INTEGRATED_MICRO_REVERSAL_EVIDENCE` once
and atomically publishes even a negative result.

No correction, rescue or rerun follows the sole execution. D2 normalization,
X/Y/Z recursion, GATE, definitive evidence and authority are absent.
