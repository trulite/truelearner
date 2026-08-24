# Physical body V1 development result

Outcome: **DEVELOPMENT POSITIVE — READY FOR HUMAN DESIGN/CODE REVIEW**.

This result does not advance successor authority, change the remote default
branch, or promote `runtime/truelearner-redesign` to `main`.

## Frozen parent and protocol

- PX-C scientific authority: `ec87c438aa8c52389fd2734667363ef43acaef93`.
- Behavior-preserving relocation baseline: `026b51b`.
- V1 protocol: `43221fa`.
- Validated implementation tip before this audit: `36916cd`.
- E2B worker: `ihvu0s15mme4snmfrkekf`.

## Production surface

Exactly two packages exist under `truelearner/`:

1. `truelearner-core` — physical execution and checkpoint state.
2. `truelearner-arena-format` — identity, canonical durable bytes, manifests,
   hashing, and validation only.

Production Cargo metadata contains no path under `experiments/`. No mmap or
memmap implementation exists.

Source hashes:

- core: `e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d`;
- arena format: `8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812`.

## Implemented V1 contract

- Stable `ArenaId`, `CellId`, `ArrowId`, and `Generation`.
- Explicit disposable `CellSlot` and `ArrowSlot`.
- Explicit reference resolution and stale-generation rejection.
- Bounded CELL/ARROW capacity and deterministic dead-ARROW reuse.
- Resident compaction that changes slot packing without changing stable refs.
- Canonical little-endian SoA arena blocks with fixed widths and section
  checksums.
- Immutable hashed `BodyVersion` manifests.
- Structural `BodyVersion` distinct from temporal restart points.
- `PhysicalClock` with pressure phase derived from tick.
- Canonical hashed `QuiescentCheckpoint` with clock.
- Canonical hashed `LiveCheckpoint` with activation, refractory state,
  eligibility, queued spikes, deterministic serial state, waiting arrivals,
  and pending storage loads including admitted availability ticks.
- Fail-closed decoding for bad magic/version/header, truncation, overlap,
  trailing bytes, invalid flags, bad checksums, invalid physical bodies, and
  stale internal references.
- Explicit RAM execution; no directly mutable mmap state.

## Targeted production validation

- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --locked`: PASS, `10/10` focused tests.
- strict workspace Clippy with `-D warnings`: PASS.
- release workspace build: PASS.
- production metadata/dependency closure: PASS.

Focused tests cover canonical arena round-trip and ordering, body-manifest
hashing, corrupt/truncated/overlapping/trailing input, duplicate identity,
canonical body reload, clock-phase-preserving quiescent restart, exact live
continuation with pending load availability, stale generation rejection,
invalid durable-reference rejection, and compaction invariance.

## PX-C regression

The complete retained development matrix passed:

- rows: `16/16`;
- clauses: `524/524`;
- maximum per-batch work: `104331 / 200000`;
- maximum resident bytes: `44328 / 65536`;
- natural quiescence: true;
- outward-only boundary: true;
- exact replay: true.

The authority matrix was not executed. V2 cold storage, asynchronous loading,
network transport, and organism-visible storage affordances remain out of
scope.
