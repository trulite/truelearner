# Workstation visual transition return

```text
eye output -> gaze changes -> retinal bin changes -> outcome returns -> path competes
```

## Outcome

Add one research-only workstation arm that carries an eye output's existing
outcome through a retinal bin that physically changed after that eye moved.
Preserve that arm as a falsified control: its held-out trace changes candidate
drive but never changes the selected direction. Add one successive research
arm that retains the retinal receptor's signed body-local offset during path
formation. The paired illuminated-key fixture must first produce distinct
transition witnesses and then a distinct executable choice or movement. This
is development evidence for visual control, not proof of an intended key press.

## Authority

- Path: `arch.md`; `LANGUAGE.md`; `algo.md`; `lessons.md` lessons 35, 58,
  64-68; retained trace
  `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/visible-key-intention-wide-retina-trace.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; trace SHA-256
  `dafc8c398aba2e760f4df315ec1e3abe60fb5ae388f1d8cf4152caaa6904a9ef`

## Model

`RetinalState` holds only the last admitted retinal bins and gaze. `observe`
compares the next bins with that state and returns changed feature identities.
Before attributing a change to gaze, the new light field is also sampled at the
old gaze. Every old-gaze bin for that eye must equal its retained value. A
changed new-gaze feature then becomes `PhysicalTransition` only when its owning
eye axis actually moved in the preceding world step and exactly one existing
output-specific outcome identifies that movement. The physical input keeps the
changed retinal junction as target and carries that outcome as origin. Static
resampling, passive frame changes, and unattributed changes remain `Sample`.

The receptor state is overwritten after each successfully admitted sample and
is included in checkpoints. Version-one checkpoints migrate with an empty
receptor state, so their first sample establishes identity without inventing a
transition.

The visual-return-only arm maps every retinal receptor to its eye-axis center.
Its retained traces prove two independent failures: keys 19 and 27 produce
different candidate input counts and drive but the same winning direction,
while the attempted 19/79 opposition control reveals that key 79 falls outside
the positive retinal sample and therefore cannot support an opposition claim.
The corrected morphology control uses real ANSI keys 26 and 87, whose lit
surfaces fall on negative and positive retinal offsets. The
successive retinotopic arm changes only the receptor placement map. A retinal
feature is placed two local units to the decrease or increase side of its eye
axis according to the sign of that feature's dominant body-local offset. Its
horizontal lattice includes a symmetric `-160/+160` pair so actual left and
right ANSI key surfaces are observed rather than the gaps beside them. This
keeps the existing locality radius connected to exactly one direction while
using no key, target, evaluator, or desired-action knowledge. Non-retinal
receptors and every earlier arm retain their existing positions.

That retinotopic arm is also retained as a falsifier. The corrected 26/87 trace
shows equal transition feature sets: the illuminated receptor changes by three
bins while the corresponding unilluminated receptor changes by one, but both
are mapped to unit impulse, so the learner remains identical. A retained
impulse-magnitude arm proves that merely changing the impulse to
`abs(to_bin - from_bin)` alters the learner but not any candidate through 64
steps: the receptor threshold quotients impulses one and three to the same
single firing. The successive threshold arm instead factors one ordinal
transition through the intermediate bin junctions it physically crossed. The
final-bin incidence alone remains the causal `Transition`; intermediate
thresholds are ordinary `Sample` incidences, so the output outcome is learned
once. Non-retinal transitions retain their existing representation.

## Invariants

- No key identity, coordinate, target, direction, score, geometry, or evaluator
  verdict enters the organism.
- Retinotopy may preserve only the sign already present in the receptor's
  body-local morphology; it cannot inspect luminance, key identity, world
  coordinates, outcomes, or evaluator state when placing a receptor.
- Retinal magnitude may use only the two already-admitted quantized bins; it
  cannot inspect raw pixels, presentation identity, or evaluator state.
- Threshold factorization may add only strictly intermediate bins crossed by
  that change; it cannot duplicate the causal transition or its outcome.
- A retinal transition requires a changed admitted bin, stability of the new
  field at every retained old-gaze receptor for that eye, and a preceding
  actual movement of that receptor's eye axis.
- Its origin is the existing outcome of exactly one actual output on that same
  axis; ambiguous or missing causes remain samples.
- Initial observation, repeated identical bins, a passively changed field,
  touch, proprioception, and generic opportunity cannot create or refresh a
  visual transition.
- Receptor state lasts one admitted sample interval and survives exact restart.
- The accepted hand arm keeps its exact contact, key press, and sequence-61
  release; production behavior and the sparse retina remain unchanged.
- The falsified visual-return-only arm remains behaviorally available as the
  negative control; signed placement exists only in the new research arm.
- The falsified unit-impulse retinotopic arm remains behaviorally available;
  impulse magnitude and threshold factorization exist only in successive
  research arms, and the failed impulse arm remains available too.
- Natural quiescence, bounded work, diagnostic purity, and the organism
  firewall remain intact.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/workstation/src/checkpoint.rs`
- `truelearner/crates/workstation/src/lib.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- candidate and verification receipts
- Excludes core learner-law changes, world rendering changes, cue semantics,
  key selection, full reach, and authority promotion.

## Development style

TDD. First prove the pure bin-change classifier, old-gaze stability guard, and
checkpoint lifetime, then require the retained paired fixture to expose
transition witnesses before any choice or movement claim.

## Focused tests

- `cargo test --locked --features research --manifest-path truelearner/Cargo.toml -p truelearner-workstation retinal_delta_requires_a_changed_bin`
  proves identity, changed-bin selection, and repeated-sample exclusion.
- `cargo test --locked --features research --manifest-path truelearner/Cargo.toml -p truelearner-workstation --test workstation_harness visual_receptor_state_restores_the_exact_next_step`
  proves the old-gaze stability guard, one-sample state, and exact restart.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation version_one_checkpoint_migrates_with_empty_retinal_state`
  proves the version-one migration establishes no invented visual transition.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib visual_transition_preserves_the_candidate_to_choice_falsifier`
  preserves the visual-return-only falsifier: candidate drive may diverge but
  selected choice and movement must not.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib retinotopic_visual_transition_preserves_the_magnitude_falsifier`
  preserves the unit-impulse retinotopic falsifier.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib impulse_magnitude_preserves_the_threshold_falsifier`
  preserves the learner-changed-but-no-candidate impulse falsifier.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib threshold_retinotopic_visual_transition_reaches_an_executable_choice`
  proves the first changed retinal transition and requires the first distinct
  choice or movement under opposite-side paired cues.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib first_choice_lifetime_presses_and_releases_real_keys`
  preserves the accepted hand trajectory.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; cold build is
recorded separately.

## Controls and evidence

The held-out case is a 64-step real-key 26/87 opposite-side paired cue trace after the focused
12-step witness. Negative controls are the retained visual-return-only
opposite-side trace, the same-side cue arm, initial observation, identical resampling,
a passively changed field even after eye movement, the prior wide-retina arm with no
visual-transition return, the accepted hand arm, replay, quiescence, and the
semantic firewall. The killing falsifier is a recorded retinal transition with
different learner state but no later distinct executable choice or movement in
the focused budget. Evidence is the retained paired trace plus validated
candidate and verification receipts. This run is not authority evidence.

## Risks and rollback

Simultaneous world and body changes can make causal attribution ambiguous. The
old-gaze stability, same-axis actual-movement, and exact-one-output guards fail
closed instead of guessing. If the arm is falsified, preserve its trace and remove the research
enum and receptor state; version-two checkpoints can still migrate legacy
state safely.

## Open decisions

None.
