# PX8 LR-C cumulative physical-closure authority v2 protocol v1

Status: **PREREGISTERED; IMPLEMENTATION ABSENT; AUTHORITY-V2 EVIDENCE UNSPENT**.

## Frozen basis

This protocol is directly parented by diagnostic classification commit
`eadc3edad648c19346f3bb7217cebdce77d97579`, tagged
`px8-lrc-closure-negative-v1-diagnostic-result-v1`.

Authority v1 remains an immutable negative at
`eca6245475bd680f1876822efc1230aea400a968` /
`px8-lrc-closure-authority-negative-v1`. It may not be rerun, repaired in
place, reinterpreted, or relabeled.

Frozen diagnostic evidence is:

| artifact | SHA-256 |
|---|---|
| active PX8 mechanism | `8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f` |
| negative-v1 diagnostic CSV | `c07f16515a5d4244242130c0eba82374a28a24d0564f21721c6c523943c5ec60` |
| negative-v1 diagnostic report | `9f8a6abdccc1e97a07555d232bc56507e7056c67c9d1d231eec0d0ff3be7f8a5` |
| retained LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| retained PX4 API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |
| retained PX7 source | `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e` |

The diagnostic localized all sixteen v1 failures exclusively to the
cross-fixture `memory_stable` measurement. All physical, cumulative, work,
quiescence, and replay clauses passed on all sixteen fresh diagnostic roots.

## Sole authorized repair

Authority v2 may change evaluator measurement only. The active PX8 mechanism,
retained PX0--PX7+LR-C laws, physical layouts, schedules, expected crossings,
work ceiling `20000`, byte ceiling `8192`, quiescence requirement, replay
requirement, clause count, and PX-C gates remain unchanged.

V1 incorrectly compared final allocation of separately constructed fixtures
with different formation histories to the primary `learn_twice()` allocation.
V2 must replace only that aggregate with same-body before/after measurements:

1. primary learned body: bytes immediately after `learn_twice()` and after
   completed reuse;
2. uninterrupted clone: bytes immediately before and after completed reuse;
3. incomplete clone: bytes immediately before and after incomplete reuse;
4. duplicate-recursive body: bytes immediately after its own `learn_twice()`
   and after duplicate reuse;
5. blocked body: bytes immediately after its own `learn_twice()` and after
   blocked reuse;
6. stale body: bytes immediately after its own `learn_once_then_age()` and
   after stale reuse; and
7. cumulative PX7 body: bytes immediately before and after held-out arrival.

Every pair must be equal, every observed size must be `<=8192`, and maximum
observed bytes must still be serialized. Allocation equality between different
fixtures or formation histories is forbidden. No physical predicate may be
removed, relaxed, or replaced.

## Fresh v2 identities and unchanged matrix

The sixteen authority-v2 roots are exactly `863001..863016`. Primary
namespaces are `root << 32`; compact controls use offsets
`10000,20000,...,60000`; cumulative PX7 namespaces are
`(root + 1_000_000) << 32`. These are disjoint from authority-v1 roots
`861001..861016`, diagnostic roots `862001..862016`, isolated development,
and all earlier serial evidence.

Construction/reflection and twists `0,137,274,411` remain exactly balanced.
Schedules remain byte-for-byte equivalent to v1 and the diagnostic:

```text
formation        learn_twice
complete         reuse all four at 61
incomplete       omit side four at 70
blocked          outward resistance 0, reuse at 61
stale            learn_once_then_age, reuse at 111
compact          direct/open/fork/ring at 0; aged at 10
PX7 cumulative   maturation at 0 and 10; held-out boundary at 20
```

## Frozen clauses and threshold

The fourteen row clauses remain:

1. formation qualified modulation and route updates;
2. completed exactly-once outward crossing;
3. exactly-once downstream return and qualified update;
4. incomplete, blocked, and stale silence;
5. open and aged silence;
6. zero-length exactly-once crossing;
7. duplicate physical and recursive exactly-once crossing;
8. branch and cycle silence;
9. inert pause and resumed/uninterrupted equality;
10. full natural quiescence and queue exhaustion;
11. maximum work `<=20000`;
12. maximum bytes `<=8192` and all seven registered same-body before/after
    allocation pairs equal;
13. cumulative PX0--PX7+LR-C conformance; and
14. exact independently reconstructed replay.

The six global clauses and definitive threshold remain exactly `16/16` rows,
`224/224` row clauses, `6/6` globals, and `230/230` total clauses.

## New authority-v2 package and firewall

A later implementation may add a new evaluator-only package
`arms/px8-lrc-closure-authority-v2` with exactly two dependencies: active PX8
and retained PX7. It must use only `--authority-v2`, emit one distinct
`PX8_LRC_CLOSURE_AUTHORITY_V2_EVIDENCE_SPENT` marker, and create only:

```text
results/px8_lrc_closure_authority_v2.csv
results/px8_lrc_closure_authority_v2.md
```

It must not accept authority-v1 or diagnostic modes, emit their markers, or
write their paths. All seven before/after byte pairs must be serialized per
root.

Authority v2 is not eligible to execute until its evaluator, Cargo surface,
complete active/evaluator coverage audit, static firewall audit, targeted E2B
validation, and implementation audit are committed and tagged in a later
workflow. This protocol commit alone grants no execution permission.

## PX-C and successor stop

Only a future positive, frozen, one-shot v2 result may authorize manifest v6
and the serial comparator against immutable PX7 manifest v5. Gates remain:

```text
primary seams                         < 110 (target 0)
semantic guard                       <= 36
evaluator guard                      <= 136
PX0--PX7+LR-C aggregate foundation      0
new seam kinds                          0
new guarded surfaces                    0
unclassified active files               0
```

This preregistration does not run authority v2, create manifest v6, execute
PX-C taxonomy/comparison, promote PX8, or claim final PX-C
continuous-organism authority.
