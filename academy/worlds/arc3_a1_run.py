#!/usr/bin/env python3
"""Run ARC3-A1 against the official environment and a persistent Rust body."""

from __future__ import annotations

import argparse
import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import arc_agi
from arcengine import GameAction

SCHEMA_VERSION = 1
TOOLKIT_REVISION = "arcprize/ARC-AGI@f12822c4d550121c35a275008d964afbbed47d2f"
IDENTITY_MAP = [1, 2, 3, 4]
SHUFFLED_MAP = [2, 1, 3, 4]


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--agent", type=Path, required=True)
    parser.add_argument("--game", default="ls20")
    parser.add_argument("--seed", type=int, default=205)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--replay-output", type=Path)
    return parser.parse_args()


@dataclass
class Agent:
    process: subprocess.Popen[str]

    @classmethod
    def start(cls, executable: Path, seed: int) -> "Agent":
        process = subprocess.Popen(
            [str(executable), str(seed)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        return cls(process)

    def command(self, command: dict[str, Any]) -> dict[str, Any]:
        if self.process.stdin is None or self.process.stdout is None:
            raise RuntimeError("ARC agent pipes are unavailable")
        self.process.stdin.write(json.dumps(command, separators=(",", ":"), sort_keys=True) + "\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        if not line:
            stderr = self.process.stderr.read() if self.process.stderr else ""
            raise RuntimeError(f"ARC agent stopped unexpectedly: {stderr}")
        response = json.loads(line)
        if response.get("response") == "error":
            raise RuntimeError(response.get("message", "ARC agent error"))
        return response

    def close(self) -> None:
        try:
            self.command({"command": "shutdown"})
        except RuntimeError:
            pass
        self.process.wait(timeout=10)
        if self.process.returncode != 0:
            stderr = self.process.stderr.read() if self.process.stderr else ""
            raise RuntimeError(f"ARC agent exited {self.process.returncode}: {stderr}")


def frame(observation: Any) -> list[int]:
    if not observation.frame:
        raise RuntimeError("official ARC observation has no frame")
    values = [int(value) for value in observation.frame[-1].reshape(-1).tolist()]
    if len(values) != 64 * 64 or any(value < 0 or value > 15 for value in values):
        raise RuntimeError("official ARC observation has an invalid raster")
    return values


def new_environment(arcade: Any, game: str, seed: int) -> tuple[Any, Any]:
    environment = arcade.make(game, seed=seed, include_frame_data=True)
    if environment is None:
        raise RuntimeError(f"unable to create ARC-AGI-3 environment {game}")
    observation = environment.reset()
    if observation is None:
        raise RuntimeError("official ARC reset returned no observation")
    return environment, observation


def organism_observe(
    agent: Agent,
    observation: Any,
    *,
    babble_action: int | None,
    support_previous: bool,
    settle_pressure: bool,
    action_map: list[int] = IDENTITY_MAP,
) -> dict[str, Any]:
    response = agent.command(
        {
            "command": "observe",
            "frame": frame(observation),
            "available_actions": [int(value) for value in observation.available_actions],
            "babble_action": babble_action,
            "support_previous": support_previous,
            "settle_pressure": settle_pressure,
            "action_map": action_map,
        }
    )
    if response.get("response") != "observation":
        raise RuntimeError(f"unexpected ARC agent response: {response}")
    del response["response"]
    return response


def turn_record(turn: int, observation: Any, organism: dict[str, Any], caption: str) -> dict[str, Any]:
    return {
        "turn": turn,
        "frame": frame(observation),
        "organism": organism,
        "official_state": observation.state.value,
        "levels_completed": int(observation.levels_completed),
        "win_levels": int(observation.win_levels),
        "caption": caption,
    }


def step(environment: Any, action_id: int | None) -> Any:
    if action_id is None:
        raise RuntimeError("episode expected an organism action but received silence")
    action = GameAction.from_id(action_id)
    data: dict[str, int] = {}
    if action.is_complex():
        data = {"x": 32, "y": 32}
    observation = environment.step(action, data=data)
    if observation is None:
        raise RuntimeError("official ARC step returned no observation")
    return observation


def episode(
    identifier: str,
    title: str,
    summary: str,
    episode_class: str,
    outcome: str,
    turns: list[dict[str, Any]],
) -> dict[str, Any]:
    return {
        "id": identifier,
        "title": title,
        "summary": summary,
        "class": episode_class,
        "outcome": outcome,
        "turns": turns,
    }


def run_suite(agent_path: Path, game: str, seed: int) -> dict[str, Any]:
    arcade = arc_agi.Arcade()
    agent = Agent.start(agent_path, seed)
    episodes: list[dict[str, Any]] = []
    try:
        # Fresh body: context routes traverse but remain below the motor threshold.
        agent.command({"command": "reset_body"})
        environment, observation = new_environment(arcade, game, seed)
        quiet = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=False,
            settle_pressure=False,
        )
        episodes.append(
            episode(
                "arc3-a1-untrained",
                "Before motor babbling",
                "The raster reaches weak motor routes, but none can yet cross the boundary.",
                "control",
                "expected-silence",
                [turn_record(0, observation, quiet, "Raster admitted · every weak route stays subthreshold")],
            )
        )

        # Initial physical exploration.
        agent.command({"command": "reset_body"})
        environment, observation = new_environment(arcade, game, seed)
        explored = organism_observe(
            agent,
            observation,
            babble_action=1,
            support_previous=False,
            settle_pressure=False,
        )
        episodes.append(
            episode(
                "arc3-a1-exploration",
                "A first physical action",
                "One ordinary babbling pulse completes a subthreshold motor path.",
                "development",
                "scaffolded-action",
                [turn_record(0, observation, explored, "Motor babbling · outward crossing selects action 1")],
            )
        )

        # Development: the official changed raster closes the loop twice.
        development_turns = [
            turn_record(0, observation, explored, "The body acts before knowing the consequence")
        ]
        observation = step(environment, explored["action"])
        learned = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=True,
            settle_pressure=True,
        )
        development_turns.append(
            turn_record(1, observation, learned, "Changed pixels return · one route is modulated · action repeats unaided")
        )
        observation = step(environment, learned["action"])
        reinforced = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=True,
            settle_pressure=True,
        )
        development_turns.append(
            turn_record(2, observation, reinforced, "A second completed loop strengthens retention")
        )
        episodes.append(
            episode(
                "arc3-a1-development",
                "The world answers",
                "Context, the motor that fired, and visible return preserve one physical route.",
                "development",
                "structure-formed",
                development_turns,
            )
        )

        # Frozen learned probe after an official environment reset.
        agent.command({"command": "clear_episode"})
        environment, observation = new_environment(arcade, game, seed)
        probe = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=False,
            settle_pressure=False,
        )
        episodes.append(
            episode(
                "arc3-a1-learned-probe",
                "The action returns without help",
                "The retained body produces one motor crossing from the reset raster alone.",
                "test",
                "learned-action",
                [turn_record(0, observation, probe, "Frozen probe · no babbling · no return modulation")],
            )
        )

        # Boundary meaning remains external to the physical motor.
        agent.command({"command": "clear_episode"})
        environment, observation = new_environment(arcade, game, seed)
        shuffled = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=False,
            settle_pressure=False,
            action_map=SHUFFLED_MAP,
        )
        episodes.append(
            episode(
                "arc3-a1-shuffled-map",
                "The boundary map changes",
                "The same motor crossing now becomes action 2 outside the organism.",
                "control",
                "mapping-followed",
                [turn_record(0, observation, shuffled, "Motor 0 is unchanged · external decoder now emits action 2")],
            )
        )

        # The twice-supported route survives a bounded gap.
        agent.command({"command": "advance_gap", "ticks": 10})
        environment, observation = new_environment(arcade, game, seed)
        retained = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=False,
            settle_pressure=False,
        )
        episodes.append(
            episode(
                "arc3-a1-retention",
                "The route survives a quiet interval",
                "After physical pressure and no rehearsal, the supported action remains available.",
                "test",
                "retained-action",
                [turn_record(0, observation, retained, "Retention probe · one learned crossing · no teaching")],
            )
        )

        # Exact counterexample: repeated babbling with return physically blocked.
        agent.command({"command": "reset_body"})
        environment, observation = new_environment(arcade, game, seed)
        blocked_turns: list[dict[str, Any]] = []
        blocked_first = organism_observe(
            agent,
            observation,
            babble_action=1,
            support_previous=False,
            settle_pressure=False,
        )
        blocked_turns.append(
            turn_record(0, observation, blocked_first, "First babbled action · return sensor blocked")
        )
        observation = step(environment, blocked_first["action"])
        blocked_second = organism_observe(
            agent,
            observation,
            babble_action=1,
            support_previous=False,
            settle_pressure=False,
        )
        blocked_turns.append(
            turn_record(1, observation, blocked_second, "Adjacent action repeats · ordinary drive gives zero credit")
        )
        observation = step(environment, blocked_second["action"])
        blocked_final = organism_observe(
            agent,
            observation,
            babble_action=None,
            support_previous=False,
            settle_pressure=True,
        )
        blocked_turns.append(
            turn_record(2, observation, blocked_final, "Pressure removes the unsupported route · silence")
        )
        episodes.append(
            episode(
                "arc3-a1-blocked-return",
                "Repetition is not evidence",
                "Two adjacent actions cannot mature a route when visible return is blocked.",
                "control",
                "expected-silence",
                blocked_turns,
            )
        )
    finally:
        agent.close()

    return {
        "schema_version": SCHEMA_VERSION,
        "game_id": game,
        "toolkit_revision": TOOLKIT_REVISION,
        "seed": seed,
        "exact_replay": False,
        "episodes": episodes,
    }


def canonical_bytes(suite: dict[str, Any]) -> bytes:
    return (json.dumps(suite, separators=(",", ":"), sort_keys=True) + "\n").encode("utf-8")


def main() -> None:
    args = arguments()
    first = run_suite(args.agent, args.game, args.seed)
    replay = run_suite(args.agent, args.game, args.seed)
    first_without_flag = dict(first)
    replay_without_flag = dict(replay)
    first_without_flag["exact_replay"] = False
    replay_without_flag["exact_replay"] = False
    if canonical_bytes(first_without_flag) != canonical_bytes(replay_without_flag):
        raise SystemExit("ARC3-A1 exact replay diverged")
    first["exact_replay"] = True
    replay["exact_replay"] = True
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(canonical_bytes(first))
    if args.replay_output:
        args.replay_output.parent.mkdir(parents=True, exist_ok=True)
        args.replay_output.write_bytes(canonical_bytes(replay))
    print(
        f"ARC3_A1_LIVE_OK game={args.game} episodes={len(first['episodes'])} "
        f"exact_replay=true output={args.output}"
    )


if __name__ == "__main__":
    main()
