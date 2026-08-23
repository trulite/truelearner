# PX7 physical arrival initiation development GATE result

Verdict: **PASS DEVELOPMENT GATE**. Authority remains absent; this was not a definitive matrix.

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. No PROBE or MICRO evidence cell was rerun.

| cell | family | coupling/resistance | held-out source/execution/boundary | follow-up source/execution/boundary | crossings | background | quiescent | duplicate | work | bytes before/after | result |
|---|---|---|---|---|---|---:|:---:|:---:|---:|---|:---:|
| M0-ordinary | hardened | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M1-mirrored | hardened | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M2-reversed-allocation | hardened | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M3-reversed-insertion-load4 | hardened | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 392 | 448/576 | PASS |
| M4-combined-load12 | hardened | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 972 | 832/960 | PASS |
| M7-post-gap | hardened | 2/6 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 642 | 640/768 | PASS |
| M5-novel-locus | hardened | 2/8 | 1/0/0 | 1/1/1 | 0/1 | 0 | true | true | 262 | 256/512 | PASS |
| M6-late-return | hardened | 1/1 | 1/0/0 | 0/0/0 | 0/0 | 0 | true | true | 171 | 256/896 | PASS |
| P-learned-return | compact-control | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 145 | 208/272 | PASS |
| P-unreturned | compact-control | 1/1 | 1/0/0 | 0/0/0 | 0/0 | 0 | true | true | 92 | 208/528 | PASS |
| P-subthreshold | compact-control | 2/9 | 0/0/0 | 0/0/0 | 0/0 | 0 | true | true | 124 | 208/272 | PASS |
| P-absent | compact-control | 2/9 | 0/0/0 | 0/0/0 | 0/0 | 0 | true | true | 120 | 208/272 | PASS |

Organism-visible execution used only the frozen physical laws and actual CELL/ARROW/SPIKE state. This result can support only a development-readiness unchanged-port handoff.
