# SSA1-S3 structural-commitment causality PROBE

- Classification: **D — P8 readout, not established causal boundary**
- Threshold-causal cells: `0/2`
- Deallocation-causal cells: `0/2`
- Post-commitment inert cells: `2/2`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Cell 2030000000 / 1:2:7:1

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `1529`, A deallocation `12509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 1529 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 12509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 1530 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 12510 | `[1, 4]` | `[2, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2030000001 / 1:2:7:1

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `1529`, A deallocation `12509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 1529 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 12509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 1530 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 12510 | `[1, 4]` | `[2, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |
