# PX0-P1 return-free proposal control result audit

Outcome: **P1-A — EXISTING PHYSICAL PROBATION POSITIVE**.

Status: **DEVELOPMENT DIAGNOSTIC; PX0 DEFINITIVE v1 REMAINS NEGATIVE; PX0 AUTHORITY ABSENT**.

## Discriminating result

The unchanged PX0 law passed both fresh topology-isolated arms:

| arm | opportunities | layout | fresh A | A gate fires | A returns | A max impulse | A max resistance | A crossing | A dies | B max impulse | B crossing |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| short | 2 | direct | yes | 0 | 0 | 1 | 1 | 0 | yes | 2 | 1 |
| threshold | 3 | mirror | yes | 0 | 0 | 1 | 1 | 0 | yes | 2 | 1 |

Both complete-state duplicates replayed exactly and all executions became
naturally quiescent.

The route stride was 8, with source/probe/contender/gate offsets `0/1/2/4`.
Every nonmatching source was therefore at least distance 4 from A's return
gate, outside the unchanged generic proposal radius 2. Fresh A proposals still
formed normally and carried impulse-1 activity into their local candidate
neighborhood.

With zero A-gate firing and zero delayed A-source return, A never exceeded
coupling/impulse 1 or resistance 1, never fired its threshold-2 contender,
never crossed outward, and disappeared under ordinary pressure. Returned B
reached impulse 2 and executed exactly once.

## Narrow claim

> The existing substrate already provides physical proposal probation: fresh
> weak structure can participate in local evidence dynamics, but genuinely
> return-free structure remains subthreshold, cannot produce outward
> correspondence, and eventually deallocates. Ordinary returned physical
> activity matures supported structure into execution.

No new probation mechanism, state variable, maturity flag, threshold rule, or
execution gate is required by this result.

## Relationship to PX0 definitive v1

PX0 definitive v1 remains an immutable authority failure. This result neither
reruns nor rescues it. It explains the failed cells jointly with the frozen
PX0-P coupling diagnostic:

```text
dense stride-6 topology
→ incidental cross-route arrow into A gate
→ real A return
→ A matures and executes

isolated stride-8 topology
→ no cross-route return
→ A remains weak and silent
```

The remaining scientific limitation is return specificity under dense local
topology: stable correspondence evidence must somehow be distinguished from
incidental return paths without semantic labels. That is not a probation
problem and requires separate preregistration.

## Frozen artifacts

- [summary CSV](../results/px0_p1_return_free_proposal_control_probe_v1.csv),
  SHA-256
  `c35572d355b8428a5f9b1afa405596864b2b6e0f16b70a6271f1410e38ea0876`;
- [physical trace](../results/px0_p1_return_free_proposal_control_probe_v1.trace.csv),
  SHA-256
  `db074402e43b1b396a9b99c62691e5c3afc0dc651bb68bba4fd5416eb0b6f118`;
- [result report](../results/px0_p1_return_free_proposal_control_probe_v1.md),
  SHA-256
  `370eca5f367c83a3b2c703d13b6aad4b9b4183b41c77bbd158dcde6671de9068`.

The active law remained byte-identical at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
