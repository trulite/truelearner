# CJ0 Arm A coincidence-threshold CELL PROBE v1 invalid-execution audit

Status: **FROZEN INVALID; NO SCIENTIFIC CLASSIFICATION; NO ARTIFACT**.

The sole v1 command ran from implementation commit
`b82133f65e6be1339196b1dc458c3b3bea5e2a7f`, tag
`cj0-a-coincidence-threshold-probe-v1-implementation`, and emitted exactly one
`CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V1_EVIDENCE_SPENT` marker. It then exited
`101` at the authoritative guard `physical arrivals cannot precede current
substrate time` before publishing a CSV or report.

The defect is mechanically unique. The protocol scheduled every occurrence
as a consecutive-tick burst. That is physically possible for the first weak
occurrence, which leaves only subthreshold convergence state and drains at
`t+1`. Once a conjunction is supported, its first occurrence fires the
convergence CELL and naturally drains return/effect activity through `t+2`;
the harness then incorrectly attempted to insert the second external
occurrence at `t+1`.

This is not a negative or positive observation of the candidate law. No
complete matched world, claim vector, result row, or scientific comparison
was produced. Both final result paths and both staging paths remain absent.
There was no rerun, regeneration, parameter tuning, MICRO, GATE, recursion,
OR/timing matrix, definitive evidence, or authority execution.

The exact v1 protocol, implementation, marker, exit, and empty-artifact state
are frozen. The standing protocol explicitly permits a mechanically forced
timing correction only under a separately frozen fresh protocol. Such a retry
must retain the same physical matter, threshold, law, ordered claims, and
controls; it may change only post-bootstrap external arrival ticks so every
finite propagation naturally drains before the next insertion.
