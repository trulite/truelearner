# PX3 recursive compression GATE implementation audit v1

Status: **FROZEN IMPLEMENTATION; E2B STATIC VALIDATION PASSED; PREFLIGHT TARGET**.

- source commit: `da472e5867a34c089df863d61f385a801d4eb9f6`;
- manifest SHA-256:
  `7cc3c067e62c4df5cb34599d9f9f05b854a2e4cf23202e7a1b51354a54618a3e`;
- source SHA-256:
  `969042a740f92237c577d82c67399447040cb96d2c003c28100034566e30d5aa`;
- protocol SHA-256:
  `e8ae58089208f475bf1e7b897a8ea2d8f13a819d566c89571afa0a7eef2a250d`;
- execution-protocol SHA-256:
  `5b55f4db4e1793bef55415ea80ce6bb5e1e5ff2b4c7d0f94ba9ce84837325399`.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` validated source commit
`da472e5`: formatting, 2/2 release static tests, and strict Clippy passed. No
GATE world or artifact was produced.

## Physical implementation audit

The authoritative PX0 source remains byte-identical at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
The executable hashes and audits frozen MICRO-A and D2-A inputs before command
dispatch.

The active topology contains:

- four primitive source/outlet participants and three derived outlets X/Y/Z;
- one `normalize` helper used for all seven participant outlets and all three
  stage P cells;
- three instances of one `Stage` shape, differing only in their input trace
  CELL identities and physical namespace;
- no enum or field that marks a participant primitive, derived, composite or
  event-like;
- zero P->output candidate ARROWs at construction;
- exactly one possible distance-one output adjacent to every P, with reflected
  worlds placing it on the opposite side;
- generic external threshold crossing at P as the only proposal route;
- one output participation trace serving both downstream participation and
  completed-effect attribution;
- threshold-three trace attribution returning unit impulse to threshold-two P.

The harness uniformly broadcasts background to all three P cells, context to
all three outputs, and ordinary return to all three attribution cells at every
active depth tick. It schedules primitive sources and physical time only. It
does not insert, select, revive, strengthen or delete a candidate.

Context-free held-out controls omit both context and return. Mature candidate
coupling two must therefore execute its threshold-two output directly; the
output's fixed unit normalizer, rather than candidate amplitude, determines
the downstream trace. Gapped X/C and Y/D controls preserve both participant
executions while removing trace overlap.

## Execution boundary

`--preflight` audits hashes, the four unique world configurations, forbidden
surfaces and artifact absence, then prints one marker. It constructs no world,
propagates nothing and writes nothing. Only `--gate` can emit the evidence
marker and atomically create the two registered artifacts.

The next clean commit containing this audit is the exact E2B preflight target.
Evidence remains unspent until that commit passes preflight and the sole
registered command is deliberately executed once.
