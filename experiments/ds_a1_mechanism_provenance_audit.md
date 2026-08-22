# DS-A1 mechanism, source, and actual-E0 provenance audit

DS-A1 is a development-only enabling gate. The accepted preregistration is the
amended protocol at `0cf66a6bf1957fc1d9e6b22d7541623e3405e354` /
`ds-a1-affordance-multiplicity-protocol-amendment`. The original protocol tag
at `08797f85b67ddfc69e6068e6bc71321ed0927a3b` was not moved and is superseded
only for the evaluator/bridge ordering correction.

## Exact immutable boundary

| Item | SHA-256 |
|---|---|
| frozen DS-E0 source | `fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615` |
| frozen accepted DS-A0 source | `3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527` |
| frozen marked DS1 learner | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |
| prior composition source | `3e3f5227fa570e52043c8eb4d3bdbe8242c74f0fa8fe8394693b76bde420af8b` |
| amended protocol | `cbce3274d1e66c31d0bf297d2117604fb8a79795961d80950b5cf3849fbfda1b` |
| DS-A1 mechanism | `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1` |
| DS-A1 runner | `4e1bab85697ab5c91d9b2e01ffcb8c2de07004b6c6dbc81fa167ed7bc5e40512` |
| build-time fingerprint audit | `0b58cb0b81c8cb1101e484570bdc4eaa4ad48f5c4686c7d10710340b38d6e5d8` |

The build independently computes the three frozen file hashes and injects them
for mechanical stage-0 comparison. All matched locally and in E2B. The exact
required parent is `f4aeae4ae2f1832bc469621d79f7bb5b3fd6d1d0` and its tag still points
there.

## Actual E0 path

Each seed runs frozen E0 acquisition, then forms twelve actual support events
and one actual target event. A composition-only shim copies exactly the fields
already audited in the frozen retry: all eight raw occurrence/tick pulses, all
five raw propagation endpoint pairs, three formed members, nine temporal
relations, and nine propagation relations. Every copy is compared field by
field to the same raw episode and `EventRelations`. Acquisition/support and
target occurrence sets are disjoint.

No A0 or A1 fixture is called on the primary path. The two target candidates
are precisely the two observed propagation relations whose endpoints are both
members of that actual formed E0 event and whose directed learned relation is
present. The other three raw relations remain present as distractors and are
excluded only by current E0 membership.

Each installed route has two newly allocated episode-local CELLs, each bound
one-to-one to a current E0 member CELL, plus one live ARROW. No persistent
template contains occurrence, CELL, episode, seed, destination, expected count,
effect, or meaning.

## Source/call inventory

The mechanical audit reports exactly one generic local proposal traversal, one
installer, one adjacency executor, and one bridge constructor. It reports zero
DS1 choose calls, zero DS1 apply calls, zero consequence paths, zero semantic
opcodes, zero expected-route tables, zero hidden executors, zero persistent
identity fields, and zero effect-to-bridge edges. Independent source mutations
make choose, apply/consequence, opcode, and hidden-executor counts nonzero.

The bridge body receives only structurally unique root references. Normalized
effect types and execution calls do not occur in it. All effect evaluation is
strictly post-bridge.
