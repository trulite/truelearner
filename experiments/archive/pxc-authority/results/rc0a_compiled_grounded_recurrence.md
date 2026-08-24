# RC0a compiled grounded recurrence

Compatibility gate: **PASS**. Claim eligible: `true`.

Generic per-step excess slope: `515212800/77281920 (6.666667)`; compiled slope: `0/77281920 (0.000000)`; 80% reduction gate: `true`.

| Arm | Correct | Total | Work |
|---|---:|---:|---:|
| concrete-reference | 768 | 768 | 7912448 |
| generic-grounded | 768 | 768 | 8653824 |
| compiled-grounded | 768 | 768 | 8192000 |
| compiled-changed-bindings | 768 | 768 | 8558999 |
| invalidated-transition | 768 | 768 | 8655192 |
| subthreshold-evidence | 768 | 768 | 8654592 |
| shuffled-consolidation-evidence | 768 | 768 | 8655069 |
| compiled-no-bindings | 0 | 768 | 267264 |

## Gates

- `frozen-ancestry-and-rp0a-parity`: PASS
- `earned-three-episode-compilation`: PASS
- `role-relative-persistent-structure`: PASS
- `compiled-fresh-and-changed-bindings`: PASS
- `compiled-local-dispatch-only`: PASS
- `lower-effects-preserved`: PASS
- `invalidation-resumes-generic`: PASS
- `subthreshold-does-not-compile`: PASS
- `shuffled-evidence-cannot-fire`: PASS
- `bindings-remain-necessary`: PASS
- `state-isolation-and-determinism`: PASS
- `per-step-slope-reduction-at-least-80-percent`: PASS
- `rc0b-excluded-source-audit`: PASS

Workspaces destroyed: `1148245/1148245`; maximum live per independent cell: `2`; parallel cells: `8`.
