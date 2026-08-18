# v17 Consolidation

**Recorded:** 2026-08-18

## Question

Is v16 doing more than a variable-depth trie, and can its tested behavior
survive a physically smaller permanent graph?

## Baseline Result

```text
                         v16 event graph    plain trie
induction accuracy       97.8%              97.8%
key-value recall         32 / 32            32 / 32
needle retrieval         3 / 3              3 / 3
pattern nodes            66,826             66,826
links                    210,057            143,231
```

The simple trie reproduces v16's tested behavior with fewer links. V16 should
therefore be interpreted as an event-driven variable-depth associative memory,
not as evidence of autonomous abstraction.

## Rest Result

```text
pattern cells            66,826 -> 11,741
arrows                   210,057 -> 44,818
induction accuracy       97.8% -> 97.8%
key-value recall         32 / 32 -> 32 / 32
needle retrieval         3 / 3 -> 3 / 3
work per query           15.9 -> 15.9
active patterns/query    7.3 -> 7.3
```

The consolidation rebuild physically removes one-off patterns. It does not
leave inactive cells in the original allocation.

The candidate rewrite is tested in read-only mode. No missing cell or arrow
can be silently recreated during evaluation.

## Control

The consolidated graph is copied without changing its size. Its prediction
targets are then reassigned arbitrarily. This destroys the learned behavior,
showing that graph reduction alone is insufficient.

## Experience Sweep

```text
experience tokens    raw patterns    consolidated patterns
1,704                9,810           1,931
2,728                16,880          2,022
4,776                30,766          2,420
8,872                57,986          3,574
17,064               110,445         6,576
```

Across this sweep, raw pattern storage grows more than eleven times.
Consolidated storage grows about three and a half times. Consolidated induction
accuracy remains above 95%.

## Replay Accounting

The main rest experiment uses 1,604 replay tokens to validate its candidate
rewrite. The replay set is counted during rest and is not retained afterward.

## Interpretation Boundary

V17 learns which exact patterns recur and discards most one-off contexts. It
does not merge different surface patterns into one shared structural concept.

The recurrence threshold, rest timing, rewrite operation, and replay suite are
supplied. V18 must test whether the machine can reuse one learned relationship
across previously unseen token identities.
