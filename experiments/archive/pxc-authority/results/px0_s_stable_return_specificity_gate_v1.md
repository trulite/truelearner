# PX0-S stable return specificity GATE v1

Outcome: **PX0-S-A — STABLE RETURN SPECIFICITY POSITIVE**.

| arm | stable route | sparse route | stable returns | sparse returns | devices | stable R | sparse final R | stable effect | sparse effect | sparse dies | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| gate-00 | 0 | 1 | 27 | 6 | 4 | 56 | 1 | 1 | 0 | true | true | true |
| gate-01 | 1 | 0 | 31 | 6 | 4 | 61 | 1 | 1 | 0 | true | true | true |
| gate-02 | 2 | 0 | 35 | 7 | 4 | 65 | 1 | 1 | 0 | true | true | true |
| gate-03 | 0 | 2 | 39 | 8 | 4 | 68 | 0 | 1 | 0 | true | true | true |
| gate-04 | 1 | 2 | 43 | 8 | 4 | 71 | 1 | 1 | 0 | true | true | true |
| gate-05 | 2 | 1 | 27 | 6 | 4 | 43 | 1 | 1 | 0 | true | true | true |
| gate-06 | 0 | 1 | 31 | 5 | 4 | 46 | 1 | 1 | 0 | true | true | true |
| gate-07 | 1 | 0 | 35 | 6 | 4 | 48 | 1 | 1 | 0 | true | true | true |
| gate-08 | 2 | 0 | 39 | 7 | 4 | 49 | 0 | 1 | 0 | true | true | true |
| gate-09 | 0 | 2 | 43 | 8 | 4 | 50 | 1 | 1 | 0 | true | true | true |
| gate-10 | 1 | 2 | 27 | 6 | 4 | 30 | 1 | 1 | 0 | true | true | true |
| gate-11 | 2 | 1 | 35 | 6 | 4 | 34 | 0 | 1 | 0 | true | true | true |

## Dense-path and absent-return controls

| arm | route 0 returns | route 1 returns | route 0 effect | route 1 effect | simultaneous | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|
| recurring:recurring-route-0-direct | 24 | 19 | 1 | 1 | 0 | true | true |
| recurring:recurring-route-1-mirrored | 19 | 24 | 1 | 1 | 0 | true | true |
| absent:absent-return-direct | 0 | 0 | 0 | 0 | 0 | true | true |
| absent:absent-return-mirror | 0 | 0 | 0 | 0 | 0 | true | true |
| switch:stability-switch-continuous | 96 | 329 | 0 | 1 | 0 | true | true |
