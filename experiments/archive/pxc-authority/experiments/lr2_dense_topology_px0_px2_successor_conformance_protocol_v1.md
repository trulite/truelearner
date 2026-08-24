# LR2 dense-topology and PX0--PX2 successor-conformance protocol v1

Status: **PREREGISTERED DEVELOPMENT; B/C IMPLEMENTATIONS AND RESULTS UNSPENT**.

Start: frozen LR1 parallel result commit `d3bc67c`, where route-aware Arm A was
negative and compartmental Arm B and modulatory Arm C were functional
positives. PX0--PX2 authority remains frozen and is not rerun by LR2.

## Question

Can either LR1 successor law preserve qualified-return discrimination under
dense, recurrent topology while reproducing the already-authoritative PX0,
PX1 and PX2 behavioral surfaces without per-world tuning?

LR2 is a development selector, not authority. A positive result may select a
successor law for later conformance replay; it cannot rewrite PX0 authority,
restore PX3 authority, or authorize PX4.

## Frozen arms

- **B -- compartmental return:** ordinary return activity lands on a separate
  ordinary CELL adjacent to the execution source. Arrival there may change a
  recently eligible outgoing ARROW without entering the source's firing state.
  There is no transmission mode.
- **C -- modulatory transmission:** drive and modulatory ARROW traversals are
  physically distinct. Drive changes activation; modulation may change
  recently eligible outgoing ARROWs but does not change activation.

The LR1 law files are copied byte-for-byte. LR2 may add only evaluator/harness
topology and measurements. No law constant or active-state layout may change.
Both arms use the same seeds, timings, amplitudes, causal schedules, pass
predicates, result schema and replay discipline. Their topology differs only
where the frozen laws require a compartment or a transmission mode.

## Fresh development matrix

Seeds `5201, 5209, 5227, 5231` cross normal/mirrored placement, normal/reverse
allocation and normal/reverse insertion. Every cell executes twice from an
identical blank state. All propagation must quiesce naturally.

### Dense specificity: ten worlds per seed

Each dense fixture contains two simultaneously eligible candidates, two lawful
return routes, eight forward distractors, eight nearby return distractors, two
cross-routes and mirrored inert structure. All physical impulses are one.

1. target candidate traverses; unrelated nearby downstream route arrives;
   lawful target return absent: target updates `0`;
2. same dense world with lawful target return: target updates exactly `1`;
3. target traverses; return for the other candidate arrives: target updates
   `0`, other candidate updates only if it participated;
4. both candidates traverse; only target return arrives: exactly target learns;
5. both candidates traverse; both lawful returns arrive: both learn once;
6. target traverses; cross-route and distractors arrive simultaneously but no
   lawful return: target updates `0`;
7. target traverses; lawful return and all distractors arrive simultaneously:
   target learns exactly once;
8. target does not traverse; lawful target route arrives: target updates `0`;
9. target traverses; lawful return arrives after eligibility expiry: target
   updates `0`;
10. immediate renewed upstream episode with no lawful return: it may execute
    the candidate again but provides `0` plasticity updates.

The decisive B hazard is intentionally physical: unrelated ordinary activity
is delivered to a crowded neighboring compartment. It is not evaluator state.
The matched C activity is ordinary drive on the corresponding route and lacks
modulatory transmission. No arm receives a route, credit, outcome or cause ID.

### PX0 lifecycle conformance: six surfaces per seed

Fresh compact fixtures test the authoritative behavioral predicates, not the
spent authority cells:

1. four traversals with qualified return acquire reusable correspondence;
2. one returned traversal is probationary and dies under pressure;
3. return supports only the physically corresponding eligible route;
4. stable structure survives bounded absence then deallocates under sufficient
   ordinary pressure;
5. renewed activity after deallocation creates a fresh ARROW generation; and
6. repeated contemporary return reacquires executable correspondence.

### PX1 participation conformance: six surfaces per seed

The six authoritative causal worlds are reproduced with ordinary transient
participation timing:

1. support A;
2. support B;
3. no support;
4. participation with qualified return blocked;
5. identical qualified return without continuation participation; and
6. genuine joint participation.

Claims require actual branch/outlet traversal, learning only for a live
participation window, supported held-out reuse, no unsupported reuse, no
autonomous source refiring and natural quiescence.

### PX2 causal-direction conformance: seven surfaces per seed

The seven authoritative causal worlds are reproduced:

1. forward traversal with qualified return;
2. reverse traversal with qualified return;
3. matched consequences/return without candidate traversal;
4. forward traversal with return blocked;
5. genuine joint traversal and return;
6. forward traversal while wrong-way observations occur at least three times
   as often; and
7. the mirrored reverse case.

Claims require direction maturation only after actual traversal plus qualified
return; correlation-only and traversal-only worlds remain unsupported; the
more frequent non-traversed observation cannot override the traversed side.

## Accounting and verdict

Each arm serializes one row per seed/world with separate fields for traversal,
qualified-return attempts/accepts, resistance/coupling/liveness, firings,
autonomous refiring, pressure/deallocation, work, persistent bytes, complete
fingerprint, permanent fingerprint, quiescence, exact replay and each claim
bit. CSV text fields may not contain commas.

The summary table reports:

- dense rows and clauses;
- PX0, PX1 and PX2 conformance rows and clauses independently;
- total/max work and persistent bytes;
- added persistent state fields;
- added transmission modes;
- replay and quiescence.

An arm is a functional LR2 positive only if every registered row and clause
passes. If both pass, B is selected developmentally on lower substrate cost
unless C has a registered robustness or work advantage. If B fails dense
specificity and C passes all suites, C becomes the development successor
candidate. Any other split remains unresolved. No result is definitive.

## Execution discipline

Both harnesses and this protocol are frozen before either result runs. Rust
formatting, compilation, tests, lint, preflight and execution occur only in two
fresh isolated E2B sandboxes. Preflights and result commands may run in
parallel. Each development result command runs once in its own sandbox;
artifacts are downloaded without tuning or rescue.
