---
name: embodiment
description: Inspect, design, attach, test, or review TrueLearner sensors, actuators, transduction, proprioception, contact, coordinate frames, physical feedback, and body-world morphology. Use before changing how a world signal enters the body, how motor output affects the world, or how an effect returns to the body.
---

# Embodiment

```text
inspect -> draw existing body -> propose fitted change -> STOP for approval
                                                        |
                                                 approved only
                                                        v
                                      failing unit test -> implementation
                                                        |
                                                        v
                                            closed loop -> Academy
```

## Inspect without editing

Read the current world, sensor, body, actuator, checkpoint, and relevant test
code. Do not edit files, add tests, tune constants, or create temporary types.

Draw one ASCII diagram of the existing physical system:

```text
world quantity -> transducer -> receptor junction -> local body tissue
      ^                                                   |
      |                                                   v
returned sensation <- world effect <- effector <- motor junction
```

Label the actual types, coordinates, ranges, resolution, neutral values,
sampling times, signal lifetimes, junctions, muscles, limits, and feedback
paths. Mark the first missing or changed physical arrow.

## Propose the fitted change

Draw a second ASCII diagram. Mark additions with `[+]`, changes with `[~]`, and
removals with `[-]`. Show how the change fits the existing morphology.

State:

- the physical fact and its owner;
- existing structures and arrows reused;
- every added, changed, or removed type, field, constant, junction, and link;
- coordinate frame, units, range, precision, baseline, saturation, latency,
  duration, and expiry;
- the actuator effect and the sensor that observes its return;
- the predicted first changed arrow and killing falsifier;
- unit, closed-loop, replay, firewall, and Academy tests;
- expected resident-memory and representative warm-wave cost;
- smaller alternatives considered and why they lose a required physical fact.

Preserve these boundaries:

- The world owns pixels, surfaces, resistance, and consequences.
- Morphology owns transduction, muscles, joints, skin, and proprioception.
- The learner receives only junction activity meeting in physical time.
- Academy observes and judges without entering the organism.
- Signed quantities use opposing physical channels.
- Independent degrees of freedom remain independent products.
- Spatial relations live in topology and coordinate transforms, not task names.

Stop and request explicit user approval. A request to diagnose, fix, continue,
or investigate is not approval of the proposed morphology.

## Implement only after approval

1. Add the smallest killing unit test and confirm the predicted failure.
2. Implement the approved physical delta without expanding it.
3. Test neutral identity, discrimination, locality, independent products,
   symmetry, reachability, timing, and the complete returned-feedback loop.
4. Prove traced and untraced execution agree and checkpoint replay is exact.
5. Verify serialized organism input contains no task, action, or evaluator data.
6. Use `$dev` for structure/law documentation, clippy, regressions, and the
   strict warm-wave limit.
7. Use `$academy` only after the physical unit and closed-loop tests pass.
8. Commit only after the approved tests and cost boundary pass.

Use `$causal-debug` when a trace is needed to locate the first failed arrow.
If evidence requires a materially different morphology, return to the proposal
stage and stop for approval again.

## Report

State what physically happened, the first failed or repaired arrow, the exact
concept delta, the tests, the measured cost, and the remaining Academy gate.
