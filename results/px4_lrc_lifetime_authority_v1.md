# PX4 LR-C physical lifetime AUTHORITY v1

Status: **DEFINITIVE MATRIX POSITIVE; authority pending coverage and PX-C audits**.

Protocol: `px4-lrc-cumulative-lifetime-authority-v1`.

Authority ancestor: `f9057fe78a86db9111b0b69310d03accef3bc970`.

- rows: `16/16`;
- clauses: `657/657`;
- resistance sequence: `4|7|12|22`;
- deallocation-pressure sequence: `4|7|12|22`;
- exact replay: `true`;
- natural quiescence: `true`;
- fresh identity/layout/schedule invariance: `true`;
- PX0--PX3+LR-C conformance: `true`.

| row | identity | flip | mirror | origin | replicate | one exposure | recurrence/pressure | reuse/reacquisition | changed experience | stale generation | controls | replay | clauses | result |
|---:|---:|:---:|:---:|---:|---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|---:|:---:|
| 0 | 461001 | false | false | 200 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 1 | 461002 | true | false | 200 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 2 | 461003 | false | true | 200 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 3 | 461004 | true | true | 200 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 4 | 461005 | false | false | 400 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 5 | 461006 | true | false | 400 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 6 | 461007 | false | true | 400 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 7 | 461008 | true | true | 400 | 1 | true | true | true | true | true | true | true | 41/41 | PASS |
| 8 | 461009 | false | false | 200 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 9 | 461010 | true | false | 200 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 10 | 461011 | false | true | 200 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 11 | 461012 | true | true | 200 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 12 | 461013 | false | false | 400 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 13 | 461014 | true | false | 400 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 14 | 461015 | false | true | 400 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |
| 15 | 461016 | true | true | 400 | 2 | true | true | true | true | true | true | true | 41/41 | PASS |

The measured quantity is ordinary ARROW resistance under ordinary pressure. No organism-visible lifetime representation, History, episode/reset boundary, cleanup/delete semantic, evaluator-derived lifetime input, typed lifetime handoff or explicit lifetime mechanism invocation was added. This artifact freezes the one-shot definitive matrix only; authority still requires the preregistered coverage, leakage and PX-C audits.
