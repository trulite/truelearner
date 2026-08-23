# PX7 physical arrival initiation PROBE result

Verdict: **PASS**.

Frozen parent: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. No new mechanism or substrate-law change was used.

| arm | learned coupling | held-out source/execution/boundary | crossing | quiescent | duplicate | work | bytes before/after | result |
|---|---:|---|---:|:---:|:---:|---:|---|:---:|
| learned-return | 2 | 1/1/1 | 1 | true | true | 145 | 208/272 | PASS |
| unreturned | 1 | 1/0/0 | 0 | true | true | 92 | 208/528 | PASS |
| subthreshold | 2 | 0/0/0 | 0 | true | true | 124 | 208/272 | PASS |
| absent | 2 | 0/0/0 | 0 | true | true | 120 | 208/272 | PASS |

Organism-visible execution used only the frozen CELL/ARROW/SPIKE substrate and actual local participation/return state. Scenario names and pass clauses were evaluator-only.
