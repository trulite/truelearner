# PX4 physical learned-lifetime development implementation audit

Status: **IMPLEMENTATION FROZEN BEFORE PROBE EXECUTION**.

## Frozen inputs

- PX2 parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- PX2 tag: `px2-physical-causal-direction-authoritative`;
- development protocol commit: `eefa31f563dbdee0dd09e743bd7901a1af5983fa`;
- development protocol SHA-256:
  `fd152ef5c73c071e68fe41bb0e1b38707b00a43b8c2447ee647e847624876bb5`;
- byte-frozen active substrate law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- PX4 development harness SHA-256:
  `1fca7f3a36279045d8188bfe62fc9ee7be59db4a70146da9b4f87c5df17092b5`.

## Change boundary

Only the new development example
`crates/px0-physical-correspondence/examples/px4_physical_lifetime.rs` is
executable. No library, retained-physics, PX0, PX1, or PX2 source changed.

The example constructs fresh CELL/ARROW/SPIKE worlds and schedules physical
arrivals. Its case/layout branches are evaluator-only experiment construction;
the substrate receives no case value, expected side, result predicate, or
serialized state. Results are observed only after propagation.

The old M4/DS6 source is neither imported nor called. The example depends only
on the already-authoritative `px0_physical_correspondence` public physical API.

## Information-flow audit

Organism-visible mutable state remains the byte-frozen `PlasticSubstrate`:
CELL state, ARROW coupling/resistance/eligibility/generation/live state, queued
SPIKE state, local time, and pressure time. The harness adds no organism state.

The implementation contains no supplied lifetime class, duration counter,
aging/expiry category, retention/deletion policy, semantic memory record,
evaluator delete, contradiction-specific decrement, reinstatement branch,
future-use input, or task-boundary cleanup. Deallocation and reproposal execute
only inside the frozen substrate law.

Serialization consumes completed evaluator observations and cannot feed back
into a substrate. The `--definitive` flag exits with status 2. Each stage uses
fixed namespaces and write-once staging/final paths.

## Accounting

Every row serializes resistance, live state, generation, effects before/after
pressure, stale effects, disuse trajectory, fresh ArrowId distinctness,
quiescence, exact normalized duplicate outcome, work totals, pressure updates,
deallocations, allocation slots, known live allocations, persistent bytes, and
complete-state fingerprint.

The vector implementation retains non-live generation tombstones in allocation
slots; this is reported as storage accounting and is not interpreted as an
executable learned path.

## Pre-execution validation

The following passed without executing a development stage:

```text
cargo fmt --all -- --check
cargo check -p px0-physical-correspondence --example px4_physical_lifetime
cargo clippy -p px0-physical-correspondence --example px4_physical_lifetime -- -D warnings
git diff --check
```

PROBE, MICRO, and GATE evidence remain unspent at this audit.
