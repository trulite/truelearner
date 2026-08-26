---
name: research-campaign
description: Turn an admissible research question into a parallel falsification-first campaign with isolated arms, composition arms, controls, budgets, convergence gates, and optional E2B execution batches. Use for diagnostics, competing mechanisms, rapid experiments, or large hypothesis searches.
---

# Research Campaign

```text
Admissible question
        |
        v
Locate first missing
physical transition
        |
        v
Fork diagnostic, solve,
control, and composition arms
        |
        v
Run tiny preflight gates
in isolated E2B workers
        |
        v
Falsified? stop + freeze
Survived? full evidence
        |
        v
Hand all outcomes to
$research-converge
```

## Rules

- Read `research/constitution.md` and applicable program lessons before authoring arms.
- Treat a high-level failure label as a symptom; locate the earliest physical divergence between positive and negative cases.
- If the divergence is unknown, parallelize diagnostic arms. Once localized, parallelize competing minimal solves.
- State each arm's prediction and killing falsifier before execution.
- Add composition arms when proposed mechanisms may be jointly necessary.
- Keep the positive reference and unchanged negative controls frozen.
- Apply cheap fixtures, leakage audit, replay equality, and natural quiescence before expensive evidence runs.
- Stop falsified arms immediately and retain their counterexamples.
- Use reusable E2B workers only for discovery; use a fresh one-shot sandbox for frozen authority evidence.
- Write campaign and arm manifests from `research/templates/`, then run `validate_campaign.py` with `uv run`.
- Use `uv run research/runtime/dispatch_e2b.py` only through a separate E2B batch manifest; do not depend on a factory workflow.
