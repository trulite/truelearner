## Claim

Which experimental claim changes?

## Evidence

- New positive cases:
- New negative controls:
- Held-out conditions:
- Baseline:

## Leakage Audit

What information is available to the learner, evaluator, and test harness?

## Supplied Priors

Which representations, objectives, algorithms, thresholds, or curricula remain
hand-authored?

## Verification

```text
cargo fmt -- --check
cargo test --lib --bin organism-v0 --test reviewer_api
cargo clippy --all-targets -- -D warnings
```
