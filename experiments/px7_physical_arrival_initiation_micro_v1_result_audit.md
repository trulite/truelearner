# PX7 anonymous physical arrival initiation MICRO v1 result audit

Status: **FROZEN POSITIVE DEVELOPMENT RESULT; GATE ELIGIBLE; NO PX7 AUTHORITY**.

## Outcome

The preregistered MICRO passed `8/8` fresh rows. The PROBE edge survived
mirroring, reversed allocation, reversed same-tick insertion, background load,
a fresh nearby arrival locus, late return, and a declared post-training gap.
No new mechanism and no PX0--PX2 change was required.

The original PROBE was not rerun. Its marked execution block remains exactly
`41d75cbd90687eaee43b8f6aa5e27d157781eb6a6b71bbe7b5a1aa248e23f57a`.

## Frozen lineage and hashes

- authoritative PX2 parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- positive PROBE commit: `1ff0eb11229bc20ab43bc649f1f377a9417f98ac`;
- MICRO protocol commit: `65bf47280643ad7ba436946bc028c30e8a36b576`;
- MICRO implementation commit: `bab5c45f0a701e781c6588384197aef9dfdd638b`;
- MICRO protocol SHA-256:
  `1f18bb049bd08e8268af2d61358f20c771babeb7ded594d812cc77e05c077d96`;
- MICRO implementation SHA-256:
  `a503ca4fb85b7c7adf667db3f6294b19a2b23b0744620360ee5e523875b69e45`;
- marked MICRO execution block SHA-256:
  `6c010caac4a87c8d5a7f852a5204bce197c9be52c0d121d00c08ea768db5ef9e`;
- MICRO CSV SHA-256:
  `690be8f361b25dcf4d2f43b167589cdcf2a7ef2263d838cb96c3b6dd78a50dc3`;
- MICRO report SHA-256:
  `215736054211a4c283e6d0ba4ddc14887e05fb39eb875ae8aed612aa45ad9a3d`.

The exact command emitted one
`PX7_PHYSICAL_ARRIVAL_INITIATION_MICRO_EVIDENCE_SPENT` marker. MICRO is
immutable and must not be rerun, regenerated, tuned, or rescued.

## Hardened observations

All five ordinary learned-layout rows executed exactly once on held-out
arrival. M5 was quiet for a fresh nearby locus and then executed exactly once
when identical external activity later reached the recurrent learned locus.
M6's offset-6 return arrived after the frozen local window; coupling remained
`1` and held-out execution stayed quiet. M7 retained coupling `2` and executed
after the preregistered tick-70 gap.

All background firing counts were zero, all queues drained naturally, and all
duplicates were exact. Work ranged from `171` units in the late-return arm to
`972` under the combined 12-cell load. Persistent storage ranged from
`256 -> 384` bytes in ordinary learned worlds to `832 -> 960` bytes under load.
The late-return negative retained `896` bytes because dead weak proposal
generations remain physically accounted. This is frozen negative accounting,
not hidden cleanup.

## Scientific interpretation

Initiation is locus- and history-dependent physical execution, not a supplied
command. A suprathreshold anonymous arrival is necessary but insufficient:
only the arrival locus whose traversed opportunity repeatedly overlapped
ordinary returned activity acquired coupling sufficient to fire the existing
downstream path. Identical activity at a novel locus, activity after late
non-coincident return, subthreshold activity, and absent activity do not
initiate that path.

No `REQUEST`, `START`, respond-now signal, event or task boundary,
evaluator-selected initiating event/path, initiation-role representation,
serializer, adapter, semantic enum, old M schema, or parallel-lane mechanism
executed.

## Next permitted step

A development-only GATE may now be separately preregistered over fresh
namespaces and a compact cross-product of the hardened dimensions. It must use
the frozen MICRO execution block unchanged, include positive learned execution
and the absent/subthreshold/unreturned/late-return/novel-locus controls, require
natural quiescence and exact replay, and remain explicitly non-definitive.

PX7 authority, a definitive matrix, PX3--PX8 advancement, and any change to
PX0--PX2 remain forbidden.
