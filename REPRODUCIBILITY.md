# Reproducibility

**Recorded:** 2026-08-19

**Rust:** 1.97.1

**External crate dependencies:** none

## Commands

```bash
rustc --version
cargo --version
cargo fmt -- --check
cargo test --lib --bin organism-v0 --test reviewer_api
cargo clippy --all-targets -- -D warnings
cargo run --release
cargo run --release --bin scaling -- --output results/v14_5_scaling.csv
cargo run --release --bin remap -- results/d2_2_remap.csv
cargo run --release --bin plasticity -- results/d2_3_plasticity.csv
cargo run --release --bin model_epistemic -- results/d3_pre_action_traces.csv
cargo run --release --bin composable_models -- results/d4a_composition_traces.csv
cargo run --release --bin counterfactual_planning -- results/d4b_planning_traces.csv
cargo run --release --bin search_value -- results/s0_s1_search_value.csv
```

`rust-toolchain.toml` pins the compiler and installs `rustfmt` and Clippy.

## Determinism

Experiments use fixed integer seeds and an internal deterministic linear
congruential generator. They do not use wall-clock time, operating-system
randomness, network services, or external datasets.

Determinism makes regressions reproducible but does not provide statistical
confidence. Reviewers should replace or supplement fixed seeds through
independent integration tests.

## Expected Test Inventory

- 113 in-crate unit tests
- 4 public reviewer-API integration tests

## Current Strongest Results

- v8 hierarchical planning: 69 primitive expansions versus 2 hierarchical
  expansions in the built-in transfer case
- v9: four structural operators learned and transferred
- v10: five selected interventions identify five opaque action rules; the
  deterministic random baseline averages 13.2
- v11: the changed rule adapts in six relevant samples over a 78-sample
  continuous stream while both unchanged rules remain stable
- v12: five opaque actions compress into three causal classes and a
  three-step procedure with support 8 and compression gain 13
- v13: the unified loop selects five causal interventions and six recurring
  task traces; random trace sampling averages 26.1
- v14: three new action aliases calibrate and a learned procedure transfers
  without target-domain task demonstrations
- v14.5: deterministic work is fitted against observation count, active
  context, and topology size; event cascades are compared with subcritical
  branching-process theory; bounded associative recall is measured across a
  16x load range
- v14.6: repeated useful cascades compress into short concept routes, useless
  activity weakens, a newly introduced unstable loop is learned away, and the
  stabilization training sweep covers one through sixteen independent routes
- v16: one persistent cell-arrow-spike learner performs repeated-sequence
  induction, thirty-two-pair recall, and three-position needle retrieval while
  rejecting remapped and unknown queries
- v17: identical recurrence-guided consolidation preserves both memories'
  tested behavior and retains the same contexts, while the trie uses fewer
  links, less estimated storage, and less query work
- v18: a solvable renaming-invariant chain benchmark produces a clean negative
  result; the unchanged learner and trie both score zero on unseen symbols and
  depths while permanent memory continues growing with training examples
- v19: six permanent cells and four learned role-routing arrows answer twenty
  thousand held-out episodes containing four hundred thousand fresh opaque
  identities while temporary state is erased and permanent state remains
  fingerprint-identical
- v20: one selected feedback route feeds the frozen v19 lookup output back
  into its temporary current role; training uses only two pulses, while held
  out depths one through four all pass with fixed permanent structure
- v21a: one external start drives up to thirty-two internally generated
  lookups through a real queued spike runtime; permanent structure stays fixed
  while internal spike work grows linearly with requested depth
- v21b: a no-result event selects an explicit finish route; held-out chains
  through depth thirty-two settle naturally without a cutoff, and separate
  depth and working-set spike curves remain linear
- d0: generic temporal coactivity proposes 18 routing arrows, recently-used
  traces plus scalar feedback retain one direction-appropriate arrow, reversed
  experience learns the reverse arrow, and random labels retain no stable
  topology; an irrelevant cue causes 15 of 32 learners to choose a shortcut
- d1: an exactly paired 792-episode comparison leaves the d0 learner unchanged;
  observation-only training produces 10 shortcut failures in 32 runs, while
  counterbalanced contrasting experience produces zero and all 32 learners
  retain the true slot route
- d2: all 32 learners prefer the action whose consequences selectively weaken
  the shortcut, none prefer disruption, and action use falls to zero after
  resolution across all six opaque-action permutations; random action search
  also solves all runs and is slightly cheaper in this small action space
- d2.1: mature learned cost stays at one action across 4-to-64 choices, random
  search grows from 2.5 to 36 actions, cumulative learned cost beats random by
  the second ambiguity at latest, and all 12,000 fresh topology workspaces are
  destroyed between problems
- d2.2: unknown replacement actions are eventually found but become more
  expensive to learn as old evidence accumulates; previously rejected
  replacements are never reconsidered in 12 mature runs within 500 problems,
  while every matched fresh policy adapts; all 7,547 workspaces are destroyed
- d2.3: the hard rejected-action remap adapts in all 12 runs after exactly
  three expectation violations and eight problems regardless of prior
  maturity; isolated noise causes no reopening, repeated regime switches
  become cheaper through retained history, and all 2,162 workspaces are
  destroyed
- d3a: role-relative action-effect models predict all 6,144 held-out
  consequences across fresh identities and sixteen action permutations;
  shuffled action/outcome training produces no confident model
- d3b: frozen action models select all 48 distinguishing first interventions
  on novel route pairs, versus 11 for empty history, 18 random, and zero for
  the disruptive heuristic; complete traces exist before action and model
  fingerprints remain unchanged
- d4a: source-role models predict all 848 held-out action sequences exactly
  through length sixteen, while a changed-role mask baseline predicts 32;
  opposite rotations with identical changed-role masks remain structurally
  distinguishable, swap-twice and order-sensitive controls pass, and frozen
  model fingerprints remain unchanged
- d4b: supplied bounded search over frozen learned transformations predicts
  and executes all 40 shortest distinguishing sequences through depth eight;
  the true-model oracle also solves 40, equal-budget random ordering solves
  30, the one-step selector solves eight, and the changed-role-mask planner
  solves none; all eight unreachable problems are reported and average
  candidate evaluations grow from 1.9 to 6,969.8
- S0/S1: a 92-entry identity-independent and goal-conditioned value learner
  matches oracle search ordering, cutting depth-eight partial expansion from
  9,839.8 to 4,924.8 and reachable model applications from 80,112 to 40,272;
  full accounting remains negative because reachable total work rises from
  1,255,048 to 1,357,960 and unreachable total work more than doubles
- S1.1: the same frozen 92 values compile into a path-independent structural
  signature with identical outputs and ordering; reachable direct lookup work
  falls to 1,196,296 and local activation work to 1,236,712, both below neutral
  exhaustive work of 1,255,048, while unreachable guidance remains overhead
- S1.2: a six-entry gate trained only from selected-mode terminal work chooses
  neutral search for shallow and unreachable contexts and compiled guidance
  for depth eight; held-out work falls to 22,817,640 versus 23,278,200 always
  neutral and 35,232,600 always guided, and exploration repays by problem 1,439
- P0a-P0d: one shared generic proposal, probation, trace, correctness, and
  pruning configuration rediscovers lookup, feedback, self-trigger, and finish
  routes in all eight seeds per isolated gate
- P0e: fresh substrates receive the complete task with terminal correctness
  only; all eight real-feedback seeds construct compact four-arrow recurrent
  programs and score 512/512 on fresh identities and unseen depths, versus
  zero competent shuffled-feedback seeds, zero random-feedback seeds, and one
  accidental activity-only seed

## Interpretation Boundary

These are deterministic synthetic experiments. They do not establish
performance on unrestricted environments or show that the remaining supplied
representations would emerge from raw physical data.
