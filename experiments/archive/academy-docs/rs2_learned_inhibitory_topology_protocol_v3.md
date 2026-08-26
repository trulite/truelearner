# RS2 learned inhibitory topology protocol v3

Status: frozen before any v3 evaluator or executable change.

Parent: RS2 v2 fixture negative `afcb073`.

## Scope

RS2 v3 changes only evaluator geometry. The cumulative CV0/J0 organism,
variation, participation, consequence, resistance, coupling, and lifetime laws
remain byte-identical.

## Training

B and A use threshold 2. The external training arrival supplies impulse 2 to B,
so B fires and ordinary variation constructs the signed candidates:

```text
B -> C+ ->(+1) A
B -> C- ->(-1) A
```

Both contact junctions receive the ordinary +1 stem traversal and fire. Their
outgoing ±1 effects remain subthreshold at A, so A cannot fire or extend the
boundary tail. Qualified Modulation is preregistered at age 2 and reaches only
the selected generated contact. The selected links consolidate; unsupported
and irrelevant links decay and their orphan junctions disappear.

## Probe

After training reaches age 10 and no further Modulation is admitted, ordinary
high-resistance probe topology is added:

```text
A -(+1)-> U -(+2)-> B -(+1)-> W -(+2, phase 1)-> A
                         \
                          -> learned C- ->(-1, phase 0)-> A
```

An external impulse 2 makes A fire. A's forward route makes B fire once. B's
learned negative relation and ordinary positive return reach A at the same
tick; the negative phase-0 effect precedes the positive phase-1 effect, leaving
A at +1, below threshold 2. The intended forward activity therefore completes
and the recurrence settles without changing learning.

Without a traversed connected learned negative relation, the +2 return makes A
fire again and the frozen observation ceiling classifies persistent recurrence.

## Frozen families

The nine RS2 v2 families and hard controls are unchanged:

1. learned negative stabilizes;
2. no qualified consequence;
3. candidate identity/sign-order permutation;
4. location translation;
5. irrelevant negative candidate;
6. useful positive control;
7. learned negative disconnected;
8. learned negative present but untraversed;
9. fresh recurrence packing.

The useful positive control uses an independent impulse-2 acyclic probe and
requires the retained +1 relation to deliver its physical effect without any
sign preference claim.

## Gates

- training A never fires before the consequence;
- the consequence is admitted at exactly age 2 in every training world;
- no anchor link traverses or consolidates during training;
- the selected relation alone survives ordinary decay;
- connected/traversed learned negative topology settles the probe after A and B
  each fire once;
- disconnected or untraversed learned negative topology cannot settle it;
- no Modulation means no selected topology;
- useful positive topology can also be selected and re-executed;
- Reference/Production ordered history, PhysicalWork, final/live state,
  replay, and quiescent/ceiling classification are exact.

Any failure stops without repair. Positive RS2 permits the preregistered CE1
successor; CE1 failure blocks FD2 and ARC.

