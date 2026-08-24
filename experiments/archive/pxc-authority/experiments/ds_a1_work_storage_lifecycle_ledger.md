# DS-A1 physical work, storage, and lifecycle ledger

The compact primary report serializes once after deterministic single-threaded
execution. No hot-loop I/O occurs. Counts below include the complete required
control matrix because each control uses the same ordinary mechanism; evaluator
comparisons are reported separately from organism work.

## Per-seed A1 ledger

| item | count |
|---|---:|
| relation observations | 210 |
| membership checks | 420 |
| learned-relation checks | 84 |
| proposals formed | 84 |
| per-episode dedup checks | 42 |
| support comparisons | 169 |
| support updates | 60 |
| route CELLs installed | 24 |
| route ARROWs installed | 12 |
| structural normalizations | 15 |
| structural dedup checks | 10 |
| generation validations | 66 |
| SPIKE propagations | 18 |
| ARROW traversals | 10 |
| state mutations | 18 |
| bridge reference copies | 9 |
| duplicate reclamations | 1 |
| cleanup items | 22 |
| evaluator comparisons, excluded from organism work | 6 |
| A1 organism work subtotal | 1,274 |

Frozen E0 work is 99,442 in MICRO and 154,354 per GATE seed. Total physical
organism work is therefore 100,716 for MICRO seed 100 and 155,628 for each
GATE seed 100..104 (778,140 total across GATE).

Persistent storage is 12 bytes for three A1 anonymous templates plus 130 bytes
for frozen E0, or 142 bytes total per seed. Peak temporary storage is 609 bytes.
Maintenance and carrying work are both zero because no such lifecycle exists
at this wave. Cleanup counts every temporary CELL, ARROW, root, handle, and
binding, and the post-cleanup inventory is exactly zero.

## Dependency delta

The frozen prior cumulative retry reached the same actual E0 episode but its A0
two-step coactivity mechanism produced one root, one effect, and one handle,
collapsing at composition stage 4. DS-A1 adds only generic per-relation local
variation/consolidation. On the actual current E0 event it produces two roots,
two structural continuations, two distinct effects, and two handles. It adds no
DS1 call, consequence, credit, rank, or later dependency. A separate future
unchanged-DS1 retry remains mandatory and was not run.
