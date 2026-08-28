# Consequence-born closure eligibility solve

```text
external incidence --records birth tick---> causal lineage
participating output --opens exact return--> local opening tick
                                               |
                        birth > opening -------+--> closure evidence
```

## Outcome

Add a separate experimental recursive protocol that preserves the existing causal
lineage but admits learner-construction evidence only when the returned physical
origin began after the exact participating return opened. Reject pre-output
current-state coactivity while retaining valid delayed consequence, ordinary outcome
strengthening, reverse consolidation, duplicate and disconnected controls, exact
replay, and natural quiescence. If the temporal gate survives, measure boundary
novelty under the unchanged construction law and stop before the hand if that next
gate fails. This is discovery evidence, not adoption.

## Authority

- Path: `research/constitution.md`, `lessons.md`,
  `research/programs/learner/lessons.toml`, and
  `research/campaigns/hand-lineage-construction-selectivity-v1/convergence.toml`
- Revision: source `b94482267e96baffecc9576ddb6878918d9a4974`;
  convergence SHA256
  `9d1eb1461931fa6159e3955d39edd814e43531f266c2be66c184b2f0df2a8915`;
  schedule `5f635972293f6bae745c00a20371f9a18f1a17c2ab8eddf41c6cde625fe164b3`;
  link `17792654f8aacfeac616aa2fb482729cc25217684c02b520ab889ca36b385846`;
  reuse `23bec91274e14d11e2c9b03fa92ea8a796bd820d76765de1b1dd54ee4f37714e`;
  outcome `5588c24f9bdeae80ce2c4c70cfe3ca5365916a5011a842c764d0ccd794492945`.

## Model

`CausalLineage` becomes a sorted unique map from actual physical origin to its latest
external birth tick. Singleton input records `arrival_tick`; merge is pointwise
maximum for equal origins and sorted union for distinct origins. Selection after
return admission preserves the original member and birth tick.

Every link records its local opening tick when allocated or reused. The new
`RecursiveLearnerConsequenceBornClosure` protocol composes one pure predicate:
`origin_birth_tick > return_opened_tick`. Return admission, physical outcome update,
reverse consolidation, and consequence memory remain unchanged; only the final
causal-closure observation is gated. Existing protocols ignore the predicate.

The experiment composes immutable parent evidence, a matched old/new temporal
fixture, negative controls, an old-protocol ablation, and—only after temporal
survival—the already declared repeated-boundary/two-surface diagnostic. Effects are
limited to Harness mutation and artifact I/O; the eligibility comparison is local and
total.

## Invariants

- Every lineage member comes from an actual firing; no origin, tick, movement,
  direction, anatomy, or expected answer is reconstructed.
- Lineage union remains associative, commutative, and idempotent; equal origins retain
  the latest actual birth tick.
- Birth/opening metadata changes neither impulse, strength, output count, competition,
  return lifetime, admission, reverse consolidation, nor old protocol behavior.
- Equality is ineligible: an incidence born at or before return opening cannot become
  evidence for that return.
- A later valid incidence may supply one closure observation; duplicate membership in
  one moment cannot supply two observations.
- Disconnected, unrelated, stale, pre-output, and same-tick controls construct none.
- Checkpoint replay includes both birth and opening ticks exactly; all runs settle
  naturally.
- Boundary novelty and the hand run remain blocked unless temporal eligibility passes.
- A surviving candidate establishes neither necessity, default adoption, nor organism
  authority.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Modify the causal-lineage representation and propagation in
  `truelearner/crates/core/src/schedule.rs`, `input.rs`, `junction.rs`, `output.rs`, and
  `outcome.rs`.
- Record link opening time in `link.rs` and expose the new candidate protocol and
  diagnostics through `core.rs`, `physics.rs`, and `trace.rs`.
- Add focused neutral tests to `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/hand-consequence-born-closure-eligibility/` and
  `research/campaigns/hand-consequence-born-closure-eligibility-v1/`.
- Add this plan and candidate/verification receipts.
- Exclude changes to existing protocol semantics, construction threshold, owner
  identity, PQLC, hand topology, frozen artifacts, program authority, defaults, and
  any hand retry after a failed boundary gate.

## Development style

TDD. First encode lineage birth algebra, pre-output rejection, delayed-consequence
acceptance, equality rejection, unchanged old candidate behavior, replay, and
quiescence in neutral core tests. Then implement the new protocol, add the isolated
experiment, preregister its campaign before the single shared evidence run, and
converge without repairing a failure.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core consequence_born_closure_`
  checks birth/opening ordering, delayed-positive, coactive/equality negatives,
  duplicate/disconnected controls, old behavior, replay, and quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`
  checks the immutable parent, complete temporal candidate, old-protocol ablation,
  control matrix, and conditional boundary gate.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves all active core contracts.
- `cargo fmt --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets`, and
  `cargo clippy --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/hand-consequence-born-closure-eligibility-v1/convergence.toml`
  validate preregistration and total fan-in.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-consequence-born-closure-eligibility/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap; candidate and
verification receipts record warm durations separately.

## Controls and evidence

Held-out cases are equality at the return-opening tick, two distinct origins with
different birth ticks, reversed surface order, and a twelve-round same-boundary
horizon. Negative controls are the immutable selectivity artifacts, old causal-lineage
coactivity, disconnected and unrelated surfaces, duplicate same-moment origin, stale
return, unchanged impulse, replay, quiescence, and the semantic firewall. Killing
falsifiers are pre-output closure evidence, loss of valid later consequence, changed
old behavior, invented or reordered lineage, multiplied physical effect, failed
checkpoint equality, non-quiescence, or advancement after a failed prerequisite.
Expected evidence is five arm artifacts and result envelopes, one convergence record,
and validated factory receipts.

## Risks and rollback

Using arrival rather than birth time would misclassify a pre-output signal delayed in
the network, so birth is recorded once at external entry and preserved thereafter.
Selecting admitted origins could accidentally reset birth time; the lineage filter
must retain original members. Link reuse could retain an old opening tick; allocation
must overwrite it for every generation. Checkpoint layout changes for candidate state,
but exact within-version replay is required and historical byte compatibility is not
claimed. Rollback removes the new protocol path, time metadata, experiment, campaign,
plan, and receipts while leaving frozen evidence untouched.

## Open decisions

None.
