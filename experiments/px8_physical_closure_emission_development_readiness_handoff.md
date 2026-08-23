# PX8 physical closure-emission development-readiness handoff

Status: **DEVELOPMENT READY; CLASS A; PX8 AUTHORITY ABSENT**.

This independent lane is several possible serial generations ahead. It has not
started or simulated an authority workflow, advanced PX3--PX8, created an
authoritative ancestor, executed a definitive matrix, or modified PX0--PX2.

## Outcome classification

**A — existing PX0--PX2 physics sufficient.**

The target already emerges from retained state accumulation/decay, threshold
firing, signed arrow coupling, causal arrow direction, ordinary pressure,
physical region crossing, participation traces, and pending-queue exhaustion.
There is no missing physical edge, new representation, or new substrate law.
The old typed-M capability supplied behavioral criteria only; none of its
machinery or evidence executes or links into the organism.

## Exact frozen chain

| freeze | commit | tag |
|---|---|---|
| starting authority | `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` | `px2-physical-causal-direction-authoritative` |
| development protocol | `48a2985e5be0c86173f5762439a73214ee77d077` | `px8-physical-closure-emission-development-protocol-v1` |
| initial implementation | `4267835606874ff146d8b1b79b5ede8a2e0f2b67` | `px8-physical-closure-emission-development-implementation-v1` |
| invalid PROBE v1 | `6546327d85b369d8fdcf4d76205f2a3c85e78565` | `px8-physical-closure-emission-probe-v1-invalid` |
| mechanical amendment | `53a95d4f9eb20b934f60a7258f85f4f85f599a24` | `px8-physical-closure-emission-probe-v2-amendment` |
| corrected implementation | `0604abc1efc1c62a10c2d46e591b5b402f799bb2` | `px8-physical-closure-emission-probe-v2-implementation` |
| positive PROBE v2 | `6d5ab750bda21f8667be80e2444892cddf87b8e5` | `px8-physical-closure-emission-probe-v2-positive` |
| positive MICRO | `42de68d0fe93491836829b5a15a17324de6d6b1b` | `px8-physical-closure-emission-micro-positive` |
| positive GATE | `319bcd37a2e7fc7d3c4824955fadcf61358521f4` | `px8-physical-closure-emission-gate-positive` |

## Frozen mechanism: no-addition result

There is no PX8 mechanism addition. The frozen mechanism is the already
authoritative PX0--PX2 substrate law:

- commit: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- tag: `px2-physical-causal-direction-authoritative`;
- source path:
  `crates/px0-physical-correspondence/src/lib.rs`;
- source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The exact frozen blocks that constitute the no-addition result are
`PlasticSubstrate::advance_time`, `PlasticSubstrate::propagate`,
`PlasticSubstrate::elapse_to`, `PlasticSubstrate::decay_cell`, the existing
CELL threshold/state/refractory fields, ARROW delay/phase/coupling/resistance
fields, SPIKE timing/origin/impulse fields, `TraceEntry`, `Crossing`,
`WorkLedger`, and deterministic pending-queue ordering. All blocks are covered
by the one exact source hash above; none changed relative to the frozen parent.

The frozen developmental physical fixture and audit boundary are:

- commit: `0604abc1efc1c62a10c2d46e591b5b402f799bb2`;
- tag: `px8-physical-closure-emission-probe-v2-implementation`;
- source path:
  `crates/px0-physical-correspondence/examples/px8_physical_closure_emission.rs`;
- source SHA-256:
  `9003411cb86ccf0a3cda534339014156c0ff6f11fb151192d58b47e52c8b402c`.

That example is not organism machinery. It freezes fresh physical initial and
boundary conditions, invokes the unchanged substrate, and observes the
returned execution afterward.

## Consumes physically

Every physical development consumes exactly these substrate facts:

- nine fresh CELLs, each with unique `physical_id`, resistance `100`, and
  initial state `0`;
- three ordinary participant CELLs, one convergence CELL, one outer CELL, and
  four unrelated inner CELLs;
- threshold `4` on the convergence CELL and threshold `1` on every other CELL;
- inner region `0` for all except the outer CELL, whose region is `1`;
- physical positions `0,20,40,80,120,180,200,220,240`, optionally mirrored;
  every separation exceeds the retained local-proposal radius, so no
  unlisted local route can form;
- two directional participant-to-convergence ARROWs with coupling `+3`, common
  delay `3,4,5,or 6`, phases `0` and `1`, and resistance `100`;
- one inhibitory participant-to-convergence ARROW with coupling `-3`, the same
  delay, phase `-1`, and resistance `100`;
- one convergence-to-outer ARROW with coupling `+1`, delay `2,3,4,or 5`, phase
  `0`, and resistance `100` in ordinary cells or `1` in the pressure-blocked
  cell;
- three unrelated inner-region ARROWs forming a four-CELL chain, each coupling
  `+1`, delay `1`, phase `0`, resistance `100`;
- ordinary external SPIKEs of impulse `+1` at tick `0`, or tick `10` after the
  pressure interval, into the physically present positive/inhibitory
  participant CELLs;
- the second positive external arrival either coincident or exactly two ticks
  later;
- in the unrelated-activity condition, exactly `4,8,12,or 24` external `+1`
  SPIKEs into the first unrelated CELL, two ticks apart;
- in the no-arrival condition, no external SPIKE;
- in the pressure-blocked condition, an empty-queue call to
  `advance_time(10)` before arrivals. The ordinary pressure step takes the
  weak outward ARROW from resistance `1` to `0`, marks it non-live, increments
  its generation, and takes every other live ARROW from `100` to `99`;
- actual SPIKE delivery order, CELL state/decay, signed impulses, threshold
  checks, ARROW traversal, `TraceEntry` participation records, `Crossing`
  records, local eligibility, ordinary pressure, and queue work from the
  retained substrate only.

No returned activity is required for the closure-emission relation. Relevant
participant and outward ARROWs receive no local-return update in the positive,
incomplete, inhibited, pressure-blocked, or no-arrival conditions. Repeated
unrelated activity can lawfully return to and strengthen only currently
eligible ARROWs in the unrelated chain; it never reaches the convergence or
outer CELL. That isolation is part of the positive control.

The complete environmental physical opportunity list is therefore: presence
or absence of the three participant arrivals; coincident or two-tick-skewed
positive timing; optional ten-tick ordinary pressure; optional unrelated
arrivals with exact count and spacing; fresh physical identities; mirrored
positions; normal/reversed allocation; normal/reversed insertion; and the
four frozen delay values. There is no hidden fixture scheduling or typed
construction after `propagate` begins.

## Produces physically

The ordinary positive conditions produce:

- first `+3` arrival: convergence CELL state remains subthreshold and no
  region-crossing occurrence exists;
- physically sufficient second `+3` arrival: convergence CELL fires and resets
  its state to `0`;
- exactly one convergence-to-outer region crossing at that firing tick;
- exactly one outer-CELL firing after the outward ARROW's physical delay;
- no new CELL or ARROW and no local-return update on a participant or outward
  ARROW;
- retained-law eligibility expiry pressures a used participant ARROW from
  resistance `100` to `99` when its physical delay `5` or `6` exceeds the
  four-tick eligibility window; delays `3` and `4` leave resistance `100`;
- the same existing expiry law pressures the outward ARROW from `100` to `99`
  only at outward delay `5`; delays `2,3,4` leave resistance `100`;
- ordinary transient eligibility/timing state already present in the
  substrate, exactly reflected in each end fingerprint;
- an empty pending queue and `naturally_quiescent = true`.

The blocked/control conditions produce:

- either positive alone: no convergence firing/crossing; convergence state `3`
  at the last relevant delivery; outward structure remains live;
- phase-earlier `-3,+3,+3`: no convergence firing/crossing; convergence state
  `3`; outward structure remains live;
- pressure-blocked outward route: convergence fires and resets to `0`, but the
  outward ARROW remains physically deallocated at resistance `0`, no crossing
  occurs, and the outer CELL does not fire; the other live ARROWs first fall
  from `100` to `99` in the ten-tick pressure step and participant delays `5`
  or `6` can apply the already-described additional eligibility-expiry
  pressure;
- unrelated activity: no convergence or outer firing and no region crossing;
  only the unrelated chain may receive retained-law local-return strengthening;
- no arrival: no activity; complete start and end fingerprints are equal;
- every condition: the pending queue empties naturally, with no activity limit,
  external cutoff, semantic stop path, or explicit quieting.

Exact per-row start/end/permanent fingerprints, signed convergence inputs,
firing/crossing ticks, work, and persistent bytes are frozen in
`results/px8_physical_closure_emission_gate.csv` (SHA-256
`2a00699990e14e045299b72a98f8af6e8eea4bfaa1620bdc2a506164b9418824`).
Those serialized physical facts, not conceptual shorthand, are the exact
output surface for the next PX stage.

## Assumptions

- authoritative PX0--PX2 commit is exactly
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- the retained substrate source remains byte-identical at SHA-256
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- every additional environmental physical opportunity is exactly the complete
  list in **Consumes physically** above;
- each serial use begins with the listed physical CELL states and topology,
  rather than a hidden reset, task boundary, or reconstructed typed object;
- evaluator expectations inspect an immutable returned `Execution` only after
  the ordinary queue has emptied;
- no hidden fixture scheduling or typed construction selects a local path.

## Forbidden

- typed adapter;
- reconstruction of any old M schema;
- semantic labels or evaluator-selected local paths;
- importing evidence or mechanisms from another parallel PX lane;
- FINISH, ANSWER, terminal supervision, semantic stop paths, episode endings,
  or renamed equivalents;
- serializer, typed organism intermediate, hidden task boundary, explicit
  quieting, or activity cutoff.

## Cumulative port rule

- import the frozen mechanism byte-for-byte onto the later authoritative
  parent;
- make no anticipatory redesign;
- run a physical dependency/path audit;
- the first incompatibility is the next cumulative collapse;
- resolve only that parent delta under a separately frozen protocol.

Specifically, when the serial ancestor reaches this capability, first port the
unchanged retained substrate blocks/hash and the exact physical fixture/hash
listed above. Do not replay this development evidence as authority. Diagnose
only the delta introduced by the then-current parent.

## Evidence and accounting

| artifact | SHA-256 |
|---|---|
| protocol | `881f4e3d46ce55aa1d637bfe2cc3cc99fb7e7ca2348fc1256e949fd1e3d36c2b` |
| mechanical amendment | `23e72d3311612518481e6d429f0943f821f9da50be82be2c8324ccbdf3b2f4fd` |
| invalid PROBE v1 audit | `504812e1777602ccda30ae8d51b11dae308b3772cdb8483a37b65eb403a66b87` |
| invalid PROBE v1 CSV | `d33963361b6c2a3f40a9eaa172d296dcdbd8f668afbdd1e26f136e00028e629f` |
| positive PROBE v2 CSV | `2356e99b5296986fdcb512ba5b6a396c408c00cb6db889dba6ae65dfc1689f3a` |
| positive PROBE v2 audit | `f58d30685171af0a940cac5cd741f3ac6592121ebd6f45c7a5096ccac3347d0f` |
| positive MICRO CSV | `91742cceabd974b409b38fa3e434438d369848d55ca99c90157161b0e6d0c605` |
| positive MICRO audit | `c883c293d382138efb2a0c0891fdff1464e5033e50f32b27ac22751275f234d3` |
| positive GATE CSV | `2a00699990e14e045299b72a98f8af6e8eea4bfaa1620bdc2a506164b9418824` |
| positive GATE audit | `41356995c8c98c8c970436421799a138f11e8cc600a31153bbb8bc83db724f1f` |

Valid PROBE v2 + MICRO + GATE account for `68,916` ledger operations and
`366,080` duplicate-inclusive persistent bytes. Including the preserved
invalid PROBE v1, total lane execution accounts for `71,074` ledger operations
and `394,240` duplicate-inclusive persistent bytes.

There are no scientific blockers or unresolved physical alternatives. The only
negative is the preserved PROBE v1 accounting invalidity; its unique mechanical
correction is frozen and it does not alter the Class A conclusion. Authority
remains absent.
