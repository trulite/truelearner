# DS-IR0 dependency invalidation and reopening

Status: **PREREGISTERED; DEVELOPMENT OUTCOME UNSPENT**

Exact parent: `d97f5038e6133a0abe4b24ea3b8eb5b4ba7cd4f4` /
`ds2-after-rt0-mechanistic-retry-collapse-handoff`.

## Sole question

> Can a retained RT0 direction be structurally validated against the current
> frozen D3 physical signal, locally invalidated when incompatible, and fall
> back to the existing generic CP0/A1 inference path while preserving the old
> asset for compatible historical return?

## Frozen lifecycle

```text
retained A1 template A
  -> ground on fresh occurrences
  -> compare physical normalized effect with current D3 structural signal

compatible A  -> execute retained path; no reacquisition
incompatible B
  -> invalidate stale temporary route
  -> existing generic CP0/A1 observe path reopens for B
  -> B installs and executes

return A -> preserved historical A asset executes without reacquisition
```

No context label, direction slot, correctness, reward, polarity, or evaluator
preference may enter validation. The mismatch is exact anonymous structural
incompatibility between two already-existing physical structures.

Ambiguous or absent D3 signal must not falsely invalidate the retained asset.
Changed history must follow the new structural signal; handle permutation,
fresh occurrence identities, and allocation/layout changes must preserve the
lifecycle. Generic reopening must use existing A1 proposal/probation/install
functions and no new learner/candidate type.

IR0 is enabling-only and claim-ineligible. M1 remains authoritative; M2 remains
absent. After readiness, retry unchanged DS2. If all stages pass, freeze the
complete development ancestor without further optimization.
