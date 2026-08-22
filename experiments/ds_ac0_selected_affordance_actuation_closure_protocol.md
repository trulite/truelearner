# DS-AC0 selected-affordance actuation closure

Status: **PREREGISTERED; DEVELOPMENT OUTCOME UNSPENT**

Exact parent: `dbf630b85ef5b01f42d734c6195077ce5bbe5604` /
`ds2-cumulative-m1-mechanistic-probe-collapse-handoff`.

Authoritative cumulative ancestor remains M1 at
`16a1002b59bf0dbc23a6b6bf03572efca53b33ce`.

## Sole question

> Can the affordance selected by byte-identical frozen DS1 actuate its already
> existing A1 executable root so that ordinary physical substrate state—not the
> choice token—produces the returned aftermath?

AC0 adds no action representation, causal learner, consequence learner,
direction label, evaluator action, or semantic feedback. It closes one physical
edge between two frozen mechanisms.

## Frozen path

```text
frozen DS1 choice slot
    -> one-to-one copy of the corresponding existing opaque A1 handle
    -> existing A1 root
    -> root SPIKE and live CELL/ARROW propagation
    -> physical substrate-state delta
    -> anonymous aftermath derived only from that delta
```

The slot-to-handle bridge may copy an existing association. It may not create a
route, inspect its semantics, rank alternatives, interpret the handle as an
operation, or manufacture an effect.

The aftermath former's only behaviorally relevant input is the physical state
delta produced by route execution. It may not receive the DS1 choice, handle,
root, expected effect, evaluator role, world-affordance label, correctness, or
control arm.

## Development gates

1. Exact M1, A1, DS1, and stage-3 collapse fingerprints match.
2. Frozen DS1 supplies a mature held-out choice without evaluator input.
3. Two already-existing A1 roots and opaque handles exist before selection.
4. The choice-to-handle bridge is one-to-one and format-only.
5. The selected handle fires its existing root and traverses live A1 arrows.
6. Anonymous aftermath is produced solely from the resulting physical state.
7. The two selectable roots produce distinct physical aftermaths.
8. All anti-cheating controls pass.

Required controls:

- choose A but physically block A: no A-dependent aftermath;
- permute opaque handle values: the associated physical root, not handle value,
  determines aftermath;
- change B's temporary binding: the changed physical B state determines the
  changed aftermath;
- skip route execution: no route-dependent aftermath;
- stale handle: no execution and no aftermath;
- fresh occurrences and allocation/layout perturbation preserve the relational
  result;
- no persistent occurrence, handle, root, destination, or effect is retained.

## Stopping boundary

AC0 is enabling-only. A positive does not advance beyond M1 and does not claim
causal-direction reconstruction. Once frozen, rerun the unchanged cumulative
DS2 mechanistic probe with AC0 as an enabling ancestor and stop at its first new
missing path. No definitive artifact is authorized for AC0.
