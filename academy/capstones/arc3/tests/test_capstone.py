from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import capstone


class FakeAgent:
    def __init__(self, responses: list[dict[str, object]] | None = None) -> None:
        self.requests: list[dict[str, object]] = []
        self.responses = list(responses or [])
        self.ready = {"response": "ready", "body_fingerprint": "initial"}

    def command(self, request: dict[str, object]) -> dict[str, object]:
        self.requests.append(request)
        if self.responses:
            return self.responses.pop(0)
        return {
            "response": "observation",
            "call": None,
            "outward_crossings": 0,
            "plasticity_updates": 0,
            "physical_work": 1,
            "naturally_quiescent": True,
            "body_fingerprint": "final",
        }

    def close(self) -> None:
        pass


def observation(**overrides: object) -> SimpleNamespace:
    values: dict[str, object] = {
        "frame": [[[0 for _ in range(64)] for _ in range(64)]],
        "available_actions": [4, 2, 1, 3],
        "game_id": "secret-game",
        "state": SimpleNamespace(value="NOT_FINISHED"),
        "levels_completed": 99,
        "score": 0.75,
    }
    values.update(overrides)
    return SimpleNamespace(**values)


class ProjectionTests(unittest.TestCase):
    def test_protocol_freezes_workstation2_negative_control(self) -> None:
        protocol = capstone.load_protocol(
            Path(__file__).resolve().parents[1] / "protocol.toml"
        )
        self.assertEqual(protocol["course_frontier"], "Drag")
        self.assertEqual(protocol["workstation_course_steps"], 256)
        self.assertEqual(protocol["workstation_course_seed"], 11)
        self.assertEqual(protocol["unmapped_actions"], [])
        self.assertEqual(
            protocol["diagnostic_classification"], "plumbing-negative-control"
        )

    def test_evaluator_fields_cannot_reach_application_request(self) -> None:
        left = capstone.project_observation(observation(game_id="one", score=0.0))
        right = capstone.project_observation(observation(game_id="two", score=1.0))
        self.assertEqual(left, right)
        self.assertEqual(set(left), {"command", "frame", "actions"})
        self.assertEqual([offer["id"] for offer in left["actions"]["offers"]], [1, 2, 3, 4])

    def test_coordinate_action_exposes_only_public_shape(self) -> None:
        request = capstone.project_observation(observation(available_actions=[6]))
        self.assertEqual(
            request["actions"],
            {"offers": [{"id": 6, "schema": {"type": "point", "width": 64, "height": 64}}]},
        )

    def test_invalid_palette_is_rejected(self) -> None:
        bad = observation(frame=[[[16 for _ in range(64)] for _ in range(64)]])
        with self.assertRaisesRegex(capstone.CapstoneError, "palette"):
            capstone.project_observation(bad)

    def test_duplicate_action_is_rejected(self) -> None:
        with self.assertRaisesRegex(capstone.CapstoneError, "duplicated"):
            capstone.project_observation(observation(available_actions=[1, 1]))


class ExecutionTests(unittest.TestCase):
    def test_game_stops_on_honest_silence(self) -> None:
        agent = FakeAgent()
        summary, transcript = capstone.run_game(
            "fixture", capstone.FixtureWorld(), agent, 4
        )
        self.assertEqual(summary["stop_reason"], "no_device_input")
        self.assertEqual(len(transcript), 1)
        self.assertEqual(set(agent.requests[0]), {"command", "frame", "actions"})
        self.assertNotIn("state", agent.requests[0])

    def test_replay_divergence_fails_closed(self) -> None:
        records = [
            {
                "turn": 0,
                "request": {"command": "observe", "frame": [0] * 4096, "actions": {"offers": [{"id": 1, "schema": {"type": "unit"}}]}},
                "response": {"response": "observation", "call": None},
            }
        ]
        agent = FakeAgent([{"response": "observation", "call": {"id": 1}}])
        with self.assertRaisesRegex(capstone.CapstoneError, "replay diverged"):
            capstone.replay_transcript(records, lambda: agent)

    def test_official_world_executes_typed_calls(self) -> None:
        class Environment:
            def __init__(self) -> None:
                self.calls: list[tuple[object, dict[str, int]]] = []

            def step(self, action: object, data: dict[str, int]) -> object:
                self.calls.append((action, data))
                return observation()

        environment = Environment()
        world = capstone.OfficialWorld(environment)
        world.step({"id": 5, "arguments": {"type": "unit"}})
        world.step({"id": 6, "arguments": {"type": "point", "x": 12, "y": 34}})
        self.assertEqual(environment.calls[0][1], {})
        self.assertEqual(environment.calls[1][1], {"x": 12, "y": 34})


class EvidenceTests(unittest.TestCase):
    def test_dirty_source_guard(self) -> None:
        def dirty(_command: list[str], _cwd: Path) -> str:
            return " M academy.md\n"

        with self.assertRaisesRegex(capstone.CapstoneError, "clean committed source"):
            capstone.require_clean_source(Path("."), dirty)

    def test_atomic_evidence_and_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "receipt"
            result = capstone.write_evidence(
                output,
                {"schema_version": 1, "verdict": "plumbing-negative-control"},
                [{"request": {"frame": [0]}, "response": {"call": None}}],
            )
            self.assertTrue((output / result["transcript_file"]).is_file())
            capstone.verify_evidence(output)
            (output / result["transcript_file"]).write_text("tampered\n")
            with self.assertRaisesRegex(capstone.CapstoneError, "transcript digest"):
                capstone.verify_evidence(output)


if __name__ == "__main__":
    unittest.main()
