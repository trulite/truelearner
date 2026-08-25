# CK0 junction checkpoint integration protocol v2

Status: frozen before any v2 evaluator change or execution.

Parent: CK0 continuation diagnostic result
`21a89a2a1b8c66070524acdf978d35e25d6340ff`
(`ck0-continuation-negative-diagnostic-result-v1`).

## Question

Does the unchanged CK0 checkpoint candidate pass its complete frozen matrix
when the evaluator observes restored execution and compares only the
preregistered physical checkpoint contract?

## Frozen runtime and science

The canonical runtime remains byte-identical at SHA-256
`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

CK0 v2 preserves all ten v1 families, roots, topology, inputs, checkpoint
bytes, stale references, mechanics, and direct predicates: 20 cases and 40
rows. It changes no checkpoint representation or organism law.

## Sole evaluator correction

1. After restoring a checkpoint, explicitly enable the causally inert
   physical-trace observer before trace comparison.
2. Keep raw checkpoint hashes in evidence, but exclude them from
   Reference/Production physical equality.
3. Compare the public PhysicalWork components and `physical_total()`; retain
   legacy `Work::total()` only as a diagnostic column.
4. Rename the output directory, report, assertion, and sentinel to CK0 v2.

No other evaluator change is permitted.

## Execution and stop rule

After targeted formatting/check/strict Clippy, the complete v2 matrix executes
exactly once in a fresh E2B worker. Any failure freezes v2 negative without
repair or rerun.

Only a complete positive permits unchanged J0 and CV0/J0+SV1 lineage replays,
followed by one unchanged consolidated RS2 rerun.

```text
CK0 v2 -> J0 -> CV0/J0+SV1 -> consolidated RS2
        -> CE1 -> FD2 v2 -> unchanged frozen ARC A2
```

Authority, oracle status, `arch.md`, and the Academy curriculum remain
unchanged here.
