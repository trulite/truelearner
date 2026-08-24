# Physical body V1 implementation protocol

## Parent

Implementation begins from the behavior-preserving relocation baseline
`026b51b` on `runtime/truelearner-redesign`. The retained scientific parent is
PX-C authority `ec87c438aa8c52389fd2734667363ef43acaef93`.

## Purpose

Define the organism's physical body independently of its storage location and
prove that canonical persistence, resident packing, restart, and machine
representation cannot silently change PX-C physics.

## Frozen architecture

### Identity

Stable identity is `(ArenaId, EntityId, Generation)`. `CellId` and `ArrowId`
are stable within an arena. `CellSlot` and `ArrowSlot` are disposable dense RAM
locations. Resolution is explicit:

```text
resolve(CellRef)  -> Option<CellSlot>
resolve(ArrowRef) -> Option<ArrowSlot>
```

Compaction may change slots but must not change stable references or behavior.

### Body and execution

- An `ArenaBlock` is immutable canonical durable CELL/ARROW structure and has
  no notion of now.
- A `BodyVersion` is a manifest mapping stable arena identity to immutable
  block content hashes. It promises structural identity, not temporal
  continuation.
- A `ResidentArena` executes explicitly in mutable RAM.
- Directly mutable mmap state is forbidden in V1.

### Time and restart

- A `PhysicalClock` contains the minimal state required by physical laws.
- Pressure phase is derived from physical tick when possible and must not be
  stored as conflicting duplicate state.
- A `QuiescentCheckpoint` contains a body version plus physical clock and
  resumes exact behavior from quiescence.
- A `LiveCheckpoint` additionally contains activation, refractory state,
  eligibility, participation traces, timing rings, outstanding arrivals, and
  pending loads required for exact continuation.

### Storage availability

Host completion time is not organism history. A deterministic quantization
boundary admits completion as a physical availability tick. After admission,
replay consumes that tick. A pending load records arena version, issue tick,
known availability tick when admitted, and waiting physical arrivals.

### Format

The canonical format fixes integer widths, little-endian encoding, section
order, offsets, alignment, padding, flags, validation, checksums, and a whole
block content hash. It is not a dump of Rust memory.

`arena-format` knows identity, durable representation, canonical bytes, hashes,
and validation. It knows nothing about firing, learning, modulation, pressure,
significance, or experiments.

## Immediate production crates

```text
truelearner/crates/core
truelearner/crates/arena-format
```

No additional production crate is authorized by this protocol.

## V1 gates

1. Retained PX-C behavior remains exact.
2. No heap allocation occurs per CELL or ARROW.
3. Arena capacity is bounded and reuse deterministic.
4. Stale generations cannot execute after reuse.
5. Encode/decode preserves every durable field.
6. Decode/encode reproduces identical canonical bytes.
7. Equivalent bodies produce identical content hashes.
8. A quiescent checkpoint reloads with identical subsequent behavior and
   physical clock phase.
9. A live checkpoint resumes identical queued execution, including admitted
   load availability.
10. Corrupt, truncated, overlapping, or incompatible blocks fail closed.
11. Runtime execution has no dependency on `experiments/`.
12. No directly mutable mmap state exists.
13. Compaction invariance: alternative resident slot packing with identical
    stable IDs produces exact physical behavior and output.

## Out of scope

- NVMe arena cache and eviction.
- Network transport or replication.
- Organism-visible fetch/prefetch/pin affordances.
- Distributed causal journaling.
- Promotion to `main`.
