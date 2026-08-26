---
name: research-program
description: Create or maintain a research program as versioned claims, dependencies, lessons, protocols, evidence, and authority transitions. Use when defining a large hypothesis, choosing the active research frontier, recording durable lessons, or deciding what research becomes admissible next.
---

# Research Program

```text
Large question
     |
     v
Read constitution,
claims, lessons,
and authority
     |
     v
Build dependency graph
and active frontier
     |
     v
Expose admissible
campaigns
     |
     v
Validate program
     |
     v
Stop before execution
```

## Rules

- Treat `research/constitution.md` as the stable research-method boundary.
- Read `research/programs/learner/lessons.toml` for learner research; keep domain lessons out of generic runtime logic.
- Represent bold hypotheses as claims with explicit dependencies, limitations, and falsifiers.
- Preserve exact parent authority and a clean pre-frontier checkpoint.
- Keep negative results and failed mechanisms addressable; they constrain future work.
- Distinguish proposed, preregistered, executed, positive, negative, inconclusive, audited, and authoritative states.
- Make discovery campaigns admissible without prescribing their implementation process.
- Do not invoke implementation or factory skills; publish neutral protocols and import neutral evidence.
- Validate program manifests with `uv run research/validators/validate_program.py --file <program>`.
- Stop before campaign execution or authority promotion.
