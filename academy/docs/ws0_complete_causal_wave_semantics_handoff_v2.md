# WS0 complete causal-wave semantics handoff v2

WS0 is development-ready on
`research/ws0-complete-causal-wave-semantics`.

## Consolidated result

The complete wave model passed `140/140` cases, `280/280` mechanics rows, and
`1860/1860` clauses across 14 Drive/Modulatory/PQLC families and five
renaming/insertion permutations.

The unchanged retained prefix then passed on reusable E2B worker
`ifk44bxtlfjlci644r63m`:

- R1-R5 mechanical differential: PASS;
- R6 partition successor: `38/38`;
- SI0 v2: `120/120`;
- CPC0 current-parent differential: `440/440` rows;
- CPC1: `620/620`;
- PQLC0: `200/200`;
- PQLC1: `780/780`;
- FD0: `100/100`;
- FD1 v3: `140/140`;
- J0: `160/160`, `1880/1880` clauses;
- CV0/J0 + SV1: `240/240`, `5480/5480` clauses.

Every retained evaluator preserved its frozen Reference/Production, replay,
and quiescence contract. The 23-file retained manifest reverified exactly.

Manifest SHA-256:
`4d8b84f085ae56f8b479865164d65b2e6bac3408b69a5ad7d05c696265ed16f0`.

Execution log SHA-256:
`2d6f73855f1bc05fdd555bcf55024aaf656eacd90438075a05766b2c8013bc2e`.

## Runtime model

The canonical runtime remains the single file
`truelearner/crates/core/src/lib.rs`.

The event constitution is now:

```text
same tick + phase + causal generation
    -> one synchronous local incidence wave

Drive
    -> signed activation incidence

Modulatory
    -> local consequence incidence, never excitation

threshold/refractory
    -> at most one CELL firing per wave

SourceFires or PQLC transmission caused by the wave
    -> next causal wave
```

Numeric identity, insertion order, serial order, and observer order are not
causal facts.

## Next boundary

The retained prefix authorizes one consolidated RS2 retry using the frozen
signed-variation/training/probe science. CE1, FD2 v2, ARC A2, authority,
oracle status, and `arch.md` remain unchanged until that retry is positive.
