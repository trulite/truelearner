# ARC3 A2 continuous-admission phase diagnostic result v1

Status: immutable development negative. No retained physics changed. The
preregistered phase-9 discriminator was not run because the primary phase-0
row failed first.

Admitting each new spatial raster at the already-quiescent physical tick
removed Academy's artificial idle tick, but it did not make newborn local
opportunities robust to the substrate's global pressure phase.

## Observed primary row

| Turn | Context | Action | Update | Candidate | Clock |
|---:|---:|---:|---:|---|---:|
| 0 | 699 | 1 | 0 | R1 / C1 / live | 3 |
| 1 | 290 | 4 | 1 | R1 / C1 / live | 6 |
| 2 | 552 | 2 | 1 | R1 / C1 / live | 9 |
| 3 | 524 | none | 1 | R0 / C1 / dead | 12 |

The first three unseen contexts each arrived without a pre-created candidate.
Their external source activity proposed an ordinary distance-1 source-to-motor
ARROW, and the proposal's first traversal coincided with the babbling unit.
Each expected outward action occurred.

The fourth context began when the retained body was quiescent at tick 9:

```text
new official raster admitted     tick 9
context source fires             tick 9
distance-1 proposal due          tick 10
ordinary global pressure         tick 10
new R1 proposal                  -> R0 / dead
scheduled traversal              -> stale
babbling unit alone              -> below motor threshold
outward action                   -> none
```

Thus the fixture gap was not the underlying problem. A newly proposed physical
opportunity can be created immediately before a global pressure epoch and die
before its earliest lawful traversal. The failure moved from the third context
to the fourth; it did not disappear.

## Classification

This is a substrate question, not an ARC adapter or replay defect. The current
pressure rule applies an absolute global epoch to a proposal that has not yet
had one causal chance to participate. Any successor must be tested as a small,
general physical law rather than repaired through ARC scheduling, proposal
strength, or curriculum timing.

The most direct next discriminator is opportunity-age-aware pressure: determine
whether a newborn resistance-1 proposal should become pressure-eligible only
after its first possible traversal window has elapsed. That hypothesis is not
implemented here.

## Evidence

- E2B sandbox: `i0sx3yhxt5wtfvhxb0j0b`;
- exact replay: true;
- primary A2: fail at the fourth context;
- phase-9 discriminator: not run, as preregistered after primary failure;
- A3-A5: not executed;
- suite SHA-256: `3d5e92b9c044cfcc2fab93a377d031f35d75e61a0a8b5bdc50e78e6906305856`;
- report SHA-256: `75991bb72403feb93e7f67cb09c1f71bb28e71444d0fe55888c2a8e0b59d83c3`;
- primary video SHA-256: `fd50cb78b899660e2fe00e9c1679177857d1d2ca7b5163bbe1e7003b9461a804`;
- files under `truelearner/` changed: zero.

Review video:

`results/arc3_a2_continuous_admission_v1/phase0/gallery/episodes/arc3-a2-four-actions/episode.mp4`
