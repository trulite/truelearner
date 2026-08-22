# DS-D0 stage-8b discrimination audit

## Source inventory

```text
frozen C0 run calls                         1
parallel matrix calls                       1
diagnostic apply_consequence edges          1
marked DS1 apply_consequence definitions    1
single-property arm variants                5
combination variants                        0
```

All parent retry, collapse-handoff, C0, E0, marked-DS1, M0, and results-tree
fingerprints match their frozen constants.

## Arm isolation

`CandidateProperty` is a Rust enum: one cell can contain exactly one of
ownership-only, temporal contrast, alternative comparison, polarity, or
outcome change. There is no multi-property variant. All property values are
episode-local and add zero persistent bytes.

The temporal arm computes only that its local times are ordered, then
deliberately exposes no evaluative boolean. The alternative-comparison arm
compares fixed pre-choice magnitudes at the frozen selected index. The polarity
arm forwards one bit. The outcome-change arm compares its before/after pair.

## Frozen learner access

The diagnostic accessor is macro-expanded outside the marked DS1 slice but
inside a byte-identical E0 composition copy so it can call the existing private
`choose` and `apply_consequence` methods. It reproduces the same deterministic
E0 support/target construction used by R0/C0. Choice equality with the frozen
C0 report is required in every cell.

Reachability requires both the candidate-to-bool mapping and an observed
increment of exactly one in frozen `credit_updates`. Thus a type-compatible
payload without physical update execution would fail the cell.

## Scope

The 320 learner bytes reported per cell are the diagnostic fixture's ordinary
one-pattern frozen DS1 allocation, not a retained candidate-property asset.
No acquisition, consolidation, held-out reconstruction, or cumulative
composition is performed. The matrix nominates properties for subsequent
diagnostics; it does not establish a learned prerequisite.
