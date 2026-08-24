# RG0a reflected grounding

Functional gate: **PASS**. Claim eligible: `true`.

Mode: `definitive`; reconstruction parity: `true`; duplicate deterministic: `true`.

| Arm | Correct | Total | Work |
|---|---:|---:|---:|
| concrete | 768 | 768 | 7912448 |
| grounded-reflected | 768 | 768 | 8653824 |
| no-bindings | 0 | 768 | 92160 |
| shuffled-bindings | 0 | 768 | 1162431 |
| activity-only-grounding | 0 | 768 | 22272 |
| random-reflected-program | 0 | 768 | 15160416 |
| shuffled-terminal-program | 0 | 768 | 106464 |
| oracle-binding-program | 768 | 768 | 8243840 |

## Gates

- `frozen-ancestry`: PASS
- `rp0a-reconstruction-parity`: PASS
- `identical-branch-state`: PASS
- `fresh-anonymous-grounding`: PASS
- `grounded-functional-substitution`: PASS
- `downward-causal-path`: PASS
- `no-lower-program-fallback`: PASS
- `state-isolation`: PASS
- `necessary-bindings`: PASS
- `necessary-structural-provenance`: PASS
- `necessary-learned-topology-credit`: PASS
- `grounding-upper-bound`: PASS
- `opacity-audit`: PASS
- `accounting-and-lifecycle`: PASS

Workspaces destroyed: `5583213/5583213`; maximum live per independent cell: `2`; parallel cells: `8`.
