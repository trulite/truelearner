# PX8 LR-C cumulative closure authority v3 result audit v1

Status: **DEFINITIVE POSITIVE; PX8 HANDOFF PERMITTED; FINAL PX-C NOT RUN**.

## Frozen lineage

- exact PX7 serial authority parent:
  `9e2aca8933df168780dd6d7e6f00c3d9feae98ee` /
  `px7-lrc-arrival-authority-v1`;
- frozen v2 diagnostic result parent:
  `15c40708220587a12ea872291e2aca3f934ff794` /
  `px8-lrc-closure-authority-v2-negative-diagnostic-result-v1`;
- v3 protocol:
  `029cb5c1791dfe72ee709bb787c3c86aefc535c6` /
  `px8-lrc-closure-authority-v3-protocol-v1`;
- evaluator implementation:
  `14d6a0e77c8fa5bd54c6cc519c6896e0216b0c91` /
  `px8-lrc-closure-authority-v3-evaluator-frozen-v1`;
- complete pre-evidence freeze:
  `1b8a4a6a422cb48a8c8e401e6d60a4f77c4ed4d9` /
  `px8-lrc-closure-authority-v3-frozen-v1`;
- immutable positive evidence:
  `99566b0ac5435fa21cfb65bbf8b8653e4fbc76f7` /
  `px8-lrc-closure-authority-v3-evidence-positive-v1`;
- manifest v6 commit: `17ab59f753175e4095335eacaeb2a6bc4ac12e97`;
- frozen PX-C evidence commit: `d70e564fa6cab043a69cb0adb3e463103a5cdacf`.

Active PX8 remained byte-identical at
`8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f`.
The retained LR-C, PX4, and PX7 hashes remained respectively
`7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`,
`a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71`,
and `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e`.

## Definitive functional result

Fresh E2B sandbox `inygidok8n3takk7qviz2` executed the registered release
matrix exactly once from commit
`1b8a4a6a422cb48a8c8e401e6d60a4f77c4ed4d9`. It emitted exactly one
`PX8_LRC_CLOSURE_AUTHORITY_V3_EVIDENCE_SPENT` marker, returned exit `0`, and
reported `16/16` rows and `230/230` clauses.

Every root `865001..865016`, layout/reflection quadrant, and twist
`0,137,274,411` passed:

1. formation produced retained updates and inward modulation;
2. a completed learned route crossed outward exactly once;
3. downstream physical return and local update occurred exactly once;
4. incomplete, blocked, and stale routes did not cross;
5. open and aged compact routes did not cross;
6. zero-length topology crossed exactly once;
7. duplicate physical and recursive inputs each crossed exactly once;
8. branch and cycle topologies did not cross;
9. pause stability and held-out resume equality passed;
10. every world exhausted its queue and became naturally quiescent;
11. maximum work was `14788 / 20000`;
12. mature memory was stable and the stale/reproposal safety conjunction passed;
13. cumulative PX0--PX7+LR-C conformance passed; and
14. independent complete-trial replay was exact.

All sixteen stale rows were identical:

```text
memory_before=5488
memory_after=5552
delta=64
capacity=8192
outward_crossings=0
stale_route_executions=0
fresh_proposals=1
queue_empty=true
quiescent=true
replay_exact=true
```

The six mature pairs were exact on every root: primary, uninterrupted,
incomplete, duplicate, and blocked were `5488==5488`; cumulative PX7 was
`2304==2304`. Overall maximum retained bytes were `5552 / 8192`.

Evidence hashes:

```text
CSV     3ed00e8d71392b5ac39f38ce4804cc71337ea86702283fdbe13425ea3240b1fa
report  f8357997574e875872c42cab361073f08b6cb39b638b587553226bf8e940ed26
```

Portable result audit sandbox `ihcriemg7ptp3g4r5n3mu` verified all rows,
clauses, bounds, mature equalities, stale conjunction fields, and absence of
staging residue. The initial frozen result branch used reserved AWK variable
`index` and failed before evaluating rows in sandbox
`i6a6ptp2bt1nbjo96shvq`; it ran no Rust or evidence. The frozen pre-evidence
audit was preserved unchanged, and the disjoint portable result audit hash is
`b3fae77462c226e2817b42420c76bc4bc266fe2eb61b80b6c350cf15b395539f`.

## Manifest and PX-C readiness result

Manifest v6 SHA-256 is
`5205d1b115e476f1ec0efea603a04425b5c9bff92a4398ea46ef89607b134f49`.
Relative to immutable v5
`db4758baa5aeba36a87251f7d2ccb85cd2215f9489a1189eae4fd9d6408001c2`,
only the PX8 predecessor row changed, to
`arms/px8-lrc-physical-closure/src/lib.rs` as
`authoritative-physical-closure-emission`.

Fresh E2B sandbox `isge3rs95jzti7cwmzfm6` ran taxonomy and comparator exactly
once. The comparator passed:

| metric | PX7 baseline | PX8 after | delta | accepted |
|---|---:|---:|---:|:---:|
| primary seams | 110 | 0 | -110 | true |
| semantic guards | 36 | 0 | -36 | true |
| evaluator guards | 136 | 0 | -136 | true |
| new seam kinds | 0 | 0 | 0 | true |
| new guarded surfaces | 0 | 0 | 0 | true |

Every after layer—PX0--PX3+LR-C, PX4, PX5, PX6, PX7, and PX8—is `0`.
Exact kind deltas are:

```text
typed_representation           -41
explicit_mechanism_invocation  -21
episode_reset_boundary          -1
seed_history_synthesis           0
semantic_condition               0
manual_temporary_cleanup        -1
typed_handoff                  -43
evaluator_derived_input         -3
```

The inline supplemental threshold helper in that sandbox had a quoting defect
after taxonomy/comparator had already passed. It did not alter their artifacts
or rerun them. Frozen portable PX-C result audit
`4b47a4812570f55433a361ef8e7bfbd72c40e35da63234d988d3842de6986a25`
then passed in sandbox `igq6m8umntoa4uip7u4wb`.

## Decision

The v3 measurement repair is validated. There is no physical counterexample,
new substrate law, scientific fork, resource failure, replay failure,
quiescence failure, active-surface gap, or PX-C readiness blocker.

This establishes the serial PX8 physical-closure/emission handoff only. It
does not run or claim final PX-C continuous-organism authority.
