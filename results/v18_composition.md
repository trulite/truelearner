# v18 Renaming-Invariant Composition

**Recorded:** 2026-08-18

## Question

Can the frozen v16 learner discover one reusable traversal procedure from
short demonstrations, then apply it to completely new symbols and much deeper
chains without additional learning?

## What Comes In

Each episode is a raw token stream containing shuffled link statements, a
distractor chain, a query, and, during training only, the correct answer.

Fixed stream markers identify link, query, answer, and end positions.

Training uses:

- 128 episodes
- chains two to four links long
- surface symbols 0 through 95

Held-out evaluation uses:

- eight episodes at each of five, eight, sixteen, and thirty-two links
- surface symbols 96 through 239
- no learning

The training and test symbol sets are disjoint.

## Results

```text
depth    event learner    context trie    validating walker
5        0 / 8            0 / 8           8 / 8
8        0 / 8            0 / 8           8 / 8
16       0 / 8            0 / 8           8 / 8
32       0 / 8            0 / 8           8 / 8
```

The walker proves that every held-out episode has a unique reachable answer.
It is not available to either learner.

## Work

```text
depth    event spikes/query    walker steps/query
5        802                   6
8        1,090                 9
16       1,858                 17
32       3,394                 33
```

Event work increases with serialized prompt length, but produces no correct
answers. It must not be interpreted as reasoning depth.

## Permanent Growth

```text
training examples    event patterns    event arrows    trie contexts
16                   1,786             5,475           1,675
32                   3,459             10,668          3,242
64                   6,668             20,671          6,253
128                  12,834            39,935          12,061
```

The learner continues accumulating surface patterns rather than converging on
one fixed traversal procedure.

Held-out read-only evaluation adds zero permanent pattern cells and zero
arrows.

## Controls

The validating walker correctly reports ambiguity for a branch, a cycle rather
than looping forever, and unknown for a query absent from the graph.

The event learner fabricates an answer on all three control prompts. This does
not show that it understands the malformed cases.

## Conclusion

V18 is a valid negative result:

> The existing cell-arrow-spike sequence learner does not discover
> renaming-invariant reusable composition.

The stream grammar and maximum pattern depth remain supplied. A specialized
temporary graph and traversal loop would solve the task, but adding them would
create another explicit mechanism rather than demonstrate learning.
