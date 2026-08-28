```text
mixed output incidence
        |
        v
partition by carried physical origin
        |
        v
resolve each origin: organism | exact learner | ambiguous
        |
        v
independent executability -> unique physical winner -> ordinary pipeline
```

# Factor mixed candidates by causal origin before ownership

## Outcome

An opt-in recursive protocol separates simultaneous completing-path firings by
their carried physical origin before ownership is resolved. Only independently
executable organism-scope or exact-learner factors are eligible. One uniquely
ranked factor may enter the unchanged candidate pipeline; ambiguity, ties, and
cross-factor strength remain rejected.

## Authority

- Path: `research/campaigns/hand-owner-local-candidate-factorization-v1/convergence.toml`
- Revision: `5526db51da57410a695f8756ff1b616130aa80f4c08bba7b397017f3795b03f2`

## Model

The finite input collection maps to a finite map keyed by carried physical origin.
Each origin factor maps to one ownership state and one candidate evaluation using
only its own path drive, current opportunity, and private consequence evidence.
Ambiguous factors and non-executable factors map to no candidate. A unique maximum
maps back to one filtered incidence; a tie maps to no change and is rejected by the
existing ambiguous pipeline. The transformation is pure apart from replacing the
incidence input list and emitting diagnostics; all learning and output effects stay
in the ordinary pipeline.

## Invariants

- A factor never borrows strength, opportunity, ownership, or memory from another origin.
- Organism and exact-owned single-origin incidences are identity cases.
- Ambiguous ownership inside one origin remains rejected.
- At most one origin factor enters the ordinary pipeline and produces one effect.
- A tie remains rejected without origin ID, insertion order, learner depth, or benchmark knowledge.
- Reversing input order preserves behavior; exact replay preserves canonical state.
- Strict owner-only factorization and every older protocol remain unchanged.
- Boundary, timing, locality, private memory, checkpoint, quiescence, and cost controls remain intact.
- Bounded send with a sufficient budget equals ordinary send; budget exhaustion
  reports non-quiescence and leaves queued physical activity observable.

## Scope

- Add one opt-in protocol predicate and routing case.
- Add a pure origin-group evaluator and unique-winner filter in `choose.rs`.
- Add one observation-only origin-resolution diagnostic event.
- Add a diagnostic-only bounded-send API that returns after a caller-supplied
  physical-moment limit; ordinary `send` remains unchanged.
- Add focused core controls and a research experiment/campaign with unrelated
  transfer, same-origin ambiguity, unchanged hand, inherited controls, and cost.
- Exclude default adoption, semantic origin classes, origin-ID ranking, strength
  summation across factors, ancestor fallback, and hand-world changes.

## Development style

TDD: encode two-origin owned/organism separation, independently subthreshold
rejection, tie rejection, one-effect behavior, input-order invariance, exact replay,
and unchanged older protocols before implementing the filter.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml --test harness_boundary causal_origin_ownership_factorization -- --nocapture`
  establishes factor isolation, unique winner, tie/subthreshold rejection, identity
  laws, bounded/ordinary equality, and explicit non-quiescent budget exhaustion.
- `cargo test --manifest-path research/experiments/hand-causal-origin-ownership-factorization/Cargo.toml`
  establishes unrelated transfer and the conditional unchanged-hand result.
- `cargo test --manifest-path research/experiments/hand-owner-local-candidate-factorization/Cargo.toml`
  preserves the frozen owner-only negative result.

## Development loop

`cargo test --manifest-path research/experiments/hand-causal-origin-ownership-factorization/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: reflected input order, relabeled origins, one origin without a
  learner owner, exact-owned-only incidence, and organism-only incidence.
- Negative controls: no independently executable factor, exact tie, ambiguous
  ownership inside a factor, unrelated surface, duplicate effect, distance-three
  locality, boundary repetition, sufficient bounded-send equality, replay, and quiescence.
- Falsifiers: no unrelated effect; multiple effects; cross-origin memory or drive;
  arbitrary tie break; no hand improvement; or any inherited regression.
- Expected artifacts: immutable per-arm evidence, convergence, candidate receipt,
  and independent verification receipt.

## Risks and rollback

Origin factorization can mistake physical identity for rank or discard legitimate
cooperation. Detect that with relabeling, tie, subthreshold, and one-effect controls.
Rollback removes the opt-in protocol, filter, event, and experiment; prior protocols
and checkpoint behavior remain unchanged.

## Open decisions

None.
