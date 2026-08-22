# DS7 cumulative plasticity-allocation GATE v2 result

Verdict: **FAIL (DEVELOPMENT GATE; IMMUTABLE)**.

Protocol: `ds7-cumulative-plasticity-allocation-gate-v2`.

All six seeds produced the same result at each load:

| load | cells | acquisition | held-out | admitted P/N | always open | reduction | shuffled value | retained economy | controls | M4 | result |
|---:|---:|:---:|---:|---:|---:|---:|:---:|:---:|:---:|:---:|:---:|
| 8 | 6 | yes | 32/32 | 4/1 | 12 | 58.33% | yes | yes | yes | yes | FAIL |
| 32 | 6 | yes | 32/32 | 4/4 | 36 | 77.78% | yes | yes | yes | yes | FAIL |
| 128 | 6 | yes | 32/32 | 4/16 | 132 | 84.85% | yes | yes | yes | yes | FAIL |

The uniform failed branch was:

```text
joint correct/shuffled withheld-edge removal     false
joint stale execution blocked                    false
correct branch reported 32/32 after repair       true
shuffled first repair blocked                    false
```

Because the v2 report exposed only joint removal/stale flags, it does not
identify which matched branch retained or recreated the withheld edge. A
separate diagnostic must split the correct and shuffled lifecycles before any
retry is preregistered.

The 18-cell report was duplicate-exact. M4 remains authoritative; M5 is absent.

