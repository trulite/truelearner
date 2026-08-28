# Give each recursive learner fresh return memory

```text
shared prototype + causal boundary
              |
              v
        construct child
          /         \
 parent memory     child memory
   preserved          empty
                         |
             actual return participation
                         v
        (link, generation, origin) accepted once
```

## Outcome

Give every constructed learner private finite-return memory while preserving the
same learning prototype and causal membership. A child must accept fresh physical
returns rejected by its parent's history, reject repeated use within its own live
return-link generation, checkpoint exactly, and use two fresh observations to
construct an adjacent descendant through depth three. This establishes fresh-memory
reentry and recursive construction only; later control, consequence, recruitment,
sibling, body-composition, and authority claims remain excluded.

## Authority

- Path: `research/campaigns/recursive-learner-fresh-memory-v1/protocol.toml`,
  `research/campaigns/recursive-learner-fresh-memory-v1/campaign.toml`, and
  `research/campaigns/recursive-learner-reentry-v1/convergence.toml`
- Revision: owner-aware candidate tree
  `72e3c1560e68330450321e96db5039159c7959506164c7dd798104e654f72477`;
  frozen fresh-memory protocol SHA256
  `623e4b3c57c862a7d8df6b21023c42841a75901f724e0e1d680007aa95b6732a`

## Model

Separate the shared learner prototype from learned return history. `LearnerState`
continues to hold inherited construction law implicitly and exact causal membership,
and gains a private sorted set of accepted return observations. Each observation is
the concrete triple `(LinkId, Generation, origin_physical)`; ownership is implicit in
the containing learner. A newly constructed learner always starts with an empty set.

For every live modulatory return link, scheduling first resolves the deepest learner
whose sorted construction lineage contains it. At physical-origin admission, the
truthful live origin junction resolves the deepest learner whose causal junction
membership contains that origin; this covers renewed return links created after the
fixed construction lineage. No owner means the organism and retains the existing
`LinkState.return_origins` behavior. A learner owner means admission reads only that
learner's private set for the link's current generation. A valid admission appends
exactly one sorted entry to that learner and does not clear, copy, or mutate the
parent's memory. Link retirement increments its generation, so physically renewed
participation is fresh while old entries remain historical and inert.

The arrows are live return link to physical owner, owner plus generation to local
eligibility, truthful origin incidence to one accepted entry, accepted return to
reverse consolidation, and consolidated return to the existing owner-aware closure
transition. These compose repeatedly:

```text
organism return memory -> learner 1 with empty memory
learner 1 return memory -> learner 2 with empty memory
learner 2 return memory -> learner 3 with empty memory
```

Wrong protocol, missing or stale link generation, duplicate owner-local entry,
invalid direct/local origin, missing participating path, insufficient closure
evidence, and physical resource exhaustion remain no-ops. Add trace events for the
return scheduling and origin-admission decisions with owner, link, generation, and
accepted status; no public operation can name or write learner memory.

## Invariants

- CORE1, every accepted protocol, frozen root construction, and the owner-aware
  closure prerequisite retain exact behavior, replay, quiescence, and cost.
- Every learner has the same construction law. A child inherits causal junction and
  link membership but no accepted-return entry, learned action, preference, credit,
  confidence, or parent lifetime state.
- The organism continues to read and write only `LinkState.return_origins`; a child
  admission does not clear, copy, or append to that parent-global memory.
- Learner-local identity is exactly causal-origin membership plus live return-link
  ID and generation plus truthful physical origin. It contains no semantic root, child,
  depth, anatomy, action, target, score, episode, or evaluator identity.
- One learner accepts at most one origin for one live link generation. Duplicate
  incidences remain rejected; a physically renewed generation is fresh.
- Only actual participating output and finite local return can write memory. No
  causal history is reconstructed at construction or checkpoint restoration.
- Owner-local memory is sorted, unique, append-only for a generation, bounded by
  actual accepted participation, serialized byte-exactly, and included in recursive
  memory accounting.
- Owner-aware closure evidence remains unique to `(parent, surface, output)` and
  constructs at most once per owner. Parentage is adjacent, ordered, and acyclic.
- Public Harness-only tests reject single, duplicate, stale, disconnected, shuffled,
  correlated, pre-construction, and unrelated returns at every realized level.
- Stop the campaign at the first failed prerequisite; do not run later control or
  composition rungs after return admission or child-owned closure fails.

## Scope

- Extend private learner state and ownership queries in
  `truelearner/crates/core/src/learner.rs` with empty-at-construction return memory.
- Route recursive-protocol return scheduling through owner-local availability in
  `truelearner/crates/core/src/choose.rs` while leaving every other protocol exact.
- Route physical-origin admission through owner-local memory in
  `truelearner/crates/core/src/outcome.rs`; preserve the existing direct/local origin
  predicate and organism-global behavior.
- Validate and restore sorted owner-local entries in
  `truelearner/crates/core/src/snapshot.rs`, account for their capacity in memory,
  and add owner-bearing scheduling/admission events in
  `truelearner/crates/core/src/trace.rs`.
- Keep live and rebuilt arena adjacency indexes in canonical link-identity order so
  link reuse followed by checkpoint restoration preserves exact physical trace
  order.
- Add public-boundary TDD regressions in
  `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/recursive-learner-fresh-memory/` with the four frozen
  arms, controls, gating, and deterministic artifact output; then add immutable arm
  results, convergence, and factory candidate and verification receipts.
- Preserve all recursive-reentry artifacts and its refuted verification unchanged.
  Exclude changes to global `LinkState.return_origins` representation, closure
  threshold, route formation, output choice, accepted protocols, Academy,
  benchmarks, later upward rungs, default adoption, and authority evidence.

## Development style

TDD. First add one public-Harness test that constructs a root, proves its return
history remains visible, requires one child-owned admission from fresh private
memory, and rejects a duplicate before any descendant assertion. Then require two
fresh child observations, adjacent ancestry through depth three, renewed-generation
eligibility, and exact checkpoint replay. Implement only the private memory,
owner-resolution, scheduling/admission, trace, and persistence changes required by
those tests before building campaign probes.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary recursive_learner`
  establishes unchanged root behavior, preserved parent history, fresh child memory,
  duplicate rejection, generation renewal, adjacent depth-three construction, and
  exact replay through public APIs.
- `cargo test --manifest-path research/experiments/recursive-learner-fresh-memory/Cargo.toml`
  establishes all four arm entrypoints, frozen controls, first-failure gating, and
  deterministic results.
- `cargo run --quiet --manifest-path research/experiments/recursive-learner-fresh-memory/Cargo.toml -- --all --output-dir research/campaigns/recursive-learner-fresh-memory-v1/artifacts`
  emits one discovery artifact per declared arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/recursive-learner-fresh-memory-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/recursive-learner-fresh-memory-v1/convergence.toml`
  establish complete frozen lineage and fan-in after execution.
- `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --all-targets -- -D warnings`
  and both affected-manifest formatting checks establish strict Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path research/experiments/recursive-learner-fresh-memory/Cargo.toml --lib`.
Its measured execution must remain strictly under 10 seconds; record cold bootstrap
separately.

## Controls and evidence

Held-out cases are partial child memory across checkpoint, retired and renewed return
generation, depths one through three, reflection, reordered discovery, and 4, 64,
and 1024 dormant surfaces. Negative controls are the frozen owner-key-only failure,
construction-time global clearing, copied parent memory, one observation, same-
generation duplicate, stale generation, disconnected, shuffled, correlated,
pre-construction, and unrelated returns. Killing falsifiers are changed parent
history, any nonempty child memory at birth, duplicate admission, rejected renewed
generation, missing reverse consolidation or owner-local closure, non-adjacent or
depth-specific construction, replay inequality, lost quiescence, global cost growth,
or a dependent arm running after failure. Expected evidence is four artifacts and
results, one convergence, and validated candidate and independent verification
receipts. No authority evidence is produced.

## Risks and rollback

The primary risk is hiding a global reset behind apparent child freshness. Public
parent history must remain byte-for-byte unchanged while owner-bearing trace events
show the child admission. Other risks are accepting two origins from one
participation, stale-generation reuse, unbounded historical memory, ambiguous return
ownership, corrupted checkpoint order, and changes to accepted sensorimotor
behavior. Exact tuple uniqueness, generation identity, actual-membership ownership,
participation-bounded accounting, checkpoint validation, frozen references, and
negative controls detect them. Rollback removes private learner return memory and
restores the refuted owner-key-only candidate without altering preserved evidence.

## Open decisions

None.
