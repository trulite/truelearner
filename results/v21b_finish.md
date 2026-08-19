# V21b Learned Finish

## Question

Can the machine learn that a lookup with no successor means it should emit the
last valid current identity as an explicit answer?

V21b freezes the successful v21a continuation path. It adds no new behavior to
successful lookup.

## Supplied Structure

After every queued lookup comparison finishes, the runtime already knows
whether it collected zero, one, or multiple successors. V21b exposes the zero
case as a neutral no-result event.

Four candidate routes are supplied:

- no result to explicit answer,
- no result to apply again,
- no result to clear current,
- no result to quiet.

The runtime does not assign meaning to no result. Terminal supervision selects
the route using only the complete expected answer.

## Training

Training uses fresh finite chains one through four links deep.

```text
training episodes    permanent cells    permanent arrows    validation
10                   15                 18                  100 / 100
100                  15                 18                  100 / 100
1,000                15                 18                  100 / 100
```

The complete v21a permanent fingerprint remains unchanged. Only the new finish
route strengths and finish training counter change.

## Held-Out Depth

All depth-sweep episodes contain forty relations. Main-chain depth changes and
distractors fill the remaining relation slots.

```text
chain depth    learner    supplied finish    no finish    internal spikes    apply activations
5              200/200    200/200            0/200        515                6
8              200/200    200/200            0/200        773                9
16             200/200    200/200            0/200        1,461              17
32             200/200    200/200            0/200        2,837              33
```

The extra apply activation performs the final lookup that finds no successor.
Every run:

- receives one external start,
- uses no execution cutoff,
- emits exactly one explicit answer,
- reaches an empty queue naturally,
- avoids the activity limit.

At fixed temporary graph size, each additional successful lookup adds 86
internal spikes. The complete run also includes the final unsuccessful lookup
and finish event.

## Working-Set Scaling

Reasoning depth stays fixed at eight while temporary relation count changes.

```text
relations          8     16     32      64       128
internal spikes   197   341    629   1,205     2,357
```

Each additional temporary relation adds 18 spikes to the complete depth-eight
run because every one of the nine lookup attempts compares both relation
slots.

This is linear but not sparse retrieval. The machine still wakes every
temporary identity occurrence for each lookup.

## Final Trace Audit

The final semantic events are exactly:

```text
current contains terminal identity
lookup terminal identity
no result
finish arrow 14
explicit answer containing terminal identity
```

The queue is empty afterward. No other semantic answer event occurs.

The successful route still uses:

```text
apply cell          6
lookup arrow        1
feedback arrow      4
self-trigger arrow  10
```

## Controls

- The frozen v21a machine without finish emits no answer.
- A supplied answer route establishes the upper bound.
- A local branch remains ambiguous and emits no answer.
- Duplicate identical relations produce one answer.
- A zero-link chain answers its query identity.
- A two-node cycle emits no answer and reaches the safety limit.

## V18 Distribution Recomposition

The v21b substrate solves 32 of 32 fresh episodes at depths five, eight,
sixteen, and thirty-two with a two-link distractor chain.

This does not change the historical v18 result. The original unified sequence
learner remains at zero of thirty-two. The later substrate succeeds only after
adding generic identity equality, relation slots, temporary state, no-result
events, and separately learned route selections.

## Conclusion

V21b supports this narrow claim:

> A finish route selected from terminal supervision converts structural
> absence into an explicit answer, allowing one externally started recurrent
> computation to determine its own finite execution length.

Cycle detection and sparse temporary retrieval remain unsolved.
