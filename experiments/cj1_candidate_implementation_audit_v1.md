# CJ1 path-local saturation candidate implementation audit v1

Status: **IMPLEMENTED AND FROZEN; PROBE EVIDENCE UNSPENT**.

## Frozen source

- authoritative PX0 input SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- candidate PROBE protocol SHA-256:
  `9481bdf46793475ac6ca28d35910ddc51dd7cafbbd94ea9a7ae7ffc2f68b9984`;
- `arms/cj1-distinct-path-coincidence/build.rs` SHA-256:
  `9cde4fa7ed6cac8dc11b9c1df7ed186c1de868d9ceafddb3ad4e0a911a75dee1`;
- `arms/cj1-distinct-path-coincidence/src/lib.rs` SHA-256:
  `7e6a45fb00017782c38a5a09b3cbebeee376cfac2ef9afe58f419efb82381fe8`;
- `arms/cj1-distinct-path-coincidence/src/bin/cj1_candidate_probe.rs` SHA-256:
  `6085e98d81b33082349e8e8032e3bcdb83328c18b33afa2ceee1c4135670cded`;
- `arms/cj1-distinct-path-coincidence/Cargo.toml` SHA-256:
  `ff4cb9dc82dc080a736a600ec636af4f73e9238897dd674d198a84411fd46dfd`.

## Mechanical audit

The build script refuses PX0 hash drift and refuses unless the full original
traversal/enqueue block occurs exactly once. Its sole scientific replacement:

- reads the traversed ARROW's existing `eligible_until` before the ordinary
  rewrite;
- changes only the enqueued `Spike.impulse` for a threshold-greater-than-one
  destination to bounded first contribution `1` or live-trace repeat `0`;
- leaves threshold-one destinations at ordinary `arrow.coupling`;
- performs the same eligibility rewrite and work accounting afterward.

The generated isolated library otherwise contains the exact frozen PX0 source.
It adds no CELL, ARROW or SPIKE field, no persistent object, no evaluator fact,
no identity comparison and no invalidation or closure signal. The authoritative
PX0 dependency remains present for the frozen unchanged-physics executable;
the candidate executable imports only the generated isolated library.

The probe runner exposes one command and four write-once destination/staging
paths, asserts both parent protocol hashes and the authoritative PX0 hash, and
contains only the preregistered two-seed, thirteen-scenario PROBE. It contains
no MICRO, GATE, definitive, authority, PX3-restart or PX-C execution surface.

No candidate evidence was executed while this audit was prepared. The first
E2B preflight refused on formatting before compilation; this audit incorporates
only that exact formatter diff. A subsequent preflight passed formatting,
focused tests and strict Clippy; root-relative runner paths were then corrected
before execution to match the exact preregistered root command. This snapshot
supersedes the unexecuted `ed50290` and `a31443f` implementation snapshots. A
final focused preflight is to be run from this clean frozen commit in the
established E2B sandbox before the evidence command.
