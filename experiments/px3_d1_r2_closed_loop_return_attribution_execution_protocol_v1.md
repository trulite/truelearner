# PX3-D1-R2 execution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

- package: `arms/px3-d1-r2-closed-loop-return-attribution`;
- preflight: `cargo run --manifest-path arms/px3-d1-r2-closed-loop-return-attribution/Cargo.toml --release -- --preflight`;
- sole evidence command: `cargo run --manifest-path arms/px3-d1-r2-closed-loop-return-attribution/Cargo.toml --release -- --r2`;
- artifacts: `results/px3_d1_r2_closed_loop_return_attribution_v1.csv` and `.md`, with corresponding hidden `.staging` paths.

For seeds `3001,3007`, exact row order is:

1. `return-only`;
2. `ab-real-loop`;
3. `ab-blocked-late-a`;
4. `ab-late-consequence`;
5. `no-ab-genuine-consequence`;
6. `ab-swapped-return-to-ac`;
7. `ab-then-cd-two-loops`;
8. `ab-real-loop-2-1`;
9. `ab-real-loop-4-4`.

Exactly 18 ordered unique rows execute twice. Preflight constructs no world and
writes nothing. Evidence emits `PX3_D1_R2_CLOSED_LOOP_RETURN_ATTRIBUTION_EVIDENCE`
once, publishes even a negative result atomically, and permits no D2/MICRO/GATE
surface. Every row records separate O->P connector and weak P->effect candidate
resistances so success cannot hide connector plasticity.
