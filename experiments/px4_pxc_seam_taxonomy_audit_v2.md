# PX4 PX-C seam taxonomy audit v2

Status: **DEVELOPMENT TAXONOMY POSITIVE; COMPARATOR UNSPENT; AUTHORITY ABSENT**.

Fresh E2B sandbox `ivhh9mi0wathsydi2wbiq` audited clean functional-result
commit `8714ce55f2c261b5209b0b42ccbad35b3d31b26d` with the frozen v2 taxonomy
script and manifest SHA-256
`28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`.
It used unique state file `px4-pxc-taxonomy-20260824.json` and was left
running.

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
| generated report | `32816a7a11af853b21ea82831f349f5fa1187128641156f225de2f20a7507f11` |

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
