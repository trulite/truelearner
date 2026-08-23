# PX4 substrate-native learned-lifetime development protocol

Protocol identifier: `px4-physical-lifetime-development-v1`

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX4 NON-AUTHORITATIVE**.

## Frozen start and authority boundary

This independent development lane starts exactly at authoritative PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
`px2-physical-causal-direction-authoritative`.

| frozen ancestor artifact | SHA-256 |
|---|---|
| PX0--PX2 active substrate law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| PX2 definitive Markdown | `eef9c336baea6aa1e5c2debde2e1286b2839759c55fd5fc008c7775fd4103cda` |
| PX2 definitive result audit | `7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe` |
| PX2 authority handoff | `98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509` |

This protocol authorizes development-only PROBE, MICRO, and GATE evidence. It
does not authorize a definitive matrix, a new authoritative ancestor, changes
to PX0--PX2, or work on PX3 or PX5--PX8.

The old typed M4/DS6 artifacts are behavioral reference only. No old M4 type,
record, signature, strength map, schema, code path, or evidence is linked or
executed.

## First question and zero-mechanism hypothesis

The first question is whether the target is already present in PX0--PX2.
Authoritative substrate physics already supplies the mechanically unique
candidate edge:

```text
actual ARROW traversal followed by local returned SPIKE activity
        -> that live ARROW's scalar resistance increases

elapsed physical time and unsupported traversal
        -> ordinary pressure lowers resistance

zero resistance
        -> ARROW becomes non-live and its generation advances

later ordinary external firing
        -> local structural proposal can form a fresh ARROW
```

The zero-mechanism hypothesis is therefore:

> Use-dependent persistence, disuse, contradiction-driven replacement, true
> forgetting, stale-path refusal, and contemporary reacquisition already
> emerge from authoritative PX0--PX2 resistance, recurrence, actual
> participation, ordinary returned activity, pressure, competition,
> deallocation, and reproposal.

No substrate change is authorized. If this hypothesis fails at a physical
edge, the negative is frozen and the lane stops; no repair is authorized by
this protocol.

## Physical construction

Every cell begins as a fresh blank `PlasticSubstrate`. It first acquires two
ordinary PX0 correspondences. It then exposes two weak anonymous PX2
continuation ARROW opportunities. Actual traversal and ordinary return may
protect one or both opportunities. Evaluator-described alternatives differ
only in physical activity, timing, placement, allocation order, and arrival
insertion order.

Organism-visible execution consists only of retained CELL, ARROW, SPIKE,
resistance, coupling, eligibility, generation, queue, and pressure state in
the byte-frozen PX0--PX2 substrate law. Scenario names, expected sides, result
aggregation, and file serialization remain outside the organism.

## Required physical clauses

The staged evidence must expose each clause independently:

1. **use-dependent persistence** -- more actual traversals with ordinary
   return leave greater resistance and survive a matched later pressure load
   that removes a weakly used alternative;
2. **disuse** -- without subsequent traversal/return, resistance is
   nonincreasing under ordinary elapsed-time pressure and reaches zero;
3. **contradiction through contemporary competition** -- after one direction
   is learned, opposite contemporary traversal protects a newly available
   path while the unused old path experiences only the ordinary pressure
   applied to every allocation;
4. **true forgetting** -- the old directional ARROW and its acquired PX0
   correspondence are non-live at zero resistance and have advanced
   generations;
5. **stale-path refusal** -- held-out activity after deallocation produces no
   effect through the old direction, including when old ArrowIds remain as
   implementation tombstones;
6. **contemporary reacquisition** -- ordinary local proposal creates fresh PX0
   correspondence ArrowIds and contemporary traversal/return protects fresh
   directional ArrowIds; no historical path is reinstated;
7. **correlation control** -- matched consequence activity without candidate
   traversal does not preserve a directional path;
8. **return-absent control** -- traversal without the ordinary return route
   does not preserve a directional path;
9. **no hidden boundary** -- changing evaluator grouping while keeping the
   same organism-visible arrivals and physical time leaves final state exact;
10. **replay, quiescence, work, and storage** -- every development is exactly
    repeatable from blank, every propagation becomes naturally quiescent, and
    work counters, live allocations, total allocation slots, persistent bytes,
    generations, and complete-state fingerprints are serialized.

No evaluator delete, special contradiction decrement, reinstatement branch,
future-use input, task-boundary cleanup, or lifetime-specific update is
allowed.

## Staged matrices and fresh identities

### PROBE

One normal-layout cell for each of six cases:

1. matched high-use survival;
2. matched low-use forgetting;
3. disuse trajectory to zero;
4. forward-to-reverse contemporary competition;
5. correlation without traversal;
6. traversal without return.

Namespaces begin at `0x6_4400_0000`, stride `0x0010_0000`. Each cell has an
independent exact duplicate at namespace `cell + 0x0008_0000`.

### MICRO

Repeat all six cases in four fresh layouts (`24` cells, `48` developments):

| layout | side spacing | traversal delay | first activity | placement | allocation | arrival insertion |
|---|---:|---:|---:|---|---|---|
| M0 | 34 | 3 | 71 | normal | normal | normal |
| M1 | 46 | 4 | 73 | mirrored | reversed | reversed |
| M2 | 58 | 5 | 79 | normal | reversed | normal |
| M3 | 74 | 6 | 83 | mirrored | normal | reversed |

Namespaces begin at `0x6_5400_0000`, layout stride `0x0100_0000`, case stride
`0x0010_0000`; duplicates use `+0x0008_0000`.

### GATE

Run eight cases in four new layouts (`32` cells, `64` developments): the six
cases above plus reverse-to-forward contemporary competition and full
deallocation followed by opposite contemporary reacquisition.

Gate layouts use side spacings `38, 52, 68, 86`, traversal delays `3, 4, 5,
6`, first-activity ticks `89, 97, 101, 107`, both placement orientations,
both allocation orders, both arrival insertion orders, and ordinary active
distractor loads `0, 8, 24, 48`. Namespaces begin at `0x6_7400_0000`, with the
same layout/case/duplicate strides as MICRO.

No identity overlaps PX0--PX2 evidence or another stage.

## Stage gates and stopping rule

PROBE passes only if all `6/6` cells and every applicable physical clause pass.
A positive PROBE makes MICRO executable. MICRO passes only if all `24/24`
cells pass and makes GATE executable. GATE passes only if all `32/32` cells and
all controls pass.

Each stage has fresh write-once CSV and Markdown outputs published atomically
from staging paths. A failure, including an accounting-only failure, is
committed and preserved unchanged. No failed stage may be rerun or rescued.

A positive GATE establishes only PX4 development readiness and the
no-new-mechanism result. It does not create PX4 authority or authorize any
later generation.

## Forbidden organism machinery

The active development source and substrate may not introduce supplied
lifetime classes, duration counters, aging classes, expiry categories,
retention/deletion policies, semantic enums, serializer-fed execution,
evaluator-selected paths, hidden task boundaries, adapters, typed intermediate
representations, or renamed equivalents.

The implementation must hash-audit the frozen ancestor, scan the new active
source and substrate information flow, refuse a definitive flag, and refuse
to overwrite prior evidence.

## Later-porting contract

If GATE is positive, the frozen result is a zero-mechanism contract: when the
serial authoritative ancestor reaches PX4, port the development harness and
its expectations unchanged first, execute it against the then-current
substrate without adding machinery, and diagnose only the parent delta. This
lane's development evidence is not itself authority evidence and must not be
reused as a definitive matrix.
