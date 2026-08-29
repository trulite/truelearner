# Composable perception-action loop

```text
physical state -> sense -> outside choice -> finished command -> act
      ^                                                    |
      +---------------- sense actual result <--------------+
```

## Outcome

Add one neutral interaction cycle to `truelearner-embodiment` that composes any
physical state, one or many sensor readings, an outside learner or controller,
and one or many finished actuator commands. The cycle senses before action, lets
the outside chooser return either a command or identity, applies an available
command exactly once, and derives the return by sensing the resulting physical
state. Demonstrate the same construction with binocular orientation, five-finger
contact, temporary visual unavailability, and a held-out mixed morphology. Reuse
it inside bounded-axis actuation without changing accepted traces. This is
infrastructure evidence, not a claim of object recognition, purposeful gaze,
occlusion continuity, depth, reaching, or contact learning.

## Authority

- Path: `arch.md`; `LANGUAGE.md`; `research/constitution.md`; `lessons.md`
  lessons 62-72; `plans/composable-active-sensing-effects.md`;
  `factory/receipts/composable-active-sensing-effects-verification.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; active-sensing plan SHA-256
  `2a64d2907afe00225a9a1e5648d4b62bf2ac2adfafc1dddb1a31a0ccea70fe4a`;
  verification verdict `supported`

## Model

An interaction cycle owns no world meaning and no learner. It receives mutable
physical state plus three transformations: sense reads the state without changing
it; choose receives the current observation and returns an optional finished
command; act receives the state and a command and returns the actual physical
effect. The cycle returns the observation before action, optional actual effect,
and observation after action. The chooser may capture ordinary physical action
context, but the library supplies no target, score, object identity, coordinate,
or desired direction.

The absent command is the identity action: act is not called, while the physical
state is sensed honestly again. An available command is applied once, and its
return is never synthesized from the request; it comes from sensing the resulting
state. Arrays, tuples, `Availability`, `Signal`, `DriverBank`, and finished
`CommandFrame` values compose multiple sensors and actuators inside the same
cycle. Repeating the function closes feedback through the caller without hidden
loop memory.

`BoundedAxis` uses the cycle internally: position sensing surrounds one bounded
physical update, actual movement is the effect, and the after-reading becomes the
proprioceptive return. Its public result remains unchanged.

## Invariants

- Sense cannot mutate the physical state through the interaction API.
- Choose sees only the supplied observation and values explicitly captured by its
  caller; the library adds no evaluator or semantic input.
- An absent command never invokes act and represents the identity body action.
- An available command invokes act exactly once.
- The after-observation is read from the actual post-action physical state, never
  reconstructed from the requested command.
- Parallel sensor values remain distinct, and a local command changes only the
  physical components implemented by its actuator.
- Unavailable observations remain unavailable unless actual state change makes
  the sensor available again.
- Repeating a cycle carries state only through the explicit physical state,
  sensor/actuator closures, and outside learner; the cycle retains no hidden path,
  target, or last-seen value.
- Bounded-axis feedback, stable fixation, Production trace bytes, replay, natural
  quiescence, and the semantic firewall remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`
- `truelearner/crates/embodiment/tests/`
- `factory/receipts/`
- Excludes core learner code, workstation or Academy behavior changes, image
  feature extraction, salience, object identity, goal representation, hidden
  persistence, asynchronous device timing, checkpoint changes, authority
  promotion, and Production adoption.

## Development style

TDD. Add interaction-law and cross-morphology tests first, implement one result
type and one function, then refactor `BoundedAxis` to use the function while
preserving its exact public effects.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment`
  proves identity, one-shot actuation, actual post-action return, binocular
  locality, five-finger contact, visual unavailability, repeated feedback, and a
  held-out mixed morphology.
- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib stable_fixation_holds_all_mirrored_relations`
  preserves the accepted six-relation fixation witness.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
It must complete in under 10 seconds; record cold bootstrap separately.

## Controls and evidence

Held-out coverage uses two visual readings, one contact reading, and three actuator
ports in one state, distinct from the binocular and five-finger fixtures. Negative controls
are an absent command, a zero-effect command at a physical limit, one
unavailable eye, unrelated fingers, a chooser returning identity despite available
signals, exact stable-fixation bytes, exact Production visual-reach bytes, and the
semantic firewall. Falsifiers are act called for identity, act called twice, return
derived from request rather than post-state, cross-port mutation, stale visual
availability, hidden retained loop state, changed accepted traces, or a warm
regression at or above ten seconds. Evidence is a validated candidate receipt and
an independent verification receipt; no capability or authority receipt is
produced.

## Risks and rollback

A generic loop could absorb learner policy or become a framework. Keep one plain
function, one plain result type, caller-supplied closures, and explicit mutable
physical state. If integration needs device nouns, dynamic dispatch, implicit
memory, or changes `AxisEffect`, remove the cycle and restore the direct bounded
axis update; no persistence migration is involved.

## Open decisions

None.
