# CORE1-E21 — Temporary Physical Credit Return Protocol v2

## Status

Implementation-conformance correction after the first P1 process stopped before
returning an observation. Protocol v1 remains binding. The E21 evidence marker
was not emitted and no matrix row ran.

The earlier P2 observation is retained only as staging evidence and must pass
again under this correction before P1 can be interpreted.

## Nonconforming seam

Protocol v1 allows same-admission traversal capture solely to locate the
actually used contact, followed immediately by conversion to physical topology
and capture-bit clearing. The first implementation reused E20's capture bits
without disabling their E20 deallocation-protection effect during that same
admission.

In hard seed 7, the initial junction therefore preserved tentative alternatives
while it was merely trying to observe traversal. Four motor routes crossed
simultaneously and the process stopped with:

```text
ambiguous organism output: 4 motor crossings
```

No temporary credit-return connection existed before this ambiguous crossing.
The event cannot test E21's proposed wire.

## Sole correction

When E21 physical credit return is enabled, traversal capture remains writable
and observable during the admission but has no deallocation-protection effect.
Only the materialized temporary physical return connection may anchor its
target contact and incident participating Drive arrows.

E20 retains its original default: passive USED-PENDING bits protect marked
arrows. No prior evaluator changes behavior unless it explicitly enables E21.

No action selection, variation, completion, consequence, PQLC, connection
formation, connection traversal, or cleanup rule changes.

## Repeated gates

The corrected candidate must rerun the complete exact P2-first gate. Only if it
passes may corrected P1 run. Any subsequent P2 or P1 failure is a stopped E21
negative; no further correction is authorized.
