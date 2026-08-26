# RS2 learned inhibitory topology immutable negative v5

Status: complete frozen negative. CE1, FD2 v2, and frozen ARC A2 did not run.

Protocol: `f7dcacf` (`rs2-learned-inhibitory-topology-protocol-v5`).

Frozen evaluator: `aeeb73f`
(`rs2-learned-inhibitory-topology-frozen-v5`).

One-shot E2B worker: `i7or19tv7bafxxa5nfa5t`.

## Exact stop

The complete RS2 v5 command executed exactly once. It stopped before
publishing a matrix row or result artifact at the AH0/SI0 runtime assertion:

```text
assertion `left == right` failed: SI0 defines Drive incidence only
left: Modulatory
right: Drive
```

The assertion is in the SI0 outgoing-transmission path of the canonical AH0
runtime. RS2's ordinary training consequence fires a modulator CELL whose
ordinary outgoing ARROW has `TransmissionMode::Modulatory`. SI0 currently
requires every outgoing transmission produced during its incidence/firing
wave execution to be Drive, so the retained Modulatory law and the retained
simultaneous-incidence law cannot yet execute cumulatively.

## Classification

This is not an RS2 sign-selection, identity-permutation, insertion-order,
checkpoint, continuation, recurrence, replay, or quiescence result. None of
those predicates was reached.

It is a cumulative substrate-integration negative:

> AH0/SI0 defines simultaneous local Drive incidence, but its firing path does
> not yet preserve ordinary Modulatory outgoing transmission.

SI0 explicitly excluded Drive/Modulatory simultaneity when it was established.
RS2 v5 is the first retained gate in this lineage that requires both laws in
one execution. V5 therefore stops without a repair or rerun. No scientific
claim about learned inhibitory topology is made.

## Frozen surface

- canonical AH0 runtime SHA-256:
  `9ec9f4fb5ae9c66b8353ef17307715782f0fee2b044928809cc7f24a9fd041db`;
- evaluator SHA-256:
  `2201581143662e0eb6ebd5be3e0b18b1fb250a71613a6189fd08a3d1f0df00a4`;
- evaluator manifest SHA-256:
  `3b09fceafb20f0052fedf74dc3585b6a2dcaad8a615918fdc2d50c5b58ce7b16`;
- protocol SHA-256:
  `cfca15cfb86aef35c6552b68a8351cbe8b2c208e1061c32d9ce4a35046d7397c`.

No organism source changed in RS2 v5. No output matrix/report exists because
the runtime stopped before serialization. Authority, oracle status, and
`arch.md` remain unchanged.
