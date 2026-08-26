# RS2 learned inhibitory topology handoff v4

```text
J0 / CV0 / SV1       development positive
RS2 v2               training-timing fixture negative
RS2 v3               evaluator-identity fixture negative
RS2 v4               complete frozen negative
CE1                   not run
FD2 v2                not run
frozen ARC A2         not run
```

RS2 v4 fixed the identity collision and executed all 180 cases. Its immutable
result is `2760/3160` clauses with exact replay, but not a positive RS2 result:

- all 180 Reference/Production comparisons fail the frozen complete-observation
  equality; the only serialized paired-row difference is live-checkpoint hash;
- all 20 identity-permutation cases fail the exact-one selected-contact
  traversal predicate in both mechanics because the contact fires twice;
- all other family-specific checks pass.

The prefix-based batch stopped correctly. Any successor requires a fresh,
separately preregistered diagnostic that can localize the complete physical
trace/checkpoint difference and the extra contact firing without changing or
rerunning v4.

