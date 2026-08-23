# PX3-R direct trace-coupling generic-opportunity PROBE v1 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; PROBE EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen implementation candidate

- source:
  `crates/px0-physical-correspondence/examples/px3_r_trace_coupling_generic_opportunity_probe.rs`;
- source SHA-256:
  `b0d13b59cf9e89ea2d41cc3807aaea40ced78b615b9ac5dbb64623ed82a378b8`;
- organism-visible block SHA-256:
  `255ea3c8b3b8fd557594d19223c8fff0286fd22b3be5c02aca91bb1e4bba629f`;
- protocol SHA-256:
  `2918aeb4088f387617708a0bd05a25aa4290c260d9121b3f0aea4853030f0599`;
- exact Class-D start:
  `873094497ff6eb74363191dc5edc479c7d66de72`;
- authoritative PX2 ancestor:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

The protocol is frozen at commit
`76ecfe0dbb77bd1dc36ba0a9bd1b8dcc262a63ae`, tag
`px3-r-trace-coupling-generic-opportunity-probe-v1-protocol`.

## Implementation isolation

The candidate source imports the authoritative substrate without changing it.
The actual-participation cell contains four anonymous physical paths whose
external driver activity propagates through ordinary ARROWs to four nearby
trace-bearing CELL loci. The direct-external reference is a fresh substrate
and cannot feed back into the actual cell. No arm-specific law, seeded local
edge, recruited shared CELL, shared convergence, typed adapter, or conceptual
relation state exists.

The evaluator sees only completed `Execution` traces and public substrate
measurements after natural queue drain. It serializes per-route firing counts,
generic proposal count, local ARROW count, complete/permanent fingerprints,
quiescence, replay equality, autonomous source refiring, and all work-ledger
fields. It cannot select a local update or endpoint.

## Frozen-input and forbidden-information audit

- authoritative PX0--PX2 law SHA-256 remains
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen PX3 negative handoff SHA-256 remains
  `a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b`;
- frozen negative CSV SHA-256 remains
  `685dc04db32a5785224c62ba5b589fa8e1e37382a8b613f5f2b5e396aa005f38`;
- exact authority tags resolve to the preregistered commits and the frozen
  Class-D start is an ancestor of the implementation;
- the organism-visible block contains none of Event, Episode, History, Pair,
  Group, member, boundary, semantic, evaluator, serializer, old-M3, DS3, or
  renamed equivalents;
- final and staging result paths are absent.

## Pre-evidence validation

- focused formatting: pass;
- focused compile: pass;
- strict focused Clippy: pass;
- no-argument refusal: exit `2` before source audit or any CELL;
- wrong-argument refusal: exit `2` before source audit or any CELL;
- no-CELL `--preflight`: pass with exactly one preflight marker;
- evidence-spent marker during validation: absent;
- authoritative/shared source changed: none;
- broad historical suite: not run because shared code did not change.

No PROBE cell, duplicate, result, or evidence marker has executed. The sole
`--probe` command remains unspent.
