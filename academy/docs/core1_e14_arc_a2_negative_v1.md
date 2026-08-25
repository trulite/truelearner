# CORE1-B E14 — unchanged frozen ARC A2 negative v1

## Result

The unchanged frozen ARC A2 regimen is negative under CORE1-B.

This run followed the E14 contract already frozen in
`core1_radical_desupply_protocol_v1`: the same five deterministic frames,
action map, `[1,4,2,3]` babbling curriculum, consequence timing, closure, and
acceptance predicate. The E13-B mechanism, CORE1-B profile, Academy boundary,
and organism runtime were not changed.

## Observation

All three executions produced the same complete observation:

```text
actions                 none | none | none | none | none
plasticity updates      0    | 0    | 0    | 0    | 0
modulatory deliveries   0    | 0    | 0    | 0    | 0
physical ticks          3    | 6    | 9    | 12   | 15
natural quiescence      true
```

Reference exact replay was `true`. Reference and Production were exactly
equal, including per-turn work, ticks, observations, and body fingerprints.

## Boundary

E13-D established that an already executable, participating route can carry
local consequence backward through at least 129 Drive links. E14 establishes
a different boundary:

> On the unchanged frozen ARC A2 contexts, CORE1-B does not autonomously
> express an initial motor action, so no changed-world consequence returns and
> the proven composition mechanism never has a participating action route to
> consolidate.

This result does not falsify E13-B composition. It shows that deep credit
composition and autonomous context-to-action route expression are separate
developmental requirements. The first broken link in E14 is before credit:
there is no outward action.

No ARC-specific accommodation, new chooser, preference, reward, timing
change, extra experience, or organism-law repair was made. No authority claim
is advanced.

## Evidence

- Matrix: `experiments/results/core1_e14_arc_a2_v1/matrix.csv`
- Generated report: `experiments/results/core1_e14_arc_a2_v1/report.md`
- Matrix SHA-256:
  `b01cece18a06f5c2581361c88b3838a30057267c15bb1869af9329b6e98b0be8`
- Report SHA-256:
  `0de7b9dd8a0234ac40aefbe07a87049245f8ef697fa59499d286bb1629ade9b6`

The live official ARC toolkit was not used in this gate; E14 is the exact
frozen Academy ARC A2 contract specified by CORE1.
