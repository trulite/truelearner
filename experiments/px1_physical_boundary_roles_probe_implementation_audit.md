# PX1 physical boundary-role PROBE implementation audit

Status: **IMPLEMENTED; DEVELOPMENT EVIDENCE UNSPENT**.

## Frozen inputs

- authoritative PX0 commit:
  `e884ae133a562d475565a36700d929b51dd2b2d2`;
- PX1 PROBE protocol commit:
  `caff6b89c42ca6db224820d65d1887f50bbf39ab`;
- protocol SHA-256:
  `39777626848a61d0e3f9d13a9f778fdb22fed588c5f2a89a657636ee2428e3e9`;
- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The PX0 law is unchanged. The only active addition is the development example
`crates/px0-physical-correspondence/examples/px1_boundary_role_probe.rs`,
SHA-256
`1fb0168729e4181a8e778a93f92ebfae7f10576e66d6ef0aa99bc3050a3021a8`.

## Causal-path audit

The wrapper creates two anonymous physical layouts and retains `CellId` handles
only for world arrivals and evaluator scoring. It never reads PX0 arrow
endpoints to construct an input object or reinjects endpoint activity during
role learning.

The active path is:

1. three anonymous source spikes fire a source cell;
2. a PX0-proposed and return-matured arrow carries activity to its endpoint;
3. the endpoint fires from that internal arrow traversal;
4. pre-existing weak endpoint-local proposals are exercised;
5. ordinary return reaches the endpoint in one physical world arm;
6. the same PX0 local-return and pressure law changes physical persistence;
7. evaluator-only cloned held-out source activity measures boundary crossing.

Direct source-to-role arrows are excluded by physical distance and explicitly
audited. The broken-path control moves endpoints outside PX0's proposal radius
while retaining endpoint opportunity and return physics.

## Pre-evidence validation

- formatting passed;
- focused compilation passed;
- strict Clippy passed;
- frozen PX0 focused tests passed `1/1`;
- authoritative source/artifact hashes passed;
- definitive invocation refused with exit `2`;
- PROBE result paths were absent;
- no old M0/M1 source or mechanism is linked into the example.

No PROBE world has executed yet. The committed implementation is ready for one
development execution using `--probe`.
