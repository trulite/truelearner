# CJ0-D local-subunit PROBE v1 first-clause failure

Status: **INVALID MECHANICAL ABORT; NO SCIENTIFIC CLASSIFICATION**.

The sole v1 PROBE invocation emitted
`CJ0_D_LOCAL_SUBUNIT_PROBE_EVIDENCE_SPENT` and then aborted at the retained
`PlasticSubstrate::advance_time` guard:

```text
physical time cannot run backward
```

The panic was at generated authoritative-body line `235`, the exact assertion
`tick >= self.tick`. No PROBE CSV, report, staging artifact, positive/negative
classification, MICRO, GATE, recursion, OR, timing, definitive, or authority
execution was produced.

This is an evaluator schedule defect. It does not falsify or support the
candidate law and may not be interpreted as scientific evidence. The v1
invocation is not rerun. Any correction requires a separately committed and
tagged fresh protocol, explicit current-tick instrumentation, and a fresh
artifact namespace.

Frozen implementation commit/tag:
`6cd7ee1`, `cj0-d-local-subunit-development-implementation-v2`.

