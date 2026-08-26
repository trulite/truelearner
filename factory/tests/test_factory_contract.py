"""Contract tests for the factory-as-code stages."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest


ROOT = Path(__file__).parents[2]
VALIDATORS = ROOT / "factory" / "validators"
SKILLS = ROOT / ".agents" / "skills"
sys.path.insert(0, str(VALIDATORS))


def load(name: str):
    spec = importlib.util.spec_from_file_location(name, VALIDATORS / f"{name}.py")
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


VALID_PLAN = """# Bounded change

## Outcome
Produce the observable bounded behavior without advancing unrelated claims.

## Authority
- Path: `arch.md`
- Revision: `0123456789abcdef`

## Model
Input becomes validated state through total transformations with typed failure.

## Invariants
Reference behavior and evaluator isolation remain exact.

## Scope
Change `crates/core/src/lib.rs`; exclude persistence and deployment.

## Development style
Use TDD because the failure is observable through a focused boundary test.

## Focused tests
Run `cargo test -p core focused_case` and `cargo clippy -p core -- -D warnings`.

## Development loop
Run the representative warm regression suite with `cargo test -p core`; it must
finish in under 10 seconds. Record cold bootstrap separately.

## Controls and evidence
Use one held-out identity permutation and one unchanged negative control.

## Risks and rollback
Revert the bounded change if replay or quiescence diverges.

## Open decisions
None.
"""


class FactoryContractTest(unittest.TestCase):
    def test_skills_call_their_validators(self):
        expected = {
            "rust-plan": "validate_plan.py",
            "rust-implement": "validate_candidate.py",
            "rust-verify": "validate_verification.py",
        }
        for skill, validator in expected.items():
            with self.subTest(skill=skill):
                text = (SKILLS / skill / "SKILL.md").read_text(encoding="utf-8")
                self.assertIn(f"name: {skill}", text)
                self.assertIn(validator, text)
                self.assertTrue((SKILLS / skill / "agents" / "openai.yaml").is_file())

    def test_factory_skills_do_not_control_research(self):
        for skill in ("rust-plan", "rust-implement", "rust-verify"):
            text = (SKILLS / skill / "SKILL.md").read_text(encoding="utf-8")
            self.assertNotIn("$research-", text)
            self.assertNotIn("research/claims", text)

    def test_every_rust_stage_keeps_the_loop_under_ten_seconds(self):
        for skill in ("rust-plan", "rust-implement", "rust-verify"):
            text = (SKILLS / skill / "SKILL.md").read_text(encoding="utf-8")
            self.assertIn("under 10 seconds", text)

    def test_plan_validator_accepts_complete_plan(self):
        validator = load("validate_plan")
        self.assertEqual([], validator.validate(VALID_PLAN))

    def test_plan_template_requires_completion(self):
        validator = load("validate_plan")
        template = (ROOT / "factory" / "templates" / "plan.md").read_text(encoding="utf-8")
        self.assertTrue(validator.validate(template))

    def test_candidate_and_verification_lineage(self):
        candidate_validator = load("validate_candidate")
        verification_validator = load("validate_verification")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            plan = directory / "plan.md"
            plan.write_text(VALID_PLAN, encoding="utf-8")
            candidate = directory / "candidate.json"
            candidate_value = {
                "schema": "rust-candidate/v1",
                "plan": {"path": str(plan), "sha256": hashlib.sha256(plan.read_bytes()).hexdigest()},
                "candidate": {"revision": "0123456789abcdef", "tree_sha256": "a" * 64},
                "scope": ["crates/core"],
                "changed_paths": ["crates/core/src/lib.rs"],
                "checks": [
                    {
                        "name": name,
                        "command": f"run {name}",
                        "status": "passed",
                        "exit_code": 0,
                        "duration_seconds": 9.0 if name == "regression-suite" else 0.1,
                    }
                    for name in ("fmt", "check", "clippy", "focused-tests", "regression-suite")
                ],
            }
            candidate.write_text(json.dumps(candidate_value), encoding="utf-8")
            self.assertEqual([], candidate_validator.validate(candidate))

            verification = directory / "verification.json"
            verification.write_text(
                json.dumps(
                    {
                        "schema": "rust-verification/v1",
                        "candidate_receipt": {
                            "path": str(candidate),
                            "sha256": hashlib.sha256(candidate.read_bytes()).hexdigest(),
                        },
                        "verdict": "supported",
                        "independent_test_count": 1,
                        "checks": [
                            {
                                "name": "held-out",
                                "command": "run held-out",
                                "status": "passed",
                                "exit_code": 0,
                                "duration_seconds": 0.2,
                            },
                            {
                                "name": "regression-suite",
                                "command": "run regression-suite",
                                "status": "passed",
                                "exit_code": 0,
                                "duration_seconds": 8.5,
                            },
                        ],
                        "findings": ["No material finding."],
                        "residual_uncertainty": ["None within the declared boundary."],
                        "artifacts": [{"path": "held-out.json", "sha256": "b" * 64}],
                    }
                ),
                encoding="utf-8",
            )
            self.assertEqual([], verification_validator.validate(verification))

            candidate_value["checks"][-1]["duration_seconds"] = 10.0
            candidate.write_text(json.dumps(candidate_value), encoding="utf-8")
            errors = candidate_validator.validate(candidate)
            self.assertIn("regression-suite must complete in under 10 seconds", errors)

    def test_plan_requires_under_ten_second_regression_loop(self):
        validator = load("validate_plan")
        slow = VALID_PLAN.replace("under 10 seconds", "under 12 seconds")
        self.assertIn("Development loop must set a strict under-10-second budget", validator.validate(slow))


if __name__ == "__main__":
    unittest.main()
