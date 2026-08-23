# PX3-R Arm B anonymous shared-CELL recruitment PROBE v2 protocol

Status: **PREREGISTERED MECHANICAL RETRY; EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

## Frozen v1 failure

PROBE v1 is permanently frozen at commit
`74813a268f593340703f1fbe4510de57e3c25276`, tag
`px3-r-shared-cell-probe-v1-first-clause-failure`.

- v1 source SHA-256:
  `2268c4445b438ae8e3d4bd6e1cbdc93d5d217c39b0d8200ac1c0a4d8d7f61e4c`;
- v1 CSV SHA-256:
  `1222b90fee4ed0ce2a20db7ea751f4c469073cde854b1b8ca0e774f9faf8038a`;
- v1 report SHA-256:
  `d7e559e4ebe434e236e8af3e08d88843dc6a4761ba879d5984452a78bb658973`.

V1 may not be changed, rerun, regenerated, or reinterpreted as evidence. It
failed because retained local return raised each trained incident coupling to
`2`, equal to the anonymous CELL threshold, making one route sufficient and
preventing old structure from weakening during swap.

Every v1 authority restriction, complete six-site opportunity symmetry,
source/driver/distractor construction, schedule, matched-marginal clause,
swap, hard control, evaluator isolation, atomicity, accounting, no-rescue rule,
and stop rule remains exact except for the sole physical correction below.

## Sole physical correction

At all six otherwise identical sites:

- the anonymous local CELL threshold is `4` rather than `2`;
- both weak incoming ARROWs retain resistance `1` but begin at coupling `2`
  rather than `1`.

Nothing else changes.

This is the conservative conjunctive setting implied by the authoritative
positive-coupling ceiling `2`: one incident ARROW remains insufficient
(`2 < 4`) before and after local return, while two incident ARROWs are exactly
sufficient (`2 + 2 = 4`). Return plasticity can change resistance but cannot
make an individual incident ARROW sufficient. The setting does not bypass or
modify the retained coupling law.

No new CELL, CELL creation, allocation rule, ARROW rule, threshold update,
semantic state, direct trace-to-trace coupling, or downstream-continuation
convergence is introduced. If this complete field fails, Arm B freezes
negative; no further opportunity parameter retry is authorized.

## Fresh namespaces and artifacts

V2 adds `0x0010_0000` to every v1 namespace, yielding exact bases
`0x9_B110_0000` through `0x9_BA10_0000`. V1 namespaces are never entered.

The sole v2 evidence command, after implementation freeze and no-CELL
validation, is:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_r_shared_cell_probe_v2 -- --probe
```

It emits exactly one `PX3_R_SHARED_CELL_PROBE_V2_EVIDENCE_SPENT` marker and
atomically creates:

```text
results/px3_r_shared_cell_probe_v2.csv
results/px3_r_shared_cell_probe_v2.md
```

Staging paths use the same basenames prefixed by `.` and suffixed `.staging`.
Any final or staging pre-existence refuses execution. Any outcome is frozen
without rerun.

## Interpretation and validation

`POSITIVE_CANDIDATE_SHARED_CELL_RECRUITMENT`,
`FROZEN_NEGATIVE_SHARED_CELL_RECRUITMENT`, and `FIRST_CLAUSE_FAILURE` retain
their exact v1 meanings. A positive remains development-only and cannot
advance PX3.

Pre-evidence validation additionally requires exact v1 source/artifact hashes,
the v1 failure tag, fresh v2 source/organism hashes, v2 namespace inventory,
artifact absence, focused formatting/compile/strict Clippy, refusal cases,
no-CELL preflight, forbidden-information scan, frozen lineage and authority
hashes, and proof that only fresh Arm B files differ from the frozen start.

No broad historical suite is authorized because shared code must not change.
No authority or definitive matrix may run or be simulated.
