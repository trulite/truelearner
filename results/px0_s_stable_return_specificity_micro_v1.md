# PX0-S stable return specificity MICRO v1

Outcome: **PX0-S-A — STABLE RETURN SPECIFICITY POSITIVE**.

| arm | stable route | sparse route | stable returns | sparse returns | devices | stable R | sparse final R | stable effect | sparse effect | sparse dies | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| micro-phase-2-direct | 2 | 0 | 27 | 6 | 4 | 56 | 1 | 1 | 0 | true | true | true |
| micro-phase-3-mirror | 0 | 2 | 43 | 8 | 4 | 71 | 1 | 1 | 0 | true | true | true |
| micro-route-1-direct | 1 | 2 | 31 | 6 | 4 | 61 | 1 | 1 | 0 | true | true | true |
| micro-route-2-mirror | 2 | 1 | 39 | 8 | 4 | 57 | 1 | 1 | 0 | true | true | true |

## Dense-path and absent-return controls

| arm | route 0 returns | route 1 returns | route 0 effect | route 1 effect | simultaneous | replay | pass |
|---|---:|---:|---:|---:|---:|---:|---:|
| recurring:recurring-route-0-direct | 24 | 19 | 1 | 1 | 0 | true | true |
| recurring:recurring-route-1-mirrored | 19 | 24 | 1 | 1 | 0 | true | true |
| absent:absent-return-direct | 0 | 0 | 0 | 0 | 0 | true | true |
| absent:absent-return-mirror | 0 | 0 | 0 | 0 | 0 | true | true |
| switch:stability-switch-continuous | 96 | 329 | 0 | 1 | 0 | true | true |
