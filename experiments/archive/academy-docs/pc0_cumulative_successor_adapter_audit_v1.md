# PC0 cumulative successor-adapter audit v1

Status: frozen before cumulative execution.

Parent candidate: `pc0-pressure-closure-compatibility-focused-positive-v1`.

## Purpose

PD2 deleted `eligible_until` from the active substrate. The frozen PD1 and CPC0
evaluators therefore cannot compile unchanged because they observe the removed
deadline field/event. PC0 uses two successor evaluators that preserve their
world construction, schedules, identities, pressure phases, thresholds,
mechanical configurations, replay checks, causal assertions, and output
comparisons.

## PD1 successor delta

The PD1 adapter removes only the deleted `eligible_until` state, reads, CSV
component, and assertions. One additional pre-evidence assertion correction is
required by PC0's already-frozen candidate law:

```text
PD1 sacrificial participation: pressure -> participation == 0
PC0 physical condition:        pressure -> participation remains > 0
```

This is not a causal-outcome repair. It is the exact physical distinction PC0
preregistered and P0-P5 already established. Every PD1 world and every retained
outcome remains unchanged: unused routes die; active unsupported routes receive
no durable learning and die after activity stops; graded participation produces
graded pressure; timely and maintained consequence strengthens; late
unmaintained and wrong-path cases do not; time partitioning remains exact.

## CPC0 successor delta

The CPC0 adapter removes only the deleted `Eligible` event count and CSV column.
It also exhaustively accepts the later `QualifiedLocalTraversal` trace variant
without using it in CPC0's older measurements. All eleven spatial-attribution
scenarios and all expected update outcomes remain byte-for-byte identical in
source.

## Prohibitions

Neither adapter changes active organism code or adds eligibility, a timer, a
pressure exception, event reordering, route identity, closure awareness, or an
ARC fact. Any failure in these successor worlds is a cumulative PC0 failure.
