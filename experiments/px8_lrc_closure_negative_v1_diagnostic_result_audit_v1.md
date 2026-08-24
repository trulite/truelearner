# PX8 closure negative-v1 diagnostic result audit v1

Status: **DIAGNOSTIC COMPLETE; MEASUREMENT/FIXTURE DEFECT FROZEN; NOT AUTHORITY**.

## Execution and artifact integrity

The one registered diagnostic matrix ran from frozen commit
`122c500a56601cc02e77a0c11a16561210ab0504`, tagged
`px8-lrc-closure-negative-v1-diagnostic-frozen-v1`, in fresh E2B sandbox
`i102ns1mer9f642nea65x`, state file
`px8-lrc-negative-v1-diagnostic-execution.json`.

It emitted the distinct diagnostic marker exactly once, reconstructed all
sixteen fresh roots twice, serialized all 224 clause records, and then passed
the result-mode static audit in the same sandbox:

```text
PX8_LRC_CLOSURE_NEGATIVE_V1_DIAGNOSTIC_SPENT
PX8_LRC_CLOSURE_DIAGNOSTIC_COMPLETE roots=16 clauses=224 failures=16 authority=false
PX8_LRC_CLOSURE_DIAGNOSTIC_AUDIT_OK ... mode=result active_changes=0 evaluator_sources=1 unclassified=0
```

| artifact | SHA-256 |
|---|---|
| diagnostic CSV | `c07f16515a5d4244242130c0eba82374a28a24d0564f21721c6c523943c5ec60` |
| diagnostic report | `9f8a6abdccc1e97a07555d232bc56507e7056c67c9d1d231eec0d0ff3be7f8a5` |

No authority-v1 marker, authority result path, PX8 promotion, or authority
claim was emitted. No Rust, project program, or project audit ran locally.

## Exact localization

Exactly one clause failed in each of roots `862001..862016`:

```text
clause 12 bounded_stable_memory
expected: maximum_bytes<=8192 && memory_stable==true
actual:   maximum_bytes=5488~memory_stable=false
```

All roots had first failed clause `bounded_stable_memory` and first replay
divergent state `none`. Thus the failure is invariant across all four
construction/reflection strata and all four twists.

Every other clause passed on all sixteen roots:

- formation qualified modulation and updates;
- completed exactly-once outward crossing;
- completed exactly-once physical return and qualified update;
- incomplete, blocked, stale, open, and aged silence;
- zero-length and duplicate exactly-once behavior;
- branch and cycle silence;
- pause/resume equality;
- full queue exhaustion and natural quiescence;
- maximum work bound;
- cumulative PX0--PX7+LR-C conformance; and
- exact duplicate-state replay.

Exact resource and cumulative observations were:

| observation | diagnostic value |
|---|---:|
| maximum PX8 field work | `14788 / 20000` |
| maximum persistent bytes | `5488 / 8192` |
| exact replay | `16/16` |
| full natural quiescence | `16/16` |
| PX7 first training modulation / updates / work | `2 / 2 / 713` |
| PX7 second training modulation / updates / work | `2 / 2 / 753` |
| PX7 held-out outward / modulation / updates / work | `1 / 0 / 0 / 514` |
| PX7 mature coupling / resistance | `2|2 / 6|6` |
| PX7 bytes / within-body stability | `2304 / true` |

## Frozen classification

Classification: **measurement/evaluator/fixture defect**.

Authority v1 computed `memory_stable` by comparing every recursive control
body's final allocation against `trained_bytes` from the primary
`learn_twice()` body. That aggregate included bodies with deliberately
different fixture histories, most importantly the stale control formed by
`learn_once_then_age()`. Cross-fixture allocation equality is not persistent
memory stability.

The preregistered claim required persistent size to remain stable across pause
and held-out reuse of a retained body. The evaluator instead imposed equality
between separately constructed bodies with different formation histories.
This is an evaluator measurement mismatch. It does not show growth during
reuse, exceed the memory ceiling, change any physical crossing, break replay,
leave queued work, or require a substrate-law change.

The exact justified repair is to capture persistent bytes immediately before
and after each body's held-out operation and require equality within that same
body. Primary completed, incomplete, duplicate, blocked, stale, and cumulative
fixtures may each retain their own baseline. Equality between different
fixture histories must not be required. The `8192` maximum remains unchanged.

No active mechanism or retained law may change. Authority v1 remains the
immutable negative and may not be rerun or relabeled.
