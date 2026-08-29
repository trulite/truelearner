# Workstation visual reach

```text
retinal junction fires -> eye path chooses -> fixed projection fires palm relay -> palm path chooses
```

## Outcome

Add one research-only workstation morphology in which the same signed retinal
firing can participate in both an eye-horizontal choice and a palm-horizontal
choice. Preserve it as a falsified initiation control: opposite keys start the
correct gaze and palm movement, but the palm crosses the key and runs to the
body boundary. Add one successive foveal morphology in which the illuminated
surface remains physically visible at alignment, and use it as the killing
falsifier for the proposed closure: opposite real keys still enter the
corresponding horizontal key span, then escape to the boundary. This is
development evidence for visual-reach initiation, not closed-loop reach;
intended-key contact is recorded but is not required and no tap, click, or
typing claim follows.

## Authority

- Path: `academy.md` Body Discovery; `arch.md`; `LANGUAGE.md`; `algo.md`;
  `lessons.md` lessons 66-68; retained threshold trace
  `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/threshold-retinotopic-visual-transition-64.json`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; trace SHA-256
  `00bfabec21474e7754ef639c3793df2bdfd3f3fba1f9aafe7d3ddfbffd9c5bf8`

## Model

The retained trace already commutes from a signed retinal transition to the
matching eye-horizontal output. It does not commute to the hand: both keys
leave the palm in the same state for sixty-four steps because each retinal
junction lives only in its eye-axis local field.

The candidate adds one ordinary inner-region relay for each retinal bin. A
fixed anatomical drive link projects the existing retinal junction to that
relay while preserving the firing's causal lineage. The relay occupies the
same signed position around the uncoupled palm-horizontal axis that the retinal
junction occupies around its eye-horizontal axis. It can therefore form and
reuse ordinary local paths to exactly one palm-horizontal direction. The
retinal input is admitted once; its eye path and its relay projection are two
outgoing physical paths from that one junction, not duplicated external input.

Only the new visual-reach arm uncouples palm horizontal, vertical, and depth
positions. Earlier palm-component and accepted-hand arms retain their exact
topology and behavior. Failure paths are absent or ambiguous retinal
transition, no relay firing, no executable palm candidate, wrong-direction
palm movement, and loss of the prior eye choice.

The retained 64-step relay trace then exposes the next quotient: its lattice
contains only peripheral horizontal offsets. Both palms enter the intended
key's horizontal span near sequence 10, but the cue has no foveal receptor at
alignment, so the learned direction continues to the body boundary. The
successive arm changes only its twelve-point retinal lattice: it retains the
real-key `-160/+160` pair and adds `(0, 0)`. Its retained 64-step trace
(`b45ae35eeb6d19fa563e758f308af47cefa9d9e5717b19b066ae445c88c33083`)
shows that this receptor fires at the real key, but without a physical
eye-to-hand alignment relation the reach still hits a body boundary. Seeing
the target at the center of an eye is not the same fact as the hand occupying
the target.

## Invariants

- The organism receives no key identity, key rectangle, target coordinate,
  desired direction, distance, score, contact verdict, or evaluator state.
- One admitted retinal input fires once. Projection occurs through a fixed
  inner physical link and preserves its ordinary causal lineage.
- Relay placement uses only eye identity, retinal body-local offset sign, bin,
  and the fixed palm-horizontal morphology.
- The foveal receptor is an ordinary retinal junction at body-local offset
  `(0, 0)`; it has no palm relay and cannot name alignment, success, or a
  preferred continuation.
- A negative retinal offset is local only to decrease; a positive offset is
  local only to increase. Vertical retinal factors cannot create the
  horizontal reach relay.
- The final retinal transition remains the only output-bearing transition;
  crossed thresholds remain samples and one outcome produces one learning
  effect.
- Production, earlier research arms, accepted key press/release behavior,
  replay, natural quiescence, bounded work, and the semantic firewall remain
  unchanged.
- Academy computes toward-key distance and contact only after outward movement;
  those evaluator facts never re-enter the harness.

## Scope

- `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/workstation/src/lib.rs`
- `research/experiments/workstation-return-bearing-opportunity-composition/`
- `research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/`
- `factory/receipts/`
- `lessons.md` only after a solve survives
- Excludes core learner-law changes, Academy world/render changes, hidden action
  routes, finger selection, tap/hold/release changes, production adoption, and
  authority promotion.

## Development style

TDD. First require a projected retinal relay to be local to exactly one
palm-horizontal direction without changing the existing retinal map. Then
require opposite real-key cues to produce opposite gaze and palm movements
toward the externally measured key surfaces. Preserve the foveal trace as the
final negative control proving that target visibility alone does not close a
hand-target relation.

## Focused tests

- `cargo test --locked --features research --manifest-path truelearner/Cargo.toml -p truelearner-workstation visual_reach_projection_preserves_sign_without_duplicating_input`
  proves one signed relay is local to the matching palm-horizontal direction.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib visual_reach_moves_eyes_and_palm_toward_opposite_real_keys`
  preserves the initiation witness and its 64-step boundary-escape falsifier.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib foveal_visibility_alone_does_not_close_the_reach`
  proves both runs enter keys 26 and 87's horizontal spans and later reach a
  horizontal body boundary.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib threshold_retinotopic_visual_transition_reaches_an_executable_choice`
  preserves the predecessor eye-only witness and its identical palm control.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib first_choice_lifetime_presses_and_releases_real_keys`
  preserves the accepted hand trajectory.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; cold bootstrap
is recorded separately.

## Controls and evidence

The focused development pair is real ANSI keys 26 and 87 for twelve steps. The
held-out continuation is the same pair for sixty-four steps because the current
twelve-receptor morphology has only that symmetric horizontal surface pair;
location transfer is residual uncertainty, not claimed evidence. Negative control
arms are the predecessor threshold arm with identical palm trajectories,
the non-foveal reach arm, visual-return, unit-retinotopy, and impulse-magnitude falsifiers, vertical
retinal factors, passive field change, exact replay, the accepted hand arm, and
the semantic firewall. The killing falsifier is correct opposite eye movement
without later entry into the correct key span followed only by monotone escape
to a horizontal body boundary. Evidence is a retained full paired trace plus validated
candidate and verification receipts. This is not authority evidence.

## Risks and rollback

The projection could duplicate consequences, collapse eye and palm outputs
into one competition, or leak a desired route. Keep it as one fixed inner link,
separate the palm axis only in this arm, inspect complete choice diagnostics,
and fail if the predecessor arm changes. If falsified, retain the trace and
remove only the new research enum and relays; no checkpoint schema changes.

## Open decisions

None.
