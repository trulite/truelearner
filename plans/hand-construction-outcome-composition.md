```text
same-tick outcome -> physical link mark -> learner construction
                          |                         |
                          +---- exact lineage -----+
                                      |
                                      v
                         existing learner memory
                         (original tick preserved)
```

# Compose an outcome through learner construction

## Outcome

Add one opt-in successor to `RecursiveLearnerCompletedCycle` that preserves an
already-recorded same-tick consequence when its exact live link becomes part of
a newly constructed learner. Use the existing learner consequence memory and
preserve the original physical tick. The exact hand prediction is only that
learner two's target-eleven completed-cycle state changes from `Missing` to
`Stale` at tick twenty-three; target choice and hand movement need not improve.

## Authority

- Path: `research/campaigns/hand-compositional-existing-trace-witness-v1/convergence.toml`
- Revision: `sha256:4a6c44b1280ab30541fbd93aa0e9ca81f414e5f5ca761f6fe1425eb9b8cfb164`

## Model

The existing physical states are sufficient. A returning outcome writes
`last_consequence_tick` to a live link. Learner construction changes the view
of the exact construction lineage. The new transformation projects a
same-tick `(link, generation, last_consequence_tick)` fact into the new
learner's existing consequence memory after that learner is formed.

The transformation is defined only when the link belongs to the exact sorted
construction lineage, remains live at the same generation, and carries a
consequence tick equal to the construction tick. It calls the ordinary learner
consequence-recording boundary and emits the existing truthful
`LearnerConsequenceRecorded` event. It does not invent a path, outcome, held
state, or timestamp.

Composition must preserve identity: a same-generation link keeps its physical
tick across the view change. It must preserve selectivity: older history,
unrelated links, sibling or parent-private memory, dead links, and replacement
generations do not enter the child. Failure to meet every condition is an
ordinary no-op.

## Invariants

- The candidate is opt-in; all existing protocols and the default remain byte-
  and behavior-compatible.
- Only links in the exact construction lineage can be projected.
- Only a live current generation with `last_consequence_tick == construction
  tick` can be projected.
- The original tick is preserved, never refreshed to a later read or choice.
- No parent, sibling, global older history, semantic output identity, position,
  direction, hand step, or expected action is consulted.
- Existing path, link, outcome, learner consequence memory, and trace types are
  reused; no new durable state or trace event is added.
- Replay, checkpoint validation, natural quiescence, zero propagation
  exhaustion, and the strictly bounded recent window remain unchanged.

## Scope

- Add one `Protocol` variant and include it in the inherited recursive learner
  predicates and bindings.
- Add the smallest construction-boundary composition in
  `truelearner/crates/core/src/learner.rs`, reusing the existing consequence
  recording function.
- Add focused core controls and one successor hand experiment/campaign that
  compares the candidate with the unchanged completed-cycle parent.
- Record candidate, verification, convergence, program-frontier, and lesson
  evidence only after execution.
- Exclude new state types, parent-memory copying, longer recency, held paths,
  outcome renewal, selection changes, adapter changes, default adoption, and
  authority promotion.

## Development style

TDD. First add a focused construction fixture that expects same-tick projection
under only the successor protocol and rejects older and unrelated physical
history. Then implement the protocol and construction transformation. Compile
the successor hand runner before its single frozen discovery execution.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary construction_outcome_composition`
  proves exact-lineage same-tick projection, original-tick preservation,
  protocol isolation, and replay/quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-construction-outcome-composition/Cargo.toml --no-run`
  proves the frozen evidence runner compiles without consuming its valid run.
- `cargo test --locked --manifest-path research/experiments/hand-construction-outcome-composition/Cargo.toml`
  proves the artifact classifier distinguishes `Missing` from `Stale` and
  enforces unchanged choice and hand controls.
- `cargo test --locked --manifest-path truelearner/Cargo.toml`
  proves inherited core behavior remains intact.

## Development loop

The representative warm regression is
`cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary construction_outcome_composition`.
Its measured duration must remain strictly under 10 seconds; cold bootstrap is
recorded separately.

## Controls and evidence

Held-out cases include an older consequence on a construction link, a
same-tick consequence on a link outside the construction lineage, a dead or
generation-replaced link, the unchanged completed-cycle parent, and checkpoint
replay. Negative controls require old protocols to produce no construction-time
private write and the candidate to leave output choice, hand summary, recency
window, adapter, and semantic isolation unchanged.

The candidate hand survives exactly when tick sixteen records the already
physical consequence into learner two at tick sixteen and tick twenty-three
reports target eleven with `consequence_tick = 16` and completed-cycle state
`Stale`, while the admitted target remains ten by the inherited fresh-
alternative rule. It is falsified if the state stays `Missing`, becomes
`Unique`, refreshes beyond tick sixteen, imports unrelated history, changes the
hand choice merely to satisfy the benchmark, or breaks integrity controls.

## Risks and rollback

The main risk is silently turning fresh learner memory into copied history.
Exact lineage plus same-tick equality makes that failure observable and keeps
the transformation local. A second risk is accidentally treating the imported
fact as fresh at tick twenty-three; the frozen `Stale` prediction kills that
error. Rollback removes the successor protocol, the construction call, and its
successor evidence without changing any existing checkpoint shape or frozen
artifact.

## Open decisions

None.
