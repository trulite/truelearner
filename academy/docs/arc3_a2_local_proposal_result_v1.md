# ARC3 A2 context-local proposal diagnostic result v1

Status: immutable development negative. No retained physics changed.

The just-in-time proposal geometry solved the birth-time-reservoir problem but
exposed a pressure-boundary collision on the third context.

## Observed sequence

| Turn | Context | Action | Update | Candidate | Clock |
|---:|---:|---:|---:|---|---:|
| 0 | 699 | 1 | 0 | R1 / C1 / live | 4 |
| 1 | 290 | 4 | 1 | R1 / C1 / live | 8 |
| 2 | 552 | none | 1 | R0 / C1 / dead | 12 |

The first context arrived with no candidate ARROW. Existing local variation
created the source-to-motor proposal, it traversed, and babbling completed the
motor threshold. The second novel context did the same. This reduced the first
turn from 536,235 to 196 units of physical work and proved that unseen contexts
need not carry pre-created weak ARROWs.

The third context exposed this exact ordering:

```text
previous quiescent clock       tick 8
Academy forced input gap       -> context source fires at tick 9
local proposal distance 1      -> first traversal due at tick 10
ordinary pressure epoch        -> proposal R1 -> R0 at tick 10
scheduled traversal resolves   -> stale/dead, no motor unit
babbling alone                 -> below threshold, no action
```

Thus the proposal was created at the correct local site but died before its
first possible traversal. Exact replay reproduced the same transition.

## Classification

This diagnostic is negative as frozen. It does not justify altering proposal
strength, pressure, distance, or any retained TrueLearner law.

It does reveal that Academy's inherited `current_tick + 1` input admission is
not causally neutral for just-in-time variation. That extra idle tick was
fixture scheduling, not a world observation or substrate requirement. A fresh
diagnostic may remove only that forced gap and admit the next official raster
at the already-quiescent current tick. Pressure-phase controls are required so
the successor cannot hide a general weakness through one favorable schedule.

## Evidence

- E2B sandbox: `i0sx3yhxt5wtfvhxb0j0b`;
- suite SHA-256: `8582462ef67c54e4f5d40f49f25298611c7a015979bcc04cc67247e84e685e3d`;
- report SHA-256: `9dc9d3fb01ea16d2c971b977c42eff8a18b4065f1f3f12b3a8ad92f6c527487e`;
- primary video SHA-256: `887dd68925762b04624c031f668172f555f7edb1275c24850b91b3a874e23324`;
- exact replay: true;
- A2: fail; A3-A5: skipped;
- files under `truelearner/` changed: zero.

Review video:

`results/arc3_a2_local_proposal_v1/gallery/episodes/arc3-a2-four-actions/episode.mp4`
