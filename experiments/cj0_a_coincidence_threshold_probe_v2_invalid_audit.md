# CJ0 Arm A coincidence-threshold CELL PROBE v2 invalid-execution audit

Status: **FROZEN INVALID; NO SCIENTIFIC CLASSIFICATION; NO ARTIFACT**.

The sole v2 command ran from `0dea4d308dd0c5e25dfa5275c71ec6767e25cf1b`,
tag `cj0-a-coincidence-threshold-probe-v2-implementation`, emitted exactly one
`CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V2_EVIDENCE_SPENT` marker, and exited `101`
at the authoritative past-arrival guard before publishing an artifact.

The remaining mechanical cause is exact. After unused local inputs deallocate,
the authoritative proposal law recreates them with delay equal to physical
distance `2`. Propagating the first fresh changed-world occurrence therefore
drains through `t+2`; the harness then attempted to insert the already
registered second occurrence at `t+1`.

The registered timestamps do not require change. The public substrate accepts
finite future external arrivals before propagation and orders them together
with internally emitted activity. A final staging-only retry may enqueue both
registered burst occurrences at `t,t+1`, call propagation once, and serialize
the resulting trace before the next burst. This adds no signal, state, law,
timing change, selector, or hidden cutoff.

No complete cell, result row, claim vector, or scientific comparison was
published. V1 and v2 artifacts/staging paths are absent. Both invalid markers,
panics, protocols, sources, commits, and tags are immutable. There was no
MICRO, GATE, recursion, OR/timing matrix, definitive evidence, or authority
execution.
