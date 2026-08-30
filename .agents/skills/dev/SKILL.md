---
name: dev
description: Develop, debug, review, or change TrueLearner code. Apply category-theory and TAME lenses, keep the implementation lagom, preserve black-box behavior, and keep representative warm wave time strictly under 25 ns unless the user expressly approves otherwise.
---

# Development

```text
problem -> compositional physical model -> smallest complete change -> verify
               |                                  |                 |
        category theory + TAME              simple code       warm wave < 25 ns
                                                                    |
                                                    slower -> ask user to approve
```

## Model

- Apply a category-theory lens: identify objects, arrows, identity, composition,
  products, and ownership boundaries. Make independent parts compose without
  hidden coupling. Keep these ideas in the modeling; use plain domain names in
  code.
- Apply the TAME lens: explain behavior through local physical incidence,
  retained history, and composable competent parts. Never admit evaluator,
  benchmark, answer, or semantic knowledge into the organism.
- Prefer the smallest complete change that preserves existing boundaries and
  black-box behavior.

## Verify

- Add the smallest law or black-box scenario that fails before the change and
  passes after it.
- Run focused tests, then the affected regression suite.
- Measure the representative warm wave before and after the change. Exclude
  build, setup, rendering, serialization, and cold bootstrap unless they are the
  behavior being changed.
- Require wave time to remain strictly below 25 ns. If it reaches 25 ns or more,
  optimize it or stop and obtain the user's express approval before accepting
  the change.
- Report behavior, tests, and wave time in simple English.
