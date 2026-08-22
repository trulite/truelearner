# SSA1-C1 adaptation under experience PROBE

- Classification: **C — curriculum prevention only**
- First collapse: `C1 learned reopening after physical counterexperience`
- Frozen parent exact: `true`
- Duplicate exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Seed 1600000000

- Productive route: `0`
- P0 baseline passed: `true`
- P0 lock: live `[4, 1] -> [4, 1]`; changed realization `[1024, 0]`
- C1 counterexperience: obtained `true`; learned change `true`; recovery `false`; persistence `false`
- C1 landscape: live `[4, 1] -> [4, 1] -> [4, 1]`; values `[25, -6] -> [25, -9]`; forced realization `[0, 1024]`
- C2 adaptation frontier: `Some(8)` initial executions
- C2 timing cells:
  - `0`: boundary live `[0, 0]`, final live `[1, 4]`, realized `[34, 990]`, recovered `true`
  - `1`: boundary live `[4, 4]`, final live `[1, 4]`, realized `[37, 987]`, recovered `true`
  - `2`: boundary live `[4, 4]`, final live `[1, 4]`, realized `[41, 983]`, recovered `true`
  - `3`: boundary live `[4, 4]`, final live `[1, 4]`, realized `[44, 980]`, recovered `true`
  - `4`: boundary live `[4, 4]`, final live `[1, 4]`, realized `[88, 936]`, recovered `true`
  - `8`: boundary live `[4, 4]`, final live `[1, 4]`, realized `[202, 822]`, recovered `true`
  - `16`: boundary live `[4, 4]`, final live `[4, 1]`, realized `[1014, 10]`, recovered `false`
  - `32`: boundary live `[4, 4]`, final live `[4, 1]`, realized `[1022, 2]`, recovered `false`
  - `64`: boundary live `[4, 1]`, final live `[4, 1]`, realized `[1024, 0]`, recovered `false`
  - `192`: boundary live `[4, 1]`, final live `[4, 1]`, realized `[1024, 0]`, recovered `false`
- C3 timing-only B executions: `0`; minimum early background: `Some(3)`
- C3 richness: obtained `true`; learned change `true`; recovery `false`; persistence `false`; realization `[0, 1024]`
- Cell controls passed: `true`
