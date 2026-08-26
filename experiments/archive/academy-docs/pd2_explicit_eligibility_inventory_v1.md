# PD2 explicit-eligibility inventory v1

Status: frozen against PD1 handoff `f5a7675bb48ae4e5b19ed9e15504d9c7e7eb9442` before PD2 runtime edits.

Active-core search terms:

```text
eligible_until
eligible_arrows
eligible_frontier
LOCAL_WINDOW
PhysicalEvent::Eligible / Eligible { ... }
```

The PD1 parent contains 65 matching tokens in `core/src/lib.rs` and eight in
`core/src/mechanics.rs`. Every causal and noncausal responsibility is accounted
for below.

| Surface | Current responsibility | Classification | PD2 action |
|---|---|---|---|
| `LOCAL_WINDOW = 4` | constructs traversal deadline | plasticity qualification / supplied horizon | delete |
| `Arrow.eligible_until` | stores deadline | typed representation | delete |
| SoA `eligible_until[]` | alternate resident layout | typed representation | delete |
| `PhysicalEvent::Eligible` | reports deadline | debug/accounting | delete |
| `PlasticSubstrate.eligible_arrows` | finds expiry candidates | pressure protection / temporary cleanup | delete |
| `ExecutionCost.eligible_frontier_*` | counts the old frontier | debug/accounting | delete |
| traversal writes at ordinary SourceFires arrows | opens old window | dead scaffolding after CPC1 | delete |
| traversal writes at QLP arrows | opens old window | dead scaffolding after CPC1 | delete |
| legacy non-CPC1 Modulation read | gates strengthening by deadline | plasticity qualification | delete with legacy branch |
| CPC1/PD1 Modulation clears | consumes old window | dead scaffolding | delete |
| legacy ordinary-pressure read | suppresses covered epochs | pressure protection | delete; PD1 already bypasses |
| unsupported-expiry scan/decrement | punishes elapsed window | pressure protection / cleanup | delete |
| constructor and arena-restore initialization | initializes field | typed representation | delete |
| index rebuild and resident-byte accounting | maintains old frontier/storage | debug/mechanics | delete |
| `local_eligible_until()` | exposes field to evaluators | debug/accounting | delete |
| `ArrowRuntime.eligible_until` | live-checkpoint bytes | checkpointing | delete |
| live-checkpoint encode/decode/capture/restore | preserves deadline | checkpointing | delete |
| quiescent-checkpoint predicate | waits for deadline cleanup | checkpointing | delete predicate |
| internal legacy tests | assert deadline behavior | test fixture | replace with participation/load assertions |
| PD1 evaluator fields/assertions | demonstrate old field is inert | test fixture | remove in successor replay |
| frozen CPC0/TC-DS0/PD0 evaluators | historical characterization | immutable evidence | leave unchanged at historical tags |

No active use remains unclassified. Archived PX/LR-C sources under
`experiments/archive/` are immutable historical implementations and are not
part of the active-core deletion target.

## Existing state that remains

PD2 adds no physical state. It retains the already-earned fields and laws:

```text
participation_level
plastic_support
pressure_load
resistance
TransmissionMode
TransmissionTrigger
```

The live-checkpoint responsibility is transferred only to those already
future-causal quantities where the successor checkpoint claims exact live
continuation. This is persistence of existing physics, not replacement
eligibility.
