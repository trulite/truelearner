# Independent Review Guide

## Review Objective

Determine which narrow capabilities are supported by the experiments and
which apparent capabilities result from supplied representations, algorithms,
thresholds, curricula, or evaluator knowledge.

The project name is historical. This repository does not claim biological
life, consciousness, general intelligence, or unrestricted real-world
learning.

## Recommended Review Tracks

### Engineering

Check:

- Rust correctness and panic behavior
- determinism and seed handling
- invalid-state handling
- algorithmic complexity
- test independence
- differences between debug and release execution
- whether assertions accidentally participate in learning

### Experimental Method

Check:

- train/test leakage
- evaluator information unavailable to the learner
- whether negative controls match positive cases fairly
- whether baselines receive equivalent information and compute
- whether thresholds were selected after seeing test results
- whether a simpler memorizer or lookup table explains the result
- whether each conclusion is narrower than its supplied priors
- whether v16 pattern cells add capability beyond a variable-order lookup
  table

## Highest-Risk Assumptions

The current v14 experiments still supply:

- relational sensor ports
- the candidate effect family: `STAY` or `FOLLOW_PORT`
- opaque action boundaries
- binary or sparse observation codecs
- compression and support objectives
- task pools and evaluation criteria
- several earlier object-grouping, tracking, planning, and search algorithms

These assumptions should be treated as part of the model, not as learned
results.

V16 removes those later modules from its learning path, but still supplies:

- ordered token boundaries
- separate joining and prediction phases
- pattern-cell recruitment
- deepest-pattern selection
- recent-activity reset and an activity limit

V17 additionally supplies:

- a two-activation retention threshold
- the timing of the rest phase
- the graph-rebuild operation
- the replay cases used to accept or reject a rewrite

Reviewers should count replay storage and test whether rare but important
one-shot associations are deleted before they are queried.

The equal-rest comparison currently favors the trie: both retain the same
contexts and behavior, while the trie uses fewer links, less estimated
container storage, and less query work. Do not interpret v17 as an
architecture-specific compression result.

V18 leaves the unified learner unchanged. Its fixed stream markers and the
separate hard-coded walker are supplied by the evaluator. The walker validates
the dataset only. V18 must not be credited with graph traversal because the
actual learner scores zero on all held-out episodes.

V19 introduces a new substrate prior:

- opaque identity equality and hashing
- exposed slot positions
- episode-local cells and arrows
- automatic temporary erasure
- three output cardinalities
- four possible role-routing arrows

The parser does not construct an answer route or perform query matching.
Terminal supervision contains only the complete correct outcome. The result
should be interpreted as learned selection among supplied role routes, not as
general variable binding or composition.

## Code Map

- `src/main.rs`: original v1-v2 runtime and executable report
- `src/inertia.rs`: v3 higher-order motion
- `src/tracking.rs`: v4 persistent identity
- `src/vision.rs`: v5 raw-frame visual templates
- `src/causal.rs`: v6-v8 causal learning, planning, and procedures
- `src/generality.rs`: v9-v14 representation through transfer experiments
- `src/scaling.rs`: v14.5 deterministic scaling and capacity probes
- `src/stability.rs`: v14.6 learned compression and self-stabilization
- `src/unified.rs`: v16 single cell-arrow-spike sequence learner
- `src/consolidation.rs`: v17 trie baseline and offline graph consolidation
- `src/composition.rs`: v18 renaming-invariant composition probe
- `src/binding.rs`: v19 temporary identity binding
- `src/bin/scaling.rs`: CSV-producing scaling runner
- `src/lib.rs`: public library and reviewer API
- `tests/reviewer_api.rs`: example independent black-box evaluation

## Standard Verification

```bash
cargo fmt -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Secret Evaluation

Reviewers should add a new integration test under `tests/` and avoid modifying
`src/`.

Use the `organism_v0::review` API to:

1. Construct a reviewer-controlled relational topology.
2. Register opaque action IDs.
3. Provide training transitions.
4. Keep topology seeds, action mappings, and held-out frames private.
5. Measure prediction, ambiguity, and rejection behavior.

The API deliberately exposes the supplied hypothesis family. This lets a
reviewer test the current claim honestly; it does not pretend that relational
ports or effect candidates were learned.

Suggested adversarial cases:

- asymmetric and non-grid graphs
- ports that merge two sensors into one
- disconnected components
- contradictory and noisy transitions
- actions outside the supported effect family
- topology changes after learning
- action aliases and action remapping
- held-out multi-sensor frames
- boundary cases where several hypotheses predict `STAY`

## Review Output

For every finding, report:

1. Severity: invalidates claim, narrows claim, or engineering issue.
2. Exact experiment and code location.
3. Reproduction with seed or custom topology.
4. Expected behavior.
5. Actual behavior.
6. Consequence for the stated conclusion.

The most valuable contribution is a reviewer-controlled test that fails for a
principled reason.
