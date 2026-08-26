# CK0 junction checkpoint integration protocol v1

Status: frozen before any checkpoint implementation or evaluator change.

Parent: consolidated RS2 checkpoint negative
`e53097f56573b7411f7de27adcdd2a3bc345eeaf`
(`rs2-learned-inhibitory-topology-consolidated-negative-v1`).

## Question

Can a dead/orphan CELL round-trip through a checkpoint when CELL lifetime is
determined by topology rather than CELL resistance?

## Candidate integration change

CK0 changes checkpoint restoration only. It does not change execution,
learning, J0 lifetime, local forgetting, causal waves, or any CELL/ARROW/SPIKE
transition law.

Under the cumulative J0 feature, restore must preserve these durable CELL
fields independently:

```text
live
generation
resistance  // retained compatibility field; causally dormant for J0 lifetime
```

The loader must stop asserting `live == (resistance > 0)` for J0 CELLs. A
record such as `live=false`, `resistance=1`, `generation=7` is valid.

Dead resident CELL records remain unavailable to ordinary `CellRef`
resolution. Dead incident ARROW records may be restored for generation-safe
history/reuse, but cannot execute. Checkpoint transient-state restoration must
locate stored dead records without making them live or resolvable.

Non-J0 lineages retain their existing CELL resistance/liveness validation.
ARROW liveness continues to equal positive ARROW resistance; CK0 does not
change ARROW lifetime.

## Frozen families

1. A live junction remains live across save/decode/restore.
2. An orphan/dead junction with nonzero dormant resistance remains dead.
3. A dead CELL slot is reused generation-safely and its old reference remains
   stale across checkpoint/restart.
4. An incoming stale ARROW cannot deliver into a replacement CELL.
5. An outgoing stale ARROW cannot execute from a replacement CELL.
6. Live incident topology retains a junction across checkpoint/restart.
7. Loss of the last live incident link kills the junction and restart
   preserves that death.
8. A live checkpoint containing pending activity continues exactly.
9. A quiescent checkpoint produces exact future physical behavior.
10. Reference and Production produce exact physical continuation.

Every family runs under two disjoint roots and both Reference and Production
mechanics. The 20 cases/40 rows compare liveness, generation, slot reuse,
stale-reference behavior, ordered physical history, PhysicalWork, clock,
pending activity, final durable body, replay, and natural quiescence.

## Required lineage replay

Only a complete CK0 positive permits the frozen J0 and cumulative CV0/J0+SV1
evaluators to rerun unchanged. Both must remain exact before RS2 can run.

If CK0, J0, and CV0 are positive, the existing consolidated RS2 evaluator is
rerun unchanged except for a new evidence output directory. Its worlds,
predicates, and observation contract may not change.

## Static prohibitions

CK0 may not add or change:

- CELL consolidation or CELL resistance learning;
- special contact/junction CELL classes;
- TTL, garbage-collection flags, or checkpoint-only liveness repair;
- stale-reference bypasses;
- execution-time topology or decay behavior;
- RS2 fixture or predicate changes.

## Execution and stop rule

After targeted formatting/check/strict Clippy, CK0 executes exactly once in a
fresh E2B worker. Any CK0 failure freezes negative without repair or rerun.

Scientific advancement remains prefix-based:

```text
CK0 -> J0 replay -> CV0/J0+SV1 replay -> consolidated RS2
    -> CE1 -> FD2 v2 -> unchanged frozen ARC A2
```

Authority, oracle status, `arch.md`, and the Academy curriculum remain
unchanged throughout CK0.
