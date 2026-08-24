# R6 Partition Invariance Protocol

## Parent and question

Parent: `psel0-production-mechanics-selection-v1`
(`72465f42bb835d0f34939276e3d15faab493ef8e`).

Question:

> Can one physical organism be partitioned among multiple simultaneously
> resident execution arenas without the partition changing its physics?

R6 adds zero latency, zero phase, and zero semantic event at a resident-arena
boundary. It is development evidence only and does not advance organism
authority.

## Identity and placement

Stable physical/durable identity remains unchanged. Resident placement is a
disposable mechanical mapping:

```text
CellId / ArrowId / Generation    physical durable identity
ResidentArenaId                 current execution placement only
CellSlot / ArrowSlot            current dense address only
```

`ResidentArenaId` may never enter a physical ordering key, canonical body
bytes, physical trace, plasticity predicate, proposal predicate, or crossing.

R6 deliberately distinguishes durable `ArenaId` from `ResidentArenaId` because
changing a durable `CellRef` would change identity. A quiescent body may be
repartitioned without rewriting durable references.

## Selected mechanics

R6 begins from `MechanicalConfig::PRODUCTION`:

```text
TimingWheel + Adjacency + Frontier + AoS + opportunistic exact batching
```

`MechanicalConfig::REFERENCE` remains the permanent one-arena oracle.

## Scheduling contract

Each resident arena may own a local timing wheel, but a body-level merge must
preserve the existing global key exactly:

```text
arrival tick
phase
origin physical identity
target physical identity
global serial
```

Resident arena, local wheel position, lookup order, hash order, and routing
order are forbidden tie-breakers. Serial remains one deterministic body-wide
space.

Cross-arena traversal remains an ordinary ARROW traversal. At zero added
latency it schedules the same target tick and phase as local traversal.

## Partition matrix

Run each selected physical world under:

```text
P0  one resident arena
P1  two contiguous partitions
P2  four contiguous partitions
P3  striped partition
P4  deterministic pseudorandom partition
P5  adversarial high-traffic cut
P6  aggressive one-cell-per-arena partition for bounded small worlds
```

The physical graph, identities, delays, phases, initial state, inputs, and
global serial history remain identical.

## Exact comparator

Must match the one-arena production body on:

```text
crossings
Drive deliveries
Modulatory deliveries
plasticity updates
proposals
deallocations
physical tick and pressure phase
canonical pending activity
canonical durable body bytes
natural quiescence
exact replay
PhysicalWork
```

Explicitly excluded mechanical cost:

```text
resident-arena hops and lookups
routing operations
active resident-arena frontier
resident capacity
queue mechanics
logical bytes touched
elapsed CPU
```

## Restart controls

- A quiescent durable body may restore and then receive a different resident
  partition before identical future arrivals.
- A live checkpoint with canonical pending activity may restore and then be
  repartitioned before continuation.
- Neither operation may alter durable identity or future physical history.

## Scope exclusions

No storage loading, non-residence, eviction, network transport, fetch,
prefetch, pinning, admitted availability delay, or new latency is allowed.
Those begin at R7 only after partition invariance passes.

Do not optimize routing in R6. A simple deterministic body-level merge is
preferred over a faster mechanism whose equivalence is harder to inspect.

All Rust formatting, compilation, tests, and matrices run in E2B. Stop on any
physical divergence or need for a new substrate law.
