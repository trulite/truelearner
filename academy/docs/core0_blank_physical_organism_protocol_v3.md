# CORE0 blank physical organism protocol v3

V3 inherits the complete v2 CORE0 candidate, worlds, profiles, predicates,
comparators and prefix discipline.

Its sole runtime change is the scheduler repair established by the frozen v2
diagnostic:

```text
before
pop next event to discover whether wave prefix ended
push it back if it belongs to a later prefix

after
peek next minimum causal key
pop only if it belongs to the current prefix
```

The repair cannot change a causal key, arrival, wave, CELL/ARROW/SPIKE state,
learning law, proposal, work definition or physical trace. It only prevents a
timing-wheel head from advancing speculatively.

Before the sole v3 matrix, the unchanged SI0/WS0 scheduling tests and targeted
CORE0 compilation must pass. V3 then executes once and publishes the 60-row
prefix matrix without rescue.
