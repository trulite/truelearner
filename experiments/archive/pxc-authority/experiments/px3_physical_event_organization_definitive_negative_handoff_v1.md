# PX3 physical event organization definitive negative handoff v1

Status: **PX3 NON-AUTHORITATIVE; PX4 BLOCKED**.

## Frozen boundary

The first PX3 definitive matrix is an immutable negative: `48/64` cells and
`592/640` claims passed. All failures are the same causal counterexample.

```text
AB episode 1
  -> candidate traverses
  -> eligibility live through tick 5
  -> no downstream return

AB episode 2 reaches P at tick 5
  -> generic PX0 local-return rule strengthens eligible candidate
  -> coupling becomes 2
  -> held-out AB executes X
```

Thus repeated upstream participation inside the eligibility window can
manufacture reusable organization without a completed downstream return.

## Preserved results

PX3 development remains valuable but insufficient for authority:

- distinct physical participation and amplitude normalization work;
- participation gates candidate eligibility;
- trace-attributed unit return credits without refiring;
- MICRO reversal and deallocation work under completed-loop schedules;
- recursive compression works and derived participants emit one ordinary
  trace;
- all definitive full-recursion, return-only, same-path, gap, replay and
  quiescence controls passed.

The missing invariant is narrower:

> Later upstream activity must not count as downstream evidence merely because
> a prior candidate remains eligible at the same source.

## Reopening boundary

No authority handoff may be written from this lineage. PX4 and later physical
stages remain blocked.

A future development successor, if explicitly opened, must use fresh commands,
artifact paths and non-definitive evidence. Its first diagnostic must isolate
the tick-5 O->P arrival that changed resistance `1 -> 4` with M firing and
M->P credit both zero. It must not tune the spent definitive schedule, weaken
the downstream-return necessity claim, or rerun any definitive cell.
