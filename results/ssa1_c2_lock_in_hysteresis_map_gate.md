# SSA1-C2 lock-in / hysteresis map GATE

- Classification: **A — finite reversal barrier**
- B-only subclassification: **absorbing for B-only same-class counterexperience**
- First non-responsive edge: `credit edge`
- Source invariant: `true`
- Frozen count capacity respected: `true`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Seed 1730000000

- Early productive route: `0`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[6,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[8,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[10,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[12,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[14,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[16,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[24,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[32,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[870,59162]`
- H=64: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[64,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[192,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[5430,54762]`

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

## Seed 1740000001

- Early productive route: `1`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[6,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[8,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[10,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[12,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[14,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[16,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[24,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[32,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[870,59162]`
- H=64: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[64,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[192,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[5430,54762]`

### Physical-support map at H=192

- S=0: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=1: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=2: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=3: B executions `10000`, B observations `10000`, live `[1, 4]`, edge `credit edge`

### Disuse map at H=192

- T=0: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=16: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=64: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=256: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=1024: after-pressure live `[0, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`

## Seed 1750000002

- Early productive route: `0`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[6,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[8,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[10,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[12,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[14,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[16,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[24,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[32,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[870,59162]`
- H=64: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[64,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[192,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[5430,54762]`

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

## Seed 1760000003

- Early productive route: `1`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[6,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[8,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[10,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[12,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[14,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[16,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[24,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[32,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[870,59162]`
- H=64: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[64,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[192,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[5430,54762]`

### Physical-support map at H=192

- S=0: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=1: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=2: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=3: B executions `10000`, B observations `10000`, live `[1, 4]`, edge `credit edge`

### Disuse map at H=192

- T=0: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=16: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=64: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=256: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=1024: after-pressure live `[0, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`

## Seed 1770000004

- Early productive route: `0`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[1, 4]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[6,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[8,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[10,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[12,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[14,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[16,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[24,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[true, false]`, margins `[32,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[1, 4]`, A/B evidence `[870,59162]`
- H=64: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[64,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[4, 1]`, M6 eligible `[true, false]`, margins `[192,0]`; E0@60000 live `[4, 1]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[1, 4]`, A/B evidence `[5430,54762]`

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

## Seed 1780000005

- Early productive route: `1`
- Finite barrier: `Some((PairedChangedWorld, 64, 10000))`
- B-only absorbing invariant: `true`
- Forgetting-only reopening: `false`
- Duplicate exact: `true`
- Controls passed: `true`

### Maturation/evidence map

- H=0: boundary live `[0, 0]`, M6 eligible `[false, false]`, margins `[0,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `3`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[34,19966]`
- H=2: boundary live `[4, 4]`, M6 eligible `[false, false]`, margins `[2,0]`; E0@10000 live `[4, 1]`, B obs `10000`, B M5 `+9997/-0`, abstentions `5`, edge `none`, reversed `true`; E1 first reversal `Some(64)`, final live `[4, 1]`, A/B evidence `[43,19959]`
- H=4: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[4,0]`; E0@10000 live `[4, 4]`, B obs `10000`, B M5 `+0/-0`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[92,19912]`
- H=6: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[6,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[133,19873]`
- H=8: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[8,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[186,19822]`
- H=10: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[10,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[243,19767]`
- H=12: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[12,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[300,19712]`
- H=14: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[14,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[357,19657]`
- H=16: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[16,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[414,59602]`
- H=24: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[24,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[642,19382]`
- H=32: boundary live `[4, 4]`, M6 eligible `[false, true]`, margins `[32,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(1024)`, final live `[4, 1]`, A/B evidence `[870,59162]`
- H=64: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[64,0]`; E0@10000 live `[1, 4]`, B obs `10000`, B M5 `+0/-3`, abstentions `10000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[1782,18282]`
- H=192: boundary live `[1, 4]`, M6 eligible `[false, true]`, margins `[192,0]`; E0@60000 live `[1, 4]`, B obs `60000`, B M5 `+0/-3`, abstentions `60000`, edge `credit edge`, reversed `false`; E1 first reversal `Some(10000)`, final live `[4, 1]`, A/B evidence `[5430,54762]`

### Physical-support map at H=192

- S=0: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=1: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=2: B executions `0`, B observations `0`, live `[1, 4]`, edge `execution edge`
- S=3: B executions `10000`, B observations `10000`, live `[1, 4]`, edge `credit edge`

### Disuse map at H=192

- T=0: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=16: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=64: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=256: after-pressure live `[1, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
- T=1024: after-pressure live `[0, 4]`, M6 evidence `[0,192]`, final live `[1, 4]`, forgetting-only `false`
