# V21a Autonomous Continuation

## Question

Can one successful learned lookup trigger another application of the same
operation without another external apply event?

V21a isolates continuation. It does not ask the learner to decide when to
finish.

## Input Boundary

The evaluator provides:

- temporary relation statements and one query identity,
- one external start spike,
- a safety cutoff after a requested number of successful lookups.

The evaluator never calls apply or lookup. A generic queue loop only delivers
spikes emitted by cells and stops when the observed cutoff is reached.

## Queued Execution

After the external start spike, the queued path is:

```text
start
apply
lookup comparisons
result
feedback
current
self-trigger
apply again
```

Lookup comparison is also queued. Each temporary identity occurrence receives
a comparison spike. Matching occurrences emit the selected frozen v19 route,
and a finalizer produces not found, one result, or ambiguous.

## Learning

Training episodes contain exactly two-link chains plus distractors. Three
candidate self-routes are supplied:

- apply again,
- read now,
- become quiet.

Terminal supervision contains only the expected identity after two successful
lookups. Every candidate is evaluated using one start spike and the same queued
runtime.

```text
training episodes    permanent cells    permanent arrows    validation
10                   12                 13                  100 / 100
100                  12                 13                  100 / 100
1,000                12                 13                  100 / 100
```

## Held-Out Depth

All held-out episodes contain a forty-link chain and eight distractor
relations. Only the evaluator cutoff changes. This holds temporary graph size
constant across the depth sweep.

```text
completed lookups    learner    supplied self-trigger    no self-trigger    internal spikes
1                    200/200    200/200                  200/200            103
2                    200/200    200/200                  0/200              205
4                    200/200    200/200                  0/200              409
8                    200/200    200/200                  0/200              817
16                   200/200    200/200                  0/200              1,633
32                   200/200    200/200                  0/200              3,265
```

Every run receives exactly one external start spike. Apply activations equal
the requested number of completed lookups. At each cutoff the selected
self-trigger has already emitted exactly one next-apply spike, which the
evaluator discards.

## Trace Audit

The thirty-two-step trace reuses:

```text
apply cell          6
lookup arrow        1
feedback arrow      4
self-trigger arrow  10
```

Each trace current identity equals the corresponding node in the held-out
chain. The first step emits 103 internal spikes; each later step emits 102.

## Memory Audit

Held-out evaluation encounters 68,400 fresh opaque identities.

```text
permanent cells             12
permanent arrows            13
permanent fingerprint       unchanged
temporary state after episode   zero
owned temporary capacity    zero
activity-limit hits         zero
```

## Controls

- A supplied self-trigger route passes every cutoff but receives no learning
  credit.
- A v20-style machine without self-trigger stops after one lookup.
- A local branch remains ambiguous.
- Duplicate identical relations produce one result.

## Conclusion

V21a supports this narrow claim:

> A self-trigger route selected from terminal supervision allows one external
> start event to generate repeated applications of the same learned lookup and
> feedback operation inside a queued spike runtime.

Execution depth is still externally capped. The machine has not learned when
to finish, detect cycles, or protect itself from every possible recurrent
structure.
