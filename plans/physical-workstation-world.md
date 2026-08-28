# Add the physical workstation world

```text
body pose -> collision -> touch -> learner -> body movement
    ^                                      |
    |                                      v
two eye rasters <- scene <- monitor <- device mechanics
                              ^              |
                              `-- cursor/text-'
```

## Outcome

Add a headless external workstation world around `WorkstationHarness` with a
standard ANSI 104-key keyboard, continuous touchpad, monitor raster containing
a fixed photographic asset, binocular hand/scene rendering, physical contact,
key travel, cursor motion, click state, typed text, and exact joint
world-and-organism checkpoint replay.

This establishes the real physical surface for the next developmental ladder.
It does not teach an action, establish digit separation, prove pointing,
clicking, typing, or depth understanding, or change learner physics or accepted
body authority.

## Authority

- Path: `arch.md` Accepted law, Boundaries, and Forward design; `academy.md`
  Ownership, Body discovery, and Evidence rules; `LANGUAGE.md`; `algo.md`;
  `truelearner/crates/workstation/README.md`
- Revision: clean parent commit `528ab0e0cbeab3f854ba87e008d32ae5b199bf48`;
  Connected Outcome Product V1 authority remains the scoped learner parent

## Model

- Add headless `academy-workstation`, depending on `truelearner-workstation` and
  ordinary image/font/serialization libraries. It owns one public
  `WorkstationHarness` and one external `WorkstationWorld`.
- `WorldGeometry` is immutable physical structure: a monitor plane, one
  continuous touchpad rectangle, and 104 bounded key rectangles with standard
  ANSI row spacing, widths, labels, and travel. Key labels and character
  mappings are external device facts and never enter the harness except as
  rendered pixels after physical contact changes device state.
- `DeviceState` contains key-down state, cursor position, touch tracking,
  visible text, click selection, and physical step. Invalid combinations are
  prevented by private constructors and bounded updates.
- `sense(body, world)` is a pure projection from body pose and world state to
  two grayscale eye rasters plus six contact samples. Each eye receives a
  separate perspective view; hand depth produces disparity. Collision alone
  produces pressure and slip.
- `advance(world, body_before, body_after)` is a pure physical transition. A key
  crosses its press/release travel thresholds, maintained touchpad contact moves
  the cursor by the actual fingertip delta, and a short low-travel release
  toggles click selection. Device events are evaluator-visible observations,
  never learner input.
- `render(world, body, eye)` composes desk, monitor photo, visible text/cursor,
  proper keyboard labels and depressed keys, touchpad, and hand geometry into a
  fixed 640x360 raster. The checked-in photo is decoded once from bytes and its
  digest is part of the world layout contract.
- `WorkstationSession::step` transactionally composes current sensing, one
  `WorkstationHarness::step`, then external device advancement. The changed
  monitor and new touch become ordinary input only on the following step.
- `SessionCheckpoint` atomically contains opaque harness bytes, device state,
  touch history, sequence, world layout version, and asset digest. Restore
  either recreates the exact next step or rejects the whole envelope.

## Invariants

- Organism-visible values are only two light fields, six contact samples,
  proprioception, and ordinary physical outcomes through `WorkstationHarness`.
- Cursor, keys, characters, click state, expected behavior, capability labels,
  and evaluator verdicts never enter the harness as structured values.
- No world rule selects an eye, hand axis, digit, direction, target, key, or
  correct action for the learner.
- One key changes only after a fingertip occupies its rectangle and crosses the
  physical press threshold; release requires crossing the separate release
  threshold. Proximity alone changes no device state.
- Touchpad cursor motion requires maintained contact and actual fingertip
  displacement. Stationary touch cannot move the cursor.
- A tap click requires contact begin, bounded duration and travel, then release;
  click meaning remains external and is visible to the learner only through
  changed monitor pixels.
- Body state, world state, and monitor output advance in one deterministic order
  and use integer/fixed geometry. Save/restore preserves the exact next sample,
  device events, body observation, fingerprints, and natural quiescence.
- Rendering, asset decoding, serialization, and evaluator observations do not
  enter learner physical time or strength.
- Existing Body Discovery and core controls remain unchanged. This is external
  world adaptation, not learner-physics or authority adoption.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Add `academy/crates/academy-workstation/` with geometry, device mechanics,
  rendering, session/checkpoint code, public tests, README, and one generated
  photographic monitor asset under `assets/`.
- Add the crate to the Academy workspace and lockfile, and document the new
  headless world in `academy/README.md` and the root overview where needed.
- Test the complete 104-key inventory, unequal key widths, contact hysteresis,
  touchpad motion/tap, binocular disparity, monitor pixel consequences,
  semantic firewall, transactional failure, checkpoint replay, and bounds.

Exclude learner-law changes, a new research campaign, teaching schedules,
capability promotion, synthetic action injection, reward, correct-key signals,
OS APIs, DOM/accessibility input, real hardware drivers, audio, networking,
Playground UI, arbitrary keyboard layouts, multi-touch gestures, grasping, and
authority promotion.

## Development style

TDD. Specify geometry counts, pure collision/device transitions, raster
consequences, firewall serialization, and checkpoint identity before composing
the live session.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --lib`
  establishes 104-key geometry, contact, device, renderer, binocular, and
  checkpoint laws.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world`
  establishes public session replay, real monitor asset use, physical-only
  device changes, and semantic-firewall controls.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  establishes unchanged body behavior.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes unchanged Body Discovery laws.
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo check --workspace --all-targets --locked --manifest-path academy/Cargo.toml`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --lib`

Its measured warm budget is strictly under 10 seconds. Asset generation and
cold image decoding are recorded separately and are not learner physical time.

## Controls and evidence

- Held-out cases: function/navigation/numpad keys, unequal-width modifiers, two
  simultaneous fingertips, keyboard-edge misses, a fresh cursor start, another
  eye baseline, and checkpoint restore during maintained touch.
- Negative controls: hovering above a key does not press it; stationary
  touchpad contact does not move the cursor; motion without contact does not
  move it; a long or dragged release is not a tap; monitor changes do not occur
  before device state changes; serialized `WorldSample` contains no key,
  cursor, click, character, target, score, or capability fields.
- Laws: sensing is deterministic; unchanged body/world is identity on device
  state; disjoint key presses commute; perspective reflection swaps eye
  disparity; device transition followed by render composes to the visible next
  sample; save/restore is identity on the next complete session step.
- Falsifiers: an evaluator noun crosses the harness boundary, a device changes
  without physical contact, current-step pixels reveal a later device change,
  key inventory/geometry is incomplete, photo bytes are absent or unstable,
  checkpoint replay differs, a run fails to quiesce, existing body tests change,
  or the warm suite reaches 10 seconds.
- Evidence: generated asset and digest, validated plan, focused law tests,
  exact session replay, unchanged body and Academy regressions, candidate
  receipt, and independent verification receipt.
- Not applicable because no learned capability or capstone is scored: this
  change constructs the external physical world and preserves the first learner
  failure for a later campaign.

## Risks and rollback

- A detailed scene can be computationally expensive. Keep one bounded raster,
  integer drawing, cached decoded asset, and the measured warm loop.
- Device mechanics can accidentally become a controller. Require body-contact
  transitions for every change and keep all semantic mappings outside inputs.
- Generated image bytes can be nondeterministic only at creation time. Check in
  the selected immutable PNG, hash it, and render deterministically thereafter.
- Full 104-key geometry is easy to miscount. Build it from explicit standard
  rows and assert count, bounds, row membership, widths, and non-overlap.
- Roll back by removing `academy-workstation`, its workspace membership, asset,
  docs, and receipts. The committed organism, Body Discovery, core checkpoints,
  and authority artifacts require no migration.

## Open decisions

None.
