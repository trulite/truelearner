# PX3-R direct physical trace coupling PROBE v2 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen retry candidate

- isolated manifest: `arms/px3-r-trace-coupling-v2/Cargo.toml`;
- arm law SHA-256:
  `0027d8356170a673c3045980fed8a5a3f1509277072753bba03cb3b43143f6c9`;
- physical world SHA-256:
  `abec17d32b110538133044f2366b27f4104b632101ed2d0f34e13f10d484a409`;
- evaluator SHA-256:
  `de2f55f74f0bc8a9b12cd0ad07c403278ecc72bcf61fa097f470be77f3a8569e`;
- lockfile SHA-256:
  `41e7bfc085a18f1b301a2f61945a30dbbd5c9f2224932b652b452f6aa3b0e429`;
- retry protocol SHA-256:
  `f671332544b3e7bf0e291d22fcf10ff4416b3f1bfa0b710e07a554933cb19a6c`.

The retry protocol is frozen at commit
`9f096208646f3b7cafafe9b62b3b847ce110cf0a`, tag
`px3-r-direct-trace-coupling-probe-v2-retry-protocol`.

## Exact delta from frozen v1 implementation

The v1 arm law was copied exactly. V2 adds one method that installs the same
preregistered numeric opportunity once, only on an empty queue and only when
absent. The world now acquires correspondence, adds weak directions, gives all
four routes one matched actual maturation occurrence with the opportunity
absent, accounts that work, then installs the opportunity. Candidate schedules
begin at tick `84`. Fresh namespaces and result paths are used.

No return, pressure, coupling, propagation, proposal, fingerprint, trace,
route, or evaluator law changed. V1 source/artifacts remain hash-exact; its CSV
is audited at runtime. Authoritative PX0--PX2 remains SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Pre-evidence checks

- formatting, focused compile, strict Clippy: pass;
- no-argument/wrong-argument refusal: exit `2`;
- no-CELL preflight: pass with one marker;
- lineage, authority hash, v1-result hash, protocol hash, executable forbidden-
  token audit: pass;
- final and staging paths: absent;
- isolated source/manifest/lock storage: `65,088` bytes;
- shared files changed: none;
- broad historical suite and authority matrix: not run.

No V2 cell, duplicate, control, result, or evidence marker has executed. The
sole `--probe` command remains unspent.
