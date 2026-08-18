# Reproducibility

**Recorded:** 2026-08-18

**Rust:** 1.97.1

**External crate dependencies:** none

## Commands

```bash
rustc --version
cargo --version
cargo fmt -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo run --release
```

`rust-toolchain.toml` pins the compiler and installs `rustfmt` and Clippy.

## Determinism

Experiments use fixed integer seeds and an internal deterministic linear
congruential generator. They do not use wall-clock time, operating-system
randomness, network services, or external datasets.

Determinism makes regressions reproducible but does not provide statistical
confidence. Reviewers should replace or supplement fixed seeds through
independent integration tests.

## Current Expected Test Inventory

- 44 in-crate unit tests
- 2 public reviewer-API integration tests

## Current Strongest Results

- v8 hierarchical planning: 69 primitive expansions versus 2 hierarchical
  expansions in the built-in transfer case
- v9: four structural operators learned and transferred
- v10: five selected interventions identify five opaque action rules; the
  deterministic random baseline averages 13.2
- v11: the changed rule adapts in six relevant samples over a 78-sample
  continuous stream while both unchanged rules remain stable
- v12: five opaque actions compress into three causal classes and a
  three-step procedure with support 8 and compression gain 13
- v13: the unified loop selects five causal interventions and six recurring
  task traces; random trace sampling averages 26.1
- v14: three new action aliases calibrate and a learned procedure transfers
  without target-domain task demonstrations

## Interpretation Boundary

These are deterministic synthetic experiments. They do not establish
performance on unrestricted environments or show that the remaining supplied
representations would emerge from raw physical data.
