# INVALID pre-opaque-location RP0a smoke

This excluded E2B smoke was produced from implementation commit `bdf3890`
before an opacity audit found that temporary lower bindings were keyed by the
evaluator's stable lower-role enum. It is preserved as invalid implementation
feedback and is not evidence for the corrected RP0a gate. Commit `9461336`
replaced those learner-facing keys with fresh opaque lower-location identities.
The frozen protocol permits no second smoke, so this smoke was not rerun.

# RP0a reflected program discovery

Smoke gate: **FAIL** (8 / 12 gates passed).

Frozen anchors: P0 `true`, P3 `true`.

| Arm | Competent | Held-out | Role transfer | Roles | Arrows | Competence | Training work | Eval work | Bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rp0a-integrated | 0/1 | 0/8 | 80/80 | 10.0 | 2.0 | none | 1475589 | 960 | 6608 |
| activity-only | 0/1 | 0/8 | 0/80 | 0.0 | 0.0 | none | 130000 | 544 | 0 |
| shuffled-provenance | 0/1 | 0/8 | 80/80 | 10.0 | 0.0 | none | 6400426 | 832 | 6608 |
| shuffled-terminal-feedback | 0/1 | 0/8 | 80/80 | 10.0 | 0.0 | none | 6352229 | 824 | 6608 |
| random-terminal-feedback | 0/1 | 0/8 | 80/80 | 10.0 | 0.0 | none | 6190643 | 824 | 6608 |
| symmetric-impossible | 0/1 | 0/8 | 80/80 | 9.0 | 0.0 | none | 1040978 | 784 | 6600 |
| oracle-reflected-program | 1/1 | 8/8 | 80/80 | 10.0 | 4.0 | 0.0 | 0 | 1288 | 0 |

## Gates

- `frozen-ancestry-and-isolation`: PASS
- `substrate-native-oracle-execution`: PASS
- `opaque-reflected-boundary`: PASS
- `anonymous-reflected-role-formation`: PASS
- `symmetric-impossible-role-discipline`: PASS
- `frozen-p0-p3-positive-anchors`: PASS
- `fresh-integrated-competence`: FAIL
- `held-out-execution-transfer`: FAIL
- `four-arrow-learned-topology`: FAIL
- `controls-discriminate`: FAIL
- `read-only-determinism`: PASS
- `accounting-and-lifecycle`: PASS

Workspaces destroyed: `281988/281988`; maximum live: `2`.
