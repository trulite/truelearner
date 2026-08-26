# CORE0 blank physical organism protocol v2

V2 inherits the complete frozen v1 protocol and candidate at `a8b1afc`.

It changes only the E2 evaluator schedule:

```text
v1 (invalid)
first generated interaction
second interaction at absolute tick 1
consequence at absolute tick 3

v2
first generated interaction
second interaction at current physical tick
consequence at current physical tick after second propagation
```

The correction cannot change elapsed idle time, the physical ordering of the
three experiences, any organism state transition, a profile, a predicate or a
comparator. It merely stops the evaluator from scheduling an arrival in the
organism's past.

V2 emits an evidence-spent marker before constructing the matrix and writes
the matrix only after all profiles have completed or stopped by the frozen
prefix rule. It executes once.
