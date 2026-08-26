# WS0 complete causal-wave semantics protocol v2

Status: frozen before the v2 evaluator-feature correction.

Parent: WS0 v1 technical negative `7e84488`.

## Sole correction

Remove the evaluator's unnecessary `cv0j0` feature. Compile the byte-identical
WS0 runtime and evaluator against exactly the retained laws exercised by the
matrix:

```text
ce0 + rs0/pqlc0 + si0
```

Delete only now-unavailable observer match arms for CV0/J0-only CELL proposal
and CELL deallocation events. WS0 worlds cannot emit those events.

## Everything else frozen

The v1 runtime candidate, complete causal-wave law, 14 families, five
permutations, roots, inputs, topology, predicates, logical normalizer,
PhysicalWork comparison, checkpoint continuation, and stop rule remain
byte-identical.

After targeted format/check/Clippy, v2 executes once in a fresh worker. Any
failure freezes v2 negative. Only a complete positive permits the retained
SI0/PQLC/cumulative replay prefix. RS2 remains stopped throughout WS0.
