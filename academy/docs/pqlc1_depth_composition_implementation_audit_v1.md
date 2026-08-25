# PQLC1 depth composition implementation audit v1

Status: frozen before PQLC1 matrix execution.

Parent protocol: `ddcd4e7`, tagged
`pqlc1-depth-composition-protocol-v1`.

## Runtime surface

PQLC1 changes no runtime or substrate source. The complete PQLC0 core surface
is byte-identical to the frozen one-hop candidate:

```text
truelearner/crates/core/Cargo.toml
14d45bc379a5220d33b028b48f38319cb888f732d0b34655fda02b3941a829a8

truelearner/crates/core/src/lib.rs
c5173e8d43d109465252813fba411288c59e3bfa274f790519747eb34314e894

truelearner/crates/core/src/mechanics.rs
266b713130be6b221432022c7518cc413a0def30ca00371422af6aceeda900da
```

Depth and break position exist only in evaluator fixture construction and
serialized evidence. The runtime receives ordinary CELLs, Drive and Modulatory
ARROWs, QLP triggers, and SPIKEs. It receives no depth, route, predecessor,
terminal, or continuation metadata.

The evaluator uses only public observation APIs for contact participation and
support. It contains no mutation of hidden participation, eligibility, or
plasticity state and does not call the private QLP propagation function.

## Frozen matrix surface

The evaluator constructs exactly:

```text
complete                    5
structural break           12
temporal break             12
wrong branch                4
honest fan-out              1
recurrent closure           5
                           --
case variants              39
```

Every variant runs under two identity roots, ten pressure phases, Reference
and Production mechanics, and exact same-mechanics reconstruction. A complete
unconditional result contains `780` physical cases and `1560` mechanics rows.

The evaluator compares complete `Observation` values before predicates. The
observation includes ordered transitions, participation, support, QLP trigger
surface and work, Drive/Modulatory deliveries, updates, proposals,
deallocations, clock/phase, durable-body hash, liveness, and quiescence.

The recurrent family retains the preregistered `8192` physical-work ceiling.
Timeout or failure to return is an immutable negative; no runtime stop or
damping exists.

## Frozen hashes

```text
evaluator Cargo.toml
5bacfb85517a0d426d6a5385b050a037ab15dbab9cddd2e901b1b6ebd084359d

evaluator main.rs
629f9179c1fd401baf3c4e9ff5fb8e37e2d539d5aa832a84053520bd94d52e74

static audit
c5bf469fba6c9d113c18dd7fd421c76a0ed7fae153e34d782d60ca190ea7d1d5

protocol
5b11d45d3d03906fd0bfad54ee9b4c8a00092af2c4f88b155f7430131b1dd6cb
```

## Targeted E2B validation

Reusable sandbox `i5uijy8319ryducypqppi` ran only:

- evaluator formatting;
- targeted evaluator release check;
- strict evaluator Clippy with `-D warnings`;
- shell-audit syntax validation;
- exact frozen core hash checks;
- the evaluator hidden-state mutation scan.

No physical world or matrix ran.

Before this freeze, one missing CSV placeholder and one Clippy-only
row-function shape were corrected. Neither correction executed a physical
world or changed fixture geometry, predicates, or runtime code.

## Boundary

No pressure, eligibility, ARC, authority, oracle, `arch.md`, or substrate law
changes are authorized. No comparator repair, fixture change, damping, rescue,
or parameter tuning is authorized after evidence begins.
