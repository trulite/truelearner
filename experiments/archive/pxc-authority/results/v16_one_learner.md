# v16 One-Learner Integration

**Recorded:** 2026-08-18

## Question

Can one cell-arrow-spike learner absorb several synthetic sequence problems
without importing the explicit frame, effect, tracking, planning, or procedure
modules from the earlier capability ladder?

## What Comes In

One opaque token at a time. Arrival order is the only supplied relationship.

## What Happens

The current token activates one receptor cell. Recently active pattern cells
join with it. A new join recruits another cell; later occurrences reuse that
cell. Learned arrows from active patterns carry prediction spikes toward token
receptors.

The same learner remains alive across the induction, many-key recall, and
needle probes. Only recent activity is reset. Learned cells and arrows remain.

## Results

```text
induction copying:          1482 / 1512 (98.0%)
one-token context control:  7.9%
many-key recall:            32 / 32
deep needle retrieval:      3 / 3
remapped control correct:   0
unknown query rejected:     true
learned pattern cells:      66,828
arrows:                     210,064
spikes:                     361,072
activity limit hits:        0
```

The three facts were inserted near the beginning, middle, and end of an
8,192-token noise stream.

## Interpretation

V16 is the first experiment in this repository where cells, arrows, and spikes
directly form one reusable sequence learner. Deeper learned patterns are
necessary for the induction result: the same learner restricted to one-token
memory reaches only 7.9%.

The result is associative sequence memory, not general algorithm learning.
The sixty-six-thousand learned pattern cells also show that the graph is
growing rather than discovering an efficient compressed representation.

## Supplied Structure

- token boundaries and ordered arrival
- separate joining and prediction phases
- pattern-cell recruitment
- preference for the deepest matching pattern
- external recent-activity reset
- fixed activity limit

Reversal, sorting, learned phase control, autonomous pruning, and graph
compression remain unproven.
