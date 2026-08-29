# Workstation binocular stable fixation

```text
off-center retina -> directional eye opportunity -> eye movement
centered retina   -> identity eye opportunity    -> unchanged gaze
                         left × right -> stable binocular product
```

## Outcome

Retain the exact post-alignment choice evidence, locate the first directional
arrow that leaves an acquired binocular relation, and test the smallest
research-only eye-local composition that maps a physically active center
receptor to the motor identity. Require the minimal acquisition parent to keep
working in all six mirrored disparity relations and then remain exactly aligned
for eight unchanged-world steps. This establishes bounded stable fixation only;
it does not establish fused depth, natural-image correspondence, reaching, or
general three-dimensional perception.

## Authority

- Path: `arch.md`, `academy.md`, `LANGUAGE.md`, `algo.md`, `lessons.md`
  lessons 35, 57-58, and 66-71; `research/constitution.md`; parent convergence
  `research/campaigns/workstation-binocular-alignment-v1/convergence.toml`;
  final protocol
  `research/campaigns/workstation-binocular-stable-fixation-v1/protocol-v2.toml`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; parent development
  artifact SHA-256
  `ad39ca0ee0080d36aa0409f553ea4abd7395aef2a66b7a583cb74489f8275329`

## Model

One eye state is the product of its raster field, signed retinal receptors,
horizontal axis, directional motor opportunities, actual movement, and returned
visual change. The parent exposes two movement arrows at every state. At exact
alignment the required arrow is the identity: the gaze must remain unchanged.
The left and right identity laws compose independently; neither eye may read or
gate the other.

First extend only the experiment projection with the existing
`ResearchChoiceDiagnostic` values, proving projection inertness by exact replay
and unchanged checkpoints. At the first post-alignment departure, classify the
directional output as generic, returned-transition continuation, completed-cycle
reuse, or cancellation. The v1 opportunity and center-return arms remain frozen
after failure. Their zero-opportunity counterexample selects the v2 common
boundary after all internal output sources join: the external eye actuator maps
an owning-eye centered state and any horizontal output to the unchanged gaze.
This is a body identity effect, not an output, reward, direction, or evaluator
stop.

Invalid external values fail before transition. Missing exploration fails the
off-center control. Premature cross-eye suppression, movement during the eight
step hold, replay difference, non-quiescence, over-budget work, or semantic
leakage rejects the candidate.

## Invariants

- The organism receives only separate raster pixels, touch, proprioception, and
  generic physical opportunity.
- Target position, disparity, error, aligned state, duration, and verdict remain
  evaluator-only.
- A center receptor is ordinary anatomy, not a target label; the same map must
  act on any centered bright feature and on only its owning eye.
- Off-center samples retain the exact parent acquisition arrows, opportunities,
  outputs, and effects.
- Left and right histories, gates, outputs, outcomes, and returns remain a
  product with no fused channel.
- Instrumentation is causally inert and retained once for offline analysis.
- Production construction and the retained visual-reach trace remain byte
  identical.
- Exact replay, checkpoints, natural quiescence, semantic firewall, bounded
  work, and the under-ten-second warm loop remain intact.

## Scope

- `research/experiments/workstation-binocular-alignment/`
- `research/campaigns/workstation-binocular-stable-fixation-v1/`
- research-only fields and eye-effect composition in
  `truelearner/crates/workstation/src/harness.rs`
- `truelearner/crates/embodiment/` only if the selected solve needs the neutral
  reusable opportunity gate
- `lessons.md` and `factory/receipts/` after a surviving solve
- Excludes core learner-law changes, Production adoption, Academy capability
  promotion, fused depth, vergence, hand movement, and device behavior.

## Development style

TDD. Add the stable-fixation oracle and inert choice projection first, preserve
the parent failure, select the branch from the first departure, then implement
only its predicted eye-local solve. Run downward removals only after the complete
candidate succeeds.

## Focused tests

- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib stable_fixation`
  checks six-relation acquisition followed by eight exact hold steps.
- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib fixation_controls`
  checks disabled identity, dark field, off-center exploration, one-eye locality,
  replay, quiet, work, and leakage.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`
  preserves neutral driver laws, body physics, checkpoints, and Production.
- `cargo run --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --bin binocular_alignment_trace -- research/campaigns/workstation-binocular-stable-fixation-v1/artifacts/stable-fixation.json stable`
  emits the retained causal and external trace.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation && cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`.
Its measured duration must remain strictly under 10 seconds; cold bootstrap is
recorded separately.

## Controls and evidence

The positive reference is the unchanged minimal acquisition parent over both
mirrors and three disparity bands. Held-out controls center only one eye, use a
dark field, and keep targets off-center. Negative controls disable the identity
map and remove the center receptor independently. The killing falsifiers are
cross-eye suppression, premature loss of exploration, a directional output
during the required hold, or any need for evaluator knowledge.

Evidence is one lossless diagnostic trace, the successful or falsified solve
traces, validated campaign convergence, a factory candidate receipt, and
independent verification. No research authority or Production adoption follows
from development success.

## Risks and rollback

A center gate can accidentally encode the fixture, suppress all eye exploration,
or hide a directional output in the observer. Detect these with mirrored and
one-eye controls, unchanged movement traces, full choices, dark fields, and
Production equality. Roll back the orthogonal research field and experiment arm
together; checkpoints and Production defaults remain unchanged.

## Open decisions

None.

## Result

The retained first departure was an ordinary directional choice, but suppressing
only generic opportunity failed because a learned retinal path could later emit
the same directional output with zero opportunity. Removing the center return
also failed. The smallest surviving common boundary is the eye actuator: when
that eye's own ordinary center receptor is bright, every horizontal output
composes to the identity body effect. Internal choices and outputs remain
visible, but gaze does not change and no false movement return is created.

The research-only composition preserved exact acquisition and then held both
eyes exactly aligned for every remaining step in all six outward and inward
far, middle, and near relations. The shortest retained hold was 29 steps, above
the required eight. Dark eyes still explored, one centered eye did not suppress
the other, replay was exact, every run quiesced naturally, and work stayed
bounded. This is bounded stable fixation development, not Production adoption
or natural-image stereo evidence.
