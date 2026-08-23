# PX0-P coupling diagnostic v2 audit

Status: **VALID DIAGNOSTIC; CROSS-ROUTE PHYSICAL RETURN IDENTIFIED; MECHANISM CHANGE STOPPED**.

The corrected fresh-namespace trace reproduces both proposal-before-evidence
crossings, but falsifies the evaluator-level description of fresh A as
physically unsupported.

## Direct two-opportunity arm

Namespace `0x9200000`, direct layout, stride 6:

```text
A source position       0
A gate position         4
B source position       6

generic radius          2
B source → A gate       physically eligible (distance 2)
```

At renewal tick 300, B source `153092107` fired. Its generic local proposal
reached A gate `153092136` at tick 302 while A probe activity reached the same
gate. Two ordinary impulses made A gate fire. At tick 303, A source
`153092106` received returned physical activity. `apply_local_return` therefore
lawfully strengthened A's currently eligible fresh outgoing arrows. On the
next renewal, A probe and A contender received impulse 2 rather than impulse
1. Later A activity crossed outward.

## Mirrored three-opportunity arm

Namespace `0x9240000`, mirrored layout, stride 6:

```text
spare source position   12
A gate position         10

generic radius          2
spare source → A gate   physically eligible (distance 2)
```

At renewal tick 300, spare source `153354250` fired. The fresh cross-route
proposal and A probe activity jointly fired A gate `153354281` at tick 302.
A source `153354251` received return at tick 303, again strengthening fresh A
through the ordinary local-return law. A was physically supported even though
the evaluator activated only B's explicit support device.

## Consequence

The current trace does not establish that an unsupported proposal becomes
executable before evidence. It establishes:

> Broad generic local proposal can create incidental return topology between
> neighboring candidate fields, and the existing learner correctly treats
> that returned physical activity as evidence.

The PX0 definitive v1 result remains permanently negative because its
preregistered P7 silence control failed. This diagnostic does not rescue or
reinterpret that matrix.

However, adding a probation mechanism now would smuggle in an evaluator-level
distinction between intended B support and ordinary cross-route physical
return. From the organism's substrate-native view, both are real return.

PX0-P therefore stops before changing the active law. The next scientific
choice is between separately preregistered discriminations:

1. **Topology-isolated probation control:** hold candidate abundance and
   renewal history fixed while placing every nonmatching source outside the
   return-gate proposal radius. This tests whether truly return-free fresh A
   ever executes under the existing law.
2. **Return-specificity program:** retain dense overlapping topology and ask
   whether local physical dynamics can distinguish which return was caused by
   which candidate without a semantic path identity. This is a different and
   deeper physical-credit question.

The active PX0 law remained exact at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
No new substrate variable, maturity class, threshold law, or execution gate
was introduced.
