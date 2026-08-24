# LR1 three-factor local plasticity parallel result audit v1

Status: **A NEGATIVE; B/C FUNCTIONAL POSITIVE; NO SUCCESSOR SELECTED; PX3 NEGATIVE**.

Evidence snapshot: implementation-audit commit
`5ff81f2cbf1b2e09ddacd3a29c376037e504a73c`, tag
`lr1-three-factor-local-plasticity-parallel-implementation-v1`.

## Write-once execution

Three fresh isolated E2B sandboxes passed formatting check, release tests,
strict release Clippy and frozen `--preflight`, then ran concurrently:

| arm | sandbox | marker |
|---|---|---|
| A | `ieir2gobu0f9ng4soxkwq` | `LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_A_EVIDENCE_SPENT` |
| B | `iqlzp45bpg949dq87apkv` | `LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_B_EVIDENCE_SPENT` |
| C | `ihbhzy0nps820squt8jz8` | `LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_C_EVIDENCE_SPENT` |

Each marker appeared once. Each command terminated and published only its own
registered pair. No evidence command was rescued, tuned or rerun.

## Frozen artifacts

| arm | CSV SHA-256 | report SHA-256 |
|---|---|---|
| A | `2c72f9a332e76693be21acdefc767cc3f96bd50c2e1063076e6dc58285710361` | `baa6d4d7b368e5198b6af8f272b18ed174a1e72a533d80bc5bfb4bb488f3f546` |
| B | `8a503faaed3bdbbd9bf719c952d1906a196c6e585950604bcafa588e994a86dd` | `156894449970ef0130593f625f098b25d3d2dfce82e8c10c3fbdf4e77597956b` |
| C | `dd5f55a9087e11470efdd69e9c1fac562595c978e1948b08d3debd44e5c58d29` | `95e4c95547a3e14e36dc9b6d5370064b52f1177f4c334b44e44528c1e164290f` |

## Results

| measure | A route-aware | B compartmental | C modulatory |
|---|---:|---:|---:|
| passing rows | 20/24 | 24/24 | 24/24 |
| validity clauses | 184/192 | 192/192 | 192/192 |
| exact replay | yes | yes | yes |
| all naturally quiescent | no | yes | yes |
| lawful/accepted updates | 3,956 | 8 | 8 |
| route edges / adjacency scans | 3,988 | 72 | 0 |
| total ledgered work | 74,656 | 2,836 | 2,960 |
| maximum row work | 18,159 | 197 | 205 |
| persistent bytes per world | 784 | 832 | 784 |
| physical transmission mode added | no | no | yes |

### Arm A -- functional negative

Every non-simultaneous control passed. All four
`simultaneous-upstream-and-return` rows were exactly replayed non-quiescent
failures. At the 1,001-delivery observation bound each had:

```text
candidate resistance = 925
qualified accepts / return updates = 988
work = 18,159
naturally quiescent = false
```

The route-aware law applies uniformly at every plasticity site. Once a live
cycle qualifies `P -> X`, it also qualifies and strengthens `X -> S` when the
new candidate traversal arrives. The strengthened drive closes an excitatory
`P -> X -> S -> P` oscillator. Existing route state is logically sufficient
for provenance, but generic cycle recognition is not a safe local learning
law.

The bounded observer prevented the Rust process from retaining an unbounded
trace. It did not clear pending activity or convert the row to quiescent.

### Arm B -- functional positive

All 24 rows and 192 clauses passed. Renewed upstream drive could execute P but
produced zero candidate updates. Timely compartmental return produced exactly
one update; independent and late return produced none; simultaneous drive and
return produced one rather than two. All rows replayed exactly and quiesced.

B adds one ordinary neighboring CELL compartment (48 persistent bytes in this
fixture) and a uniform adjacency interaction. It adds no arrow mode, semantic
return flag or route search. Its 72 adjacency scans and 2,836 total work were
the lowest measured architectural work among the functional positives.

### Arm C -- functional positive

All 24 rows and 192 clauses passed with the same causal update vector as B.
The 16 modulatory deliveries never entered P's activation state. Timely
modulation updated eligible structure exactly once; renewed drive never did;
all rows replayed exactly and quiesced.

C adds an explicit physical `Drive | Modulatory` field to Arrow transmission.
It adds no valence or outcome identity. Rust layout padding kept this tiny
fixture's `persistent_bytes` measurement at 784, but the active-state schema is
strictly larger and its total ledgered work was 2,960.

## Generated CSV defect

All three generated CSVs serialize `incoming_routes` using Rust endpoint debug
text such as `(CellId(7), CellId(5))` without CSV quoting. Each row therefore
contains three extra commas and is not directly RFC-CSV parseable.

The reports were generated from in-memory typed rows before serialization and
are unaffected. Row audit is deterministic: preserve columns 0--14, join raw
fragments 15--18 as `incoming_routes`, then align raw fragment 19 with declared
column 16. This recovers exactly 33 declared fields for A/B and 35 for C, and
reproduces the report counts above.

This is an artifact-serialization defect, not a causal evaluator discrepancy.
It remains frozen unchanged; no rerun is permitted. Future workflows must use
proper CSV quoting before evidence.

## Selection boundary

B is the current lower-cost **development front-runner**, not a selected
successor. The small matrix does not establish that arbitrary activity in a
neighboring compartment remains specific in dense/recurrent topology. C is a
clean functional fallback with a larger explicit substrate change. A is
rejected in its frozen generic-cycle form.

Before selection or PX3 reopening, B and C require independently
preregistered successor-conformance tests against authoritative PX0 worlds and
frozen PX1/PX2 behavior, plus dense-topology controls for B's compartment
specificity. PX0 authority remains intact for its original law/worlds; no LR1
authority exists; PX3 remains negative and PX4 blocked.
