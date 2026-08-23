# CJ0-NOT-2 temporal-absence PROBE v1 result audit

Status: **PROBE POSITIVE; DEFINITIVE ELIGIBLE; PX2 REMAINS AUTHORITATIVE**.

The sole execution emitted
`CJ0_NOT2_TEMPORAL_ABSENCE_PROBE_V1_EVIDENCE_SPENT`, exited zero, and
atomically published:

| artifact | SHA-256 |
|---|---|
| PROBE CSV | `07cb0d4ccbd817c6de56166f89d4e5719a4d645bfab9d78718169538d36cad7d` |
| PROBE report | `0f4b0c554275a820dfc6a3de7799736edebf2f5aa272dc211b7228a66c3fd05b` |

All `12/12` rows passed with exact duplicate replay and natural quiescence in
both trigger and final propagations. In both independently identified
normal/mirror layouts:

- trigger fired and changed the complete state fingerprint before B or closure
  was entered;
- B absent through closure: closure reached the transient CELL at tick `2`,
  transient/output each fired once at tick `2`;
- B in-window at tick `1`: its negative activity erased the alternative and
  transient/output remained silent;
- B at tick `2` before closure: its negative activity arrived at the closure
  boundary and transient/output remained silent;
- B after closure at tick `3`: output had already fired at tick `2`, with no
  retroactive effect;
- blocked and pressure-staled B paths did not deliver negative activity, and
  the closure/output path operated.

Work was nonzero in every row (`46..71` ledger operations), persistent storage
was exactly `496` bytes per fixture, and the stale rows recorded one physical
deallocation. Complete/permanent fingerprints and staging-path cleanup were
verified.

The authoritative PX0 source and PX2 definitive CSV hashes remained exact. No
authoritative byte changed.

## Classification boundary

NOT-2 is positive at PROBE resolution: existing transient CELL state, ordinary
time/decay, signed coupling, physical closure, pressure, firing, and quiescence
support the preregistered temporal-absence behavior. This is not yet
definitive, introduces no absence symbol or persistent variable, does not speak
for NOT-1, does not reinterpret PX3, and advances no authority. A fresh
definitive protocol may now be preregistered.
