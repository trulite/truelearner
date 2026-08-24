# PX-C readiness delta gate v1 audit

Status: factory guardrail frozen. No organism mechanism or PX4--PX8 authority
is changed.

## Lineage

- PX3+LR-C authority: `f9057fe78a86db9111b0b69310d03accef3bc970`
- immutable taxonomy baseline: `8c5e8b7e5615528807510d18ead7580b9248f2cc`
- comparator implementation: `caff3df2c56579d890fee5ce2d9eb2162281e3e5`
- complete control suite: `abdb54a90acd9dea5b92da89c2ab3ae3eb812ab4`
- E2B sandbox: `ir10qhnwx0sd9qpi0xe81`

## Gate

Every PX4--PX8 development-readiness handoff must attach raw before/after
taxonomy and guard inventories plus their summaries. The comparator
independently reconstructs:

- total occurrences and unique source lines;
- every primary-kind count;
- every layer count;
- semantic-condition guard count;
- evaluator-input guard count;
- seam kinds newly introduced or reintroduced after reaching zero; and
- guarded semantic surfaces newly appearing by guard, layer, path, and token.

It rejects summary/inventory disagreement before judging the scientific delta.
For a readiness claim, primary seams must strictly decrease; semantic and
evaluator guards may not rise; and both novelty counters must remain zero.

## E2B controls

The committed control suite produced:

```text
exact replay                 PASS
strict no-change claim       REJECT
consistent real reduction    PASS  (368 -> 367)
summary tampering             REJECT
rising primary total         REJECT (368 -> 369)
new seam kind                REJECT (1 new kind)
new semantic surface         REJECT (1 new surface)
```

The real-reduction positive removed one serialized episode-reset occurrence,
updated its raw taxonomy inventory, and consistently changed total, unique
line, kind, and layer counts. The comparator accepted it under strict mode.

The tamper control edited only the summary. Independent reconstruction found
`kinds=368` while the claimed total was `369` and failed closed before a delta
verdict.

The new-kind control held the total at 368 while trading an existing typed
representation occurrence for `semantic_adapter`. It was rejected and the new
kind was serialized.

The new-surface control added
`semantic_condition/PX4/arms/new_semantic_adapter.rs/correct`. It was rejected
and serialized even though it could have been hidden by removing other guard
occurrences in a net-only metric.

## Mandatory handoff table

```text
                         before   after
primary seams               N       M
semantic guard              X       Y
evaluator guard             P       Q
new seam kinds              0       0
new semantic surfaces       0       0
```

Functional success plus this table is necessary for development readiness.
Neither is sufficient for serial authority, which remains a later disjoint
workflow.
