```text
one local non-recent unanswered return
                 |
       actual owner relation
          /              \
 same owner or         every other
 organism -> root       relation
       |                   |
 one transient UNIT       reject
       |
 existing replacement consumes exact return
```

# Recruit one root learner from organism opportunity

## Outcome

A new opt-in cumulative protocol extends the frozen bounded fresh-opportunity law by
one ancestry case: an organism-owned return may supply one physically local balanced
candidate owned by a causally constructed root learner. All other cross-owner
relations remain rejected.

## Authority

- Path: `research/campaigns/hand-opportunity-owner-boundary-v1/convergence.toml`
- Revision: `sha256:11320a3b280ad3cc6e4f73af94c2b4c7f642b22f2b4eb10423ba380c6af9a6df`

## Model

The existing local donor/recipient pipeline is unchanged except for its compatibility
predicate. `SameOwner` remains admissible. `OrganismToRoot` becomes admissible only
when actual learner ancestry proves that the recipient exists and has no learner
parent. Root-to-organism, parent/child, child/parent, siblings, unrelated roots,
grandchildren, and missing IDs remain ineligible. One transient `UNIT`, pre-existing
balanced paths, non-recent donor lifetime, deterministic sparse selection, ordinary
replacement, and exact supersession remain unchanged.

## Invariants

- The earlier strict-owner protocol stays frozen and behaviorally unchanged.
- Only `SameOwner` and `OrganismToRoot` pass; physical locality or shared origin alone
  cannot substitute for ancestry.
- One return supplies at most one recipient and existing supersession consumes it.
- No path, strength, owner, learner, memory, sign, or semantic motor preference is
  created by the law.
- Recent, answered, missing, repeated, nonlocal, child, sibling, and unrelated cases
  remain negative.
- Old protocols, replay, natural quiescence, propagation bounds, and evaluator
  isolation remain unchanged.

## Scope

- Add one cumulative protocol and parameterize the existing compatibility predicate
  by typed ownership relation.
- Add focused root, sibling/unrelated, grandchild, lifetime, reflection, and hand
  evidence plus one solve campaign.
- Exclude arbitrary cross-owner transfer, origin-only ownership, broader
  parent-to-child recruitment, random choice, strength reset, persistent memory,
  default adoption, commits, and authority promotion.

## Development style

TDD. Require the new protocol—but not its strict parent—to admit one organism-to-root
fixture. Then preserve every other relation and lifetime control before running the
unchanged hand.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary organism_root_fresh_opportunity`
  proves exact ancestry admission, strict-parent equality, cross-owner controls,
  exact consumption, reflection, replay, and quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-organism-root-fresh-opportunity/Cargo.toml`
  proves hand release/composition predicates and frozen controls.
- `cargo test --locked --manifest-path research/experiments/hand-opportunity-owner-boundary/Cargo.toml`
  preserves the diagnostic parent.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-organism-root-fresh-opportunity/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: strict same-owner transfer, organism-to-root, two unrelated roots,
  organism-to-grandchild, siblings, both parent/child directions, missing owner, and
  reflected local ordering.
- Negative controls: answered, recent, repeated exact return, nonlocal alternative,
  frozen strict protocol, old protocols, unchanged path strengths, replay,
  quiescence, propagation, and bounded work.
- Falsifiers: any forbidden relation transfers; exact return consumption fails; the
  fixture works but the hand does not execute motor 20000 and leave upper contact;
  or any integrity/cost gate fails.

## Risks and rollback

Organism-to-root recruitment could bridge independent roots. Actual recipient-root
membership plus sparse local selection and explicit two-root controls bound that
risk. Rollback removes the new protocol and relation allowance while retaining the
strict failed protocol and diagnostics.

## Open decisions

None.
