# SI0 simultaneous local incidence protocol v1

Status: frozen before any SI0 substrate or evaluator change.

Parent: RI0 real renaming-invariance negative `a263096`.

## Question

Can Drive physics distinguish genuine simultaneity from causal succession so
that opaque handles and insertion order no longer decide organism behavior?

SI0 proposes one new physical affordance and excludes Modulatory arrivals.

## Candidate law

At one live generation-safe CELL junction, all valid Drive arrivals sharing:

- physical tick;
- phase;
- causal wave;
- target junction;

form one local incidence. Their signed impulses combine. Existing CELL state is
updated once, threshold/refractory is evaluated once, and the CELL fires at
most once.

Firing creates new arrivals in the next causal wave, including zero-delay
arrivals at the same tick and phase. A causal wave is derived only from event
generation:

```text
wave 0   admitted/current external incidence
wave 1   events caused by wave 0 firing/traversal
wave N   events caused by wave N-1
```

Serial remains available for recording/canonical representation but cannot
change a simultaneous local incidence. No CELL, ARROW, physical identity,
slot, arena, insertion order, or serial value may define causal wave.

## Frozen families

1. same junction `+2/-1` under all arrival orders and handle renamings;
2. threshold composition: `+1+1 @ T2` fires once, `+2-1 @ T2` does not fire,
   `+2+1 @ T2` fires once;
3. pre-existing CELL activation plus simultaneous incidence;
4. independent same-tick/phase junctions under reversed execution/insertion;
5. parallel arrows under reversed ARROW creation;
6. CELL/ARROW/source/target handle bijections normalized back to logical names;
7. zero-delay A-to-B chain, where B executes in the next wave;
8. zero-delay fanout/merge, where same-wave arrivals at D combine locally;
9. zero-delay A-to-B-to-A cycle with no cycle detector, TTL, or maximum-wave
   physical rule;
10. Reference and Production equivalence after logical renaming.

The evaluator records complete ordered transitions annotated with causal wave,
firing, signed deliveries/incidences, durable/future-causal state, Work, clock,
pending activity, replay, and natural quiescence.

## Hard controls

- Permuting serial assignment among simultaneous arrivals cannot change output.
- Different-junction order is inert unless one junction physically causes a
  later-wave arrival at the other.
- Zero-delay consequences never join their cause's incidence.
- A zero-delay cycle must quiesce from existing threshold/refractory/local
  physics or SI0 stops negative. Observation ceilings diagnose but are not
  physics.
- Mixed Drive/Modulatory incidence is outside scope and makes no claim.
- Canonical serialization may sort handles; causal execution may not.

## Representation gate

Reference and Production must agree after logical renaming on physical
transitions, incidence/firing history, durable and future-causal state,
PhysicalWork, clock, pending activity, replay, and quiescence. ExecutionCost is
excluded.

Any failure stops SI0 without repair or rerun. SI0 does not rerun RS2 and does
not advance CE1, FD2, ARC, authority, oracle status, or `arch.md`.

