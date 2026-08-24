#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.11"
# dependencies = ["e2b>=2,<3"]
# ///
"""Run a committed TrueLearner command in a fresh, self-terminating E2B worker."""

import argparse
import os
from pathlib import Path
import subprocess

from e2b import Sandbox


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", nargs="+", help="remote commands, run in order")
    parser.add_argument("--template", default="truelearner-rust-1-97-worker")
    parser.add_argument("--sandbox-timeout", type=int, default=3600)
    parser.add_argument("--command-timeout", type=int, default=3300)
    parser.add_argument(
        "--download",
        action="append",
        default=[],
        metavar="REMOTE=LOCAL",
        help="download a remote artifact even when the remote command is negative",
    )
    return parser.parse_args()


def checked_output(*command: str) -> str:
    return subprocess.check_output(command, text=True).strip()


def main() -> None:
    args = arguments()
    if "E2B_API_KEY" not in os.environ:
        raise SystemExit("E2B_API_KEY is not set; source the existing .env.e2b first")
    tracked = checked_output("git", "status", "--porcelain", "--untracked-files=no")
    if tracked:
        raise SystemExit("E2B source upload requires a clean tracked Git worktree")

    commit = checked_output("git", "rev-parse", "HEAD")
    archive = subprocess.check_output(["git", "archive", "--format=tar.gz", "HEAD"])
    sandbox = Sandbox.create(
        template=args.template,
        timeout=args.sandbox_timeout,
        metadata={
            "project": "truelearner",
            "experiment": "physical-body-v1-authority",
            "commit": commit,
        },
    )
    print(f"sandbox_id={sandbox.sandbox_id}")
    print(f"commit={commit}")
    try:
        sandbox.files.write("/tmp/source.tar.gz", archive, use_octet_stream=True)
        setup = (
            "export PATH=/home/user/.cargo/bin:$PATH && "
            "mkdir -p /home/user/truelearner && "
            "tar -xzf /tmp/source.tar.gz -C /home/user/truelearner && "
            "cd /home/user/truelearner"
        )
        checks = " && ".join(f"({item})" for item in args.command)
        command = (
            setup
            + " && { "
            + checks
            + "; remote_status=$?; printf '\\n__E2B_REMOTE_EXIT__=%s\\n' "$remote_status"; exit 0; }"
        )
        result = sandbox.commands.run(command, timeout=args.command_timeout)
        stdout = result.stdout or ""
        marker = "__E2B_REMOTE_EXIT__="
        marker_lines = [line for line in stdout.splitlines() if line.startswith(marker)]
        if len(marker_lines) != 1:
            raise SystemExit("remote command did not report a unique exit status")
        remote_status = int(marker_lines[0].removeprefix(marker))
        stdout = "\n".join(line for line in stdout.splitlines() if not line.startswith(marker))
        if stdout:
            print(stdout)
        if result.stderr:
            print(result.stderr, end="" if result.stderr.endswith("\n") else "\n")

        for mapping in args.download:
            remote, separator, local = mapping.partition("=")
            if not separator:
                raise SystemExit(f"invalid --download mapping: {mapping}")
            remote_path = remote if remote.startswith("/") else f"/home/user/truelearner/{remote}"
            local_path = Path(local)
            try:
                payload = sandbox.files.read(remote_path, format="bytes")
            except Exception as error:
                print(f"artifact unavailable: {remote_path}: {error}")
                continue
            local_path.parent.mkdir(parents=True, exist_ok=True)
            local_path.write_bytes(payload)
            print(f"downloaded {remote_path} to {local_path}")

        if remote_status != 0:
            raise SystemExit(remote_status)
    finally:
        sandbox.kill()


if __name__ == "__main__":
    main()
