# TC-DS1 continuous path participation implementation audit v1

Status: candidate and evaluator frozen before matrix execution.

## Scope

- Candidate state is feature-gated behind `tc-ds1` and absent from default
  builds.
- Actual ARROW traversal alone adds one universal fixed-point impulse.
- Elapsed physical time alone applies the universal `15/16` relaxation.
- Participation does not affect pressure, resistance, coupling, firing,
  retained eligibility, or plasticity.
- The retained `eligible_until` law and `LOCAL_WINDOW = 4` remain active.
- No ARC world, pressure repair, authority workflow, oracle, or `arch.md` was
  invoked or changed.

The first targeted feature compile exposed one mechanical omission: the new
ARROW-local scalar was present in the AoS value but absent from the already
supported SoA resident store. The frozen candidate carries the same scalar in
both layouts. This changes representation coverage only; the physical rule is
singular and shared by Reference and Production.

## Frozen hashes

```text
core lib.rs
71d9989df2b662c3086e55e40512a67861ee4f7f44d2d9c1a48eed70ad3402e2

core mechanics.rs
476e282e954f311cbd7163ed6f564b5c48d2b46db1d47e70a8094d574c7f8442

core Cargo.toml
86e21ea8c89fc43287ec81648c9e09d1f84545c778ed9859c9dcbdd5a94edbf6

evaluator main.rs
e179cb9f54d82c7ef76e52f179067d5860208c548f1a1da3b2b0e167d6622561

evaluator Cargo.toml
07f6cbd7062c6c8c7a0f0827e8e99cc6da288dad3f9e1d282da35713ee848f41

static audit
8be2615da61aa3d481b6ccdea53017e248622be54efc379429057dda95fe81a0
```

## Targeted E2B validation

Reusable development sandbox `in98yfaf846rubbbwcvp9` ran only:

- core and evaluator rustfmt checks;
- evaluator feature strict release Clippy with `-D warnings`;
- default-feature core release tests: `15/15`;
- default-feature core strict release Clippy with `-D warnings`.

All passed at candidate commit
`65a0f5097d2e9d4d3b9b6246227cc67f72966087`. The TC-DS1 matrix has not run and
no result artifact exists at this freeze.

## Measurement boundary

The evaluator observes feature-gated path-local magnitudes and causally inert
contact events in addition to the retained physical history. It cannot mutate
participation, select a contacted ARROW, redirect modulation, or change the
retained plasticity law.

Reference and Production execute one shared candidate rule. The matrix must
assert exact equality of their future-relevant physical histories before
serializing each pair. Raw cross-mechanics checkpoint hashes and
`ExecutionCost` remain outside physical equivalence for the reasons frozen by
TC-DS0.
