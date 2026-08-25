#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

check_hash() {
  local expected="$1"
  local path="$2"
  local actual
  actual="$(sha256sum "$path" | awk '{print $1}')"
  test "$actual" = "$expected" || {
    printf 'hash mismatch: %s\nexpected %s\nactual   %s\n' "$path" "$expected" "$actual" >&2
    exit 1
  }
}

check_hash e7b9d60ce0330d10692b13fe85967e189d734a00177edef98018f9b4499a09ed \
  truelearner/crates/core/src/lib.rs
check_hash 297775ee625d55e116adb92c9f6906c8a5da40e8533bce2fa71cf7bf4b002947 \
  truelearner/crates/core/src/mechanics.rs
check_hash c919b87fb2628f23e019a59ec59eab3fefb7faffa3a48fa03e6e9ea4d1ebbb4c \
  truelearner/crates/core/Cargo.toml
check_hash 8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812 \
  truelearner/crates/arena-format/src/lib.rs
check_hash 9e14b0f065ba37966c2ffc300f6d149b0847d092cb90e666456d3889d889d9c6 \
  experiments/archive/pxc-authority/results/px6_lrc_consequence_authority_v1.csv
check_hash 2ca3ae797a079387ff7e9f4413ae5030f380ab997bea520c79460ffac9f95709 \
  results/fd1_consequence_consolidation_v3/matrix.csv

test "$(awk -F, 'NR > 1 && $9 == 4 && $10 == 2 && $21 == "true" { count += 1 } END { print count + 0 }' \
  experiments/archive/pxc-authority/results/px6_lrc_consequence_authority_v1.csv)" -gt 0
test "$(awk -F, '$4 == "one_qualified_consequence" && $6 ~ /after_consequence@5\/[0-9]+:1\/4\// { count += 1 } END { print count + 0 }' \
  results/fd1_consequence_consolidation_v3/matrix.csv)" -eq 40

if rg -n 'arrow\.coupling\s*=|coupling\s*\+=|coupling\s*=\s*coupling\.' \
  truelearner/crates/core/src/lib.rs; then
  printf 'unexpected coupling mutation in frozen FD1 core\n' >&2
  exit 1
fi

if rg -n 'reward|correctness|path_id|route_id|predecessor|hop_count|continue_credit' \
  experiments/arms/cr0-coupling-necessity/src/main.rs; then
  printf 'semantic routing surface found in CR0 evaluator\n' >&2
  exit 1
fi

printf 'CR0_COUPLING_NECESSITY_V1_STATIC_AUDIT_PASS\n'
