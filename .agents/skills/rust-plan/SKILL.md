---
name: rust-plan
description: Plan or refine a non-trivial Rust change using repository evidence, explicit invariants, categorical modeling, concise skill-author document conventions, and a validated verification strategy. Use before changing architecture, behavior, state models, learning mechanisms, experiment code, persistence, or runtime semantics.
---

# Rust Planning

```text
Change request
      |
      v
Read authority, code,
tests, and evidence
      |
      v
Resolve material questions
      |
      v
Model states, arrows,
composition, and laws
with $categorical-rust
      |
      v
Apply plan-relevant
$skill-author conventions
      |
      v
Write factory plan
      |
      v
Validate plan
      |
      v
Stop before implementation
```

## Rules

- Ground the plan in authoritative architecture, current code, tests, and reproducibility evidence.
- Separate the intended claim or behavior from the mechanism proposed to produce it.
- Ask at most three grounded questions per round; ask only when the answer changes behavior, invariants, evidence, scope, or risk.
- Identify types, transformations, failure paths, effects, composition boundaries, and ownership boundaries.
- Preserve the boundary between system inputs and evaluator-only knowledge.
- Define focused tests, held-out cases, negative controls, and falsifiers when applicable.
- Choose TDD or implementation-first explicitly and name exact verification commands.
- Define the representative warm regression command and keep its development loop strictly under 10 seconds.
- Follow the plan-relevant conventions from `$skill-author`: lead with one ASCII text diagram, stay lagom, remove history, duplication, and obvious advice, and keep ownership boundaries and stop conditions explicit. Do not copy skill-only metadata steps into a plan.
- Write the plan from `factory/templates/plan.md`.
- Run `uv run factory/validators/validate_plan.py --file <plan>` and resolve every error.
- Stop before implementation.
