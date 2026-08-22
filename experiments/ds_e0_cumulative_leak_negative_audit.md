# DS-E0 cumulative-development leak and negative-control audit

No definitive command ran and no result artifact was written.

## Static boundary audit

The organism-visible input types contain only:

```text
Spike { occurrence, local_tick }
Propagation { from, to }
RawActivity { spikes, propagation }
```

The persistent mechanism contains only `BTreeMap<RelationShape,
ShapeEvidence>`, work counters, and invalidation/reopening counters.
`RelationShape` contains complete relative temporal and propagation matrices;
`ShapeEvidence` contains strength, success/failure counts, maturity, and
contradictions. Neither can contain an occurrence or episode membership.

The temporary value contains current membership, the two complete relation
matrices, and their already-derived relative ranks/equivalence bits. It has no
event ID, supplied grouping ID, semantic endpoint, evaluator value, future
destination, or stable cross-episode token. Canonical ordering reads matrix
values, not occurrence numeric values, allocation order, or layout.

The only `pair`, `witness`, and `window` destination names are inside the
byte-frozen DS1 schema and the format-only serializer/provenance checks. They
do not exist in E0 input, persistent state, candidate descriptions, or
temporary relation formation. `Fixture.selected` and `.misleading` are
evaluator-only sets used after a physical candidate proposal to emit ordinary
consequence or score behavior; neither is reachable from `form` or
`serialize_once`.

## Dynamic GATE controls

Every seed `100..104` produced the same observations:

| Control | Observation per seed |
|---|---|
| integrated interleaved candidates/distractors | 16/16 correct temporary formations |
| same timing, different propagation | 16/16 correct formations |
| same propagation, different timing | 16/16 correct formations |
| fresh IDs and bijective relabeling | 16/16; acquisition/evaluation sets disjoint |
| allocation and absolute-layout change | 16/16 each |
| ambiguous equally described candidates | 16/16 abstentions |
| shuffled timing | 16/16 abstentions |
| shuffled propagation | 16/16 abstentions |
| no structure | 16/16 abstentions |
| random consequence | no evaluator-competent recovery |
| deliberately misleading consequence | no evaluator-competent recovery |
| equally time-proximate fixed-window baseline | 0/16 correct selections |
| contradiction/reopening | four shapes invalidate after exactly two contradictions and reconsolidate generically |
| persistence leak | zero retained occurrences and zero retained memberships |
| serializer | 96/96 exact field copies; no search or inference |

The fixed-window baseline is a separate evaluator-only function and shares no
state or choice with E0. Its complete failure while E0 is 16/16 proves a
fixed proximity cluster cannot pass this fixture.

## Negative conclusion boundary

E0 readiness says only that legal raw anonymous relations can produce a
temporary interface-sufficient event. It supplies no cumulative DS1
acquisition, mature route, functional recovery, recursion, compilation, FFS,
or economics. The optional default-state frozen DS1 probe returns unavailable;
that missing downstream history is recorded and not rescued.
