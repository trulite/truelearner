# PX8 substrate-native physical closure-emission development protocol

Status: **PREREGISTERED DEVELOPMENT ONLY; EVIDENCE UNSPENT; PX8 NON-AUTHORITATIVE**.

## Frozen start and lane boundary

This independent lane starts exactly at authoritative PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
`px2-physical-causal-direction-authoritative`, on branch
`research/px8-physical-closure-emission`.

| frozen parent | commit / SHA-256 |
|---|---|
| authoritative PX0 commit | `e884ae133a562d475565a36700d929b51dd2b2d2` |
| authoritative PX1 commit | `3ae1e35755dcca6fac1073469ab36b466e38a1c9` |
| authoritative PX2 commit | `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` |
| retained PX0--PX2 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX0 definitive CSV | `b750792123de1c0aa7d3104d2d1bcd3fdc6e26a70e54b10f5eedf320fe7d95c9` |
| PX1 definitive CSV | `6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6` |
| PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| PX2 result audit | `7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe` |
| PX2 authority handoff | `98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509` |

The lane may execute PROBE, MICRO, and GATE development only. It may not
advance PX3--PX8 authority, create an authoritative ancestor, execute a
definitive matrix, alter PX0--PX2, or consume mechanisms or evidence from a
parallel development lane.

## Frozen behavioral reference only

The old typed-M closure-emission record is used only to freeze these external
behaviors: silence before physical closure, an ordinary outward crossing after
closure, silence when closure or the outward route is blocked, and natural
quiescence. Its protocol and final development report have SHA-256
`140d2392263359c666f364e1923f956a4b2b09e4107a0fd2f7f8f469d97be154`
and `6d5b1a66d5064f80237a58fedd8aca35688a46b70f1879e4f09c6a34fd88e028`.
No old-M source, schema, route, learner, serializer, or artifact is linked or
read by the experimental executable.

## First question and no-new-mechanism hypothesis

The first question is whether the target already follows from retained
PX0--PX2 physics. The preregistered hypothesis is:

```text
ordinary directional participation A --+
                                       +--> existing CELL threshold
ordinary directional participation B --+           |
                                                    v
                                           existing live ARROW
                                                    |
                                                    v
                                           physical region crossing
                                                    |
                                                    v
                                           ordinary queue exhaustion
```

Both participating impulses are individually subthreshold. Existing cell
state, decay, threshold firing, arrow coupling/delay, physical region labels,
crossing observation, and queue propagation are sufficient. An ordinary
negative coupling may prevent threshold firing. No new representation,
substrate state, substrate law, trace, semantic path, or stopping operation is
proposed.

The source `crates/px0-physical-correspondence/src/lib.rs` must remain
byte-identical. The developmental executable may only assemble fresh fixed
CELL/ARROW state, introduce ordinary external SPIKE occurrences, call
`advance_time` where preregistered, and call `propagate`. Expectations and
measurements are evaluator-only and can run only after propagation returns.

## Exact physical circuit

Each cell contains three ordinary participant cells, one convergence cell, one
outer-region cell, and four unrelated activity cells. All positions are more
than the PX0 local-variation radius apart, so external arrivals cannot create
an unpreregistered local route.

- participant-to-convergence coupling: `+3`;
- inhibitory-participant-to-convergence coupling: `-3`;
- convergence threshold: `4`;
- participant and outer-cell threshold: `1`;
- convergence-to-outer coupling: `+1`, delay `2`;
- ordinary arrow resistance: `100`, except the blocked outward arrow starts at
  `1` and is deallocated by one ordinary ten-tick pressure interval;
- all cells start with state `0`, resistance `100`;
- inner region: `0`; outer region: `1`;
- no evaluator observation can change state or choose an execution path.

Coincident `+3,+3` arrivals cross threshold on the second arrival. In the
skewed-positive condition the second arrival is two ticks later: retained
linear decay changes `3` to `1`, then the second `+3` reaches threshold `4`.
The negative arrival precedes the two positive arrivals by ordinary phase
ordering and leaves total state below threshold.

## Eight physical conditions

Every stage runs all eight conditions:

1. two coincident participating arrivals;
2. two participating arrivals with the preregistered two-tick skew;
3. only the first positive participant;
4. only the second positive participant;
5. both positives plus the ordinary negative participant;
6. both positives after ordinary pressure deallocates only the outward arrow;
7. matched unrelated inner-region activity with no convergence participation;
8. no external arrival.

Conditions 1--2 must produce exactly one convergence firing, exactly one
convergence-to-outer crossing, and exactly one later outer-cell firing.
Conditions 3--5, 7, and 8 must not fire the convergence cell or cross a region.
Condition 6 must fire the convergence cell but must not cross a region or fire
the outer cell. Thus closure and outward availability are independently
controlled.

## Independently serialized clauses

Every row serializes these ten clauses separately:

1. `P0`: frozen source/parent basis and fresh namespace are exact;
2. `P1`: positive participant trace arrivals/firings match the condition;
3. `P2`: convergence receives only the physically emitted signed impulses;
4. `P3`: no outward crossing occurs before convergence threshold firing;
5. `P4`: convergence firing matches positive, incomplete, and inhibited
   physical closure;
6. `P5`: outward crossing matches convergence firing plus live outward
   structure;
7. `P6`: outer firing follows the crossing by exactly the physical arrow
   delay and never occurs otherwise;
8. `P7`: no alternate region crossing occurs;
9. `P8`: propagation ends in natural queue quiescence without a cutoff;
10. `P9`: byte/execution-identical duplicate development is exact.

The CSV also records signed convergence impulses and ticks, first/second
participant ticks, convergence tick, crossing tick, outer tick, live outward
state, complete/permanent fingerprints, ledgered work, persistent bytes, and
allocation/insertion/layout variations. No compound pass may hide a failed
physical stage.

## Fresh staged matrices

Every matrix row is executed once plus one preregistered exact duplicate.
Identity reuse is limited to the exact duplicate pair; every distinct row has
a fresh namespace.

| stage | seeds | conditions | rows | physical developments | namespace base |
|---|---:|---:|---:|---:|---|
| PROBE | 2 | 8 | 16 | 32 | `0x8_8000_0000` |
| MICRO | 8 | 8 | 64 | 128 | `0x8_8200_0000` |
| GATE | 16 | 8 | 128 | 256 | `0x8_8600_0000` |

Seeds rotate through mirrored/non-mirrored placement, normal/reversed cell
allocation, normal/reversed external-arrival insertion, four participant
delays, four outward delays, physical-id permutations, and unrelated activity
loads `0, 4, 12, 24`. These changes may alter work and fingerprints, but not
the ten conjunctive claims.

## Anti-cheat and source boundary

Organism-visible execution is exactly the dependency-free
`PlasticSubstrate`: CELL state, ARROW structure, SPIKE occurrences, local
physical time/order, decay, thresholds, resistance/pressure, participation
traces, crossings, and the ordinary pending queue.

The executing substrate and its call boundary may contain no FINISH, ANSWER,
terminal supervision, semantic stop path, episode ending, serializer, adapter,
old-M schema, typed organism intermediate, evaluator-selected path, hidden task
boundary, explicit quieting, activity cutoff, or renamed equivalent. The
experimental condition table and pass clauses remain outside the substrate;
they set physical initial/boundary conditions and inspect an immutable returned
execution only.

The executable must refuse any definitive argument before assembling a cell.
Static audits must establish that the retained substrate source is exact, the
PX0 crate has no dependencies, no old-M module is imported, and no forbidden
semantic control term occurs in the retained substrate law. The example source
may contain condition and audit language only in evaluator code; it may not
add a runtime alternative to `propagate`.

## Ordered evidence and immutable stopping rule

The implementation is frozen before any evidence. Then:

1. run PROBE exactly once and atomically freeze its CSV/report;
2. only a `16/16`, `160/160` positive permits MICRO;
3. run MICRO exactly once and atomically freeze its CSV/report;
4. only a `64/64`, `640/640` positive permits GATE;
5. run GATE exactly once and atomically freeze its CSV/report;
6. only a `128/128`, `1,280/1,280` positive permits a development-readiness
   handoff.

Any negative is frozen unchanged and ends this lane unless it mechanically
identifies one unique missing physical edge already allowed by retained law.
No result may be rescued, regenerated, overwritten, or rerun. Multiple
scientifically distinct missing relations require diagnostic discrimination;
unresolved ambiguity stops development. A need for new representation, new
substrate law, parent modification, or semantic choice stops immediately.

Each command writes through stage-specific `.staging` paths and refuses if a
staging or final artifact exists. Each emits exactly one stage evidence marker.
There is no definitive mode.

## Development-ready consequence and later-porting contract

A positive GATE demonstrates **no new mechanism**: closure-emission is an
ordinary retained threshold/arrow/crossing/quiescence trajectory. It does not
make PX8 authoritative and does not authorize PX3--PX8 execution.

When the serial authoritative ancestor reaches this capability, port unchanged
first:

- the exact retained substrate source hash above;
- the physical convergence relation (`+3,+3`, threshold `4`, optional `-3`);
- the ordinary live outward arrow and region crossing;
- the absence of any semantic control or explicit stopping path;
- the eight conditions and ten serialized clauses.

Only the delta introduced by the then-current serial parent may be diagnosed.
This lane's evidence is never replayed as serial authority.
