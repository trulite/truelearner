# FD1 consequence consolidation immutable negative v1

Status: frozen negative. FD1 development readiness was not established.

Frozen candidate: `fd1-consequence-consolidation-frozen-v1`
(`b7df578`).

E2B evidence sandbox: `iacjqsbp8sukwc27n77o2`.

## Observed stop

The focused evaluator executed once. It stopped at the first identity root,
creation phase zero, family C3:

```text
FD1 family failed: same_durable_state_same_future
```

The evaluator order establishes the following limited facts before the stop:

- C0 unused proposal passed;
- C1 repeated use without consequence passed;
- C2 one qualified consequence passed;
- C3 same-mechanics replay passed under Reference;
- C3 same-mechanics replay passed under Production;
- the complete C3 Reference and Production observations were exactly equal;
- the frozen C3 scientific predicate was false.

Therefore the failure is not a Reference/Production mechanics divergence or a
replay divergence. It is a failure of at least one member of the preregistered
C3 conjunction:

```text
same candidate state immediately after consolidation
same candidate state at the last-live observation
same candidate state after death
same future PhysicalWork
```

The evaluator asserted before serializing rows, so the exact false member is
not recoverable from this event. No result matrix, report, or checksum file was
published.

## Unexecuted gates

Because the focused matrix stopped immediately:

- C4 wrong-path Modulation did not run;
- C5 late stale return did not run;
- C6 repeated supported consequence did not run;
- the unchanged FD0 evaluator did not run;
- the FD0 exact artifact-hash check did not run;
- the final static evidence command did not run.

No ARC case, CPC/PQLC replay, resource competition, authority, oracle, or
`arch.md` change occurred.

## Scientific accounting

FD1 v1 remains permanently negative. The event does not establish whether the
candidate consolidation-rebase law is wrong or whether C3 compared a transient
or work component that was not actually normalized by its fixture. That
classification requires a separately preregistered diagnostic which serializes
each C3 conjunct independently. FD1 v1 may not be rerun, repaired, or relabeled.
