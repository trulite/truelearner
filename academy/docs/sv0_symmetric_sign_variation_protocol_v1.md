# SV0 symmetric sign variation protocol v1

Status: frozen before any SV0 Rust or evaluator change.

Parent: RS2 Gate-A negative `af570f7`.

## Question

If ordinary Drive coupling is already a signed physical quantity, can local
variation expose weak possibilities on both sides of zero without knowing
which sign will be useful?

## Sole candidate law

For each otherwise-eligible local source-to-target opportunity, replace the
single weak proposal with the unordered physical alternative set:

```text
Drive coupling +1
Drive coupling -1
```

Both alternatives have identical source, target, absolute local distance,
delay, phase, resistance, lifetime, trigger, and proposal condition. Both are
created; there is no sign choice, preferred sign, random search, activity
inspection, target inspection, consequence inspection, or evaluator input.

The change is feature-gated as `sv0 = ["pd1"]`. It changes proposal support
only. Continuous participation, consequence-supported resistance, local
forgetting, transmission, firing, and all mechanics remain unchanged. CE0
efficacy plasticity is not enabled, so accepted coupling values remain `+1`
and `-1` throughout SV0.

## Frozen worlds

Use two fresh identity roots, absolute start ticks `0,3,7`, two translated
position origins, Reference and Production mechanics, and exact
same-mechanics replay. Each world contains one local source-to-target
opportunity. The target is across an ordinary region boundary and has a high
threshold, so both signed candidate traversals are visible as ordinary
crossings without either candidate firing it.

The external world may return a physically identical Modulatory consequence
after observing a selected crossing sign. The mapping is private to the world;
the organism receives only ordinary physical crossings and the later ordinary
consequence. Candidate IDs, ordering, topology, and organism inputs before the
consequence remain unchanged when usefulness is swapped.

### A — symmetry without consequence

One opportunity must create exactly two proposals and two traversals. The
candidate specs must differ only in coupling sign. With no consequence they
must have equal resistance, participation evolution, decay load, lifetime,
and deallocation age across identities, phases, positions, and mechanics.

### B — positive-world selection

Only the `+1` crossing is mapped by the external world to a consequence.
Required positive result:

```text
+1  supported, resistance matures, remains live at the retained probe
-1  unsupported, resistance remains weak, locally forgotten
```

### C — negative-world selection

The same world and candidate IDs/order are used, but only the `-1` crossing is
mapped to consequence. The required result is the exact sign-swapped state.

### D — usefulness permutation

B and C must have identical physical history through candidate traversal.
After the external mapping changes, learning must follow consequence rather
than ArrowId, insertion order, or sign preference.

### E — neither useful

Both candidates traverse repeatedly, but the world returns no consequence.
Neither may gain durable resistance. Continued use alone must not prevent
eventual local forgetting.

### F — both useful

Both crossings are mapped to genuine consequences. Both candidates may mature
and remain live; no winner-take-all condition is imposed.

### G — bounded variation

One local opportunity creates exactly two weak structures. Repeated external
activation while either alternative remains live creates no additional sign
variants. After both unsupported alternatives are reclaimed, a fresh local
opportunity may create exactly one new pair. Record proposal count, peak live
weak structures, PhysicalWork, and eventual reclamation. The frozen work
ceiling is 256 per finite world.

## Hard equivalence and static gates

Require exact Reference/Production and replay agreement on ordered physical
transitions, signed Drive deliveries, crossings, proposals, participation,
resistance changes, deallocations, durable body, clock, quiescence, and
PhysicalWork. ExecutionCost remains non-physical.

Static audit rejects any equivalent of:

```text
oscillation or activity inspection
target-state or threshold inspection
preferred or random sign
reward, error, stability score, cycle detector
sign-specific learning, forgetting, or lifetime
recursive sign-variant proposal
```

## Decision

- **SV0 positive:** all gates pass. Existing consequence learning, not proposal
  sign or candidate identity, selects which symmetric possibility persists.
- **SV0 negative:** stop at the first frozen failed gate. Do not repair
  attribution, gating, coupling learning, or variation inside SV0.

RS2 Gate B is not rerun inside SV0. FD2, ARC, authority, the oracle, and
`arch.md` remain unchanged.
