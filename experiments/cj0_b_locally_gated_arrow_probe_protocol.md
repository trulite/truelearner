# CJ0 ARM CJ-B locally gated ARROW PROBE protocol

Status: **PREREGISTERED; PROBE UNSPENT; NO AUTHORITY EXECUTION**.

## Authority and isolation

This development arm begins at exact commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tagged
`px2-physical-causal-direction-authoritative`. PX0 correspondence, PX1
boundary roles, and PX2 causal direction are authoritative and immutable.
Their source bytes may not change.

PX3 Class D and PX3-R Arms A/B/C remain immutable negatives. This arm does
not advance PX3, reopen PX3-R, execute a definitive matrix, or touch PX4--PX8
or PX-C. Its implementation must be a fresh standalone addition under
`arms/cj-b-locally-gated-arrow/`.

## One candidate law

At outgoing-ARROW inspection time, decay the destination CELL's existing
ordinary state to the current tick and compute:

```text
available = destination.state + arrow.coupling
```

If `available < destination.threshold`, the ARROW emits nothing, creates no
eligibility, and changes no structure. If `available >=
destination.threshold`, atomically consume the destination's current state,
emit one ordinary SPIKE whose impulse is `available`, and make that traversed
ARROW locally return-eligible exactly as in PX0--PX2. Delivery, threshold
firing, return strengthening, unsupported-use pressure, ordinary pressure,
generation invalidation, and natural queue drain remain physical operations.

There is no gate field. Strong ordinary ARROWs whose coupling alone reaches a
destination threshold propagate exactly as ordinary ARROWs. Weak local
ARROWs require simultaneous/recent destination state. The law is identical
for primitive and learned source CELL firings.

This is one local consume/produce rule, not a logical operator. A need to add
another persistent field or a scientifically distinct gating law is a stop.

## Physical opportunity and bootstrap

Each cell contains a complete symmetric numeric field of weak candidate
ARROWs for all four relevant directed conjunction sites. The field carries no
scenario-dependent additions. Contributor activity fans out as ordinary
external physical SPIKE effects to its fixed nearby sites; trigger activity
fires an ordinary source CELL. Every relevant route receives exactly the same
number and magnitude of external arrivals across trained and crossed worlds.

Weak candidate ARROWs begin at coupling 1/resistance 1. Destination CELL
threshold is 3 and current contributor participation supplies state 2.
Therefore a fresh weak ARROW can bootstrap from ordinary co-activity without
a mature higher-order firing. Returned activity may raise coupling only to 2,
so source-alone activity remains below threshold forever.

After physical deallocation, the existing local numeric proposal rule may
recreate a missing weak coupling-1 ARROW from an externally fired source to a
live CELL within radius 2. Proposal uses no scenario label or expected
organization. Held-out observations fire sources through pre-existing strong
driver ARROWs, so observation does not itself install a missing candidate.

## PROBE matrix

Run four positive fresh deterministic cells:

1. normal layout/allocation/insertion;
2. mirrored layout;
3. reversed allocation and physical-identity order;
4. permuted contributor orientation and reversed arrival insertion.

Each cell acquires A+B and C+D for 8 matched rounds. Cluster order alternates;
every route has identical occurrence count, external arrival count, total
impulse, traversal opportunity, returned-activity opportunity, pressure, and
outward-effect opportunity. Held-out observations run on clones after a
fixed quiescent gap:

- A+B and C+D must each produce one organized outward crossing;
- A+D and C+B must produce zero organized outward crossings;
- A, B, C, and D alone must each produce zero;
- an old learned organization activated by its trigger alone must produce no
  destination consumption, candidate traversal, return eligibility, output,
  or strength change.

Run isolated controls with the same physical law:

- destination contribution too late;
- passive correlation without trigger traversal;
- candidate traversal with the physical return ARROW absent;
- both contributors genuinely participate;
- missing weak opportunity;
- blocked/deallocated weak path;
- one stale coactivity followed by ordinary pressure;
- ambiguous three-contributor activity;
- exact replay from byte-identical complete state;
- natural quiescence and finite recurrence without runaway.

PROBE does not execute reversal, recursion, OR, or the extended temporal
matrix. Those are gated behind a positive PROBE and separately frozen MICRO
and GATE protocols.

## Independently serialized stages

Every row must serialize separately:

- initial, post-acquisition, and post-gap permanent fingerprints;
- per-route external arrivals/source firing/participation deposition;
- per-candidate attempted inspection, suppressed transmission, consumed
  state, emitted SPIKE impulse, destination firing, return arrival, local
  strengthening, outward crossing, and resistance/live state;
- trained, crossed, singleton, self-evidence, and control observations;
- pressure, proposal, deallocation, generation, queue, and total work;
- persistent bytes, natural quiescence, and duplicate equality.

Evaluator names, schedules, and pass clauses exist only after the
organism-visible physical module boundary and have no update path into it.

## Conjunctive PROBE clauses

`P0` frozen ancestry/hashes/isolation; `P1` matched marginals; `P2` genuine
co-participation consumes state and transmits; `P3` both singletons and the
self-evidence observation do not transmit; `P4` trained held-out activates;
`P5` crossed held-out does not; `P6` no-return, correlation-only, absent,
blocked, stale, too-late, and ambiguity controls match their physical
predictions; `P7` exact replay; `P8` quiescence/no runaway; `P9` complete
stage serialization and accounting.

All clauses must pass in every cell/control. The first failure freezes the
exact result and stops this candidate before MICRO. Mechanically unique
serialization or forced timing defects may be corrected only under a fresh
frozen protocol; scientific failures may not be rescued.

## Execution discipline

The committed implementation must provide no-argument/wrong-argument refusal
and a no-CELL `--preflight`. Before the one-shot command, verify format,
focused tests, strict Clippy, zero dependencies, exact frozen hashes,
fresh-path isolation, forbidden-token/source leak audit, and absence of all
staging/final result paths.

The sole evidence command will be:

```text
cargo run --release --manifest-path arms/cj-b-locally-gated-arrow/Cargo.toml \
  --bin probe -- --probe
```

It must atomically publish result CSV/report or a first-clause-failure pair.
No rerun, tuning, regeneration, definitive execution, or authority claim is
permitted after publication.
