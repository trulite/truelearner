# CJ0 ARM CJ-B GATE v2 result audit

Status: **POSITIVE TERMINAL DEVELOPMENT GATE; EVIDENCE FROZEN**.

## Atomic result

The sole preregistered command emitted
`CJ0_B_LOCALLY_GATED_ARROW_GATE_V2_EVIDENCE_SPENT` and exited `0`. It was not
rerun, regenerated, tuned, or repaired.

| artifact | SHA-256 |
|---|---|
| `results/cj0_b_locally_gated_arrow_gate_v2.csv` | `4c3d2e63765f5eb70f0ab0e1d673462a9bc32a7bf45fc51603a115e900bfed69` |
| `results/cj0_b_locally_gated_arrow_gate_v2.md` | `12c8a59897709a0c0927e1cfe5f96dadf881179ad40f58ed02dd82a789b980bd` |

Both staging paths are absent. The CSV is 9 lines, 130 columns, and 7,029
bytes. The report is 1,262 bytes. All eight namespaces are distinct.

## Result matrix

All `88/88` clauses pass across `8/8` fresh rows.

- uniform changed offset: `2` in every row;
- initial training A+B/C+D: `8|8`;
- trained held-out A+B/C+D: `1|1`;
- initial crossed A+D/C+B: `0|0`;
- source-only self-evidence crossing/consumption: `0|0`;
- changed training A+D/C+B: `20|20`;
- changed held-out A+D/C+B: `1|1`;
- old immediate and late A+B/C+D: `0|0|0|0`;
- old live support: `0|0`; new live support: `1|1`;
- recursive level/full/missing: `1|1|1 / 1|1|1 / 0|0|0`;
- all three recursive candidate couplings/resistances: `2 / 20`;
- ordinary convergence A-only/B-only/joint: `1|1|1`;
- temporal together/A-then-B/overlap/within/absent: `1|0|1|1|0`;
- replay and natural quiescence: pass.

S0 new A+D/C+B resistance is `38|38`; S1 is `34|34`. Thus the corrected
schedule is mirrored between the changed routes within each timing stratum.

## Timing-boundary characterization

Every row independently records the same substrate-native sequence:

| stage | serialized observation |
|---|---|
| first proposal | tick `8`, one generic proposal, coupling/resistance `1/1`, delivery due `10` |
| pressure crossing | three ordinary arrow pressure updates at tick `10`, one physical deallocation |
| generation check | three delivered spikes/checks total; stale proposal destination firings/crossings `0|0`; proposal records/live/generation `1|0|5` |
| replacement | tick `11`, one generic replacement, delivery tick `13`, destination firing/crossing `1|1` |
| maturation | one ordinary return; records/live/resistance/coupling/generation `2|1|4|2|5` |
| held-out | tick `14`, delivery tick `16`, new proposals `0`, destination firing/crossing `1|1` |

Boundary replay, quiescence, and work `147` pass in every row. This preserves
the v1 mechanism: a weak queued proposal that crosses ordinary pressure can
die, and its stale generation cannot execute. Ordinary activity can then
propose, mature, and reuse a replacement without historical resurrection.

## Accounting and isolation

- total ledgered physical work: `93,880` operations;
- per-row total work: S0 `11,172`, S1 `12,298`;
- result-pair storage: `8,291` bytes;
- physical module SHA-256: `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`;
- GATE v2 evaluator SHA-256: `4733bc8d1cc06881102e0d36067649d22381be554b71056debf36a306b83bb86`;
- GATE v1 protocol, implementation, results, negative audit, tag, and handoff:
  unchanged and still classified as a frozen terminal development negative;
- authoritative PX0--PX2 and frozen PX3/PX3-R material: unchanged;
- later scientific surface: none.

This result closes CJ-B development at GATE.
