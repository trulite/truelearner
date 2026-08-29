# Composable active sensing and effects

```text
available sensor fields -> independent signals -> simultaneous relations
        -> routed commands -> collect every source -> constrain body effect
        -> actual movement and honest sensory return
```

## Outcome

Extend `truelearner-embodiment` with the smallest complete construction shared by
natural sensor fields, clutter, occlusion, binocular relations, cross-modal reach,
and contact or fixation constraints. The library represents availability without
inventing a hidden value, preserves multiple simultaneous signals, composes paired
observations only when both are physically present, collects every command source
before an actuator constraint can be applied, and represents the identity body
effect explicitly. Migrate stable binocular fixation to that common effect boundary
without changing its trace. This is infrastructure evidence, not a claim that
natural-image correspondence, occlusion continuity, stereo depth, or reaching has
been learned.

## Authority

- Path: `LANGUAGE.md`; `research/constitution.md`; `lessons.md` lessons 66, 69,
  71, and 72; `research/campaigns/workstation-binocular-stable-fixation-v1/convergence.toml`;
  `plans/composable-embodiment-drivers.md`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; lessons SHA-256
  `2a050aa751f7f6cb6574c6530fdfb5d5425898f3ff40b83b7aa58015529b2ad6`;
  convergence SHA-256
  `d8d81a018c40b7e8f2c03c10d5416abc0269f59671977d3948584353911a5454`

## Model

`Availability` of a value is either `Available(value)` or `Unavailable`. Mapping preserves
availability, and zipping two observations produces a pair only when both are
available in the same step. Wrapped in the existing `Signal`, ordinary
`ChangeDetector` records disappearance and reappearance honestly without retaining
or reconstructing an occluded value. Existing banks, ports, routing, and fan-out
carry any number of natural-image, clutter, touch, auditory, or proprioceptive
signals without selecting a semantic target.

`CommandCollector` accepts zero or more commands at each opaque actuator
port and combines them with a caller-supplied associative operation. `finish`
consumes the collector and returns a fixed-arity `CommandFrame`; the type boundary makes it
impossible to add another source after physical constraints begin. `EffectMode`
selects either the collected command or `None`, the identity/no-effect case.
Constraints are port-local. The workstation uses component-wise bounded opposed
effort as its command operation, collects every learner output, finishes the frame,
and only then applies the centred-eye constraint.

The categorical laws are ordinary domain laws: unavailable mapped is unavailable;
available mapping preserves composition; pairing is local and requires both
participants; no command is the collection identity; command combination is
associative for bounded opposed effort; applying preserves the finished command;
identity removes it; and constraining one port cannot affect another.

## Invariants

- Absence never contains a stale or reconstructed sensor value.
- Natural fields and clutter may carry zero, one, or many independent signals; the
  library never chooses a target or correspondence identity.
- Paired observations use only values simultaneously available to the driver.
- Mapping, pairing, routing, collection, and effect constraint preserve physical
  locality and introduce no evaluator knowledge.
- Every command source for a port is combined before its body-effect constraint;
  the type lifecycle prevents late command insertion.
- An identity effect produces no physical movement and therefore no false movement
  return.
- The same command and constraint types compose two eyes, five fingers, and a
  held-out actuator count without device-specific branches.
- Stable binocular fixation, dark-eye exploration, other-eye independence,
  Production trace bytes, replay, natural quiescence, and the semantic firewall
  remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`
- `truelearner/crates/embodiment/tests/`
- `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/workstation/src/state.rs`
- `factory/receipts/`
- Excludes image feature extraction, salience or target selection, hidden object
  identity, retained last-seen values, learned correspondence, depth estimation,
  reach goals, morphology constants in the shared crate, learner changes,
  checkpoint changes, Academy promotion, and Production adoption.

## Development style

TDD. Add law tests and composed sensor/actuator fixtures first, implement only the
types required by those tests, then replace the eye-specific pre-collection skip
with the finished-frame constraint.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment`
  proves availability mapping and pairing, honest occlusion transitions, multiple
  simultaneous field signals, command identity and association, lifecycle,
  binocular locality, five-finger locality, and a held-out actuator count.
- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib stable_fixation_holds_all_mirrored_relations`
  proves the common effect boundary preserves all six fixation witnesses.
- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib fixation_controls_preserve_exploration_and_eye_locality`
  preserves dark-eye exploration and other-eye independence.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves workstation state, checkpoint, and replay behavior.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; record cold
bootstrap separately.

## Controls and evidence

Held-out cases are a three-cell field with two simultaneous available values, one
temporarily unavailable binocular factor, and seven actuator ports. Negative controls
are repeated unavailability, pairing with either side unavailable,
identity command collection, a constrained port receiving multiple sources,
neighboring unconstrained ports, invalid ports, unchanged stable-fixation bytes,
and unchanged Production visual trace bytes. Falsifiers are a stale occluded value,
implicit candidate selection, a constraint bypassed by a late source, loss of
origin/incidence, cross-port suppression, changed accepted traces, invented return,
or a warm regression at or above ten seconds. Evidence is a validated candidate
receipt and independent verification receipt; no capability or authority receipt
is produced.

## Risks and rollback

Generic machinery could become a framework or hide sensor meaning. Keep the
existing single `Driver` trait, use four small concrete types, accept combination
as an explicit function at collection, and reuse `Option` for physical identity.
If the API requires device nouns, stale memory, dynamic dispatch, or changes an
accepted trace, remove this extension and restore the previous harness integration;
there is no persistence migration.

## Open decisions

None.
