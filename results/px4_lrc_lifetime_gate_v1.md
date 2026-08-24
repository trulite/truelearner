# PX4 LR-C physical lifetime GATE v1

Status: **DEVELOPMENT POSITIVE**; authority absent.

Protocol: `px4-lrc-physical-lifetime-gate-v1`.

Authority ancestor: `f9057fe78a86db9111b0b69310d03accef3bc970`.

- rows: `8/8`;
- resistance sequence: `4|7|12|22`;
- deallocation-pressure sequence: `4|7|12|22`;
- exact replay: `true`;
- natural quiescence: `true`;
- fresh identity/layout invariance: `true`;
- PX0--PX3 conformance: `true`.

| row | identity | flip | mirror | one exposure | recurrence/pressure | reuse/reacquisition | changed experience | stale generation | controls | replay | result |
|---:|---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 0 | 153001 | false | false | true | true | true | true | true | true | true | PASS |
| 1 | 153002 | true | false | true | true | true | true | true | true | true | PASS |
| 2 | 153003 | false | true | true | true | true | true | true | true | true | PASS |
| 3 | 153004 | true | true | true | true | true | true | true | true | true | PASS |
| 4 | 153005 | false | false | true | true | true | true | true | true | true | PASS |
| 5 | 153006 | true | false | true | true | true | true | true | true | true | PASS |
| 6 | 153007 | false | true | true | true | true | true | true | true | true | PASS |
| 7 | 153008 | true | true | true | true | true | true | true | true | true | PASS |

The measured quantity is ordinary ARROW resistance under ordinary pressure. No organism-visible lifetime representation, episode boundary, cleanup call or delete operation was added. This artifact is development evidence only and does not advance authority.
