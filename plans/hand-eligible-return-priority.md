# Eligible return priority composition

```text
one physical moment
      |
      +--> eligible live returns ----+
      |                               +--> existing admission and one effect
      `--> ineligible live returns --+
```

## Outcome

Preserve the falsified temporal-only and temporal-plus-cohort variants, then add one
separate experimental composition that evaluates consequence-born live returns before
ineligible live returns within the same physical moment. Prevent a newly opened
ineligible return from consuming an origin before an older valid return, while keeping
one admission and one physical update per origin/moment. Re-run the temporal gate and
only then inspect boundary novelty. No default or authority changes.

## Authority

- Path: `research/constitution.md`, lessons `LP-023`, `LP-028`, `LP-032`, `RM-003`,
  `plans/hand-consequence-born-closure-eligibility.md`, and
  `plans/hand-consequence-cohort-closure.md`
- Revision: temporal-only plan
  `1c56f53667d7fb05de8b4c74bda8c629fe9bf7e3b4221b77c2129a10a9e709be`;
  cohort composition plan
  `1a719577ecf5afae4e603115768f20015dc084f56e6f7df8063dfa6242105d5f`;
  preflight counterexample: the second valid delayed surface reaches an eligible old
  return, but an ineligible newly opened return admits the origin first.

## Model

For each immutable outcome firing in one moment, compute a local boolean: at least one
carried origin has `birth_tick > return.opened_tick`. The new protocol performs a
stable partition of outcome firings into eligible then ineligible, preserving serial
order within each class. Existing return admission, same-moment duplicate suppression,
one physical outcome update per firing, per-origin reverse consolidation, and cohort
retirement then run unchanged.

The partition is a pure transformation over actual firing and live-link state. It does
not choose an action, surface, movement, learner, or answer. Old variants retain their
original iteration order and counterexamples.

## Invariants

- Eligibility priority changes only candidate outcome iteration order within one
  physical moment.
- Stable order is preserved among two eligible or two ineligible firings.
- No origin is admitted twice and no physical outcome update is multiplied.
- Pre-output and equality cases remain ineligible; valid later consequence wins over a
  competing newly opened ineligible return.
- Cohort retirement occurs only after all same-moment outcomes are processed.
- Temporal-only and temporal-plus-cohort failures remain reproducible.
- Lineage membership, impulse, strength, output, competition, lifetime, owner memory,
  construction threshold, old protocols, replay, and quiescence remain intact.
- Boundary and hand rungs remain blocked until the complete temporal composition
  survives.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Add candidate routing in `truelearner/crates/core/src/core.rs` and `physics.rs`.
- Add a small eligible-first ordering helper in `outcome.rs`.
- Add focused tests in `truelearner/crates/core/tests/harness_boundary.rs`.
- Complete the isolated experiment and frozen campaign under
  `research/experiments/hand-consequence-born-closure-eligibility/` and
  `research/campaigns/hand-consequence-born-closure-eligibility-v1/`.
- Add candidate and verification receipts.
- Exclude semantic prioritization, cross-moment sorting, changed admission predicates,
  extra physical updates, boundary-novelty solves, hand-world changes, adoption, and
  authority promotion.

## Development style

TDD. Preserve both earlier negative arms, then require the complete composition to
reject repeated coactivity, construct after two valid delayed rounds, replay exactly,
and retain duplicate/disconnected/equality controls before implementing eligible-first
ordering.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core consequence_born_`
  checks both frozen failures and the complete composition.
- `cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`
  checks all declared arms and conditional gates.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves the core regression suite.
- `cargo fmt --manifest-path truelearner/Cargo.toml --all -- --check`,
  `cargo check --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets`, and
  `cargo clippy --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/convergence.toml`
  validate the frozen round.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap; candidate and
verification receipts record warm durations separately.

## Controls and evidence

Held-out cases are two eligible returns in reversed link order, two useful same-moment
surfaces, equality at opening, and twelve same-boundary repetitions. Negative controls
are both frozen predecessor variants, duplicate origin, disconnected and unrelated
surfaces, stale post-cohort delivery, immutable parent evidence, old protocols, replay,
quiescence, and evaluator isolation. Killing falsifiers are an ineligible return
winning over an eligible one, any order change within one eligibility class, duplicate
credit, multiplied strengthening, pre-output construction, lost delayed construction,
old behavior change, replay inequality, non-quiescence, or advancement after a failed
gate. Expected evidence is five arm artifacts and results, convergence, and validated
factory receipts.

## Risks and rollback

Sorting whole outcomes can make one firing with mixed lineage hide another member;
classification uses any eligible member, while per-origin admission and closure
eligibility remain independent. Reordering can accidentally change physical effect if
multiple distinct outcomes are admitted; one-effect and output equality controls detect
it. Rollback removes only the new eligible-priority variant and ordering helper while
preserving both earlier negative variants and evidence.

## Open decisions

None.
