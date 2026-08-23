# CJ0 ARM CJ-B terminal GATE v1 frozen-negative audit

Status: **FROZEN TERMINAL DEVELOPMENT NEGATIVE; LANE STOPS**.

## Atomic result

The sole GATE command emitted one
`CJ0_B_LOCALLY_GATED_ARROW_GATE_V1_EVIDENCE_SPENT` marker and exited `1`.
It was not rerun, repaired, regenerated, or extended.

| artifact | SHA-256 |
|---|---|
| `results/cj0_b_locally_gated_arrow_gate_v1.csv` | `e76f7256033de9352fac76de72c5ff37a8dcec899c2ff5eec3447d9286604707` |
| `results/cj0_b_locally_gated_arrow_gate_v1.md` | `08394d47aa7c7c494cc5d5b266868780ecba6009950db1f63eb50112bed1a1d8` |

Both staging paths are absent. The CSV is 9 lines, 94 columns, 4,978 bytes;
the report is 1,100 bytes.

## Exact first failure

The matrix passed `76/80` claims and `4/8` rows. The only false clause was
`G3` in all four timing-stratum S0 rows:

```text
S0 changed training A+D | C+B = 20 | 19
S1 changed training A+D | C+B = 20 | 20
```

Every other serialized `G0--G9` bit is true. S0 still ended with changed
held-out A+D/C+B `1|1`, old immediate/late A+B/C+D `0|0|0|0`, old live counts
`0|0`, and new live counts `1|1`. The failure is the preregistered exact
changed-training outward-effect count, not final reuse.

## Physical failure mechanism

The cause follows deterministically from the frozen S0 schedule and physical
source; this audit does not execute another cell.

- S0 initial activity ends at tick 30 and its fixed gap ends at tick 55.
- First changed A+D occurs at tick 55.
- First changed C+B occurs at tick 59. Its missing local C-to-CB ARROW is
  generically proposed at resistance 1/coupling 1 with physical distance/delay
  2. Current contributor state makes the transmission inspection succeed.
- The queued SPIKE is due at tick 61. `elapse_to(61)` crosses the ordinary
  tick-60 pressure boundary and subtracts 1 from every live ARROW.
- The just-proposed resistance-1 ARROW physically deallocates and increments
  generation before the queued SPIKE's generation check. That first C+B output
  is therefore absent. Later returned occurrences mature the replacement,
  yielding 19 outputs and successful final reuse.
- S1 starts changed activity at tick 70, after rather than immediately before
  a pressure boundary, so both changed routes produce all 20 outputs.

This is a substrate-native first-use delivery/pressure race. No evaluator
invalidated an endpoint and no relation-change signal exists. It is preserved
as measured rather than normalized away.

## Passed terminal surfaces

Across all eight fresh rows:

- initial trained A+B/C+D held-out: `1|1`; crossed A+D/C+B: `0|0`;
- self-evidence output/consumption: `0|0`;
- post-change new held-out: `1|1`; old immediate/late: `0|0|0|0`;
- recursion A+B->X, X+C->Y, Y+D->Z: `1|1|1`;
- full A+B+C+D chain X/Y/Z: `1|1|1`, one Z outward crossing;
- missing B/C/D recursive crossings: `0|0|0`;
- all three recursive candidates: resistance `20`, coupling `2`;
- ordinary convergent A-only/B-only/joint activation: `1|1|1`;
- temporal crossing signature together/A-then-B/overlap/within/absent:
  `1|0|1|1|0`;
- temporal impulses/ticks: together `4@0`, overlap `4@1`, within `3@1`;
- exact replay and natural quiescence: pass.

Thus recursion, ordinary inclusive alternatives, and the preregistered
temporal expressivity tests do not create the failure.

## Accounting and boundary

- ledgered physical work: `92,852` operations;
- S0/S1 final flat storage: `1,984`/`2,688` bytes per row;
- recursive storage: `1,024` bytes per row;
- physical module and all frozen PROBE/MICRO sources/results: unchanged;
- authoritative PX0--PX2 and PX3/PX3-R negatives: unchanged;
- later-stage surface: none.

The protocol makes any GATE failure terminal. This lane therefore returns a
frozen CJ-B development negative, does not advance PX3 or any parked stage,
and creates no subsequent scientific surface.
