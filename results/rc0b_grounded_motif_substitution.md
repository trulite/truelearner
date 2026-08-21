# RC0b grounded motif substitution

- protocol: `grounded-motif-substitution-rc0b-v1`
- mode: `definitive`
- claim eligible: `true`
- RC0b-A: `true`
- RC0b-B: `true`

| arm | correct | trace matches | work | motif firings |
|---|---:|---:|---:|---:|
| concrete-reference | 768/768 | 768/768 | 7912448 | 0 |
| full-rc0a | 768/768 | 768/768 | 8192000 | 0 |
| motif-substitute | 768/768 | 768/768 | 7547264 | 97920 |
| changed-surroundings | 768/768 | 768/768 | 7691619 | 97920 |
| interruption-reentry | 768/768 | 768/768 | 7547264 | 97920 |
| context-effect-invalidation | 768/768 | 768/768 | 8238208 | 0 |
| forced-stale-same-endpoint | 768/768 | 0/768 | 7547264 | 97920 |
| rc0a-parent-invalidation | 768/768 | 768/768 | 8713728 | 0 |
| subthreshold-evidence | 768/768 | 768/768 | 8192768 | 0 |
| shuffled-recurrence-evidence | 768/768 | 768/768 | 8194304 | 0 |
| no-bindings | 0/768 | 768/768 | 267264 | 0 |

## Gates

- frozen-ancestry-and-reconstruction: **PASS**
- one-motif-earned-from-three-episodes: **PASS**
- role-relative-persistent-structure: **PASS**
- observable-equivalence-and-lower-work: **PASS**
- fresh-bindings-and-changed-surroundings: **PASS**
- interruption-reentry-equivalence: **PASS**
- context-effect-invalidates-to-rc0a: **PASS**
- same-endpoint-stale-shortcut-fails-trace: **PASS**
- parent-invalidation-resumes-rg0a: **PASS**
- subthreshold-does-not-compile: **PASS**
- shuffled-evidence-cannot-fire: **PASS**
- bindings-remain-necessary: **PASS**
- state-isolation-and-determinism: **PASS**
- single-motif-same-executor-source-audit: **PASS**
- whole-runtime-below-concrete-diagnostic: **PASS**
