#!/usr/bin/env python3
"""Run the blind TrueLearner ARC-AGI-3 development probe."""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import logging
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
    """A fail-closed boundary, protocol, or evidence error."""


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
        "body_parent_revision",
        "toolkit_source_revision",
        "arc_agi_version",
        "arcengine_version",
        "official_selection",
        "public_game",
        "organism_seed",
        "max_actions_per_game",
        "frame_side",
        "palette_size",
        "supported_actions",
        "agent_request_fields",
        "replay_required",
        "holdout_policy",
    }
    missing = sorted(required - data.keys())
    if missing:
        raise CapstoneError(f"protocol is missing fields: {', '.join(missing)}")
    if data["agent_request_fields"] != ["command", "frame", "actions"]:
        raise CapstoneError("protocol changes the organism-visible projection")
    if data["supported_actions"] != [1, 2, 3, 4, 5, 6, 7]:
        raise CapstoneError("protocol changes the frozen physical actuators")
    return data


def _flatten(values: Any) -> Iterable[Any]:
    if hasattr(values, "reshape"):
        yield from values.reshape(-1).tolist()
    elif isinstance(values, (list, tuple)):
        for value in values:
            yield from _flatten(value)
    else:
        yield values


def project_observation(observation: Any) -> dict[str, object]:
    """The complete organism-visible interface: pixels and legal action shapes."""
    frames = observation.frame
    if not frames:
        raise CapstoneError("official observation contains no frame")
    frame = [int(value) for value in _flatten(frames[-1])]
    if len(frame) != 64 * 64:
        raise CapstoneError(f"frame has {len(frame)} cells; expected 4096")
    if any(value < 0 or value >= 16 for value in frame):
        raise CapstoneError("frame contains a value outside the 16-color palette")
    actions = [int(value) for value in observation.available_actions]
    if not actions or len(actions) != len(set(actions)):
        raise CapstoneError("available actions are empty or duplicated")
    if any(value < 1 or value > 7 for value in actions):
        raise CapstoneError("official observation exposes an invalid action")
    offers = [
        {
            "id": action,
            "schema": (
                {"type": "point", "width": 64, "height": 64}
                if action == 6
                else {"type": "unit"}
            ),
        }
        for action in sorted(actions)
    ]
    return {"command": "observe", "frame": frame, "actions": {"offers": offers}}


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
        agent = cls(process, {})
        ready = agent._read()
        if ready.get("response") != "ready":
            process.kill()
            raise CapstoneError(f"agent did not become ready: {ready}")
        agent.ready = ready
        return agent

    def _read(self) -> dict[str, object]:
        if self.process.stdout is None:
            raise CapstoneError("agent stdout is unavailable")
        line = self.process.stdout.readline()
        if not line:
            stderr = self.process.stderr.read() if self.process.stderr else ""
            raise CapstoneError(f"agent exited unexpectedly: {stderr.strip()}")
        value = json.loads(line)
        if not isinstance(value, dict):
            raise CapstoneError("agent emitted a non-object response")
        return value

    def command(self, request: dict[str, object]) -> dict[str, object]:
        if self.process.stdin is None:
            raise CapstoneError("agent stdin is unavailable")
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
            raise CapstoneError(f"agent exited {self.process.returncode}: {stderr.strip()}")


class FixtureWorld:
    """Deterministic boundary fixture. It does not teach an action."""

    def __init__(self) -> None:
        self.turn = 0
        self.last_action = 0

    def _observation(self) -> SimpleNamespace:
        frame = [[0 for _ in range(64)] for _ in range(64)]
        frame[31][31] = 9
        frame[31][32] = self.turn
        frame[31][33] = self.last_action
        terminal = self.turn >= 3
        return SimpleNamespace(
            frame=[frame],
            available_actions=[1, 2, 3, 4, 5, 7],
            state=SimpleNamespace(value="WIN" if terminal else "NOT_FINISHED"),
            levels_completed=int(terminal),
        )

    def reset(self) -> SimpleNamespace:
        self.turn = 0
        self.last_action = 0
        return self._observation()

    def step(self, call: dict[str, object]) -> SimpleNamespace:
        action = int(call["id"])
        if action not in (1, 2, 3, 4, 5, 7):
            raise CapstoneError(f"fixture received unavailable action {action}")
        if call.get("arguments") != {"type": "unit"}:
            raise CapstoneError(f"fixture received invalid arguments {call}")
        self.turn += 1
        self.last_action = action
        return self._observation()


class OfficialWorld:
    def __init__(self, environment: Any) -> None:
        self.environment = environment

    def reset(self) -> Any:
        observation = self.environment.reset()
        if observation is None:
            raise CapstoneError("official reset returned no observation")
        return observation

    def step(self, call: dict[str, object]) -> Any:
        from arcengine import GameAction

        action_id = int(call["id"])
        action = GameAction.from_id(action_id)
        arguments = call.get("arguments")
        if action.is_simple():
            if arguments != {"type": "unit"}:
                raise CapstoneError(f"simple action {action_id} has invalid arguments")
            data: dict[str, int] = {}
        else:
            if not isinstance(arguments, dict) or arguments.get("type") != "point":
                raise CapstoneError(f"point action {action_id} has invalid arguments")
            data = {"x": int(arguments["x"]), "y": int(arguments["y"])}
        observation = self.environment.step(action, data=data)
        if observation is None:
            raise CapstoneError("official step returned no observation")
        return observation


def _state(observation: Any) -> str:
    state = getattr(observation, "state", "UNKNOWN")
    return str(getattr(state, "value", state))


def _terminal(observation: Any) -> bool:
    return _state(observation) in {"WIN", "GAME_OVER"}


def run_game(
    game: str, world: Any, agent: Agent, max_actions: int
) -> tuple[dict[str, object], list[dict[str, object]]]:
    observation = world.reset()
    transcript: list[dict[str, object]] = []
    totals = {"outward_crossings": 0, "plasticity_updates": 0, "physical_work": 0}
    stop_reason = "action_budget"
    first_failure: str | None = None

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
            "external": {
                "state": _state(observation),
                "levels_completed": int(getattr(observation, "levels_completed", 0)),
            },
        }
        if turn == 0:
            record["initial"] = agent.ready
        transcript.append(record)
        if response.get("response") == "error":
            stop_reason = "boundary_failure"
            first_failure = str(response.get("message", "agent boundary error"))
            break
        if response.get("response") != "observation":
            stop_reason = "protocol_failure"
            first_failure = f"unexpected response {response.get('response')!r}"
            break
        for name in totals:
            totals[name] += int(response.get(name, 0))
        if not bool(response.get("naturally_quiescent", False)):
            stop_reason = "non_quiescent"
            first_failure = "agent transition did not reach natural quiescence"
            break
        call = response.get("call")
        if call is None:
            stop_reason = "no_outward_crossing"
            first_failure = "no mapped offered motor crossing survived competition"
            break
        if not isinstance(call, dict):
            stop_reason = "protocol_failure"
            first_failure = "agent emitted a non-object action call"
            break
        offered = {int(offer["id"]) for offer in request["actions"]["offers"]}
        if int(call.get("id", -1)) not in offered:
            stop_reason = "unavailable_action"
            first_failure = f"agent selected unavailable action {call.get('id')}"
            break
        observation = world.step(call)
    else:
        first_failure = f"action budget {max_actions} exhausted"

    summary: dict[str, object] = {
        "game": game,
        "actions": sum(
            record["response"].get("call") is not None
            for record in transcript
            if record["response"].get("response") == "observation"
        ),
        "observations": len(transcript),
        "official_state": _state(observation),
        "levels_completed": int(getattr(observation, "levels_completed", 0)),
        "stop_reason": stop_reason,
        "first_failure": first_failure,
        "initial_body_fingerprint": str(agent.ready.get("body_fingerprint", "")),
        "final_body_fingerprint": str(
            transcript[-1]["response"].get("body_fingerprint", "")
            if transcript
            else agent.ready.get("body_fingerprint", "")
        ),
        **totals,
    }
    return summary, transcript


def replay_transcript(
    records: list[dict[str, object]], agent_factory: Callable[[], Agent]
) -> dict[str, object]:
    agent = agent_factory()
    try:
        if records and "initial" in records[0]:
            if canonical_bytes(agent.ready) != canonical_bytes(records[0]["initial"]):
                raise CapstoneError("initial body replay diverged")
        for record in records:
            actual = agent.command(record["request"])
            if canonical_bytes(actual) != canonical_bytes(record["response"]):
                raise CapstoneError(f"transcript replay diverged at turn {record['turn']}")
    finally:
        agent.close()
    return {"exact": True, "observations": len(records)}


def _default_git(command: list[str], cwd: Path) -> str:
    return subprocess.run(
        command, cwd=cwd, check=True, capture_output=True, text=True
    ).stdout.strip()


def require_clean_source(
    repository: Path, runner: Callable[[list[str], Path], str] = _default_git
) -> str:
    status = runner(
        ["git", "status", "--porcelain", "--untracked-files=normal"], repository
    )
    if status:
        raise CapstoneError("public mode requires clean committed source")
    revision = runner(["git", "rev-parse", "HEAD"], repository)
    if len(revision) != 40:
        raise CapstoneError("public mode could not resolve a named source commit")
    return revision


def write_evidence(
    output: Path, receipt: dict[str, object], records: list[dict[str, object]]
) -> dict[str, object]:
    if output.exists():
        raise CapstoneError(f"output already exists: {output}")
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=f".{output.name}-", dir=output.parent))
    try:
        transcript_bytes = b"".join(canonical_bytes(record) for record in records)
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


def verify_evidence(output: Path) -> dict[str, object]:
    receipt = json.loads((output / "receipt.json").read_text())
    transcript = output / str(receipt["transcript_file"])
    if sha256_file(transcript) != receipt["transcript_sha256"]:
        raise CapstoneError("transcript digest mismatch")
    unhashed = dict(receipt)
    claimed = unhashed.pop("receipt_sha256")
    if sha256_bytes(canonical_bytes(unhashed)) != claimed:
        raise CapstoneError("receipt digest mismatch")
    return receipt


def _versions(protocol: dict[str, Any]) -> dict[str, str]:
    actual = {
        "arc-agi": importlib.metadata.version("arc-agi"),
        "arcengine": importlib.metadata.version("arcengine"),
    }
    expected = {
        "arc-agi": str(protocol["arc_agi_version"]),
        "arcengine": str(protocol["arcengine_version"]),
    }
    if actual != expected:
        raise CapstoneError(f"SDK version drift: expected {expected}, found {actual}")
    return actual


def _receipt(
    mode: str,
    protocol: dict[str, Any],
    protocol_path: Path,
    agent_path: Path,
    revision: str,
    summary: dict[str, object],
    replay: dict[str, object],
    wall_seconds: float,
) -> dict[str, object]:
    return {
        "schema_version": int(protocol["receipt_schema_version"]),
        "mode": mode,
        "verdict": "fixture-only" if mode == "fixture" else "development-evidence",
        "official": False,
        "source_revision": revision,
        "body_parent_revision": protocol["body_parent_revision"],
        "protocol_sha256": sha256_file(protocol_path),
        "agent_sha256": sha256_file(agent_path),
        "toolkit_source_revision": protocol["toolkit_source_revision"],
        "sdk_versions": _versions(protocol),
        "organism_seed": int(protocol["organism_seed"]),
        "suite_selection": protocol["official_selection"],
        "holdout_policy": protocol["holdout_policy"],
        "games": [summary],
        "physical_totals": {
            name: int(summary[name])
            for name in (
                "actions",
                "observations",
                "outward_crossings",
                "plasticity_updates",
                "physical_work",
            )
        },
        "first_failure": summary["first_failure"],
        "transcript_replay": replay,
        "official_scorecard": None,
        "wall_seconds": wall_seconds,
    }


def run_fixture(
    repository: Path, protocol_path: Path, agent_path: Path, output: Path
) -> dict[str, object]:
    protocol = load_protocol(protocol_path)
    seed = int(protocol["organism_seed"])
    started = time.monotonic()
    agent = AgentProcess.start(agent_path, seed)
    try:
        summary, transcript = run_game("public-fixture", FixtureWorld(), agent, 4)
    finally:
        agent.close()
    replay = replay_transcript(transcript, lambda: AgentProcess.start(agent_path, seed))
    revision = _default_git(["git", "rev-parse", "HEAD"], repository)
    return write_evidence(
        output,
        _receipt(
            "fixture",
            protocol,
            protocol_path,
            agent_path,
            revision,
            summary,
            replay,
            time.monotonic() - started,
        ),
        transcript,
    )


def run_public(
    repository: Path,
    protocol_path: Path,
    agent_path: Path,
    output: Path,
    game: str,
) -> dict[str, object]:
    revision = require_clean_source(repository)
    protocol = load_protocol(protocol_path)
    if game != protocol["public_game"]:
        raise CapstoneError(f"public game must remain frozen as {protocol['public_game']}")
    seed = int(protocol["organism_seed"])
    started = time.monotonic()

    from arc_agi import Arcade, OperationMode

    logger = logging.getLogger("truelearner.arc3.development")
    logger.handlers = [logging.NullHandler()]
    logger.propagate = False
    sdk_storage = output.parent / "arc3-sdk"
    arcade = Arcade(
        operation_mode=OperationMode.NORMAL,
        environments_dir=str(sdk_storage / "environment_files"),
        recordings_dir=str(sdk_storage / "recordings"),
        logger=logger,
    )
    environment = arcade.make(
        game, seed=seed, save_recording=False, include_frame_data=True
    )
    if environment is None:
        raise CapstoneError(f"public SDK could not create {game}")
    agent = AgentProcess.start(agent_path, seed)
    try:
        summary, transcript = run_game(
            game, OfficialWorld(environment), agent, int(protocol["max_actions_per_game"])
        )
    finally:
        agent.close()
    replay = replay_transcript(transcript, lambda: AgentProcess.start(agent_path, seed))
    return write_evidence(
        output,
        _receipt(
            "public",
            protocol,
            protocol_path,
            agent_path,
            revision,
            summary,
            replay,
            time.monotonic() - started,
        ),
        transcript,
    )


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("fixture", "public"), required=True)
    parser.add_argument("--agent", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--game", default="ls20")
    return parser.parse_args()


def main() -> int:
    args = arguments()
    script = Path(__file__).resolve()
    repository = script.parents[3]
    protocol_path = script.with_name("protocol.toml")
    agent_path = args.agent.resolve()
    if not agent_path.is_file():
        raise CapstoneError(f"agent does not exist: {agent_path}")
    if args.mode == "fixture":
        receipt = run_fixture(repository, protocol_path, agent_path, args.output.resolve())
    else:
        receipt = run_public(
            repository, protocol_path, agent_path, args.output.resolve(), args.game
        )
    print(
        "ARC3_DEVELOPMENT_PROBE_COMPLETE "
        f"mode={receipt['mode']} verdict={receipt['verdict']} "
        f"receipt={args.output.resolve() / 'receipt.json'}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except CapstoneError as error:
        raise SystemExit(f"ARC3_DEVELOPMENT_PROBE_ERROR {error}") from error
