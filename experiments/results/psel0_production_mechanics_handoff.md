# PSEL0 Production Mechanics Handoff

## Selected body

Use `MechanicalConfig::PRODUCTION` for the next production-runtime successor:

```text
TimingWheel
Adjacency
Frontier
AoS
Batched where safe; scalar fallback for live zero-delay topology
```

Keep `MechanicalConfig::REFERENCE` and the differential harness permanently.

## R6 starting invariant

R6 must start from this exact selection on a new branch. Its first discriminator
is only:

```text
one resident arena
vs
the same physical graph partitioned across N resident arenas
with zero added inter-arena latency
→ exact same physical history
```

Storage loading, eviction, network transport, and admitted latency remain R7+
questions and must not be introduced into that first R6 control.

## Review targets

- `truelearner/crates/core/src/lib.rs`
- `truelearner/crates/core/src/mechanics.rs`
- `experiments/performance/psel0-mechanics/src/main.rs`
- `results/psel0_mechanics_v1/psel0_costs.csv`
- `experiments/results/psel0_production_mechanics_selection.md`

The untracked root file `academy.md` remains untouched.
