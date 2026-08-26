# TC-DS0 checkpoint-negative diagnostic protocol v1

Status: frozen before diagnostic implementation or execution.

Parent negative: `tc-ds0-old-window-characterization-negative-v1` at commit
`c792eb8`.

## Question

Why did the first TC-DS0 row produce different canonical live-checkpoint
hashes under Reference and Production despite identical causal observations,
durable body, physical trace, clock, quiescence, and independent replay?

## Scope

Reproduce only the failed case:

```text
phase               0
delay               0
initial resistance  1
scenario            prompt_modulation
final tick          15
```

The 1,920-row characterization matrix is forbidden. Runtime physics and the
frozen v1 evaluator are byte-identical.

## Diagnostic serialization

For each mechanics configuration, decode the public canonical checkpoint bytes
and serialize:

- header clock, next serial, and section counts;
- every CELL runtime tuple `(id, state, last_update_tick, refractory_until)`;
- every ARROW runtime tuple `(id, eligible_until)`;
- pending activity and pending-load counts;
- first differing byte offset;
- all differing decoded fields;
- durable-body and physical-transition hashes;
- checkpoint hash and independent replay.

Then restore each checkpoint with its originating mechanics, admit the same
future physical inputs, and compare causal physical history and durable body.

## Classification

- Physical counterexample: a differing field changes later physical behavior.
- Mechanics-bookkeeping difference: differences are confined to causally
  inert representation state and identical continuation is observed.
- Uninterpretable: bytes cannot be reconciled or continuation is ambiguous.

This diagnostic cannot repair TC-DS0 v1, publish matrix rows, change a gate, or
advance TC-DS1. Any v2 measurement repair requires its own frozen protocol.
