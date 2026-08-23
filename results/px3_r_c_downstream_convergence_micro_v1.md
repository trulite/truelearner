# PX3-R Arm C MICRO development result

Verdict: **FROZEN NEGATIVE**. PX3 remains absent; this is not definitive evidence.

- first collapse: `STAGE_CONJUNCTION`
- rows: `1`
- total ledgered work: `446632`
- summed persistent storage: `32304` bytes

| cell | trained use | crossed use | trained common | crossed common | old before/after | new after | swap new/old | duplicate | pass |
|---|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|
| swap-and-controls | [1, 1] | [0, 0] | [1, 1] | [0, 0] | 23/35 | 24 | [2, 2]/[2, 2] | true | false |

## Serialized physical state

### swap-and-controls

- individual correspondence resistance: `[32, 32, 32, 32]`
- individual direction resistance: `[27, 27, 27, 27]`
- opportunity resistance: `[[20, 11, 0, 0], [20, 13, 0, 0], [13, 27, 0, 0], [11, 15, 0, 0]]`
- measured opportunity impulse: `[[2, 2, 0, 0], [2, 2, 0, 0], [2, 2, 0, 0], [2, 2, 0, 0]]`
- controls: `stale=0|ambiguous_symmetric=true|ambiguous_sites=[0, 0, 0, 0]|multiple=false|correlation=0|without_return=[0, 0]|absent=[0, 0]`
- opportunity additions / ARROW count / persistent bytes: `364/456/32304`
- permanent fingerprint: `7950204310758621851`
