# TC-DS0 old-window characterization protocol v2

Status: frozen measurement repair before implementation or execution.

V1 remains immutable negative. Its separately executed diagnostic established
that raw live-checkpoint hashes differed only because FullScan eagerly advances
`last_update_tick` on zero-state inactive CELLs while Frontier leaves that
causally inert timestamp untouched.

## Sole repair

Keep the complete v1 protocol, 960 cases, 1,920 mechanics rows, runtime hash,
scenario geometry, schedules, observations, and gates unchanged except:

```text
Reference checkpoint hash == Production checkpoint hash
    no longer required

each checkpoint hash serialized
each checkpoint independently exact-replayable
    still required
```

Cross-mechanics equality still includes:

- emitted eligibility and per-tick path trajectories;
- plastic updates, proposals, and deallocations;
- final live/resistance/coupling/eligibility observations;
- physical-transition hash;
- canonical durable-body hash;
- final clock and pressure phase;
- natural quiescence.

Pending physical activity must also match whenever present. Every v2 row ends
quiescent, so both pending sets must be empty.

No other field or assertion may change. The v1 evaluator and negative remain
frozen. The active organism must retain core SHA-256:

```text
d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
```

V2 executes once for characterization evidence and once in a separate fresh
worker for exact artifact replay. It does not run ARC or select a TC-DS1
candidate.
