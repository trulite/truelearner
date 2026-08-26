---
name: rust-verify
description: Independently verify a Rust candidate against its validated plan, invariants, checks, controls, and reproducibility requirements. Use after implementation or when reviewing experimental, architectural, or behavioral Rust changes.
---

# Rust Verification

```text
Plan + candidate receipt
          |
          v
Recover claims, invariants,
controls, and expected evidence
          |
          v
Inspect candidate and diff
          |
          v
Run independent checks,
held-out cases, and controls
          |
          v
Write verification receipt
          |
          v
Validate exact candidate
and report verdict
```

## Rules

- Validate both the plan and candidate receipt before verification.
- Judge the candidate against the plan and authoritative architecture, not against its own implementation.
- Check that inputs, ground truth, and evaluator-only information remain separated.
- Prefer independent tests that do not reproduce the implementation logic.
- Exercise held-out cases, negative controls, ambiguity, lifetime, and failure paths where relevant.
- Check identity, associativity, preservation, and composition laws when claimed.
- Preserve failing evidence; never weaken, skip, retag, or reinterpret it.
- Independently time the representative warm regression suite and require a duration strictly under 10 seconds.
- Write a receipt from `factory/templates/verification-receipt.json` with the exact candidate-receipt digest.
- Run `uv run factory/validators/validate_verification.py --file <receipt>`.
- Report exact commands, results, findings, and residual uncertainty.
- Do not fix production code during verification unless explicitly requested.
