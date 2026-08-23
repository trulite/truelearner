# SSA1-C2 lock-in / hysteresis map PROBE

- Classification: **B — moving but uncrossed barrier**
- B-only subclassification: **not absorbing**
- First non-responsive edge: `none`
- Source invariant: `true`
- Frozen count capacity respected: `true`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Seed 1700000000

- Early productive route: `0`
- Finite barrier: `None`
- B-only absorbing invariant: `false`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(4)`, final live `[1, 4]`, A/B evidence `[34,19966]`
- H=8: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[8,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(0)`, final live `[1, 4]`, A/B evidence `[186,19822]`
- H=16: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[16,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(0)`, final live `[1, 4]`, A/B evidence `[414,19602]`
- H=32: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[32,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(0)`, final live `[1, 4]`, A/B evidence `[870,19162]`

### Physical-support map at H=192

- S=0: B executions `0`, B observations `0`, live `[4, 1]`, edge `execution edge`
- S=1: B executions `0`, B observations `0`, live `[4, 1]`, edge `execution edge`
- S=2: B executions `0`, B observations `0`, live `[4, 1]`, edge `execution edge`
- S=3: B executions `10000`, B observations `10000`, live `[4, 1]`, edge `credit edge`

### Disuse map at H=192

- T=0: after-pressure live `[4, 1]`, M6 evidence `[192,0]`, final live `[4, 1]`, forgetting-only `false`
- T=16: after-pressure live `[4, 1]`, M6 evidence `[192,0]`, final live `[4, 1]`, forgetting-only `false`
- T=64: after-pressure live `[4, 1]`, M6 evidence `[192,0]`, final live `[4, 1]`, forgetting-only `false`
- T=256: after-pressure live `[4, 1]`, M6 evidence `[192,0]`, final live `[4, 1]`, forgetting-only `false`
- T=1024: after-pressure live `[4, 0]`, M6 evidence `[192,0]`, final live `[4, 1]`, forgetting-only `false`
