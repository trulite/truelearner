# PX1 recurrent role-stability diagnostic v2 result audit

Outcome: **FROZEN DEVELOPMENTAL NEGATIVE — ZERO OF FOUR ARMS PASSED**.

The separately named v2 diagnostic executed once from frozen implementation
commit `21d6676176d19b5855f9fc52628b99303ba39e47`. It emitted one v2 development
marker and exited `0`. No arm physics, schedule, readout, or pass criterion was
changed after execution.

## Matrix

| arm | completed | timed out | role resistance primary | role resistance transfer | held-out effects primary | held-out effects transfer | extra source firings | productive recurrence | quiescent | pass |
|---|:---:|:---:|---:|---:|---:|---:|---:|:---:|:---:|:---:|
| margin | yes | no | `[19,19]` | `[19,19]` | `[1,1]` | `[1,1]` | `0/0` | yes/yes | yes/yes | no |
| inhibition | yes | no | `[19,19]` | `[19,19]` | `[1,1]` | `[1,1]` | `0/0` | yes/yes | yes/yes | no |
| distance | yes | no | `[0,0]` | `[0,0]` | `[0,0]` | `[0,0]` | `0/0` | no/no | yes/yes | no |
| timing | no | yes | unavailable | unavailable | unavailable | unavailable | unavailable | unavailable | no | no |

Margin and inhibition each preserved positive internal source return (`32`) and
learning-site return (`8`) in both primary and transfer worlds. Inhibition also
matured and fired its local brake. Both arms stopped source refiring and
remained productively recurrent, duplicate-exact, and naturally quiescent.
They nevertheless failed because both anonymous endpoint opportunities matured
to resistance `19` and both produced a held-out outward crossing. Quietness was
therefore achieved without differential boundary-role formation.

Distance prevented reciprocal self-excitation, but separated the site far
enough that neither endpoint-local role matured, no role return completed, and
no outward effect executed. Timing did not reach natural quiescence within the
frozen five-second evaluator bound.

## First missing physical relation

The diagnostic rejects the narrow hypothesis that excitation margin,
substrate-native braking, spatial separation, or eligibility timing alone can
preserve all of:

```text
differential role learning
+ useful recurrent execution
+ natural quiescence
```

In the two quiet recurrent arms, ordinary traversal of each authoritative PX0
correspondence repeatedly reached its endpoint. Under the frozen
`apply_local_return` rule, that local activity was sufficient for every
endpoint opportunity to mature, including the arm without role-specific
driver activity. The substrate therefore had no local physical basis for
distinguishing role-specific returned activity from ordinary use of the
correspondence that hosts the role opportunity.

This is not evidence for a semantic role label. It identifies a missing
physical discrimination problem:

> Can returned activity remain locally attributable to the particular
> developing continuation that caused it, rather than indiscriminately
> stabilizing every opportunity at a repeatedly traversed correspondence
> endpoint?

No representation, mechanism, or repair is authorized by this audit.

## Integrity

- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen v2 implementation SHA-256:
  `21480777059d7446c248e388819004807732e3c1a6949480a71c06cf3dbb1587`;
- CSV SHA-256:
  `7ddf75567e4b61fd735a042ddafb949fd85be57021285465a08ca17285c61e80`;
- report SHA-256:
  `c1ec0abd6fbfd5070b11c7f54065e35597afe4ef71f0199dfbd6a21a62497633`;
- completed arms: `3/4`;
- timed-out arms: `1/4`;
- passing arms: `0/4`;
- definitive execution: none;
- PX1 authority: absent.

The v1 operational negative remains immutable and is not reinterpreted by v2.
