"""Contract tests for research-program-as-code and its loose coupling boundary."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import textwrap
import unittest


ROOT = Path(__file__).parents[2]
VALIDATORS = ROOT / "research" / "validators"
SKILLS = ROOT / ".agents" / "skills"
sys.path.insert(0, str(VALIDATORS))


def load(name: str):
    spec = importlib.util.spec_from_file_location(name, VALIDATORS / f"{name}.py")
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class ResearchContractTest(unittest.TestCase):
    def test_research_skills_are_present_and_do_not_invoke_factory(self):
        expected = {
            "research-program": "validate_program.py",
            "research-campaign": "validate_campaign.py",
            "research-converge": "validate_convergence.py",
            "research-adjudicate": "validate_adjudication.py",
        }
        forbidden = ("$rust-plan", "$rust-implement", "$rust-verify")
        for skill, validator in expected.items():
            with self.subTest(skill=skill):
                text = (SKILLS / skill / "SKILL.md").read_text(encoding="utf-8")
                self.assertIn(f"name: {skill}", text)
                self.assertIn(validator, text)
                self.assertFalse(any(token in text for token in forbidden))
                self.assertTrue((SKILLS / skill / "agents" / "openai.yaml").is_file())

    def test_original_lessons_remain_and_new_lessons_have_unique_ids(self):
        import tomllib

        value = tomllib.loads(
            (ROOT / "research" / "programs" / "learner" / "lessons.toml").read_text(encoding="utf-8")
        )
        lessons = value["lesson"]
        self.assertGreaterEqual(len(lessons), 40)
        self.assertEqual(len(lessons), len({lesson["id"] for lesson in lessons}))
        self.assertEqual({"learner-physics", "research-method"}, {lesson["kind"] for lesson in lessons})

    def test_program_graph_validation(self):
        validator = load("validate_program")
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "program.toml"
            path.write_text(
                textwrap.dedent(
                    """
                    schema = "research-program/v1"
                    id = "core-next"
                    thesis = "Local physical laws can produce the next bounded capability."
                    pre_frontier_checkpoint = "core1-authority"
                    active_frontier = ["C2"]

                    [[claim]]
                    id = "C1"
                    statement = "Participation records a temporary causal path."
                    status = "authoritative"
                    depends_on = []
                    falsifiers = ["No path appears after participation."]
                    limitations = ["Bounded to the accepted fixture family."]

                    [[claim]]
                    id = "C2"
                    statement = "The path supports a new bounded continuation."
                    status = "proposed"
                    depends_on = ["C1"]
                    falsifiers = ["Continuation fails with a live path."]
                    limitations = ["Does not establish unrestricted planning."]
                    """
                ),
                encoding="utf-8",
            )
            self.assertEqual([], validator.validate(path))

    def test_campaign_requires_parallel_arms_and_composition(self):
        validator = load("validate_campaign")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            digest = "a" * 64
            for arm_id, kind in (("A", "solve"), ("B", "solve"), ("AB", "composition")):
                (directory / f"{arm_id}.toml").write_text(
                    textwrap.dedent(
                        f"""
                        schema = "research-arm/v1"
                        id = "{arm_id}"
                        campaign = "campaign-1"
                        kind = "{kind}"
                        parents = []
                        imports = []
                        mechanism = "Bounded mechanism {arm_id}."
                        prediction = "Arm {arm_id} has a distinct observable result."
                        falsifiers = ["The declared result does not occur."]
                        source_revision = "revision-{arm_id}"
                        protocol_sha256 = "{digest}"
                        budget_minutes = 10
                        [gates]
                        tiny_fixture = "tiny-{arm_id}"
                        full_evidence = "full-{arm_id}"
                        """
                    ),
                    encoding="utf-8",
                )
            campaign = directory / "campaign.toml"
            campaign.write_text(
                textwrap.dedent(
                    """
                    schema = "research-campaign/v1"
                    id = "campaign-1"
                    program = "core-next"
                    kind = "solve"
                    mode = "discovery"
                    hypothesis = "Two local mechanisms jointly close the missing transition."
                    first_divergence = "Participation leaves no live return path."
                    missing_transition = "Participation-born temporary topology."
                    prediction = "The composition survives both isolated falsifiers."
                    falsifiers = ["The composition cannot return consequence."]
                    positive_reference = "accepted-positive"
                    negative_controls = ["unchanged-negative"]
                    arm_paths = ["A.toml", "B.toml", "AB.toml"]
                    max_parallel_arms = 3
                    max_rounds = 2
                    interaction_expected = true
                    [budget]
                    total_sandbox_minutes = 60
                    max_minutes_per_arm = 20
                    [preflight]
                    tiny_fixture = true
                    hidden_authority_audit = true
                    reference_replay_equality = true
                    natural_quiescence = true
                    [convergence]
                    after_each_round = true
                    survivor_fraction = 0.5
                    [authority]
                    fresh_sandbox = true
                    max_valid_runs = 1
                    requires_frozen_protocol = true
                    """
                ),
                encoding="utf-8",
            )
            self.assertEqual([], validator.validate(campaign))

    def test_protocol_evidence_and_adjudication_lineage(self):
        protocol_validator = load("validate_protocol")
        evidence_validator = load("validate_evidence")
        adjudication_validator = load("validate_adjudication")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            protocol = directory / "protocol.toml"
            protocol.write_text(
                textwrap.dedent(
                    """
                    schema = "research-protocol/v1"
                    id = "E1"
                    program = "core-next"
                    claim = "C2"
                    status = "preregistered"
                    parent_authority = "core1-authority"
                    question = "Can temporary topology preserve the exact causal participant?"
                    observables = ["participant identity", "consequence return"]
                    frozen_variables = ["accepted positive and negative fixtures"]
                    permitted_changes = ["temporary topology lifetime"]
                    positive_predicates = ["consequence reaches the participant"]
                    negative_controls = ["non-participant receives no consequence"]
                    stop_conditions = ["any hidden authority freezes negative"]
                    [run_policy]
                    max_valid_runs = 1
                    fresh_environment = true
                    """
                ),
                encoding="utf-8",
            )
            self.assertEqual([], protocol_validator.validate(protocol))
            protocol_digest = hashlib.sha256(protocol.read_bytes()).hexdigest()
            evidence = directory / "evidence.json"
            evidence.write_text(
                json.dumps(
                    {
                        "schema": "research-evidence/v1",
                        "experiment": "E1",
                        "protocol_sha256": protocol_digest,
                        "subject_digest": "b" * 64,
                        "producer": "isolated-e2b-runner",
                        "environment": {"identity": "sandbox-1"},
                        "observations": {"consequence reaches the participant": "passed"},
                        "artifacts": [{"path": "matrix.csv", "sha256": "c" * 64}],
                        "completed": True,
                    }
                ),
                encoding="utf-8",
            )
            self.assertEqual([], evidence_validator.validate(evidence))
            adjudication = directory / "adjudication.toml"
            adjudication.write_text(
                textwrap.dedent(
                    f"""
                    schema = "research-adjudication/v1"
                    protocol_path = "{protocol}"
                    protocol_sha256 = "{protocol_digest}"
                    evidence_path = "{evidence}"
                    evidence_sha256 = "{hashlib.sha256(evidence.read_bytes()).hexdigest()}"
                    verdict = "positive"
                    predicate_results = ["consequence return=passed"]
                    control_results = ["non-participant=passed"]
                    scientific_findings = ["The bounded prediction survived."]
                    residual_uncertainty = ["Only the frozen fixture family was tested."]
                    sufficiency = "established"
                    integration = "not-attempted"
                    adoption = "not-adopted"
                    authority = "not-promoted"
                    authorized_by = "none"
                    """
                ),
                encoding="utf-8",
            )
            self.assertEqual([], adjudication_validator.validate(adjudication))

    def test_e2b_batch_adapter_dispatches_isolated_arms(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            adapter = directory / "fake_e2b_adapter.py"
            adapter.write_text(
                textwrap.dedent(
                    """
                    import json
                    from pathlib import Path
                    import sys

                    arm = sys.argv[1]
                    Path("result.json").write_text(json.dumps({
                        "schema": "research-arm-result/v1",
                        "arm": arm,
                        "outcome": "survived" if arm == "A" else "falsified",
                        "falsifier": "none" if arm == "A" else "tiny fixture failed",
                        "observations": {"tiny": "passed" if arm == "A" else "failed"},
                        "artifacts": [],
                    }))
                    """
                ),
                encoding="utf-8",
            )
            arms = []
            for arm_id in ("A", "B"):
                cwd = directory / arm_id
                cwd.mkdir()
                arms.append(
                    textwrap.dedent(
                        f"""
                        [[arm]]
                        id = "{arm_id}"
                        cwd = "{cwd}"
                        command = ["{sys.executable}", "{adapter}", "{arm_id}"]
                        protocol_sha256 = "{'a' * 64}"
                        result_path = "result.json"
                        timeout_seconds = 10
                        """
                    )
                )
            batch = directory / "batch.toml"
            batch.write_text(
                textwrap.dedent(
                    f"""
                    schema = "research-e2b-batch/v1"
                    id = "batch-1"
                    mode = "discovery"
                    max_parallel = 2
                    output_directory = "{directory / 'out'}"
                    """
                )
                + "\n".join(arms),
                encoding="utf-8",
            )
            subprocess.run(
                [sys.executable, str(ROOT / "research" / "runtime" / "dispatch_e2b.py"), "--batch", str(batch)],
                check=True,
                capture_output=True,
                text=True,
            )
            result = json.loads((directory / "out" / "batch-result.json").read_text(encoding="utf-8"))
            self.assertEqual(["A", "B"], [arm["arm"] for arm in result["arms"]])
            self.assertEqual(["survived", "falsified"], [arm["outcome"] for arm in result["arms"]])


if __name__ == "__main__":
    unittest.main()
