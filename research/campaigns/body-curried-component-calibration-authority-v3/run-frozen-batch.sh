#!/bin/sh
set -eu

REPO_ROOT=$(git rev-parse --show-toplevel)
SUBJECT_COMMIT=9679c4e2115ce904a5bf52295f7351f6fa6e4f33
SUBJECT_TREE=c29793653bb725ac273d1465c67e4010adf738dc
ARTIFACT_DIR="$REPO_ROOT/research/campaigns/body-curried-component-calibration-authority-v3/artifacts"
SANDBOX_ROOT=$(mktemp -d /tmp/truelearner-calibration-authority-v3.XXXXXX)
WORKTREE_PATH="$SANDBOX_ROOT/subject"
TARGET_PATH=$(mktemp -d /tmp/truelearner-calibration-authority-v3-target.XXXXXX)

cleanup() {
  git -C "$REPO_ROOT" worktree remove --force "$WORKTREE_PATH" >/dev/null 2>&1 || true
  rmdir "$SANDBOX_ROOT" >/dev/null 2>&1 || true
  rm -rf "$TARGET_PATH"
}
trap cleanup EXIT INT TERM

if [ -e "$ARTIFACT_DIR" ]; then
  echo "authority artifacts already exist; refusing a second valid run" >&2
  exit 1
fi
mkdir -p "$ARTIFACT_DIR"

git -C "$REPO_ROOT" worktree add --detach "$WORKTREE_PATH" "$SUBJECT_COMMIT" >/dev/null
cd "$WORKTREE_PATH"
export CARGO_TARGET_DIR="$TARGET_PATH"

actual_commit=$(git rev-parse HEAD)
actual_tree=$(git rev-parse HEAD^{tree})
before_status=$(git status --porcelain | wc -l | tr -d ' ')
test "$actual_commit" = "$SUBJECT_COMMIT"
test "$actual_tree" = "$SUBJECT_TREE"
test "$before_status" = "0"

{
  echo "worktree=$WORKTREE_PATH"
  echo "cargo_target=$TARGET_PATH"
  echo "subject_commit=$actual_commit"
  echo "subject_tree=$actual_tree"
  echo "clean_status_lines_before=$before_status"
  rustc --version
  uname -sm
} > "$ARTIFACT_DIR/environment.log"

tracked_artifacts=$(git ls-files research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts)
test "$tracked_artifacts" = "research/campaigns/workstation-return-bearing-opportunity-composition-v1/artifacts/preflight-failure.json"
oversized_blobs=$(git ls-tree -rl HEAD | awk '$4 > 100000000 { print }')
test -z "$oversized_blobs"
{
  echo "tracked_workstation_artifacts:"
  echo "$tracked_artifacts"
  echo "subject_tree_blobs_over_100000000=0"
  git ls-tree -rl HEAD | sort -k4 -nr | head -1 | awk '{print "largest_subject_blob_bytes=" $4 " path=" $5}'
} > "$ARTIFACT_DIR/publishability.log"

cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test calibration calibration_laws > "$ARTIFACT_DIR/laws-preflight.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test calibration > "$ARTIFACT_DIR/laws-and-transfer.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_trace_materializes_drive_fall_return_and_zero_identity -- --exact > "$ARTIFACT_DIR/physical-calibration-trace.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment regulation_body_curried_calibration -- --exact > "$ARTIFACT_DIR/complete-regulation.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_persistent_drive -- --ignored --exact > "$ARTIFACT_DIR/remove-persistent-drive.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_directional_change -- --ignored --exact > "$ARTIFACT_DIR/remove-directional-change.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_zero_identity -- --ignored --exact > "$ARTIFACT_DIR/remove-zero-identity.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_terminal_shifted_context_reference -- --ignored --exact > "$ARTIFACT_DIR/complete-terminal-context.log" 2>&1
cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_fixed_context_terminal_residence -- --ignored --exact > "$ARTIFACT_DIR/remove-body-context-terminal.log" 2>&1
if cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_fixed_context -- --ignored --exact > "$ARTIFACT_DIR/historical-any-window-context.log" 2>&1; then
  echo "historical falsified oracle unexpectedly passed" >&2
  exit 1
fi

cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core > "$ARTIFACT_DIR/core-preflight.log" 2>&1
/usr/bin/time -p cargo test --workspace --locked --manifest-path truelearner/Cargo.toml > "$ARTIFACT_DIR/workspace-bootstrap.log" 2>&1
/usr/bin/time -p cargo test --workspace --locked --manifest-path truelearner/Cargo.toml > "$ARTIFACT_DIR/workspace-warm-regression.log" 2>&1
warm_seconds=$(awk '/^real / { value=$2 } END { print value }' "$ARTIFACT_DIR/workspace-warm-regression.log")
awk -v seconds="$warm_seconds" 'BEGIN { exit !(seconds < 10.0) }'

identifier_pattern='(?<![A-Za-z0-9])(finger|hand|eye|ear|voice|modality|answer|score|evaluator)(?![A-Za-z0-9])'
printf 'finger_signal\nEvaluator\n' | rg -ni -P "$identifier_pattern" > "$ARTIFACT_DIR/firewall-positive-control.log"
if printf 'TRACE_CLEAR_PHASE\ntruelearner_core\n' | rg -ni -P "$identifier_pattern" > "$ARTIFACT_DIR/firewall-substring-control.log"; then
  echo "substring counterexample incorrectly matched" >&2
  exit 1
fi
printf 'TRACE_CLEAR_PHASE: no match\ntruelearner_core: no match\n' > "$ARTIFACT_DIR/firewall-substring-control.log"
if sed -n '180,680p' truelearner/crates/embodiment/src/lib.rs | rg -ni -P "$identifier_pattern" > "$ARTIFACT_DIR/semantic-firewall.log"; then
  echo "forbidden production identifier component found" >&2
  exit 1
fi
echo "production_forbidden_identifier_components=0" > "$ARTIFACT_DIR/semantic-firewall.log"
if rg -n 'CalibrationAblation' truelearner/crates/embodiment/src/lib.rs >> "$ARTIFACT_DIR/semantic-firewall.log"; then
  echo "production calibration ablation wiring found" >&2
  exit 1
fi
echo "production_calibration_ablation_wiring=0" >> "$ARTIFACT_DIR/semantic-firewall.log"

after_status=$(git status --porcelain | wc -l | tr -d ' ')
test "$after_status" = "0"
echo "clean_status_lines_after=$after_status" >> "$ARTIFACT_DIR/environment.log"
{
  echo "valid_runs=1"
  echo "invalid_infrastructure_attempts_without_scientific_artifacts=0"
  echo "result=completed"
} > "$ARTIFACT_DIR/batch-status.log"
