# PX4 physical learned-lifetime development-readiness handoff

Status: **DEVELOPMENT READY; CLASS A; AUTHORITY ABSENT**.

This handoff is not an authority workflow, a definitive matrix, or a new
authoritative ancestor. It does not advance PX3 or PX5--PX8.

## Lane outcome classification

**A = existing PX0--PX2 physics sufficient.**

The result is not between classes. No physical edge is missing, no competing
scientific mechanisms remain, and no new representation or substrate law is
required for the tested learned-lifetime capability.

The complete staged result is:

- PROBE v1: preserved `0/6` mechanical/accounting negative, commit/tag
  `709d9ba86a961f8560928928b5a0ffeb6001a12a` /
  `px4-physical-lifetime-probe-v1-negative`;
- PROBE v2: `6/6` positive, commit/tag
  `0de2fd85b7e255a633c959d782b45bd8051a4c2d` /
  `px4-physical-lifetime-probe-v2-positive`;
- MICRO v1: `24/24` positive, commit/tag
  `53bfef088140b109b7c6743e13e70b46417cf80c` /
  `px4-physical-lifetime-micro-v1-positive`;
- GATE v1: `32/32` positive, commit/tag
  `03fa0d04b82538c5625130e5e7c5dfb2235d442f` /
  `px4-physical-lifetime-gate-v1-positive`.

## Frozen mechanism: no-addition result

No PX4 mechanism source was added. The sufficient mechanism is the existing
PX0--PX2 physical law at:

- commit: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- tag: `px2-physical-causal-direction-authoritative`;
- source: `crates/px0-physical-correspondence/src/lib.rs`;
- full source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The exact frozen blocks constituting the no-addition result are line-numbered
against that source at the frozen commit:

| physical block | lines | SHA-256 |
|---|---:|---|
| local window, returned strength, unsupported-use pressure, ordinary-pressure period, variation radius, coupling ceiling | 9--14 | `f7bc06909711c7f19f38e6d936cc6fd59b12539bd958ab311db92ff49f542dc1` |
| elapsed-time entry and work accounting | 234--244 | `53b33f36e301758a5dc26911436c3b7c5e51ffb191db63f3355fd2b1ba803397` |
| queue propagation, generation refusal, threshold firing, outward crossing, eligibility, and SPIKE emission | 246--357 | `c857ea85910b1f2663d7dc7e41440ac14ae7383b6b3156a5d33d451fb11a69a5` |
| live/resistance/slot/generation observations | 372--390 | `229ce11f03146d9c47cf791798c342fe4ac2629619335475914577717177f6a1` |
| actual local return increases resistance/coupling | 409--424 | `dde858c9064dae68045793eb4f9437819e9a3cec539516b0223a0a614abe5f1f` |
| ordinary elapsed and unsupported-use pressure | 426--454 | `9af24de6e770d1f15903740a49acd11bcc92e37ef34422b47eb1a2fa8f6092c9` |
| local structural reproposal | 456--492 | `8aa30e34e2baca922a59e07d88ddf373430a05eafc156bf230eba48f4838961d` |
| zero-resistance non-live transition and generation advance | 611--618 | `ab70650bc87085c996afc3a2c35f93646a69a2b1dc13c52de5e74792458014fc` |

The frozen development harness is evaluator-only evidence code:

- commit/tag: `df7e2a0cc039c1b64bb3c83c5adcceaa87b30638` /
  `px4-physical-lifetime-probe-v2-implementation`;
- source:
  `crates/px0-physical-correspondence/examples/px4_physical_lifetime.rs`;
- SHA-256:
  `801944549876a6cc0a828cec5d9590b0df6d5700bd999621bfe4d2df231299cc`.

The final GATE evidence is commit/tag
`03fa0d04b82538c5625130e5e7c5dfb2235d442f` /
`px4-physical-lifetime-gate-v1-positive`; its CSV/report hashes are
`9c5524cc8c5da3d987bc03817d165d873e7ccac75dfb16d986a5bc28afa483c5`
and
`b1d85e29a2881db61ba4e9ec77f38a40c606c4df20503f4d17e735fb4748495d`.

## Consumes physically

The no-addition result consumes these exact substrate facts, not conceptual
memory shorthand:

1. persistent CELL identities with mutable threshold/state/generation/live
   state;
2. directional ARROWs with local `from/to`, delay, phase, coupling,
   source-generation, generation, scalar resistance, live state, and a
   four-tick local traversal eligibility trace;
3. fresh SPIKE occurrences with arrival tick, phase, physical origin, target,
   target generation, impulse, serial order, and optional traversed-ARROW
   generation;
4. actual queue-ordered traversal of a candidate ARROW from a continuation
   CELL to a consequence CELL;
5. consequence firing into an outward region crossing and into the retained
   PX2 physical return topology: consequence to hub, hub to trace CELL, trace
   CELL back to continuation;
6. returned SPIKE arrival at the continuation while the actually traversed
   ARROW remains locally eligible;
7. ordinary elapsed-time pressure every ten physical ticks and one additional
   ordinary pressure unit when a traversal's local eligibility closes without
   return;
8. local position topology with distance `1..=2` permitting ordinary PX0
   proposal from an externally firing arrival CELL to a neighboring
   correspondence-end CELL;
9. generation checks on queued ARROW and target references, which refuse
   activity after physical deallocation;
10. threshold firing, refractory order, CELL-state decay, inhibitory/ordinary
    closure already present in the frozen substrate; and
11. enough physical work/time for propagation to quiescence and for ordinary
    pressure to act.

The concrete test topology contains, per anonymous side: arrival CELL,
correspondence-end CELL, continuation CELL, consequence CELL, physical trace
CELL, outside CELL, acquisition driver, participation driver, independent
activity driver, and gate; plus shared context and hub CELLs. GATE layouts add
`0,8,24,48` ordinary active driver-to-sink distractor pairs.

## Additional environmental physical opportunities and scheduling

All environmental opportunity supplied by the development fixture is explicit:

- both sides initially receive a physical reserve-3 continuation-to-
  consequence ARROW opportunity at their already-fixed first possible use
  tick;
- contemporary contradiction worlds later receive one fresh reserve-3
  opposite-side ARROW opportunity only after ordinary time has advanced to
  its first possible use tick;
- full-deallocation/reacquisition worlds receive ordinary external arrival
  SPIKEs that allow the frozen local proposal law to form fresh
  arrival-to-correspondence ARROWs, followed by fresh reserve-3 opportunities
  on both anonymous sides;
- ordinary external SPIKE schedules physically drive acquisition,
  participation, matched independent consequence activity, context support,
  and distractors at the exact ticks serialized by the frozen harness;
- held-out execution physically supplies both arrival CELLs and the shared
  context CELL; the substrate, not the evaluator, determines which live path
  crosses outward.

These are world topology/activity assumptions, not organism machinery. The
fixture never passes scenario identity, expected side, use count, result
predicate, or serialized observation into `PlasticSubstrate`. There is no
hidden task-boundary signal or typed construction inside organism-visible
execution.

## Produces physically

Depending only on the physical history, the frozen substrate leaves:

- used candidate ARROWs live with increased scalar resistance and coupling;
  after the matched GATE pressure gap, high-use paths retain resistance
  `11` or `12` and still produce one held-out outward crossing;
- disused/unsupported candidate ARROWs at resistance `0`, `live=false`, with
  generation advanced; their old ArrowId slots remain counted as non-live
  tombstones but cannot emit;
- under contemporary opposite activity, old effects become zero while the
  newly used physical path retains resistance `35` or `36` and produces one
  opposite outward crossing;
- after full forgetting, zero live old acquired correspondence or direction
  paths, zero stale outward effects, one fresh proposed correspondence ArrowId
  per side, fresh direction ArrowIds distinct from the old IDs, and a live
  contemporary reverse path producing effects `0|1`;
- transient trace/return/queue activity during development and held-out
  execution, followed in every propagation by an empty pending SPIKE queue and
  natural quiescence;
- exact work ledgers, pressure/deallocation counts, persistent allocation-slot
  bytes including tombstones, and complete-state fingerprints for audit.

No learned lifetime class, policy record, old M schema, or semantic state is
left for the next PX stage. The next stage can consume only the actual live or
non-live CELL/ARROW state, resistance/coupling/generation, local trace state,
and any later physical SPIKE activity.

## Assumptions

- authoritative PX0--PX2 parent is exactly
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- the active substrate source remains exactly hash
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- every additional environmental physical opportunity and schedule is the
  one explicitly listed above;
- weak physical opportunities are introduced contemporaneously, after prior
  elapsed pressure has been applied, and receive no immunity afterward;
- enough ordinary physical time and work are available for pressure and
  quiescence;
- non-live vector tombstones are storage implementation residue, not retained
  executable structure; and
- there is no hidden fixture scheduling or typed organism construction beyond
  the explicit physical world/activity facts above.

## Forbidden

- typed adapter;
- reconstruction of any old M schema;
- supplied lifetime/duration/aging/expiry class or retention/deletion policy;
- semantic labels or evaluator-selected local execution paths;
- serializer-fed execution, evaluator delete, contradiction-specific
  decrement, future-use input, or task-boundary cleanup;
- importing evidence or mechanisms from another parallel PX lane;
- modification of authoritative PX0--PX2; and
- authority, definitive-matrix, or later-PX execution from this lane.

Evaluator scenario names and expected vectors exist only in the frozen
development harness after/before physical execution boundaries. They are not
organism-visible and cannot select a local executed path; live physical
topology, threshold firing, queue order, and generation checks do that.

## Cumulative unchanged-port rule

When the serial authoritative ancestor eventually reaches PX4:

1. import the frozen no-addition mechanism byte-for-byte onto the later
   authoritative parent -- specifically, preserve the exact physical blocks
   and hashes above and add no PX4 organism machinery;
2. port the frozen development harness and physical expectations unchanged
   first;
3. make no anticipatory redesign;
4. run a physical dependency/path audit from actual CELL/ARROW/SPIKE inputs,
   through traversal, trace/return, resistance, pressure, deallocation,
   generation refusal, reproposal, and outward crossing;
5. treat the first incompatibility as the next cumulative collapse; and
6. resolve only that parent delta under a separately frozen protocol.

Development identities and evidence are spent and may not be reused as later
authority evidence. The old typed M4 remains behavioral reference only.
