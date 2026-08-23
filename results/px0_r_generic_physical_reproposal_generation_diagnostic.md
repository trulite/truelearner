# PX0-R fresh-generation path diagnostic

Status: **development diagnostic; no authority evidence**.

The frozen PROBE v2 formed four fresh arrows per encounter, but every fresh
arrow remained at resistance `1`, deallocated at the next pressure step, and
was proposed again. Evaluator-side trace inspection showed that no fresh arrow
delivered its first spike.

Static path audit located the first missing physical edge:

```text
fresh arrow generation > 1
        ↓
executor compared arrow generation to source-cell generation == 1
        ↓
fresh arrow rejected before propagation
```

The executor had used one scalar for two different physical facts:

- the generation of the source CELL to which an ARROW is attached;
- the ARROW's own generation, captured by queued spikes for stale-path
  invalidation.

The mechanically forced repair separates `source_generation` from the arrow's
own `generation`. Existing fixed arrows retain `1/1`. Fresh proposals retain
the current source-cell generation while receiving a new arrow generation.
Queued spikes continue to validate the arrow's own generation. No semantic
state, route identity, historical endpoint lookup, or evaluator mutation is
introduced.

The diagnostic was run against temporary `/tmp` result paths after both v1 and
v2 negatives had been frozen. It did not rerun or alter either frozen result.
