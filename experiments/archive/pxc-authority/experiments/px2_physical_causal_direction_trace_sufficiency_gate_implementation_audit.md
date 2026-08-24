# PX2 physical causal direction trace-sufficiency GATE implementation audit

Status: **IMPLEMENTATION FROZEN; GATE EVIDENCE UNSPENT; PX2 NON-AUTHORITATIVE**.

- GATE source SHA-256:
  `8cdd72cff084c6a85d65629fd6504f5ca96f14d281a7a5ac518fd9c4754579ec`;
- positive MICRO source SHA-256:
  `cc31a4992488c5fefe35703d34174ac6c50c0d4fbdd1c79623b3723a3aa4e27a`;
- GATE protocol SHA-256:
  `fd0235ef42b54e4cc5a7e0f2673d57249f8b60b4bb2ca7ddc35396c162e75917`.

The substrate crate is byte-identical. The implementation changes only the
evaluator/world matrix around the frozen PX1 participation-trace law:

- four fresh physical namespaces/layout strata;
- traversal delays `3..=6` with the trace/return interval unchanged;
- `0,8,24,48` active distractor pairs;
- `1..=4` genuinely executable parallel weak paths;
- adversarial consequence schedules, interleaved traversal schedules, and two
  forgetting/opposite-reacquisition histories;
- independently serialized path count, resistance range, traversal, trace,
  return, execution, stale-generation, quiescence, replay, and work fields.

No trace, threshold, coupling law, eligibility duration, pressure rule,
plasticity rule, return route, damping rule, or organism-visible representation
was added. Evaluator-only scenario names and pass vectors have no path into the
substrate.

Before evidence, formatting, focused compilation, strict focused Clippy, the
focused substrate test, wrong/definitive-mode refusal, frozen-parent hash
checks, result-path absence, and diff checking passed. Definitive execution
remains forbidden.
