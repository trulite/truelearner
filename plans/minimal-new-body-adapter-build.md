# Make compact-body construction attachment-only

```text
Morphology -> attach motors -> attach sensors -> NewOrganism
```

## Outcome

Make `NewBodyAdapter::build` a small orchestration method whose only work is to
create an empty organism, attach every declared motor, attach every declared
sensor, and return it. Keep identifier lookup, outward-effect observation, and
nearby-motor resolution private to `NewOrganism` attachment methods.

## Authority

- Path: `truelearner/crates/body/tests/behavior_contract/new_body.rs`,
  `truelearner/crates/body/src/harness.rs`, and
  `truelearner/crates/behavior-contract/src/model.rs`
- Revision: content digests
  `112ede7b4762f883092a7fe00a5fc60f9cc47b3a1024d14cd8d1467c5e323d6a`,
  `786678a9e6f45479721b45fc4a4065cfc427755f69f0090e8aa6a1772cd69088`,
  and `c4455c6185b27e169e0b9e14b7e9444eb58c564e96c2563ba36423229b297830`.

## Model

`NewOrganism` owns a compact `Body` plus the private boundary handles needed to
address sensors, stimulate motor opportunities, and observe motor effects. Its
`attach_motor` and `attach_sensor` transformations extend that state while
preserving existing identifiers and physical attachment behavior. `build` is
only the ordered fold of those two transformations over the morphology.

## Invariants

- `build` contains no capacity calculation, map insertion, junction conversion,
  nearness resolution, or outward-effect wiring.
- Motors are attached before sensors so sensor nearness can resolve motor
  opportunity handles.
- The shared scenario format and all production body physics remain unchanged.
- Existing old-body results, compact-body compilation, and known compact-body
  behavioral failures remain unchanged.
- Duplicate and unknown-reference failures stay typed and defensive even though
  shared scenario validation normally rejects them first.

## Scope

Refactor only `truelearner/crates/body/tests/behavior_contract/new_body.rs` and
add candidate and verification receipts for this plan. Do not change the shared
contract, body-owned harness, production physics, scenarios, or expectations.

## Development style

Use implementation-first because this is a behavior-preserving private
extraction already covered by the old oracle and strict adapter compile/lint
gates.

## Focused tests

- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check` checks
  formatting.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body --tests`
  checks the body test boundary.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract -- -D warnings`
  checks the adapter under strict linting.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract --no-run`
  checks compact-body contract compilation without redefining its current
  behavioral evidence.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  checks the complete old-body oracle.

## Development loop

The representative warm regression suite is `cargo test --manifest-path
truelearner/Cargo.toml -p truelearner-core --test behavior_contract`; it must
remain strictly under 10 seconds.

## Controls and evidence

The old-body suite is the held-out behavioral control. Strict compact-adapter
lint and compilation are structural controls. The change is falsified if
`build` still performs bookkeeping, production code changes, the compact
adapter stops compiling, or the old oracle changes. Expected artifacts are the
validated plan and validated candidate and verification receipts. No new
negative control is needed because no behavior or validation rule changes.

## Risks and rollback

The risk is losing one of the private sensor, opportunity, or effect mappings
during extraction. Compilation and the old oracle detect boundary drift;
focused adapter tests detect malformed construction. Rollback restores the
single inline `build` implementation.

## Open decisions

None.
