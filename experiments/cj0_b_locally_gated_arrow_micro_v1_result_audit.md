# CJ0 ARM CJ-B locally gated ARROW MICRO v1 result audit

Status: **POSITIVE DEVELOPMENT MICRO; GATE ELIGIBLE AND UNSPENT**.

## Atomic result

The sole MICRO command emitted one
`CJ0_B_LOCALLY_GATED_ARROW_MICRO_V1_EVIDENCE_SPENT` marker and exited `0`.

| artifact | SHA-256 |
|---|---|
| `results/cj0_b_locally_gated_arrow_micro_v1.csv` | `d88aea36480022740067148431afbfe4b8150c729d476670b8b4dcaf53c4a2fa` |
| `results/cj0_b_locally_gated_arrow_micro_v1.md` | `bb7ea15b79ec0dbbd6d77591b2f0adf7a8bd44661923d6bae9a4e0669e13a871` |

Both staging paths are absent. The CSV is 5 lines, 68 columns, 2,198 bytes;
the report is 865 bytes.

## Reversal result

All four fresh physical variants passed: `4/4` rows and `40/40` conjunctive
claims.

- old A+B/C+D held-out before change: `1|1` per row;
- changed experience: A+D/C+B `20|20` occurrences per row;
- changed A+D/C+B held-out after change: `1|1` per row;
- old A+B/C+D immediate and additional-gap held-out: `0|0|0|0` per row;
- old candidate live counts/resistance: `0|0` / `0|0`;
- new candidate live counts/resistance/coupling: `1|1` / `38|38` / `2|2`;
- new candidate record counts/generations show ordinary proposal after
  deallocation rather than restoration of old records;
- no endpoint-specific invalidation or change input exists.

## Bootstrap result

Thirty ticks of ordinary pressure first killed the complete weak field. From
that fully deallocated state, ordinary A+D/C+B activity generically proposed
new coupling-1 ARROWs, immediately consumed current state 2, and matured the
new structures. Held-out changed execution was `1|1` in every row, with live
resistance `14|14`. The otherwise identical no-return control was `0|0`.

Thus changed organization did not require a mature higher-order structure to
fire first. Returned activity, not a relation-change signal, separated reusable
organization from transient first use.

## Controls and accounting

- trigger-only/contributor-only output and candidate consumption: zero;
- exact replay: pass;
- natural quiescence and finite returned recurrence: pass;
- proposals/deallocations: `7/9` per primary row;
- final retained structure: `21` ARROW records, `1,920` bytes per row;
- ledgered physical work: `55,032` operations;
- physical module and frozen PROBE evaluator/results: byte-identical;
- later-stage execution: none.

This result licenses only the separately preregistered development GATE and
does not advance PX3 or any parked program stage.

