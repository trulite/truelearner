---
name: rust-implement
description: Implement a validated Rust plan as the smallest complete, readable change and produce machine-checkable candidate evidence. Use when modifying Rust types, transformations, learning mechanisms, experiments, persistence, or runtime behavior after the intended outcome and verification strategy are settled.
---

# Rust Implementation

```text
Validated plan
      |
      v
Read authority, affected
code, tests, and boundaries
      |
      v
Follow selected test style
      |
      v
Use $categorical-rust
for the model
      |
      v
Implement smallest
complete change
      |
      v
Run declared checks
and write receipt
      |
      v
Hand off to verification
```

## Rules

- Validate the plan before changing code.
- Follow its scope and development style; return material new decisions to planning.
- Preserve architectural authority, experimental controls, and evaluator isolation.
- Use category theory to simplify the model, not to introduce ceremonial traits.
- Prefer explicit types, total functions, typed failures, and pure transformations.
- Keep randomness, I/O, clocks, and mutation at visible boundaries.
- Implement the smallest complete change without unrelated cleanup.
- Never weaken an oracle or patch an unchanged negative control.
- Keep the representative warm regression suite under 10 seconds; treat a slower loop as a failed factory gate.
- Use `factory/runners/run_candidate_checks.py` to execute declared checks and generate the candidate receipt.
- Run `uv run factory/validators/validate_candidate.py --file <receipt>` before handoff.
- Do not commit, publish, or expand scope unless explicitly requested.
