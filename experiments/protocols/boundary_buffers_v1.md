# Boundary Buffers V1 successor protocol

Status: preregistered before implementation and execution.

Parent: `physical-body-v1-authority-v1` at `c9daa0af96e87bc8b0e6ef0f30bea137e0cfc33b`.

## Claim

A bounded host boundary can stage anonymous `SpikeInput` values and ordinary
`Crossing` observations without changing the accepted PX-C/Physical Body V1
physics. Input order is preserved, output order is preserved, full buffers
apply explicit backpressure, and live continuation includes both buffers.

## Frozen design

- `BoundaryRuntime` owns one unchanged `PlasticSubstrate`.
- Input and output buffers have fixed non-zero capacities.
- Enqueuing a batch is atomic: insufficient input capacity changes nothing.
- Running is transactional with respect to output capacity: if produced
  crossings do not fit, substrate and input buffer remain unchanged.
- A successful run consumes the staged inputs, executes the existing
  `enter`/`propagate` laws, and appends crossings in their existing order.
- Partial or complete output draining never affects substrate physics.
- Modulation, pressure, proposal, refractory, generation, and crossing laws
  remain byte-identical in meaning and implementation.
- A boundary live checkpoint stores capacities, outward region, queued inputs,
  queued outputs, and the canonical core live checkpoint.

## Required controls

1. Input capacity rejects atomically.
2. Existing output occupancy causes reversible backpressure.
3. A batch larger than total output capacity is distinguished from temporary
   fullness and changes no state.
4. Partial drain preserves FIFO order.
5. Same-tick input ordering matches the direct accepted path.
6. Buffered live checkpoint bytes round-trip canonically and resume exactly.
7. Empty-buffer buffered execution is exactly equal to direct `arrive` for
   the complete 16-row cumulative PX0-PX8 matrix.
8. The cumulative matrix remains 524/524 and the Physical Body clauses remain
   16/16; the new buffer clauses must all pass.
9. Natural quiescence, work accounting, resident-body accounting, compaction,
   stale generation rejection, and pressure phase remain unchanged.

## Execution discipline

All Rust formatting, compilation, linting, tests, and regression execution run
in E2B. The accepted authority artifacts are not overwritten. This successor
publishes separate development evidence; the oracle changes only after every
gate passes.
