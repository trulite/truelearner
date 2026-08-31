# TrueLearner closure checker

This pinned Lean project defines the first executable boundary-closure model.
It checks a frozen causal trace after an Academy experience; it is never linked
into the organism and never contributes to physical time.

Build the checker:

```sh
lake build
```

The executable is written to:

```text
.lake/build/bin/truelearner-closure-check
```

It accepts one `truelearner-causal-check/v1` JSON request on standard input and
emits one `truelearner-causal-receipt/v1` JSON receipt on standard output.

The current model proves and checks:

- no causal explanation makes no closure claim;
- one causal explanation closes exactly that historical witness;
- several explanations are ambiguous;
- ambiguous and absent closure persist nothing;
- a unique closure persists exactly the witnessed path support;
- timing alone cannot establish causal ancestry;
- explicit ancestry can order a crossing and return within one physical tick;
- adding tested contexts can only refine contextual equivalence.

This is a formal hypothesis and an evidence checker, not learner authority.
Academy experiments must still establish that the causal edges in a request
correspond to the world.
