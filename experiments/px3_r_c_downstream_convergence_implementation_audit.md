# PX3-R Arm C downstream-convergence implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; DEVELOPMENT EVIDENCE UNSPENT; PX3 ABSENT**.

## Candidate

- source:
  `crates/px0-physical-correspondence/examples/px3_r_c_downstream_convergence.rs`;
- source SHA-256:
  `8f8f8714af17d4de70e5c21660d8a5016a703ac872a131b564a6300f8aa47a31`;
- organism-visible block SHA-256:
  `088f8c0e20d3041272b3d1c30dc2e7f26190b8ee95df992c4f16d79d207eabc0`;
- source size: `53,971` bytes;
- protocol SHA-256:
  `f51d557d66757c04d7905aa54fae4ca71de4f5243f22d99cdb9273fd584bd80b`;
- first-collapse SHA-256:
  `5bc10db8be8625be3ebbe5ae67bad4986b2190d4b4d656db015204ae2517e56f`.

## Frozen inputs and negative preservation

Preflight checks, before any Arm C CELL can be constructed:

- exact frozen start tag resolves to
  `873094497ff6eb74363191dc5edc479c7d66de72`;
- exact authoritative PX2 tag resolves to
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` and remains ancestral;
- authoritative law/source hashes remain exact;
- frozen PX3 no-addition source, handoff, CSV, and report hashes remain exact;
- first-collapse and protocol hashes remain exact;
- all final and staging artifact paths for all three stages are absent;
- organism-visible forbidden-token scan is exact.

No authoritative or frozen file is modified. The candidate imports the public
frozen substrate API and adds no shared-code law.

## Physical opportunity implementation

The source constructs all four learned route motifs, all four ordinary
continuations, and all sixteen edge-local approaches from numeric physical
specifications. The only weak field is the full `4 x 4` set of resistance-`1`
approach-to-continuation ARROWs. Full-field replenishment iterates every numeric
route/continuation coordinate and adds only a missing live opportunity; it has
no input for the active routes, active continuation, evaluator label, or
expected result.

Each occurrence then supplies external SPIKEs to the actually active route
sources and participation drivers, and optionally to contemporary ordinary
continuation drivers. Physical continuation firing and local returned activity
alone determine which independently eligible weak ARROWs strengthen. Existing
ordinary pressure handles unsupported and stale ARROWs. No ARROW is reset,
deleted, strengthened, or selected by evaluator comparison.

There is no dedicated shared CELL. Common endpoint identity is measured after
execution from independently retained ARROW endpoints. There is no direct
trace-to-trace ARROW.

## Evaluator separation and serialization

Evaluator code exists only after the organism-visible end marker. It schedules
the preregistered physical arrivals, clones complete substrate state for
held-out use, reads traces/crossings/resistance/live state, computes common
endpoint overlap, and writes atomic artifacts. It cannot access private CELL or
ARROW state or mutate an opportunity selectively.

Individual correspondence and direction resistance arrays are serialized
separately from the opportunity resistance/live/measured-impulse matrices.
Measured opportunity impulse is obtained by activating one route at a time on
discarded complete-state clones and reading physical SPIKE impulses at each
ordinary continuation.

## Pre-evidence validation

- focused formatting: pass;
- focused compile: pass;
- strict focused Clippy with `-D warnings`: pass;
- no-argument refusal: exit `2`, pass;
- wrong-argument refusal: exit `2`, pass;
- no-CELL preflight marker:
  `PX3_R_C_DOWNSTREAM_CONVERGENCE_PREFLIGHT_OK`, pass;
- forbidden-token scan of organism-visible block: zero hits;
- result and staging artifacts: absent;
- PROBE/MICRO/GATE evidence markers: absent;
- authoritative PX0-PX2 files changed: none;
- broad historical suite: not run because shared code is unchanged;
- definitive command/matrix: nonexistent and not run.

No opportunity CELL, stage cell, control, duplicate, or result has executed.
The three development commands remain unspent.
