# PX3-D1-R2 closed-loop return attribution result audit v1

Status: **EXECUTION COLLAPSE; EVIDENCE SPENT ONCE; NO R2 VERDICT**.

## Frozen execution

- implementation commit: `841cf585adb4cce3ff61e941e02efe74a44b4e3c`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole registered `--r2` execution emitted
  `PX3_D1_R2_CLOSED_LOOP_RETURN_ATTRIBUTION_EVIDENCE` and was killed with
  exit status `137`;
- neither final artifact nor hidden staging artifact was present after the
  process ended;
- no second evidence execution was made.

This is an execution collapse, not a positive or negative answer to the
registered attribution question.

## Mechanical cause

The diagnostic accidentally closes an autonomous amplifying cycle:

```text
P(threshold 1)
  -> weak candidate, delay 2, initial coupling 1
  -> effect(threshold 2)
  -> relay(threshold 1)
  -> P, delay 1
```

During a real AB episode, context supplies the effect's other unit input. The
effect fires and its relay returns to P while the candidate is eligible. In
`PlasticSubstrate::propagate`, every arrival at P first calls
`apply_local_return`; this changes the candidate from resistance/coupling
`1/1` to `4/2`. The same arrival then fires threshold-one P. P therefore sends
the newly amplified coupling-two impulse to the threshold-two effect. The
effect can now fire without context and returns through the relay again:

```text
tick 3: P fires; candidate becomes eligible
tick 5: candidate(1) + context(1) fire effect
tick 6: relay arrives at P; candidate becomes resistance 4/coupling 2;
        P fires again
tick 8: candidate(2) alone fires effect
tick 9: relay arrives at P; P fires again
... repeats every three ticks
```

Pressure cannot terminate the cycle: each return adds three resistance while
ordinary pressure removes resistance more slowly, and coupling remains two.
`propagate` has no event or time bound and appends every delivery/crossing to
its `trace` and `crossings` vectors. The physical oscillator therefore makes
those vectors grow without bound until the E2B worker kills the process for
memory exhaustion.

This is not evidence of a leaking Rust allocation. It is a non-quiescent
substrate circuit combined with intentionally unbounded execution logging.

## Scope

- D1 core remains unchanged.
- D1-R2 is unresolved.
- No result CSV/Markdown exists to interpret.
- D2, candidate formation, MICRO, GATE and authority are unaffected by this
  execution collapse.

Any successor must be preregistered as a new diagnostic. It must preserve a
completed physical return path without allowing the crediting return itself to
re-execute the just-amplified candidate indefinitely.
