# Body-curried component calibration

```text
local body context + component observation -> residual relation
                                             |
                                             v
                             attached physical residual trace
                               |          |          |
                            smaller     larger      zero
                               |          |          |
                         return outcome  keep drive  identity
```

## Outcome

Add one generic `calibrate(body, relation)` transformation to
`truelearner-embodiment`. It captures local body context and returns a reusable
driver from any available component observation to a nonnegative residual. The
same driver shape must cover finger pressure, a five-finger product, binocular
offset, bilateral sound, a vocal spectrum, and a held-out structured value
without device-specific library branches.

Add one physical calibration mode to the existing attached threshold trace. A
nonzero residual fires one ordinary local drive surface; a smaller residual
returns through that same surface, a larger residual remains a truthful distinct
change without receiving outcome, and zero emits no drive. Conditionally climb
the existing two-sided scalar regulation rung, including reflected actuator
effects and shifted body context. This is discovery evidence for calibration
shape and bounded regulation only, not authority, arbitrary sensor assimilation,
speech, hearing, hand, eye, or whole-body competence.

## Authority

- Path: `LANGUAGE.md`; `research/constitution.md`; `lessons.md` lessons 54-72;
  `plans/composable-active-sensing-effects.md`;
  `plans/composable-perception-action-loop.md`;
  `research/campaigns/runtime-attached-natural-closure-v1/convergence.toml`.
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  lessons SHA-256
  `2a050aa751f7f6cb6574c6530fdfb5d5425898f3ff40b83b7aa58015529b2ad6`;
  natural-closure convergence SHA-256
  `f07ffd240767d9c3d2e6f5cc7228bae97ce3f4525061648ece20a4dacdaa1226`.

## Model

The input objects are local body context `B`, an available or unavailable
component observation `S`, and the observation's existing physical `Signal`.
The body supplies a pure relation `B x S -> Residual`; `Residual` is a
nonnegative saturating additive value with zero as quiet. Currying produces
`calibrate(B, relation): S -> Residual`. In Rust this is one concrete
`Normalizer` parameterized by body and relation types, implementing the existing `Driver` trait. It maps through
`Signal` and `Availability`, preserving physical origin, incidence, and honest
absence. It owns no clock, previous sample, effect identity, actuator mapping,
or evaluator state.

Zero residual maps to the existing identity effect and no opportunity; nonzero
residual maps to apply and an ordinary opportunity. Independent residuals
compose by saturating addition, while local normalizers remain factored when
their effects are independent. No generic category traits or sensor taxonomy are
introduced.

The physical implementation reuses `PhysicalTraceComponent`. Calibration mode
adds one port and one junction at the local attachment site. The port receives a
late ordinary sample only while residual is nonzero. Existing trace tissue
stores residual thresholds. Existing fall outputs feed the same local drive
junction before the late sample, so an action-caused smaller residual closes the
exact participating path first. Rise outputs remain physically distinct and do
not receive outcome. At residual zero there is no late drive; the final fall may
close the path and the body becomes quiet.

The runtime harness maps scalar position and a body-supplied band to residual;
the learner receives only anonymous trace incidence, residual drive, actual
outputs, and truthful physical returns. It never receives position, band bounds,
distance, direction, motor identity, or evaluator residence. Reflected actuator
effects and a shifted band test that calibration has not captured a motor map or
fixed center.

## Invariants

- `calibrate` takes body context and returns a function over component
  observations; it is not a method that asks a sensor to invent its own norm.
- Calibration preserves `Signal` origin and incidence exactly and maps
  `Unavailable` to `Unavailable` without evaluating the relation.
- Residual zero is identity and supplies no opportunity. A positive residual
  cannot name or select an actuator.
- Residual combination has zero identity and is associative under saturating
  addition.
- Independent factors retain independent normalizers and effect ports; a global
  aggregate may observe total residual but cannot erase local identities.
- All temporal comparison state used by the runtime organism lives in attached,
  checkpointed junction and link physics. `Normalizer` retains body context but
  no previous observation.
- A smaller actual residual may return consequence to the exact used action path.
  A larger residual remains a truthful transition but cannot strengthen that
  path. Equal nonzero residual keeps ordinary drive without fabricating change.
- The calibration drive is late enough that a returned fall closes before the
  next unresolved drive acts. Zero residual emits no drive or false return.
- Runtime calibration and attachment name no motor, direction, sensor modality,
  scalar position, target band, correct action, or evaluator result inside the
  core learner.
- Default protocols, existing trace builders, accepted serial articulation,
  checkpoint replay, natural quiescence, active-frontier work, and all retained
  negative probes remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`: `Residual`, `Normalizer`,
  `calibrate`, and one calibration build mode for existing physical trace tissue.
- `truelearner/crates/embodiment/tests/calibration.rs`: categorical laws plus
  finger, hand, eye, ear, voice, cross-modal, and held-out type rungs.
- `truelearner/crates/embodiment/tests/runtime_attachment.rs`: conditional
  two-sided, reflected-effect, shifted-context regulation using the opt-in exact
  natural-cycle closure parent.
- `research/campaigns/body-curried-component-calibration-v1/`: frozen protocol,
  three arms, retained evidence, and convergence.
- `research/programs/learner/program.toml` and `factory/receipts/`: converged
  frontier and machine-checkable software evidence.
- Excludes a sensor taxonomy, external history, a general metric trait, dynamic
  dispatch, motor maps, sensor callbacks into core, global body snapshots, core
  learner changes, new checkpoint state, semantic tasks, Academy or workstation
  adoption, authority promotion, and claims of real fingers, hands, eyes, ears,
  voice, speech, or arbitrary morphology.

## Development style

TDD. Add generic law and modality tests first, then the smallest pure curried
driver. Add failing physical calibration-trace tests before extending the
existing builder. Run the generic and cross-modality gates before the runtime
regulation rung; stop dependent runtime cases at their first failed physical
transition and preserve the trace without weakening the oracle.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test calibration calibration_laws_`
  proves currying, origin/incidence preservation, honest absence, zero identity,
  residual association, and context change.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test calibration calibration_ladder_`
  proves one unchanged transformation over finger, five-finger product,
  binocular, bilateral acoustic, vocal-spectrum, mixed, and held-out values.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_trace_`
  proves attached residual initialization, held nonzero drive, smaller-return
  routing, larger-transition separation, zero identity, checkpoint replay, and
  natural quiescence.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment regulation_body_curried_calibration -- --exact`
  conditionally tests two disturbances, reflected effects, shifted body context,
  exact replay, residence, and quiet after the lower gates pass.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`,
  `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`, and
  `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment --all-targets --all-features -- -D warnings`
  preserve the affected crates.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test calibration && cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment`.
Its measured warm duration must remain strictly under 10 seconds; cold bootstrap
is recorded separately.

## Controls and evidence

The complete candidate includes existing signal/availability laws, driver
composition, actual before/effect/after returns, command identity, physical trace
memory, runtime attachment, exact natural-cycle closure, learner construction,
and authoritative connected-component choice/opportunity composition. It
excludes retired sensor-local fixed completion gates, external previous values,
global action ancestry, raw tick windows, semantic goals, and named anatomy.

Negative controls are unavailable body reference, unavailable observation,
already-normal input, repeated equal residual, larger residual, an unrelated
signal origin, collapsed global hand residual, reflected actuator effects,
shifted context, silent attachment, and existing frozen negative probes. Held-out
cases are a structured enum, a different arity product, a shifted body band, and
reversed motor-effect assignment. Evidence includes lossless modality results,
physical trace events, scalar trajectories, exact replay, natural quiescence,
work, campaign convergence, and valid candidate and verification receipts.

Killing falsifiers are relation evaluation on unavailable input, lost origin or
incidence, nonzero acting as identity, zero producing an effect, cross-factor
suppression, larger residual receiving consequence, smaller residual acting
before its return closes, either disturbance failing residence, reflected effects
revealing a fixed motor map, shifted context revealing a fixed center, replay or
quiet failure, semantic leakage, or warm regression at or above 10 seconds.

## Risks and rollback

The main risk is disguising evaluator reward as body context. The runtime body
relation is explicit frozen morphology; the learner receives only its anonymous
physical residual surface, and reflection plus context-shift controls kill fixed
direction and fixed-center interpretations. A generic metric framework would be
larger than the evidence requires, so the caller supplies one ordinary pure
relation and the library introduces only `Residual` and `Normalizer`.

Temporal ordering may let unresolved drive act before a smaller return closes.
Dedicated trace tests require the fall-to-drive edge to precede the late active
sample. If existing junction phases cannot express that order, freeze the
physical arm as falsified rather than changing core scheduling. Rollback removes
the new driver, calibration build mode, tests, and opt-in campaign; no accepted
protocol, checkpoint schema, or existing builder requires migration.

## Open decisions

None.
