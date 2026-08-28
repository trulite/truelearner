# Hand causal-lineage preservation solve

```text
actual firing origins --sorted union--> temporary causal lineage
        |                                      |
        | one physical impulse                 | distinct attribution
        v                                      v
 unchanged integration                 return admission per origin
                                               |
                              one strengthening, reverse attempt per origin
```

## Outcome

Add a separate experimental recursive protocol that carries the sorted, duplicate-free
lineage of physical origins that actually contributed to a firing. When several
origins meet, preserve their lineage while leaving the existing scalar origin,
competition, impulse, strength, timing, and old protocols unchanged. At a physical
return, evaluate each lineage member once, apply the returned strength once per return
firing, and attempt reverse consolidation for each admitted member. Test the complete
candidate in the unchanged anonymous hand world through truthful surface admission,
same-lineage closure evidence two, and construction. This produces discovery evidence
only; it does not adopt the protocol or change a default.

## Authority

- Path: `lessons.md`, `research/programs/learner/lessons.toml`,
  `research/constitution.md`,
  `research/campaigns/hand-same-lineage-closure-renewal-v1/convergence.toml`, and
  the current `truelearner-core` firing/return path
- Revision: source revision `b94482267e96baffecc9576ddb6878918d9a4974`;
  lessons SHA256
  `7f557e4b11fbc9f1bf404e333d93cb15378b69f66917ef34dee94690f6131fb5`;
  program lessons SHA256
  `e3076d1f3c156eb8ce744763ae943f36d203f9d3f62167a96b4ce4a6cd26e3ad`;
  constitution SHA256
  `f0f934a91d6024b0c327a6efa416a67504f8e2fd6dc9e879d443a8474ac77b08`;
  parent convergence SHA256
  `d07822a401fdcdf91e108a0cb46686706d9d4bea26c2bc519510d618d79c7287`;
  schedule SHA256
  `571529426525e97b6d2546fa2aee851af50f6189a33b35051c5e75f7a8958a4f`;
  junction SHA256
  `0edcff610c1b5aee3b5b6f5d006ba52ebfefb7b167b07847abb7b1aaef289ad4`;
  outcome SHA256
  `b32429c951fe2913499485458fdd3b0ec854bb7c7de6f7721be2906e5fc65c01`

## Model

`CausalLineage` is a private non-empty sorted set of physical-origin identifiers.
Singleton construction maps one actual external incidence to its lineage. Merge is
set union and must be associative, commutative, and idempotent. Its lifetime is the
ordinary pending firing graph: it is not stored as episode, action, anatomy, or policy
memory and disappears at quiescence.

`Firing` and `Fired` carry an optional lineage. Only the new
`RecursiveLearnerCausalLineage` protocol creates, merges, propagates, reads, and
checkpoints it. Existing protocols continue to use the current scalar
`origin_physical` rule. The new protocol deliberately retains that scalar as the
existing competition scope, so causal-wave sparsity and motor choice do not read the
new set.

At a junction, the candidate maps all incoming lineages to their sorted union without
changing summed strength. Output fanout clones attribution onto each actual outgoing
firing but retains one ordinary impulse per edge. At a modulatory return, the candidate
evaluates every distinct lineage member against the same live return. Accepted members
are grouped into one returned physical effect: local participating structure is
strengthened once, learner consequence is recorded once per distinct owning learner,
and reverse consolidation is attempted once per accepted origin. Qualified local
propagation carries only the admitted lineage.

The complete candidate includes all existing recursive-construction, finite-return,
origin-scoped competition, participation, owner-local memory, reverse consolidation,
and construction mechanisms. It excludes scalar replacement, first-member selection,
semantic body structure, reset, longer lifetime, lower construction threshold,
changed PQLC, and evaluator-supplied identity.

The successor experiment freezes the prior negative reference and rebuilds the same
anonymous hand topology under only the new protocol. Its upward ladder is: lineage
union, unchanged impulse, truthful surface return evaluation, delivered-surface
admission, reverse consolidation, same closure key at evidence two, construction,
replay, quiescence, and fixed-horizon behavior. Dependent stages stop at the first
failure. Removal ablations run only if the complete candidate survives.

## Invariants

- Existing `RecursiveLearnerConstruction` behavior, trace classifications, tests, and
  frozen research artifacts remain unchanged.
- A lineage contains only origins carried by actual incoming firings; junction IDs,
  expected surfaces, intended actions, and evaluator labels are never inserted to
  repair it.
- Lineage union is sorted, unique, order-independent, associative, and idempotent.
- Lineage membership never changes impulse, material strength, threshold integration,
  output count, timing, causal wave, link lifetime, or competition scope.
- One physical return applies local strengthening once even when it admits multiple
  lineage members. Duplicate lineage members cannot duplicate admission, consequence,
  consolidation, or closure credit.
- Each distinct admitted origin retains existing direct/local, owner, generation,
  stale, duplicate, and same-moment admission rules.
- Qualified local propagation carries admitted actual lineage rather than reconstructing
  it from a junction after the event.
- Candidate checkpoint replay is exact, natural quiescence is required, and temporary
  lineage does not persist after the pending physical computation settles.
- The unchanged hand topology, origins, reflection, capacities, and sixteen-step horizon
  contain no semantic anatomy, correct direction, target, score, or stage signal.
- Discovery survival establishes neither necessity nor adoption. A failed candidate is
  frozen and no downward ablation is interpreted.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Modify `truelearner/crates/core/src/schedule.rs`, `core.rs`, `physics.rs`, `input.rs`,
  `junction.rs`, `output.rs`, `outcome.rs`, `learner.rs`, `choose.rs`, and `trace.rs`.
- Modify `truelearner/crates/core/tests/harness_boundary.rs` for neutral core laws.
- Add `research/experiments/hand-causal-lineage-preservation/` as a new isolated crate.
- Add a preregistered successor campaign, arm manifests, immutable artifacts, result
  envelopes, and convergence under
  `research/campaigns/hand-causal-lineage-preservation-v1/`.
- Add this plan and candidate/verification receipts under `factory-artifacts/`.
- Exclude changes to old experiment crates, old campaigns or artifacts, defaults,
  adopted protocols, construction thresholds, PQLC, hand topology, program authority,
  Academy, and benchmarks beyond the declared anonymous hand fixture.

## Development style

TDD. First add neutral core tests for lineage algebra, unchanged scalar competition,
one-impulse/one-strengthening preservation, multi-origin return decisions, order,
duplicates, unrelated origin, replay, and opt-in tracing. Then implement the new
protocol. Finally preregister and implement the independent successor experiment,
freeze one evidence run, and converge every declared arm without repairing a failure.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core causal_lineage_candidate_preserves_actual_origins_without_changing_impulse`
  checks union and physical-effect preservation against the scalar protocol.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core causal_lineage_candidate_is_order_independent_and_duplicate_free`
  checks the lineage algebra and return evaluation order controls.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core causal_lineage_candidate_preserves_replay_and_old_protocol`
  checks exact replay and the frozen scalar fallback.
- `cargo test --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml lineage_primitive_survives_controls`
  checks truthful membership, unrelated/outcome rejection, unchanged impulse, and
  duplicate-free admission before the hand run.
- `cargo test --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml complete_hand_candidate_is_totally_classified`
  requires a total survived/falsified result at the earliest declared stage without
  forcing success.
- `cargo test --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml inherited_negative_reference_is_unchanged`
  checks the frozen predecessor classification and counterexample.
- `cargo test --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml --lib -- --test-threads=1`
  is the representative new experiment suite.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves the active core contract.
- `cargo run --quiet --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml -- --all --output-dir research/campaigns/hand-causal-lineage-preservation-v1/artifacts`
  performs the single shared discovery measurement and writes every arm artifact.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-causal-lineage-preservation-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/hand-causal-lineage-preservation-v1/convergence.toml`
  validate preregistration and total fan-in.
- `cargo fmt --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml --all-targets`, and
  `cargo clippy --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-causal-lineage-preservation/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap; candidate and
verification receipts record warm durations separately.

## Controls and evidence

Held-out cases are reversed input order, duplicate fan-in, one local plus one unrelated
origin, an outcome-origin input, a single-origin return, independently driven far
outputs, and qualified local propagation. The positive reference is the established
scripted two-cycle closure. Negative controls are the frozen scalar hand failure,
unchanged old recursive protocol, duplicate/stale/disconnected/shuffled returns,
fixed horizon, replay, quiescence, capacity, and no-consequence release. Killing
falsifiers are invented lineage membership, changed impulse/output/choice, multiplied
strengthening, loss of old classifications, missing delivered-surface admission,
surface admission without reverse consolidation, changed closure identity, failure to
construct, replay inequality, non-quiescence, semantic leakage, or over-budget work.
Expected evidence is four arm artifacts and result envelopes, one convergence record,
and validated factory receipts. If the complete candidate survives, a later round may
ablate lineage union and individual-member return evaluation separately; otherwise no
necessity claim is admissible.

## Risks and rollback

Cloning lineages on fanout can increase traced candidate memory and work with the active
causal frontier. The fixed hand capacity, cost observations, quiescence, and a neutral
fanout control expose runaway growth. Multiple admitted owners can accidentally
multiply physical learning; grouping the returned effect and testing exact strength
deltas prevents that. A new protocol variant can leak into old ownership predicates;
central protocol predicates and old-protocol equality tests detect omissions. Rollback
removes only the new protocol path, private lineage field, successor experiment,
campaign, plan, and receipts; frozen scalar behavior and evidence remain.

## Open decisions

None.
