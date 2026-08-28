```text
candidate path inputs -> completed-cycle read -> exact witness links
          |                       |                    |
          +-----------------------+--------------------+
                                  v
                 construction projection factors every born admission
```

# Verify the complete construction-naturality contract

## Outcome

Make completed-cycle diagnostics retain the exact live link generations that
supplied each candidate's consequence tick, then freeze and test the complete
construction-composition hand contract: ten admitted completed cycles,
twenty-four choices, and an exact projected physical witness for every
construction-born admission. If the contract and selectivity controls survive,
record clean opt-in integration without changing accepted default authority.

## Authority

- Path: `research/campaigns/hand-unconditional-existing-trace-localization-v1/convergence.toml`
- Revision: `sha256:df9d51847988b5eeda2dfab116464e2aca029d392710e6d5071e1bd25dc57999`

## Model

A candidate consequence read is an arrow from a set of live path firings to an
optional latest consequence tick. Refine its diagnostic result to the product
of that tick and the sorted exact `(link, generation)` witnesses attaining it.
The behavior continues to consume only the tick; trace emission additionally
retains the witnesses.

The complete-contract fold composes three existing facts: learner construction
projected a same-tick consequence onto an exact link generation; a later
completed-cycle candidate read that original tick from the same live generation;
and that candidate was uniquely admitted by completed-cycle evaluation. The
fold is total and returns typed missing, mismatched, or composed results. A
construction-born admission exists when its owner has a recorded construction
at exactly its consequence tick; ordinary later cycle admissions remain outside
this narrower proof obligation.

## Invariants

- Choice ranking, recency, memory writes, learner construction, link lifetime,
  adapter force, and hand behavior do not change.
- Witness links are derived only from the candidate's actual live completing
  path firings and the same owner-local consequence lookup already used for the
  decision.
- Organism-owned reads use the existing live link consequence and exact firing
  generation; learner-owned reads use existing private consequence memory.
- Witnesses are sorted and deduplicated; missing or ambiguous evidence is
  represented explicitly rather than guessed.
- The full snapshot is retained before contract interpretation.
- No semantic position, direction, hand, expected target, score, or evaluator
  answer enters the learner or fold.
- Frozen predecessor source and artifacts remain unchanged.
- Accepted default authority is unchanged; successful integration applies only
  to the opt-in construction-composition candidate.

## Scope

- Extend `CompletedCycleContinuationEvaluated` and its public research evidence
  projection with exact consequence-witness link generations.
- Add pure core tests proving exact owner-local witness selection and rejection
  of wrong generation, stale, unrelated, dead, and non-completing inputs.
- Add one successor experiment and campaign that run the unchanged opt-in hand
  once, retain the full trace, and verify every construction-born admission.
- Rerun the existing same-tick construction, replay, quiescence, and lineage
  selectivity controls.
- Exclude ranking, memory, protocol-default, adapter, and authority changes.

## Development style

TDD. Add failing pure witness-selection tests first, implement the diagnostic
projection, then test the offline contract fold with synthetic construction and
completed-cycle records. Compile the one-shot hand runner without executing it.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core construction_outcome_composition`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core completed_cycle_consequence_witnesses`
- `cargo test --locked --manifest-path research/experiments/hand-complete-construction-naturality-contract/Cargo.toml`
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-complete-construction-naturality-contract-v1/campaign.toml`

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
It must remain strictly under 10 seconds. Cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases include a nonwinning completed-cycle candidate, a wrong link
generation, a same-tick unrelated link, a dead link, an older stale consequence,
and a live non-completing input. Negative controls preserve the immutable parent,
all exact external hand metrics, replay, natural quiescence, zero propagation
exhaustion, fresh child memory, sibling isolation, deallocation, replacement,
ambiguity, and the unchanged accepted protocol matrix. The frozen candidate
survives only when all construction-born admissions factor through at least one
exact projected witness and every selectivity control passes.

## Risks and rollback

The primary risk is accidentally letting richer diagnostics influence ranking.
The decision continues to compute and store the same tick before witnesses are
derived solely for tracing, and equality tests compare complete runs with trace
disabled and enabled where applicable. Another risk is treating every later
cycle as construction-born; the fold requires exact equality between the
candidate consequence tick and its owner's construction tick. Rollback removes
the new evidence field, pure helper, successor experiment, and campaign.

## Open decisions

None.
