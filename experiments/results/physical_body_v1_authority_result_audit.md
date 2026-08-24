# Physical Body V1 authority result audit

Outcome: **AUTHORITY POSITIVE**.

## Immutable evidence

- Evidence-eligible commit: `010c8712f06418a5bc9511687b17b79441419e77`.
- Frozen tag: `physical-body-v1-authority-frozen-v1`.
- Sole definitive sandbox: `iptf6mhecxja8cmtnwwqw`.
- Immutable evidence commit:
  `4b2c77331708c7b6314ca3dd56d0c0607b6beff7`.
- Evidence tag: `physical-body-v1-authority-positive-v1`.
- CSV SHA-256:
  `37b668f498881ceea60b9a910b34c0b11ca3499e093bad4d90aad984ecc4aad0`.
- Markdown SHA-256:
  `170cd1429b1852534dc2650b423128590fec67930dd91e56f2d0f0e80584955b`.

The definitive command printed
`PHYSICAL_BODY_V1_AUTHORITY_EVIDENCE_SPENT` exactly once and completed with
`PHYSICAL_BODY_V1_AUTHORITY_ESTABLISHED rows=16/16 clauses=540/540`.
It was not rerun.

The two caught panic messages in the sandbox log are the preregistered bounded
capacity controls. They demonstrate that an arena with no free CELL or ARROW
identity rejects overflow. Both panics were caught by the evaluator; all
subsequent clauses and the positive process exit completed normally.

## Functional result

| Evidence | Result |
|---|---:|
| Fresh roots | `16/16` |
| Retained PX-C row clauses | `512/512` |
| Retained cumulative globals | `12/12` |
| Physical Body V1 clauses | `16/16` |
| Total | `540/540` |
| Exact replay | `true` |
| Natural quiescence | `true` |
| Outward-only crossings | `true` |
| Maximum per-batch work | `104331 / 200000` |
| Maximum resident bytes | `44328 / 65536` |

All fresh roots `4_100_001..4_100_016`, origins
`1_040/1_170/1_300/1_430`, allocation orders, and reflected layouts passed.
Construction, pressure origin, and first arrival were equal in every row and
all origins preserved the ten-tick pressure phase.

The cumulative result preserves formation, selective support, persistence,
unsupported and adjacent controls, duplication, resistance controls, direct
and recursive execution, incomplete/open/fork/cycle silence, aging,
deallocation, modulation, physical arrival initiation, closure, and outward
crossing across the retained PX0–PX8 stack.

The successor body result establishes canonical arena and manifest bytes,
stable hashes, identity independent of slots, compaction invariance,
clock-preserving quiescent restart, exact live continuation with admitted load
availability, bounded capacity, deterministic reuse with generation change,
stale-reference rejection, and fail-closed durable decoding.

## Independent artifact audit

Static audit sandbox `itty019pe2fnp1i9f1abx`, commit `53c8b49`, did not
compile or execute the organism. It parsed the immutable CSV and Markdown and
checked:

- exact evidence and source hashes;
- exact root and origin registries;
- four-by-four layout distribution;
- phase-preserving construction geometry;
- every 32-element row vector;
- every row verdict, replay, quiescence, and boundary predicate;
- work and memory bounds;
- all retained positive and negative contrasts;
- all twelve cumulative globals;
- all sixteen named body clauses;
- total clause accounting.

Result: `PHYSICAL_BODY_V1_AUTHORITY_RESULT_AUDIT_PASS`.

Audit JSON SHA-256:
`ee6f6ae720c3506780cd14cc19dc99fc85c26a7a53cde593ac8b058be1d74c25`.

## Authority decision

Physical Body V1 is accepted as the authoritative successor runtime/body.
PX-C remains its cumulative scientific ancestor. No retained physical law was
changed, no semantic surface was added, and production remains independent of
experiment code.

Cold NVMe residence, asynchronous loading, network transport, distributed
execution, foveated embodiment, and organism-visible storage affordances were
not tested and are not claimed.
