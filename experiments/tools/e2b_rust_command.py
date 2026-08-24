#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.11"
# dependencies = ["e2b>=2,<3"]
# ///
"""Run committed TrueLearner commands in fresh or reusable E2B workers."""

import argparse
import os
from pathlib import Path
import subprocess

from e2b import Sandbox, SandboxNotFoundException


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", nargs="*", help="remote commands, run in order")
    parser.add_argument("--template", default="truelearner-rust-1-97-worker")
    parser.add_argument("--sandbox-timeout", type=int, default=3600)
    parser.add_argument("--command-timeout", type=int, default=3300)
    parser.add_argument(
        "--state-file",
        type=Path,
        help=(
            "reuse one development sandbox and its Cargo/sccache artifacts; "
            "omit for fresh one-shot evidence"
        ),
    )
    parser.add_argument(
        "--terminate-state",
        action="store_true",
        help="terminate the reusable worker named by --state-file, then exit",
    )
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
    if args.terminate_state:
        if not args.state_file:
            raise SystemExit("--terminate-state requires --state-file")
        if args.command:
            raise SystemExit("--terminate-state does not accept remote commands")
        if not args.state_file.exists():
            print("sandbox_state=absent")
            return
        sandbox_id = args.state_file.read_text().strip()
        try:
            Sandbox.connect(sandbox_id, timeout=60).kill()
            print(f"sandbox_terminated={sandbox_id}")
        except SandboxNotFoundException:
            print(f"sandbox_already_absent={sandbox_id}")
        finally:
            args.state_file.unlink(missing_ok=True)
        return
    if not args.command:
        raise SystemExit("at least one remote command is required")
    tracked = checked_output("git", "status", "--porcelain", "--untracked-files=no")
    if tracked:
        raise SystemExit("E2B source upload requires a clean tracked Git worktree")

    commit = checked_output("git", "rev-parse", "HEAD")
    archive = subprocess.check_output(["git", "archive", "--format=tar.gz", "HEAD"])
    sandbox = None
    reused = False
    if args.state_file and args.state_file.exists():
        sandbox_id = args.state_file.read_text().strip()
        if sandbox_id:
            try:
                sandbox = Sandbox.connect(sandbox_id, timeout=args.sandbox_timeout)
                reused = True
            except SandboxNotFoundException:
                args.state_file.unlink(missing_ok=True)
    if sandbox is None:
        sandbox = Sandbox.create(
            template=args.template,
            timeout=args.sandbox_timeout,
            metadata={
                "project": "truelearner",
                "experiment": "physical-body-v1-development"
                if args.state_file
                else "physical-body-v1-authority",
                "commit": commit,
            },
        )
        if args.state_file:
            args.state_file.parent.mkdir(parents=True, exist_ok=True)
            args.state_file.write_text(f"{sandbox.sandbox_id}\n")
    print(f"sandbox_id={sandbox.sandbox_id}")
    print(f"sandbox_reused={str(reused).lower()}")
    print(f"commit={commit}")
    try:
        sandbox.files.write("/tmp/source.tar.gz", archive, use_octet_stream=True)
        setup = (
            "export PATH=/home/user/.cargo/bin:$PATH && "
            "rm -rf /home/user/truelearner && "
            "mkdir -p /home/user/truelearner /home/user/truelearner-target && "
            "tar -xzf /tmp/source.tar.gz -C /home/user/truelearner && "
            "cd /home/user/truelearner && "
            "export CARGO_TARGET_DIR=/home/user/truelearner-target && "
            "export CARGO_INCREMENTAL=0 && "
            "export SCCACHE_DIR=/home/user/.cache/sccache && "
            "export SCCACHE_CACHE_SIZE=20G && "
            "export SCCACHE_BASEDIRS=/home/user/truelearner"
        )
        if args.state_file:
            setup += (
                " && if ! command -v sccache >/dev/null 2>&1; then "
                "case $(uname -m) in "
                "x86_64) sccache_arch=x86_64 ;; "
                "aarch64|arm64) sccache_arch=aarch64 ;; "
                "*) printf 'unsupported sccache architecture: %s\\n' \"$(uname -m)\" >&2; "
                "exit 2 ;; esac && "
                "sccache_version=0.17.0 && "
                "sccache_name=sccache-v${sccache_version}-${sccache_arch}-unknown-linux-musl && "
                "curl -fsSL \"https://github.com/mozilla/sccache/releases/download/"
                "v${sccache_version}/${sccache_name}.tar.gz\" "
                "| tar -xz -C /tmp && "
                "install -m 755 \"/tmp/${sccache_name}/sccache\" "
                "/home/user/.cargo/bin/sccache; fi"
            )
        checks = " && ".join(f"({item})" for item in args.command)
        command = (
            setup
            + " && { "
            + checks
            + "; remote_status=$?; printf '\\n__E2B_REMOTE_EXIT__=%s\\n' "
            + '"$remote_status"; exit 0; }'
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
        if not args.state_file:
            sandbox.kill()


if __name__ == "__main__":
    main()
