# PX3 physical event-boundary no-new-mechanism PROBE v4 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen candidate

- source:
  `crates/px0-physical-correspondence/examples/px3_physical_event_boundaries_probe_v3.rs`;
- source SHA-256:
  `39ec595fc1204a29083d271ebcadcdb7950c07d1c44e4ce07c0107fca54730ba`;
- organism-visible block SHA-256:
  `ac11bd435098469cdf2a16b3d75dddf4285396c3a75aa31a87bff1f775142fee`;
- v3 protocol SHA-256:
  `2d36cd60a08c16abb41eb596dde3ddcfe70515731756f40cfdf665b2e774d4f9`;
- v4 protocol SHA-256:
  `fbb6661861b796bdaf4be5d93afd71b1ac42a09168dc2e6d3008c5bc996695f8`;
- exact frozen PX2 parent:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

The organism-visible block hash is byte-identical to failed v2. V4 changes no
organism code: only evaluator-side fresh namespaces, the whole-stream external
phase, relation-normalized read-only comparison, frozen-source audits, fresh
artifact paths, marker, and the isolated subthreshold observation gap differ.

## Frozen-negative preservation

Preflight verifies exact hashes of the failed v2 source and both failed v2
artifacts before it can enter a CELL. V2 remains byte-identical:

- source:
  `a15f2b1b5070d3fc707b68d0a4f7135834efbd9fc919e6a3c27d60f7751afad9`;
- CSV:
  `5ae68dad569f943d08d945014ed5491a93eb30021b1afc78966ed39fd15b4cc4`;
- report:
  `941c1754004b43eee9a99b969bfbad9e1fd75257091fc475e4dbb196084eef66`.

No v2 path is reused. V3/v4 namespaces and final/staging artifacts are fresh
and absent.

## Exact physical construction

The physical inputs remain the fourfold population-only replication of the
authoritative PX2 motif documented in the v2 implementation audit. There is
no relation CELL, pair ARROW, semantic type, negative coupling, new trace,
new threshold, new resistance, new update, hidden cut, reset, or cutoff.

V4 queues all acquisition or held-out arrivals before one propagation. The
only main-stream difference is external arrival phase `64` instead of `66`.
The isolated once-returned control advances ordinary physical time by `70`
ticks so the already-authoritative pressure law deallocates resistance `6`.
Advance work is fully ledgered.

The read-only relation normalization sorts four recorded route resistance/live
values only after execution. It neither serializes state into the organism nor
selects a local path. Exact unnormalized arrays and fingerprints are also
published.

## Pre-evidence validation

- formatting: pass;
- focused compile: pass;
- strict focused Clippy: pass;
- no-CELL preflight: pass with the exact v3 marker;
- organism-visible forbidden-vocabulary scan: pass;
- organism-visible block byte identity with v2: pass;
- frozen PX0–PX2, old behavioral reference, v2 source/result, and v1–v4
  protocol hashes: exact;
- exact authoritative parent ancestry: pass;
- v3/v4 final and staging artifacts: absent;
- evidence marker during validation: absent.

No v3/v4 cell, duplicate, control, result, or evidence marker has executed.
The sole v4-authorized `--probe` command remains unspent.

