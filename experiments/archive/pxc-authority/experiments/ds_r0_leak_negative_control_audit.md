# DS-R0 leak and negative-control audit

All controls passed for MICRO seed 100 and GATE seeds 100..104.

| Control | Required result | Outcome |
|---|---|---|
| Fresh activity identities | support and target disjoint | PASS |
| Bijective relabeling | same relation shape | PASS |
| Allocation/layout reversal | same relation | PASS |
| Opaque-handle permutation | different real route, lawful transfer | PASS |
| Changed later activity | stale shape rejected | PASS |
| Other route, same local return law | transfer | PASS |
| Interleaved executions | two graph-local relations, no cross-binding | PASS |
| Equally-close unrelated distractor | ignored | PASS |
| Delayed activity | mismatched shape rejected | PASS |
| Ambiguous fork | abstain | PASS |
| Shuffled temporal order | reject | PASS |
| Reversed propagation | reject | PASS |
| No execution | no relation | PASS |
| No later activity | no relation | PASS |
| Stale route generation | no execution/relation | PASS |
| Subthreshold support | no mature relation | PASS |
| Plasticity disabled | no reconstruction | PASS |
| Evidence bridge | four one-to-one copies | PASS |
| Persistent identity audit | zero forbidden fields | PASS |
| Semantic/update audit | zero consequence or DS1 update paths | PASS |
| Mutation audit | all asserted zero paths detectable | PASS |
| Cleanup | zero temporary relation/surface/routes | PASS |

The interleaved and distractor controls rule out a fixed temporal window: close
activity without a connected physical propagation path is never attached, and
two simultaneous connected components remain separate.

The evaluator effect fingerprint never enters return learning or bridge
membership. It only checks, after execution, that the selected physical trace
belongs to the frozen pre-choice route inventory.
