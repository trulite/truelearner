# PX5 physical plasticity-allocation development-readiness handoff

Status: **DEVELOPMENT READY; CLASSIFICATION A; AUTHORITY ABSENT**.

## Outcome

Existing authoritative PX0--PX2 physics is sufficient. PX5 adds no organism
mechanism, representation, substrate law, threshold, pressure rule, trace,
allocator, or gating policy.

The frozen progression is:

| stage | outcome | commit | tag | primary artifact SHA-256 |
|---|---|---|---|---|
| PROBE | `4/4`, `32/32` positive | `f5e1b80677ef89a28801eadb58a613497f139fd8` | `px5-physical-plasticity-allocation-probe-v1-positive` | CSV `866673057bef55e45a284b6d6d557f7dd4fccc7b0fab3f73f5028656f8047ded` |
| MICRO | `8/8`, `80/80` positive | `1b2c4aaaf0f6fbc858e7289b527e884cc9623d45` | `px5-physical-plasticity-allocation-micro-v1-positive` | CSV `16bc4595a90cd61b015874358eb21dbf2dee469c76b2c8a813f0aa036476837f` |
| GATE v1 | immutable `132/144` negative | `2107558b6cb6b13f501bdbc50924cb493dbcf8fe` | `px5-physical-plasticity-allocation-gate-v1-negative` | CSV `e50e9c88cc26e8b418b8264a3cb7cb71973e661d5001c76a6c05c28f4a79747c` |
| GATE v2 | `12/12`, `144/144` positive | `e4415934b6a222d74289e9bcd8abac88c65a96de` | `px5-physical-plasticity-allocation-gate-v2-positive` | CSV `24d8eb9f22a2c06edbc1c9b6a5a9cce31193fe0fc6cf61fd95cef89a4059ec29` |

GATE v1 failed only because a hot edge entered withholding at resistance `10`
but the preregistered horizon supplied insufficient pressure. The frozen v2
delta extended only that horizon and used fresh identities; no scientific or
mechanism change occurred.

## Lane classification

**A = existing PX0--PX2 physics sufficient.**

The result is not between classes. There is no missing physical edge (B), no
unresolved choice among distinct physical mechanisms (C), and no new
representation or substrate law requirement (D).

The behavioral reference to old typed M5 is limited to four endpoints:
productive physical patterns receive more future structural work, distractors
remain sparse, stale useful structure reacquires, and shuffled evaluator-side
allocation cannot substitute. No old M5 code or schema linked or executed.

## Frozen mechanism: no-addition contract

The exact frozen organism mechanism is the already-authoritative PX0--PX2 law:

- commit: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- tag: `px2-physical-causal-direction-authoritative`;
- source path: `crates/px0-physical-correspondence/src/lib.rs`;
- source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

No PX5 organism source exists. The exact frozen blocks constituting the
no-addition result are all in that one source hash:

1. constants `LOCAL_WINDOW = 4`, `LOCAL_RETURN_STRENGTH = 3`,
   `UNSUPPORTED_USE_PRESSURE = 1`, `ORDINARY_PRESSURE_PERIOD = 10`,
   `LOCAL_VARIATION_RADIUS = 2`, and `COUPLING_PLASTICITY_CEILING = 16`;
2. `PlasticSubstrate::propagate`: actual SPIKE delivery, CELL firing, external
   firing's call to generic `propose_local_arrows`, outgoing ARROW traversal,
   `eligible_until = tick + 4`, and emitted SPIKE generation binding;
3. `PlasticSubstrate::apply_local_return`: a SPIKE physically arriving at a
   CELL changes only live outgoing ARROWs from that CELL whose own participation
   eligibility is still current, adding resistance `3`, saturating positive
   coupling at `2`, and consuming eligibility;
4. `PlasticSubstrate::elapse_to`: scalar pressure every ten ticks plus one unit
   for expired unsupported participation, physical deallocation, CELL decay;
5. `PlasticSubstrate::propose_local_arrows`: generic external-firing variation
   to every live CELL one or two physical position units away when no equivalent
   live ARROW exists, with resistance `1`, coupling `1`, distance delay, and a
   fresh generation;
6. `pressure_arrow`: resistance subtraction to zero, liveness removal,
   generation increment, and eligibility erasure.

The frozen positive GATE evaluator is not organism machinery, but fixes the
development evidence boundary:

- commit: `b2139866646c448d3bfabab763bd5b8265c8c22e`;
- tag: `px5-physical-plasticity-allocation-gate-v2-implementation`;
- source path:
  `crates/px0-physical-correspondence/examples/px5_physical_plasticity_allocation_gate_v2.rs`;
- SHA-256:
  `bc9b951e2e043f274d3800edce59664482f3408f74898eb116707663f3e23a7c`.

It must never be linked or ported as organism machinery.

## Consumes physically

The mechanism consumes these exact substrate facts, not conceptual shorthand.

### Retained structures

- live CELL records with physical identity, integer position, physical region,
  positive threshold, decaying scalar state, last-update tick, refractory tick,
  generation, resistance, and liveness;
- live or dead ARROW records with concrete source and target CELL handles,
  delay, phase, scalar coupling, captured source generation, own generation,
  scalar resistance, liveness, and optional `eligible_until` tick;
- queued SPIKE records with arrival tick, phase, physical origin, concrete
  target CELL, captured target generation, scalar impulse, serial order, and
  optional traversed ARROW/generation binding.

### Required activity and timing

- an ordinary external SPIKE must make a concrete source CELL cross threshold;
  only that physical firing opens generic local structural opportunity;
- an emitted SPIKE must traverse a concrete live outgoing ARROW; that exact
  ARROW receives its existing participation eligibility through tick `t + 4`;
- ordinary returned SPIKE activity must physically arrive back at the same
  source CELL no later than the eligible tick to add local structural work;
- physical time must advance; every completed ten-tick interval pressures all
  live ARROW resistance, and an expired unsupported eligibility supplies one
  additional pressure unit;
- recurrence is repeated physical CELL firing and ARROW traversal, not a stored
  class or counter supplied by an evaluator;
- stale execution is blocked only by ARROW liveness and generation mismatch;
- outward effect is an actual live ARROW emission whose endpoint CELL is in a
  different physical region, recorded as a crossing after emission.

The PX1/PX2 participation trace consumed here is exactly the ARROW-local
`eligible_until` left by real traversal. Returned activity is exactly a SPIKE
delivered to the participating source CELL. No provenance owner or direction
field is consumed.

## Environmental physical opportunities and assumptions

The frozen parent assumption is authoritative PX0--PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, unchanged.

Every additional environmental opportunity used in development is explicit:

1. blank live CELL pairs were physically placed twelve position units apart so
   neighborhoods did not overlap; paired CELL distance was `1` or `2` and the
   two endpoints occupied different physical regions;
2. all useful and distractor source CELLs received ordinary external threshold
   activity at tick `0`, creating equal generic initial opportunity;
3. only useful sources received ordinary returned activity at tick `2`;
4. half of useful sources physically fired/returned at `10/12`, `30/32`, and
   `50/52`; all useful sources fired/returned at `20/22` and `40/42`;
5. up to eight concrete distractor sources physically refired at `15, 30, 45`
   without return;
6. all useful sources physically fired at `60` for held-out execution and
   received ordinary return at `62`;
7. source `0` then received no SPIKE activity while every other useful source
   fired/returned at `70/72, 80/82, 90/92, 100/102, 110/112, 120/122,
   130/132, 140/142`; time advanced to `150`;
8. withheld source `0` then received fresh ordinary activity at `150` and
   return at `152` for generic reacquisition;
9. the matched return-free control fired one source at `0, 6, 12, 18, 24, 30`
   and advanced time to `36`;
10. the late-return control fired at `0` and supplied source activity only at
    `6`, outside the four-tick window, then advanced to `12`;
11. the outside-radius control placed the only other CELL three position units
    away before source activity at `0`;
12. the physical-return permutation fired all sites at `0`, but delivered the
    same count of tick-`2` return SPIKEs to the first distractor sources and
    advanced to `20`;
13. mirrored position, reversed CELL allocation, reversed SPIKE insertion, and
    fresh physical identity variants changed only those actual physical facts;
14. evaluator-only observation vectors were reversed strictly after execution,
    with no write or mutation surface back into the substrate.

These opportunities are physical inputs supplied by the experimental
environment. A later cumulative port may rely only on equivalent CELL
placement/topology and ordinary SPIKE timing present in that parent/environment.
It may not reconstruct the schedule as a hidden typed fixture, infer a task
boundary, or inject a selected local update.

## Produces physically

For an externally fired source, generic opportunity produces live outgoing
ARROW variation to every currently eligible physical neighbor, initially at
resistance `1`, coupling `1`, phase `0`, and delay equal to physical distance.

Actual traversal plus timely return produces:

- a consumed transient participation eligibility on the traversed ARROW;
- resistance raised by exactly `3` per timely local return, opposed by the two
  frozen pressure paths;
- positive coupling physically saturating at `2` while resistance is below the
  frozen ceiling;
- retained live useful ARROW topology that emits actual SPIKEs on later CELL
  firing and creates outward crossings when physical regions differ.

Return-free, late-return, or outside-radius conditions produce no retained
useful ARROW: low-resistance structure reaches zero, becomes non-live, clears
eligibility, and increments generation. Queued stale SPIKEs bound to the old
generation cannot execute.

After withholding, fresh source firing produces a distinct replacement ARROW
through the same generic radius law; its first emission crosses outward, and
timely return raises it from resistance `1` to `4` and coupling `2`. At the end
of the GATE reacquisition run, that replacement remains live with participation
eligibility through tick `156`; other useful routes remain live; distractor
routes are dead. Every propagation queue is naturally empty. No allocator,
class, policy, or task state remains.

The frozen positive GATE measured `1,067,970` work-ledger operations and
`338,560` bytes of persistent substrate storage across all twelve primary and
control cells. It produced `4/4` or `8/8` held-out crossings, exactly one
reacquisition crossing, zero live distractor routes, and duplicate-exact final
fingerprints.

## Forbidden in every port

- typed adapter;
- reconstruction of any old M schema;
- semantic labels or evaluator-selected local paths;
- importing evidence or mechanisms from another parallel PX lane;
- encounter class, encounter snapshot, `LEARN_HERE`, proposal-site label, or a
  renamed equivalent;
- supplied gating/allocation policy, semantic enum, typed intermediate
  representation, serializer, evaluator feedback, selected mutation, hidden
  task/episode boundary, or organism reset.

Evaluator words such as useful, distractor, hot, warm, withheld, or shuffled
may name report rows only. They may not enter retained or transient organism
state or select a local physical update.

## Cumulative port rule

When the serial authoritative ancestor eventually reaches PX5:

1. import the frozen mechanism byte-for-byte onto the later authoritative
   parent; because this is classification A, the import is the exact unchanged
   PX0--PX2 blocks and no PX5 organism source;
2. make no anticipatory redesign;
3. run a physical dependency/path audit from actual parent CELL/ARROW/SPIKE
   state through generic opportunity, traversal eligibility, return, pressure,
   retained topology, deallocation, and crossing;
4. the first incompatibility is the next cumulative collapse;
5. resolve only that parent delta under a separately frozen protocol.

If the later parent lacks one of the explicitly listed physical opportunities,
that absence is a parent delta; it does not authorize an allocator, adapter, or
reconstruction of old M5.

## Preservation and authority boundary

All positive and negative protocols, implementations, CSVs, reports, and
audits are frozen. In particular, GATE v1 remains an immutable negative.

No authority workflow was started or simulated. No definitive matrix was
executed. This lane did not advance PX3--PX8, alter PX0--PX2, create an
authoritative ancestor, or consume another parallel lane's evidence or
mechanism. Authority remains absent.
