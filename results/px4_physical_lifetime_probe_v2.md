# PX4 physical learned-lifetime PROBE v1

Status: **PASS** (`6/6` cells).

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` / `px2-physical-causal-direction-authoritative`.

Active-law SHA-256: `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`. No substrate or PX0--PX2 source changed.

This is development evidence only. It is not a definitive matrix and creates no authority.

| layout | case | result | trained resistance | final resistance | before effects | after effects | stale refused | work | slots/live/bytes |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P0 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 12413 | 30/29/2976 |
| P0 | matched-low-use-forgetting | PASS | 9|0 | 0|0 | 1|0 | 0|0 | true | 4515 | 30/26/2976 |
| P0 | disuse-to-zero | PASS | 17|0 | 0|0 | 1|0 | 0|0 | true | 8956 | 30/26/2976 |
| P0 | forward-to-reverse-competition | PASS | 17|0 | 0|35 | 1|0 | 0|1 | true | 26570 | 31/29/3040 |
| P0 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 11533 | 30/28/2976 |
| P0 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 9036 | 28/26/2848 |

## Interpretation

All preregistered physical clauses passed using only byte-frozen PX0--PX2 state and laws. Resistance changed through actual traversal/ordinary return and ordinary pressure; zero-resistance paths refused stale execution. No lifetime-specific mechanism executed.
