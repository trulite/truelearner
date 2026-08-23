# SSA1-S exposure bias map PROBE

- Classification: **C — exposure-insensitive allocation**
- Environmental exposure varied: `true`
- Phase map monotonic: `true`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Seed 1900000000

- Incumbent physical side: `0`
- Side -> route: `[0, 1]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 4 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `true`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `true`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`

## Seed 1900000001

- Incumbent physical side: `1`
- Side -> route: `[1, 0]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 4 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `true`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `true`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
