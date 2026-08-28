```text
mixed motor incidence
        |
        v
partition completing inputs by existing learner owner
        |
        v
evaluate private groups -> unique lawful winner -> ordinary candidate pipeline
        |                         |
        `--- no winner -----------'---> preserve rejection
```

# Factor mixed motor candidates by physical learner owner

## Outcome

An opt-in recursive protocol may resolve a mixed-owner output incidence only when
one existing owner-local input group is independently executable and uniquely wins
by the already observable consequence, drive, and participation evidence. The
selected group then follows the ordinary candidate, return, choice, and output
pipeline. No group shares memory, and at most one physical effect is produced.

## Authority

- Path: `research/programs/learner/forecasts/mixed-owner-motor-candidate-v1.md`
- Revision: `hand-autonomous-reuse-localization-v1/convergence@1ae83ef6d8b98e5045cf5e634b73c2c3a69065411c83b36c85081a70f41a8c9c`

## Model

An output incidence is a finite collection of physical firings. Completing-path
firings map partially to stable learner owners through their carried physical
origins. Exact ownership is unchanged. A genuinely mixed incidence maps to owner-
local groups only when every completing firing has an owner and at least two owners
are present. Each group is evaluated with only its own path firings and same-owner
current opportunity. The partial transformation returns one unique executable group
or no group. A selected group replaces the mixed input collection before the existing
candidate pipeline; rejection leaves the current ambiguous behavior intact. Trace is
an observation effect and persistent state mutation remains in the existing pipeline.

## Invariants

- Exact-owned and organism candidates are identity cases.
- Missing or unowned path ownership remains ambiguous and rejected.
- No owner group may use another owner's opportunity or consequence memory.
- A subthreshold group remains subthreshold; groups are never summed across owners.
- A tie with no unique physical winner remains rejected rather than using owner ID,
  insertion order, learner depth, or benchmark knowledge.
- At most one owner group enters the ordinary pipeline and at most one output effect occurs.
- Input order and owner construction order do not change the result.
- Existing protocols, checkpoint compatibility, work semantics, replay, quiescence,
  boundary novelty, locality, and temporal controls remain unchanged.

## Scope

- Add one opt-in protocol and protocol predicate in core routing.
- Add a pure owner-group evaluation and unique-winner transformation in `choose.rs`.
- Add one diagnostic event for mixed-owner resolution and extend candidate diagnostics
  with distinct path-origin and owner counts.
- Add adversarial core fixtures and a research experiment/campaign that compare
  owner factorization with causal-origin and common-boundary prerequisites, then run
  the unchanged hand conditionally.
- Exclude default adoption, path lifetime changes, owner-memory changes, ancestor
  fallback, semantic controllers, and hand-world changes.

## Development style

TDD: first encode exact-owned identity, two-owner unique winner, tie rejection,
subthreshold isolation, unrelated/unowned rejection, input-order invariance, and one-
effect behavior; then implement the smallest transformation satisfying them.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml --test harness_boundary owner_local_factorization -- --nocapture`
  establishes the transformation laws and inherited negative controls.
- `cargo test --manifest-path research/experiments/hand-owner-local-candidate-factorization/Cargo.toml`
  establishes unrelated transfer, competing-hypothesis discrimination, and the
  conditional unchanged-hand result.
- `cargo test --manifest-path research/experiments/hand-autonomous-reuse-localization/Cargo.toml`
  preserves the frozen diagnostic interpretation under the unchanged protocols.

## Development loop

`cargo test --manifest-path research/experiments/hand-owner-local-candidate-factorization/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: reversed owner construction and input order; one physical origin
  containing multiple owners; siblings with no consequential common owner.
- Negative controls: exact-owned identity, organism identity, unowned mixture,
  independently subthreshold groups, exact tie, unrelated sibling opportunity,
  distance-three locality, exact boundary repetition, replay, and quiescence.
- Falsifiers: no unrelated-world effect; more than one effect; cross-owner read;
  ordering dependence; hand remains at three changed steps; or any inherited regression.
- Expected artifacts: per-arm immutable evidence, convergence with failed alternatives,
  candidate receipt, and independent verification receipt.

## Risks and rollback

Factoring inputs can accidentally manufacture strength, encode an arbitrary tie break,
or multiply outputs. Detect these with summed-strength, tie, permutation, and one-effect
tests plus the unchanged hand trace. Roll back the new protocol predicate, transformation,
trace event, and experiment; existing protocol behavior and checkpoints remain usable.

## Open decisions

None.
