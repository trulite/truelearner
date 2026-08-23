# PX1-PT0 physical participation trace MICRO

Outcome: **POSITIVE**.

- rows: `16/16` passed
- frozen PX0 law changed: `false`
- PX1 authoritative: `false`
- definitive evidence executed: `false`

| scenario | transfer | active | expected mature | resistance | held-out effects | return | quiescent | replay | pass |
|---|:---:|---:|---:|---:|---:|---:|:---:|:---:|:---:|
| in-window-a | false | true|false|false | true|false|false | 5|2|2 | 1|0|0 | 1|1|1 | true | true | true |
| in-window-a | true | true|false|false | true|false|false | 5|2|2 | 1|0|0 | 1|1|1 | true | true | true |
| late-a | false | true|false|false | false|false|false | 1|2|2 | 0|0|0 | 1|1|1 | true | true | true |
| late-a | true | true|false|false | false|false|false | 1|2|2 | 0|0|0 | 1|1|1 | true | true | true |
| no-participation | false | false|false|false | false|false|false | 2|2|2 | 0|0|0 | 1|1|1 | true | true | true |
| no-participation | true | false|false|false | false|false|false | 2|2|2 | 0|0|0 | 1|1|1 | true | true | true |
| swap-b | false | false|true|false | false|true|false | 2|5|2 | 0|1|0 | 1|1|1 | true | true | true |
| swap-b | true | false|true|false | false|true|false | 2|5|2 | 0|1|0 | 1|1|1 | true | true | true |
| joint | false | true|true|false | true|true|false | 5|5|2 | 1|1|0 | 1|1|1 | true | true | true |
| joint | true | true|true|false | true|true|false | 5|5|2 | 1|1|0 | 1|1|1 | true | true | true |
| no-return-a | false | true|false|false | false|false|false | 2|2|2 | 0|0|0 | 0|0|0 | true | true | true |
| no-return-a | true | true|false|false | false|false|false | 2|2|2 | 0|0|0 | 0|0|0 | true | true | true |
| boundary-inside-a | false | true|false|false | true|false|false | 5|2|2 | 1|0|0 | 1|1|1 | true | true | true |
| boundary-inside-a | true | true|false|false | true|false|false | 5|2|2 | 1|0|0 | 1|1|1 | true | true | true |
| boundary-outside-a | false | true|false|false | false|false|false | 1|2|2 | 0|0|0 | 1|1|1 | true | true | true |
| boundary-outside-a | true | true|false|false | false|false|false | 1|2|2 | 0|0|0 | 1|1|1 | true | true | true |

The return is physically identical at every branch. Only recent branch activity may create an eligibility window; no provenance value enters the substrate.
