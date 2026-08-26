# CK0 junction checkpoint integration immutable negative v1

Status: complete frozen negative. J0/CV0 replay and consolidated RS2 did not
run.

Protocol: `746559c9ed2dc8ec53628515ac958b0b57f198bf`
(`ck0-junction-checkpoint-integration-protocol-v1`).

Frozen implementation: `5d3e2d834b769ada52528adb4b9e0d848177672f`
(`ck0-junction-checkpoint-integration-frozen-v1`).

One-shot fresh E2B worker: `i22mmngije2o3xr932boc`.

## Exact matrix

- cases: `20/20`;
- rows: `40/40`;
- clauses: `184/216`;
- same-mechanics replay: `40/40`;
- natural quiescence: `40/40`;
- maximum PhysicalWork: `4`;
- complete positive: false.

Artifacts:

- `experiments/results/ck0_junction_checkpoint_integration_v1/matrix.csv`;
- `experiments/results/ck0_junction_checkpoint_integration_v1/report.md`.

Hashes:

- matrix:
  `6335f374413ebdf64fcb12c3e4ad206ebd509f022f00b090aa0702c6337ffbb1`;
- report:
  `a4a1984597339a3f100cd13c3e7c080f01e4f348c774b898a8848789232921fd`.

## What passed

Every direct liveness, topology, reuse, and stale-reference predicate passed
under both roots and both mechanics:

- live junction round-trip;
- dead junction with nonzero dormant resistance round-trip;
- dead-slot reuse with advanced generation;
- old CELL references remain stale;
- incoming and outgoing stale ARROWs remain inert;
- live topology retains its junction;
- loss of the last live incident link kills the junction;
- the mixed Reference/Production continuation family.

The checkpoint loader no longer rejects a J0 body as `InvalidPhysicalBody`.
All cases replayed exactly and quiesced naturally.

## Frozen failures

The 32 failed clauses have two classes.

### Raw checkpoint-hash comparison: 24 clauses

Families 2-7 passed all scientific checks, had identical reported
PhysicalWork, tick, durable-body hash, signature, and quiescence, but their raw
live-checkpoint hashes differed between Reference and Production. The
evaluator included `checkpoint_hash` in `Observation` equality even though the
protocol explicitly made raw checkpoint bytes diagnostic rather than a
cross-mechanics equivalence predicate.

This is a frozen evaluator measurement defect. It is the same category as the
previously established distinction between causally meaningful checkpoint
state and mechanically inert serialized bookkeeping.

### Composite continuation predicate: 8 clauses

The live-pending and quiescent-future families failed their internal
`Continuation` equality predicate under both roots and both mechanics. The
frozen composite compares:

- ordered trace;
- the entire `Work` value, including its private legacy `total` field;
- tick;
- durable-body hash;
- quiescence.

The matrix serializes only the resulting composite failure and the actual
continuation, not a component-by-component expected/actual difference. The
exact differing component therefore cannot be recovered without a separately
preregistered diagnostic execution. The evaluator also overreaches relative
to the protocol by comparing full `Work` rather than `PhysicalWork`.

CK0 v1 remains negative despite the strong direct results. No comparator was
repaired and no matrix was rerun.

## Boundary

The J0/CV0 lineage and consolidated RS2 were correctly not executed. CE1,
FD2 v2, frozen ARC A2, authority, oracle status, `arch.md`, and the Academy
curriculum remain unchanged.
