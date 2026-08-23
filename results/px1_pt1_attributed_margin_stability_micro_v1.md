# PX1-PT1 attributed-margin stability MICRO v1

Outcome: **NEGATIVE** (`8/12` cells).

| scenario | transfer | train branch | train outlet | train trace arrival/fire | train local return | resistance | held-out branch/outlet | held-out trace arrival/fire | held-out local return/effect | post-gap effect | source refire train/held/post | quiescent train/held/post | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| support-a | false | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `17|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| support-a | true | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `17|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| support-b | false | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|17` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| support-b | true | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|17` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| no-support | false | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| no-support | true | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| blocked-return | false | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | false |
| blocked-return | true | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | false |
| return-without-effect | false | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| return-without-effect | true | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| joint | false | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `17|17` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | false |
| joint | true | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `17|17` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | false |

Every physical stage is serialized separately. PX0 changed: `false`. PX1 authoritative: `false`. Definitive evidence executed: `false`.
