# Reenter causal closure through learner ownership

```text
fresh accepted causal return
            |
            v
deepest learner owning the surface
            |
            v
  (owner, surface, output) closure
            |
       two observations
            v
       child learner
            |
            `---- owns the next fresh incidence ---->
```

## Outcome

Let an already constructed learner receive fresh ordinary causal activity as its
own closure evidence. The same anonymous construction law must then produce one
descendant per adjacent ownership boundary through depth three, without a learner
address, evaluator route, depth signal, new public input, or change to accepted
organism physics. This establishes only recursive reentry and rung four; it does
not establish local control, nested consequence, recruitment, sibling composition,
body composition, or organism authority.

## Authority

- Path: `research/campaigns/recursive-learner-reentry-v1/protocol.toml`,
  `research/campaigns/recursive-learner-reentry-v1/campaign.toml`, and
  `research/campaigns/recursive-learner-construction-v1/convergence.toml`
- Revision: `b94482267e96baffecc9576ddb6878918d9a4974`; frozen protocol SHA256
  `92948a32126df9aaf41f9356d18d6b9205424c09b2c82fbaef2c71f039bc2718`

## Model

The states are accepted live lineage, resolved physical owner, owner-local closure,
evidence below threshold, constructed closure, and descendant-capable learner. An
accepted causal return first resolves the deepest existing learner whose sorted
membership contains the participating surface. The closure key is then
`(owner, surface, output)`, not merely `(surface, output)`. A constructed closure is
terminal for that owner, while construction changes the owner seen by the next
fresh return. Two observations therefore compose the same transition repeatedly:

```text
closure at organism -> learner 1
closure at learner 1 -> learner 2
closure at learner 2 -> learner 3
```

`CausalClosureState` already stores the owner as `parent`; no new public type is
required. Extract owner resolution as one private total query, resolve it before
closure lookup, and include it in lookup and checkpoint uniqueness. Construction
continues to copy only sorted live participating membership, appends the child after
its parent, and records the already public parent-bearing physical events. Invalid
lineage, wrong protocol, a completed closure for the same owner, insufficient fresh
evidence, and exhausted physical capacity remain no-ops.

Checkpoint restoration must preserve the parent forest and one closure per
`(parent, surface, output)`. Every referenced parent must exist and own the closure
surface; every constructed learner must match its closure's parent, surface, and
output. Mutation and tracing remain inside `Body`; the `Harness` remains the sole
system boundary.

## Invariants

- Every accepted protocol and the frozen recursive-construction rungs zero through
  three retain exact behavior, replay, quiescence, classifications, and cost.
- Owner resolution uses only constructed membership and construction order; it sees
  no root, child, depth, anatomy, action, target, score, episode, or evaluator data.
- The organism owns a surface only when no learner yet contains it. After
  construction, the deepest constructed member owns the next fresh incidence.
- Evidence is unique to `(parent, surface, output)`; one owner-local closure can
  construct at most once and incomplete evidence is never re-parented.
- Each constructed learner's parent is the owner resolved before its closure was
  first observed. The forest remains ordered, acyclic, and adjacent through depths
  one, two, and three.
- Construction records only actual fresh accepted returns over live lineage. It does
  not reconstruct history, copy durable strength, or admit single, disconnected,
  shuffled, correlated, duplicate-without-output, stale, or unrelated activity.
- Checkpoint bytes preserve learners, owner-local closures, evidence, ancestry,
  membership, next identity, trace-visible behavior, and natural quiescence exactly.
- Academy, research, and core probes use only public `Harness` and `HarnessBuilder`
  operations. No learner-selection or learner-routing API is added.
- The campaign stops at the first failed transition. Rungs five through ten and all
  downward ablations remain out of scope.

## Scope

- Change `truelearner/crates/core/src/learner.rs` only enough to resolve ownership
  before closure lookup and key closures by owner, surface, and output.
- Change `truelearner/crates/core/src/snapshot.rs` to validate owner-local closure
  uniqueness and closure-to-parent/child consistency.
- Add public-boundary regressions to
  `truelearner/crates/core/tests/harness_boundary.rs`; reuse the existing protocol,
  observations, work counters, and physical events.
- Add `research/experiments/recursive-learner-reentry/` with the three preregistered
  arm entrypoints, gating, artifact output, and no dependency on private core state.
- Add the campaign's E2B adapter batch, immutable artifacts, arm results,
  convergence, candidate receipt, and independent verification receipt.
- Exclude changes to route formation, output choice, consequence physics, closure
  threshold, learner resource policy, accepted protocols, prior campaign artifacts,
  Academy, benchmarks, default adoption, and authority evidence.

## Development style

TDD. First add a public-Harness regression that reproduces the current absorbed
post-construction return and requires owner-bearing closure evidence before any
descendant assertion. Then require one grandchild, depth three, exact replay, and
the negative controls. Implement only the owner-before-lookup transition and
checkpoint validation needed to satisfy those tests; build campaign probes after
the core boundary passes.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary recursive_learner`
  establishes unchanged root construction, owner-local evidence, exact adjacent
  ancestry through depth three, false-closure rejection, replay, and public access.
- `cargo test --manifest-path research/experiments/recursive-learner-reentry/Cargo.toml`
  establishes all declared arm names, dependency gating, controls, and result
  production.
- `cargo run --quiet --manifest-path research/experiments/recursive-learner-reentry/Cargo.toml -- --all --output-dir research/campaigns/recursive-learner-reentry-v1/artifacts`
  emits one deterministic discovery artifact per arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/recursive-learner-reentry-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/recursive-learner-reentry-v1/convergence.toml`
  establish complete protocol lineage and round fan-in after execution.
- `uv run research/runtime/dispatch_e2b.py --batch research/campaigns/recursive-learner-reentry-v1/e2b-batch.toml --dry-run`
  validates the isolated discovery adapter before dispatch.
- `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --all-targets -- -D warnings`
  and `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check` establish
  strict Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path research/experiments/recursive-learner-reentry/Cargo.toml --lib`.
Its measured execution must remain strictly under 10 seconds; record cold bootstrap
separately.

## Controls and evidence

Held-out cases are depths one through three, reflected physical order, reordered
discovery, checkpoint continuation at partial evidence, and 4, 64, and 1024 dormant
anonymous surfaces. Negative controls are the unchanged prior campaign, one
owner-local observation, matched activity before ownership changes, disconnected,
shuffled, correlated, duplicate-without-fresh-output, stale, and unrelated-surface
activity. Killing falsifiers are missing child-owned evidence, any learner routing or
semantic branch, duplicate construction for one owner, non-adjacent or cyclic
ancestry, false descendants, changed frozen references, replay inequality, lost
quiescence, global dormant-surface cost, or a dependent arm running after failure.
Expected evidence is one artifact and result for each of the three arms, the E2B
batch record, one convergence, and machine-checkable candidate and independent
verification receipts. No authority evidence is produced because this is a
discovery campaign.

## Risks and rollback

The central risk is relabeling repeated observation as recursion without a physical
boundary effect. The matched pre-construction trace and parent-bearing closure event
must show that construction changed only ownership of later actual participation;
the fixture may not route to a learner. Other risks are reusing completed evidence,
ambiguous overlapping ownership, malformed checkpoint ancestry, unbounded depth,
and mutation of prior behavior. Per-owner terminal closures, append-only ancestry,
fixed arena capacity, checkpoint validation, frozen references, and cost controls
detect them. Rollback removes the owner component from closure identity and the new
experimental campaign while retaining commit `4665502` as the preserved first-round
failure.

## Open decisions

None.
