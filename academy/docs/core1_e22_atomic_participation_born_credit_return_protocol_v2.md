# CORE1-E22 — Atomic Participation-Born Credit Return Protocol v2

## Status

Implementation-conformance correction after the first attempted P2 process.
Protocol v1 remains binding. No evidence marker or matrix result exists.

The first process is invalid staging, not a scientific P2 negative, because its
genesis hook did not execute at the event preregistered by v1.

## Misplaced hook

Protocol v1 defines atomic genesis at ordinary forward Drive traversal:

```text
contact fires
-> contact -> downstream arrow participates/emits
-> temporary returning -> contact edge appears in that same event
```

The first implementation instead waited until the downstream target received
the forward spike and itself crossed threshold. By then the generated contact's
incoming stem witness had already disappeared. The process recorded zero return
edges, zero Modulatory deliveries, and zero PQLC updates.

That is another post-participation reconstruction point and does not implement
the frozen candidate.

## Sole correction

Move the unchanged atomic-genesis call to the arrow-emission event immediately
after that Drive arrow records ordinary participation and before its forward
spike is enqueued. At that instant:

- the participating arrow supplies its source contact and downstream target;
- the live same-position incoming stem physically identifies the source as a
  generated subdivision;
- the temporary Modulatory return edge is added before any later delivery,
  decay, completion, quiescence, or inspection.

Remove the receiver-firing hook. No other topology, predicate, lifetime,
completion, consequence, PQLC, evaluator, or success rule changes.

The complete exact P2-first gate must rerun. Any subsequent valid P2 failure is
a stopped E22 negative with no further correction.
