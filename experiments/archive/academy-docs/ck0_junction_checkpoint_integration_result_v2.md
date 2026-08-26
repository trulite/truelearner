# CK0 junction checkpoint integration result v2

Status: complete development positive.

Protocol: `f297bc1c6dcb6638dfa80b4c80dcb26a9319c214`.
Frozen evaluator: `19e33cd1e72eb60de64e9c4773f3ced6b61a5f27`.
Fresh E2B worker: `i8n4yvko3ndpe9fl5xndt`.

## Result

- cases: `20/20`;
- rows: `40/40`;
- clauses: `216/216`;
- Reference/Production physical continuation exact: true;
- same-mechanics replay exact: true;
- natural quiescence: true;
- maximum PhysicalWork: `4`.

All ten frozen families passed under both disjoint roots and both mechanics:

- live junction checkpoint round-trip;
- dead junction with nonzero dormant resistance;
- generation-safe slot reuse;
- incoming and outgoing stale ARROW safety;
- live-topology retention;
- last-link junction death;
- live pending continuation;
- quiescent future behavior;
- Reference/Production continuation.

The runtime remained byte-identical to CK0 v1 at
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

Evidence hashes:

- matrix:
  `5dad361350f0c12e1f5b105c3c9c7c98bebf5e3cd865fa0c06f5425741827768`;
- report:
  `72b4c2cd9a318236efe5ddabca229073894acc2fe209aba099fb4b1096e28acb`.

CK0 now establishes:

> Junction lifetime and checkpoint liveness are topology/generation facts;
> dormant CELL resistance is not their authority.

The frozen prefix next permits unchanged J0 and CV0/J0+SV1 lineage replays.
RS2 remains blocked until both pass.
