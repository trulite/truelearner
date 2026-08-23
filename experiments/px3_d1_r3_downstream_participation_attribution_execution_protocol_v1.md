# PX3-D1-R3 execution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

- package:
  `arms/px3-d1-r3-downstream-participation-attribution`;
- preflight:
  `cargo run --manifest-path arms/px3-d1-r3-downstream-participation-attribution/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-d1-r3-downstream-participation-attribution/Cargo.toml --release -- --r3`;
- artifacts:
  `results/px3_d1_r3_downstream_participation_attribution_v1.csv` and `.md`,
  with corresponding hidden `.staging` paths.

For seeds `3201,3209`, exact row order is:

1. `return-only`;
2. `ab-real-subthreshold`;
3. `ab-blocked-late-a`;
4. `ab-blocked-effect`;
5. `no-ab-independent-effect`;
6. `ab-real-no-return`;
7. `ab-real-late-return`;
8. `ab-then-cd-two-completed`;
9. `ab-suprathreshold-return-control`.

Exactly 18 ordered unique rows execute twice. Preflight constructs no world,
propagates nothing, writes nothing and emits no evidence marker. Evidence emits
`PX3_D1_R3_DOWNSTREAM_PARTICIPATION_ATTRIBUTION_EVIDENCE` once and publishes
even a negative result atomically. No rescue, tuning or rerun follows the sole
execution. D2, formation, persistence, MICRO, GATE and authority are absent.
