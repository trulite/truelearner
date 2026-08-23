# PX4 physical learned-lifetime GATE v1

Status: **PASS** (`32/32` cells).

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` / `px2-physical-causal-direction-authoritative`.

Active-law SHA-256: `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`. No substrate or PX0--PX2 source changed.

This is development evidence only. It is not a definitive matrix and creates no authority.

| layout | case | result | trained resistance | final resistance | before effects | after effects | stale refused | work | slots/live/bytes |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| G0 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 12659 | 32/29/3104 |
| G0 | matched-low-use-forgetting | PASS | 9|0 | 0|0 | 1|0 | 0|0 | true | 4473 | 32/26/3104 |
| G0 | disuse-to-zero | PASS | 17|0 | 0|0 | 1|0 | 0|0 | true | 9074 | 32/26/3104 |
| G0 | forward-to-reverse-competition | PASS | 17|0 | 0|36 | 1|0 | 0|1 | true | 27299 | 33/29/3168 |
| G0 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 11755 | 32/28/3104 |
| G0 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 9269 | 30/26/2976 |
| G0 | reverse-to-forward-competition | PASS | 0|17 | 36|0 | 0|1 | 1|0 | true | 27299 | 33/29/3168 |
| G0 | full-deallocation-opposite-reacquisition | PASS | 17|0 | 0|36 | 1|0 | 0|1 | true | 30480 | 36/29/3360 |
| G1 | matched-high-use-survival | PASS | 22|0 | 11|0 | 1|0 | 1|0 | false | 26997 | 40/37/4384 |
| G1 | matched-low-use-forgetting | PASS | 8|0 | 0|0 | 1|0 | 0|0 | true | 8456 | 40/34/4384 |
| G1 | disuse-to-zero | PASS | 16|0 | 0|0 | 1|0 | 0|0 | true | 18824 | 40/34/4384 |
| G1 | forward-to-reverse-competition | PASS | 16|0 | 0|36 | 1|0 | 0|1 | true | 60472 | 41/37/4448 |
| G1 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 25800 | 40/36/4384 |
| G1 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 22703 | 38/34/4256 |
| G1 | reverse-to-forward-competition | PASS | 0|16 | 36|0 | 0|1 | 1|0 | true | 60472 | 41/37/4448 |
| G1 | full-deallocation-opposite-reacquisition | PASS | 16|0 | 0|36 | 1|0 | 0|1 | true | 64810 | 44/37/4640 |
| G2 | matched-high-use-survival | PASS | 23|0 | 12|0 | 1|0 | 1|0 | false | 69941 | 54/53/6816 |
| G2 | matched-low-use-forgetting | PASS | 9|0 | 0|0 | 1|0 | 0|0 | true | 19602 | 54/50/6816 |
| G2 | disuse-to-zero | PASS | 17|0 | 0|0 | 1|0 | 0|0 | true | 47664 | 54/50/6816 |
| G2 | forward-to-reverse-competition | PASS | 17|0 | 0|35 | 1|0 | 0|1 | true | 160608 | 55/53/6880 |
| G2 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 68203 | 54/52/6816 |
| G2 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 64114 | 52/50/6688 |
| G2 | reverse-to-forward-competition | PASS | 0|17 | 35|0 | 0|1 | 1|0 | true | 160608 | 55/53/6880 |
| G2 | full-deallocation-opposite-reacquisition | PASS | 17|0 | 0|35 | 1|0 | 0|1 | true | 170108 | 60/53/7200 |
| G3 | matched-high-use-survival | PASS | 22|0 | 11|0 | 1|0 | 1|0 | false | 178849 | 80/77/10784 |
| G3 | matched-low-use-forgetting | PASS | 8|0 | 0|0 | 1|0 | 0|0 | true | 47666 | 80/74/10784 |
| G3 | disuse-to-zero | PASS | 16|0 | 0|0 | 1|0 | 0|0 | true | 120684 | 80/74/10784 |
| G3 | forward-to-reverse-competition | PASS | 16|0 | 0|35 | 1|0 | 0|1 | true | 414171 | 81/77/10848 |
| G3 | correlation-without-traversal | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 176198 | 80/76/10784 |
| G3 | traversal-without-return | PASS | 0|0 | 0|0 | 0|0 | 0|0 | true | 170335 | 78/74/10656 |
| G3 | reverse-to-forward-competition | PASS | 0|16 | 35|0 | 0|1 | 1|0 | true | 414171 | 81/77/10848 |
| G3 | full-deallocation-opposite-reacquisition | PASS | 16|0 | 0|35 | 1|0 | 0|1 | true | 424905 | 84/77/11040 |

## Interpretation

All preregistered physical clauses passed using only byte-frozen PX0--PX2 state and laws. Resistance changed through actual traversal/ordinary return and ordinary pressure; zero-resistance paths refused stale execution. No lifetime-specific mechanism executed.
