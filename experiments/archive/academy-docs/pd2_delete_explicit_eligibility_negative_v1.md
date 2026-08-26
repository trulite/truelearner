# PD2 delete explicit eligibility negative v1

Status: immutable development negative.

The deletion itself is mechanically complete, but the cumulative physical
stack does not preserve the preregistered CPC/PQLC behavior across pressure
phase. PD2 therefore stops before the PD1 ten-family successor matrix and
before ARC A2.

## Exact failure

The integrated law currently does this at a pressure epoch:

```text
local participation
    -> absorbs pressure
    -> is spent

later Modulation at the same tick
    -> encounters zero participation
    -> produces zero durable update
    -> cannot trigger local closure
```

This is observable when forward participation occurs immediately before a
pressure boundary and the downstream consequence arrives immediately after
it. It is not a scheduler, layout, batching, or partition bug: Reference and
Production produced identical ordered histories in every serialized row.

### CPC1

The unchanged CPC1 evaluator stops at its first positive-curve assertion. By
the frozen loop ordering, the first failing world is root `1500000`, initial
phase `0`, delay `10`: the pressure event at tick 10 spends the contact's
remaining participation before Modulation reads it. The evaluator publishes no
result artifact after the assertion.

### PQLC0

The unchanged PQLC0 evaluator serialized all `200` physical cases and `400`
mechanics rows:

- Reference/Production exact history agreement: `200/200`;
- same-mechanics exact reconstruction: `800/800` runs;
- failed physical cases: `12/200`;
- failed mechanics rows: `24/400`.

Positive one-hop, two-upstream, and contact-fanout closure fail at phase 9.
The cycle loses closure depth at phases 7-9. Negative controls remain safe and
all worlds remain bounded and quiescent.

### PQLC1

The unchanged depth evaluator serialized all `780` physical cases and `1560`
mechanics rows:

- Reference/Production exact history agreement: `780/780`;
- same-mechanics exact reconstruction: `3120/3120` runs;
- predicate-positive physical cases: `682/780`;
- failed physical cases: `98/780`;
- failed mechanics rows: `196/1560`;
- complete depth worlds fail at phase 9 for depths `1,2,4,8,16`;
- recurrent closure is phase-sensitive at every phase because some closure
  activity crosses a pressure epoch;
- all serialized cases remain bounded and quiescent.

Artifacts:

- `results/pd2_delete_explicit_eligibility_negative_v1/pqlc0_matrix.csv`
- `results/pd2_delete_explicit_eligibility_negative_v1/pqlc0_report.md`
- `results/pd2_delete_explicit_eligibility_negative_v1/pqlc1_matrix.csv`
- `results/pd2_delete_explicit_eligibility_negative_v1/pqlc1_report.md`

## Classification

This is a scientific compositional failure, not a measurement defect and not
evidence that the old four-tick deadline is a good mechanism. The failure shows
that the accepted PD1 pressure exchange and accepted PQLC closure had not yet
been cumulatively tested at the pressure boundary. The deleted deadline used
to mask this boundary in the older stack, while the PD1 feature path had never
earned the full PQLC phase corpus.

Restoring the deadline, adding a grace period, making pressure skip a trace, or
changing PQLC qualification would all be replacement physics. The frozen PD2
protocol forbids such a rescue. A new separately named experiment is required
to ask how pressure and local participation interact when consequence closure
is concurrently arriving.

## Accounting

- D0 deletion/static/checkpoint mechanics: positive;
- D3 core/R1-R6: positive (`14/14`);
- D2 retained local-credit corpus: negative;
- D1 dedicated ten-family successor replay: not run after the D2 stop;
- D4 unchanged ARC A2: not run;
- ARC A3-A5: still paused;
- authority/oracle/`arch.md`: unchanged;
- new substrate law: none.

E2B sandbox: `ixmxf0e4mbxm6zr81z717`.
