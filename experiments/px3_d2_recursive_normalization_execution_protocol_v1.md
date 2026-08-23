# PX3-D2 recursive normalization execution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

- package: `arms/px3-d2-recursive-normalization`;
- preflight: `cargo run --manifest-path arms/px3-d2-recursive-normalization/Cargo.toml --release -- --preflight`;
- sole evidence command: `cargo run --manifest-path arms/px3-d2-recursive-normalization/Cargo.toml --release -- --d2`;
- artifacts: `results/px3_d2_recursive_normalization_v1.csv` and `.md`, with corresponding hidden `.staging` paths.

Seed `3101` uses normal insertion and seed `3109` mirrored insertion. Exact row
order per seed is:

1. `a-alone`;
2. `b-alone`;
3. `a4-alone`;
4. `a-repeated`;
5. `ab-mature-1`;
6. `ab-mature-2`;
7. `ab-mature-4`;
8. `x-plus-c-overlap`;
9. `x-then-c-gapped`;
10. `d-plus-c-primitive-baseline`.

Exactly 20 ordered unique rows execute twice. Preflight constructs no world and
writes nothing. Evidence emits `PX3_D2_RECURSIVE_NORMALIZATION_EVIDENCE` once
and publishes atomically even on failure. No learning, R2, Y/Z recursion,
MICRO/GATE or authority path exists.
