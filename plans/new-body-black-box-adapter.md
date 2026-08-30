# Run the shared behavior contract on the compact body

```text
shared Scenario -> NewBodyAdapter -> Body + attach/Join -> normalized Observation
```

## Outcome

Add a test-only adapter under `truelearner-body` that instantiates the existing
shared black-box morphology with the body-owned harness helpers and runs the
same scenario catalog used by the old harness. This change exposes behavioral
agreement or disagreement; it does not repair body physics or claim equality
when a scenario fails.

## Authority

- Path: `truelearner/crates/body/src/harness.rs`,
  `truelearner/crates/body/src/attachment.rs`,
  `truelearner/crates/body/src/engine.rs`,
  `truelearner/crates/behavior-contract/src/runner.rs`, and
  `truelearner/crates/behavior-contract/src/scenarios.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; content digests
  `786678a9e6f45479721b45fc4a4065cfc427755f69f0090e8aa6a1772cd69088`,
  `c63d1ef9995a462704a5b869070be8d266a9e28cb05983d94396c8e7ad25e4a3`,
  `8162cf58c7ed6907ef5ddd79900449cfe6d82d644b26284da5ab1b67a4ec4366`,
  `3d0c43549220fd354be1039d325830019ba75c8187bd4214038e8dd2b9e67b94`,
  and `e97c2e73d54c76d4451013fd48a872baf0303c555ac0318141bb79b35a8bb1e3`.

## Model

`NewBodyAdapter::build` lowers neutral nodes into junctions, declared motor
links into the existing `motor` helper, local relations into `attach_sensor`,
and boundary ports into private junction maps. `run` groups inputs by time,
sends caused arrivals, observes only outward junction events, and returns the
shared `Observation`. `save` and `restore` copy a naturally quiet body and its
private port maps. Unsupported morphology returns typed adapter errors.

The shared runner remains the sole evaluator. The adapter contains no expected
effects, scenario names, learning assertions, or branches that make a test
pass.

## Invariants

- Use the existing body-owned `motor` and `attach_sensor` helpers; add no second
  attachment mechanism.
- Do not change body physics, learning, attachment, harness helpers, shared
  scenarios, expected behavior, or the old adapter.
- Preserve causes, times, impulses, outward port identity, natural quiet, and
  learned state across episodes and checkpoint copies.
- Construction order and dormant nodes must not change normalized behavior.
- A mismatch remains a shared `ContractError::Mismatch`, never an adapter
  success override.

## Scope

Add a dev-dependency from `truelearner-body` to
`truelearner-behavior-contract`, a body integration-test entry, its private
adapter module, and candidate/verification receipts. Do not recreate the
removed `truelearner-new-harness` package or modify production Rust modules.

## Development style

Use TDD: add the adapter integration tests against the frozen scenario catalog,
implement only the lowering needed by the neutral format, then preserve and
report every behavioral failure without changing its oracle.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract --no-run`
  checks that the adapter and complete shared catalog compile.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract -- --nocapture`
  records exact compact-body agreement and disagreement.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract`
  holds validation, runner composition, and mismatch reporting.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  holds the accepted old-harness oracle.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body --tests`
  checks the body test boundary.

## Development loop

The representative warm regression command is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test
behavior_contract --no-run`; it must remain strictly under 10 seconds. Execute
the behavioral suite separately because disagreement is evidence, not a build
failure to conceal.

## Controls and evidence

Negative controls are quiet identity, distance outside the local radius, and
dormant parts. Held-out controls are the old-harness contract suite, shared
contract tests, the body library tests, construction reversal, cause changes,
and 1,024 dormant nodes. The adapter is falsified by private expected values,
dropped cause/time/impulse data, custom attachment, hidden mismatch, or a warm
compile gate at 10 seconds. Expected evidence is a validated plan, compiling
adapter, exact scenario result, unchanged old oracle, and validated receipts.

## Risks and rollback

The main risk is inferring a motor from arbitrary low-level morphology. Accept
only the physical motor shape represented by the current shared catalog and
return a typed error otherwise. Another risk is confusing an adapter failure
with a body-law failure; keep typed lowering errors distinct from shared
behavior mismatches. Rollback removes the dev-dependency and new test files.

## Open decisions

None.
