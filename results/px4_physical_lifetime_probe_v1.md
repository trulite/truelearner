# PX4 physical learned-lifetime PROBE v1

Status: **FAIL** (`0/6` cells).

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` / `px2-physical-causal-direction-authoritative`.

Active-law SHA-256: `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`. No substrate or PX0--PX2 source changed.

This is development evidence only. It is not a definitive matrix and creates no authority.

| layout | case | result | trained resistance | final resistance | before effects | after effects | stale refused | work | slots/live/bytes |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P0 | matched-high-use-survival | FAIL | 0|0 | 0|0 | 0|0 | 0|0 | false | 10491 | 30/28/2976 |
| P0 | matched-low-use-forgetting | FAIL | 0|0 | 0|0 | 0|0 | 0|0 | true | 4036 | 30/26/2976 |
| P0 | disuse-to-zero | FAIL | 0|0 | 0|0 | 0|0 | 0|0 | true | 7673 | 30/26/2976 |
| P0 | forward-to-reverse-competition | FAIL | 0|0 | 0|34 | 0|0 | 0|1 | true | 25274 | 31/29/3040 |
| P0 | correlation-without-traversal | FAIL | 0|0 | 0|0 | 0|0 | 0|0 | true | 11529 | 30/28/2976 |
| P0 | traversal-without-return | FAIL | 0|0 | 0|0 | 0|0 | 0|0 | true | 8941 | 28/26/2848 |

## Interpretation

At least one preregistered physical clause failed. This result is frozen without rescue or rerun.
