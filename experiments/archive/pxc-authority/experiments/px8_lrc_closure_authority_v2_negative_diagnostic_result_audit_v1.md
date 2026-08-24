# PX8 authority-v2 negative diagnostic result audit v1

Status: **DIAGNOSTIC COMPLETE; FIXTURE/EVALUATOR DEFECT FROZEN; STOPPED; NOT AUTHORITY**.

## Execution and artifacts

The one registered v2-negative diagnostic ran from frozen commit
`73ef33b1082922d6634005e31eb7a135b28390f5`, tagged
`px8-lrc-closure-authority-v2-negative-diagnostic-frozen-v1`, in fresh E2B
sandbox `ivwrbmh1rijq5c4dzw9dw`, state file
`px8-lrc-v2-negative-diagnostic-execution.json`.

It emitted its distinct diagnostic marker once, serialized every record, and
passed the non-executing result audit:

```text
PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_SPENT
PX8_LRC_CLOSURE_AUTHORITY_V2_DIAGNOSTIC_COMPLETE roots=16 clauses=224 failures=16 authority=false
PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_AUDIT_OK ... mode=result active_changes=0 evaluator_sources=1 unclassified=0
```

| artifact | SHA-256 |
|---|---|
| diagnostic CSV | `fca01ce4cea457c9bbfcdece78f469e654ff350704c214be7e1296264c1019ae` |
| diagnostic report | `e9fe49aa44b4126c824d306ff52368ef745b11921559067157ba91b6d7e8b551` |

No authority marker, authority result, PX8 promotion, manifest change,
taxonomy, or comparator occurred. No Rust, project program, or project audit
ran locally.

## Exact failure evidence

Every root `864001..864016` failed exactly clause 12
`bounded_same_body_memory`. All construction/reflection/twist strata produced
the same observations:

```text
expected: maximum_bytes<=8192 && all seven before==after
actual:   maximum_bytes=5552; stable=false

primary         5488 -> 5488
uninterrupted   5488 -> 5488
incomplete      5488 -> 5488
duplicate       5488 -> 5488
blocked         5488 -> 5488
stale           5488 -> 5552
cumulative PX7  2304 -> 2304
```

The sole divergence is the stale pair: `+64` bytes. Every root's first failed
clause is clause 12 and first independently reconstructed divergent state is
`none`.

All thirteen other clauses passed on all sixteen roots, including:

- formation modulation/updates;
- completed outward crossing, physical return, and update exactly once;
- incomplete, blocked, stale, open, aged, branch, and cycle crossing silence;
- zero-length and duplicate exactly-once behavior;
- pause/resume equality;
- maximum work `14788 / 20000`;
- maximum bytes `5552 / 8192`;
- queue exhaustion and natural quiescence `16/16`;
- retained PX0--PX7+LR-C cumulative controls; and
- exact replay `16/16`.

## Frozen classification

Classification: **fixture/evaluator defect**.

The stale body is an intentionally different negative-control fixture. It is
formed once, aged until its learned route is physically stale, and then
exposed to reuse. Retained LR-C law may append a new structural proposal when
reuse encounters the dead route. The observed `64`-byte append is deterministic,
bounded, naturally quiescent, and produces no outward crossing.

Authority v2 incorrectly included this stale negative-control fixture in the
same allocation-stability conjunction intended to measure pause and held-out
reuse of retained mature bodies. Six retained/non-stale fixtures—including the
primary completed body and cumulative PX7 body—show exact before/after
stability. Therefore this is not evidence of unbounded memory, nonquiescence,
incorrect crossing, broken replay, or a need for mechanism/substrate-law
change.

No v3 protocol, evaluator, implementation, compilation, audit, or authority
evidence is created. Further repair design awaits parent/user review.
