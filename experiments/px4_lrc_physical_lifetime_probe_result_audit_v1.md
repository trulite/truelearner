# PX4 LR-C physical lifetime PROBE result audit v1

Status: **DEVELOPMENT PROBE POSITIVE; MICRO ELIGIBLE; AUTHORITY ABSENT**.

The preregistered PROBE executed once from clean audit commit
`5a5f4260a92cf6c9eb7c2a623228408acc508865` in fresh E2B sandbox
`ia85yqpfd6f99a1jariv9`, using the unique state file
`px4-lrc-lifetime-probe-20260824.json`. The sandbox was left running.

## Frozen artifacts

| artifact | SHA-256 |
|---|---|
| PROBE CSV | `077ccac316d40d50b69e0ceb5d6c3ba48712c239bfa92909646b8a7f4554d35c` |
| PROBE report | `95d77a7d4654f5af81e55f13966b69005f9a9c7042c3467ec2d4c9df817dbe7e` |

The CSV has one data row and exactly 38 fields matching its 38-field header.

## Result

Identity `151001` passed every conjunctive observation and an exact fresh
replay:

```text
one unsupported exposure deallocated                 true
one qualified exposure resistance/coupling          4 / 2
recurrence resistances                         4 / 7 / 12 / 22
pressure steps to deallocation                  4 / 7 / 12 / 22
reuse without proposal, outward impulse              true / 2
continued disuse deallocated                           true
ordinary reacquisition, old/new generations       true / 4 / 4
changed old dead / recurrent new live / resistance true / true / 6
stale crossing blocked, effect fires, deallocations true / 1 / 1
return-alone / late-return / Drive-return controls true / true / true
fresh identity/layout invariance                         true
PX0 / PX1 / PX2 / PX3 conformance       true / true / true / true
exact replay / natural quiescence                  true / true
```

The equal displayed old/new generation values do not alias the arrows. The
old queued spike addresses the first arrow identity at its pre-pressure
generation; pressure advances that same arrow to generation `4` and makes it
dead. Reproposal creates a distinct second arrow identity whose initial
generation is also `4`. The old queued crossing is rejected; only the new
arrow reaches the effect.

No new physical field or update was needed. The first scientific collapse is
`none`. MICRO may execute unchanged in its separately registered fresh
identity matrix. This positive does not create or advance PX4 authority.
