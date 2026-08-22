# DS-A0 executable-route trace equivalence/difference audit

Implementation: `375bf2c3f7da3f2695cc4910347826c3b5b37278`.

Every alternative is executed from a fresh clone of the exact same frozen
temporary substrate after all routes have been installed. Execution begins
with a SPIKE at the bridge-resolved root. The executor has no route-cell array;
each next CELL is discovered from live ARROW adjacency.

For the ordinary allocation arm, the six event/distractor CELLs occupy
episode-local slots 0..5. Plastic installation then produces two isolated
current routes with trace shapes `[6,7,8]` and `[9,10,11]`. These numbers are
temporary allocation evidence only and are never persisted. Each trace has
three SPIKE propagations, three state mutations, and two ARROW traversals.
Their cloned end states differ at the corresponding three activation slots.

| Matrix | Independent executions | ARROW steps | Distinct trace/effect pairs | Empty paths | Equal pair effects |
|---|---:|---:|---:|---:|---:|
| MICRO seed 100 | 16 | 32 | 8 | 0 | 0 |
| GATE each seed | 32 | 64 | 16 | 0 | 0 |
| GATE seeds 100..104 | 160 | 320 | 80 | 0 | 0 |

Handle permutation changes which opaque integer is copied beside which root,
but the sorted multiset of independently produced temporary states is exactly
unchanged. Relabeling and allocation/layout controls preserve the count and
effect distinction without preserving any concrete slot. The symmetric arm
also exposes two roots but no rank or preferred handle.

Changing one installed root generation causes validation and pruning before
bridge construction, reducing the route inventory from two to one. Removing
one current raw propagation observation preserves DS-E0 support but causes the
plastic installer to create only one route. Disabling formation over identical
raw activity creates no installed route and therefore no trace at all.
