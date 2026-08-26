# SV1 compartmentalized signed variation protocol v1

Status: frozen before any SV1 evaluator or executable change.

Parent: SV0 stopped-negative result `9ad6a5f`.

## Question

Can ordinary local variation generate positive and negative Drive possibilities
in separate ordinary CELL contact compartments, so ordinary CPC0 locality lets
consequence select between them?

The required physical organization is:

```text
             C+ --(+1)--> X+
            /
P ----------
            \
             C- --(-1)--> X-
```

not the SV0 shared-contact pair:

```text
P/C --(+1)--> X
P/C --(-1)--> X
```

## Static Gate A — topology support

Before constructing any selection world, audit whether the accepted ordinary
variation machinery itself can create the required ordinary contact topology.
Gate A passes only if variation—not evaluator construction—can create the
ordinary `C+` and `C-` CELL compartments and the corresponding weak signed
ARROW opportunities without a sign preference, selected candidate identity,
or special inhibitory-contact operation.

Existing generic CELLs placed by the evaluator do not satisfy this gate.
Preplacing `C+` and `C-` would supply the attribution resolution whose
development SV1 is intended to test.

If variation only proposes ARROWs among already-existing CELLs, SV1 stops
static negative. It must not add CELL proposal, contact budding, inhibitory
contact construction, or a hidden topology template inside SV1.

## Blocked runtime gates

Only after Gate A passes may SV1 test:

- symmetric contact creation except for signed coupling;
- positive-only and negative-only local consequence selection;
- identity, slot, and position permutation;
- neither-useful and both-useful controls;
- deliberate shared-contact reproduction of SV0;
- bounded nonrecursive contact variation;
- exact Reference/Production history, replay, and quiescence.

No gate may use a preferred sign, winner flag, selected Arrow/Cell ID,
sign-specific Modulation, reward, or evaluator-supplied contact topology.

## Decision

A Gate-A negative claims only that current variation lacks contact-compartment
construction support. It does not choose or authorize a new CELL-variation
law. RS2 is not rerun inside SV1. FD2, ARC, authority, the oracle, and
`arch.md` remain unchanged.
