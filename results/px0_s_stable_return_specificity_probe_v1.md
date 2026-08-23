# PX0-S stable return specificity PROBE v1

Outcome: **TARGET NOT MET**.

| arm | stable route | sparse route | stable returns | sparse returns | devices | stable R | sparse final R | stable effect | sparse effect | sparse dies | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| stable-route-1-direct | 1 | 0 | 35 | 6 | 4 | 65 | 1 | 1 | 0 | true | true | false |
| stable-route-0-mirrored | 0 | 1 | 35 | 6 | 4 | 65 | 1 | 1 | 0 | true | true | false |

## Recurring dense-path controls

| arm | route 0 returns | route 1 returns | route 0 effect | route 1 effect | simultaneous | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|
| recurring-route-0-direct | 24 | 19 | 1 | 1 | 0 | true | false |
| recurring-route-1-mirrored | 19 | 24 | 1 | 1 | 0 | true | false |
