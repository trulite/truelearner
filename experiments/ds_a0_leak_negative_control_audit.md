# DS-A0 leak and negative-control audit

Implementation audited: `375bf2c3f7da3f2695cc4910347826c3b5b37278`.

## Boundary audit

Persistent learner state consists only of `RouteTemplate { temporal_deltas,
local_incidence }` and `SupportEvidence { episodes }`. Its GATE fingerprint is
`265f5745ff745b48` for every seed and it occupies 14 bytes per independent
learner. It contains no occurrence, CELL, ARROW, binding, destination, root,
handle, layout, semantic role, evaluator value, future answer, or consequence.

The owned-source audit reports:

```text
semantic opcode sites                 0
evaluator selection/ranking sites     0
hidden handle/root executor sites     0
DS1 choose calls                      0
DS1 apply calls                       0
post-action consequence paths         0 (derived)
root executors                        1
bridge constructors                   1
plastic route installers              1
preassembled RouteInstance fields     0
```

The final three zero counts are not availability constants. The audit searches
owned call surfaces for DS1 calls, extracts the actual `execute_handle` body,
counts consequence/credit/reward/terminal sink references there, and adds any
DS1 apply edge. Per-seed values copy this derived audit. The focused
`forbidden_calls_and_sinks_are_derived_from_owned_source` test requires these
derived counts and the copied seed values to remain zero.

## Positive and negative controls

All controls passed in MICRO seed 100 and GATE seeds 100..104:

| Control | Audited observation |
|---|---|
| fresh/disjoint IDs | acquisition and evaluation occurrence sets are disjoint |
| bijective relabel | route count transfers |
| allocation/layout | changed CELL allocation plus 257-byte padding transfers |
| handle permutation | multiset of physical states is invariant; no rank survives |
| distractor activity | installed roots bind only current DS-E0 members |
| shuffled coactivity | persistent fingerprint is unchanged |
| shuffled propagation storage | installed route count is unchanged |
| one physical observation removed | unchanged DS-E0 EventFrame support; installed roots lawfully fall 2 -> 1 |
| changed surface timing | constant shift preserves relative support and 2 routes |
| symmetric ambiguity | separately learned symmetric support installs 2 unranked routes |
| no local plastic formation | identical mature-support/raw-activity arm installs 0 CELLs/ARROWs and exposes 0 roots |
| unsupported route | raw activity with an empty learner installs/exposes 0 roots |
| stale generation | changing one installed root generation prunes roots 2 -> 1 |
| cleanup | retained handles, roots, CELLs, ARROWs, spikes, propagation observations all 0 |
| independent execution | every route starts from the identical cloned state; all 80 GATE route pairs differ |

The baseline contains two distractor ARROWs outside DS-E0 membership and zero
executable event roots. Raw propagation observations are deliberately a
non-executable type. Only the learned plastic installer can create the 3-CELL,
2-ARROW route chains consumed by the bridge and executor.

No action consequence, correctness, rank, DS1 acquisition, or DS1 retry was
run. No result artifact exists.
