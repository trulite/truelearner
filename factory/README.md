# Factory as code

```text
request -> validated plan -> exact candidate -> independent verification
```

The factory owns engineering mechanics. It may consume an immutable external
protocol by path and digest, but it never changes research claims or authority.

Its agent procedures live in [rust-plan](../.agents/skills/rust-plan/),
[rust-implement](../.agents/skills/rust-implement/), and
[rust-verify](../.agents/skills/rust-verify/).

## Artifacts

- `templates/plan.md`: readable decision contract.
- `templates/candidate-receipt.json`: plan-to-candidate lineage and executed checks.
- `templates/verification-receipt.json`: independent verdict tied to the exact candidate.

The representative warm regression suite is a hard development-loop gate and
must complete in strictly less than 10 seconds. Cold bootstrap time is reported
separately and does not replace the warm-loop measurement.

## Commands

```bash
uv run factory/validators/validate_plan.py --file <plan.md>
uv run factory/runners/run_candidate_checks.py \
  --plan <plan.md> --output <candidate.json> --cwd <rust-repository> \
  --scope <path> \
  --check 'fmt=cargo fmt --all -- --check' \
  --check 'check=cargo check --workspace --locked' \
  --check 'clippy=cargo clippy --workspace --all-targets --locked -- -D warnings' \
  --check 'focused-tests=cargo test -p <crate> <focused-test>' \
  --check 'regression-suite=cargo test --workspace --locked'
uv run factory/validators/validate_candidate.py --file <candidate.json>
uv run factory/validators/validate_verification.py --file <verification.json>
```

Run the contract suite with:

```bash
uv run -m unittest discover -s factory/tests -v
```
