# CORE1-E13-D — composition depth and lifetime boundary protocol v1

## Status

Evaluator-only successor to the frozen CORE1-B E13 composition result
`32cd8e4`. The CORE1-B runtime, material laws, PQLC trigger, variation law,
causal-wave semantics, and two-experience developmental count are immutable.
No ARC task is executed in this gate.

## Question

How far does the already-established local consequence composition remain
available as ordinary physical burdens increase?

The primary measurement is the first point where downstream consequence no
longer consolidates every still-required Drive link back to the source.

## Fixed E13-B geometry

Every world uses only ordinary CORE1-B material:

```text
source -> C0 -> C1 -> ... -> Cn -> motor -> outward boundary

return --Modulatory/SourceFires--> motor
motor  --Modulatory/QLP---------> Cn
Cn     --Modulatory/QLP---------> ... -> C0 -> source
```

The forward route and its QLP relay links receive the case's same initial
resistance. Boundary anchor links are ordinary high-resistance fixtures and
are identical in every world. The forward Drive links have coupling `+1`.

Every case receives exactly two supported experiences:

```text
forward use -> fixed delay -> one ordinary returning consequence
one recovery tick
forward use -> fixed delay -> one ordinary returning consequence
```

The evaluator never trains until success.

## Frozen axes

Only one axis varies in each family:

| Family | Depth | Resistance | Return delay | Signed noise pairs |
|---|---:|---:|---:|---:|
| depth | `1,2,4,8,16,32,64,128` | 8 | 4 | 0 |
| lifetime | 8 | `1,2,4,8,16,32` | 12 | 0 |
| delay | 8 | 8 | `0,1,2,4,8,12,16,24,32,48,64,96` | 0 |
| variation | 8 | 8 | 4 | `0,1,2,4,8,16,32,64` |

A signed noise pair is two ordinary contact branches sharing the source and
motor, one ending in `+1` Drive and one in `-1` Drive. Their simultaneous net
motor effect is zero. Each has the same local PQLC topology as any other
contact. This is a fixed physical load equivalent to products of CORE1-B's
generic signed edits; it contains no task label or preferred sign.

## Measurements

For every case record:

- action crossings for each experience;
- live desired Drive and QLP links before consequence;
- participation at consequence;
- QLP traversals;
- number of desired links whose resistance increases;
- downstream-to-source supported depth;
- unrelated/noise links that also consolidate;
- deallocations, final tick, PhysicalWork, natural quiescence;
- canonical final body hash and ordered physical trace.

`full_closure` means every desired Drive link increased resistance and the
complete desired QLP chain traversed in both fixed experiences. A failed case
is recorded, not repaired.

## Representation gate

Each case runs as:

```text
Reference
Reference exact replay
Production
```

Reference replay and Reference/Production must match exactly on the frozen
physical observation. ExecutionCost is excluded.

## Prohibitions

No mechanism or evaluator branch may introduce a path ID, predecessor lookup,
depth counter in organism physics, backward execution mode, credit packet,
special long-delay protection, adaptive lifetime, train-until-success,
noise suppression, ARC accommodation, or runtime-law change.
