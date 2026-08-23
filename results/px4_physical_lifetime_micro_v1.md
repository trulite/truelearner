# PX4 physical learned-lifetime MICRO v1

Status: **PASS** (`24/24` cells).

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` / `px2-physical-causal-direction-authoritative`.

Active-law SHA-256: `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`. No substrate or PX0--PX2 source changed.

This is development evidence only. It is not a definitive matrix and creates no authority.

| layout | case | result | trained resistance | final resistance | before effects | after effects | stale refused | work | slots/live/bytes |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| M0 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 12413 | 30/29/2976 |
| M0 | matched-low-use-forgetting | PASS | 9|0 | 0|0 | 1|0 | 0|0 | true | 4515 | 30/26/2976 |
| M0 | disuse-to-zero | PASS | 17|0 | 0|0 | 1|0 | 0|0 | true | 8956 | 30/26/2976 |
| M0 | forward-to-reverse-competition | PASS | 17|0 | 0|35 | 1|0 | 0|1 | true | 26570 | 31/29/3040 |
| M0 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 11533 | 30/28/2976 |
| M0 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 9036 | 28/26/2848 |
| M1 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 18533 | 34/33/3616 |
| M1 | matched-low-use-forgetting | PASS | 8|0 | 0|0 | 1|0 | 0|0 | true | 6179 | 34/30/3616 |
| M1 | disuse-to-zero | PASS | 16|0 | 0|0 | 1|0 | 0|0 | true | 13121 | 34/30/3616 |
| M1 | forward-to-reverse-competition | PASS | 16|0 | 0|36 | 1|0 | 0|1 | true | 40949 | 35/33/3680 |
| M1 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 17509 | 34/32/3616 |
| M1 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 14822 | 32/30/3488 |
| M2 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 35880 | 44/41/5024 |
| M2 | matched-low-use-forgetting | PASS | 9|0 | 0|0 | 1|0 | 0|0 | true | 10627 | 44/38/5024 |
| M2 | disuse-to-zero | PASS | 17|0 | 0|0 | 1|0 | 0|0 | true | 24731 | 44/38/5024 |
| M2 | forward-to-reverse-competition | PASS | 17|0 | 0|35 | 1|0 | 0|1 | true | 81483 | 45/41/5088 |
| M2 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 34562 | 44/40/5024 |
| M2 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 31227 | 42/38/4896 |
| M3 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 69941 | 54/53/6816 |
| M3 | matched-low-use-forgetting | PASS | 8|0 | 0|0 | 1|0 | 0|0 | true | 19655 | 54/50/6816 |
| M3 | disuse-to-zero | PASS | 16|0 | 0|0 | 1|0 | 0|0 | true | 47717 | 54/50/6816 |
| M3 | forward-to-reverse-competition | PASS | 16|0 | 0|36 | 1|0 | 0|1 | true | 160607 | 55/53/6880 |
| M3 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 68203 | 54/52/6816 |
| M3 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 64116 | 52/50/6688 |

## Interpretation

All preregistered physical clauses passed using only byte-frozen PX0--PX2 state and laws. Resistance changed through actual traversal/ordinary return and ordinary pressure; zero-resistance paths refused stale execution. No lifetime-specific mechanism executed.
