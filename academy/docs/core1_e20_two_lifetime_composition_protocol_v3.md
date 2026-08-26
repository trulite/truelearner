# CORE1-E20 — Two-Lifetime Composition Protocol v3

## Status

Evaluator-only correction after the first attempted P2 process stopped before
route participation and before completion or consequence. Protocols v1 and v2
remain binding. No valid P2 observation and no E20 evidence marker exists.

## Corrected evaluator mismatch

E20 v1 freezes the E19 body and initial decision activation. The first E20
evaluator instead bypassed that activation and forced every first route through
the later E17 transient-history continuation surface. In useful-first seed 0,
that surface correctly rejected its input because only one signed contact
remained:

```text
transient history requires live positive and negative contact alternatives;
positive=true negative=false contacts=1
```

This occurred before route 1 participated, so it cannot answer P2.

## Sole correction

At the start of every opportunity, restore E19's ordinary junction activation
over the four opaque actions. While OPEN, actual Drive traversal in that
activation is eligible to write USED-PENDING. If it emits a motor action, that
is the first actual route attempt. If it emits no motor action, no route has
participated and any speculative traversal-capture bits from that activation
are cleared before the unchanged refractory selector may initiate a route.

This correction is identical for all seeds and mechanics. It does not inspect
usefulness, choose an action, add a retry, return a completion, preserve an
alternative, or change PQLC. Later attempts still use the frozen E17
transient-history continuation surface and still require a preceding positive
completion.

## Unchanged stop boundary

After this correction, any P1 or P2 failure is a stopped E20 negative. No
further fixture correction or candidate repair is authorized.
