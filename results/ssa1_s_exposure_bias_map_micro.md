# SSA1-S exposure bias map MICRO

- Classification: **E — scientific ambiguity**
- Environmental exposure varied: `true`
- Phase map monotonic: `false`
- Frozen parent exact: `true`
- Development-valid: `false`
- Definitive claim eligible: `false`

## Seed 1910000000

- Incumbent physical side: `0`
- Side -> route: `[0, 1]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (2, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (8, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `false`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `false`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16000,2000]`, M6 margins `[7,2000]`, M5 scores `[-2362,322]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12000,6000]`, M6 margins `[6,6000]`, M5 scores `[-3188,1593]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6000,12000]`, M6 margins `[2,12000]`, M5 scores `[-1708,3417]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2000,16000]`, M6 margins `[21,16000]`, M5 scores `[-250,2005]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16002,2000]`, M6 margins `[9,2000]`, M5 scores `[-2661,345]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14402,3600]`, M6 margins `[1,3600]`, M5 scores `[-5337,1432]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12002,6000]`, M6 margins `[8,6000]`, M5 scores `[-381,169]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9002,9000]`, M6 margins `[0,9000]`, M5 scores `[-4495,4496]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6002,12000]`, M6 margins `[4,12000]`, M5 scores `[-1123,2247]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3602,14400]`, M6 margins `[1,14400]`, M5 scores `[-1563,6253]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2002,16000]`, M6 margins `[23,16000]`, M5 scores `[-182,1461]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16008,2000]`, M6 margins `[15,2000]`, M5 scores `[-1485,203]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14408,3600]`, M6 margins `[4,3600]`, M5 scores `[-3550,959]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12008,6000]`, M6 margins `[14,6000]`, M5 scores `[13,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9008,9000]`, M6 margins `[0,9000]`, M5 scores `[-3250,3256]`, live `[4, 4]`, class `MIXED`
- H=8 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6008,12000]`, M6 margins `[10,12000]`, M5 scores `[7,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3608,14400]`, M6 margins `[3,14400]`, M5 scores `[-1133,4553]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2008,16000]`, M6 margins `[29,16000]`, M5 scores `[-129,1077]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16032,2000]`, M6 margins `[39,2000]`, M5 scores `[61,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12032,6000]`, M6 margins `[38,6000]`, M5 scores `[37,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6032,12000]`, M6 margins `[34,12000]`, M5 scores `[31,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2032,16000]`, M6 margins `[53,16000]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16128,2000]`, M6 margins `[135,2000]`, M5 scores `[157,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12128,6000]`, M6 margins `[134,6000]`, M5 scores `[133,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6128,12000]`, M6 margins `[130,12000]`, M5 scores `[127,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2128,16000]`, M6 margins `[149,16000]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=2: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17997`
- H=8: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`

## Seed 1920000001

- Incumbent physical side: `1`
- Side -> route: `[1, 0]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (2, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (8, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `false`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `false`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16000,2000]`, M6 margins `[7,2000]`, M5 scores `[-2362,322]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12000,6000]`, M6 margins `[6,6000]`, M5 scores `[-3188,1593]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6000,12000]`, M6 margins `[2,12000]`, M5 scores `[-1708,3417]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2000,16000]`, M6 margins `[21,16000]`, M5 scores `[-250,2005]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16002,2000]`, M6 margins `[9,2000]`, M5 scores `[-2661,345]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14402,3600]`, M6 margins `[1,3600]`, M5 scores `[-5337,1432]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12002,6000]`, M6 margins `[8,6000]`, M5 scores `[-381,169]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9002,9000]`, M6 margins `[0,9000]`, M5 scores `[-4495,4496]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6002,12000]`, M6 margins `[4,12000]`, M5 scores `[-1123,2247]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3602,14400]`, M6 margins `[1,14400]`, M5 scores `[-1563,6253]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2002,16000]`, M6 margins `[23,16000]`, M5 scores `[-182,1461]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16008,2000]`, M6 margins `[15,2000]`, M5 scores `[-1485,203]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14408,3600]`, M6 margins `[4,3600]`, M5 scores `[-3550,959]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12008,6000]`, M6 margins `[14,6000]`, M5 scores `[13,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9008,9000]`, M6 margins `[0,9000]`, M5 scores `[-3250,3256]`, live `[4, 4]`, class `MIXED`
- H=8 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6008,12000]`, M6 margins `[10,12000]`, M5 scores `[7,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3608,14400]`, M6 margins `[3,14400]`, M5 scores `[-1133,4553]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2008,16000]`, M6 margins `[29,16000]`, M5 scores `[-129,1077]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16032,2000]`, M6 margins `[39,2000]`, M5 scores `[61,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12032,6000]`, M6 margins `[38,6000]`, M5 scores `[37,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6032,12000]`, M6 margins `[34,12000]`, M5 scores `[31,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2032,16000]`, M6 margins `[53,16000]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16128,2000]`, M6 margins `[135,2000]`, M5 scores `[157,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12128,6000]`, M6 margins `[134,6000]`, M5 scores `[133,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6128,12000]`, M6 margins `[130,12000]`, M5 scores `[127,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2128,16000]`, M6 margins `[149,16000]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=2: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17997`
- H=8: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`

## Seed 1910000002

- Incumbent physical side: `0`
- Side -> route: `[0, 1]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (2, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (8, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `false`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `false`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16000,2000]`, M6 margins `[7,2000]`, M5 scores `[-2362,322]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12000,6000]`, M6 margins `[6,6000]`, M5 scores `[-3188,1593]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6000,12000]`, M6 margins `[2,12000]`, M5 scores `[-1708,3417]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2000,16000]`, M6 margins `[21,16000]`, M5 scores `[-250,2005]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16002,2000]`, M6 margins `[9,2000]`, M5 scores `[-2661,345]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14402,3600]`, M6 margins `[1,3600]`, M5 scores `[-5337,1432]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12002,6000]`, M6 margins `[8,6000]`, M5 scores `[-381,169]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9002,9000]`, M6 margins `[0,9000]`, M5 scores `[-4495,4496]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6002,12000]`, M6 margins `[4,12000]`, M5 scores `[-1123,2247]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3602,14400]`, M6 margins `[1,14400]`, M5 scores `[-1563,6253]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2002,16000]`, M6 margins `[23,16000]`, M5 scores `[-182,1461]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16008,2000]`, M6 margins `[15,2000]`, M5 scores `[-1485,203]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14408,3600]`, M6 margins `[4,3600]`, M5 scores `[-3550,959]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12008,6000]`, M6 margins `[14,6000]`, M5 scores `[13,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9008,9000]`, M6 margins `[0,9000]`, M5 scores `[-3250,3256]`, live `[4, 4]`, class `MIXED`
- H=8 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6008,12000]`, M6 margins `[10,12000]`, M5 scores `[7,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3608,14400]`, M6 margins `[3,14400]`, M5 scores `[-1133,4553]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2008,16000]`, M6 margins `[29,16000]`, M5 scores `[-129,1077]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16032,2000]`, M6 margins `[39,2000]`, M5 scores `[61,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12032,6000]`, M6 margins `[38,6000]`, M5 scores `[37,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6032,12000]`, M6 margins `[34,12000]`, M5 scores `[31,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2032,16000]`, M6 margins `[53,16000]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16128,2000]`, M6 margins `[135,2000]`, M5 scores `[157,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12128,6000]`, M6 margins `[134,6000]`, M5 scores `[133,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6128,12000]`, M6 margins `[130,12000]`, M5 scores `[127,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2128,16000]`, M6 margins `[149,16000]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=2: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17997`
- H=8: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`

## Seed 1920000003

- Incumbent physical side: `1`
- Side -> route: `[1, 0]`
- Reversal thresholds: `[(0, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (2, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (8, Some(OpportunityRatio { alternative: 1, incumbent: 8 })), (32, None), (128, None)]`
- Exposure monotonic: `true`
- Allocation monotonic: `false`
- Maturity monotonic: `true`
- Stale blocked: `true`
- Post-closure inert: `true`
- Anti-adaptation: `true`
- Controls passed: `false`

### Opportunity -> evidence -> allocation

- H=0 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16000,2000]`, M6 margins `[7,2000]`, M5 scores `[-2362,322]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14400,3600]`, M6 margins `[1,3600]`, M5 scores `[-5376,1410]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12000,6000]`, M6 margins `[6,6000]`, M5 scores `[-3188,1593]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9000,9000]`, M6 margins `[0,9000]`, M5 scores `[-4634,4635]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6000,12000]`, M6 margins `[2,12000]`, M5 scores `[-1708,3417]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3600,14400]`, M6 margins `[1,14400]`, M5 scores `[-1425,5701]`, live `[1, 4]`, class `ALTERNATIVE`
- H=0 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2000,16000]`, M6 margins `[21,16000]`, M5 scores `[-250,2005]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16002,2000]`, M6 margins `[9,2000]`, M5 scores `[-2661,345]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14402,3600]`, M6 margins `[1,3600]`, M5 scores `[-5337,1432]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12002,6000]`, M6 margins `[8,6000]`, M5 scores `[-381,169]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9002,9000]`, M6 margins `[0,9000]`, M5 scores `[-4495,4496]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6002,12000]`, M6 margins `[4,12000]`, M5 scores `[-1123,2247]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3602,14400]`, M6 margins `[1,14400]`, M5 scores `[-1563,6253]`, live `[1, 4]`, class `ALTERNATIVE`
- H=2 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2002,16000]`, M6 margins `[23,16000]`, M5 scores `[-182,1461]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16008,2000]`, M6 margins `[15,2000]`, M5 scores `[-1485,203]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14408,3600]`, M6 margins `[4,3600]`, M5 scores `[-3550,959]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12008,6000]`, M6 margins `[14,6000]`, M5 scores `[13,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9008,9000]`, M6 margins `[0,9000]`, M5 scores `[-3250,3256]`, live `[4, 4]`, class `MIXED`
- H=8 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6008,12000]`, M6 margins `[10,12000]`, M5 scores `[7,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=8 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3608,14400]`, M6 margins `[3,14400]`, M5 scores `[-1133,4553]`, live `[1, 4]`, class `ALTERNATIVE`
- H=8 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2008,16000]`, M6 margins `[29,16000]`, M5 scores `[-129,1077]`, live `[1, 4]`, class `ALTERNATIVE`
- H=32 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16032,2000]`, M6 margins `[39,2000]`, M5 scores `[61,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14432,3600]`, M6 margins `[28,3600]`, M5 scores `[45,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12032,6000]`, M6 margins `[38,6000]`, M5 scores `[37,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9032,9000]`, M6 margins `[12,9000]`, M5 scores `[33,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6032,12000]`, M6 margins `[34,12000]`, M5 scores `[31,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3632,14400]`, M6 margins `[27,14400]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=32 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2032,16000]`, M6 margins `[53,16000]`, M5 scores `[30,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:8: scheduled `[16000, 2000]`, executions `[16000, 2000]`, M6 observations `[16128,2000]`, M6 margins `[135,2000]`, M5 scores `[157,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:4: scheduled `[14400, 3600]`, executions `[14400, 3600]`, M6 observations `[14528,3600]`, M6 margins `[124,3600]`, M5 scores `[141,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:2: scheduled `[12000, 6000]`, executions `[12000, 6000]`, M6 observations `[12128,6000]`, M6 margins `[134,6000]`, M5 scores `[133,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=1:1: scheduled `[9000, 9000]`, executions `[9000, 9000]`, M6 observations `[9128,9000]`, M6 margins `[108,9000]`, M5 scores `[129,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=2:1: scheduled `[6000, 12000]`, executions `[6000, 12000]`, M6 observations `[6128,12000]`, M6 margins `[130,12000]`, M5 scores `[127,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=4:1: scheduled `[3600, 14400]`, executions `[3600, 14400]`, M6 observations `[3728,14400]`, M6 margins `[123,14400]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`
- H=128 B:A=8:1: scheduled `[2000, 16000]`, executions `[2000, 16000]`, M6 observations `[2128,16000]`, M6 margins `[149,16000]`, M5 scores `[126,-3]`, live `[4, 1]`, class `INCUMBENT_LOCK`

### Equal-consequence controls

- H=0: executions `[9000, 9000]`, live `[4, 4]`, class `MIXED`, abstentions `17999`
- H=2: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17997`
- H=8: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=32: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
- H=128: executions `[9000, 9000]`, live `[4, 1]`, class `INCUMBENT_LOCK`, abstentions `17996`
