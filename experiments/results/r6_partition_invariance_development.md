# R6 Partition Invariance Development Result

## Outcome

R6 is development-positive:

```text
partition comparisons       36 / 36
checkpoint controls           2 / 2
total clauses                38 / 38
exact replay                  true
natural quiescence            true
added arena-boundary latency  0
```

The same physical bodies ran under:

```text
P0  one resident arena (control)
P1  two contiguous resident arenas
P2  four contiguous resident arenas
P3  four striped resident arenas
P4  seven deterministic pseudorandom resident arenas
P5  adversarial high-traffic cuts
P6  aggressive cuts, up to 128 resident arenas
```

Every partition matched the one-arena `MechanicalConfig::PRODUCTION` body on
canonical pending activity, crossings, physical trace, Drive and Modulatory
deliveries, plasticity, proposals, deallocations, body-wide clock and pressure
phase, canonical durable body bytes, natural quiescence, and physical work.

## Architectural result

Partition placement is mechanically observable in cost but physically inert at
zero admitted latency.

R6 required two explicit notions:

```text
ArenaId
    durable identity/content unit

ResidentArenaId
    disposable execution placement
```

Changing `ResidentArenaId` does not alter `CellRef`, `ArrowRef`, generation,
canonical body bytes, physical trace, or behavior. This is the resident-arena
analogue of the already accepted `slot != identity` result.

Each resident arena owns a timing wheel. A deliberately simple body-level merge
selects activity through the unchanged global ordering key:

```text
arrival tick
phase
origin physical identity
target physical identity
global body serial
```

Resident placement never enters the key. Serial remains body-wide. Cross-arena
ARROW traversal schedules the ordinary target activity with its original delay
and phase.

## Checkpoint controls

- A quiescent checkpoint restored into a different resident partition and
  produced identical future physics.
- A live checkpoint containing canonical pending activity restored, was
  repartitioned, and continued identically.

Resident placement is intentionally absent from durable and live checkpoint
semantics. It can be reconstructed as execution machinery after restore.

## Preserved oracle

The final consolidated E2B run also established:

```text
arena-format tests                4 / 4
core tests                       14 / 14
R1-R5 differential pairs        80 / 80
accepted behavioral clauses    536 / 536
strict Clippy                    PASS
format check                     PASS
```

The new core test directly confirms that repartition preserves durable identity,
canonical pending order, canonical body bytes, and future physical history.

## Diagnostics before freeze

Three pre-freeze development corrections were made without changing the frozen
question:

1. Durable restore directly constructed CELL storage and initially omitted the
   default resident-placement entry. The restore path now initializes every
   restored CELL to resident arena zero before optional repartition.
2. The quiescent checkpoint control initially attempted capture while ARROW
   eligibility was still live. It now advances past transient eligibility
   before taking the checkpoint.
3. The first P4 multiplier was divisible by seven and collapsed its intended
   pseudorandom placement to one arena. The corrected deterministic hash uses
   all seven arenas. The final matrix is the corrected execution.

None was a physical counterexample or substrate-law change.

## Mechanical cost boundary

Arena hops, merge lookups, active-arena frontier size, resident capacity,
queue operations, comparisons, allocations, bytes touched, and elapsed time are
recorded as mechanics only. Aggressive partitioning is intentionally slow in
this proof implementation; routing optimization was out of scope.

## E2B provenance

- Initial compile/format: `izio7nnaht7fxd53gjh2j`
- Positive corrected matrix: `i45dqsmk9bz7h1sb9tgbp`
- Final consolidated validation: `iidxzbmzqczwmm891ele5`

No Rust command ran locally.

## Frozen hashes

```text
core lib.rs       33a8882ba83f2e464f6d3d5476b1dec94dfd0c8b7febcf05c517e0eb2bdd1e58
mechanics.rs      ba81648a0318aedfbf90fe968ca51bdcb7efaddf844c0967887fb35a3f6d69be
matrix CSV        cf19de011d02dacb84cb0fc7d8a67e5472b0ad4fa152fa45418e5bccfe397111
matrix report     2e5d0b0fa697cd36df13f68ded950dfb12040a6c7dcacee1335f798b54610ef9
```

## Boundary

R6 does not implement or claim storage, non-residence, loading, transport,
eviction, fetch/prefetch/pin operations, routing optimization, or physical I/O
latency. Organism authority is not advanced. R7 has not started.
