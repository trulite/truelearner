# Express black-box worlds as sensors and motors

```text
World { sensors, motors, nearby } -> adapter -> organism -> episodes -> effects
```

## Outcome

Replace the shared generic node/link/port morphology with the physical parts
the tests actually exercise: sensors, motors, nearness, and typed input targets.
The old adapter must continue to pass every fixed and deterministic scenario
with unchanged expected behavior. The compact-body adapter must compile as a
direct composition of its existing `motor` and `attach_sensor` helpers, without
inferring motor shape from thresholds or links.

## Authority

- Path: `truelearner/crates/behavior-contract/src/model.rs`,
  `truelearner/crates/behavior-contract/src/scenarios.rs`,
  `truelearner/crates/behavior-contract/src/properties.rs`,
  `truelearner/crates/core/tests/behavior_contract/legacy.rs`,
  `truelearner/crates/body/tests/behavior_contract/new_body.rs`, and
  `truelearner/crates/body/src/harness.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; content digests
  `196d708f1efbb21d27bbffd61d49c391a3c63bc34930089f31e747bdea5c526b`,
  `e97c2e73d54c76d4451013fd48a872baf0303c555ac0318141bb79b35a8bb1e3`,
  `ab1891fdff1caa32c10c772e8e88af5be403e4d96747ed704808e03f2faec067`,
  `faa982c25a69b6006741a012397e709f0e6e00d2819f9015655056755fb15d8b`,
  `5feca3b2c095c97dbcc93aae511b09ac6391ad0a06cbd33624e88e92b0a74852`,
  and `786678a9e6f45479721b45fc4a4065cfc427755f69f0090e8aa6a1772cd69088`.

## Model

`Morphology` contains `Sensor`, `Motor`, and `Nearby`. A `BoundaryInput` targets
either a sensor or a motor's physical input; an `Effect` names a motor. Sensor
retention remains explicit. Motor internals and old outcome wiring remain
adapter-private.

Validation checks unique and known sensor/motor identities, valid retention,
positive nearness, valid input targets, monotone time, and checkpoint names.
Properties vary causes, reverse sensor/motor/nearness construction, add dormant
sensors, and vary distance. The runner and episode/checkpoint composition stay
unchanged.

## Invariants

- Shared scenarios expose no legacy protocol, junction, region, resistance,
  outcome source, compact-body junction identity, motor threshold, internal
  motor link, trigger, or attachment port.
- Old normalized effects, causes, times, quietness, learning, and checkpoint
  replay remain unchanged.
- The old adapter privately lowers motors and consequence wiring; the new
  adapter calls only existing body-owned physical helpers.
- A motor input is lowered privately to the old opportunity incidence and to an
  ordinary caused compact-body arrival.
- Observer purity, construction order, distance controls, and dormant-sensor
  controls remain active.

## Scope

Change the behavior-contract model, scenarios, properties, and their tests;
refactor the private old and compact-body adapters and their integration tests;
and add candidate/verification receipts. Do not change historical plans, old or
compact production physics, body-owned harness helpers, learning laws, or
expected effects.

## Development style

Use TDD: change the shared types and scenarios first, then refactor each adapter
until the frozen old suite passes and the compact-body suite compiles. Preserve
the compact body's existing behavioral failures as evidence rather than
changing expectations.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract`
  checks validation, composition, mismatch data, and deterministic properties.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  requires the complete old black-box oracle to remain green.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract --no-run`
  checks that the simplified compact-body adapter and catalog compile.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract -p truelearner-core -p truelearner-body --tests`
  checks all three boundaries.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract --tests -- -D warnings`
  and the warning-denying adapter integration crates check the refactor.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test
behavior_contract`; it must remain strictly under 10 seconds.

## Controls and evidence

Negative controls are quiet identity, distance beyond the local radius, invalid
references, and dormant sensors. Held-out controls are shared runner tests,
observer purity, construction reversal, cause variants, compact-body compile,
body kernel tests, and the 79 existing legacy regressions. The refactor is
falsified by any old scenario mismatch, a shared internal motor detail, motor
inference in the compact adapter, failure to compile either adapter, or a warm
old suite at 10 seconds. Expected artifacts are the validated plan, passing old
oracle, structural compact-body gate, and validated receipts.

## Risks and rollback

The risk is accidentally weakening the old oracle while changing identifiers.
Keep scenario times and expected effects byte-for-byte equivalent in meaning and
run the unchanged behavior assertions through the old harness. Rollback restores
the former generic morphology and both adapter lowerings.

## Open decisions

None.
