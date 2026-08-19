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

D2.3 introduces a separate current-regime action record and an append-only
historical outcome record. Three consecutive violations of a trusted action
reopen the local action mapping. Isolated failures do not.

Across 16 and 64 choices and maturity levels 10, 50, and 100, reopening always
occurs after three violations and adaptation always completes in eight
problems. Previously rejected actions become eligible again.

A full reset adapts in six problems and is therefore initially cheaper, but it
loses all earlier evidence. After both regimes have been experienced, repeated
switches cost fewer actions because historical evidence prioritizes previously
useful alternatives.

The three-violation threshold, the separation into historical and
current-regime state, and the usefulness of distinguishing evidence are
supplied. D2.3 does not discover its own plasticity timescale.

D3a trains action-effect models without an ambiguity task. Generic
same/different observations propose action-to-role changed and preserved
arrows. Action identities and temporary occupants are opaque, but role
positions are supplied.

D3b freezes those models. It compares competing route activity using generic
set and connection intersection and difference. The comparison does not emit a
named distinguishing role. A supplied epistemic preference scores predicted
action consequences by whether they change route-specific evidence while
preserving common and competing evidence.

The model selects all 48 first actions correctly under sixteen opaque action
permutations. Empty-history, random, and disruptive baselines score 11, 18,
and zero. All action predictions and structural comparisons are recorded
before the selected action executes, and the model fingerprint remains
unchanged.

D3 demonstrates model-based selection within a supplied role and comparison
ontology. It does not learn the structural comparison or the epistemic
preference itself.

D4a replaces changed/preserved action masks with learned occupant provenance.
For each output role, it learns which input role supplies the occupant. Frozen
individual-action models are then repeatedly applied to action sequences that
never appeared during training.

The provenance model predicts all 848 held-out sequences exactly through
length sixteen. A changed-role mask baseline predicts only 32. Opposite
rotations that change identical role sets remain indistinguishable to the mask
but are distinguished by learned source-role structure.

D4a supplies role positions, identity equality, the action sequence, and
generic repeated model application. It demonstrates composition of learned
world models, not planning or learned search.

D4b trains the same provenance learner on individual actions, freezes the
resulting 507-entry model, and supplies a shortest-first bounded sequence
enumerator. Candidate futures use only learned transformations. The true
environment executes only the selected sequence.

Across required depths one, two, three, four, and eight, learned-model
prediction and real execution both solve all 40 problems. The true-model
oracle also solves all 40, equal-budget random ordering solves 30, the
one-step selector solves eight, and the changed-role-mask planner solves none.
All eight unreachable cases are reported.

Average candidate evaluations grow from 1.9 to 6,969.8, making the cost of
supplied exhaustive enumeration explicit. D4b demonstrates planning with
learned compositional models, not learned or efficient search.

S0 learns a coarse value over canonical temporary-state equality, route
structure, and remaining budget. The learner never receives the successful
action sequence. It reaches 92 permanent entries and remains invariant to
opaque identity renaming.

S1 uses frozen value only to order a supplied lazy complete search. At depth
eight it matches oracle ordering and cuts average partial expansion from
9,839.8 to 4,924.8. Shortest-plan accuracy and real execution remain exact.
Shuffled values and exact-path memory provide no transferred advantage.

Full accounting is unfavorable. Reachable total work rises from 1,255,048 to
1,357,960 because value evaluation costs more than the saved model
applications. Unreachable work rises from 1,848,672 to 3,973,536 because
completeness requires identical expansion plus heuristic overhead.

S0 demonstrates reusable search-value knowledge. S1 demonstrates useful
ordering, but not an economic search advantage.

S1.1 freezes those 92 values and compiles the expensive recognition boundary.
An incremental equality signature is updated alongside each learned model
application. The same signature either directly indexes value or activates a
signature cell whose fixed arrow reaches a high, medium, or low value cell.

The original, compiled, and local evaluators agree on all 46,944 audited
states and produce identical search ordering. Convergent action paths are
checked for path-independent signatures, and collisions between differently
valued structural states are rejected.

On the frozen reachable workload, direct compiled work is 1,196,296 and local
activation work is 1,236,712, both below neutral exhaustive work of 1,255,048.
The one-time compilation costs repay after two and five reachable problems.
Unreachable work remains higher because complete search still exhausts the
level.

These are deterministic work counts. S1.1 does not establish hardware speed,
learn signature compilation, or learn when guidance is worth invoking.

S1.2 freezes S0 and S1.1, then adds a six-entry structural bandit gate between
neutral and compiled-guided search. Training reveals only the selected mode's
correctness and paid work. The unchosen mode, required plan depth,
reachability, and oracle choice are hidden from the learner.

Training and held-out data counterbalance all six opaque action orderings.
Exploration is fully charged. The gate learns neutral search for reachable
depths one through four and unreachable search, and guided search for depth
eight.

Held-out work is 22,817,640 for the learned gate, versus 23,278,200 for always
neutral, 35,232,600 for always guided, and 30,056,073 for random gating.
Cumulative training work repays exploration at problem 1,439 and remains
ahead through problem 2,880.

The per-problem oracle sees hidden action-order economics that the gate input
does not. Therefore the claim concerns expected cost for observable
structural contexts, not perfect individual oracle classification.

S1.2 supplies the bandit update, exploration rule, conservative near-tie
policy, two reasoning modes, signature, and search algorithms. It does not
learn how much thinking to buy.

P0 begins the de-supply ladder. It removes the four route-candidate lists from
v19-v21 and replaces them with one generic all-pairs proposal field. Recently
used arrows receive only terminal correctness or failure. The same probation,
strengthening, weakening, consolidation, pruning, and activity settings are
used for every isolated gate and for fresh integrated P0e seeds.

P0e receives the complete traversal task from the first episode. It does not
inherit isolated routes or receive a staged lookup, feedback, continuation, or
finish curriculum. Real correctness produces eight functional four-arrow
programs in eight seeds. Shuffled and random feedback produce none.
Activity-only learning accidentally produces one functional program in eight
seeds, which is retained as a chance result.

P0 still supplies role cells, identity equality, relation slots, temporary
lifetime, neutral no-result events, proposal physics, trace duration, terminal
correctness, and pruning. The target-cell semantics also remain supplied. The
claim is program-topology discovery inside this ontology, not program
discovery from raw sensation.

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
  amortization, d2.2 silent-remap diagnostics, and d2.3 local reopening
- `src/model_epistemic.rs`: d3a action-effect learning and d3b pre-action
  structural experiment selection
- `src/composable_models.rs`: d4a source-role learning and frozen model
  composition plus d4b bounded counterfactual search
- `src/search_value.rs`: S0 goal-conditioned value learning and S1 lazy
  supplied search ordering plus S1.1 compiled recognition and local activation
- `src/program_discovery.rs`: P0a-P0d isolated route discovery and fresh P0e
  end-to-end recurrent program discovery
- `src/role_discovery.rs`: P1 anonymous sensory-position role recruitment,
  lookup through learned roles, and fresh role-plus-program integration
- `src/local_plasticity.rs`: P2 local structural plasticity, P2.1 selective
  plasticity, and P2.2 learned encounter representations and values
- `src/internal_roles.rs`: P3 anonymous working and control-event role
  recruitment plus fresh role-and-program integration
- `src/request_roles.rs`: P4 anonymous request-event role recruitment,
  request-only execution boundary, and fresh request-plus-program integration
- `src/bin/remap.rs`: CSV writer for complete d2.2 value trajectories
- `src/bin/plasticity.rs`: CSV writer for d2.3 regime trajectories
- `src/bin/model_epistemic.rs`: CSV writer for d3 pre-action traces
- `src/bin/composable_models.rs`: CSV writer for d4a composition traces
- `src/bin/counterfactual_planning.rs`: CSV writer for d4b candidate traces
- `src/bin/search_value.rs`: CSV writer for S0/S1 work curves
- `src/bin/compiled_search_value.rs`: CSV writer for S1.1 compilation economics
- `src/bin/guidance_gate.rs`: CSV writer for S1.2 metareasoning economics
- `src/bin/program_discovery.rs`: CSV writer for P0 program-discovery
  trajectories and controls
- `src/bin/role_discovery.rs`: CSV writer for P1 sensory-role discovery
- `src/bin/local_plasticity.rs`: CSV writer for P2 local-plasticity scaling
- `src/bin/selective_plasticity.rs`: CSV writer for P2.1 gate economics
- `src/bin/encounter_representations.rs`: CSV writer for P2.2 learned
  encounter representations
- `src/bin/internal_roles.rs`: CSV writer for P3 working/control roles and
  integrated program discovery
- `src/bin/request_roles.rs`: CSV writer for P4 request-role discovery and
  fresh integration
- `src/bin/scaling.rs`: CSV-producing scaling runner
- `src/lib.rs`: public library and reviewer API
- `tests/reviewer_api.rs`: example independent black-box evaluation

## Standard Verification

```bash
cargo fmt -- --check
cargo test --lib --bin organism-v0 --test reviewer_api
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
