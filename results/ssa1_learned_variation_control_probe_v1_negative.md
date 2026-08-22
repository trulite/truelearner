# SSA1 learned variation-control PROBE v1 negative

Status: **frozen development negative; no claim-eligible evidence**.

The first SSA1 implementation compiled the byte-frozen M5/M6 learner with the
frozen clean substrate, then stopped at the first physical composition edge:

```text
old M6 diagnostic RawConsequence variant
  topology root/arrows
  + independently indexed tick array
        |
        v
attempted single causal CELL/ARROW/SPIKE realization
        |
        X
not every historical topology/tick pairing is one causal trace
```

The test stopped at `physical_consequence_exact(raw)` before an SSA1 world
result. Nothing was tuned, rerun as evidence, or interpreted as an SSA1
classification.

Mechanistic audit:

- the frozen M6 normalizer uses the minimum tick and the topology as separate
  physical descriptors;
- some historical diagnostic variants assign an indexed tick order that does
  not follow the selected root and arrows;
- a single propagation trace therefore cannot reproduce all of those paired
  descriptors literally;
- the M6 learning mechanism itself does not require those particular
  diagnostic variants.

The smallest lawful retry keeps one real executable causal chain and varies
only its physical inter-arrival delays. Equal activation magnitude, delayed
arrival, M6 normalization, recurrence, contrast, M5 eligibility, and every
frozen threshold remain unchanged. This is a missing physical linker repair,
not a new learner or substrate change.
