# Reordered de-supply sprint activation handoff

Status: **M3 -> DS6 CUMULATIVE DEVELOPMENT ACTIVE**.

This handoff activates the ordering amendment frozen at commit
`12ad06dfd2363ad8daea918931fd572c7b431f79`, tag
`desupply-sprint-order-after-ds4-negative-v1`.

The current serial authority state is:

```text
M3 authoritative
DS4 attempt #1 definitive negative and immutable
M4 absent
current cumulative target DS6 lifetime/persistence
```

The active authority line is:

```text
M3 -> DS6 -> M4 -> DS7 -> M5 -> DS8 -> M6
   -> separately named DS4 successor -> M7 -> DS5 -> M8
```

DS6 development must start from byte-frozen M3. It may inspect and reuse
historical parts, but no isolated result or counterfactual parent is authority.
Supplied request/start and finish/output conventions may operate the external
task harness only; they may not enter acquisition, storage, lifecycle,
retention, erasure, reopening, or reuse decisions.

No DS6 definitive execution is authorized. The permitted sequence is:

```text
static dependency audit
-> frozen diagnostic protocol
-> PROBE
-> MICRO if warranted
-> GATE if warranted
-> separate definitive preregistration and authority execution
```

The standing mechanical-linker policy remains active. A missing connection is
repaired and retried without changing the target. A new persistent
representation, equally supported non-semantic lifecycle choices, or semantic
boundary leakage requires an explicit scientific stop.

