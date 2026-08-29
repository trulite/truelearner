# Focused field learner participation

```text
complete field + physical focus -> bounded focused values -> fixed receptor frame
                                                           |
                                                           v
blank-to-scene changes -> ordinary physical inputs -> Harness -> core fingerprint
```

## Outcome

Compose the established focused sensor field with the established generic signal,
change-detector, wiring, and public `Harness` boundaries. Add a fixed-width
`FocusedReceptorFrame` whose active focused regions occupy deterministic slots and
whose unused slots are explicitly unavailable. Feed exact binary channels of the
retained real binocular region sums through ordinary receptor junctions and show
whether the two cue branches first differ inside the core learner fingerprint.
This establishes bounded physical learner participation only; it does not claim
that focus is necessary, that a glyph is recognized, that an action is selected,
or that a key is reached or pressed.

## Authority

- Path: `academy.md`; `arch.md`; `LANGUAGE.md`; `algo.md`;
  `research/constitution.md`; `lessons.md` lessons 0a, 0b, 35, 66-69, and 71;
  `research/campaigns/workstation-focused-sensor-field-v1/convergence.toml`;
  `factory/receipts/composable-focused-sensor-fields-verification.json`
- Revision: HEAD `dfe933886d4a030d7775356f78e908e8531c2fc2`;
  `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  convergence SHA-256
  `0b4f469e6518c416796edd330219e4cf57bfedeb3201a54e5b51c2fc9c2449b6`;
  parent verification SHA-256
  `2ac898274ee7233ed468a27893ee638b68d744a9a7a726aa945b6a2f165a68ea`

## Model

`FocusedField` is the existing bounded variable-length product. Its new
consuming arrow `into_receptor_frame` returns `FocusedReceptorFrame` with
exactly `profile.region_bound()` slots. Regions retain their deterministic
partition order; remaining slots contain `Availability::Unavailable`. The frame
retains the original shape, profile, foci, and active-region count for external
inspection. Mapping values before or after framing commutes, repeated framing is
exact, and independent fields compose as independent frames. No geometry or
region descriptor enters a slot value.

The research adapter renders the retained blank, cue A, cue B, and cue A again
from one unchanged post-development state. Each eye uses the frozen depth-seven,
one-focus profile and its actual physical gaze. Complete unsigned pixel addition
produces one checked `u32` per focused region. Framing pads each eye to 57 slots.
Each available sum is factored exactly into 32 ordinary binary channels, giving
`2 * 57 * 32 = 3,648` fixed features. Unavailable slots remain unavailable; the
adapter never hashes, selects, or labels a region.

A bank of availability-aware change detectors observes the blank frame, then
separate cloned banks observe cue A or cue B. Only genuine blank-to-cue changes
become physical transitions. A frozen generic receptor fixture gives every
feature three ordinary unavailable/false/true junctions and a neutral internal
relay. Feature and
bit order determine physical identity before the cue comparison. Each changed
binary value fires its matching receptor through public `Harness::send_physical`;
the adapter passes no cue identity, coordinate, expected answer, target, or
evaluator result.

Both branches start from one identical saved Harness checkpoint. The observer
records only change counts, admitted inputs, natural quiescence, physical work,
core-only `HarnessObservation::fingerprint`, live participation summaries, and
exact replay. The no-admission control keeps both branches at the identical
baseline fingerprint. A no-focus arm repeats the same composition with one
region per eye to determine whether this cue pair needs focus; success there
forbids a focus-necessity claim.

## Invariants

- A focused frame has exactly the checked profile bound in every valid case;
  active regions are neither lost, copied, reordered, nor semantically selected.
- Padding is the explicit unavailable identity; it cannot invent a sensed value
  or physical transition.
- Mapping commutes with framing, and identical field, profile, and focus produce
  identical frame slots.
- Region geometry, cue identity, target coordinates, correctness, reward, and
  evaluator comparisons never enter organism-visible slot values or inputs.
- Binary factorization is exact for every checked `u32`; differing sums differ in
  at least one fixed physical feature without magnitude-threshold collapse.
- A first blank observation and an unchanged cue replay are samples; only an
  actual changed binary value is a transition.
- Focused, no-focus, cue A, and cue B branches use the same transducer, channel
  order, receptor construction, protocol, and initial Harness checkpoint.
- Removing admission leaves the core fingerprint unchanged; exact repeated
  admission from the same checkpoint reproduces outputs, work, quiescence, and
  final core fingerprint.
- All physical runs reach natural quiescence within the declared work bound.
- Existing focused-field evidence, runtime retina, workstation checkpoint,
  learner law, Academy world, semantic firewall, hand behavior, replay, and
  Production behavior remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`: fixed focused receptor frame and
  consuming frame conversion.
- `truelearner/crates/embodiment/tests/focused_receptor_frames.rs`: arity,
  padding, preservation, mapping, repeat, focus movement, and held-out modality
  laws.
- `research/experiments/workstation-return-bearing-opportunity-composition/src/bin/`:
  compact retained-raster focused-receptor participation experiment.
- `research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml`
  and `Cargo.lock`: explicit public core Harness dependency for that research
  executable.
- `research/campaigns/workstation-focused-receptor-participation-v1/`: frozen
  discovery protocol, parent, complete composition, no-admission and no-focus
  controls, compact artifact, and convergence.
- `factory/receipts/`: candidate and independent verification receipts.
- Excludes core learner-law changes, runtime workstation sensor changes,
  persistence or checkpoint migration, Academy curriculum changes, semantic
  visual features, learned attention, active focus movement, motor wiring,
  action choice, reaching, key selection, authority promotion, and Production
  adoption.

## Development style

TDD. First require fixed arity, exact active-slot order, unavailable padding,
mapping commutation, repeat equality, changed-focus honesty, two-field
independence, and held-out dimensional transfer. Then implement the smallest
frame type and conversion. Add the research receptor fixture and real retained
projection only after the shared laws pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test focused_receptor_frames`
  proves fixed arity, padding, preservation, mapping, repeat, focus movement,
  independent frames, and transfer.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin focused_receptor_participation -- research/campaigns/workstation-monitor-cue-reuse-v1/artifacts/monitor-cue-reuse.json research/campaigns/workstation-focused-receptor-participation-v1/artifacts/participation.json`
  tests the real focused, no-admission, no-focus, replay, and same-cue controls.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test focused_sensor_fields`
  preserves the complete focused-field parent laws.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib monitor_cue_action_outcome_reuse_retains_the_first_broken_arrow`
  preserves the retained parent trace and first-failure classification.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.
- `cargo fmt --all -- --check`, `cargo check --locked`, and
  `cargo clippy --locked -- -D warnings` run in every affected workspace.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; record cold
bootstrap separately.

## Controls and evidence

Held-out cases are one-dimensional and three-dimensional frames, two independent
eyes, five foci, an edge focus, and a focus move that changes region count.
Negative controls are empty focus, zero refinement, all-unavailable values,
uniform values, identical cue replay, no physical admission, no-focus physical
admission, reversed cue evaluation order, wrong driver-bank arity, unchanged
parent artifact digests, and the semantic firewall. Falsifiers are variable
frame arity, reordered active regions, invented padding, map/frame mismatch,
semantic or evaluator leakage, differing initial checkpoints, a no-admission
fingerprint change, cue-specific wiring, inexact binary channels, non-replay,
exhaustion, non-quiescence, changed parent evidence, or a warm regression at or
above ten seconds.

Evidence is a validated candidate receipt, independent verification receipt, and
one compact discovery artifact containing source digests, frame and channel
bounds, physical change counts, core fingerprints, work, quiescence, controls,
and exact replay. It contains no raw raster, semantic region label, target,
expected action, or evaluator feedback to the organism.

## Risks and rollback

A fixed frame could silently turn slot order into a semantic attention pointer,
or binary expansion could become an expensive encoding trick. Keep slot order as
the existing value-blind partition traversal, pad only with unavailable identity,
freeze exact unsigned scalar factorization before cue comparison, and report the
full channel and work cost. Stop if the result requires a selected differing
region, cue-dependent wiring, adaptive quantization, or target focus. Rollback
removes the frame type, tests, research binary, unadopted campaign, and receipts;
the established focused field remains unchanged and no persisted format moves.

## Open decisions

None.
