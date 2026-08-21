# RP0b reflected-program economics

RP0b.1 technical gate: **FAIL**.

RP0b.2 amortization: **NOT EVALUATED**.

Reconstruction parity: `true`; duplicate deterministic: `true`.

| Depth | Concrete runtime | Reflected runtime | Delta | Oracle |
|---:|---:|---:|---:|---:|
| 5 | 27648 | 50944 | 23296 | 27648 |
| 8 | 44928 | 68224 | 23296 | 44928 |
| 16 | 102272 | 125568 | 23296 | 102272 |
| 32 | 266112 | 289408 | 23296 | 266112 |
| 64 | 790400 | 813696 | 23296 | 790400 |
| 128 | 2625408 | 2648704 | 23296 | 2625408 |

## Gates

- `frozen-ancestry`: PASS
- `rp0a-reconstruction-parity`: PASS
- `identical-correct-behavior`: PASS
- `learned-invocation-boundary`: PASS
- `read-only-determinism`: PASS
- `accounting-and-lifecycle`: PASS
- `reflected-runtime-below-concrete`: FAIL
- `conditional-amortization`: NOT_EVALUATED

Absolute scaling improvement: `true`; fractional scaling improvement: `true`.

Workspaces destroyed: `1150205/1150205`; maximum live: `2`.
