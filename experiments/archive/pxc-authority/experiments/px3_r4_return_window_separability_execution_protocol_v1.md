# PX3-R4 return-window separability execution protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

- package: `arms/px3-r4-return-window-separability`;
- preflight:
  `cargo run --manifest-path arms/px3-r4-return-window-separability/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-r4-return-window-separability/Cargo.toml --release -- --r4`;
- artifacts:
  `results/px3_r4_return_window_separability_v1.csv` and `.md`, with
  corresponding hidden `.staging` paths.

For seeds `3501,3509`, exact row order is:

1. lawful-return ticks `0,1,2,3,4,5,6`;
2. renewed-input offsets `1,2,3,4,5,6`;
3. same-tick collision with return tick 3 and recurrence offset 3.

Exactly 28 ordered unique rows execute twice. Preflight audits only static
hashes, matrix identity and artifact absence. It constructs no world,
propagates nothing, writes nothing and emits no evidence marker.

Evidence emits
`PX3_R4_RETURN_WINDOW_SEPARABILITY_EVIDENCE_SPENT` exactly once and atomically
publishes the measured `R4-A`, `R4-B` or `R4-C` classification. Even an
uninterpretable or surprising result is published unchanged. No rescue,
tuning, regeneration or rerun follows the sole execution.

Execution uses the persistent E2B development sandbox
`i6x9gykt9tvp6xfz5z8ra`. No Rust command executes locally. R4 changes no PX0
law, performs no PX3 authority retry and cannot convert the frozen definitive
negative into authority.
