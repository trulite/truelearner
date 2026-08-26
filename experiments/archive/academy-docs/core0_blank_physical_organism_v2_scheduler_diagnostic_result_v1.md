# CORE0 v2 scheduler diagnostic result

The diagnostic localized the v2 stop to:

```text
profile   CORE-B
gate      E11 stable recurrence
mechanics Production
```

CORE-B reached E11 only after E0--E10 had completed under Reference, exact
Reference replay and Production. E11 Reference and its exact replay completed;
Production stopped while scheduling a causally produced same-tick wave.

Root cause is mechanical. `PendingSchedule::drain_minimum_wave` popped one event
beyond the selected `(tick, phase, causal_wave)` prefix to discover that the
prefix had ended. On a timing wheel, that speculative pop advanced `head_tick`
to the future event. The code then pushed the future event back. While the
current wave was being evaluated, a lawful zero-delay consequence attempted to
schedule the next causal wave at the current tick, now earlier than the
speculatively advanced head.

The Vec reference scheduler has no head and therefore hid the defect. Numeric
identity, organism physics, CORE0 material state and evaluator timing were not
the cause.

The only repair is for wave draining to inspect the next minimum key without
popping it. It may pop only while the minimum key still has the selected wave
prefix. This changes queue mechanics only.
