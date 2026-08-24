# PSEL0 Production Mechanics Selection Protocol

## Parent and claim boundary

Parent: `r1-r5-mechanical-optimization-development-v1`
(`3411aba95485a309d0d4f74ec8824c5029681c82`).

PSEL0 is an engineering discriminator. It adds no physical law and advances no
organism authority. The accepted 16-world corpus remains the physics oracle.
Stress worlds added here measure mechanics only and make no cognitive claim.

R6 multi-arena work is explicitly out of scope and must begin on a later branch
from the selected PSEL0 result.

## Candidate configurations

```text
A  TimingWheel + Adjacency + Frontier + AoS + Scalar
B  TimingWheel + Adjacency + Frontier + AoS + Batched
C  TimingWheel + Adjacency + Frontier + SoA + Scalar
D  TimingWheel + Adjacency + Frontier + SoA + Batched
```

The permanent reference remains:

```text
Vec + GlobalScan + FullScan + AoS + Scalar
```

## Physics gate

Before cost ranking, every candidate must match the permanent reference on the
frozen 16 worlds using the existing canonical comparator and exact replay.
Failure disqualifies the candidate regardless of speed.

No performance world may alter or weaken this gate.

## Required cost instrumentation

Record measured, non-placeholder values for:

```text
allocations
bytes touched
peak resident bytes
queue operations
ordering comparisons
global scans
adjacency accesses
active frontier size (mean and maximum)
eligible frontier size (mean and maximum)
batch size histogram
batch fallback count and reason
elapsed CPU (diagnostic only)
```

Deterministic counters and physical equivalence decide acceptance. CPU timing
is reported but never treated as reproducible evidence.

## Mechanical stress corpus

Use deterministic, non-semantic workload families spanning:

```text
many cells
many arrows
sparse activity
dense activity
long delays beyond the near wheel
many same-tick arrivals
high fan-out
heavy modulation
mostly dormant graph
zero-delay topology requiring safe batch fallback
```

Each family must use fresh physical identities and bounded capacity. It must
quiesce, replay exactly, and match the reference physical history. Workload
scale may differ from the authority corpus but physical laws may not.

## Selection rule

Select mechanics compositionally rather than treating D as an indivisible
runtime:

1. retain TimingWheel, Adjacency, and Frontier unless instrumentation exposes a
   workload-specific regression large enough to reject one;
2. select AoS or SoA from measured access, allocation, residence, and runtime
   cost after equivalent accessor paths are used;
3. enable batching opportunistically only where it reduces cost and preserves
   exact ordering; fallback is lawful and must be measured;
4. do not implement or select SIMD in PSEL0.

The selected configuration must never remove `ReferencePhysics` or its
differential harness.

## Execution rules

- All Rust formatting, compilation, tests, and workloads run in E2B.
- Use one consolidated dependency build where practical.
- Do not rerun full suites after documentation-only changes.
- Do not add traits or strategies for the physical learning law.
- Stop on a physical divergence, new substrate-law proposal, or measurement
  ambiguity that changes the selection outcome.
