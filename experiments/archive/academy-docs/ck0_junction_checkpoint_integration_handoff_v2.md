# CK0 junction checkpoint integration handoff v2

Status: development-ready.

CK0 v2 establishes the checkpoint contract required by junction-derived
lifetime without changing runtime physics.

## Frozen result

- CK0 cases: `20/20`;
- CK0 rows: `40/40`;
- CK0 clauses: `216/216`;
- Reference/Production physical continuation: exact;
- same-mechanics replay: exact;
- natural quiescence: exact;
- maximum CK0 PhysicalWork: `4`.

Checkpoint restore now treats `CELL.live`, `CELL.generation`, and dormant
legacy `CELL.resistance` as independent serialized fields under J0. Live
topology and generation remain the physical liveness authority. The runtime
is byte-identical to the CK0 v1 candidate at
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

The corrected evaluator compares the causal checkpoint contract: durable
body, clock, canonical pending activity, explicit PhysicalWork, and identical
continuation. Raw serialization hashes remain diagnostic and observer tracing
is explicitly restored before continuation.

## Retained lineage replay

The authorized retained prefix was replayed unchanged after CK0 v2:

- J0: `160/160` cases, `320/320` rows, `1880/1880` clauses;
- CV0/J0+SV1: `240/240` cases, `480/480` rows, `5480/5480` clauses;
- Reference/Production history, replay, and quiescence: exact throughout;
- maximum PhysicalWork: J0 `10`, CV0 `35`.

Evidence hashes:

- J0 matrix: `c581c71bd4871d51e8ed45ac916b553c8e867c3e276820d012730f551a37c5a1`;
- J0 report: `1fadfc8f3cd06dfa33441a732034ae16a386b3e8487c2b2851ad61883ef772fd`;
- CV0 matrix: `3a19e2f769c692dd05461970cdddfc8e6bc3c0375a26a9c78298d5011ec4a1d9`;
- CV0 report: `a4cf0418a6f7503657109e161af8cb166e574f8620609f84c73087c94fce28c4`.

## Handoff

CK0, J0, and CV0/J0+SV1 are development-ready as one cumulative prefix.
The next permitted action is one unchanged consolidated RS2 execution on this
exact parent. No authority or oracle state advances through this handoff.
