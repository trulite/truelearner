# CJ0-D MICRO preflight transition protocol

Status: **PREREGISTERED MECHANICAL TRANSITION; MICRO UNSPENT**.

Positive PROBE v4 is frozen at commit `45bf6aa`, tag
`cj0-d-local-subunit-probe-v4-positive`:

- CSV SHA-256:
  `313168dd2672373260a459ce4e79b4ed17d97451779dea83d6fa94deb2efd626`;
- report SHA-256:
  `822bb52cae83ab30a1d6df0c83f064100834b146c23060264816010a424959f7`.

The current generic preflight incorrectly requires every development-stage
artifact to be absent. The exact authorized transition is:

1. require both frozen PROBE v4 artifacts to exist with the hashes above;
2. require MICRO v1 and GATE v1 result/staging paths to be absent;
3. keep no-argument/wrong-argument refusal and no-CELL entry exact;
4. make no other source, law, fixture, schedule, clause, or path change.

This development protocol authorizes preparation for MICRO only. It creates no
surface beyond PROBE/MICRO/GATE and cannot advance the lane past GATE.

