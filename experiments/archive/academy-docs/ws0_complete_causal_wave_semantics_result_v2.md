# WS0 complete causal-wave semantics result v2

Status: development positive; non-authoritative.

Protocol: `ca341b6` (`ws0-complete-causal-wave-semantics-protocol-v2`).

Frozen candidate: `7b0abff`
(`ws0-complete-causal-wave-semantics-frozen-v2`).

One-shot E2B worker: `i8wqq0gc94bcta63wcbfh`.

## Result

- families: `14/14`;
- permutations per family: `5/5`;
- cases: `140/140`;
- Reference/Production rows: `280/280`;
- clauses: `1860/1860`;
- exact same-mechanics replay: `280/280`;
- exact Reference/Production equality: `280/280`;
- exact handle/physical-ID/insertion/input permutation equality: `280/280`;
- exact live-checkpoint continuation: `280/280`;
- natural quiescence: `280/280`;
- maximum PhysicalWork: `11`.

The matrix covers multiple signed Drive arrivals, multiple Modulatory arrivals,
mixed same/different-junction incidence, Drive-caused Modulation, ordinary
SourceFires Modulation, PQLC continuation, zero-delay Drive and Modulatory
chains, fan-out/merge, recurrence, mixed recurrence, handle renaming, and
reversed insertion/input order.

## Development claim

WS0 establishes:

> Events admitted in one physical wave have no arbitrary software order.
> Drive, Modulatory consequence, and PQLC act through one synchronous local
> incidence model; every caused zero-delay same-phase event belongs to the next
> causal wave.

Modulation never fired a CELL. Drive never acted as consequence. Same-wave
Drive-created participation was unavailable to simultaneous Modulation, while
pre-existing participation remained available. Every CELL fired at most once
per wave.

## Artifacts

- matrix SHA-256:
  `d1d98b76fa6b47cb1096bfa2bcaab251e07b893c6c4a384049b31e0302e24847`;
- report SHA-256:
  `f5a4e58aa153f20dab974bb1fbd72001bf4b3d2505db5b315915f6a8852b90e7`.

The retained SI0/PQLC/cumulative replay prefix is now permitted. RS2 remains
stopped until that prefix passes.
