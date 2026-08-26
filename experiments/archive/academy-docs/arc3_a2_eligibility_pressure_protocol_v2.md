# ARC3 A2 eligibility-aware pressure successor protocol v2

Status: frozen after the v1 accounting negative and before v2 implementation
or execution.

## Sole measurement repair

Keep the v1 pressure law, worlds, schedules, mechanics, and behavioral
expectations byte-identical. Replace only Academy's use of legacy
`Work::total()` for the field named `physical_work` with a causal aggregate of:

```text
Drive deliveries
+ Modulatory deliveries
+ local return updates
+ local structural proposals
+ physical deallocations
```

These are the exact counters already compared by the permanent R1-R6
reference/production differential oracle. Queue operations, comparisons,
scans, allocations, bytes touched, and resident capacity remain execution
cost and must not enter the aggregate.

## Gates

1. Rerun the same focused paired substrate worlds.
2. Rerun the same bounded four-context phases 0 and 9 under reference and
   production mechanics. Complete observations must now match.
3. Only if both pass, run the official A2-only phases 0 and 9 independently
   under reference and production, each with exact replay.
4. Compare the official causal observations after excluding only the explicit
   mechanics label.

Any other failure freezes v2. Broader retained PXR0/PX-C replay remains gated
on all four official rows passing.
