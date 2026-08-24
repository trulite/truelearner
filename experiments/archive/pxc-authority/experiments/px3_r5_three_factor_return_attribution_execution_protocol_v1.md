# PX3-R5 three-factor return attribution execution protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

- package: `arms/px3-r5-three-factor-return-attribution`;
- preflight:
  `cargo run --manifest-path arms/px3-r5-three-factor-return-attribution/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-r5-three-factor-return-attribution/Cargo.toml --release -- --r5`;
- artifacts: `results/px3_r5_three_factor_return_attribution_v1.csv` and
  `.md`, with corresponding hidden `.staging` paths.

For seeds `3601,3609`, row order is exactly:

1. `complete-pxr`;
2. `px-no-return`;
3. `pr-x-blocked`;
4. `xr-p-absent`;
5. `px-late-a-no-return`;
6. `adjacent-ab-no-return`;
7. `collision-real-return`;
8. `two-completed-pxr`.

Exactly 16 ordered rows execute from fresh state twice. Preflight performs
only frozen hash, matrix, refusal and artifact-absence checks. It constructs no
world, propagates nothing, writes nothing and emits no evidence marker.

Evidence emits `PX3_R5_THREE_FACTOR_RETURN_ATTRIBUTION_EVIDENCE_SPENT` once and
atomically publishes R5-A, R5-B or R5-C. The adjacent no-return result is not
repairable after execution. No tuning, rescue, regeneration or rerun follows.

All Rust execution occurs in E2B sandbox `i6x9gykt9tvp6xfz5z8ra`. R5 is a
development diagnostic, changes no PX0 law and performs no PX3 authority run.
