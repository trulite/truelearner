# PX2-H1 matched-schedule hysteresis diagnostic v1

Classification: **C — ordering matters but protection-first is not sufficient**.

Cells: `40`; duplicate-exact: `40`.

| stratum | schedule | first | first mature | first deallocation | final resistance | held-out | post-gap | replay |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| G0 | forward-block | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G0 | reverse-block | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G0 | forward-alternating | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G0 | reverse-alternating | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G0 | rotation-0 | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G0 | rotation-1 | 0 | `0|-1` | `5|1` | `0|0` | `0|0` | `0|0` | true |
| G0 | rotation-2 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G0 | rotation-3 | 0 | `0|-1` | `3|1` | `0|0` | `0|0` | `0|0` | true |
| G0 | rotation-4 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G0 | rotation-5 | 1 | `-1|0` | `1|3` | `0|0` | `0|0` | `0|0` | true |
| G1 | forward-block | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G1 | reverse-block | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G1 | forward-alternating | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G1 | reverse-alternating | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G1 | rotation-0 | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G1 | rotation-1 | 0 | `0|-1` | `5|1` | `0|0` | `0|0` | `0|0` | true |
| G1 | rotation-2 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G1 | rotation-3 | 0 | `0|-1` | `3|1` | `0|0` | `0|0` | `0|0` | true |
| G1 | rotation-4 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G1 | rotation-5 | 1 | `-1|0` | `1|3` | `0|0` | `0|0` | `0|0` | true |
| G2 | forward-block | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G2 | reverse-block | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G2 | forward-alternating | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G2 | reverse-alternating | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G2 | rotation-0 | 0 | `0|-1` | `-1|1` | `5|0` | `1|0` | `1|0` | true |
| G2 | rotation-1 | 0 | `0|-1` | `5|1` | `0|0` | `0|0` | `0|0` | true |
| G2 | rotation-2 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G2 | rotation-3 | 0 | `0|-1` | `3|1` | `0|0` | `0|0` | `0|0` | true |
| G2 | rotation-4 | 1 | `-1|0` | `1|-1` | `0|5` | `0|1` | `0|1` | true |
| G2 | rotation-5 | 1 | `-1|0` | `1|3` | `0|0` | `0|0` | `0|0` | true |
| G3 | forward-block | 0 | `0|-1` | `-1|1` | `4|0` | `1|0` | `1|0` | true |
| G3 | reverse-block | 1 | `-1|0` | `1|-1` | `0|4` | `0|1` | `0|1` | true |
| G3 | forward-alternating | 0 | `0|-1` | `-1|1` | `4|0` | `1|0` | `1|0` | true |
| G3 | reverse-alternating | 1 | `-1|0` | `1|-1` | `0|4` | `0|1` | `0|1` | true |
| G3 | rotation-0 | 0 | `0|-1` | `-1|1` | `4|0` | `1|0` | `1|0` | true |
| G3 | rotation-1 | 0 | `0|-1` | `5|1` | `0|0` | `0|0` | `0|0` | true |
| G3 | rotation-2 | 1 | `-1|0` | `1|-1` | `0|4` | `0|1` | `0|1` | true |
| G3 | rotation-3 | 0 | `0|-1` | `3|1` | `0|0` | `0|0` | `0|0` | true |
| G3 | rotation-4 | 1 | `-1|0` | `1|-1` | `0|4` | `0|1` | `0|1` | true |
| G3 | rotation-5 | 1 | `-1|0` | `1|3` | `0|0` | `0|0` | `0|0` | true |

The substrate law is unchanged. This diagnostic does not repair GATE v1, advance PX2, or unblock PX3.
