# RS2 learned inhibitory topology post-CK0 immutable negative v1

Status: complete immutable negative.

Protocol: `8aa7500`
(`rs2-learned-inhibitory-topology-post-ck0-protocol-v1`).
Frozen evaluator: `538d1f9`
(`rs2-learned-inhibitory-topology-post-ck0-frozen-v1`).
Fresh E2B worker: `iqz7isfpr1k5kwaob09ey`.

## Result

The complete matrix executed exactly once and published all evidence before
the frozen final assertion stopped:

- cases: `180/180`;
- rows: `360/360`;
- clauses: `3440/3520`;
- Reference/Production wave-normalized equality: `360/360`;
- identity renaming equality: `360/360`;
- same-mechanics replay equality: `360/360`;
- meaningful checkpoint continuation: exact;
- maximum PhysicalWork: `128`.

CK0 resolved the prior pre-row checkpoint failure. No restore, stale-reference,
representation, renaming, insertion-order, replay, or mechanics divergence
remained.

## Exact failures

Exactly `80` rows failed: two predicates across both roots, all ten phases,
and both mechanics.

### No-Modulation control: 40 rows

The generated candidates disappeared, anchors remained unchanged, and the
uninhibited recurrence remained non-quiescent with `11/10` target/source
firings. The predicate nevertheless required both non-quiescence and the
observation-ceiling marker. The run was non-quiescent but did not mark the
ceiling, so `uninhibited_recurrence_persists` failed.

### Useful-positive control: 40 rows

The positive relation was selected and retained, the negative relation was
removed, anchors remained unchanged, and the selected contact fired again
(`1 -> 2`). The predicate searched for the pre-WS0 ordered
`PhysicalEvent::Deliver` observation with impulse `+1`; under WS0 the Drive
arrivals are represented by a wave-level incidence, so its count remained
`0 -> 0`. Therefore `positive_relation_reexecutes` failed despite the contact
re-execution.

These are systematic observation/predicate boundary failures, not a CK0
checkpoint recurrence. They strongly suggest evaluator integration debt, but
this gate does not relabel the scientific result: its frozen predicates did
not all pass.

## Passed RS2 families

All rows passed for:

- learned negative selection and recurrence stabilization;
- candidate identity/sign-order permutation;
- inhibitory location permutation;
- irrelevant-negative rejection;
- disconnected learned inhibitor control;
- untraversed learned inhibitor control;
- fresh recurrence packing.

The principal positive family recorded selected negative link resistances
`[4,4]`, unsupported relation removal, exactly one intended firing at each
cycle cell, one learned-contact firing, and natural quiescence.

## Evidence

- matrix SHA-256:
  `ab507b7548653679f37f97b29b77cb1df2571b6607d708563dd47283f4823eb5`;
- report SHA-256:
  `f49ff57c3a657f224439b208062191d1e6fd107c1bb497776036d37390fb12f8`.

The canonical runtime remained unchanged at
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.
No repair or rerun occurred.

