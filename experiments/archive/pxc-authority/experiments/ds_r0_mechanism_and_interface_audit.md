# DS-R0 mechanism and interface audit

## Exact source boundary

| Item | Frozen SHA-256 |
|---|---|
| DS-R0 mechanism | `f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51` |
| DS-R0 runner | `652767b7fa7eb49684d330f99a3c64fbc17be1ebe2fdd69f61bf4098feaddf5c` |
| E0 source | `fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615` |
| A1 source | `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1` |
| Stage-six parent | `3b96de98a8f91ca9f7338d1184d4d2e6c10e6528783820030d6ae74dae81d08e` |
| Stage-seven handoff | `3f68560d86171a29ea159c90e5a05584554a9d06c4fa12f9ee54a192f9b53bfd` |
| Frozen DS1 slice | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |

The build copies frozen E0/A1 sources without editing them. DS-R0 adds one
semantics-blind physical activity observer, one return learner, and one
format-only bridge.

## Runtime interface

The observer is inside the ordinary selected-route traversal. For every actual
root injection/CELL activation it emits a fresh occurrence; for every actual
local transition it emits a directed propagation relation. The learner sees
only occurrence-local ticks and this physical adjacency.

The learner's persistent state is exactly:

```text
ReturnShape { lag: u16, hops: u8 }
ShapeSupport { count: u16 }
```

It stores no occurrence, opaque handle, route root, CELL, destination,
episode, filler, or evaluator value. Target membership exists only in the
temporary relation and is erased after the bridge copies it.

The bridge copies exactly four already-materialized values:

```text
temporary member 0 -> anonymous member 0
temporary member 1 -> anonymous member 1
relative lag       -> relative lag
propagation hops   -> propagation hops
```

It does not search raw activity, infer membership, interpret a route, compare
effects, construct a boolean, or call DS1.

## Derived source inventory

```text
physical observer definitions          1
return learner implementations         1
evidence bridge definitions            1
frozen DS1 choose calls                 1
frozen DS1 apply/update calls           0
semantic outcome sites                  0
evaluator-to-return-learner edges       0
evaluator-to-evidence-bridge edges      0
persistent identity fields              0
```

The asserted zero update/evaluator paths are mutation-sensitive. Injected
synthetic apply, evaluator-to-learner, and evaluator-to-bridge paths are each
detected by the audit.
