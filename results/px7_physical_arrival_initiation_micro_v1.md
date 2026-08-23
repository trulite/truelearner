# PX7 physical arrival initiation MICRO result

Verdict: **PASS**.

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. The PROBE was not rerun and no new mechanism was used.

| case | coupling/resistance | held-out source/execution/boundary | follow-up source/execution/boundary | crossings initial/follow-up | background | quiescent | duplicate | work | bytes before/after | result |
|---|---|---|---|---|---:|:---:|:---:|---:|---|:---:|
| M0-ordinary | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M1-mirrored | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M2-reversed-allocation | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 222 | 256/384 | PASS |
| M3-reversed-insertion-load4 | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 392 | 448/576 | PASS |
| M4-combined-load12 | 2/9 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 972 | 832/960 | PASS |
| M5-novel-locus | 2/8 | 1/0/0 | 1/1/1 | 0/1 | 0 | true | true | 262 | 256/512 | PASS |
| M6-late-return | 1/1 | 1/0/0 | 0/0/0 | 0/0 | 0 | true | true | 171 | 256/896 | PASS |
| M7-post-gap | 2/6 | 1/1/1 | 0/0/0 | 1/0 | 0 | true | true | 642 | 640/768 | PASS |

All supplied schedules were fixed anonymous physical arrivals. Organism-visible execution used only frozen CELL/ARROW/SPIKE state and local physical timing.
