# Express organism acceptance through attached, calibrated bodies

```text
raw reading --calibrate--> sensor part --attach--> Body --run--> attached effect
```

## Outcome

Rewrite the 28 `truelearner-new-harness` body laws so physical morphology is
made from quiet bodies joined through ports, and raw sensor values are converted
to residual impulses by calibration. Remove hidden position, region, and
resistance metadata. Keep `Body::run` as the only organism-level composition
boundary and preserve every existing positive claim and negative control.

The claim is that the acceptance contract uses the new attachment and
calibration language. It does not claim that automatic recognition already
satisfies the higher learner laws.

## Authority

- Path: `LANGUAGE.md`, `lessons.md`,
  `truelearner/crates/body/src/attachment.rs`,
  `truelearner/crates/body/src/calibration.rs`, and
  `truelearner/crates/new-harness/tests/body_laws.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; authority digests
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`,
  `5b50453e4895e5a25c337555af167894cbfd4625d89837976815914aa21e1bb0`,
  `c63d1ef9995a462704a5b869070be8d266a9e28cb05983d94396c8e7ad25e4a3`,
  `5a8d6012813f02b845924718a9b7a2e783df7e67a95aa8aecccaca7713ebc2d2`,
  and `2f3084ee863b303c61b0e4ffb8bfe35edad279aeeabd68a85a98a39511d97133`.

## Model

The objects are a host body, sensor parts, actuator parts, ports, directional
joins, raw readings, calibrated residuals, physical arrivals, and observed
physical events. Sensor-to-output nearness is ordinary topology: a dormant
zero-impulse join whose delay records path length. An actuator is an outward
join from an organism opportunity to the attached effect. A calibrated reading
enters the attached sensor port; zero is quiet and a non-zero residual is a
physical input.

The composed episode is:

```text
reading -> residual -> attached surface -> recognition -> choice -> attached effect
```

The tests may build parts, expose ports, declare joins, calibrate readings,
submit arrivals, and observe events. They may not construct learner
interpretations or call `react` or `apply`. Failed higher laws must therefore
identify missing body-owned recognition rather than a test-side adapter.

## Invariants

- All 28 retained laws and their controls remain present.
- `Place`, position, region, and resistance metadata disappear from the suite.
- Sensors and effects are separate bodies before attachment.
- Calibration is the only raw-reading-to-impulse transformation.
- Attachment declares topology but adds no hidden motor map or learner meaning.
- No test imports or constructs `react`, `Context`, `Candidate`, `Closure`,
  `CyclePath`, `ReturnedOutcome`, `Surface`, `UsedPath`, `Owner`, or `Path`.
- Tests inspect only physical events, quiet state, held values, and work.
- Kernel and checkpoint suites remain unchanged.

## Scope

Update `truelearner/crates/new-harness/tests/body_laws.rs` and this plan only.
Do not change `truelearner-body`, checkpointing, the old core, workstation,
Academy, research evidence, workspace membership, or production selection.

## Development style

Use TDD. Rewrite the complete acceptance fixture in one pass, compile it, then
run the full suite once to retain the first body-owned falsifiers. Do not repair
missing automatic recognition in this change.

## Focused tests

- `cargo fmt --all -- --check` establishes formatting.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --test body_laws --no-run`
  establishes that the new physical contract compiles.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --test body_laws`
  records which retained higher laws the current body does not yet satisfy.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body` holds
  the compact physical kernel fixed.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --tests -- -D warnings -A clippy::obfuscated-if-else`
  establishes a warning-free test contract while allowing the unchanged engine
  style lint.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --test body_laws`.
Its execution must remain strictly under 10 seconds even while red.

## Controls and evidence

The unchanged `truelearner-body` suite is held-out evidence. Negative controls
remain repetition, expiry, mixed cause, absent or distant output, boundary
reentry, ambiguity, preopening, duplicate consequence, stale witness, dormant
growth, construction order, and disconnected composition. The rewrite is
falsified if hidden placement remains, any retained law disappears, calibration
is bypassed for a sensor reading, or tests manufacture learner interpretation.

## Risks and rollback

The main risk is making an inert topology link accidentally actuate tissue; use
a zero impulse and retain the no-output and distance controls. Another risk is
mistaking a red higher law for a malformed fixture; require compilation, kernel
regression, and passing primitive laws before interpreting learner failures.
Rollback restores the prior test file and plan; production code is untouched.

## Open decisions

None.
