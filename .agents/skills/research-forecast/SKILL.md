---
name: research-forecast
description: Synthesize established lessons into the strongest justified candidate, then forecast upward capability, failure-localization, and downward-ablation ladders. Use when asking what mechanisms will probably be needed, whether accumulated lessons are jointly sufficient, what later failures can already be anticipated, or how to avoid rediscovering requirements one experiment at a time.
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
Compose every relevant,
compatible established lesson
        |
        v
Test the complete candidate
up the capability ladder
        |
        +-- fails --> descend to first
        |             broken transition
        |
        `-- works --> ablate downward
                      toward necessity
```

## Rules

- Read `research/constitution.md`, the applicable program claims and lessons, and relevant convergence records before forecasting.
- Classify prior results as applicable established mechanisms, inherited constraints, incompatible alternatives, or falsified proposals.
- Compose every relevant, compatible established mechanism into the strongest justified candidate; do not withhold a known prerequisite merely to rediscover it one rung at a time.
- Describe dependencies as observable physical transitions, not capability labels or benchmark vocabulary.
- For each dependency, state its role, prerequisites, evidence, status, and killing falsifier.
- Label status as `established`, `constrained`, `predicted`, `unknown`, or `retired`; never present a prediction as an experimental result.
- Carry forward negative evidence as design constraints, especially stale credit, broad coactivation, interference, replay failure, and failure to quiesce.
- Prefer compositions of surviving local laws; reject semantic identities, correct-answer knowledge, evaluator intelligence, and hard-coded body structure unless explicitly under test.
- Separate necessity, sufficiency, and adoption. A plausible dependency is not yet necessary; a surviving stack is not yet sufficient or authoritative.
- Freeze the synthesized candidate and test it upward from primitive physical continuity through composed capability and the desired end-to-end behavior.
- If the candidate fails, descend through its trace to the earliest broken physical transition. Do not perform removal ablations or infer necessity from a candidate that never worked.
- If the candidate works, freeze it as the positive reference and forecast a downward ablation ladder: remove one mechanism at a time, restore the reference between removals, then test evidence-motivated coalitions and interactions.
- Require every ablation to rerun the complete upward ladder; a removal survives only if capability, controls, replay, natural quiescence, and cost bounds remain intact.
- Preregister a killing test for every predicted transition and ablation in the same forecast.
- Batch the complete candidate, upward rungs, diagnostic fixtures, and conditional ablations in one campaign. Supply prerequisites through neutral fixtures only for diagnosis, never as a substitute for end-to-end sufficiency.
- Stop dependent upward stages after a prerequisite fails; still run an isolated later diagnostic when its supplied prerequisites do not encode the result being measured. Run ablations only after the complete candidate passes.
- Distinguish failure of current physics from evidence that a predicted dependency is necessary: the former falsifies current sufficiency, not alternatives never tested.
- Choose the earliest uncertain transition only after the batch converges; do not add speculative physics until the synthesized established stack has exposed a genuine discontinuity.
- Hand experiment authoring to `$research-campaign`, durable claim changes to `$research-program`, and completed-round synthesis to `$research-converge`.

## Output

Return a compact synthesis forecast containing:

- the complete candidate's included mechanisms, constraints, exclusions, and evidence;
- an upward ladder of physical transitions and composed capabilities;
- a failure descent to the first observable discontinuity;
- a conditional downward ablation ladder for necessity and redundancy;
- the strongest inherited counterexample and killing falsifier for every uncertain transition or removal;
- batch fixtures, controls, and neutrally supplied diagnostic prerequisites;
- the post-convergence frontier.

Stop before implementation, campaign execution, or authority promotion.
