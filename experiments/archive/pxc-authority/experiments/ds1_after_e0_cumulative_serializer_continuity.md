# E0-B serializer and provenance continuity proof

Status: development-only interface continuity proof.

The complete `src/ds_e0_anonymous_event_formation.rs` SHA-256 is
`fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615`,
identical to enabling parent `d154fde`. Therefore E0-A, `EventRelations`,
`serialize_once`, and the embedded frozen DS1 learner are byte-identical.

The frozen provenance audit
`experiments/ds_e0_cumulative_serializer_provenance_audit.md` remains
byte-identical at SHA-256
`edfa40c5cedd9359b0677d36a26844a502a2c5b05e7f4c0b137c8922ff4c11a7`.
Its one-to-one mapping remains the only serialization path:

| Destination | Already-formed E0 source |
|---|---|
| `pair[0].identity` | canonical current member 0 |
| `pair[1].identity` | canonical current member 1 |
| `witness.identity` | canonical current member 2 |
| each `position` | corresponding propagation rank |
| each `tick` | corresponding temporal rank |
| each `window` | corresponding attachment-equivalence bit |

GATE produced 96 exact copies per seed. The independent probe then formed one
additional current E0 event, serialized it exactly once, and invoked the
unchanged frozen learner's read-only `frozen_choice` on that produced
`Neighborhood`. It returned unavailable because no DS1 pattern had been
acquired. This is interface consumption, not maturity, correctness, or a
claim-eligible result.

The composition harness has no raw-activity, candidate, `EventRelations`, or
serializer access. It cannot search, select membership, infer a window,
canonicalize, or read evaluator truth. It receives only the public DS-E0 audit
report after the frozen path has run.
