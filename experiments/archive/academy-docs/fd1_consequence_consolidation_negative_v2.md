# FD1 consequence consolidation immutable negative v2

Status: frozen negative; classified evaluator observation-time defect.

Frozen evaluator: `fd1-consequence-consolidation-v2-frozen-v1`
(`ba0c6b4`).

E2B evidence sandbox: `ijopamqsfuedzhb3416ja`.

## Result

- physical cases serialized: `140/140`;
- mechanics rows serialized: `280/280`;
- C0, C1, C2, C4, C5, C6: `20/20` each;
- C3: `0/20`;
- same-mechanics replay: passed in every row;
- Reference/Production equality: passed in every row;
- natural quiescence: passed in every row;
- C3 internal `equivalent_future`: true in every row.

The focused assertion stopped after the complete artifacts were written. FD0
correctly did not execute.

## Exact defect

`equivalent_future` captured `early_after`, `early_last`, `early_dead`,
`late_after`, `late_last`, and `late_dead` at their correct physical times and
confirmed their equality. However, it constructed the three serialized `Point`
values only after advancing the retained late world to death:

```text
advance late world to age 49
point("after_consequence", 9)  // reads dead age-49 state
point("last_live", 48)         // reads dead age-49 state
point("death", 49)             // reads dead age-49 state
```

Consequently every C3 row records:

```text
after_consequence: dead / resistance 0 / decay load 0
last_live:         dead / resistance 0 / decay load 0
death:             dead / resistance 0 / decay load 0
```

while `equivalent_future=1`, replay, mechanics equality, and quiescence all
remain true. The diagnostic had already serialized the correctly captured
states:

```text
after consequence: live / resistance 4 / decay load 0
39 ticks later:    live / resistance 1 / decay load 9
40 ticks later:    dead / resistance 0
```

Classification: evaluator/measurement defect. The candidate physical law is
not contradicted by v2.

## Accounting

V2 remains permanently negative and may not be rerun or relabeled. A fresh v3
is scientifically eligible with exactly one repair: retain the already-captured
late state at each observation time when constructing its serialized `Point`.
No core, schedule, predicate, expected value, identity, or other family may
change.

Artifacts:

- matrix SHA-256
  `89a017fdef99ceada9b7609dc181b53c9b7b24f177a26f1e08298de0eb1d7da8`;
- report SHA-256
  `20f5406a2f1d1cc4ad78439a07545838415381308f2b819bfa95b07bc19f48d6`;
- checksum-manifest SHA-256
  `cc524fd3bc2667a1231b35ded2add9517350fb470de0a1130b9a9ffac80c5ab0`.

No FD0 replay, ARC, CPC/PQLC, RC0, authority, oracle, or `arch.md` change
occurred.
