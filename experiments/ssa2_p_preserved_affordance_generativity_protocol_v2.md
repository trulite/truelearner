# SSA2-P preserved-affordance generativity protocol v2 amendment

Status: **development preregistration; no definitive execution authorized**.

This amendment preserves protocol v1 and its immutable PROBE negative. It does
not reinterpret or rerun that artifact.

## Mechanically demonstrated v1 mismatch

PROBE v1 produced:

```text
16/16 length-8 trajectories valid
8/8 distinct trajectories per cell
8/8 distinct trace fingerprints per cell
all physical controls passed
```

Its sole failed conjunct was `both physical sides at every layer`.

Protocol v1 assigned the first six layer choices from the six low bits of
history `h`, but PROBE uses only histories `0..7`. Bits 3, 4, and 5 are
therefore identically zero by construction. No possible implementation could
satisfy the preregistered coverage control with that library.

This is a deterministic schedule-coverage error, not evidence about Frozen
Organism v1 or the physical trajectory mechanism.

## Sole amendment

Let `k = log2(history_count)` for the fixed power-of-two stage libraries:

```text
PROBE  histories=8   k=3
MICRO  histories=32  k=5
GATE   histories=64  k=6
```

Replace the v1 history-side function with:

```text
bit(h,d) = bit (d mod k) of h
```

This cyclic deterministic word guarantees both physical sides are represented
at every layer across each complete history library, while retaining exactly
`8`, `32`, and `64` distinct transient histories. It is fixed before v2
evidence and contains no RNG, feedback, route sampling, or organism-visible
bit representation.

## Everything else remains exact

Unchanged:

- Frozen Organism v1 and ordinary blank-start `[4,4]` development;
- the connected single-propagation CELL/ARROW/SPIKE world;
- transition topology;
- early/late ticks `6/7`, threshold, and inhibition;
- PROBE/MICRO/GATE depth, histories, and cell counts;
- diversity and validity thresholds;
- mirrors, identities, layouts, handles, collapsed, blocked, broken-arrow, and
  replay controls;
- all anti-cheat exclusions and development classifications;
- no definitive execution and no SSA1 reinterpretation.

Result artifacts from v2 must use separate filenames. If any v2 scientific or
physical control fails, freeze it without further schedule or mechanism change.
