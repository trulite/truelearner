# PC0 cumulative-negative v1 diagnostic result v1

Status: diagnostic negative; PC0 is a scientific stopped negative.

Frozen evaluator: `d3ac1a8649374d38bde37b88040c42d0e547b8bd`.
Fresh E2B sandbox: `ivf6u4mteqlbhw1ptzkd2`.

## Result

The diagnostic removed only the first spent predicate requiring every
magnitude contact's pre-pressure participation to be below `Q`. It then stopped
at the next unchanged PD1 magnitude assertion:

```text
before_pressure:
    low participation < medium participation < high participation

after_pressure:
    low pressure_load > medium pressure_load > high pressure_load

failure:
    medium pressure_load > high pressure_load was false
```

No diagnostic artifacts were published because the retained assertion failed
before serialization.

## Classification

This is a scientific counterexample to PC0's single candidate law, not another
deadline-column measurement mismatch.

The focused P0 construction deliberately placed all nonzero traces below one
pressure quantum and therefore established a graded sub-`Q` response. In the
unchanged PD1 reuse geometry, non-consuming participation can accumulate beyond
that range. The candidate law clamps attenuation with:

```text
attenuation = min(participation, Q)
```

Once distinct participation magnitudes reach the saturating region, pressure
can no longer preserve their required graded ordering. Thus the law avoids a
Boolean `participation > 0` shield at low magnitude but creates a fully shielded
class at high maintained participation.

## Immutable accounting

- focused PC0 v1: `120/120`, retained as narrow positive evidence;
- cumulative PC0 v1: immutable negative;
- cumulative-negative diagnostic: immutable scientific negative;
- PC0 development readiness: not established;
- active PC0 candidate: not accepted as successor physics;
- PD2, ARC A2, authority, oracle, and `arch.md`: unchanged.

Any next attempt must preregister a different continuous pressure/participation
interaction. It may not repair this gate by widening `Q`, clipping the trace,
restoring eligibility, consuming participation, or adding a Boolean shield.
