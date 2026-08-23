# PX3-R6 execution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

- package: `arms/px3-r6-return-triggered-trace-readout`;
- preflight: `cargo run --manifest-path arms/px3-r6-return-triggered-trace-readout/Cargo.toml --release -- --preflight`;
- sole evidence command: `cargo run --manifest-path arms/px3-r6-return-triggered-trace-readout/Cargo.toml --release -- --r6`;
- atomic artifacts: `results/px3_r6_return_triggered_trace_readout_v1.csv`
  and `.md`, plus hidden `.staging` paths.

Seeds `3701,3709` execute the six protocol rows in declared order, each twice,
for 12 rows. Preflight constructs no world and emits no evidence marker.
Evidence emits `PX3_R6_RETURN_TRIGGERED_TRACE_READOUT_EVIDENCE_SPENT` once.
No rescue, tuning, regeneration or rerun is permitted. All Rust execution is
in E2B sandbox `i6x9gykt9tvp6xfz5z8ra`.
