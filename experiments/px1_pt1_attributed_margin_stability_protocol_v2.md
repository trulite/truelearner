# PX1-PT1 attributed-margin stability protocol v2

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; V1 SUPERSEDED BEFORE EXECUTION**.

Protocol v1 was frozen but executed no implementation, world, seed, or result.
Static composition found that branch firing and continuation execution are not
identical: during held-out use, two branches may fire while only one downstream
effect crosses threshold. V2 strengthens the physical participation criterion
without changing the research question or the margin condition.

All frozen inputs, stages, controls, pass rules, stopping rules, and authority
boundaries from v1 remain in force.

## Required target-execution trace

The ordinary global return must not directly select a branch. Instead each
continuation has an identical local coincidence cell:

```text
continuation effect fires
→ one subthreshold local trace impulse

global physical return
→ identical impulse reaches every continuation trace cell

effect trace + in-window global return
→ local trace cell fires
→ ordinary local return reaches that continuation's branch
```

The trace cell uses only ordinary mutable CELL state and its normal temporal
decay. It contains no route identity, cause field, ownership field, or
eligibility metadata. The same global return topology reaches every trace cell.

Consequently:

- a branch may fire and emit a weak continuation without receiving credit if
  its downstream effect does not fire;
- the continuation whose downstream effect actually fired may receive local
  return;
- if two downstream effects fire, both local trace cells may fire;
- if global return arrives after transient trace state decays, neither may
  fire.

## Additional conjunctive controls

PT1 now separately serializes:

1. branch firing for A/B;
2. downstream effect firing for A/B;
3. global return arrival at both trace cells;
4. local trace-cell firing for A/B;
5. local branch-return arrival for A/B;
6. continuation resistance for A/B;
7. held-out outward effects for A/B.

The critical negative control is:

```text
both branches fire
+ only A downstream effect fires
+ identical global return reaches both trace cells
→ only A trace cell fires
→ only A continuation changes
```

No compound predicate may hide which clause failed.

The v1 wording “same return reaches every branch” is superseded by the more
precise physical requirement: the same global return reaches every branch-local
trace cell; only a trace cell containing contemporaneous effect activity may
then propagate ordinary local return to its branch.

No PT1 evidence may execute from protocol v1.

