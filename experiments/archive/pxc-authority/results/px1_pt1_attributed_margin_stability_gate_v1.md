# PX1-PT1 attributed-margin stability GATE v1

Outcome: **POSITIVE** (`36/36` cells).

| stratum | scenario | train branch | train outlet | train trace arrival/fire | train local return | resistance | held-out branch/outlet | held-out trace arrival/fire | held-out local return/effect | post-gap effect | source refire train/held/post | quiescent train/held/post | replay | pass |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| S0 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `17|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S0 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|17` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S0 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S0 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S0 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S0 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `17|17` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |
| S1 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `17|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S1 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|17` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S1 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S1 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S1 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S1 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `17|17` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |
| S2 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `18|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S2 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|18` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S2 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S2 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S2 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S2 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `18|18` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |
| S3 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `18|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S3 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|18` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S3 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S3 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S3 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S3 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `18|18` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |
| S4 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `16|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S4 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|16` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S4 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S4 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S4 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S4 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `16|16` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |
| S5 | support-a | `8|0` | `8|0` | `16|8/8|0` | `8|0` | `16|0` | `1|1/1|0` | `2|1/1|0` | `1|0/1|0` | `1|0` | `0/0/0` | `true/true/true` | true | true |
| S5 | support-b | `0|8` | `0|8` | `8|16/0|8` | `0|8` | `0|16` | `1|1/0|1` | `1|2/0|1` | `0|1/0|1` | `0|1` | `0/0/0` | `true/true/true` | true | true |
| S5 | no-support | `0|0` | `0|0` | `0|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S5 | blocked-return | `8|0` | `1|0` | `1|0/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S5 | return-without-effect | `0|0` | `0|0` | `8|8/0|0` | `0|0` | `0|0` | `1|1/0|0` | `0|0/0|0` | `0|0/0|0` | `0|0` | `0/0/0` | `true/true/true` | true | true |
| S5 | joint | `8|8` | `8|8` | `16|16/8|8` | `8|8` | `16|16` | `1|1/1|1` | `2|2/1|1` | `1|1/1|1` | `1|1` | `0/0/0` | `true/true/true` | true | true |

Every physical stage is serialized separately. PX0 changed: `false`. PX1 authoritative: `false`. Definitive evidence executed: `false`.
