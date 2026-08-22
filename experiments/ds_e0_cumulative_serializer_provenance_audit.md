# DS-E0-B one-to-one serializer provenance audit

Status: **E0-B READY FOR INTERFACE CONSUMPTION** in development only. This is
not cumulative DS1 success.

`serialize_once` accepts exactly `&EventRelations` and `&mut WorkLedger`. It
cannot access `RawActivity`, unselected spikes, the formation learner,
evaluator membership, a consequence closure, or a control label. It performs
one fixed construction and returns. No loop, candidate search, sort,
comparison, relation derivation, or fallback exists in the function.

## Exhaustive field mapping

| Destination DS1 field | Single existing E0 source | Operation |
|---|---|---|
| `pair[0].identity` | `members[0]` | width conversion and copy |
| `pair[1].identity` | `members[1]` | width conversion and copy |
| `witness.identity` | `members[2]` | width conversion and copy |
| `pair[0].position` | `propagation_rank[0]` | copy |
| `pair[1].position` | `propagation_rank[1]` | copy |
| `witness.position` | `propagation_rank[2]` | copy |
| `pair[0].tick` | `temporal_rank[0]` | copy |
| `pair[1].tick` | `temporal_rank[1]` | copy |
| `witness.tick` | `temporal_rank[2]` | copy |
| `pair[0].window` | `attachment_equivalence[0]` | copy |
| `pair[1].window` | `attachment_equivalence[1]` | copy |
| `witness.window` | `attachment_equivalence[2]` | copy |

`EventRelations.temporal` and `.propagation` are retained in temporary state
as the complete provenance matrices. The ranks/equivalence bits above are
already materialized by E0-A before E0-B is called; the serializer never
reads or recomputes the matrices.

The focused exact-copy test uses distinct values in every source category.
The development matrix then checks every emitted field on every serialized
event. MICRO records 48/48 exact copies. Each GATE seed records 96/96 exact
copies across integrated, same-timing/different-propagation,
same-propagation/different-timing, relabeling, allocation, and layout arms.
Each successfully formed event is serialized once. A separate formed event is
serialized once for the optional downstream probe.

The downstream marked DS1 extraction is byte-identical at SHA-256
`adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`.
The optional read-only probe consumes the schema but correctly returns no
mature route from a default frozen DS1 learner. The next missing prerequisite
is frozen DS1 acquisition/consequence history. No DS1 rescue was attempted.
