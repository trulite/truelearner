---
name: research-converge
description: Converge a completed research round by preserving failed arms, comparing surviving mechanisms, extracting counterexamples, cross-pollinating compatible ideas, and defining the next discriminating arms. Use after parallel experiment batches or whenever a research program is branching without synthesis.
---

# Research Convergence

```text
Arm evidence
     |
     v
Account for every arm
     |
     v
Freeze failures and
extract counterexamples
     |
     v
Compare surviving
mechanisms
     |
     v
Retire, discriminate,
or cross-pollinate
     |
     v
Write next frontier
and validate lineage
```

## Rules

- Account for every launched arm as survived, falsified, inconclusive, or infrastructure-failed.
- Preserve failed mechanisms unchanged and carry their strongest counterexamples forward.
- Rank survivors by falsifier survival, explanatory simplicity, transfer, reproducibility, distinctness, and next-test value—not by a raw leaderboard alone.
- Cross-pollinate ideas through a new arm with explicit parents, imported mechanisms, interaction prediction, and new falsifiers.
- Distinguish a mechanism's sufficiency from its adoption.
- Prefer the next experiment that discriminates surviving explanations.
- Force fan-in at every declared convergence boundary; do not permit unbounded branch growth.
- Write `research/templates/convergence.toml` and validate it with `uv run research/validators/validate_convergence.py`.
- Do not promote authority or mutate a frozen result.
