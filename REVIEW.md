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

V20 freezes the selected v19 lookup and additionally supplies:

- one identical apply event with host-controlled timing
- temporary current and result roles
- a read event
- three candidate feedback routes

The host does not pass the previous result back as a new query and no apply
number is exposed. Terminal supervision selects the feedback route from the
complete two-pulse outcome. The three- and four-pulse results show that the
selected route can be reused, but do not show that the machine learned when to
continue or stop.

V21a moves apply, temporary comparison, result, feedback, current update, and
self-trigger execution into one queued spike runtime. It additionally
supplies:

- one external start event
- a safety cutoff observed by the evaluator
- the frozen v20 operation
- three candidate self-routes
- permanent apply, start, read, and quiet control cells

The evaluator drains a generic queue and never calls apply or lookup. It does
observe the number of completed lookups so it can kill execution at the test
cutoff. The result supports autonomous continuation through selected
structure; it does not support learned finishing, cycle detection, or
unbounded safe execution.

V21b freezes the v21a successful path and additionally supplies:

- a neutral no-result event after a completed lookup finds zero successors
- permanent answer and clear cells
- four candidate finish routes

Only finish-route confidence changes during v21b training. Held-out scoring
accepts an explicit answer event and never reads the temporary current cell.
The evaluator supplies no cutoff. Cycles still reach a safety limit and emit no
answer.

The 32-of-32 recomposed v18-distribution result must not replace the original
v18 failure. The later substrate receives identity equality, structured
relation slots, temporary lifetime, and candidate route families that the v18
unified sequence learner did not receive.

D0 begins a separate discovery ladder. It removes v19's fixed list of four
slot routes, but still supplies:

- opaque equality and permanent sensory-role cells,
- episode boundaries and automatic temporary erasure,
- a fixed temporal coactivity window,
- bidirectional proposal between nearby active permanent cells,
- one-arrow competition and exploration,
- recently-used arrow traces,
- scalar terminal success or failure,
- a fixed consolidation threshold and pruning operation.

Only the arrow used to produce the attempted answer receives feedback. The
evaluator does not score every candidate route or identify a responsible
arrow.

The irrelevant-cue control is deliberately unfavorable: 15 of 32 learners
select the shortcut and fail when it is removed. D0 therefore demonstrates
topology selection under reward, not causal route discovery.

D1 changes only the curriculum. The proposal, trace, scalar feedback,
competition, and consolidation behavior remain unchanged from d0.

The paired observation and contrasting streams share identities, relation
order, answer changes, learner seeds, and episode budgets. Cue location is the
only difference. The contrasting stream distributes the cue equally across
all ten relation positions plus absence and balances changed against unchanged
answers.

The observation-only side of this paired comparison fails after cue removal
in 10 of 32 runs; the contrasting side fails in zero. The separate historical
d0 control remains 15 of 32 because it uses a different curriculum.

D1 should be interpreted as information supplied by an intervention
curriculum, not intervention understanding or active causal learning.

D2 adds a separate action-learning loop. It supplies:

- an unresolved-topology context,
- three opaque action identities plus no action,
- a fixed real-action cost,
- an action trace that snapshots plausible route identities and strengths,
- temporary protection from pruning while the trace gathers evidence,
- informative, disruptive, and uninformative classifications derived from
  route-strength changes,
- unique-winner consolidation after an action window,
- environment-defined action effects, randomly permuted between runs.

The evaluator does not identify an informative action or route. Action credit
comes from the learner's own before-and-after route strengths.

Random actions solve all 32 runs and are slightly cheaper than the learned
policy in this three-action environment. D2 therefore supports learned
context-dependent action preference and stopping, not sample-efficiency or
counterfactual experiment planning.

D2.1 supplies fresh problem boundaries. Every ambiguity constructs and
destroys a new d0 topology workspace while one variable-length action policy
persists. The policy type contains action values, tried flags, and an
exploration cursor; it contains no problem identities or route references.

Action meanings remain fixed across one hundred problems in a run. This makes
amortization possible and must not be interpreted as action-remapping
adaptation.

The learned mature policy uses one action from four through sixty-four
choices. Random search grows from 2.5 to 36 actions. All 12,000 workspaces are
destroyed before the next problem begins.

D2.2 changes only the action mapping after policy maturity. It does not add
forgetting, value decay, or exploration reopening. Mature and fresh policies
receive identical remapped effects and problem seeds.

When the new informative action was never tried, mature policies eventually
adapt but become slower and more expensive as old evidence accumulates. When
the new informative action had previously received negative evidence, no
mature policy adapts within five hundred problems, while every fresh policy
does.

D2.2 therefore demonstrates rigidity, not successful continual adaptation.
The usefulness of selectively separating routes remains supplied. Only the
opaque action-to-consequence mapping changes.

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
- `src/iteration.rs`: v20 repeated use of one learned lookup and feedback route
- `src/continuation.rs`: v21a autonomous continuation and v21b learned finish
- `src/discovery.rs`: d0 generic topology proposal, trace, reward, and pruning
  plus d1 contrasting experience, d2 learned epistemic action, and d2.1
  amortization and d2.2 silent-remap diagnostics
- `src/bin/remap.rs`: CSV writer for complete d2.2 value trajectories
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
