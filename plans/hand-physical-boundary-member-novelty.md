# Physical boundary member novelty

```text
proposed causal boundary B
          |
          +-- no parent ----------> admit root evidence
          |
          `-- parent owns P ------> admit only when B ∖ P is non-empty
```

## Outcome

Compose the surviving consequence-born closure protocol with one local construction
gate: a proposed child boundary earns closure evidence only when its participating
physical junction set contains at least one junction not already owned by the proposed
parent. Exact repetition of one owned surface/output boundary must not create deeper
learners; distinct connected sibling surfaces inside the inherited local return radius
and legitimate adjacent expansion must remain constructible. Retry the unchanged
reflected hand only after this gate survives.
No default or authority change.

## Authority

- Path: `research/constitution.md`, lessons `LP-039` and `LP-040`, and
  `research/campaigns/hand-consequence-born-closure-eligibility-v1/convergence.toml`
- Revision: convergence
  `b8e0518d323c576923bdb2c677f0db1dff2c3452fb0a51beae9451f5c3fae5c6`

## Model

The proposed boundary is the sorted unique set of stable physical junctions already
derived from the closure surface, output, and live participating lineage-link endpoints.
The proposed parent is the existing deepest learner owning the surface. The novelty
transformation is the pure set difference `B ∖ P.junctions`. A root proposal has no
parent and remains eligible; a child proposal is eligible exactly when the difference is
non-empty. Link identities are excluded because temporary links are regenerated and do
not represent new physical boundary membership.

Only an eligible proposal may create or increment `CausalClosureState`. Rejected
proposals still retain temporal eligibility, return admission, reverse consolidation,
and ordinary physical effects. A diagnostic event records parent, proposed member
count, novel member count, and eligibility. The new experimental protocol composes
this gate with consequence timing, cohort closure, and eligible-return priority.

## Invariants

- The proposal and parent sets are sorted, unique junction identities; set difference is
  deterministic, order-independent, and checkpoint-replayable.
- Root construction still requires two exact causal observations.
- A proposed child wholly contained in its parent contributes zero closure evidence and
  creates no learner, regardless of repeated temporary link generations.
- A proposal with at least one new participating physical junction retains the ordinary
  two-observation construction threshold and fresh child memory.
- Distinct unowned connected surfaces within the inherited local radius can construct
  independent root siblings in either order; a distance-three surface remains rejected
  by unchanged return locality; adjacent owned-surface expansion can construct a child
  when its boundary adds a new junction.
- Timing, cohort retirement, eligible-first ordering, causal lineage, impulse, reverse
  consolidation, one-effect conservation, owner memory, old protocols, replay, and
  quiescence remain unchanged.
- The unchanged hand is not run unless every boundary primitive and control survives.
- The representative warm regression suite remains strictly under 10 seconds.

## Scope

- Add an isolated `RecursiveLearnerBoundaryNovelty` protocol predicate and routing in
  `truelearner/crates/core/src/core.rs` and `physics.rs`.
- Add the local set-difference gate in `learner.rs`, one diagnostic event in `trace.rs`,
  and focused public-Harness tests in `harness_boundary.rs`.
- Add a frozen campaign and experiment under
  `research/experiments/hand-physical-boundary-member-novelty/` and
  `research/campaigns/hand-physical-boundary-member-novelty-v1/`.
- Add a neutral protocol-parameter adapter to the existing unchanged reflected-joint
  world in `research/experiments/developmental-hand-construction-admission/`; its current
  default entrypoints and frozen-reference behavior remain byte-for-byte equivalent.
- Add candidate and verification receipts.
- Exclude link-ID novelty, semantic anatomy, depth limits, evaluator stages, changed
  thresholds, boundary mutation, hand-world changes, default adoption, and authority
  promotion.

## Development style

TDD. First freeze exact-boundary repetition, sibling order, adjacent expansion, temporal
composition, replay, and quiescence; then add the isolated gate.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core boundary_novelty_`
  checks repetition rejection, sibling construction in both orders, adjacent expansion,
  diagnostics, and replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core consequence_born_`
  preserves the complete temporal composition and both frozen partial failures.
- `cargo test --locked --manifest-path research/experiments/hand-physical-boundary-member-novelty/Cargo.toml --lib -- --test-threads=1`
  checks every declared research arm and the conditional hand gate.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml --lib`
  proves the shared hand adapter did not alter its existing default evidence contract.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`,
  Rustfmt, Cargo check, Clippy with `-D warnings`, and the research validators preserve
  the full software and evidence contracts.

## Development loop

The representative warm regression command is
`cargo test --locked --manifest-path research/experiments/hand-physical-boundary-member-novelty/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap.

## Controls and evidence

Held-out cases are twelve exact-boundary repetitions, a second local connected surface in
both orders, two simultaneous surfaces with one effect, an adjacent expansion adding only
one new endpoint, and a proposal whose only apparent novelty is regenerated links. Negative controls
are the inherited distance-three return-locality rejection, immutable temporal evidence,
pre-output/equality/duplicate/disconnected/stale
cases, an owned subset proposal, old protocols, replay, quiescence, and evaluator
isolation. Killing falsifiers are any repeated-boundary child, loss of a distinct sibling
or adjacent expansion, link-generation novelty, temporal regression, multiplied effect,
order dependence, replay inequality, non-quiescence, or a hand run after a failed gate.
Expected evidence is seven arm artifacts, convergence, and validated factory receipts;
the hand artifact is emitted only if the boundary composition survives.

## Risks and rollback

Computing membership after closure state mutation could preserve false evidence, so the
gate runs before state lookup or increment. Treating transient links as members would
recreate false novelty, so only junction endpoints enter the set. Treating every unowned
surface as a child could distort sibling ownership, so existing parent selection remains
unchanged. Rollback removes only the new protocol predicate, gate, diagnostic, tests,
and isolated research artifacts while preserving the temporal survivor and all failures.

## Open decisions

None.
