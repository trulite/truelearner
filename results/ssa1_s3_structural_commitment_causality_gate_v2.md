# SSA1-S3 structural-commitment causality GATE

- Classification: **D — P8 readout, not established causal boundary**
- Threshold-causal cells: `0/18`
- Deallocation-causal cells: `0/18`
- Post-commitment inert cells: `18/18`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Cell 2140000000 / 1:2:7:1

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

## Cell 2140000000 / 1:2:13:43

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000000 / 1:2:17:1

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000001 / 1:2:7:1

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

## Cell 2140000001 / 1:2:13:43

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000001 / 1:2:17:1

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000002 / 1:2:7:1

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

## Cell 2140000002 / 1:2:13:43

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000002 / 1:2:17:1

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000003 / 1:2:7:1

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

## Cell 2140000003 / 1:2:13:43

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000003 / 1:2:17:1

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000004 / 1:2:7:1

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

## Cell 2140000004 / 1:2:13:43

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000004 / 1:2:17:1

- Side -> route: `[0, 1]`; incumbent side `0`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000005 / 1:2:7:1

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

## Cell 2140000005 / 1:2:13:43

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `429`, A deallocation `3509`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 429 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3509 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 430 | `[4, 4]` | `[4, 4]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3510 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |

## Cell 2140000005 / 1:2:17:1

- Side -> route: `[1, 0]`; incumbent side `1`
- Reference transitions: B threshold `432`, A deallocation `3533`
- Prefix exact: `true`
- T1 threshold causal: `false`
- T2 deallocation causal: `false`
- T3 post-commitment inert: `true`
- Controls passed: `true`

| arm | episode | before | after | recurrences | transition changed | final | class |
|---|---:|---|---|---:|---|---|---|
| T0-reference | 0 | `[0, 0]` | `[0, 0]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T1-threshold-block | 432 | `[4, 1]` | `[4, 1]` | 0 | true | `[1, 4]` | ALTERNATIVE |
| T2-deallocation-protection | 3533 | `[4, 4]` | `[1, 4]` | 4 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-threshold-block | 433 | `[4, 4]` | `[4, 1]` | 0 | false | `[1, 4]` | ALTERNATIVE |
| T3-post-deallocation-recurrence | 3534 | `[1, 4]` | `[1, 4]` | 1 | false | `[2, 4]` | ALTERNATIVE |
