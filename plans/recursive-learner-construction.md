# Construct recursive causal learners inside the organism

```text
participating output -> finite causal return -> repeated closure
                                                |
                                                v
                                      self-similar learner boundary
                                                |
                           owned surface + new closure -> child boundary
```

## Outcome

Add an experimental protocol in which repeated, actually participating
sensorimotor closure constructs a bounded causal learner inside the organism. A
constructed learner is a self-similar ownership boundary over the exact live causal
lineage: it uses the same learning physics as its parent and a surface already owned
by it may construct a descendant from a different closure. Run the preregistered
campaign upward, stop at the first failed capability, preserve the failure, and make
no default-authority claim.

## Authority

- Path: `research/campaigns/recursive-learner-construction-v1/protocol.toml`,
  `research/campaigns/recursive-learner-construction-v1/campaign.toml`, and
  `research/programs/learner/forecasts/causal-learner-construction-v1.md`
- Revision: `0e82c73c43b84d78f60f1e019e45e21ff1633845`; frozen protocol SHA256
  `abcaf0db09e7a47ad27e783719b09afda77448f6093db5b75d9cab003fd22173`

## Model

The categorical states are open activity, participating output, finite return,
causal closure evidence, constructed learner, and descendant-capable learner. The
existing sensorimotor arrows compose from output participation through truthful
proprioceptive incidence to reverse-path consolidation. The new experimental arrow
maps a second fresh output-return observation of the same live `(surface, output)`
closure to one learner
whose junction and link membership is exactly the union of the participating action,
return, and reverse lineages. Closure identity is physical, not semantic.

Learners form a finite parent forest. The parent of a new closure is the deepest
existing learner that owned its surface before the first observation; otherwise it
is the organism. The same closure is never re-parented or reconstructed. Because the
parent rule is applied identically to every owned surface, learner construction is
closed under nesting without a root, child, or depth branch. Construction metadata
is an observation and credit-locality boundary; ordinary firing, formation,
strengthening, decay, scheduling, and quiescence remain the existing learner laws.

Effects remain at the public `Harness`: inputs enter, outputs leave, observations
report the constructed forest, and checkpoints preserve it. Invalid or stale
lineage, missing members, false closure, insufficient repeated evidence, and resource
exhaustion produce no construction. The harness never requests, names, allocates, or
routes to a child.

## Invariants

- `Physical`, the two unanswered-return protocols, `SensorimotorCandidate`, and
  `SensorimotorSynthesis` retain exact behavior, replay, and quiescence.
- Construction exists only in `RecursiveLearnerConstruction` and only after two
  fresh output participations each produce an accepted observation of the same
  actual causal closure.
- Disconnected, shuffled, merely correlated, duplicate-after-construction, and stale
  evidence construct no learner.
- Learner and closure identity depends only on physical junctions, links, causal
  participation, and time; the organism contains no anatomy, action, target, score,
  evaluator, root, depth, or benchmark identity.
- A learner contains the exact sorted, unique, live causal lineage observed at
  construction. Its parent is fixed and acyclic; every child is governed by the same
  construction law.
- Construction does not copy durable learned strength into new topology and does not
  reconstruct historical participation.
- Checkpoint round-trip preserves closure evidence, learner ancestry, membership,
  next identity, output behavior, cost observations, and natural quiescence exactly.
- Academy, research, and core tests call only public `Harness`/`HarnessBuilder`, never
  private `Body`.
- The upward ladder stops after the first failed prerequisite. Downward ablations run
  only if the exact complete candidate passes rung ten.

## Scope

- Add learner identity, observations, physical trace events, and experimental
  protocol plumbing in `truelearner/crates/core/src/`.
- Add causal-closure state, deterministic construction, recursive parent selection,
  memory accounting, and checkpoint validation in the private organism.
- Add public-Harness TDD regressions in
  `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/recursive-learner-construction/` with all seven
  preregistered arm entrypoints, upward gating, controls, replay, and artifact output.
- Add immutable first-round artifacts, arm results, convergence, candidate receipt,
  and independent verification receipt.
- Exclude default adoption, Academy and benchmark changes, evaluator-created
  learners, semantic ports, separate per-anatomy controllers, production authority,
  and repair of frozen evidence.

## Development style

TDD. First add public-Harness tests for repeated true closure, false closure,
single-construction identity, recursive parent selection, checkpoint replay, and
unchanged default protocols. Then implement the smallest organism state and causal
transition that makes those tests pass. Build campaign probes only after the core
boundary is executable.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary recursive_learner`
  establishes true/false closure discrimination, bounded construction, recursive
  ancestry, replay, and public-Harness-only access.
- `cargo test --manifest-path research/experiments/recursive-learner-construction/Cargo.toml`
  establishes all preregistered probe names, upward stop semantics, controls, and
  conditional downward ablations.
- `cargo run --quiet --manifest-path research/experiments/recursive-learner-construction/Cargo.toml -- --all --output-dir research/campaigns/recursive-learner-construction-v1/artifacts`
  emits one deterministic first-round artifact per arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/recursive-learner-construction-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/recursive-learner-construction-v1/convergence.toml`
  validate lineage and complete fan-in.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path research/experiments/recursive-learner-construction/Cargo.toml --lib`.
Its measured execution must remain strictly under 10 seconds; record cold bootstrap
separately.

## Controls and evidence

Held-out cases are reflected and reversed discovery order, depths one through three,
two sibling surfaces, and 4, 64, and 1024 dormant anonymous surfaces. Negative controls
are all unchanged protocols, one closure only, disconnected input,
correlated input without the action return, shuffled return, duplicate closure after
construction, unrelated sibling activity, valid delayed return, stale return, and
the frozen monolithic synthesis classification vector. Killing falsifiers are any
false construction, missing construction after repeated valid closure, re-budding of
one closure, special root-only behavior, replay inequality, lost quiescence, global
dormant-surface cost growth, or a campaign rung executed after failure. Expected
evidence is one artifact and result for every declared arm, preserved falsifications,
one convergence, a machine-checkable candidate receipt, and an independent
verification receipt.

## Risks and rollback

The central risk is mistaking observer metadata for independently useful learning.
The upward ladder therefore treats construction as only rung two and must falsify at
the first later transition that the boundary does not cause. Other risks are hidden
fixture boundary selection, accidental semantic identity, duplicate nesting,
checkpoint corruption, and changes to accepted sensorimotor behavior. Public trace
audits, matched false closures, fixed physical membership, exact replay, frozen
references, and protocol isolation detect them. Rollback removes the experimental
protocol and its private construction state without changing any accepted protocol.

## Open decisions

None.
