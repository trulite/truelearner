# PX3-R Arm B anonymous shared-CELL PROBE v1 result audit

Status: **FROZEN FIRST-CLAUSE FAILURE; DEVELOPMENT EVIDENCE SPENT; PX3 AUTHORITY ABSENT**.

## Frozen result

The sole preregistered command ran once from implementation commit
`351b77a1db94aca0bc227a26a1dba7b5dd614ede`, tag
`px3-r-shared-cell-probe-v1-implementation`, and emitted exactly one
`PX3_R_SHARED_CELL_PROBE_V1_EVIDENCE_SPENT` marker.

- classification: `FIRST_CLAUSE_FAILURE`;
- process exit: `1`;
- CSV: `results/px3_r_shared_cell_probe_v1.csv`, SHA-256
  `1222b90fee4ed0ce2a20db7ea751f4c469073cde854b1b8ca0e774f9faf8038a`;
- report: `results/px3_r_shared_cell_probe_v1.md`, SHA-256
  `d7e559e4ebe434e236e8af3e08d88843dc6a4761ba879d5984452a78bb658973`;
- CSV shape: `11` data rows and `31` columns;
- CSV/report storage: `6,607` and `1,473` bytes;
- total ledgered work: `1,880,777` operations.

The artifacts are final, atomically published, and must not be regenerated or
rerun.

## Exact collapse

Initial acquisition did stabilize only the intended incident topology:

- all four separately serialized route-strength multisets were exact equals,
  `0|0|33`;
- anonymous sites `A+B` and `C+D` each retained incoming resistance `33|33`;
- all four other sites retained `0|0`;
- training source firing counts were `16` at every one of the 12 physical
  ports;
- training anonymous-CELL and outward counts were `16|0|0|0|0|16`.

However, the retained PX0-PX2 local-return law increases a positive eligible
ARROW's coupling from `1` to its ceiling `2` while its prior resistance is at
or below `16`. The preregistered anonymous CELL threshold was `2`. After the
first successful joint use, either stabilized incident ARROW therefore became
individually sufficient to fire that CELL.

Consequently, held-out `A+D` and `C+B` did not test as crossed absence. Each
route activated its old learned anonymous CELL independently: the held-out
rows contain one outward crossing at both old sites for crossed uses, rather
than zero. This violates the mandatory discriminator even though individual
route strength was initially equal.

During the swap stream, single old-route participation continued to fire each
old anonymous CELL and return physical activity. Old incoming resistance grew
from `33` to `72` rather than weakening. New sites reached `39|39` and `41|41`,
but old and new structures coexisted for the wrong physical reason. The
post-swap per-route multisets became `0|39|72` for two routes and `0|41|72`
for the other two, so the exact matched-marginal clause also failed.

This is not evidence for Arm B recruitment and is not an interpretable Arm B
negative: the preregistered opportunity ceased to require co-participation
after retained coupling plasticity acted. It is a frozen first-clause failure.

## Controls and integrity

- frozen lineage/source/negative hashes: pass;
- forbidden-information audit: pass;
- exact complete-state duplicate replay: pass for every row;
- natural quiescence: pass for every finite propagation;
- exact explicit source firing counts and zero autonomous source refiring:
  pass;
- correlated-without-participation, blocked-return, absent-opportunity, and
  stale controls: physical absence after pressure;
- stable alternative and multi-organization controls: topology formed, but
  the same single-ARROW sufficiency invalidated held-out interpretation;
- authoritative/shared files changed: none;
- authority/definitive matrix: not run or simulated;
- broad historical suite: not run because shared code did not change.

## Mechanically constrained dependency

Any further Arm B opportunity test must preserve this failure byte-for-byte
and must make the anonymous CELL physically conjunctive even after the retained
coupling law reaches its ceiling. It may not weaken or bypass that law, add
CELL creation, add substrate allocation, use direct trace-to-trace coupling,
or use downstream-continuation convergence.

With exactly two incident positive ARROWs and the authoritative coupling
ceiling `2`, the conservative no-new-law opportunity is threshold `4` with
both weak resistance-`1` incoming ARROWs already at ordinary coupling `2`.
One incident ARROW then remains insufficient (`2 < 4`) and two are exactly
sufficient (`2 + 2 = 4`) before and after return plasticity. This is a physical
opportunity correction, not a substrate update. It requires a fresh protocol,
source, namespaces, artifacts, and one-shot evidence; v1 cannot be amended or
rerun.
