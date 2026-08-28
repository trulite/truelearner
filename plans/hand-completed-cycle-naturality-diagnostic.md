```text
candidate view -> local admission -> one or many physical targets
      |                                      |
 ownership refactor                 one target, preserved?
      |                                      |
      v                                      v
new candidate view -> local admission -> one or many physical targets
```

# Diagnose completed-cycle naturality

## Outcome

Add a causally inert, typed record of each local output admission and use it to
identify the first point where selection either ceases to be one physical arrow
or changes that arrow across successive learner ownership views. This localizes
the next missing transition; it does not change learner physics or claim hand
control.

## Authority

- Path: `research/campaigns/hand-completed-cycle-composition-v1/convergence.toml`
- Revision: `sha256:6c3548b7555f3af252bad7ff6087fd8fb2347777dcd75fbff26ba86d44b5729a`

## Model

A local candidate group is an object. Local selection is an arrow only when it
admits exactly one anonymous physical target; admitting a recent cohort is a
multi-target relation and therefore a prior failure of functionality. Learner
ownership reconstruction is a change of representation. Once selection is
functional, the square commutes when consecutive ownership views admit the same
physical target and fails naturally when the representation and target change.

Refactor completed-cycle resolution into one pure total function returning a
typed state: not applicable, missing, stale, ambiguous latest, or unique. Emit
one `OutputChoiceResolved` event after the existing winner is computed. It
records the ordinary, current-transition, coherent-effect, completed-cycle,
and computed winner targets; the actual admitted target set; winner and
admission bases; completed-cycle state; admitted owners; and whether the group
crosses ownership views. The event reads existing values only.

An experiment-side pure fold compares the ordered events. It first records the
earliest multi-target admission. Among single-target admissions it records
ownership changes that preserve the target and the first ownership change that
changes the target, together with the exact admission basis and completed-cycle
state at that point.
External hand position and direction remain evidence only and never enter the
core diagnostic or learner.

## Invariants

- Choice, output, path strength, memory, physical time, and work are byte-for-byte
  unchanged by tracing.
- The diagnostic contains anonymous junction and learner identities only; no
  motor meaning, direction, position, limit, target, score, or expected action.
- Completed-cycle resolution is computed once and preserves the existing unique
  recent winner for every candidate group.
- Not-applicable, missing, stale, ambiguous, and unique states are mutually exclusive and
  exhaustive when the completed-cycle protocol applies.
- The diagnostic reports actual admitted targets after recent-cohort handling;
  it never treats the earlier computed winner as the effective choice.
- The first-failure fold is deterministic, order-preserving, replayable, and
  outside organism state.
- The prior frozen candidate summary remains exact: twelve actual changes, four
  opposing-output steps, final position minus two, nine completed-cycle
  admissions, two cross-view admissions, neither boundary reached, exact replay,
  natural quiescence, and zero propagation exhaustion.

## Scope

- Extend core trace types and local choice instrumentation without adding a
  protocol or changing selection.
- Extend reusable reflected-hand evidence with group-level choice records.
- Add one frozen diagnostic campaign and pure first-failure analyzer.
- Exclude learner state, adapter force changes, semantic action identity,
  position/direction input, new selection laws, default adoption, and authority
  promotion.

## Development style

TDD. First test the exhaustive completed-cycle resolution states and the
agreement between the group-level selected target and the existing per-candidate
admission trace. Then compile and run the frozen diagnostic once.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml output_choice_resolution`
  proves typed missing, stale, ambiguous, and unique resolution plus agreement
  with the existing final admissions, including a multi-target cohort.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml output_choice_resolution`
  proves the official batched hand exposes the diagnostic without changing its
  replay, quiescence, or trajectory summary.
- `cargo test --locked --manifest-path research/experiments/hand-completed-cycle-naturality/Cargo.toml --no-run`
  compiles the frozen analyzer without consuming its one valid run.

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml output_choice_resolution`.
It must remain strictly under 10 seconds. Record cold bootstrap separately.

## Controls and evidence

Held-out states are no consequence, only stale consequences, equal latest
consequences, one unique recent consequence, a multi-target recent cohort, an
ownership change preserving a target, and an ownership change changing a
target. Negative controls are the
exact prior completed-cycle hand summary, trace-off equality, old protocols,
replay, natural quiescence, and zero propagation exhaustion.

The frozen artifact records the ordered group admissions, identifies the
earliest multi-target relation, counts commuting ownership changes, and
identifies the earliest non-commuting single-target change with its
previous/current target and owner, admission basis, completed-cycle state,
tick, phase, and hand step. The diagnostic is falsified if it finds neither
categorical failure, if the first failure remains unclassified, or if any prior
physical result changes.

## Risks and rollback

Duplicating winner logic in diagnostics could make the explanation disagree
with behavior. Compute typed resolution once and derive both behavior and trace
from it. Rollback removes the event, evidence mapping, and diagnostic campaign;
no learner state or physical migration is required.

## Open decisions

None.
