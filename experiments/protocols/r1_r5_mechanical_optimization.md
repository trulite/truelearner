# R1-R5 Mechanical Optimization Protocol

## Parent and scope

Parent: `boundary-buffers-v1-authority-v1` (`2b929e520a4153f7249ebb03005a4e283b5838a1`).

This successor changes only execution machinery. The accepted CELL/ARROW/SPIKE
transition laws remain singular and unchanged. The current slow configuration
is retained permanently as `ReferencePhysics`.

## Independently switchable axes

```text
scheduler  Vec | TimingWheel
traversal  GlobalScan | Adjacency
activity   FullScan | Frontier
layout     AoS | SoA
executor   Scalar | Batched
```

Reference configuration:

```text
Vec + GlobalScan + FullScan + AoS + Scalar
```

Production configuration:

```text
TimingWheel + Adjacency + Frontier + SoA + Batched
```

The switches are mechanical proof points, not alternative learning laws.

## Sequential acceptance inside one engineering branch

```text
R1  TimingWheel
R2  TimingWheel + Adjacency
R3  TimingWheel + Adjacency + Frontier
R4  TimingWheel + Adjacency + Frontier + SoA
R5  TimingWheel + Adjacency + Frontier + SoA + Batched
```

Each prefix must pass before the next prefix can be accepted. A failure stops
acceptance at that prefix even if later machinery is already implemented.

## R1 scheduling contract

The wheel is a bounded near ring plus an ordered overflow structure. The full
physical ordering key remains:

```text
arrival tick
phase
origin physical identity
target physical identity
serial
```

The current-tick bucket remains open for deterministic zero-delay insertions.
Canonical checkpoint activity is a sorted physical sequence independent of the
in-memory scheduler representation.

## R2 traversal contract

Adjacency indexes firing and modulatory lookup by physical source. They may not
change stored identity, traversal order, proposal behavior, or plasticity.

## R3 frontier contract

The frontier may make inactive CELL decay and eligibility expiry sparse only
when it produces exactly the eager state at every observable boundary.
Ordinary pressure remains the accepted full scan with epoch zero and period ten
through R5. Time may not disappear merely because structure is inactive.

## R4 layout contract

AoS and SoA are alternate resident representations behind one physical-law
surface. Stable `CellId`/`ArrowId` and generation resolve through disposable
resident slots. Canonical durable bytes must be identical across layouts.

## R5 executor contract

The batched executor may collect bounded ordered work, but commits physical
transitions in the exact accepted order. Same-target summation is forbidden
unless a future discriminator proves it equivalent under threshold,
refractory, phase, and zero-delay recurrence.

## Equivalence comparator

For every physical world, compare reference and candidate on:

```text
crossings
drive deliveries
modulatory deliveries
plasticity updates
structural proposals
physical deallocations
physical clock and pressure phase
canonical pending activity
canonical durable body bytes
natural quiescence
exact replay
```

`Work.total` from the accepted runtime is legacy implementation accounting and
is not an equivalence predicate. New evidence separates:

```text
PhysicalWork   must match
ExecutionCost  may and should differ
```

Execution cost records queue operations, comparisons, scans, allocations,
bytes touched, resident bytes, and elapsed CPU. Elapsed CPU is diagnostic and
never part of deterministic evidence.

## Failure localization

On any mismatch:

1. run the sequential configuration prefixes;
2. identify the first failing prefix;
3. bisect any sub-switches within that prefix;
4. enable canonical physical tracing only for the failing world;
5. serialize the first differing physical transition.

Physical traces contain ticks, phases, deliveries, firing, traversal,
eligibility, resistance changes, deallocation, and crossings. They contain no
wheel bucket, resident slot, SIMD lane, allocator, or implementation detail.

## Engineering and evidence rules

- Do not duplicate or parameterize the physical learning laws.
- Do not add cognitive or semantic nouns.
- Keep the whole-substrate boundary clone as the correctness path through R5.
- No R6 multi-arena, storage, transport, framebuffer, frozen-context, or
  journal claim is in scope.
- All Rust formatting, compilation, tests, matrices, and benchmarks run in E2B.
- Development evidence may exercise every configuration. Authority remains
  separately preregistered and sequential after development readiness.
