# PD2 explicit-eligibility deletion implementation audit v1

Status: frozen candidate implementation; no development-readiness claim.

Parent: `f5a7675bb48ae4e5b19ed9e15504d9c7e7eb9442`

Candidate: `7e7a5757005ac2e83ba76208a52f118a67ce35ac`

## Deletion

The active runtime removes the entire old deadline surface:

- `LOCAL_WINDOW`;
- the ARROW and SoA deadline fields;
- the deadline event and frontier;
- expiry scanning and unsupported-expiry pressure;
- deadline checkpoint bytes, accessors, tests, and execution-cost counters;
- the legacy deadline-qualified Modulation and pressure branches.

The active core contains no remaining occurrence of the frozen deletion search
terms. The change removes 578 lines and adds 82 lines, primarily to make the
already-earned participation, pressure-load, and QLP state unconditional and
to preserve it in live checkpoints. The checkpoint format version changes from
1 to 2.

Core hashes:

```text
lib.rs       f12470deaa4f45acf72ea56fe486a86a26d138b4d6bf178db3390596e35bb450
mechanics.rs 60131f74b318d7698634208f1c7df64b2fcccd4086b3620bf28f1a08b84f2951
```

## No replacement physics

No new state, timer, horizon, address, route identity, pressure exception, or
learning mode was added.

Modulation visits every live outgoing local contact and computes the frozen
arithmetic response from its participation magnitude. Zero participation
arithmetically yields zero gain; there is no Boolean Modulation-admission
branch. The one remaining `participation_level > 0` coincidence is the
previously earned PQLC trigger, which the protocol explicitly preserves.

## Targeted validation

E2B sandbox `ixmxf0e4mbxm6zr81z717` ran only the required targeted work:

- `cargo fmt --all -- --check`: pass;
- `cargo check -p truelearner-core --all-features`: pass;
- core library tests: `14/14` pass, including R1-R5 differential mechanics,
  R6 resident partition invariance, live/quiescent checkpoints, compaction,
  stale generations, boundary buffers, and exact replay.

No Rust or project command ran locally.

## Boundary

This audit freezes what was implemented. Retained local-credit conformance
subsequently reached the protocol's mandatory negative stop; this candidate is
not development-ready and does not replace PD1, alter authority, or update the
oracle.
