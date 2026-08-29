# Composable embodiment drivers

```text
world -> sensor driver -> signal wiring -> actuator driver -> body change
                         ^                              |
                         +------ physical feedback ----+
```

## Outcome

Add a neutral `truelearner-embodiment` Rust crate for composing deterministic,
stateful sensor and actuator drivers. Prove its laws with small unit tests and
exercise the same API as binocular change sensing and five independent finger
axes. Reuse its pure channel, quantization, and opposed-effort operations in the
workstation without changing any learner, body, trace, checkpoint, or Academy
behavior. This is infrastructure evidence, not a binocular-depth or hand-control
capability claim.

## Authority

- Path: `arch.md` accepted body and boundaries; `academy.md` Body Discovery;
  `LANGUAGE.md`; `algo.md`; `AGENTS.md`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`

## Model

A driver is a deterministic state machine `state + input -> state + output`.
`Identity`, serial composition, parallel composition, and homogeneous banks
construct larger drivers without I/O, clocks, randomness, learner access, or
evaluator knowledge. A typed `Signal` carries an opaque physical origin and honest
`Sample` or `Transition` incidence through value mapping, routing, and fan-out.

`ChangeDetector` retains only the preceding typed value: the first observation and
an equal observation are samples; a different value is a transition. A bounded
axis consumes opposing effort, clamps its physical position, and returns actual
movement plus proprioceptive feedback bearing the input cause. Equal opposing
effort is the identity action. Invalid bank arity is a typed error and changes
no driver state.

The crate supplies mechanisms, not device nouns. Binocular and finger fixtures
exist only in tests: two composed detectors receive separate eye values, while
five identical bounded axes receive separate efforts. The workstation consumes
only the shared pure primitives initially; migration of retained retinal state,
morphology construction, or checkpoints is a separate change.

## Invariants

- Identity and serial association preserve outputs and retained state.
- Parallel factors and independent bank members do not share state or depend on
  evaluation order.
- Mapping, routing, and fan-out preserve physical origin and incidence; fan-out
  does not represent another admitted external input.
- A repeated value cannot become a transition, and a first sample cannot claim
  prior physical change.
- Bounded actuation reports actual clamped movement; equal opposing effort and
  saturation report no transition.
- Bank arity failure is atomic.
- Drivers contain no key, hand, eye, target, answer, score, capability, action
  route, evaluator state, wall time, randomness, Harness, or learner reference.
- Existing Production and research traces, checkpoint bytes, replay, natural
  quiescence, semantic firewall, and accepted hand behavior remain unchanged.

## Scope

- `truelearner/Cargo.toml`
- `truelearner/Cargo.lock`
- `truelearner/crates/embodiment/`
- `truelearner/crates/workstation/Cargo.toml`
- `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/workstation/src/state.rs`
- `academy/Cargo.lock`
- `research/experiments/workstation-return-bearing-opportunity-composition/Cargo.lock`
- `factory/receipts/`
- Excludes core learner code, Academy runtime code, checkpoint schema changes,
  morphology migration, new sensor physics, new actuator choices, and authority
  promotion.

## Development style

TDD. Write law and body-shape tests against the public driver API first, then
implement the smallest concrete types and functions that satisfy them. Refactor
the workstation only after the new crate tests pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment`
  proves identity, association, parallel independence, honest change detection,
  provenance preservation, atomic bank failure, bounded feedback, binocular
  composition, and five-finger independence.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  proves the workstation refactor preserves body, checkpoint, and replay tests.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation separate_eyes_receive_different_hand_projection`
  preserves real binocular hand rendering.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib visual_reach_moves_eyes_and_palm_toward_opposite_real_keys`
  preserves the current sensorimotor development witness.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; cold bootstrap
is recorded separately.

## Controls and evidence

Held-out composition uses three parallel sensor factors and seven bounded axes,
which are absent from the named binocular and five-finger tests. Negative controls
are repeated samples, equal opposing effort, lower/upper saturation,
wrong bank arity, reversed factor registration, the Academy semantic firewall,
and unchanged workstation visual-reach behavior. The falsifiers are any loss of
origin/incidence, shared factor state, non-atomic failure, invented movement,
checkpoint/replay change, or warm regression at or above ten seconds. Evidence
is a validated candidate receipt and an independent verification receipt; no
capability or authority receipt is produced.

## Risks and rollback

Generic machinery could obscure ownership or encode device meaning. Keep one
small `Driver` trait, concrete composition structs, explicit signal envelopes,
and ordinary domain names. If the workstation trace changes or the API needs
device labels, remove the dependency and crate; no persistence migration is
required.

## Open decisions

None.
