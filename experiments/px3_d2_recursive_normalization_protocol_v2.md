# PX3-D2 recursive normalization protocol v2

Status: **PREREGISTERED; SUPERSEDES UNEXECUTED V1 HUB SCOPE; EVIDENCE UNSPENT**.

The unexecuted v1 protocol used the phrase “shared return” without fixing its
scope. A single hub shared across staggered A/B/C/D episodes would itself retain
eligibility/plasticity and could later broadcast coupling greater than one into
inactive primitive traces. That would contaminate the normalization comparison.

V2 freezes one identical authoritative normalization motif per participant,
primitive or derived:

```text
participant outlet
  -> unit local trace input
  -> ordinary participant-local return relay -> unit trace input
trace threshold 2 -> exactly one unit participation firing
```

A, B, C, D and X all use this exact topology, delays, thresholds, couplings and
resistances. No participant has a primitive/derived flag. Hubs are not shared
across participant identities or staggered episodes. Downstream X+C and D+C
still receive identical unit trace ARROWs.

All v1 hypotheses, scenarios, seeds, commands, artifact paths, verdicts and
write-once rules remain unchanged. No v1 world or evidence run occurred.
