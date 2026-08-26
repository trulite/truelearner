# TC-DS0 implementation audit v1

Status: evaluator frozen before characterization evidence.

## Scope separation

- Active organism source changed from frozen ARC A2 candidate: no.
- `LOCAL_WINDOW = 4` changed: no.
- ARC world or curriculum invoked: no.
- Candidate de-supply mechanism implemented: no.
- Evaluator location: experiment-only standalone crate.
- Reference and production physical law: shared `truelearner-core`.

## Frozen hashes

```text
core lib.rs
d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58

evaluator main.rs
44cdb4b7f3854da038cb7c088873a9f5e84643f128b9fb9a31dd90417c9e865e

evaluator Cargo.toml
51c9beb4507e6fe6837c29a6689e93451fd8310dbc63c90acaa0ed58a5b87f57

static audit
133d109fcb6ba5155dd68cd421c36dfed2dfb666a82a45c0093df1a2de86837b
```

## Targeted E2B validation

Sandbox `i66lao4w60le4hybt1bov` ran only:

- evaluator rustfmt check;
- evaluator `cargo check`;
- evaluator strict Clippy with `-D warnings`.

All passed at candidate commit `934de5029414c19f41f61a006d4575a64dd2df98`.
The characterization executable has not run. No result artifact exists yet.

## Measurement boundary

The evaluator observes public physical transitions, causal `Work` counters,
durable arena state, physical clock, and canonical checkpoints. It reconstructs
eligibility-live only from emitted candidate-specific `Eligible` transitions,
candidate-specific resistance updates, deallocation, and physical time. It
does not access or mutate private runtime state.

The evaluator asserts exact reference/production equality before emitting each
pair. `ExecutionCost` is intentionally absent from physical comparison and
evidence.
