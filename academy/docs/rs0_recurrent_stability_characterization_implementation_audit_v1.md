# RS0 recurrent stability characterization implementation audit v1

Status: evidence-eligible characterization candidate frozen before any RS0
matrix execution.

## Scope

- parent: CE0 immutable-negative handoff `ed1a6f4`;
- protocol: `be3b4c8`, tag
  `rs0-recurrent-stability-characterization-protocol-v1`;
- physical-law additions: zero;
- plasticity during probe: zero;
- coupling/resistance/participation updates from consequence: disabled because
  the RS0 arm enables `rs0`, not `ce0`, and constructs no Modulatory topology;
- FD2, ARC, authority, oracle, and `arch.md`: unchanged.

## Observer boundary

RS0 adds a feature-gated `ObservedRun` and
`propagate_with_observation_ceiling`. Ordinary `propagate()` now delegates to
the same internal loop with `None`, preserving its unbounded behavior.

The observer changes only loop suspension and maximum mechanics batch size near
the ceiling. Every popped SPIKE undergoes the unchanged physical transition
path. Pending activity remains resident and a subsequent observer call resumes
it. Quiescent RS0 cases additionally run the ordinary unbounded method and
require identical trace, PhysicalWork, clock, durable body, and quiescence.

## Frozen candidate hashes

```text
core lib.rs       007764acac5c45d920512af759c5ac22344074f53b9d091c56ffe5abbd0a341d
core Cargo.toml   5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef
evaluator         37a59bbb7a109d7a916f8c3591ebe32c3161f44f00f4bc317dbd7136dcd640ac
arm Cargo.toml    d0f7bc6e48b0aad1167ca5535e0653db06e9c8cfe10b65ad5a9968b7e89ded14
protocol          ce64bbcdaca7665f7938d8a945b566a1f77c29432a6c9f6aef43985c6f14dee0
CE0 handoff       926683f29535310ee8ebbaa9d46ecc6f2b6cb50411e903237121f697d56b7274
```

## Matrix construction

Twenty frozen geometries cover one-way, reciprocal, chain, and cycles of
lengths 2/3/4/8; coupling 1/2; thresholds 1/2/3; delays 0/1/2/3; mixed delays;
and relative phase. Each uses resistance 1,000,000 and only one initial external
Drive pulse.

The first observer segment is 256 scheduled deliveries. A still-active body is
continued for another 32. Activity classification is computed from observed
CELL firing identity and physical tick periodicity. It cannot affect execution.

The standalone evaluator was formatted and compiled in reusable E2B worker
`i8mm34sawk38wa16yua5o`. The matrix has not run. Strict Clippy, the static
audit, and the relevant observer/no-world tests remain before the one-shot
characterization.

