# PX1-PT1 attributed-margin stability MICRO v2 result audit

Outcome: **POSITIVE (`12/12`)**. The measurement-only successor executed once
from its frozen implementation. There was no rescue or rerun.

## Evidence

- frozen implementation commit: `dc6f357`;
- implementation source SHA-256:
  `6d9b2b5a20fa7f09aeb07f31fab3031d243c5b59f6e7447308e17b80257a2257`;
- CSV SHA-256:
  `4e315c3f30b62c4cd6168a86e29564647b37d047ae686e0fbd2f0626b4f90025`;
- report SHA-256:
  `0ed6fe52d6a5602ac24d011f48785f478143a90a287e9911c1051e1415028a30`;
- cells: `12/12` pass;
- duplicate-exact cells: `12/12`;
- training/held-out/post-gap quiescence: `12/12` in every phase;
- autonomous source refiring: `0` in every phase and cell;
- evidence marker count: `1`;
- process exit: `0`.

## Physical result

Support A and support B transferred under fresh identities, mirrored placement,
and reversed allocation. During held-out execution both branches fired, but
only the mature outlet fired; the same global return reached both trace areas,
only the effect-bearing trace fired, only that branch received local return,
and only its continuation crossed outward. Mature continuation resistance was
`17`; the nonparticipant was `0`.

No support produced no maturation or execution. Blocking global return allowed
real A branch/outlet participation and direct trace arrival, but produced zero
trace firing, local return, retained continuation, or held-out crossing. An
external anonymous global return without branch/outlet participation reached
both trace cells eight times and produced zero trace firing, credit, maturation,
or execution.

When both continuations genuinely participated, both matured to resistance
`17`, both executed held-out and post-gap, useful downstream recurrence was
preserved, and the system still became naturally quiescent.

The result supports development of the narrow physical chain:

```text
actual continuation participation
→ short-lived local physical coincidence
→ ordinary returned activity
→ differential continuation persistence
→ reusable role behavior
```

MICRO v1 remains an immutable accounting negative. PX0 remains authoritative;
PX1 remains non-authoritative. A fresh six-world GATE is eligible. Definitive
execution is not authorized.
