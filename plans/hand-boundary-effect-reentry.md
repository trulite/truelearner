```text
source fires -> crosses physical region boundary -> effect receiver fires
                                      |
                         +------------+------------+
                         |                         |
             deliver effect normally       path genesis decision
                                                   |
                         regional closure OR causal effect terminality
```

# Separate outward effect delivery from inward path genesis

## Outcome

Two opt-in recursive protocols test the localized re-entry boundary without changing
effect delivery. Regional closure permits learned source-to-output paths only inside
one physical region. Boundary-effect terminality instead permits ordinary path
formation except on the exact firing caused by a link that crossed a region boundary.
An unrelated feedback fixture and an external-surface discriminator select the
smallest law before either candidate is tried on the unchanged hand.

## Authority

- Path: `research/campaigns/hand-origin-feedback-cycle-localization-v1/convergence.toml`
- Revision: `103bbb83226fa9914300e59bc1d6ddaf9684f926da612ff85383549f4de7abe7`

## Model

Each valid drive incidence maps to a `Fired` value carrying whether any actual input
link crossed between physical regions. Output delivery remains the existing
`Fired -> scheduled firing + external Output` transformation. Path genesis maps:

- regional closure: `(source_region, output_region)` to formation only when equal;
- effect terminality: `(boundary_effect, ordinary eligibility)` to formation only
  when the firing was not caused across a region boundary.

Old protocols map every firing through the identity behavior. The two candidates
change structural genesis only; selection, strength, consequence, execution, and
output effects remain in their existing transformations.

## Invariants

- A cross-region output link still emits exactly the same outward effect and schedules
  the same receiving firing.
- Boundary-effect terminality suppresses only new path formation from that receiving
  firing; it does not suppress firing, propagation, consequence, or existing paths.
- A genuinely external input at the same physical junction is not a boundary effect
  and remains eligible to form paths under the narrower law.
- Regional closure compares physical regions, never semantic sensor, motor, hand,
  direction, or benchmark identities.
- Existing protocols, checkpoints, traces, replay, and canonical state remain unchanged.
- Internal same-region sensor-to-motor formation and one-effect behavior remain intact.
- Natural quiescence must occur without evaluator termination; bounded exhaustion is
  a falsifier, not a success condition.
- Neither experimental law becomes the default or adopted physics in this change.

## Scope

- Add two opt-in protocol variants and predicates in the core.
- Carry one transient `boundary_effect` bit from actual incidence to path genesis.
- Gate path genesis in `path.rs`; do not persist the bit in checkpoints or learner state.
- Add core fixtures for cross-region feedback, same-region formation, external outward
  input, effect equality, replay, and quiescence.
- Add a research experiment and campaign that compare both laws, then conditionally
  retry the unchanged bounded hand.
- Exclude origin, ownership, candidate ranking, strength, lifetime, hand-world,
  evaluator, default-protocol, and authority changes.

## Development style

TDD. Encode the unrelated cross-region feedback failure, exact outward effect, external
outward input discriminator, and same-region preservation before adding either gate.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary boundary_effect_reentry`
  establishes the unrelated loop, candidate quiescence, unchanged effect, external
  input discriminator, same-region formation, replay, and old-protocol identity.
- `cargo test --locked --manifest-path research/experiments/hand-boundary-effect-reentry/Cargo.toml`
  establishes candidate selection in fixtures and preserves the conditional hand result.
- `cargo test --locked --manifest-path research/experiments/hand-origin-feedback-cycle-localization/Cargo.toml`
  preserves the frozen six-edge diagnosis.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-boundary-effect-reentry/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: external firing of an outward-region surface, a same-region source,
  an existing cross-region link, reversed input order, and exact replay.
- Negative controls: unchanged causal-origin protocol still exposes the unrelated
  feedback failure under a bound; both candidates preserve initial outward effect;
  older protocols and the frozen diagnostic remain exact.
- Falsifiers: effect delivery changes; the narrower law blocks external surface path
  formation; the unrelated loop remains; the hand still exhausts; useful movement is
  lost without a stronger safety gain; or replay, quiescence, locality, and cost regress.
- Expected artifacts: per-arm immutable evidence, convergence, candidate receipt,
  and independent verification receipt.

## Risks and rollback

Region boundaries may carry legitimate bidirectional physics, and a terminal rule may
hide useful embodied feedback. The external-surface and same-region controls detect
overreach before the hand. Rollback removes the two opt-in variants and transient bit;
all existing protocols and frozen evidence remain unchanged.

## Open decisions

None.
