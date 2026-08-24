# PX4 PX-C seam taxonomy audit v2

Status: **DEVELOPMENT TAXONOMY POSITIVE; COMPARATOR UNSPENT; AUTHORITY ABSENT**.

Fresh E2B sandbox `i6mlrh7u2iv4abtdkz01e` audited clean readiness commit
`b0ac4b3c2e9ea49a9d3a13098088a62108d508e8` with the frozen v2 taxonomy
script and manifest SHA-256
`28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`.
It used unique state file `px4-pxc-taxonomy-final-20260824-b.json` and was
left running. This final replay supersedes the byte-equal inventories and
counts from earlier fresh taxonomy sandbox `ivhh9mi0wathsydi2wbiq`; only the
generated report's audited-commit line differs. The superseded earlier sandbox
was later terminated after its artifacts were frozen.

The E2B invocation enforced:

```text
PXC_MAX_TOTAL=367
PXC_MAX_SEMANTIC_GUARD=218
PXC_MAX_EVALUATOR_GUARD=752
```

It also required the exact v2 manifest hash and reverified all three immutable
v1 baseline artifact hashes before scanning.

## Artifact hashes

| artifact | SHA-256 |
|---|---|
| taxonomy inventory | `742532d904622ecf4c5641b55f078fbd9f732b7f5898d5120b0f730c3a0ccec0` |
| guard inventory | `5a23d1f89476d87f2a630d89145cd87a8a6169d9a1a72d6b309cf859400d8675` |
| summary | `59c48919e91d698033a7931c15690fa148691a8afaf89cd22a4b7794ce4e2ee0` |
| generated report | `0949eb76b65ca6f45d46fdb3d8178267c133f9f32cdfee222f651d3fea7d68f6` |

## Exact taxonomy result

| primary kind | immutable PX3 baseline | PX4 after | delta |
|---|---:|---:|---:|
| typed representation | 87 | 85 | -2 |
| explicit mechanism invocation | 72 | 66 | -6 |
| episode/reset boundary | 1 | 1 | 0 |
| seed/history synthesis | 61 | 9 | -52 |
| semantic condition | 38 | 38 | 0 |
| manual temporary cleanup | 1 | 1 | 0 |
| typed handoff | 98 | 90 | -8 |
| evaluator-derived input | 10 | 7 | -3 |
| **primary total** | **368** | **297** | **-71** |

| relocation guard | baseline | after | delta |
|---|---:|---:|---:|
| semantic condition | 218 | 162 | -56 |
| evaluator-derived input | 752 | 559 | -193 |

| layer | baseline | after | delta |
|---|---:|---:|---:|
| PX0--PX3+LR-C | 0 | 0 | 0 |
| PX4 | 71 | 0 | -71 |
| PX5 | 14 | 14 | 0 |
| PX6 | 37 | 37 | 0 |
| PX7 | 136 | 136 | 0 |
| PX8 | 110 | 110 | 0 |

The after inventory contains 297 occurrences on 232 unique source lines. The
active PX4 surface contributes zero headline or guarded matches. All remaining
counts belong to unchanged PX5--PX8 predecessor entries; neither the
authoritative foundation nor another future lane changed.

The separately frozen readiness comparator must still prove new seam kinds
`0` and new guarded surfaces `0` from raw inventories. This audit does not
advance authority.
