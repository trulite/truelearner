# PX4 LR-C physical lifetime GATE result audit v1

Status: **DEVELOPMENT GATE POSITIVE; READINESS AUDIT ELIGIBLE; AUTHORITY ABSENT**.

The unchanged implementation executed the preregistered eight-row GATE from
clean commit `1040b9fe5832aed6f12bd5e759b9284d67c9b868` in fresh E2B
sandbox `i9y7kazos146gzwuphwk6`, using unique state file
`px4-lrc-lifetime-gate-20260824.json`. A second fresh sandbox
`i05mu61g04ubqr5aqn6n5`, with unique state file
`px4-lrc-lifetime-gate-replay-20260824.json`, executed the identical clean
snapshot and reproduced both artifacts byte-for-byte. Both sandboxes were left
running.

## Frozen artifacts and exact replay

| artifact | first run SHA-256 | replay SHA-256 | exact |
|---|---|---|:---:|
| GATE CSV | `7789fe652e39e77e8d909b2cd34ec71b8fcdc3ee6564d8f18ba1840f8fdb9d54` | `7789fe652e39e77e8d909b2cd34ec71b8fcdc3ee6564d8f18ba1840f8fdb9d54` | true |
| GATE report | `1242962338614ea4087e5d0bf4e0f52ea336aa69d8aa03c0715236ecc711ed71` | `1242962338614ea4087e5d0bf4e0f52ea336aa69d8aa03c0715236ecc711ed71` | true |

The CSV contains eight data rows; every row and its header have exactly 38
fields.

## Functional verdict

```text
rows                                  8/8
normal / reversed allocation          4 / 4
forward / reflected geometry          4 / 4
resistance after 1/2/4/8 support      4 / 7 / 12 / 22
pressure steps to deallocation        4 / 7 / 12 / 22
one-exposure controls                 8/8
recurrent persistence                 8/8
disuse and pressure deallocation      8/8
reuse advantage / no reproposal       8/8
changed old loss / new persistence    8/8
ordinary reacquisition                8/8
stale-generation blocking             8/8
return-alone / late / Drive controls  24/24
fresh identity/layout invariance      true
PX0 / PX1 / PX2 / PX3 conformance     true / true / true / true
in-row exact replay                   true
cross-sandbox artifact replay         true
natural quiescence                    true
first collapse                        none
```

## Development claim

Within the registered smallest existing-physics geometries, physical lifetime
collapses onto the already-authoritative scalar resistance acted on by
recurrence/reuse through eligibility and LR-C modulation, and spent by
ordinary pressure. The same physical deallocation and reproposal path handles
disuse, changed participation, stale queued activity and reacquisition. No
lifetime representation, history object, episode boundary, cleanup call,
delete semantic or new substrate law was added.

This is a functional development result only. PX4 development readiness still
requires complete active-surface coverage, an E2B PX-C taxonomy result and a
passing comparator against the immutable PX3 baseline. No authority execution
or authority claim is permitted.
