# DS-RT0 information-flow audit

The retained asset is only the existing A1 local-template/support map. After
the `FREEZE_PROBATION_ASSET` boundary, source audit found:

```text
probation.observe sites        0
probation.install sites        1
fresh execute_root sites       1
direct support mutations       0
new mechanism types            0
semantic/identity fields       0
```

Fresh CELL bindings and route roots are temporary. Persistent state contains
no occurrence, opaque handle, direction slot, evaluator label, reward,
correctness, polarity, or causal metadata.
