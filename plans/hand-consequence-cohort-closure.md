# Consequence-born cohort closure composition

```text
output participation ---> return cohort opens
                              |
first consequence moment ----+-- closes cohort after all same-moment members
                              |
origin birth > cohort opening +-- permits closure evidence
```

## Outcome

Preserve the falsified temporal-only protocol and add a separate experimental
composition that closes every sibling return opened by one physical output after the
first accepted consequence moment. Together with consequence-born timing, reject
repeated pre-output coactivity attaching to an unanswered old sibling while retaining
valid delayed consequence and same-moment multisurface credit. Run the temporal gate,
conditional boundary-novelty gate, and only advance if each prerequisite survives.
This remains discovery evidence and changes no default.

## Authority

- Path: `research/constitution.md`, lessons `LP-023`, `LP-028`, `LP-032`, `RM-003`,
  `research/campaigns/hand-lineage-construction-selectivity-v1/convergence.toml`, and
  the failing core preflight
  `consequence_born_closure_rejects_preoutput_coactivity_and_keeps_delayed_consequence`
- Revision: source `b94482267e96baffecc9576ddb6878918d9a4974`; parent convergence
  `9d1eb1461931fa6159e3955d39edd814e43531f266c2be66c184b2f0df2a8915`;
  temporal-only plan
  `1c56f53667d7fb05de8b4c74bda8c629fe9bf7e3b4221b77c2129a10a9e709be`.

## Model

A return cohort is the sorted set of live modulatory links sharing the same physical
outcome source and local opening tick. It is discovered from live topology, not stored
as action, episode, or semantic identity. Accepted returns collect their cohort keys;
after the whole physical moment is processed, each collected cohort retires once.
This preserves all useful same-moment members while preventing a later incidence from
claiming an unanswered sibling of the old output.

`RecursiveLearnerConsequenceBornClosure` remains the temporal-only negative arm. New
`RecursiveLearnerConsequenceCohortClosure` composes its strict birth-after-opening
predicate with cohort retirement. Cohort lookup and retirement are total local
functions; mutation occurs only after return processing for the moment completes.

## Invariants

- Temporal-only behavior and its old-sibling counterexample remain reproducible.
- Cohort identity uses only return source and opening tick from actual live links.
- One accepted consequence moment closes the cohort after, never during, processing.
- Multiple distinct useful origins arriving in the same moment remain admissible;
  duplicates still count once.
- Later origins cannot use any retired sibling from that cohort.
- Valid delayed consequence following a fresh output constructs after two distinct
  output/consequence rounds.
- Cohort closure changes no impulse, strength calculation, competition, origin
  lineage, construction threshold, or old protocol behavior.
- Disconnected, unrelated, pre-output, equality, stale, replay, and quiescence
  controls remain negative or intact as declared.
- Boundary novelty and hand execution remain conditional on prior gates.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Extend candidate protocol routing in `truelearner/crates/core/src/core.rs` and
  `physics.rs`.
- Add local cohort collection/retirement and diagnostics in `outcome.rs` and
  `trace.rs`.
- Complete the temporal metadata work already scoped by
  `plans/hand-consequence-born-closure-eligibility.md`.
- Add focused tests in `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/hand-consequence-born-closure-eligibility/`, its frozen
  campaign, convergence, and factory receipts.
- Exclude old protocol mutation, semantic action grouping, extended lifetime,
  construction-threshold changes, boundary-novelty solves, hand-world changes,
  defaults, adoption, and authority promotion.

## Development style

TDD. Preserve the failing temporal-only assertion as an explicit negative arm, then
add composition tests for repeated coactivity rejection, valid two-round delayed
construction, same-moment multisurface preservation, exact cohort retirement, old
protocol equality, replay, and quiescence before implementing cohort closure.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core consequence_born_`
  checks temporal-only falsification and temporal-plus-cohort survival.
- `cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`
  checks inherited evidence, temporal-only counterexample, composition, controls,
  ablation, and the conditional boundary gate.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves active core contracts.
- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check`,
  `cargo check --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets`, and
  `cargo clippy --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/convergence.toml`
  validate frozen evidence and fan-in.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap; candidate and
verification receipts record warm durations separately.

## Controls and evidence

Held-out cases are two useful surfaces in one consequence moment, reversed surface
order, equality at cohort opening, and twelve repetitions of one exact boundary.
Negative controls are temporal-only old-sibling reuse, cohort-only absence of temporal
gating, duplicate origin, disconnected and unrelated surfaces, stale later delivery,
old protocols, immutable parent artifacts, replay, quiescence, and evaluator
isolation. Killing falsifiers are loss of a same-moment useful member, survival of a
later old-sibling return, pre-output construction, failure of valid delayed
construction, changed old output or strength, invented cohort membership, replay
inequality, non-quiescence, or advancement past a failed gate. Expected evidence is
five arm artifacts and results, one convergence record, and validated factory
receipts.

## Risks and rollback

Closing links while iterating could suppress a valid same-moment member; collect
cohort keys first and retire only after all incidences finish. Grouping solely by tick
could cross outputs; include the physical outcome source. Never-closing unrelated
cohorts could retain stale credit; held-out stale delivery probes exact retirement.
Rollback removes the composition variant and cohort logic while preserving the
temporal-only negative arm and all frozen evidence.

## Open decisions

None.
