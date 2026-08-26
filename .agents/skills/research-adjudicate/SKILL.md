---
name: research-adjudicate
description: Adjudicate immutable research evidence against a frozen protocol and record a positive, negative, or inconclusive scientific verdict without repairing the experiment. Use after a frozen run, independent audit, replication, or candidate-physics integration result.
---

# Research Adjudication

```text
Frozen protocol
+ neutral evidence
        |
        v
Verify exact lineage,
run policy, and artifacts
        |
        v
Compare every predicate
and negative control
        |
        v
Positive, negative,
or inconclusive
        |
        v
Record sufficiency,
integration, adoption,
and authority separately
```

## Rules

- Validate the protocol and evidence before interpreting results.
- Judge observations against the frozen protocol, not against implementation success.
- Preserve negative and inconclusive outcomes without repair, rerun, or acceptance edits.
- Separate behavioral truth from a potentially stale contract; revise a stale contract only through a separate successor decision.
- Treat software success as neither scientific success nor adoption.
- Require clean integration, removal of experimental wiring, unchanged controls, replay equality, Production equality, natural quiescence, and no hidden authority before adoption.
- Require explicit authorization and independent evidence before authority promotion.
- Write `research/templates/adjudication.toml` and validate it with `uv run research/validators/validate_adjudication.py`.
- Do not invoke factory skills or mutate their receipts.
