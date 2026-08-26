#!/usr/bin/env python3
"""Run the teaching-free ARC-AGI-3 Academy capstone or its offline fixture."""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import os
import shutil
import subprocess
import tempfile
import time
from collections.abc import Callable, Iterable
from dataclasses import dataclass
from pathlib import Path
from types import SimpleNamespace
from typing import Any, Protocol

import tomllib


class CapstoneError(RuntimeError):
    """A fail-closed capstone configuration, boundary, or evidence error."""


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, separators=(",", ":"), sort_keys=True) + "\n").encode()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_protocol(path: Path) -> dict[str, Any]:
    data = tomllib.loads(path.read_text())
    required = {
        "schema_version",
        "receipt_schema_version",
        "toolkit_source_revision",
        "arc_agi_version",
        "arcengine_version",
        "official_selection",
        "initialization",
        "organism_seed",
        "max_actions_per_game",
        "frame_side",
        "palette_size",
        "supported_actions",
        "agent_request_fields",
        "replay_required",
    }
    missing = sorted(required - data.keys())
    if missing:
        raise CapstoneError(f"protocol is missing fields: {', '.join(missing)}")
    if data["agent_request_fields"] != ["command", "frame", "available_actions"]:
        raise CapstoneError("protocol changes the organism-visible request projection")
    if data["supported_actions"] != [1, 2, 3, 4]:
        raise CapstoneError("protocol changes the frozen physical actuators")
    return data


def _flatten(values: Any) -> Iterable[Any]:
    if hasattr(values, "reshape"):
        yield from values.reshape(-1).tolist()
        return
    if isinstance(values, (list, tuple)):
        for value in values:
            yield from _flatten(value)
        return
    yield values


def project_observation(observation: Any) -> dict[str, object]:
    """Project official state onto the complete organism-visible interface."""
    frames = observation.frame
    if not frames:
        raise CapstoneError("official observation contains no frame")
    frame = [int(value) for value in _flatten(frames[-1])]
    if len(frame) != 64 * 64:
        raise CapstoneError(f"frame has {len(frame)} cells; expected 4096")
    if any(value < 0 or value >= 16 for value in frame):
        raise CapstoneError("frame contains a value outside the 16-color palette")
    actions = [int(value) for value in observation.available_actions]
    if not actions:
        raise CapstoneError("official observation exposes no available actions")
    if any(value < 1 or value > 7 for value in actions):
        raise CapstoneError("official observation exposes an invalid action identifier")
    return {
        "command": "observe",
        "frame": frame,
        "available_actions": actions,
    }


class Agent(Protocol):
    ready: dict[str, object]

    def command(self, request: dict[str, object]) -> dict[str, object]: ...

    def close(self) -> None: ...


@dataclass
class AgentProcess:
    process: subprocess.Popen[str]
    ready: dict[str, object]

    @classmethod
    def start(cls, executable: Path, seed: int) -> AgentProcess:
        process = subprocess.Popen(
            [str(executable), str(seed)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        agent = cls(process=process, ready={})
        ready = agent._read()
        if ready.get("response") != "ready":
            process.kill()
            raise CapstoneError(f"capstone agent did not become ready: {ready}")
        agent.ready = ready
        return agent

    def _read(self) -> dict[str, object]:
        if self.process.stdout is None:
            raise CapstoneError("capstone agent stdout is unavailable")
        line = self.process.stdout.readline()
        if not line:
            stderr = self.process.stderr.read() if self.process.stderr else ""
            raise CapstoneError(f"capstone agent exited unexpectedly: {stderr.strip()}")
        value = json.loads(line)
        if not isinstance(value, dict):
            raise CapstoneError("capstone agent emitted a non-object response")
        return value

    def command(self, request: dict[str, object]) -> dict[str, object]:
        if self.process.stdin is None:
            raise CapstoneError("capstone agent stdin is unavailable")
        self.process.stdin.write(canonical_bytes(request).decode())
        self.process.stdin.flush()
        return self._read()

    def close(self) -> None:
        if self.process.poll() is None and self.process.stdin is not None:
            try:
                self.process.stdin.write('{"command":"shutdown"}\n')
                self.process.stdin.flush()
                self.process.wait(timeout=10)
            except (BrokenPipeError, subprocess.TimeoutExpired):
                self.process.kill()
                self.process.wait(timeout=10)
        if self.process.returncode not in (0, None):
            stderr = self.process.stderr.read() if self.process.stderr else ""
            raise CapstoneError(
                f"capstone agent exited {self.process.returncode}: {stderr.strip()}"
            )


class FixtureWorld:
    """Public, deterministic, non-scoring smoke world."""

    def reset(self) -> SimpleNamespace:
        frame = [[0 for _ in range(64)] for _ in range(64)]
        frame[31][31] = 9
        return SimpleNamespace(
            frame=[frame],
            available_actions=[1, 2, 3, 4],
            state=SimpleNamespace(value="NOT_FINISHED"),
            levels_completed=0,
        )

    def step(self, _action: int) -> SimpleNamespace:
        raise CapstoneError("fixture body unexpectedly produced an action")


def _state(observation: Any) -> str:
    state = getattr(observation, "state", "UNKNOWN")
    return str(getattr(state, "value", state))


def _terminal(observation: Any) -> bool:
    return _state(observation) in {"WIN", "GAME_OVER"}


def _observation_metrics(response: dict[str, object]) -> dict[str, int]:
    fields = (
        "outward_crossings",
        "plasticity_updates",
        "modulatory_deliveries",
        "physical_work",
    )
    return {field: int(response.get(field, 0)) for field in fields}


def run_game(
    game: str,
    world: Any,
    agent: Agent,
    max_actions: int,
) -> tuple[dict[str, object], list[dict[str, object]]]:
    observation = world.reset()
    transcript: list[dict[str, object]] = []
    totals = {
        "outward_crossings": 0,
        "plasticity_updates": 0,
        "modulatory_deliveries": 0,
        "physical_work": 0,
    }
    stop_reason = "action_budget"
    failure: str | None = None
    final_fingerprint = str(agent.ready.get("body_fingerprint", ""))

    for turn in range(max_actions):
        if _terminal(observation):
            stop_reason = "official_terminal"
            break
        request = project_observation(observation)
        response = agent.command(request)
        record: dict[str, object] = {
            "game": game,
            "turn": turn,
            "request": request,
            "response": response,
        }
        if turn == 0:
            record["initial"] = agent.ready
        transcript.append(record)
        if response.get("response") == "error":
            stop_reason = "boundary_failure"
            failure = str(response.get("message", "agent boundary error"))
            break
        if response.get("response") != "observation":
            stop_reason = "protocol_failure"
            failure = f"unexpected agent response {response.get('response')!r}"
            break
        for name, value in _observation_metrics(response).items():
            totals[name] += value
        if not bool(response.get("naturally_quiescent", False)):
            stop_reason = "non_quiescent"
            failure = "agent transition did not reach natural quiescence"
            break
        final_fingerprint = str(response.get("body_fingerprint", ""))
        action = response.get("action")
        if action is None:
            stop_reason = "no_outward_crossing"
            break
        action_id = int(action)
        if action_id not in request["available_actions"]:
            stop_reason = "unavailable_action"
            failure = f"agent selected unavailable action {action_id}"
            break
        observation = world.step(action_id)
    else:
        failure = f"action budget {max_actions} exhausted"

    summary: dict[str, object] = {
        "game": game,
        "actions": sum(
            record["response"].get("action") is not None
            for record in transcript
            if record["response"].get("response") == "observation"
        ),
        "observations": len(transcript),
        "official_state": _state(observation),
        "levels_completed": int(getattr(observation, "levels_completed", 0)),
        "stop_reason": stop_reason,
        "first_failure": failure,
        "initial_body_fingerprint": str(agent.ready.get("body_fingerprint", "")),
        "final_body_fingerprint": final_fingerprint,
        **totals,
    }
    return summary, transcript


def replay_transcript(
    records: list[dict[str, object]],
    agent_factory: Callable[[str], Agent],
) -> dict[str, object]:
    games: list[str] = []
    for record in records:
        game = str(record["game"])
        if game not in games:
            games.append(game)
    replayed = 0
    for game in games:
        agent = agent_factory(game)
        try:
            game_records = [item for item in records if item["game"] == game]
            if (
                game_records
                and "initial" in game_records[0]
                and canonical_bytes(agent.ready)
                != canonical_bytes(game_records[0]["initial"])
            ):
                raise CapstoneError(f"initial body replay diverged for {game}")
            for record in game_records:
                actual = agent.command(record["request"])
                if canonical_bytes(actual) != canonical_bytes(record["response"]):
                    raise CapstoneError(
                        f"transcript replay diverged for {game} turn {record['turn']}"
                    )
                replayed += 1
        finally:
            agent.close()
    return {"exact": True, "games": len(games), "observations": replayed}


def _default_git(command: list[str], cwd: Path) -> str:
    return subprocess.run(
        command,
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def require_clean_source(
    repository: Path,
    runner: Callable[[list[str], Path], str] = _default_git,
) -> str:
    status = runner(
        ["git", "status", "--porcelain", "--untracked-files=normal"], repository
    )
    if status:
        raise CapstoneError("official mode requires clean committed source")
    revision = runner(["git", "rev-parse", "HEAD"], repository)
    if len(revision) != 40:
        raise CapstoneError("official mode could not resolve a named source commit")
    return revision


def _transcript_bytes(records: list[dict[str, object]]) -> bytes:
    return b"".join(canonical_bytes(record) for record in records)


def write_evidence(
    output: Path,
    receipt: dict[str, object],
    records: list[dict[str, object]],
) -> dict[str, object]:
    if output.exists():
        raise CapstoneError(f"output already exists: {output}")
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=f".{output.name}-", dir=output.parent))
    try:
        transcript_bytes = _transcript_bytes(records)
        transcript_digest = sha256_bytes(transcript_bytes)
        transcript_name = f"transcript-{transcript_digest}.jsonl"
        completed = dict(receipt)
        completed["transcript_file"] = transcript_name
        completed["transcript_sha256"] = transcript_digest
        completed["receipt_sha256"] = sha256_bytes(canonical_bytes(completed))
        (temporary / transcript_name).write_bytes(transcript_bytes)
        (temporary / "receipt.json").write_bytes(canonical_bytes(completed))
        temporary.replace(output)
        return completed
    except BaseException:
        shutil.rmtree(temporary, ignore_errors=True)
        raise


def verify_evidence(
    output: Path,
    *,
    agent_path: Path | None = None,
    protocol_path: Path | None = None,
) -> dict[str, object]:
    receipt_path = output / "receipt.json"
    receipt = json.loads(receipt_path.read_text())
    transcript_path = output / str(receipt["transcript_file"])
    if sha256_file(transcript_path) != receipt["transcript_sha256"]:
        raise CapstoneError("transcript digest mismatch")
    unhashed = dict(receipt)
    claimed_receipt_digest = unhashed.pop("receipt_sha256")
    if sha256_bytes(canonical_bytes(unhashed)) != claimed_receipt_digest:
        raise CapstoneError("receipt digest mismatch")
    if agent_path is not None and sha256_file(agent_path) != receipt["agent_sha256"]:
        raise CapstoneError("agent binary digest mismatch")
    if (
        protocol_path is not None
        and sha256_file(protocol_path) != receipt["protocol_sha256"]
    ):
        raise CapstoneError("protocol digest mismatch")
    return receipt


def _totals(summaries: list[dict[str, object]]) -> dict[str, int]:
    fields = (
        "actions",
        "observations",
        "outward_crossings",
        "plasticity_updates",
        "modulatory_deliveries",
        "physical_work",
    )
    return {name: sum(int(summary[name]) for summary in summaries) for name in fields}


def _versions(protocol: dict[str, Any]) -> dict[str, str]:
    versions = {
        "arc-agi": importlib.metadata.version("arc-agi"),
        "arcengine": importlib.metadata.version("arcengine"),
    }
    expected = {
        "arc-agi": str(protocol["arc_agi_version"]),
        "arcengine": str(protocol["arcengine_version"]),
    }
    if versions != expected:
        raise CapstoneError(f"SDK version drift: expected {expected}, found {versions}")
    return versions


def _base_receipt(
    mode: str,
    protocol: dict[str, Any],
    protocol_path: Path,
    agent_path: Path,
    source_revision: str,
) -> dict[str, object]:
    return {
        "schema_version": int(protocol["receipt_schema_version"]),
        "mode": mode,
        "source_revision": source_revision,
        "protocol_sha256": sha256_file(protocol_path),
        "agent_sha256": sha256_file(agent_path),
        "toolkit_source_revision": protocol["toolkit_source_revision"],
        "sdk_versions": _versions(protocol),
        "organism_seed": int(protocol["organism_seed"]),
        "official_scorecard": None,
    }


def run_fixture(
    repository: Path,
    protocol_path: Path,
    agent_path: Path,
    output: Path,
) -> dict[str, object]:
    protocol = load_protocol(protocol_path)
    seed = int(protocol["organism_seed"])
    agent = AgentProcess.start(agent_path, seed)
    try:
        summary, transcript = run_game("public-fixture", FixtureWorld(), agent, 4)
    finally:
        agent.close()
    replay = replay_transcript(
        transcript, lambda _game: AgentProcess.start(agent_path, seed)
    )
    revision = _default_git(["git", "rev-parse", "HEAD"], repository)
    receipt = _base_receipt("fixture", protocol, protocol_path, agent_path, revision)
    receipt.update(
        {
            "verdict": "fixture-only",
            "official": False,
            "suite_selection": "public-synthetic-smoke",
            "games": [summary],
            "physical_totals": _totals([summary]),
            "first_failure": summary["first_failure"],
            "transcript_replay": replay,
        }
    )
    return write_evidence(output, receipt, transcript)


def _scrub_secrets(value: Any) -> Any:
    if isinstance(value, dict):
        return {
            key: _scrub_secrets(item)
            for key, item in value.items()
            if key.lower() not in {"api_key", "arc_api_key", "token", "authorization"}
        }
    if isinstance(value, list):
        return [_scrub_secrets(item) for item in value]
    return value


class OfficialWorld:
    def __init__(self, environment: Any) -> None:
        self.environment = environment

    def reset(self) -> Any:
        observation = self.environment.reset()
        if observation is None:
            raise CapstoneError("official reset returned no observation")
        return observation

    def step(self, action_id: int) -> Any:
        from arcengine import GameAction

        action = GameAction.from_id(action_id)
        if not action.is_simple():
            raise CapstoneError(f"coordinate action {action_id} is unsupported")
        observation = self.environment.step(action, data={})
        if observation is None:
            raise CapstoneError("official step returned no observation")
        return observation


def run_official(
    repository: Path,
    protocol_path: Path,
    agent_path: Path,
    output: Path,
) -> dict[str, object]:
    revision = require_clean_source(repository)
    api_key = os.environ.get("ARC_API_KEY")
    if not api_key:
        raise CapstoneError("official mode requires ARC_API_KEY")
    protocol = load_protocol(protocol_path)
    seed = int(protocol["organism_seed"])
    max_actions = int(protocol["max_actions_per_game"])
    receipt = _base_receipt("official", protocol, protocol_path, agent_path, revision)

    from arc_agi import Arcade, OperationMode

    arcade = Arcade(arc_api_key=api_key, operation_mode=OperationMode.COMPETITION)
    scorecard_id: str | None = None
    scorecard: object | None = None
    close_failure: str | None = None
    summaries: list[dict[str, object]] = []
    transcript: list[dict[str, object]] = []
    discovered: list[str] = []
    suite_failure: str | None = None
    started = time.monotonic()
    try:
        environments = arcade.get_environments()
        discovered = [str(item.game_id) for item in environments]
        if not discovered or len(discovered) != len(set(discovered)):
            raise CapstoneError(
                "server-discovered suite is empty or contains duplicates"
            )
        scorecard_id = arcade.create_scorecard(
            tags=["truelearner-academy", "arc3-capstone"],
            opaque={
                "source_revision": revision,
                "protocol_sha256": receipt["protocol_sha256"],
                "agent_sha256": receipt["agent_sha256"],
            },
        )
        for game in discovered:
            agent: AgentProcess | None = None
            try:
                agent = AgentProcess.start(agent_path, seed)
                environment = arcade.make(
                    game,
                    seed=seed,
                    scorecard_id=scorecard_id,
                    save_recording=False,
                    include_frame_data=True,
                )
                if environment is None:
                    raise CapstoneError(f"official SDK could not create {game}")
                summary, records = run_game(
                    game, OfficialWorld(environment), agent, max_actions
                )
                summaries.append(summary)
                transcript.extend(records)
            except Exception as error:  # noqa: BLE001 - SDK/process failures are evidence.
                summaries.append(
                    {
                        "game": game,
                        "actions": 0,
                        "observations": 0,
                        "outward_crossings": 0,
                        "plasticity_updates": 0,
                        "modulatory_deliveries": 0,
                        "physical_work": 0,
                        "official_state": "UNKNOWN",
                        "levels_completed": 0,
                        "stop_reason": "execution_failure",
                        "first_failure": str(error),
                        "initial_body_fingerprint": str(
                            agent.ready.get("body_fingerprint", "") if agent else ""
                        ),
                        "final_body_fingerprint": str(
                            agent.ready.get("body_fingerprint", "") if agent else ""
                        ),
                    }
                )
            finally:
                if agent is not None:
                    try:
                        agent.close()
                    except Exception as error:  # noqa: BLE001 - preserve partial evidence.
                        summaries[-1]["first_failure"] = (
                            summaries[-1].get("first_failure")
                            or f"agent close failed: {error}"
                        )
                        summaries[-1]["stop_reason"] = "agent_exit"
    except Exception as error:  # noqa: BLE001 - preserve partial SDK evidence.
        suite_failure = str(error)
    finally:
        if scorecard_id is not None:
            try:
                closed = arcade.close_scorecard(scorecard_id)
                if closed is not None:
                    scorecard = _scrub_secrets(closed.model_dump(mode="json"))
            except Exception as error:  # noqa: BLE001 - closing failure is evidence.
                close_failure = str(error)

    replay: dict[str, object]
    try:
        replay = replay_transcript(
            transcript, lambda _game: AgentProcess.start(agent_path, seed)
        )
    except Exception as error:  # noqa: BLE001 - divergence must become a receipt.
        replay = {"exact": False, "error": str(error)}

    failures = [
        str(summary["first_failure"])
        for summary in summaries
        if summary.get("first_failure") is not None
    ]
    if close_failure:
        failures.append(f"scorecard close failed: {close_failure}")
    if suite_failure:
        failures.append(suite_failure)
    if not bool(replay.get("exact")):
        failures.append(str(replay.get("error", "transcript replay diverged")))
    receipt.update(
        {
            "verdict": "complete"
            if not failures and scorecard is not None
            else "inconclusive",
            "official": True,
            "suite_selection": "complete-server-discovered-suite",
            "discovered_games": discovered,
            "games": summaries,
            "physical_totals": _totals(summaries),
            "first_failure": failures[0] if failures else None,
            "transcript_replay": replay,
            "official_scorecard": scorecard,
            "wall_seconds": time.monotonic() - started,
        }
    )
    return write_evidence(output, receipt, transcript)


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("fixture", "official"), required=True)
    parser.add_argument("--agent", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    return parser.parse_args()


def main() -> int:
    args = arguments()
    script = Path(__file__).resolve()
    repository = script.parents[3]
    protocol_path = script.with_name("protocol.toml")
    agent_path = args.agent.resolve()
    if not agent_path.is_file():
        raise CapstoneError(f"capstone agent does not exist: {agent_path}")
    if args.mode == "fixture":
        receipt = run_fixture(
            repository, protocol_path, agent_path, args.output.resolve()
        )
    else:
        receipt = run_official(
            repository, protocol_path, agent_path, args.output.resolve()
        )
    print(
        "ARC3_CAPSTONE_COMPLETE "
        f"mode={receipt['mode']} verdict={receipt['verdict']} "
        f"receipt={args.output.resolve() / 'receipt.json'}"
    )
    return 0 if receipt["verdict"] in {"complete", "fixture-only"} else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except CapstoneError as error:
        raise SystemExit(f"ARC3_CAPSTONE_ERROR {error}") from error
