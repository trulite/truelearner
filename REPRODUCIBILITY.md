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
cargo run --release --bin scaling -- --output results/v14_5_scaling.csv
```

`rust-toolchain.toml` pins the compiler and installs `rustfmt` and Clippy.

## Determinism

Experiments use fixed integer seeds and an internal deterministic linear
congruential generator. They do not use wall-clock time, operating-system
randomness, network services, or external datasets.

Determinism makes regressions reproducible but does not provide statistical
confidence. Reviewers should replace or supplement fixed seeds through
independent integration tests.

## Expected Test Inventory

- 70 in-crate unit tests
- 4 public reviewer-API integration tests

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
- v14.5: deterministic work is fitted against observation count, active
  context, and topology size; event cascades are compared with subcritical
  branching-process theory; bounded associative recall is measured across a
  16x load range
- v14.6: repeated useful cascades compress into short concept routes, useless
  activity weakens, a newly introduced unstable loop is learned away, and the
  stabilization training sweep covers one through sixteen independent routes
- v16: one persistent cell-arrow-spike learner performs repeated-sequence
  induction, thirty-two-pair recall, and three-position needle retrieval while
  rejecting remapped and unknown queries
- v17: identical recurrence-guided consolidation preserves both memories'
  tested behavior and retains the same contexts, while the trie uses fewer
  links, less estimated storage, and less query work
- v18: a solvable renaming-invariant chain benchmark produces a clean negative
  result; the unchanged learner and trie both score zero on unseen symbols and
  depths while permanent memory continues growing with training examples
- v19: six permanent cells and four learned role-routing arrows answer twenty
  thousand held-out episodes containing four hundred thousand fresh opaque
  identities while temporary state is erased and permanent state remains
  fingerprint-identical

## Interpretation Boundary

These are deterministic synthetic experiments. They do not establish
performance on unrestricted environments or show that the remaining supplied
representations would emerge from raw physical data.
