# DS7 cumulative learned plasticity-allocation GATE v3 result

Verdict: **PASS (DEVELOPMENT GATE)**.

Protocol: `ds7-cumulative-plasticity-allocation-gate-v3`.

All six fresh seeds passed each load:

| load | cells | acquisition | held-out | admitted P/N | always open | reduction | entry resistance | final resistance | removed | stale | repaired | shuffled repair blocked | controls | M4 |
|---:|---:|:---:|---:|---:|---:|---:|---:|---:|:---:|:---:|---:|:---:|:---:|:---:|
| 8 | 6 | yes | 32/32 | 4/1 | 12 | 58.33% | 105/105 | 0/0 | yes/yes | yes | 32/32 | yes | yes | yes |
| 32 | 6 | yes | 32/32 | 4/4 | 36 | 77.78% | 105/105 | 0/0 | yes/yes | yes | 32/32 | yes | yes | yes |
| 128 | 6 | yes | 32/32 | 4/16 | 132 | 84.85% | 105/105 | 0/0 | yes/yes | yes | 32/32 | yes | yes | yes |

Additional conjunctive checks passed in all 18 cells:

- learned-value shuffle reversed productive/distractor admission;
- unused distractor proposals retained less allocation than always-open;
- recurrent route proposals, encounter prototypes, and value remained live;
- no-coactivity, outside-radius, inactive-feedback, fresh-identity, and
  absolute-layout controls;
- source and forbidden-information audit;
- unchanged authoritative M4 exact lifetime vector `[1, 3, 6, 13, 27]`;
- complete report duplicate exactness.

The exact 424-event withholding phase did not change the M4 lifecycle. It
supplied 105 pressure boundaries to structures whose learned history had built
resistance 105. Both correct and shuffled branches physically deallocated the
withheld edge. Only learned encounter value then distinguished repair: the
correct allocator reopened the local variation and restored route execution;
the shuffled allocator did not.

This result establishes DS7 cumulative development readiness only. M4 remains
authoritative and M5 is absent pending a separate one-shot definitive matrix.

