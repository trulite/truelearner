# Workstation focused vision body integration

```text
world light + body gaze -> focused receptor frame -> retained body change state
                                                       |
                                                       v
                                  changed receptors -> owned Workstation Harness
```

## Outcome

Add the established focused receptor field as an optional research morphology of
the real `WorkstationHarness`. The workstation body owns focus, receptor
factorization, change state, physical receptor sites, and checkpoint persistence.
In this morphology the old sparse retinal inputs are silent, and only changed
focused receptor values enter the same learner core used by the hand and motors.
This establishes body-boundary integration only; it does not claim focus is
necessary, learning occurs, an action is chosen, or a key is reached.

## Authority

- Path: `academy.md`; `arch.md`; `LANGUAGE.md`; `algo.md`;
  `plans/focused-field-learner-participation.md`;
  `research/campaigns/workstation-focused-receptor-participation-v1/artifacts/participation.json`;
  `factory/receipts/focused-field-learner-participation-verification.json`
- Revision: HEAD `dfe933886d4a030d7775356f78e908e8531c2fc2`;
  architecture SHA-256
  `02d837a8dc205aae7b088147226c94aa08783898a653550334718bbdf0cc003f`;
  parent artifact SHA-256
  `be700e95a4fb8373f2ad93022ae980e8736fae7f47ce3de0c0251788558d4639`;
  parent verification SHA-256
  `01890dcc6aa146a8d756e96c47697d9cac3f18ef3cfe9d96348c45788982dab9`

## Model

The external object is a pair of complete light fields. The body object is its
current gaze, one depth-seven one-focus profile per eye, a fixed frame of 57
region slots per eye, exact 32-bit factors per slot, three physical values per
factor (unavailable, false, true), and the previous factor frame. The body arrow
maps `WorldSample + WorkstationState + FocusedVision` to
`FocusedVision + changed PhysicalInput values`. Repeating the same receptor
state is the identity and emits nothing. Changing a factor emits exactly one
ordinary physical transition to its fixed receptor site.

`WorkstationHarness` owns this state and its physical sites. Its normal step
composes focused transitions with touch, proprioception, and motor opportunity
inputs before the single public core admission. A focused research body does not
also admit the legacy 24 sparse retinal values. Observer-only research data
reports active-region counts, focused transition identities, and the number of
sparse retinal admissions; it does not feed back into the body.

The optional organ and its previous frame are part of a version-three
workstation checkpoint. Version-one and version-two checkpoints migrate with no
focused organ. Restore validates every retained focused site against the same
core checkpoint before making the body available.

## Invariants

- Rendering, cue identity, target, expected action, and evaluator comparisons
  remain outside the organism; the workstation body receives only light and
  contact fields.
- Actual body gaze is the only focus; no evaluator or experiment chooses a
  region or receptor.
- Each eye always produces 57 slots and 1,824 three-valued binary factors in
  deterministic order; the two-eye product has exactly 3,648 factors.
- Binary factorization is exact, unavailable padding remains unavailable, and
  a factor change emits exactly one transition while an exact repeat emits none.
- Focused and sparse visual admission are mutually exclusive in one body step.
- Focused transitions enter the owned `WorkstationHarness` core alongside the
  existing hand, proprioception, and motor topology; no second learner fixture
  exists.
- Save and restore preserve the exact next focused transition, body result,
  core result, and observer record.
- Existing production construction, sparse retina behavior, learner law,
  Academy semantic firewall, and accepted checkpoint migrations remain equal.

## Scope

- `truelearner/crates/workstation/src/harness.rs`: focused organ, physical sites,
  body-owned transduction and admission, research construction, and inert
  observation.
- `truelearner/crates/workstation/src/checkpoint.rs`: version-three focused body
  persistence and version-one/version-two migration.
- `truelearner/crates/workstation/src/lib.rs`: research-only construction and
  observation types.
- `truelearner/crates/workstation/tests/focused_vision_body.rs`: body ownership,
  change/repeat, branch distinction, sparse exclusion, and exact restart.
- `factory/receipts/`: candidate and verification evidence.
- Excludes production adoption, learner-law changes, semantic features, learned
  focus, motor-policy changes, action claims, Academy curriculum changes, and
  authority promotion.

## Development style

TDD. Add focused-body tests for first-sample identity, exact change, repeat,
sparse exclusion, branch distinction, and checkpoint replay before completing
the body driver and version-three migration.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test focused_vision_body`
  proves that the real workstation body owns the focused organ, admits only
  changed focused receptors, silences sparse visual admission, distinguishes
  two light fields, and restores the exact next step.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research checkpoint`
  proves version-one, version-two, and current checkpoint behavior.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test focused_receptor_frames`
  preserves the fixed-frame and map-preservation laws used by the body.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  preserves normal sparse-retina body behavior without research features.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path truelearner/Cargo.toml`, and
  `cargo clippy --locked --manifest-path truelearner/Cargo.toml --all-targets --all-features -- -D warnings`
  enforce workspace formatting, typing, and lint gates.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research`.
Its measured warm duration must remain strictly under 10 seconds; record cold
bootstrap separately.

## Controls and evidence

Held-out cases are unequal eye dimensions, edge gaze, two independently changed
eyes, an unavailable padded slot, and a changed light field after checkpoint
restore. Negative controls are an initial observation, an exact repeated field,
unchanged non-visual inputs, the normal sparse body, zero sparse admissions in a
focused body, invalid mode composition, corrupt focused state, and old checkpoint
migration. Falsifiers are external focus selection, simultaneous sparse and
focused admission, an input on an unchanged factor, missing changed factors,
inexact restart, invalid retained sites accepted on restore, Production drift,
semantic leakage, non-quiescence, or a warm regression at or above ten seconds.

Evidence is a validated candidate receipt and independent verification receipt
recording exact commands, results, tree digest, and warm duration. No rendered
image, cue label, expected action, or evaluator verdict enters body input or
checkpoint state.

## Risks and rollback

The full exact frame adds 10,944 receptor junctions and therefore remains a
research morphology until a later cost comparison. Dynamic focus must not become
dynamic wiring: receptor identities stay fixed and only their physical values
change. Checkpoint migration can fail if old layouts are decoded as the new
payload, so each version has an explicit payload type and migration test.
Rollback removes the optional organ and version-three payload while leaving the
established focused-field library and normal workstation body unchanged.

## Open decisions

None.
