```text
immutable parent choices ----+
                             +-> pure ordinal comparison -> first added admission
candidate existing trace ----+                              |
                                                            v
                                      construction projection -> live path link
```

# Locate the first construction-projection admission

## Outcome

Retain the existing ordered choice and physical trace from one unchanged
`RecursiveLearnerConstructionOutcomeComposition` hand run, compare it with the
immutable completed-cycle parent, and identify the first added completed-cycle
admission plus the construction projection and live path link that supplied it.
No learner or adapter behavior changes.

## Authority

- Path: `research/campaigns/hand-construction-outcome-composition-v1/convergence.toml`
- Revision: `sha256:7ed858f0726ad449d59106f561f1c97623e756e78d0075a0406a2b3755356083`

## Model

Each hand choice is an ordered record keyed by ordinal, hand step, tick, and
phase. A completed-cycle admission exists exactly when its state is `Unique`.
The pure comparison walks parent and candidate choices in order and returns the
first ordinal where the parent is not `Unique` and the candidate is `Unique`.
Length, identity, or earlier ordering mismatch is a typed insufficient result.

From that choice, a second pure fold walks backward through the candidate's
already-existing trace to the latest learner construction. It composes adjacent
`LearnerConstructed` and `LearnerConsequenceRecorded` events with same-generation
drive and lineage events that complete the newly unique target. The result
names the construction tick, owner, consequence tick, projected links, and the
subset that physically completes the target at the added admission.

The preregistered prediction is ordinal seven, hand step six, tick forty-seven:
learner three gains a unique target-eleven completed cycle from tick-forty-four
construction projection while the selected target remains eleven. The exact
participating link is left for the trace to decide.

## Invariants

- The prior negative artifacts, protocol, adjudication, and convergence remain
  immutable.
- Core, learner state, recency, choice, path, outcome, and adapter behavior do
  not change.
- The candidate runs once; the parent is read only from immutable artifact
  `e4c38011...` and is not rerun.
- Comparison uses structural ordinal, tick, phase, owner, target, link,
  generation, and consequence facts only.
- Position, direction, hand meaning, expected action, and evaluator answer do
  not enter the comparator.
- Missing or conflicting existing evidence returns a typed insufficient result
  rather than an inferred cause.
- Full ordered choices and the decisive existing-event slice are persisted so
  no further organism rerun is needed for this divergence.

## Scope

- Add one successor research experiment and diagnostic campaign.
- Reuse public evidence types from
  `developmental-hand-construction-admission` and the immutable parent JSON.
- Add a pure typed comparator with synthetic tests for added admission, identity
  mismatch, no divergence, and missing link composition.
- Exclude all changes to `truelearner-core`, the hand adapter, prior experiments,
  frozen artifacts, selection, recency, learner memory, and authority status.

## Development style

TDD. Build synthetic parent/candidate choice sequences and trace slices first,
then add the one-shot runner and compile it without executing the hand.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/hand-construction-projection-admission-retention/Cargo.toml`
  proves the pure comparator distinguishes the first added admission, no
  divergence, identity mismatch, and insufficient physical composition.
- `cargo test --locked --manifest-path research/experiments/hand-construction-projection-admission-retention/Cargo.toml --no-run`
  proves the frozen runner compiles without consuming its valid hand run.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-construction-projection-admission-retention-v1/campaign.toml`
  proves the successor protocol and arms are frozen before execution.

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path research/experiments/hand-construction-projection-admission-retention/Cargo.toml`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases are an added `Unique` state at a later ordinal, a candidate
choice identity mismatch, equal sequences, and a unique choice with no composed
projected link. Negative controls are exact parent artifact hash, exact prior
candidate external summary, replay, quiescence, zero propagation exhaustion,
and unchanged core and adapter source.

The diagnostic survives when it locates the first added admission, preserves
all earlier choice identities, names the preceding construction and original
consequence tick, and proves at least one same-generation projected link
completes the newly unique target. It is falsified by a different first ordinal,
an earlier target change, or no physical link composition. Missing retained
facts make it inconclusive.

## Risks and rollback

The main risk is confusing the first choice-state difference at tick twenty-
three (`Missing` to `Stale`) with the first added admission. The comparator
therefore searches specifically for non-`Unique` to `Unique`. Another risk is
equating construction links with later path participation; the fold requires
recorded downstream composition. Rollback removes only the successor diagnostic
crate and campaign.

## Open decisions

None.
