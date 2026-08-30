# Freeze old-organism behavior through shared physical scenarios

```text
shared physical scenario -> legacy adapter -> old Harness -> normalized behavior
```

## Outcome

Add a small implementation-neutral scenario crate and a new legacy-only test
suite that runs those scenarios through the existing `truelearner-core`
`Harness`. The shared surface exposes only physical construction, boundary
input, time, cause, effects, quiet, learning across episodes, and checkpoint
continuation.

The first suite freezes the old organism's core story: quiet identity, local
formation and action, rejection outside the local radius, consequence-driven
learning, later reuse from the surface input alone, observation purity, and
checkpoint replay. It establishes a reusable old-harness oracle; it does not
yet test the new body or claim complete equivalence across every historical
protocol.

## Authority

- Path: `LANGUAGE.md`, `truelearner/crates/core/src/core.rs`,
  `truelearner/crates/core/tests/harness_boundary.rs`,
  `truelearner/crates/core/tests/runtime_attachment.rs`, and
  `truelearner/crates/new-harness/tests/body_laws.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; authority
  digests `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`,
  `a6de6ef89712eb3908f1eadbe8df99d386cfa9c88fc77eba308635ffed822269`,
  `b2ea8c33a6a6c4f8466ddb439b4eff7e736fea4e40c268c0db77284ccb7097c8`,
  and `a98e6b3790fcef02cdf21ed12190278fb8efd48353f2cd05951888b64b8605fa`.

## Model

Create `truelearner-behavior-contract` with no dependency on either organism
implementation. Its public model is ordinary Rust data:

```text
Scenario
  morphology: nodes + directed physical joins + named boundary ports
  episodes: time + boundary readings + cause + moment limit
  expectations: normalized effects + quiet + retained behavior
```

Nodes expose only integrating threshold or sampled lifetime. Joins expose only
source port, destination port, delay, impulse, and trigger. Boundary ports are
typed as inward or outward. A scenario never contains legacy junction IDs,
physical IDs, absolute positions, regions, resistance, `Protocol`, learner
IDs, paths, candidates, outcomes, or expected internal graph edits.

The crate owns four transformations:

- `validate`: reject unknown ports, invalid thresholds or lifetimes, backward
  episode time, duplicate names, invalid joins, and references to unknown
  checkpoints before any adapter runs.
- `run`: apply a valid scenario through an `Adapter` and return normalized
  episode observations.
- `assert_expected`: compare only effects, effect time, cause, natural quiet,
  and later behavior named by the scenario.
- deterministic property variants: reorder construction, vary external
  causes, vary locality distance, and add dormant physical parts.

The legacy test adapter is private to `truelearner-core/tests`. It lowers
neutral nodes, joins, and ports into `HarnessBuilder`, maps outward ports to the
legacy outward region, derives deterministic legacy placement from declared
local joins, sends boundary activity through `send_physical`, and projects
`Run` into the normalized observation. Legacy `Protocol`, outcome-source
wiring, absolute identities, regions, and checkpoint bytes remain inside this
adapter.

The old test history contains many protocol arms rather than one runtime that
simultaneously embodies every retained law. The private adapter therefore uses
named legacy profiles selected by the test catalog, while the shared
`Scenario` remains protocol-free. Each profile names the established old
protocol and physical lowering needed to reproduce that law. A later new-body
adapter must run the same scenarios without seeing those profiles.

Checkpoint operations are explicit scenario steps: save under a local name,
restore that name, then continue with the same episode. Checkpoint bytes and
private observations are never part of the shared expected value.

## Invariants

- `truelearner-behavior-contract` depends on neither `truelearner-core` nor
  `truelearner-body`.
- Shared scenarios contain physical inputs and expected behavior, never the
  learner interpretation that would make the expectation true.
- Every old-only position, region, resistance, identity, protocol, diagnostic,
  and outcome-source detail stays in the legacy adapter.
- Fixed scenarios and generated variants use the same runner and normalized
  observation type.
- Generated variants are deterministic from an explicit case number and print
  that number on failure; no random dependency is required in the first suite.
- Assertions compare public behavior, not internal graph shape, allocation
  order, raw work counters, or checkpoint encoding.
- Observer enablement cannot change normalized behavior.
- Invalid scenarios fail before constructing or mutating an organism.
- Existing legacy tests remain unchanged and continue to pass.
- The representative warm suite remains strictly under 10 seconds.

## Scope

Add `truelearner/crates/behavior-contract` with `model`, `observation`,
`runner`, `scenarios`, and `properties` modules. Add it to the TrueLearner
workspace. Add a dev-dependency from `truelearner-core` and a new organized
integration-test entry under `truelearner/crates/core/tests/behavior_contract`.
Add a candidate receipt under `factory/receipts` after implementation.

The initial fixed catalog covers the core physical-learning story and its
negative controls; it need not port all 69 legacy harness and 10 runtime
attachment tests in this change. Do not modify old production body or harness
code, existing tests, the new body, `new-harness`, workstation, embodiment,
Academy, research programs, protocol behavior, or checkpoint format. Do not add
JSON, serialization, snapshots, golden files, property-testing dependencies,
or a new-body adapter yet.

## Development style

Use TDD. First write contract validation and runner tests using a tiny fake
adapter. Then implement the legacy adapter and port one fixed scenario per
behavior family. Finally add deterministic property variants around the same
catalog and run all existing legacy regressions unchanged.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract`
  checks scenario validation, normalized comparison, fake-adapter composition,
  and deterministic property variants.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`
  runs the fixed and generated black-box suite through the old `Harness`.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary --test runtime_attachment`
  holds all 79 existing legacy laws unchanged.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract -p truelearner-core --tests`
  checks dependency direction and both test boundaries.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-behavior-contract --tests -- -D warnings`
  checks the shared model without weakening warnings. The legacy integration
  test crate declares `#![deny(warnings)]` and is checked separately with
  `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-core
  --test behavior_contract`, isolating its strict lint gate from unrelated
  warnings in an already-dirty production tree.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-core --test behavior_contract`.
Its measured budget is strictly under 10 seconds; record cold compilation
separately from the warmed test duration.

## Controls and evidence

Negative controls include quiet input, distance outside the local radius, and
dormant parts. Held-out cases are all 69 existing `harness_boundary` tests, all
10 existing `runtime_attachment` tests, protocol reads, canonical checkpoint
tests, strict Clippy, and dependency inspection.

The change is falsified if a shared type exposes a legacy protocol or internal
identity, a scenario supplies a path or learning decision, the adapter changes
old production code, generated cases are not replayable, observer settings
change normalized results, an invalid scenario mutates state, any existing old
test fails, or the warm contract suite reaches 10 seconds. Expected evidence is
the validated plan, passing fixed and generated legacy contract suite, complete
legacy regressions, and a validated candidate receipt.

## Risks and rollback

The main risk is hiding old semantics in the shared format. Keep physical world
construction in scenarios but place all old protocol selection, outcome-source
registration, placement, region, resistance, and identity translation in the
legacy adapter. Another risk is freezing incidental traces; normalize only
external effects, time, cause, quiet, and cross-episode behavior. A third risk
is pretending the union of old protocol arms is one old runtime; retain the
private profile name in test failure output and make no such claim.

Rollback removes the contract crate, workspace/dev-dependency entries, and new
legacy integration test. Existing old implementation and tests are untouched.

## Open decisions

None.
