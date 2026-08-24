# Unchanged DS1-on-DS-E0 leak audit

Outcome: **PASS TO FIRST COLLAPSE**. No isolated fixture, evaluator route
adapter, action semantics, or consequence rescue exists in the composition
wiring.

## Call and data boundary

The new harness makes one mechanism call: `ds_e0::run(mode)`. It does not have
access to E0 raw activity, evaluator membership, candidates, `EventRelations`,
`Neighborhood`, `Learner`, a route index, or consequence callback. E0-A,
E0-B, and the marked learner remain private and byte-identical to `d154fde`.

The existing frozen E0 probe is the only stage-3 path:

```text
formed E0 EventRelations -> unchanged serialize_once -> Neighborhood
-> unchanged Learner::frozen_choice (read-only)
```

It never calls `choose` or `apply_consequence`. The composition harness stops
at stage 4 rather than manufacturing the actions those calls would require.

## Forbidden-channel audit

- no adapter or route target;
- no source/target, boundary label, consume/produce, input/output, or action
  semantics;
- no filler equality, `SAME`, stable filler/port/occurrence token, event truth,
  window truth, or evaluator membership;
- no expected action, answer, economics, pass/fail input, or fallback;
- no isolated DS1 synthetic two-route fixture;
- no manufactured anonymous action alternatives;
- no interpretation of choice index 0/1 as correctness;
- no evaluator ordering or correctness used as consequence;
- no evaluator truth used to create route-contingent consequence;
- no synthesized route effects, acquisition history, or terminal wiring.

The new source contains no evaluator callback or environment action call.
Per-seed action availability is derived from `ActionSurfaceInventory`; there
is no literal `false` in `SeedCompositionAudit` construction. All causally
later values are `Option::None` and render as blank.

The inventory mechanically extracts exact frozen signatures and type surfaces.
It distinguishes E0's formation callback
`FnMut(&[Occurrence; 3]) -> bool` from a post-DS1-action consequence; the
callback consumes a proposed current membership and returns only E0 formation
credit. It also distinguishes M0's private `Resolution`/`ArrowStore` execution
surface from a DS1 action surface: none of the three M0 execution signatures
accepts `Neighborhood`, `choice: usize`, or an action pair. The only public E0
composition outputs are two owned report structs with no borrowed action
handle, callback, or pair value.

The source audit checks that the frozen E0 source still contains exactly one
serializer and the existing read-only probe, and that the new wiring contains
no isolated-fixture/expected-action identifiers. External SHA-256 checks are
the authoritative byte-continuity proof.

Two focused amendment tests freeze the inventory counts and require that stage
4 cannot report absence if an exported action pair, DS1-compatible M0 execution
surface, choice-to-execution edge, or natural consequence edge appears.

## Non-claims

DS-E0 evaluator-side formation outcomes are reported only after the frozen
path runs. They are not used to create DS1 alternatives or credit. No DS1
choice fired, no consequence returned, no learner acquisition occurred, and
no correctness or recovery result exists.
