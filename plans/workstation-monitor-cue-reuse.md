# Workstation monitor-cue action-outcome reuse

```text
external monitor glyph -> rendered pixels -> retina -> learner path -> outward effect
           ^                                  |
           +------ prior real key effect ------+
```

## Outcome

Add a replayable monitor-glyph presentation to the real workstation world and a
two-key development experiment that asks whether a later glyph reuses the
physical path whose proper key press previously produced that glyph. Retain one
complete trace from world raster through retina, learner-only change, candidates,
choices, body effects, contact, key events, outcome write/read, replay, and
natural quiet. The result is development evidence for or against cue-first
action-outcome reuse; it does not claim keyboard search, visual correspondence,
closed reach, intended typing, general recognition, or authority.

## Authority

- Path: `arch.md`, `academy.md`, `LANGUAGE.md`, `algo.md`,
  `research/constitution.md`, `lessons.md` lessons 35 and 58-72, and
  `research/programs/learner/forecasts/workstation-key-selection-v1.md`
- Revision: HEAD `dfe933886d4a030d7775356f78e908e8531c2fc2`; first-choice
  verification SHA-256 `51a28b37ab7881d5c7467991bd88eb202b2c81949f3f3d689ce504a7bd36734e`;
  wide-retina trace SHA-256
  `dafc8c398aba2e760f4df315ec1e3abe60fb5ae388f1d8cf4152caaa6904a9ef`

## Model

`WorkstationPresentation` gains an optional printable monitor glyph. Rendering
maps that external value to a fixed visible monitor region; `WorldSample` remains
the only organism input and contains pixels, contact, and no presentation value.
Changing presentation is an external world effect that changes neither learner
state nor physical time. Session checkpointing owns the presentation so restore
reproduces the exact next sample.

The research experiment starts from the strongest unchanged workstation
candidate, observes ordinary proper-key presses and their printed monitor
effects, and retains their exact participating outcome paths. From one identical
post-development checkpoint it presents each of two already experienced glyphs
in paired probes. A probe commutes only when glyph-specific retinal input reaches
the learner and reuses the corresponding participating physical path strongly
enough to produce a different executable choice or outward effect. Evaluator
comparison occurs only after outward observation.

Failures are typed by the first absent transformation: render, retinal sampling,
learner participation, action-to-monitor return, outcome write, cue-driven path
read, executable choice, or physical effect. No inverse is synthesized from the
forward action-outcome record.

## Invariants

- Monitor presentation reaches the organism only as binocular raster values.
- No glyph, text value, key ID, coordinate, desired action, correctness bit,
  score, or evaluator state is serialized in `WorldSample` or core input.
- Default presentation renders and behaves byte-for-byte as before.
- The full ANSI keyboard, real labels, collision, key hysteresis, visible hand,
  and ordinary device effects remain unchanged.
- Presentation changes do not advance physical time or mutate the learner.
- Checkpoint save/restore preserves presentation and exact next-step replay.
- Paired probes start from the same checkpoint and differ only in external cue.
- Development keys are selected from actual printable key effects observed in
  the retained ordinary session; Academy never forces a matching output.
- An unchanged resample cannot create or refresh a physical-transition outcome.
- Existing trace data is composed first; a new diagnostic event is added only
  if the required physical fact is absent.
- Stable fixation, accepted hand press/release, learner-only fingerprinting,
  Production/reference equality, natural quiescence, and bounded work remain
  unchanged.

## Scope

- `academy/crates/academy-workstation/src/world.rs`: optional monitor-glyph
  presentation, validation, and replay-safe external update.
- `academy/crates/academy-workstation/src/render.rs`: render the cue in a fixed
  monitor region distinct from ordinary typed text.
- `academy/crates/academy-workstation/src/session.rs`: presentation update at
  the external world boundary without harness mutation or physical-time change.
- `academy/crates/academy-workstation/tests/workstation_world.rs`: default
  identity, pixels-only firewall, update, checkpoint, and replay tests.
- `research/experiments/workstation-return-bearing-opportunity-composition/`:
  paired development/probe runner, complete retained trace, and focused tests.
- `research/campaigns/workstation-monitor-cue-reuse-v1/`: neutral protocol,
  arms, frozen artifacts, results, and convergence after execution.
- `factory/receipts/`: candidate and independent verification receipts.
- Excludes accepted core-law changes, new learner memory, key-target adapters,
  supplied gaze or hand routes, general image features, closed reach, authority
  promotion, and edits to existing negative evidence.

## Development style

TDD. First require default-presentation identity, cue-only raster change,
organism-sample firewall, presentation update purity, and exact checkpoint
replay. Then add the paired experiment and require it to report the earliest
failed arrow without treating a negative capability result as a software test
failure.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world monitor_cue`
  proves default identity, pixels-only presentation, update purity, firewall,
  checkpointing, and replay.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib monitor_cue_action_outcome_reuse`
  proves paired probes share one checkpoint, retain the complete causal slice,
  and classify the first absent transformation.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib first_choice_lifetime_presses_and_releases_real_keys`
  preserves the real hand press/release reference.
- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib stable_fixation_holds_all_mirrored_relations`
  preserves stable binocular fixation.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`
  preserves the neutral embodiment library and workstation body.
- `cargo fmt --all -- --check`, `cargo check --locked`, and
  `cargo clippy --locked -- -D warnings` run in each affected workspace.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world monitor_cue`
plus the focused research library test. Its combined warm time must remain
strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

The held-out case swaps the two cue presentations while preserving the exact
development checkpoint and all body/world state. Negative controls are blank
cue, unexperienced printable glyph, identical cue resampling, monitor cue with
no learner transition, default-presentation byte identity, and keyboard-visible
without a cue. The semantic-firewall test searches the serialized organism
sample and core input boundary for presentation data. Evidence is one lossless
paired trace, exact replay digests, learner-only and whole-session fingerprints,
device events, natural-quiescence and work records, plus validated candidate and
verification receipts. A result that reaches retina but not cue-specific path
reuse is a valid negative scientific result.

## Risks and rollback

The cue could overwrite ordinary typed text, leak through a serialized field,
mutate learner state during presentation change, or make a paired run differ in
more than photons. Keep it in external presentation state, render it in a
separate monitor cell, compare pre/post harness fingerprints before stepping,
and audit serialized input. If any invariant fails, remove the optional cue and
session update while retaining the failed experiment record; default checkpoints
remain compatible because the optional field defaults to absent.

## Open decisions

None.
