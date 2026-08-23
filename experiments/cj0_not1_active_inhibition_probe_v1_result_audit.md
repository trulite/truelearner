# CJ0-NOT-1 active-inhibition PROBE v1 result audit

Status: **PROBE POSITIVE; DEFINITIVE ELIGIBLE; PX2 REMAINS AUTHORITATIVE**.

The sole execution emitted
`CJ0_NOT1_ACTIVE_INHIBITION_PROBE_V1_EVIDENCE_SPENT`, exited zero, and
atomically published:

| artifact | SHA-256 |
|---|---|
| PROBE CSV | `4f3ad19bea689a60641852ef038e7ba5d8938e8dcdba802f0019dea8df68dedb` |
| PROBE report | `365f665e609b50ec6b35b4d3768f7a78f8199f9f645fcbb16319c9abee1bd5df` |

All `10/10` rows passed with exact duplicate replay and natural quiescence.
In both independently identified normal/mirror layouts:

- A absent: B, integration, and output fired once;
- A timely: A and B fired and the `-2` impulse arrived at tick `1`, while
  integration and output remained silent;
- A too late: output fired at tick `2`, before the negative impulse arrived at
  tick `3`;
- blocked A: A fired but no negative impulse arrived, so B/output operated;
- stale A: ordinary pressure physically deallocated the weak delayed A path,
  no negative impulse arrived, and B/output operated.

Work was nonzero in every row (`28..50` ledger operations), persistent storage
was exactly `384` bytes per fixture, and complete/permanent fingerprints were
serialized separately. Staging paths were absent after publication.

The authoritative PX0 source and PX2 definitive CSV hashes remained exactly
`3ee8b2bf...ad12d` and `921e433e...eb18`. No authoritative byte changed.

## Classification boundary

NOT-1 is positive at PROBE resolution: existing ordinary negative coupling can
make timely A activity suppress a B path, and cannot act retroactively. This is
not yet definitive, does not create a logical NOT primitive, does not speak to
temporal absence, does not reinterpret PX3, and advances no authority. A fresh
definitive protocol may now be preregistered.
