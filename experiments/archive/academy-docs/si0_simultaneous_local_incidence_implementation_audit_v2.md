# SI0 simultaneous local incidence implementation audit v2

Status: frozen observer-only candidate; SI0 v2 evidence has not run.

Protocol: `d2b66fc` (`si0-simultaneous-local-incidence-protocol-v2`).

## Runtime identity

The runtime is byte-identical to SI0 v1:

- `truelearner/crates/core/Cargo.toml`: `d7d34bb477bc74657d8d1486d2c04fef759bb5f91ce5b08b805891f0bd75819c`
- `truelearner/crates/core/src/lib.rs`: `f19a89ac92c12cc4910047021c8bdedfa42b4c4dc2f5c3fcfa83e2a0b2a4c978`
- `truelearner/crates/core/src/mechanics.rs`: `5f1172a0eaa0628d1775029c44e7a1b5bb2c4525c713b468f756a0705ef822a4`

The v1-to-v2 source diff contains no runtime path.

## Observer delta

Exactly one evaluator responsibility changed. `normalize_trace` now groups
events by `(tick, phase, causal_wave)` and records three independently sorted
collections:

- combined junction incidences;
- junctions that fired;
- remaining ordinary effects produced in that wave.

It never attaches a fire to an incidence by sequential adjacency and never
uses CELL/ARROW/physical identities or serial as an observation ordering rule.
Different wave keys remain ordered, preserving genuine causal succession.

Apart from the v2 result path, report title, and sentinel, the frozen
world-building and comparison source is byte-identical to v1. Evaluator SHA-256:
`0d5bbb53c52682cd49b0b9f554ff7e1b89a96967a4d2f84567ca46306fa0915d`.

Targeted formatting, release check, and strict Clippy passed in reusable E2B
worker `ifk44bxtlfjlci644r63m`. No world executed during validation.

No architectural hardening or downstream experiment is part of this freeze.
