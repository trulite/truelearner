# WS0 complete causal-wave semantics implementation audit v1

Status: frozen before WS0 evidence.

Protocol: `7f194d4` (`ws0-complete-causal-wave-semantics-protocol-v1`).

## Runtime delta

The one-file physical runtime now gives SI0 wave execution complete coverage
of the existing transmission surface:

- a drained `(tick, phase, causal_wave)` admits Drive and Modulatory arrivals;
- arrivals are grouped by junction and effect without CELL/ARROW ordering;
- Modulatory incidence preserves multiplicity, cannot excite a CELL, and acts
  on pre-wave local participation;
- signed Drive incidence updates activation once and evaluates firing once;
- SourceFires Drive and Modulatory ARROWs share one outgoing traversal path;
- PQLC uses the same Modulatory incidence path;
- every zero-delay, same-phase caused transmission receives
  `causal_wave + 1`;
- positive delay or changed phase retains the existing time/phase separation;
- a new `ModulatoryIncidence` trace event exposes wave, target, combined
  impulse, and arrival multiplicity without becoming organism state.

The obsolete assertions that SI0 outgoing/arriving ARROWs must be Drive were
deleted. No new CELL, ARROW, SPIKE, timer, path, predecessor, depth, credit,
reward, backward mode, or runtime dependency was added.

The canonical runtime remains one file:
`truelearner/crates/core/src/lib.rs`, 5,556 lines, SHA-256
`d12b02bbb85645a916a5690d5ce5ebfd8e5c9d6820025a0c6d315a55aa0180a9`.

## Frozen evaluator

The WS0 evaluator contains the 14 preregistered families and five mechanical
permutations. It compares logical wave-normalized histories, PhysicalWork,
clock, normalized durable body, same-mechanics replay, Reference/Production,
permutation invariance, natural quiescence, and live-checkpoint continuation.

Evaluator SHA-256:
`53d1f1f81a3f24796d2c3c3b5d9e9458aa77b94861f0687a73e1df2370f0d2cb`.

## Targeted validation

Reusable E2B Rust worker `ifk44bxtlfjlci644r63m`, committed candidate
`6713d5dc8ea96edd4a2a0a6ff29b621b026a5a4b`:

- core and WS0 rustfmt checks: PASS;
- WS0 release `cargo check`: PASS;
- WS0 strict release Clippy: PASS;
- retained SI0 evaluator release check: PASS;
- stopped RS2 evaluator release check: PASS.

No WS0 physical world had executed when this audit was frozen.
