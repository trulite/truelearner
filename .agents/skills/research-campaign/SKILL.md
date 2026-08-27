---
name: research-campaign
description: Run a synthesis-first research campaign that tests a complete evidence-backed candidate upward, localizes its first failure, or ablates a successful candidate downward. Use for capability climbs, diagnostics, competing mechanisms, controlled removals, rapid experiments, or large hypothesis searches.
---

# Research Campaign

```text
Admissible question
        |
        v
Build complete candidate from
applicable established lessons
        |
        v
Run controls and climb the
upward capability ladder
        |
        +-- fails --> localize first break
        |             and test minimal solves
        |
        `-- works --> remove downward and
                      rerun the full ladder
```

## Rules

- Read `research/constitution.md` and applicable program lessons before authoring arms.
- Manifest the complete candidate by listing every included established mechanism, inherited constraint, and excluded incompatible or falsified proposal.
- Make that complete candidate the frozen positive reference. Do not begin by withholding established prerequisites.
- Test it upward from primitive transitions to the end-to-end capability, with cheap gates before expensive rungs.
- Treat a high-level failure label as a symptom; locate the earliest physical divergence between positive and negative cases.
- If the complete candidate fails, stop dependent upward rungs, parallelize diagnostic arms, and then test competing minimal solves at the first divergence. Do not run removal ablations.
- If the complete candidate succeeds, run downward ablations against the frozen reference: remove one mechanism, rerun the whole upward ladder, restore the reference, and then test justified mechanism coalitions.
- State each arm's prediction and killing falsifier before execution.
- Add composition arms when proposed mechanisms may be jointly necessary.
- Keep the positive reference and unchanged negative controls frozen.
- Apply cheap fixtures, leakage audit, replay equality, and natural quiescence before expensive evidence runs.
- Stop falsified arms immediately and retain their counterexamples.
- Use reusable E2B workers only for discovery; use a fresh one-shot sandbox for frozen authority evidence.
- Write campaign and arm manifests from `research/templates/`, then run `validate_campaign.py` with `uv run`.
- Use `uv run research/runtime/dispatch_e2b.py` only through a separate E2B batch manifest; do not depend on a factory workflow.
