# PX6 physical consequence-credit no-new-mechanism PROBE v1

Outcome: **1/6 worlds passed**.

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. Active substrate SHA-256: `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

Total measured work: `16095`. Aggregate per-world persistent storage: `11808` bytes.

| world | traversal | downstream | return arrivals | resistance after | live | held-out | work | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|
| left | 8|0 | 8|8 | 8|8 | 17|0 | true|false | 1|0 | 2966 | true | false |
| right | 0|8 | 8|8 | 8|8 | 0|17 | false|true | 0|1 | 2966 | true | false |
| both | 8|8 | 8|8 | 8|8 | 17|17 | true|true | 1|1 | 3745 | true | false |
| correlation | 0|0 | 8|8 | 8|8 | 0|0 | false|false | 0|0 | 2306 | true | true |
| crossed-return | 8|0 | 2|8 | 0|2 | 0|0 | false|false | 0|0 | 2108 | true | false |
| no-return | 8|0 | 2|8 | 0|0 | 0|0 | false|false | 0|0 | 2004 | true | false |

The unchanged PX0--PX2 law alone produced differential persistence. Downstream occurrence and return without local participation did not preserve either weak arrow. No additional organism mechanism executed.
