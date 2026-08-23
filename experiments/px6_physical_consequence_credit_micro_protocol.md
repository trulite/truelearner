# PX6 physical consequence-credit no-new-mechanism MICRO protocol

Status: **PREREGISTERED; MICRO EVIDENCE UNSPENT; DEVELOPMENT ONLY**.

## Frozen basis

- authoritative PX2 parent:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- unchanged substrate SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- positive PROBE v2 implementation SHA-256:
  `c5c21029f8be8abc90d6e1bd810e838484146945ebf71a5eacef43ca656ce681`;
- positive PROBE v2 CSV SHA-256:
  `7a5a90ceda8f668ea83eeb2bd46f6e523f10b8f2530932fe58a166558ffab50f`;
- positive PROBE v2 audit SHA-256:
  `04dd5a23b8ef14db6b11d8c820e4e094e2725d48b6783f42b925d59a6523f9c2`.

PROBE v1 remains an immutable evaluator negative. No PROBE identity or result
path may recur.

## Fresh 24-cell matrix

Run the six frozen PROBE worlds across four physical strata. Run every cell
twice from blank state and require exact complete-state equality.

| stratum | side spacing | candidate delay | first tick | presentation spacing | placement | allocation | arrival insertion |
|---|---:|---:|---:|---:|---|---|---|
| S0 | 40 | 1 | 2 | 14 | normal | normal | normal |
| S1 | 52 | 2 | 3 | 15 | mirrored | reversed | reversed |
| S2 | 64 | 1 | 4 | 16 | normal | reversed | normal |
| S3 | 76 | 2 | 5 | 17 | mirrored | normal | reversed |

Namespaces begin at `0x6_2200_0000`, advance by `0x0100_0000` per stratum,
and by `0x0010_0000` per world. Layout, allocation, and insertion order carry
no organism-visible role.

The candidate-delay-two strata place ordinary return exactly at the frozen
four-tick eligibility boundary. No timing window or law changes.

## Frozen conjunction

Every cell independently serializes and checks:

1. external participant firing and actual weak-arrow traversal;
2. downstream and trace firing;
3. physical return arrival at each source;
4. outward boundary crossing;
5. candidate resistance and liveness after eight presentations;
6. held-out outward execution from fresh activity;
7. global substrate work, pressure, deallocation, storage, and fingerprint;
8. natural quiescence and exact duplicate replay.

The expected retained and held-out vectors remain:

```text
left             true|false   1|0
right            false|true   0|1
both             true|true    1|1
correlation      false|false  0|0
crossed-return   false|false  0|0
no-return        false|false  0|0
```

Crossed-return and no-return again require positive measured participating-side
downstream execution before lawful deallocation, not execution after the weak
arrow has disappeared.

## Anti-smuggling boundary

Only the unchanged `PlasticSubstrate` executes. No old M source or state is
linked. No serializer, adapter, typed organism intermediate, outcome field,
evaluator-selected learning call, hidden boundary, route attribution, or
return ownership enters substrate state. Stratum/world values and expected
vectors remain external measurements only.

## Pass and stopping rule

All `24/24` cells and all duplicate developments must pass. Failure is frozen
without rescue. Success makes a separately preregistered GATE eligible; it
does not create authority, advance another PX target, or authorize definitive
evidence.
