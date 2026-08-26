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
        self.ready = {"response": "ready", "sequence": 0, "body_fingerprint": "initial"}

    def command(self, request: dict[str, object]) -> dict[str, object]:
        self.requests.append(request)
        if self.responses:
            return self.responses.pop(0)
        return {
            "response": "observation",
            "sequence": len(self.requests) - 1,
            "action": None,
            "outward_crossings": 0,
            "plasticity_updates": 0,
            "modulatory_deliveries": 0,
            "physical_work": 1,
            "naturally_quiescent": True,
            "body_fingerprint": "final",
            "physical_tick": 1,
            "pressure_phase": 0,
        }

    def close(self) -> None:
        pass


def observation(**overrides: object) -> SimpleNamespace:
    values: dict[str, object] = {
        "frame": [[[0 for _ in range(64)] for _ in range(64)]],
        "available_actions": [1, 2, 3, 4],
        "game_id": "secret-game",
        "state": SimpleNamespace(value="NOT_FINISHED"),
        "levels_completed": 99,
        "win_levels": 100,
        "score": 0.75,
        "tags": ["secret"],
    }
    values.update(overrides)
    return SimpleNamespace(**values)


class ProjectionTests(unittest.TestCase):
    def test_evaluator_fields_cannot_reach_agent_request(self) -> None:
        left = capstone.project_observation(observation(game_id="one", score=0.0))
        right = capstone.project_observation(observation(game_id="two", score=1.0))
        self.assertEqual(left, right)
        self.assertEqual(set(left), {"command", "frame", "available_actions"})

    def test_invalid_palette_is_rejected(self) -> None:
        bad = observation(frame=[[[16 for _ in range(64)] for _ in range(64)]])
        with self.assertRaisesRegex(capstone.CapstoneError, "palette"):
            capstone.project_observation(bad)


class ExecutionTests(unittest.TestCase):
    def test_fake_game_stops_on_honest_silence(self) -> None:
        agent = FakeAgent()
        world = capstone.FixtureWorld()
        summary, transcript = capstone.run_game("fixture", world, agent, 4)
        self.assertEqual(summary["stop_reason"], "no_outward_crossing")
        self.assertEqual(len(transcript), 1)
        self.assertEqual(
            set(agent.requests[0]), {"command", "frame", "available_actions"}
        )

    def test_replay_divergence_fails_closed(self) -> None:
        records = [
            {
                "game": "fixture",
                "turn": 0,
                "request": {
                    "command": "observe",
                    "frame": [0] * 4096,
                    "available_actions": [1],
                },
                "response": {"response": "observation", "action": None},
            }
        ]
        agent = FakeAgent([{"response": "observation", "action": 1}])
        with self.assertRaisesRegex(capstone.CapstoneError, "replay diverged"):
            capstone.replay_transcript(records, lambda _game: agent)

    def test_boundary_failure_is_preserved_as_partial_evidence(self) -> None:
        agent = FakeAgent(
            [{"response": "error", "message": "received unsupported action 6"}]
        )
        summary, transcript = capstone.run_game(
            "fixture", capstone.FixtureWorld(), agent, 4
        )

        self.assertEqual(summary["stop_reason"], "boundary_failure")
        self.assertIn("action 6", summary["first_failure"])
        self.assertEqual(transcript[0]["response"]["response"], "error")


class EvidenceTests(unittest.TestCase):
    def test_dirty_source_guard(self) -> None:
        def dirty(_command: list[str], _cwd: Path) -> str:
            return " M academy.md\n"

        with self.assertRaisesRegex(capstone.CapstoneError, "clean committed source"):
            capstone.require_clean_source(Path("."), dirty)

    def test_atomic_output_refuses_overwrite_and_names_transcript_by_digest(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "receipt"
            receipt = {"schema_version": 1, "verdict": "fixture-only"}
            records = [{"request": {"frame": [0]}, "response": {"action": None}}]
            result = capstone.write_evidence(output, receipt, records)
            transcript = output / result["transcript_file"]
            self.assertTrue(transcript.is_file())
            self.assertIn(result["transcript_sha256"], transcript.name)
            with self.assertRaisesRegex(capstone.CapstoneError, "already exists"):
                capstone.write_evidence(output, receipt, records)

            tampered = json.loads((output / "receipt.json").read_text())
            self.assertEqual(tampered["transcript_sha256"], result["transcript_sha256"])
            transcript.write_text("tampered\n")
            with self.assertRaisesRegex(capstone.CapstoneError, "transcript digest"):
                capstone.verify_evidence(output)


if __name__ == "__main__":
    unittest.main()
