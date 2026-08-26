# RI0 renaming-invariance implementation audit v1

Status: frozen before RI0 evidence.

Protocol: `5238d6f` (`ri0-renaming-invariance-protocol-v1`).

The evaluator contains two minimal same-tick collision worlds and seven frozen
renaming/insertion permutations. It maps CELL, ARROW, and physical identities
back to logical graph names before comparing ordered transitions, durable and
future-causal transient state, Work, clock, pending state, replay, and
quiescence.

It changes no core, scheduler, RS2 evaluator, or organism law.

Targeted E2B validation at committed evaluator `4b30d74`:

- worker: `iovubicq36kzjvwodff8f`;
- rustfmt check: PASS;
- release check: PASS;
- strict release Clippy: PASS.

No RI0 physical world had executed when this audit was frozen.

