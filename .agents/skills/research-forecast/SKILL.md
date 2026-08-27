---
name: research-forecast
description: Predict an evidence-ranked dependency ladder for a future capability from prior experiments, lessons, failures, and established mechanisms. Use when asking what mechanisms will probably be needed, whether later failures can already be anticipated, or how to avoid rediscovering requirements one experiment at a time.
---

# Research Forecast

```text
Desired capability
        |
        v
Read claims, lessons,
convergences, evidence
        |
        v
Extract physical transitions
and inherited constraints
        |
        v
Build an evidence-ranked
dependency ladder
        |
        v
Find earliest uncertainty
and its killing test
        |
        v
Hand one bounded frontier
to $research-campaign
```

## Rules

- Read `research/constitution.md`, the applicable program claims and lessons, and relevant convergence records before forecasting.
- Describe dependencies as observable physical transitions, not capability labels or benchmark vocabulary.
- For each dependency, state its role, prerequisites, evidence, status, and killing falsifier.
- Label status as `established`, `constrained`, `predicted`, `unknown`, or `retired`; never present a prediction as an experimental result.
- Carry forward negative evidence as design constraints, especially stale credit, broad coactivation, interference, replay failure, and failure to quiesce.
- Prefer compositions of surviving local laws; reject semantic identities, correct-answer knowledge, evaluator intelligence, and hard-coded body structure unless explicitly under test.
- Separate necessity, sufficiency, and adoption. A plausible dependency is not yet necessary; a surviving stack is not yet sufficient or authoritative.
- Order the ladder by dependency and stop later stages when an earlier prerequisite fails.
- Choose the earliest uncertain transition as the next frontier, with one prediction and one decisive falsifier.
- Hand experiment authoring to `$research-campaign`, durable claim changes to `$research-program`, and completed-round synthesis to `$research-converge`.

## Output

Return a compact dependency ladder containing:

- dependency and physical transition;
- prerequisites;
- evidence and status;
- strongest inherited counterexample;
- killing falsifier;
- immediate frontier and deferred stages.

Stop before implementation, campaign execution, or authority promotion.
