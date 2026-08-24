#!/usr/bin/env python3
"""Capture an official ARC-AGI-3 session into Academy's normalized JSONL."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import arc_agi
from arcengine import GameAction

SCHEMA_VERSION = 1
TOOLKIT_REVISION = "arcprize/ARC-AGI@f12822c4d550121c35a275008d964afbbed47d2f"


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--game", default="ls20")
    parser.add_argument("--turns", type=int, default=12)
    parser.add_argument("--seed", type=int, default=205)
    parser.add_argument("--output", type=Path, required=True)
    return parser.parse_args()


def normalized_frames(observation: Any) -> list[list[int]]:
    frames: list[list[int]] = []
    for frame in observation.frame:
        values = [int(value) for value in frame.reshape(-1).tolist()]
        if len(values) != 64 * 64:
            raise ValueError(f"unexpected ARC frame size: {len(values)}")
        if any(value < 0 or value > 15 for value in values):
            raise ValueError("ARC frame color outside 0..15")
        frames.append(values)
    if not frames:
        raise ValueError("ARC observation contains no frame")
    return frames


def observation_record(game_id: str, turn: int, action: dict[str, Any] | None, observation: Any) -> dict[str, Any]:
    return {
        "schema_version": SCHEMA_VERSION,
        "kind": "observation",
        "game_id": game_id,
        "turn": turn,
        "action": action,
        "state": observation.state.value,
        "levels_completed": int(observation.levels_completed),
        "win_levels": int(observation.win_levels),
        "full_reset": bool(observation.full_reset),
        "available_actions": [int(action_id) for action_id in observation.available_actions],
        "frames": normalized_frames(observation),
    }


def choose_action(available: list[int], turn: int) -> tuple[GameAction, dict[str, int], dict[str, Any]]:
    choices = [action_id for action_id in available if action_id != 0]
    if not choices:
        raise RuntimeError("environment exposes no non-reset action")
    action_id = choices[turn % len(choices)]
    action = GameAction.from_id(action_id)
    data: dict[str, int] = {}
    record: dict[str, Any] = {"id": action_id}
    if action.is_complex():
        coordinate = (turn * 11) % 64
        data = {"x": coordinate, "y": (63 - coordinate)}
        record["data"] = data
    return action, data, record


def main() -> None:
    args = arguments()
    if args.turns < 0:
        raise SystemExit("--turns must be non-negative")
    arcade = arc_agi.Arcade()
    environment = arcade.make(args.game, seed=args.seed, include_frame_data=True)
    if environment is None:
        raise SystemExit(f"unable to create ARC-AGI-3 environment {args.game}")
    first = environment.reset()
    if first is None:
        raise SystemExit("ARC-AGI-3 reset returned no observation")

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", encoding="utf-8") as destination:
        metadata = {
            "schema_version": SCHEMA_VERSION,
            "kind": "metadata",
            "game_id": args.game,
            "toolkit_revision": TOOLKIT_REVISION,
            "seed": args.seed,
            "available_actions": [int(action_id) for action_id in first.available_actions],
        }
        destination.write(json.dumps(metadata, separators=(",", ":"), sort_keys=True) + "\n")
        destination.write(json.dumps(observation_record(args.game, 0, None, first), separators=(",", ":"), sort_keys=True) + "\n")

        observation = first
        for turn in range(args.turns):
            action, data, record = choose_action(observation.available_actions, turn)
            observation = environment.step(action, data=data)
            if observation is None:
                raise RuntimeError(f"ARC-AGI-3 step {turn + 1} returned no observation")
            destination.write(
                json.dumps(
                    observation_record(args.game, turn + 1, record, observation),
                    separators=(",", ":"),
                    sort_keys=True,
                )
                + "\n"
            )

    print(f"ARC3_CAPTURE_OK game={args.game} observations={args.turns + 1} output={args.output}")


if __name__ == "__main__":
    main()
