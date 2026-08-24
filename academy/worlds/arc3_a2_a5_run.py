#!/usr/bin/env python3
"""Run the frozen ARC3 A2-A5 ladder against one persistent spatial body."""

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
ACTION_MAP = [1, 2, 3, 4]
CURRICULUM = [1, 4, 2, 3]
AUTONOMOUS_LIMIT = 64


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--agent", type=Path, required=True)
    parser.add_argument("--game", default="ls20")
    parser.add_argument("--seed", type=int, default=205)
    parser.add_argument("--held-out-seed", type=int, default=206)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--report-output", type=Path, required=True)
    parser.add_argument("--replay-output", type=Path)
    parser.add_argument("--stop-after-a3", action="store_true")
    parser.add_argument("--initial-gap", type=int, default=0)
    return parser.parse_args()


@dataclass
class Agent:
    process: subprocess.Popen[str]

    @classmethod
    def start(cls, executable: Path, seed: int) -> "Agent":
        return cls(
            subprocess.Popen(
                [str(executable), str(seed), "spatial"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
            )
        )

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


def environment(arcade: Any, game: str, seed: int) -> tuple[Any, Any]:
    instance = arcade.make(game, seed=seed, include_frame_data=True)
    if instance is None:
        raise RuntimeError(f"unable to create ARC-AGI-3 environment {game}")
    observation = instance.reset()
    if observation is None:
        raise RuntimeError("official ARC reset returned no observation")
    return instance, observation


def observe(
    agent: Agent,
    observation: Any,
    *,
    babble_action: int | None,
    support_previous: bool,
) -> dict[str, Any]:
    response = agent.command(
        {
            "command": "observe",
            "frame": frame(observation),
            "available_actions": [int(value) for value in observation.available_actions],
            "babble_action": babble_action,
            "support_previous": support_previous,
            "settle_pressure": False,
            "action_map": ACTION_MAP,
        }
    )
    if response.get("response") != "observation":
        raise RuntimeError(f"unexpected ARC agent response: {response}")
    del response["response"]
    return response


def step(instance: Any, action_id: int) -> Any:
    action = GameAction.from_id(action_id)
    data: dict[str, int] = {"x": 32, "y": 32} if action.is_complex() else {}
    observation = instance.step(action, data=data)
    if observation is None:
        raise RuntimeError("official ARC step returned no observation")
    return observation


def turn(
    index: int,
    observation: Any,
    organism: dict[str, Any],
    caption: str,
) -> dict[str, Any]:
    return {
        "turn": index,
        "frame": frame(observation),
        "organism": organism,
        "official_state": observation.state.value,
        "levels_completed": int(observation.levels_completed),
        "win_levels": int(observation.win_levels),
        "caption": caption,
    }


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


def gate(identifier: str, status: str, claim: str, details: dict[str, Any]) -> dict[str, Any]:
    return {"id": identifier, "status": status, "claim": claim, "details": details}


def skipped_gate(identifier: str, claim: str, reason: str) -> dict[str, Any]:
    return gate(identifier, "skipped", claim, {"reason": reason})


def run_ladder(
    agent_path: Path,
    game: str,
    seed: int,
    held_out_seed: int,
    stop_after_a3: bool,
    initial_gap: int,
) -> tuple[dict[str, Any], dict[str, Any]]:
    arcade = arc_agi.Arcade()
    agent = Agent.start(agent_path, seed)
    episodes: list[dict[str, Any]] = []
    gates: list[dict[str, Any]] = []
    first_failure: str | None = None
    try:
        agent.command({"command": "reset_body"})
        if initial_gap:
            agent.command({"command": "advance_gap", "ticks": initial_gap})
        instance, observation = environment(arcade, game, seed)
        development_turns: list[dict[str, Any]] = []
        action_turns: list[list[dict[str, Any]]] = [[] for _ in CURRICULUM]
        contexts: list[int] = []
        changes: list[int] = []
        updates: list[int] = []
        actions: list[int | None] = []
        quiescent = True

        previous_raster = frame(observation)
        for index, expected_action in enumerate(CURRICULUM):
            organism = observe(
                agent,
                observation,
                babble_action=expected_action,
                support_previous=index > 0,
            )
            caption = (
                f"State {organism['context']} · motor babbling crosses as action {expected_action}"
                if index == 0
                else f"Visible return credits the prior route · state {organism['context']} babbles action {expected_action}"
            )
            record = turn(index, observation, organism, caption)
            development_turns.append(record)
            action_turns[index].append(record)
            contexts.append(int(organism["context"]))
            updates.append(int(organism["plasticity_updates"]))
            actions.append(organism["action"])
            quiescent = quiescent and bool(organism["naturally_quiescent"])
            if organism["action"] is None:
                break
            observation = step(instance, int(organism["action"]))
            current_raster = frame(observation)
            changes.append(sum(left != right for left, right in zip(previous_raster, current_raster)))
            previous_raster = current_raster

        if len(actions) == len(CURRICULUM) and all(action is not None for action in actions):
            final = observe(agent, observation, babble_action=None, support_previous=True)
            final_record = turn(
                len(CURRICULUM),
                observation,
                final,
                "The fourth changed raster returns · its traversed route receives the final qualified update",
            )
            development_turns.append(final_record)
            action_turns[-1].append(final_record)
            updates.append(int(final["plasticity_updates"]))
            quiescent = quiescent and bool(final["naturally_quiescent"])

        a2_pass = (
            len(contexts) == len(CURRICULUM)
            and len(set(contexts)) == len(CURRICULUM)
            and actions == CURRICULUM
            and len(changes) == len(CURRICULUM)
            and all(value > 0 for value in changes)
            and updates == [0, 1, 1, 1, 1]
            and quiescent
        )
        gates.append(
            gate(
                "A2",
                "pass" if a2_pass else "fail",
                "four contextual action-effect relations coexist in one retained body",
                {
                    "curriculum": CURRICULUM,
                    "contexts": contexts,
                    "changed_pixels": changes,
                    "plasticity_updates": updates,
                    "actions": actions,
                    "naturally_quiescent": quiescent,
                },
            )
        )
        episodes.append(
            episode(
                "arc3-a2-four-actions",
                "Four actions enter one body",
                "Four distinct whole-raster states preserve four different motor routes.",
                "development",
                "structure-formed" if a2_pass else "expected-silence",
                development_turns,
            )
        )
        for index, expected_action in enumerate(CURRICULUM):
            episodes.append(
                episode(
                    f"arc3-a2-action-{expected_action}",
                    f"Action {expected_action} changes the world",
                    f"Context {contexts[index] if index < len(contexts) else 'missing'} traverses motor {expected_action}.",
                    "development",
                    "scaffolded-action" if index < len(actions) and actions[index] == expected_action else "expected-silence",
                    action_turns[index] or development_turns[-1:],
                )
            )

        if not a2_pass:
            first_failure = "A2"
            gates.extend(
                [
                    skipped_gate("A3", "state-conditioned autonomous replay", "A2 failed"),
                    skipped_gate("A4", "autonomous official level completion", "A2 failed"),
                    skipped_gate("A5", "held-out-seed transfer", "A2 failed"),
                ]
            )
        else:
            agent.command({"command": "clear_episode"})
            replay_environment, replay_observation = environment(arcade, game, seed)
            replay_turns: list[dict[str, Any]] = []
            replay_actions: list[int | None] = []
            replay_contexts: list[int] = []
            replay_updates: list[int] = []
            replay_quiescent = True
            for index, expected_action in enumerate(CURRICULUM):
                organism = observe(
                    agent,
                    replay_observation,
                    babble_action=None,
                    support_previous=False,
                )
                replay_turns.append(
                    turn(
                        index,
                        replay_observation,
                        organism,
                        f"No scaffold · retained context {organism['context']} emits action {organism['action']}",
                    )
                )
                replay_actions.append(organism["action"])
                replay_contexts.append(int(organism["context"]))
                replay_updates.append(int(organism["plasticity_updates"]))
                replay_quiescent = replay_quiescent and bool(organism["naturally_quiescent"])
                if organism["action"] is None:
                    break
                replay_observation = step(replay_environment, int(organism["action"]))

            a3_pass = (
                replay_actions == CURRICULUM
                and replay_contexts == contexts
                and replay_updates == [0] * len(CURRICULUM)
                and replay_quiescent
            )
            gates.append(
                gate(
                    "A3",
                    "pass" if a3_pass else "fail",
                    "the retained body selects four actions from four official raster states without help",
                    {
                        "expected_actions": CURRICULUM,
                        "observed_actions": replay_actions,
                        "development_contexts": contexts,
                        "probe_contexts": replay_contexts,
                        "plasticity_updates": replay_updates,
                        "naturally_quiescent": replay_quiescent,
                    },
                )
            )
            episodes.append(
                episode(
                    "arc3-a3-autonomous-replay",
                    "The four choices return",
                    "A reset world elicits the retained four-action sequence without babbling or modulation.",
                    "test",
                    "learned-action" if a3_pass else "expected-silence",
                    replay_turns or development_turns[-1:],
                )
            )

            if not a3_pass:
                first_failure = "A3"
                gates.extend(
                    [
                        skipped_gate("A4", "autonomous official level completion", "A3 failed"),
                        skipped_gate("A5", "held-out-seed transfer", "A3 failed"),
                    ]
                )
            elif stop_after_a3:
                gates.extend(
                    [
                        skipped_gate("A4", "autonomous official level completion", "diagnostic stops after A3 replay"),
                        skipped_gate("A5", "held-out-seed transfer", "diagnostic stops after A3 replay"),
                    ]
                )
            else:
                agent.command({"command": "clear_episode"})
                autonomous_environment, autonomous_observation = environment(arcade, game, seed)
                autonomous_turns: list[dict[str, Any]] = []
                completed = False
                stop_reason = "action ceiling"
                for index in range(AUTONOMOUS_LIMIT):
                    organism = observe(
                        agent,
                        autonomous_observation,
                        babble_action=None,
                        support_previous=False,
                    )
                    autonomous_turns.append(
                        turn(
                            index,
                            autonomous_observation,
                            organism,
                            (
                                f"Autonomous action {organism['action']} from context {organism['context']}"
                                if organism["action"] is not None
                                else f"Unfamiliar context {organism['context']} · the body is silent"
                            ),
                        )
                    )
                    if int(autonomous_observation.levels_completed) > 0:
                        completed = True
                        stop_reason = "level completed"
                        break
                    if organism["action"] is None:
                        stop_reason = "no outward crossing"
                        break
                    autonomous_observation = step(autonomous_environment, int(organism["action"]))
                    if int(autonomous_observation.levels_completed) > 0:
                        completed = True
                        stop_reason = "level completed"
                        terminal = observe(
                            agent,
                            autonomous_observation,
                            babble_action=None,
                            support_previous=False,
                        )
                        autonomous_turns.append(
                            turn(
                                index + 1,
                                autonomous_observation,
                                terminal,
                                "The official world reports a completed level; that field never entered the body",
                            )
                        )
                        break

                gates.append(
                    gate(
                        "A4",
                        "pass" if completed else "fail",
                        "the retained body completes an official level without curriculum or semantic feedback",
                        {
                            "completed": completed,
                            "actions_attempted": sum(
                                item["organism"]["action"] is not None for item in autonomous_turns
                            ),
                            "levels_completed": int(autonomous_observation.levels_completed),
                            "stop_reason": stop_reason,
                            "limit": AUTONOMOUS_LIMIT,
                        },
                    )
                )
                episodes.append(
                    episode(
                        "arc3-a4-level-attempt",
                        "Beyond the learned square",
                        "The curriculum is gone; the body must now continue from raw consequences alone.",
                        "test",
                        "retained-action" if completed else "expected-silence",
                        autonomous_turns,
                    )
                )

                if not completed:
                    first_failure = "A4"
                    gates.append(skipped_gate("A5", "held-out-seed transfer", "A4 failed"))
                else:
                    agent.command({"command": "clear_episode"})
                    transfer_environment, transfer_observation = environment(
                        arcade, game, held_out_seed
                    )
                    transfer_turns: list[dict[str, Any]] = []
                    transfer_completed = False
                    transfer_reason = "action ceiling"
                    for index in range(AUTONOMOUS_LIMIT):
                        organism = observe(
                            agent,
                            transfer_observation,
                            babble_action=None,
                            support_previous=False,
                        )
                        transfer_turns.append(
                            turn(
                                index,
                                transfer_observation,
                                organism,
                                f"Held-out seed · context {organism['context']} · action {organism['action']}",
                            )
                        )
                        if organism["action"] is None:
                            transfer_reason = "no outward crossing"
                            break
                        transfer_observation = step(
                            transfer_environment, int(organism["action"])
                        )
                        if int(transfer_observation.levels_completed) > 0:
                            transfer_completed = True
                            transfer_reason = "level completed"
                            break
                    gates.append(
                        gate(
                            "A5",
                            "pass" if transfer_completed else "fail",
                            "the retained body completes a held-out-seed level without teaching",
                            {
                                "completed": transfer_completed,
                                "seed": held_out_seed,
                                "stop_reason": transfer_reason,
                            },
                        )
                    )
                    episodes.append(
                        episode(
                            "arc3-a5-transfer",
                            "A fresh world seed",
                            "The same retained body receives no further curriculum.",
                            "test",
                            "retained-action" if transfer_completed else "expected-silence",
                            transfer_turns,
                        )
                    )
                    if not transfer_completed:
                        first_failure = "A5"
    finally:
        agent.close()

    # The existing episode renderer expects seven entries. Skipped gates are
    # represented by the final executed physical frame, never by invented body
    # activity.
    while len(episodes) < 7:
        failed = first_failure or "none"
        episodes.append(
            episode(
                f"arc3-ladder-skipped-{len(episodes)}",
                "Later gate not executed",
                f"The sequential ladder stopped at {failed}; no later evidence was manufactured.",
                "control",
                "expected-silence",
                episodes[-1]["turns"][-1:],
            )
        )
    episodes = episodes[:7]

    suite = {
        "schema_version": SCHEMA_VERSION,
        "game_id": game,
        "toolkit_revision": TOOLKIT_REVISION,
        "seed": seed,
        "exact_replay": False,
        "episodes": episodes,
    }
    report = {
        "schema_version": SCHEMA_VERSION,
        "game_id": game,
        "development_seed": seed,
        "held_out_seed": held_out_seed,
        "initial_gap": initial_gap,
        "curriculum": CURRICULUM,
        "first_failure": first_failure,
        "all_gates_passed": first_failure is None and not stop_after_a3,
        "executed_gates_passed": first_failure is None,
        "completion_scope": "A3" if stop_after_a3 else "A5",
        "exact_replay": False,
        "gates": gates,
    }
    return suite, report


def canonical_bytes(value: dict[str, Any]) -> bytes:
    return (json.dumps(value, separators=(",", ":"), sort_keys=True) + "\n").encode("utf-8")


def main() -> None:
    args = arguments()
    if args.initial_gap < 0:
        raise SystemExit("--initial-gap must be non-negative")
    first_suite, first_report = run_ladder(
        args.agent,
        args.game,
        args.seed,
        args.held_out_seed,
        args.stop_after_a3,
        args.initial_gap,
    )
    replay_suite, replay_report = run_ladder(
        args.agent,
        args.game,
        args.seed,
        args.held_out_seed,
        args.stop_after_a3,
        args.initial_gap,
    )
    if canonical_bytes(first_suite) != canonical_bytes(replay_suite):
        raise SystemExit("ARC3 A2-A5 suite replay diverged")
    if canonical_bytes(first_report) != canonical_bytes(replay_report):
        raise SystemExit("ARC3 A2-A5 gate report replay diverged")
    first_suite["exact_replay"] = True
    first_report["exact_replay"] = True
    replay_suite["exact_replay"] = True
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.report_output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(canonical_bytes(first_suite))
    args.report_output.write_bytes(canonical_bytes(first_report))
    if args.replay_output:
        args.replay_output.parent.mkdir(parents=True, exist_ok=True)
        args.replay_output.write_bytes(canonical_bytes(replay_suite))
    statuses = ",".join(f"{item['id']}={item['status']}" for item in first_report["gates"])
    print(
        f"ARC3_A2_A5_LADDER_COMPLETE {statuses} "
        f"first_failure={first_report['first_failure']} exact_replay=true"
    )


if __name__ == "__main__":
    main()
