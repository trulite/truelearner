# SSA1-S2 application-history predictor PROBE

- Classification: **B — commitment-state law**
- Selected predictor: `P8-structural-commitment`
- Basin diversity: `3`
- Trace attribution exact: `true`
- Frozen parent exact: `true`
- Development-valid: `true`
- Definitive claim eligible: `false`

## Predictor library

| predictor | discovery accuracy | held-out accuracy | held-out coverage | minimum cell | latest episode | qualifies |
|---|---:|---:|---:|---:|---:|---|
| P0-ratio-majority | 66.66% | 25.00% | 100.00% | 25.00% | 0 | false |
| P1-first-application | 25.00% | 8.33% | 100.00% | 8.33% | 1 | false |
| P2-first-4-balance | 0.00% | 8.33% | 100.00% | 8.33% | 9425 | false |
| P3-first-8-balance | 33.33% | 16.66% | 91.66% | 16.66% | 9429 | false |
| P4-first-16-balance | 33.33% | 16.66% | 91.66% | 16.66% | 10502 | false |
| P5-gap-before-opposition | 0.00% | 8.33% | 100.00% | 8.33% | 9425 | false |
| P6-longest-run-first-90 | 41.66% | 25.00% | 83.33% | 25.00% | 16498 | false |
| P7-gap-after-episode-90 | 25.00% | 8.33% | 100.00% | 8.33% | 90 | false |
| P8-structural-commitment | 100.00% | 100.00% | 100.00% | 100.00% | 18000 | true |
| P9-composite-tuple | 75.00% | 91.66% | 91.66% | 91.66% | 17941 | false |

## Seed 2000000000

- Incumbent side: `0`
- Side -> route: `[0, 1]`
- Stale blocked: `true`
- Post-closure inert: `true`
- Observation inert: `true`
- Controls passed: `true`

### Basin counts by equal-multiset ratio

- B:A `1:1` -> incumbent `0`, mixed `4`, alternative `2`, subthreshold `0`
- B:A `1:2` -> incumbent `2`, mixed `1`, alternative `3`, subthreshold `0`
- B:A `2:1` -> incumbent `0`, mixed `4`, alternative `2`, subthreshold `0`
- B:A `4:1` -> incumbent `2`, mixed `3`, alternative `1`, subthreshold `0`

## Seed 2000000001

- Incumbent side: `1`
- Side -> route: `[1, 0]`
- Stale blocked: `true`
- Post-closure inert: `true`
- Observation inert: `true`
- Controls passed: `true`

### Basin counts by equal-multiset ratio

- B:A `1:1` -> incumbent `0`, mixed `4`, alternative `2`, subthreshold `0`
- B:A `1:2` -> incumbent `2`, mixed `1`, alternative `3`, subthreshold `0`
- B:A `2:1` -> incumbent `0`, mixed `4`, alternative `2`, subthreshold `0`
- B:A `4:1` -> incumbent `2`, mixed `3`, alternative `1`, subthreshold `0`
